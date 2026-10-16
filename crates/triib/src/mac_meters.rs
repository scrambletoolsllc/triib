//! The Mac's own entity's meters: how loud the first two channels of each
//! of its audio streams are, measured in a helper, `triib --meters`, from
//! the Core Audio device macOS makes of the entity: its input for what
//! the entity receives, and taps on its outputs for what apps send it, as
//! the monitor hears them. The app starts the helper again when it ends
//! or stops reporting, as a Core Audio call can block for good when the
//! AVB device goes.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use atdecc::EntityId;
use triib_endpointd::meters::{EVERY, Meters, StreamMeter};
use triib_stream::audio::{Input, Source};

/// The flag that runs this binary as the helper.
pub const FLAG: &str = "--meters";

/// The rate the helper reads at, between the devices' own.
const RATE: u32 = 48_000;
/// Frames read at a time: 5 ms.
const CHUNK: usize = 240;
/// How long without a report before the helper counts as hung.
const HUNG: Duration = Duration::from_secs(5);
/// How long the helper waits for a device that is not there, and the app
/// before starting a helper again.
const RETRY: Duration = Duration::from_secs(1);
const RESTART: Duration = Duration::from_secs(3);
/// How often the helper looks at the device.
const LOOK: Duration = Duration::from_secs(1);
/// Reads in a second while an input had nothing, past which it counts as
/// stalled: the device went, or stopped delivering.
const STALLED: u64 = 100;

/// What the helper reads of one side of the device: its input, or the
/// taps on its outputs, each stream's channels after the one before's.
struct Side {
    input: Input,
    /// Whether these are the entity's input streams.
    received: bool,
    streams: Vec<u16>,
    channels: usize,
    samples: Vec<f32>,
    /// Each stream's loudest samples in its first two channels since they
    /// were last reported.
    peaks: Vec<[f32; 2]>,
    underruns: u64,
}

impl Side {
    fn open(source: &Source, streams: Vec<u16>, received: bool) -> Result<Self, String> {
        let channels: u16 = streams.iter().sum();
        let input = Input::open(source, channels, RATE).map_err(|error| error.to_string())?;
        Ok(Side {
            underruns: input.counts().1,
            input,
            received,
            peaks: vec![[0.0; 2]; streams.len()],
            streams,
            channels: usize::from(channels),
            samples: vec![0.0; CHUNK * usize::from(channels)],
        })
    }

    /// Reads a chunk, raising each stream's peaks.
    fn read(&mut self) {
        self.input.read(&mut self.samples);
        for frame in self.samples.chunks_exact(self.channels.max(1)) {
            let mut first = 0;
            for (&channels, peaks) in self.streams.iter().zip(&mut self.peaks) {
                let channels = usize::from(channels);
                let stream = frame.get(first..first + channels).unwrap_or_default();
                for (peak, sample) in peaks.iter_mut().zip(stream) {
                    *peak = peak.max(sample.abs());
                }
                first += channels;
            }
        }
    }

    /// Each stream's peaks since the last time, by its place on its side
    /// of the device, starting them again from silence.
    fn take(&mut self, entity_id: &str, meters: &mut Vec<StreamMeter>) {
        for (index, peaks) in self.peaks.iter_mut().enumerate() {
            meters.push(StreamMeter {
                entity_id: entity_id.to_owned(),
                input: self.received,
                index: index as u16,
                peaks: std::mem::take(peaks),
            });
        }
    }

    /// Whether the input found nothing too often since last asked.
    fn stalled(&mut self) -> bool {
        let now = self.input.counts().1;
        let stalled = now.saturating_sub(self.underruns) > STALLED;
        self.underruns = now;
        stalled
    }
}

/// Runs as the helper with the arguments after the flag: the Core Audio
/// device and the entity's ID. Reports until killed.
pub fn serve(arguments: &[String]) -> i32 {
    let [device, entity_id] = arguments else {
        eprintln!("usage: triib {FLAG} DEVICE ENTITY-ID");
        return 2;
    };
    // The app holds the other end of stdin: once it closes, however the
    // app ended, this ends too.
    std::thread::spawn(|| {
        let mut sink = [0u8; 64];
        while matches!(std::io::Read::read(&mut std::io::stdin(), &mut sink), Ok(read) if read > 0)
        {
        }
        std::process::exit(0);
    });
    let mut sequence = 0;
    loop {
        let received = crate::mac::input_streams(device);
        let sent = crate::mac::output_streams(device);
        let mut sides = Vec::new();
        if !received.is_empty() {
            let source = Source::Device {
                name: Some(device.clone()),
                first_channel: 0,
            };
            match Side::open(&source, received.clone(), true) {
                Ok(side) => sides.push(side),
                Err(error) => eprintln!("{device}'s input: {error}"),
            }
        }
        // What apps send is heard through taps, read into a buffer this
        // takes from as from any other input.
        let mut tap = None;
        if !sent.is_empty() && crate::mac::taps_supported() {
            let streams: Vec<u16> = (0..sent.len() as u16).collect();
            let opened = crate::mac::SentTap::open_streams(device, &streams).and_then(|mut tap| {
                let shared = tap.start(0, tap.channels)?;
                Ok((tap, shared))
            });
            match opened.and_then(|(opened, shared)| {
                Ok((
                    opened,
                    Side::open(&Source::Shared(shared), sent.clone(), false)?,
                ))
            }) {
                Ok((opened, side)) => {
                    tap = Some(opened);
                    sides.push(side);
                }
                Err(error) => eprintln!("{device}'s outputs: {error}"),
            }
        }
        if sides.is_empty() {
            report(&mut sequence, Vec::new());
            std::thread::sleep(RETRY);
            continue;
        }
        measure(
            device,
            entity_id,
            &mut sides,
            &mut sequence,
            (&received, &sent),
        );
        // The taps' buffer is read until its reader goes.
        drop(sides);
        drop(tap);
    }
}

/// Reads `sides` and reports their streams' peaks each [`EVERY`] until an
/// input stalls or the device's streams are no longer `layout`.
fn measure(
    device: &str,
    entity_id: &str,
    sides: &mut [Side],
    sequence: &mut u64,
    layout: (&[u16], &[u16]),
) {
    let started = Instant::now();
    let mut moved = 0u64;
    let mut reported = Instant::now();
    let mut looked = Instant::now();
    loop {
        let due = (started.elapsed().as_secs_f64() * f64::from(RATE)) as u64;
        while moved + CHUNK as u64 <= due {
            for side in sides.iter_mut() {
                side.read();
            }
            moved += CHUNK as u64;
        }
        if reported.elapsed() >= EVERY {
            reported = Instant::now();
            let mut meters = Vec::new();
            for side in sides.iter_mut() {
                side.take(entity_id, &mut meters);
            }
            report(sequence, meters);
        }
        if looked.elapsed() >= LOOK {
            looked = Instant::now();
            let stalled = sides.iter_mut().any(Side::stalled);
            let now = (
                crate::mac::input_streams(device),
                crate::mac::output_streams(device),
            );
            if stalled || (now.0.as_slice(), now.1.as_slice()) != layout {
                report(sequence, Vec::new());
                return;
            }
        }
        std::thread::sleep(Duration::from_millis(2));
    }
}

/// Tells the app, one JSON line each time.
fn report(sequence: &mut u64, streams: Vec<StreamMeter>) {
    *sequence += 1;
    let meters = Meters {
        sequence: *sequence,
        streams,
    };
    if let Ok(line) = serde_json::to_string(&meters) {
        let mut out = std::io::stdout().lock();
        let _ = writeln!(out, "{line}");
        let _ = out.flush();
    }
}

/// While the subscription runs, what the helper measures of the Mac's own
/// entity, whose Core Audio device is the one named: the helper started
/// again when it ends or stops reporting, and stopped with the
/// subscription. `None` while it does not report.
pub fn readings(
    of: &(String, EntityId),
) -> impl iced::futures::Stream<Item = Option<Meters>> + use<> {
    let (device, entity_id) = of.clone();
    let (mut sender, receiver) = iced::futures::channel::mpsc::channel(1);
    let _ = std::thread::Builder::new()
        .name("mac-meters".into())
        .spawn(move || {
            while !sender.is_closed() {
                let spawned = std::env::current_exe().and_then(|program| {
                    Command::new(program)
                        .arg(FLAG)
                        .arg(&device)
                        .arg(entity_id.to_string())
                        // Held open by the child's handle while it runs.
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::null())
                        .spawn()
                });
                let Ok(mut child) = spawned else {
                    std::thread::sleep(RESTART);
                    continue;
                };
                let (lines, said) = mpsc::channel();
                if let Some(out) = child.stdout.take() {
                    std::thread::spawn(move || {
                        for line in BufReader::new(out).lines().map_while(Result::ok) {
                            if lines.send(line).is_err() {
                                return;
                            }
                        }
                    });
                }
                // Until it ends, says nothing for too long, or the
                // subscription stops.
                let mut heard = Instant::now();
                while !sender.is_closed() && heard.elapsed() < HUNG {
                    match said.recv_timeout(Duration::from_millis(250)) {
                        Ok(line) => {
                            heard = Instant::now();
                            if let Ok(meters) = serde_json::from_str::<Meters>(&line) {
                                // One the app is busy with, the next
                                // stands for.
                                let _ = sender.try_send(Some(meters));
                            }
                        }
                        Err(mpsc::RecvTimeoutError::Timeout) => {}
                        Err(mpsc::RecvTimeoutError::Disconnected) => break,
                    }
                }
                let _ = child.kill();
                let _ = child.wait();
                if !sender.is_closed() {
                    let _ = sender.try_send(None);
                    std::thread::sleep(RESTART);
                }
            }
        });
    receiver
}
