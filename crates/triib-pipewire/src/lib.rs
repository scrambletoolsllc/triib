//! Virtual audio devices on PipeWire for this computer's streams: for a
//! stream output, a playback device apps play into; for a stream input, a
//! recording device apps record from. Each fills or drains a buffer the
//! stream reads or writes as it would a device's, on PipeWire's clock.
//!
//! One thread runs PipeWire's loop and owns the devices; [`VirtualAudio`]
//! asks it to add them, and dropping a [`VirtualDevice`] removes it.
//! Elsewhere than Linux, starting says it is not supported.

use std::io;
use std::sync::Arc;

use avb_stream::audio::{FrameBuffer, SharedBuffer};

/// A virtual device to add.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceSpec {
    /// PipeWire's node name, unique among the computer's: letters, digits
    /// and dashes.
    pub name: String,
    /// What apps and the desktop show.
    pub description: String,
    pub channels: u16,
    pub rate: u32,
    /// A device apps play into, for a stream output; else one apps record
    /// from, for a stream input.
    pub playback: bool,
}

/// A virtual device, there while it lives.
pub struct VirtualDevice {
    shared: SharedBuffer,
    _remove: imp::Remover,
}

impl VirtualDevice {
    /// The buffer the device fills, apps having played into it, or drains
    /// for apps to record.
    pub fn shared(&self) -> SharedBuffer {
        self.shared.clone()
    }
}

/// The thread that runs PipeWire and its devices. Dropping it removes
/// them all.
pub struct VirtualAudio {
    inner: imp::Audio,
}

impl VirtualAudio {
    /// Connects to PipeWire, failing where it does not run.
    pub fn start() -> io::Result<Self> {
        Ok(VirtualAudio {
            inner: imp::Audio::start()?,
        })
    }

    /// Adds a virtual device.
    pub fn add(&self, spec: DeviceSpec) -> io::Result<VirtualDevice> {
        let channels = spec.channels.max(1);
        let rate = spec.rate.max(1);
        // 10 ms kept beyond PipeWire's chunks, as for an audio device.
        let buffer = Arc::new(FrameBuffer::new(channels, rate as usize / 100));
        let remove = self.inner.add(&spec, buffer.clone())?;
        Ok(VirtualDevice {
            shared: SharedBuffer {
                buffer,
                channels,
                rate,
            },
            _remove: remove,
        })
    }
}

#[cfg(target_os = "linux")]
mod imp {
    use std::collections::HashMap;
    use std::io;
    use std::sync::atomic::{AtomicPtr, AtomicU64, Ordering};
    use std::sync::{Arc, mpsc};
    use std::thread::JoinHandle;

    use avb_stream::audio::FrameBuffer;
    use pipewire as pw;
    use pw::properties::properties;
    use pw::spa;

    use super::DeviceSpec;

    /// What the loop's thread is asked to do.
    enum Command {
        Add {
            id: u64,
            spec: DeviceSpec,
            buffer: Arc<FrameBuffer>,
            done: mpsc::Sender<io::Result<()>>,
        },
        Remove(u64),
        Quit,
    }

    pub struct Audio {
        sender: pw::channel::Sender<Command>,
        thread: Option<JoinHandle<()>>,
        next: AtomicU64,
    }

    /// Removes its device when dropped.
    pub struct Remover {
        id: u64,
        sender: pw::channel::Sender<Command>,
    }

    impl Drop for Remover {
        fn drop(&mut self) {
            let _ = self.sender.send(Command::Remove(self.id));
        }
    }

    fn error(error: impl std::fmt::Display) -> io::Error {
        io::Error::other(format!("PipeWire: {error}"))
    }

    impl Audio {
        pub fn start() -> io::Result<Self> {
            let (sender, receiver) = pw::channel::channel::<Command>();
            let (started, ready) = mpsc::channel::<io::Result<()>>();
            let thread = std::thread::Builder::new()
                .name("pipewire".into())
                .spawn(move || run(receiver, &started))?;
            match ready.recv() {
                Ok(Ok(())) => Ok(Audio {
                    sender,
                    thread: Some(thread),
                    next: AtomicU64::new(1),
                }),
                Ok(Err(failure)) => {
                    let _ = thread.join();
                    Err(failure)
                }
                Err(_) => Err(error("its thread ended")),
            }
        }

        pub fn add(&self, spec: &DeviceSpec, buffer: Arc<FrameBuffer>) -> io::Result<Remover> {
            let id = self.next.fetch_add(1, Ordering::Relaxed);
            let (done, result) = mpsc::channel();
            self.sender
                .send(Command::Add {
                    id,
                    spec: spec.clone(),
                    buffer,
                    done,
                })
                .map_err(|_| error("its thread ended"))?;
            result
                .recv_timeout(std::time::Duration::from_secs(5))
                .map_err(|_| error("no answer"))??;
            Ok(Remover {
                id,
                sender: self.sender.clone(),
            })
        }
    }

    impl Drop for Audio {
        fn drop(&mut self) {
            let _ = self.sender.send(Command::Quit);
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }
    }

    /// A device the loop owns: its stream and the listener its callbacks
    /// live in.
    struct Device {
        _stream: pw::stream::StreamRc,
        _listener: pw::stream::StreamListener<()>,
    }

    fn run(receiver: pw::channel::Receiver<Command>, started: &mpsc::Sender<io::Result<()>>) {
        pw::init();
        let made = (|| {
            let mainloop = pw::main_loop::MainLoopRc::new(None).map_err(error)?;
            let context = pw::context::ContextRc::new(&mainloop, None).map_err(error)?;
            let core = context.connect_rc(None).map_err(error)?;
            Ok::<_, io::Error>((mainloop, context, core))
        })();
        let (mainloop, _context, core) = match made {
            Ok(made) => made,
            Err(failure) => {
                let _ = started.send(Err(failure));
                return;
            }
        };
        let _ = started.send(Ok(()));
        let devices: std::rc::Rc<std::cell::RefCell<HashMap<u64, Device>>> = Default::default();
        let quit = mainloop.clone();
        let owned = devices.clone();
        let _attached = receiver.attach(mainloop.loop_(), move |command| match command {
            Command::Add {
                id,
                spec,
                buffer,
                done,
            } => {
                let result = add(&core, &spec, buffer).map(|device| {
                    owned.borrow_mut().insert(id, device);
                });
                let _ = done.send(result);
            }
            Command::Remove(id) => {
                owned.borrow_mut().remove(&id);
            }
            Command::Quit => quit.quit(),
        });
        mainloop.run();
        devices.borrow_mut().clear();
    }

    /// The format a device takes: 32-bit float at its rate and channels,
    /// each channel auxiliary, as an interface's are.
    fn format(spec: &DeviceSpec) -> Vec<u8> {
        let mut info = spa::param::audio::AudioInfoRaw::new();
        info.set_format(spa::param::audio::AudioFormat::F32LE);
        info.set_rate(spec.rate);
        info.set_channels(u32::from(spec.channels));
        let mut position = [0; spa::param::audio::MAX_CHANNELS];
        for (index, slot) in position
            .iter_mut()
            .take(usize::from(spec.channels))
            .enumerate()
        {
            *slot = spa::sys::SPA_AUDIO_CHANNEL_AUX0 + index as u32;
        }
        info.set_position(position);
        spa::pod::serialize::PodSerializer::serialize(
            std::io::Cursor::new(Vec::new()),
            &spa::pod::Value::Object(spa::pod::Object {
                type_: spa::sys::SPA_TYPE_OBJECT_Format,
                id: spa::sys::SPA_PARAM_EnumFormat,
                properties: info.into(),
            }),
        )
        .map(|(cursor, _)| cursor.into_inner())
        .unwrap_or_default()
    }

    /// The frames of a cycle of PipeWire's graph, at the device's rate,
    /// as its driver's clock says; 10 ms before it says.
    #[allow(unsafe_code)]
    fn cycle_frames(position: &AtomicPtr<spa::sys::spa_io_position>, rate: u32) -> usize {
        let position = position.load(Ordering::Acquire);
        if position.is_null() {
            return rate as usize / 100;
        }
        // SAFETY: PipeWire gives the stream this area for as long as the
        // stream is connected, and the callbacks reading it run only
        // while it is; it changes the area only between cycles.
        let clock = unsafe { (*position).clock };
        let graph = clock.rate.denom.max(1);
        (clock.duration as u128 * u128::from(rate) / u128::from(graph)) as usize
    }

    fn add(
        core: &pw::core::CoreRc,
        spec: &DeviceSpec,
        buffer: Arc<FrameBuffer>,
    ) -> io::Result<Device> {
        let channels = usize::from(spec.channels.max(1));
        let stream = pw::stream::StreamRc::new(
            core.clone(),
            &spec.name,
            properties! {
                *pw::keys::MEDIA_TYPE => "Audio",
                *pw::keys::MEDIA_CATEGORY => if spec.playback { "Capture" } else { "Playback" },
                *pw::keys::MEDIA_CLASS => if spec.playback { "Audio/Sink" } else { "Audio/Source" },
                *pw::keys::NODE_NAME => spec.name.as_str(),
                *pw::keys::NODE_DESCRIPTION => spec.description.as_str(),
                "node.virtual" => "true",
            },
        )
        .map_err(error)?;
        let position = Arc::new(AtomicPtr::new(std::ptr::null_mut()));
        let watching = position.clone();
        let rate = spec.rate;
        let playback = spec.playback;
        let mut samples: Vec<f32> = Vec::new();
        let listener = stream
            .add_local_listener_with_user_data(())
            .io_changed(move |_, _, id, area, _| {
                if id == spa::sys::SPA_IO_Position {
                    watching.store(area.cast(), Ordering::Release);
                }
            })
            .process(move |stream, _| {
                let Some(mut taken) = stream.dequeue_buffer() else {
                    return;
                };
                let datas = taken.datas_mut();
                let Some(data) = datas.first_mut() else {
                    return;
                };
                let stride = 4 * channels;
                if playback {
                    // Apps played into it: their frames to the stream.
                    let size = data.chunk().size() as usize;
                    let Some(bytes) = data.data() else {
                        return;
                    };
                    let bytes = &bytes[..size.min(bytes.len())];
                    samples.clear();
                    samples.extend(
                        bytes
                            .as_chunks::<4>()
                            .0
                            .iter()
                            .map(|sample| f32::from_le_bytes(*sample)),
                    );
                    samples.truncate(samples.len() - samples.len() % channels);
                    buffer.device_push(&samples);
                } else {
                    // Apps record from it: a cycle of the stream's frames.
                    let wanted = cycle_frames(&position, rate);
                    let written = {
                        let Some(bytes) = data.data() else {
                            return;
                        };
                        let frames = wanted.min(bytes.len() / stride);
                        samples.resize(frames * channels, 0.0);
                        buffer.device_pull(&mut samples, None);
                        for (slot, sample) in bytes.as_chunks_mut::<4>().0.iter_mut().zip(&samples)
                        {
                            slot.copy_from_slice(&sample.to_le_bytes());
                        }
                        frames
                    };
                    let chunk = data.chunk_mut();
                    *chunk.offset_mut() = 0;
                    *chunk.stride_mut() = stride as i32;
                    *chunk.size_mut() = (stride * written) as u32;
                }
            })
            .register()
            .map_err(error)?;
        let values = format(spec);
        let pod = spa::pod::Pod::from_bytes(&values).ok_or_else(|| error("no format"))?;
        let mut params = [pod];
        stream
            .connect(
                if spec.playback {
                    spa::utils::Direction::Input
                } else {
                    spa::utils::Direction::Output
                },
                None,
                pw::stream::StreamFlags::MAP_BUFFERS | pw::stream::StreamFlags::RT_PROCESS,
                &mut params,
            )
            .map_err(error)?;
        Ok(Device {
            _stream: stream,
            _listener: listener,
        })
    }
}

#[cfg(not(target_os = "linux"))]
mod imp {
    use std::io;
    use std::sync::Arc;

    use avb_stream::audio::FrameBuffer;

    use super::DeviceSpec;

    pub struct Audio;

    pub struct Remover;

    impl Audio {
        pub fn start() -> io::Result<Self> {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "virtual audio devices need PipeWire",
            ))
        }

        pub fn add(&self, _: &DeviceSpec, _: Arc<FrameBuffer>) -> io::Result<Remover> {
            Ok(Remover)
        }
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::process::Command;
    use std::time::{Duration, Instant};

    /// The loudest of `samples`.
    fn peak(samples: &[f32]) -> f32 {
        samples
            .iter()
            .fold(0.0, |most, sample| most.max(sample.abs()))
    }

    /// On a computer running PipeWire: what an app plays into the playback
    /// device reaches its buffer, and what the buffer holds for the
    /// recording device reaches an app recording it.
    #[test]
    #[ignore = "needs PipeWire, pw-play and pw-record"]
    fn apps_play_into_and_record_from_virtual_devices() {
        let audio = VirtualAudio::start().expect("PipeWire runs");
        let spec = |name: &str, playback| DeviceSpec {
            name: name.into(),
            description: name.into(),
            channels: 2,
            rate: 48_000,
            playback,
        };
        let playback = audio.add(spec("triib-test-out", true)).expect("added");
        let recording = audio.add(spec("triib-test-in", false)).expect("added");
        let dir = std::env::temp_dir();
        // A second of 440 Hz at half scale, 16-bit stereo.
        let tone = dir.join("triib-test-tone.wav");
        let frames = 48_000u32;
        let mut wav = Vec::new();
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&(36 + frames * 4).to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
        wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&2u16.to_le_bytes());
        wav.extend_from_slice(&48_000u32.to_le_bytes());
        wav.extend_from_slice(&(48_000u32 * 4).to_le_bytes());
        wav.extend_from_slice(&4u16.to_le_bytes());
        wav.extend_from_slice(&16u16.to_le_bytes());
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&(frames * 4).to_le_bytes());
        for frame in 0..frames {
            let value =
                (16_384.0 * (std::f32::consts::TAU * 440.0 * frame as f32 / 48_000.0).sin()) as i16;
            wav.extend_from_slice(&value.to_le_bytes());
            wav.extend_from_slice(&value.to_le_bytes());
        }
        std::fs::write(&tone, wav).expect("written");
        let shared = playback.shared();
        let mut player = Command::new("pw-play")
            .args(["--target", "triib-test-out"])
            .arg(&tone)
            .spawn()
            .expect("pw-play");
        let mut loudest = 0.0f32;
        let mut taken = vec![0.0f32; 2 * 480];
        let until = Instant::now() + Duration::from_millis(1500);
        while Instant::now() < until {
            std::thread::sleep(Duration::from_millis(10));
            shared.buffer.pull(&mut taken);
            loudest = loudest.max(peak(&taken));
        }
        let _ = player.wait();
        assert!(loudest > 0.4, "played at half scale, heard {loudest}");
        // The recording device gives what the stream writes.
        let recorded = dir.join("triib-test-recorded.raw");
        let feeding = recording.shared();
        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let feeder = {
            let stop = stop.clone();
            std::thread::spawn(move || {
                let chunk = vec![0.25f32; 2 * 480];
                while !stop.load(std::sync::atomic::Ordering::Relaxed) {
                    if feeding.buffer.level() < 2_000 {
                        feeding.buffer.push(&chunk);
                    }
                    std::thread::sleep(Duration::from_millis(5));
                }
            })
        };
        let _ = Command::new("timeout")
            .args(["1.5", "pw-record", "--target", "triib-test-in"])
            .args([
                "--rate",
                "48000",
                "--channels",
                "2",
                "--format",
                "f32",
                "--raw",
            ])
            .arg(&recorded)
            .status();
        stop.store(true, std::sync::atomic::Ordering::Relaxed);
        let _ = feeder.join();
        let bytes = std::fs::read(&recorded).unwrap_or_default();
        let samples: Vec<f32> = bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|sample| f32::from_le_bytes(*sample))
            .collect();
        assert!(samples.len() > 48_000, "recorded {} samples", samples.len());
        assert!(
            (peak(&samples) - 0.25).abs() < 0.01,
            "peak {}",
            peak(&samples)
        );
        let _ = std::fs::remove_file(tone);
        let _ = std::fs::remove_file(recorded);
    }
}
