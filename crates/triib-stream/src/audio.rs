//! Where a talker's samples come from and where a listener's go: the
//! computer's audio devices through cpal, a test tone, or nothing.
//!
//! A device runs on its own clock, not gPTP's, so the samples pass
//! through a buffer and are resampled on the stream's side of it, by a
//! ratio a slow controller steers to keep the buffer near its fill
//! level: the device's drift, a few hundred parts per million at most,
//! is taken up without a sample frame dropped or played twice.
//!
//! A listener's output steers by time instead, once it knows when the
//! device plays: each sample then plays at its presentation time, or a
//! fixed delay after it where the device's own latency is longer than
//! the stream allows.

use std::collections::VecDeque;
use std::f32::consts::TAU;
use std::io;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError, mpsc};
use std::thread::JoinHandle;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample};

/// Where a talker's samples come from.
#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    Silence,
    /// A sine on every channel, its level from 0 to 1.
    Tone {
        hertz: f32,
        level: f32,
    },
    /// An audio input, by name or the default one, the stream's channels
    /// taken from its channel `first_channel` on.
    Device {
        name: Option<String>,
        first_channel: u16,
    },
    /// A buffer another part of the program fills as a device would, such
    /// as a virtual audio device apps play into.
    Shared(SharedBuffer),
}

/// A buffer another part of the program fills or drains as an audio
/// device would, on its own clock: `channels` channels at `rate` hertz.
#[derive(Clone)]
pub struct SharedBuffer {
    pub buffer: Arc<FrameBuffer>,
    pub channels: u16,
    pub rate: u32,
}

impl PartialEq for SharedBuffer {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.buffer, &other.buffer)
    }
}

impl std::fmt::Debug for SharedBuffer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "SharedBuffer({} channels at {} Hz)",
            self.channels, self.rate
        )
    }
}

/// Where a listener's samples go.
#[derive(Debug, Clone, PartialEq)]
pub enum Sink {
    /// Nowhere, though their level is still measured.
    Discard,
    /// An audio output, by name or the default one, the stream's channels
    /// played on its channel `first_channel` on.
    Device {
        name: Option<String>,
        first_channel: u16,
    },
    /// A buffer another part of the program drains as a device would, such
    /// as a virtual audio device apps record from.
    Shared(SharedBuffer),
}

/// Interleaved sample frames passed between an audio device's callback
/// and a stream's thread.
pub struct FrameBuffer {
    held: Mutex<Held>,
    channels: usize,
    /// The fill kept beyond the device's own chunks, in frames.
    margin: usize,
    /// Frames read while it was empty, and times frames were dropped as
    /// it held four times what it keeps.
    pub underruns: AtomicU64,
    pub adjustments: AtomicU64,
}

struct Held {
    samples: VecDeque<f32>,
    /// Frames ever pushed on the stream's side: the oldest held is frame
    /// `pushed` less those held.
    pushed: u64,
    /// Frames held beyond the usual most, for a listener's delay.
    more: usize,
    /// The fill right after the device's last chunk, which the controller
    /// steers by.
    sample: Option<DeviceSample>,
}

/// The buffer's fill right after one of the device's chunks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceSample {
    /// Counts the device's chunks, telling a new sample from the last.
    pub chunks: u64,
    pub level: usize,
    pub chunk: usize,
    /// Frames pushed by then, so the oldest held is frame `pushed -
    /// level`.
    pub pushed: u64,
    /// When the oldest frame held plays, in monotonic nanoseconds, where
    /// the device says.
    pub plays_at: Option<i64>,
}

impl FrameBuffer {
    pub fn new(channels: u16, margin: usize) -> Self {
        let channels = usize::from(channels).max(1);
        FrameBuffer {
            held: Mutex::new(Held {
                samples: VecDeque::with_capacity(4 * margin * channels),
                pushed: 0,
                more: 0,
                sample: None,
            }),
            channels,
            margin: margin.max(1),
            underruns: AtomicU64::new(0),
            adjustments: AtomicU64::new(0),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Held> {
        self.held.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Adds interleaved frames, keeping no more than four times the
    /// margin and the device's chunk.
    pub fn push(&self, samples: &[f32]) {
        let mut held = self.lock();
        self.push_into(&mut held, samples);
    }

    fn push_into(&self, held: &mut Held, samples: &[f32]) {
        held.samples.extend(samples);
        held.pushed += (samples.len() / self.channels) as u64;
        let chunk = held.sample.map_or(0, |sample| sample.chunk);
        let most = (4 * (self.margin + chunk) + held.more) * self.channels;
        if held.samples.len() > most {
            let extra = held.samples.len() - most;
            held.samples.drain(..extra - extra % self.channels);
            self.adjustments.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Fills `out` with the oldest frames, silence where there are none.
    pub fn pull(&self, out: &mut [f32]) {
        let mut held = self.lock();
        self.pull_from(&mut held, out);
    }

    fn pull_from(&self, held: &mut Held, out: &mut [f32]) {
        let mut short = false;
        for slot in out.iter_mut() {
            *slot = held.samples.pop_front().unwrap_or_else(|| {
                short = true;
                0.0
            });
        }
        if short {
            self.underruns.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Frames ever pushed on the stream's side.
    pub fn pushed(&self) -> u64 {
        self.lock().pushed
    }

    /// Holds as many as `frames` more before dropping the oldest.
    pub fn hold_more(&self, frames: usize) {
        let mut held = self.lock();
        held.more = held.more.max(frames);
    }

    /// Drops the oldest `frames`, as many as are held at most.
    pub fn drop_oldest(&self, frames: usize) {
        let mut held = self.lock();
        let samples = (frames * self.channels).min(held.samples.len());
        held.samples.drain(..samples);
    }

    /// The device's side: adds one chunk, and notes the fill after it.
    pub fn device_push(&self, samples: &[f32]) {
        let mut held = self.lock();
        self.push_into(&mut held, samples);
        self.note(&mut held, samples.len(), None);
    }

    /// The device's side: takes one chunk, and notes the fill after it
    /// and, where the device says, when the oldest frame left plays.
    pub fn device_pull(&self, out: &mut [f32], plays_at: Option<i64>) {
        let mut held = self.lock();
        self.pull_from(&mut held, out);
        self.note(&mut held, out.len(), plays_at);
    }

    fn note(&self, held: &mut Held, samples: usize, plays_at: Option<i64>) {
        let chunks = held.sample.map_or(1, |sample| sample.chunks + 1);
        held.sample = Some(DeviceSample {
            chunks,
            level: held.samples.len() / self.channels,
            chunk: samples / self.channels,
            pushed: held.pushed,
            plays_at,
        });
    }

    /// The fill right after the device's last chunk.
    pub fn device_sample(&self) -> Option<DeviceSample> {
        self.lock().sample
    }

    /// The frames held.
    pub fn level(&self) -> usize {
        self.lock().samples.len() / self.channels
    }
}

/// The controller's gains, for each of the device's chunks: how much of
/// each new fill its smoothed error takes, the ratio's change for an
/// error of a whole chunk, and the integral's growth for it. Near
/// critically damped, settling within 40 ppm in about 30 s of 1024-frame
/// chunks, faster with smaller ones.
const SMOOTHING: f64 = 0.02;
const PROPORTIONAL: f64 = 0.0051;
const INTEGRAL: f64 = 1e-5;
/// The most the ratio strays from 1.
const MOST_DRIFT: f64 = 0.002;

/// Which side of the buffer the device is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceSide {
    /// An input: the device pushes, the stream pulls.
    Pushes,
    /// An output: the stream pushes, the device pulls.
    Pulls,
}

/// Resamples interleaved frames from one clock to another close to it, by
/// cubic interpolation between the input frames, at a ratio steered from
/// the buffer's fill after each of the device's chunks.
pub struct Drift {
    channels: usize,
    side: DeviceSide,
    /// Input frames for each output frame were both clocks exact: 1
    /// when the device runs at the stream's rate, 2 for a 96 kHz stream
    /// to a 48 kHz device.
    nominal: f64,
    /// Input frames: the one before the current position, then those
    /// after.
    held: VecDeque<f32>,
    /// Where the next output frame lies past the second held frame, from
    /// 0 to 1.
    position: f64,
    /// Input frames for each output frame.
    ratio: f64,
    /// The last device chunk steered by, the smoothed error and the
    /// integral.
    chunks: u64,
    error: Option<f64>,
    integral: f64,
}

impl Drift {
    pub fn new(channels: u16, side: DeviceSide, nominal: f64) -> Self {
        Drift {
            channels: usize::from(channels).max(1),
            side,
            nominal,
            held: VecDeque::new(),
            position: 0.0,
            ratio: nominal,
            chunks: 0,
            error: None,
            integral: 0.0,
        }
    }

    /// Input frames for each output frame now.
    pub fn ratio(&self) -> f64 {
        self.ratio
    }

    /// How far the device's clock runs from the stream's, in parts per
    /// million, as the ratio follows it.
    pub fn device_ppm(&self) -> i32 {
        let relative = self.ratio / self.nominal;
        let ppm = match self.side {
            // A fast input device gives more frames than the stream takes.
            DeviceSide::Pushes => relative - 1.0,
            // A slow output device takes fewer than the stream gives.
            DeviceSide::Pulls => 1.0 / relative - 1.0,
        };
        (ppm * 1e6).round() as i32
    }

    /// Steers the ratio from the fill after the device's last chunk, once
    /// for each chunk: a fuller buffer than kept takes more input frames
    /// for each output frame, whichever side the device is on.
    pub fn steer(&mut self, buffer: &FrameBuffer) {
        if let Some(sample) = self.new_sample(buffer) {
            let excess = sample.level as f64 - self.kept(buffer, &sample) as f64;
            self.follow(excess, sample.chunk);
        }
    }

    /// The device's last chunk, when not yet steered by.
    fn new_sample(&mut self, buffer: &FrameBuffer) -> Option<DeviceSample> {
        let sample = buffer.device_sample()?;
        if sample.chunks == self.chunks {
            return None;
        }
        self.chunks = sample.chunks;
        Some(sample)
    }

    /// The fill kept right after a chunk: after an input's the buffer
    /// holds it on top of what is kept; after an output's, just what is
    /// kept.
    fn kept(&self, buffer: &FrameBuffer, sample: &DeviceSample) -> usize {
        match self.side {
            DeviceSide::Pushes => buffer.margin + sample.chunk,
            DeviceSide::Pulls => buffer.margin,
        }
    }

    /// Steers by `excess` frames held beyond what is wanted, after a
    /// device chunk of `chunk` frames.
    fn follow(&mut self, excess: f64, chunk: usize) {
        let error = excess / chunk.max(1) as f64;
        let smoothed = self.error.get_or_insert(error);
        *smoothed += SMOOTHING * (error - *smoothed);
        self.integral = (self.integral + INTEGRAL * *smoothed).clamp(-MOST_DRIFT, MOST_DRIFT);
        self.ratio = self.nominal
            * (1.0 + PROPORTIONAL * *smoothed + self.integral)
                .clamp(1.0 - MOST_DRIFT, 1.0 + MOST_DRIFT);
    }

    fn held_frames(&self) -> usize {
        self.held.len() / self.channels
    }

    /// Input frames taken but not yet made into output, counted from
    /// where the next output frame lies.
    fn pending(&self) -> f64 {
        (self.held_frames() as f64 - 1.0 - self.position).max(0.0)
    }

    /// The input frames to add before `frames` output frames can be made.
    pub fn wanted(&self, frames: usize) -> usize {
        if frames == 0 {
            return 0;
        }
        let last = self.position + (frames - 1) as f64 * self.ratio;
        (last as usize + 4).saturating_sub(self.held_frames())
    }

    pub fn push(&mut self, input: &[f32]) {
        self.held.extend(input);
    }

    /// Makes output frames while there are input frames enough, into
    /// `out`, up to `most` of them.
    pub fn make(&mut self, out: &mut Vec<f32>, most: usize) {
        let channels = self.channels;
        let mut made = 0;
        while made < most && self.position as usize + 4 <= self.held_frames() {
            let base = self.position as usize;
            let t = (self.position - base as f64) as f32;
            for channel in 0..channels {
                let at = |frame: usize| self.held[(base + frame) * channels + channel];
                let (before, from, to, after) = (at(0), at(1), at(2), at(3));
                // Catmull-Rom between `from` and `to`.
                let a = -0.5 * before + 1.5 * from - 1.5 * to + 0.5 * after;
                let b = before - 2.5 * from + 2.0 * to - 0.5 * after;
                let c = -0.5 * before + 0.5 * to;
                out.push(((a * t + b) * t + c) * t + from);
            }
            made += 1;
            self.position += self.ratio;
            let passed = self.position as usize;
            if passed > 0 {
                self.held.drain(..passed * channels);
                self.position -= passed as f64;
            }
        }
    }
}

/// A thread holding an open cpal stream until dropped.
struct DeviceThread {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl Drop for DeviceThread {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.thread().unpark();
            let _ = handle.join();
        }
    }
}

fn device_error(error: impl std::fmt::Display) -> io::Error {
    io::Error::other(error.to_string())
}

/// The input or output device with `name`, or the default one.
fn find_device(name: Option<&str>, input: bool) -> io::Result<cpal::Device> {
    let host = cpal::default_host();
    let found = match name {
        Some(name) => {
            let devices = if input {
                host.input_devices()
            } else {
                host.output_devices()
            }
            .map_err(device_error)?;
            devices
                .into_iter()
                .find(|device| device.description().is_ok_and(|about| about.name() == name))
        }
        None if input => host.default_input_device(),
        None => host.default_output_device(),
    };
    found.ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            match name {
                Some(name) => format!("no audio device named {name}"),
                None => "no default audio device".to_owned(),
            },
        )
    })
}

/// The rate to open a device at: the stream's when the device takes it,
/// else the device's own, which the resampler then converts from or to.
fn device_rate(device: &cpal::Device, input: bool, wanted: u32) -> io::Result<u32> {
    let default = if input {
        device.default_input_config()
    } else {
        device.default_output_config()
    }
    .map_err(device_error)?;
    if default.sample_rate() == wanted {
        return Ok(wanted);
    }
    let ranges: Vec<cpal::SupportedStreamConfigRange> = if input {
        device.supported_input_configs().map(Iterator::collect)
    } else {
        device.supported_output_configs().map(Iterator::collect)
    }
    .unwrap_or_default();
    let supported = ranges.iter().any(|range| {
        range.channels() == default.channels()
            && (range.min_sample_rate()..=range.max_sample_rate()).contains(&wanted)
    });
    Ok(if supported {
        wanted
    } else {
        default.sample_rate()
    })
}

/// The names of the audio devices: inputs, then outputs.
pub fn device_names() -> (Vec<String>, Vec<String>) {
    let host = cpal::default_host();
    let named = |device: cpal::Device| Some(device.description().ok()?.name().to_owned());
    let inputs = host
        .input_devices()
        .map(|devices| devices.filter_map(named).collect())
        .unwrap_or_default();
    let outputs = host
        .output_devices()
        .map(|devices| devices.filter_map(named).collect())
        .unwrap_or_default();
    (inputs, outputs)
}

/// Opens a device on a thread of its own, which `build` gives a stream to
/// play until the thread is told to stop.
fn device_thread(
    name: String,
    build: impl FnOnce() -> io::Result<cpal::Stream> + Send + 'static,
) -> io::Result<DeviceThread> {
    let stop = Arc::new(AtomicBool::new(false));
    let stopping = stop.clone();
    let (opened, started) = mpsc::channel();
    let handle = std::thread::Builder::new().name(name).spawn(move || {
        let stream = match build().and_then(|stream| {
            stream.play().map_err(device_error)?;
            Ok(stream)
        }) {
            Ok(stream) => {
                let _ = opened.send(Ok(()));
                stream
            }
            Err(error) => {
                let _ = opened.send(Err(error));
                return;
            }
        };
        while !stopping.load(Ordering::Relaxed) {
            std::thread::park_timeout(Duration::from_millis(200));
        }
        drop(stream);
    })?;
    started
        .recv()
        .unwrap_or_else(|_| Err(io::Error::other("the audio thread stopped")))?;
    Ok(DeviceThread {
        stop,
        handle: Some(handle),
    })
}

fn input_stream<T>(
    device: &cpal::Device,
    config: cpal::StreamConfig,
    channels: usize,
    first: usize,
    buffer: Arc<FrameBuffer>,
) -> io::Result<cpal::Stream>
where
    T: SizedSample,
    f32: FromSample<T>,
{
    let device_channels = usize::from(config.channels).max(1);
    let mut frames = Vec::new();
    device
        .build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                frames.clear();
                for frame in data.chunks_exact(device_channels) {
                    for channel in 0..channels {
                        frames.push(
                            frame
                                .get(first + channel)
                                .map_or(0.0, |sample| sample.to_sample::<f32>()),
                        );
                    }
                }
                buffer.device_push(&frames);
            },
            |_| {},
            None,
        )
        .map_err(device_error)
}

fn output_stream<T>(
    device: &cpal::Device,
    config: cpal::StreamConfig,
    channels: usize,
    first: usize,
    buffer: Arc<FrameBuffer>,
) -> io::Result<cpal::Stream>
where
    T: SizedSample + FromSample<f32>,
{
    let device_channels = usize::from(config.channels).max(1);
    let rate = i64::from(config.sample_rate.max(1));
    let mut frames = Vec::new();
    device
        .build_output_stream(
            config,
            move |data: &mut [T], info: &cpal::OutputCallbackInfo| {
                let count = data.len() / device_channels;
                frames.resize(count * channels, 0.0);
                // The chunk's first frame plays the device's latency from
                // now, the frame after its last a chunk later.
                let stamp = info.timestamp();
                let latency = stamp.playback.duration_since(stamp.callback);
                let plays_at = avb_net::clock::monotonic_now() + latency;
                let after = plays_at.as_nanos() as i64 + count as i64 * 1_000_000_000 / rate;
                buffer.device_pull(&mut frames, Some(after));
                for (frame, samples) in data
                    .chunks_exact_mut(device_channels)
                    .zip(frames.chunks_exact(channels.max(1)))
                {
                    for (channel, slot) in frame.iter_mut().enumerate() {
                        let sample = channel
                            .checked_sub(first)
                            .and_then(|stream| samples.get(stream))
                            .copied()
                            .unwrap_or(0.0);
                        *slot = T::from_sample(sample);
                    }
                }
            },
            |_| {},
            None,
        )
        .map_err(device_error)
}

/// The stream's samples from its source.
pub struct Input {
    kind: InputKind,
    channels: usize,
}

enum InputKind {
    Silence,
    Tone {
        phase: f32,
        step: f32,
        level: f32,
    },
    Device {
        buffer: Arc<FrameBuffer>,
        drift: Drift,
        /// Input frames on their way to the resampler, and its output.
        taken: Vec<f32>,
        made: Vec<f32>,
        _thread: Option<DeviceThread>,
    },
}

impl Input {
    /// Opens `source` for `channels` channels at `rate` hertz.
    pub fn open(source: &Source, channels: u16, rate: u32) -> io::Result<Self> {
        let kind = match source {
            Source::Silence => InputKind::Silence,
            Source::Tone { hertz, level } => InputKind::Tone {
                phase: 0.0,
                step: TAU * hertz / rate as f32,
                level: level.clamp(0.0, 1.0),
            },
            Source::Device {
                name,
                first_channel,
            } => {
                let device = find_device(name.as_deref(), true)?;
                let device_rate = device_rate(&device, true, rate)?;
                // 10 ms kept beyond the device's chunks.
                let margin = device_rate as usize / 100;
                let buffer = Arc::new(FrameBuffer::new(channels, margin));
                let filling = buffer.clone();
                let first = usize::from(*first_channel);
                let wanted = usize::from(channels);
                let thread = device_thread("audio input".into(), move || {
                    let supported = device.default_input_config().map_err(device_error)?;
                    let mut config = supported.config();
                    config.sample_rate = device_rate;
                    match supported.sample_format() {
                        cpal::SampleFormat::F32 => {
                            input_stream::<f32>(&device, config, wanted, first, filling)
                        }
                        cpal::SampleFormat::I16 => {
                            input_stream::<i16>(&device, config, wanted, first, filling)
                        }
                        cpal::SampleFormat::I32 => {
                            input_stream::<i32>(&device, config, wanted, first, filling)
                        }
                        other => Err(io::Error::new(
                            io::ErrorKind::Unsupported,
                            format!("samples in {other}"),
                        )),
                    }
                })?;
                InputKind::Device {
                    buffer,
                    drift: Drift::new(
                        channels,
                        DeviceSide::Pushes,
                        f64::from(device_rate) / f64::from(rate),
                    ),
                    taken: Vec::new(),
                    made: Vec::new(),
                    _thread: Some(thread),
                }
            }
            Source::Shared(shared) => {
                if shared.channels != channels {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "a shared buffer of other channels than the stream's",
                    ));
                }
                // What it held from before goes, so the stream starts from
                // now rather than behind.
                shared.buffer.drop_oldest(shared.buffer.level());
                InputKind::Device {
                    buffer: shared.buffer.clone(),
                    drift: Drift::new(
                        channels,
                        DeviceSide::Pushes,
                        f64::from(shared.rate) / f64::from(rate),
                    ),
                    taken: Vec::new(),
                    made: Vec::new(),
                    _thread: None,
                }
            }
        };
        Ok(Input {
            kind,
            channels: usize::from(channels),
        })
    }

    /// How far the device's clock runs from the stream's, in parts per
    /// million, as the resampler follows it.
    pub fn drift_ppm(&self) -> i32 {
        match &self.kind {
            InputKind::Device { drift, .. } => drift.device_ppm(),
            _ => 0,
        }
    }

    /// Times frames were dropped as the device ran too far ahead, and
    /// reads while it had none.
    pub fn counts(&self) -> (u64, u64) {
        match &self.kind {
            InputKind::Device { buffer, .. } => (
                buffer.adjustments.load(Ordering::Relaxed),
                buffer.underruns.load(Ordering::Relaxed),
            ),
            _ => (0, 0),
        }
    }

    /// Fills `out` with interleaved frames.
    pub fn read(&mut self, out: &mut [f32]) {
        match &mut self.kind {
            InputKind::Silence => out.fill(0.0),
            InputKind::Tone { phase, step, level } => {
                for frame in out.chunks_exact_mut(self.channels.max(1)) {
                    frame.fill(phase.sin() * *level);
                    *phase = (*phase + *step) % TAU;
                }
            }
            InputKind::Device {
                buffer,
                drift,
                taken,
                made,
                ..
            } => {
                let frames = out.len() / self.channels.max(1);
                drift.steer(buffer);
                taken.resize(drift.wanted(frames) * self.channels, 0.0);
                buffer.pull(taken);
                drift.push(taken);
                made.clear();
                drift.make(made, frames);
                out.fill(0.0);
                out[..made.len()].copy_from_slice(made);
            }
        }
    }
}

/// When the samples a listener writes are due.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Presented {
    /// The first sample's presentation time, in gPTP nanoseconds.
    pub first: i64,
    /// gPTP time less monotonic time, in nanoseconds.
    pub offset: i64,
}

/// How long an output watches its device before choosing the delay.
const CHOOSING: i64 = 2_000_000_000;
/// The delay comes in steps of this, and grows by one each time the
/// device then finds nothing to play.
const DELAY_STEP: i64 = 1_000_000;
const MOST_DELAY: i64 = 500_000_000;
/// Frames further ahead of the device than this say the clocks disagree,
/// and the output steers by the fill alone.
const MOST_AHEAD: i64 = 500_000_000;

/// The resampler toward the device, and when its frames play.
struct Playing {
    drift: Drift,
    made: Vec<f32>,
    /// The presentation time of the next input frame, and gPTP time less
    /// monotonic time, in nanoseconds.
    next_in: Option<i64>,
    offset: Option<i64>,
    /// Device frame `.0` presents at gPTP nanoseconds `.1`.
    anchor: Option<(u64, i64)>,
    /// How long after its presentation time each frame plays, once
    /// chosen, and how far from that the last chunk played.
    delay: Option<i64>,
    error: i64,
    /// While choosing: when it started, in monotonic nanoseconds, and the
    /// lateness at the kept fill, summed, with the times it was seen.
    choosing: Option<(i64, f64, u32)>,
    /// The device's underruns when the delay was last set.
    underruns: u64,
}

/// The stream's samples to their sink.
pub struct Output {
    buffer: Option<Arc<FrameBuffer>>,
    _thread: Option<DeviceThread>,
    playing: Mutex<Playing>,
    /// Input frames a second.
    rate: u32,
    /// The loudest sample since the last look, as f32 bits.
    peak: AtomicU32,
}

impl Output {
    /// Opens `sink` for `channels` channels at `rate` hertz.
    pub fn open(sink: &Sink, channels: u16, rate: u32) -> io::Result<Self> {
        match sink {
            Sink::Discard => Ok(Output::new(None, None, channels, rate, rate)),
            Sink::Shared(shared) if shared.channels != channels => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "a shared buffer of other channels than the stream's",
            )),
            Sink::Shared(shared) => Ok(Output::new(
                Some(shared.buffer.clone()),
                None,
                channels,
                rate,
                shared.rate,
            )),
            Sink::Device {
                name,
                first_channel,
            } => {
                let device = find_device(name.as_deref(), false)?;
                let device_rate = device_rate(&device, false, rate)?;
                let margin = device_rate as usize / 100;
                let buffer = Arc::new(FrameBuffer::new(channels, margin));
                let draining = buffer.clone();
                let first = usize::from(*first_channel);
                let wanted = usize::from(channels);
                let thread = device_thread("audio output".into(), move || {
                    let supported = device.default_output_config().map_err(device_error)?;
                    let mut config = supported.config();
                    config.sample_rate = device_rate;
                    match supported.sample_format() {
                        cpal::SampleFormat::F32 => {
                            output_stream::<f32>(&device, config, wanted, first, draining)
                        }
                        cpal::SampleFormat::I16 => {
                            output_stream::<i16>(&device, config, wanted, first, draining)
                        }
                        cpal::SampleFormat::I32 => {
                            output_stream::<i32>(&device, config, wanted, first, draining)
                        }
                        other => Err(io::Error::new(
                            io::ErrorKind::Unsupported,
                            format!("samples in {other}"),
                        )),
                    }
                })?;
                Ok(Output::new(
                    Some(buffer),
                    Some(thread),
                    channels,
                    rate,
                    device_rate,
                ))
            }
        }
    }

    fn new(
        buffer: Option<Arc<FrameBuffer>>,
        thread: Option<DeviceThread>,
        channels: u16,
        rate: u32,
        device_rate: u32,
    ) -> Self {
        let nominal = f64::from(rate) / f64::from(device_rate.max(1));
        Output {
            buffer,
            _thread: thread,
            playing: Mutex::new(Playing {
                drift: Drift::new(channels, DeviceSide::Pulls, nominal),
                made: Vec::new(),
                next_in: None,
                offset: None,
                anchor: None,
                delay: None,
                error: 0,
                choosing: None,
                underruns: 0,
            }),
            rate: rate.max(1),
            peak: AtomicU32::new(0),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Playing> {
        self.playing.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Takes interleaved frames, and when they are due where the stream
    /// says.
    pub fn write(&self, samples: &[f32], presented: Option<Presented>) {
        let loudest = samples
            .iter()
            .fold(0.0f32, |most, sample| most.max(sample.abs()));
        self.peak.fetch_max(loudest.to_bits(), Ordering::Relaxed);
        let Some(buffer) = &self.buffer else {
            return;
        };
        let mut playing = self.lock();
        let playing = &mut *playing;
        if let Some(presented) = presented {
            playing.next_in = Some(presented.first);
            playing.offset = Some(presented.offset);
        }
        if let Some(sample) = playing.drift.new_sample(buffer) {
            self.steer(playing, buffer, &sample);
        }
        playing.drift.push(samples);
        playing.made.clear();
        playing.drift.make(&mut playing.made, usize::MAX);
        buffer.push(&playing.made);
        let frames = samples.len() / playing.drift.channels;
        let frame = 1e9 / f64::from(self.rate);
        if let Some(next) = &mut playing.next_in {
            *next += (frames as f64 * frame).round() as i64;
            // The next frame the resampler makes lies the frames it holds
            // back before the next to come.
            let made = *next - (playing.drift.pending() * frame).round() as i64;
            playing.anchor = Some((buffer.pushed(), made));
        }
    }

    /// Steers by when the frames play, once the device and the stream say
    /// when, else by the buffer's fill.
    fn steer(&self, playing: &mut Playing, buffer: &FrameBuffer, sample: &DeviceSample) {
        let excess = sample.level as f64 - playing.drift.kept(buffer, sample) as f64;
        let (Some(plays_at), Some((anchor, presented)), Some(offset)) =
            (sample.plays_at, playing.anchor, playing.offset)
        else {
            playing.drift.follow(excess, sample.chunk);
            return;
        };
        // How late the oldest frame held plays.
        let frame = playing.drift.ratio() * 1e9 / f64::from(self.rate);
        let oldest = sample.pushed - sample.level as u64;
        let presents = presented as f64 - (anchor as f64 - oldest as f64) * frame;
        let late = (plays_at + offset) as f64 - presents;
        if !(-MOST_AHEAD as f64..=2.0 * MOST_DELAY as f64).contains(&late) {
            playing.drift.follow(excess, sample.chunk);
            return;
        }
        let Some(delay) = playing.delay else {
            // Steered by the fill while choosing, noting how late frames
            // play at the fill kept: the earliest that is safe.
            playing.drift.follow(excess, sample.chunk);
            let (since, sum, seen) = playing.choosing.get_or_insert((plays_at, 0.0, 0));
            *sum += late - excess * frame;
            *seen += 1;
            if plays_at - *since >= CHOOSING {
                let earliest = (*sum / f64::from(*seen)).max(0.0) as i64;
                let steps = (earliest + DELAY_STEP - 1) / DELAY_STEP;
                let delay = (steps * DELAY_STEP).min(MOST_DELAY);
                self.set_delay(playing, buffer, delay, (late - delay as f64) / frame);
            }
            return;
        };
        let underruns = buffer.underruns.load(Ordering::Relaxed);
        if underruns > playing.underruns && delay < MOST_DELAY {
            let later = delay + DELAY_STEP;
            self.set_delay(playing, buffer, later, (late - later as f64) / frame);
            return;
        }
        playing.error = (late - delay as f64) as i64;
        playing
            .drift
            .follow((late - delay as f64) / frame, sample.chunk);
    }

    /// Plays each frame `delay` after its presentation time from now on,
    /// moving at once by the `excess` frames held beyond that.
    fn set_delay(&self, playing: &mut Playing, buffer: &FrameBuffer, delay: i64, excess: f64) {
        playing.delay = Some(delay);
        playing.error = 0;
        playing.underruns = buffer.underruns.load(Ordering::Relaxed);
        let excess = excess.round() as i64;
        buffer.hold_more(excess.unsigned_abs() as usize);
        if excess > 0 {
            buffer.drop_oldest(excess as usize);
        } else if excess < 0 {
            // Silence in front of what comes next, which then presents
            // that much later than the frames before it.
            let frames = excess.unsigned_abs() as usize;
            buffer.push(&vec![0.0; frames * playing.drift.channels]);
            if let Some((anchor, _)) = &mut playing.anchor {
                *anchor += frames as u64;
            }
        }
    }

    /// How long after its presentation time each sample plays, once the
    /// output knows, and how far from that the last chunk played, in
    /// nanoseconds.
    pub fn playout(&self) -> Option<(i64, i64)> {
        let playing = self.lock();
        playing.delay.map(|delay| (delay, playing.error))
    }

    /// How far the device's clock runs from the stream's, in parts per
    /// million, as the resampler follows it.
    pub fn drift_ppm(&self) -> i32 {
        if self.buffer.is_none() {
            return 0;
        }
        self.lock().drift.device_ppm()
    }

    /// Times the device found nothing to play.
    pub fn underruns(&self) -> u64 {
        self.buffer
            .as_ref()
            .map_or(0, |buffer| buffer.underruns.load(Ordering::Relaxed))
    }

    /// The loudest sample since the last call, from 0 to 1.
    pub fn take_peak(&self) -> f32 {
        f32::from_bits(self.peak.swap(0, Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_buffer_notes_its_fill_after_each_device_chunk() {
        let buffer = FrameBuffer::new(2, 10);
        assert_eq!(buffer.device_sample(), None);
        buffer.device_push(&[1.0; 8]);
        buffer.push(&[2.0; 4]);
        assert_eq!(
            buffer.device_sample(),
            Some(DeviceSample {
                chunks: 1,
                level: 4,
                chunk: 4,
                pushed: 4,
                plays_at: None,
            })
        );
        let mut out = [9.0; 6];
        buffer.device_pull(&mut out, None);
        assert_eq!(out, [1.0; 6]);
        assert_eq!(buffer.device_sample().map(|sample| sample.chunks), Some(2));
        // Past four times the margin and the chunk, the oldest go.
        buffer.push(&[3.0; 200]);
        assert_eq!(buffer.level(), 4 * (10 + 3));
        assert!(buffer.adjustments.load(Ordering::Relaxed) > 0);
        // Empty: silence, counted.
        let mut out = [9.0; 120];
        buffer.pull(&mut out);
        assert_eq!(out[104..], [0.0; 16]);
        assert_eq!(buffer.underruns.load(Ordering::Relaxed), 1);
    }

    /// A sine at `hertz` sampled at `rate`, from sample `start` on.
    fn sine(hertz: f64, rate: f64, start: u64, frames: usize) -> Vec<f32> {
        (0..frames as u64)
            .map(|index| (TAU as f64 * hertz * (start + index) as f64 / rate).sin() as f32 * 0.5)
            .collect()
    }

    /// When the device's `index`th chunk comes, at `rate`: on time give
    /// or take up to 2% of a chunk.
    fn chunk_time(index: u64, chunk: usize, rate: f64) -> f64 {
        let nominal = index as f64 * chunk as f64 / rate;
        let wobble = ((index * 7919) % 41) as f64 / 40.0 - 0.5;
        nominal + 0.04 * wobble * chunk as f64 / rate
    }

    /// A device 120 ppm fast feeding a talker's stream, in chunks of 1024
    /// frames that come a little early or late, for a simulated two
    /// minutes: the ratio settles on the drift, nothing runs short or
    /// over once started, and the sine comes out whole.
    #[test]
    fn a_fast_device_is_followed_without_a_gap() {
        let rate = 48_000.0;
        let device_rate = rate * (1.0 + 120e-6);
        let buffer = FrameBuffer::new(1, 480);
        let mut drift = Drift::new(1, DeviceSide::Pushes, 1.0);
        let mut chunks = 0;
        let mut out = Vec::new();
        let mut steps = Vec::new();
        let mut ratios = Vec::new();
        for packet in 0..8000 * 120u64 {
            let now = packet as f64 / 8000.0;
            while chunk_time(chunks, 1024, device_rate) <= now {
                buffer.device_push(&sine(1000.0, device_rate, chunks * 1024, 1024));
                chunks += 1;
            }
            drift.steer(&buffer);
            let mut taken = vec![0.0; drift.wanted(6)];
            buffer.pull(&mut taken);
            drift.push(&taken);
            out.clear();
            drift.make(&mut out, 6);
            assert_eq!(out.len(), 6);
            if packet == 8000 {
                buffer.underruns.store(0, Ordering::Relaxed);
            }
            if packet > 8000 * 40 {
                steps.extend(out.windows(2).map(|pair| (pair[1] - pair[0]).abs()));
                ratios.push(drift.ratio());
            }
        }
        for ratio in &ratios {
            assert!((ratio - (1.0 + 120e-6)).abs() < 30e-6, "{ratio}");
        }
        assert_eq!(buffer.adjustments.load(Ordering::Relaxed), 0);
        assert_eq!(buffer.underruns.load(Ordering::Relaxed), 0);
        // No frame is missing: no step is bigger than a 1 kHz sine at
        // half scale ever takes between two samples.
        let most = steps.iter().fold(0.0f32, |most, step| most.max(*step));
        assert!(most < 0.07, "{most}");
    }

    /// A slow device playing a listener's stream in chunks of 1024
    /// frames, the other side of the buffer: it settles too, and never
    /// finds the buffer empty once playing.
    #[test]
    fn a_slow_device_is_fed_without_a_gap() {
        let rate = 48_000.0;
        let device_rate = rate * (1.0 - 80e-6);
        let buffer = FrameBuffer::new(1, 480);
        let mut drift = Drift::new(1, DeviceSide::Pulls, 1.0);
        let mut chunks = 0;
        let mut made = Vec::new();
        let mut played = vec![0.0; 1024];
        let mut ratios = Vec::new();
        for packet in 0..8000 * 120u64 {
            drift.steer(&buffer);
            drift.push(&sine(1000.0, rate, packet * 6, 6));
            made.clear();
            drift.make(&mut made, usize::MAX);
            buffer.push(&made);
            let now = packet as f64 / 8000.0;
            while chunk_time(chunks + 1, 1024, device_rate) <= now {
                buffer.device_pull(&mut played, None);
                chunks += 1;
            }
            if packet == 8000 {
                buffer.underruns.store(0, Ordering::Relaxed);
            }
            if packet > 8000 * 40 {
                ratios.push(drift.ratio());
            }
        }
        for ratio in &ratios {
            assert!((ratio - 1.0 / (1.0 - 80e-6)).abs() < 30e-6, "{ratio}");
        }
        assert_eq!(buffer.adjustments.load(Ordering::Relaxed), 0);
        assert_eq!(buffer.underruns.load(Ordering::Relaxed), 0);
    }

    /// A 48 kHz device 50 ppm fast feeding a 96 kHz stream: the ratio
    /// settles on half the device's frames for each of the stream's,
    /// the drift reads as the device's own, and the sine comes out whole
    /// at twice the rate.
    #[test]
    fn a_device_at_half_the_rate_is_followed() {
        let device_rate = 48_000.0 * (1.0 + 50e-6);
        let buffer = FrameBuffer::new(1, 480);
        let mut drift = Drift::new(1, DeviceSide::Pushes, 0.5);
        let mut chunks = 0;
        let mut out = Vec::new();
        let mut steps = Vec::new();
        let mut ppms = Vec::new();
        for packet in 0..8000 * 120u64 {
            let now = packet as f64 / 8000.0;
            while chunk_time(chunks, 1024, device_rate) <= now {
                buffer.device_push(&sine(1000.0, device_rate, chunks * 1024, 1024));
                chunks += 1;
            }
            drift.steer(&buffer);
            let mut taken = vec![0.0; drift.wanted(12)];
            buffer.pull(&mut taken);
            drift.push(&taken);
            out.clear();
            drift.make(&mut out, 12);
            assert_eq!(out.len(), 12);
            if packet == 8000 {
                buffer.underruns.store(0, Ordering::Relaxed);
            }
            if packet > 8000 * 40 {
                steps.extend(out.windows(2).map(|pair| (pair[1] - pair[0]).abs()));
                ppms.push(drift.device_ppm());
            }
        }
        for ppm in &ppms {
            assert!((ppm - 50).abs() < 30, "{ppm}");
        }
        assert_eq!(buffer.adjustments.load(Ordering::Relaxed), 0);
        assert_eq!(buffer.underruns.load(Ordering::Relaxed), 0);
        // A 1 kHz sine at half scale steps at most 0.033 at 96 kHz.
        let most = steps.iter().fold(0.0f32, |most, step| most.max(*step));
        assert!(most < 0.04, "{most}");
    }

    /// A listener's output to a device 50 ppm slow that plays each chunk
    /// of `chunk` frames `latency` after its callback, for frames arriving
    /// `ahead` before their presentation time, for a simulated minute: the
    /// delay it chooses, and the most any sample in the last 20 s played
    /// away from that delay after its presentation time, in seconds.
    fn play_out(chunk: usize, latency: f64, ahead: f64) -> (i64, f64, u64) {
        let rate = 48_000.0;
        let device_rate = rate * (1.0 - 50e-6);
        let buffer = Arc::new(FrameBuffer::new(1, 480));
        let output = Output::new(Some(buffer.clone()), None, 1, 48_000, 48_000);
        let tone = |seconds: f64| (TAU as f64 * 100.0 * seconds).sin() as f32 * 0.5;
        let mut played = vec![0.0; chunk];
        let mut chunks = 0;
        let mut most = 0.0f64;
        let mut underruns = 0;
        for packet in 0..8000 * 60u64 {
            let arrives = packet as f64 / 8000.0;
            while chunk_time(chunks, chunk, device_rate) <= arrives {
                let plays = chunks as f64 * chunk as f64 / device_rate + latency;
                let after = plays + chunk as f64 / device_rate;
                buffer.device_pull(&mut played, Some((after * 1e9) as i64));
                if arrives > 40.0 {
                    let (delay, _) = output.playout().unwrap();
                    for (index, sample) in played.iter().enumerate() {
                        let at = plays + index as f64 / device_rate - delay as f64 * 1e-9;
                        // How far in time the sample is from the one due.
                        let off = f64::from(sample - tone(at)) / (TAU as f64 * 100.0 * 0.5);
                        most = most.max(off.abs());
                    }
                }
                chunks += 1;
            }
            if packet == 8000 * 10 {
                underruns = buffer.underruns.load(Ordering::Relaxed);
            }
            let first = arrives + ahead;
            let samples: Vec<f32> = (0..6)
                .map(|index| tone(first + index as f64 / rate))
                .collect();
            let presented = Presented {
                first: (first * 1e9) as i64,
                offset: 0,
            };
            output.write(&samples, Some(presented));
        }
        let (delay, _) = output.playout().unwrap();
        let more = buffer.underruns.load(Ordering::Relaxed) - underruns;
        (delay, most, more)
    }

    /// Frames 1.7 ms ahead to a device 25 ms behind its callbacks of 1024
    /// frames: too late for presentation time, so each plays the same
    /// whole milliseconds after it, the device's latency, a chunk and
    /// the margin less the 1.7 ms.
    #[test]
    fn a_slow_device_plays_a_fixed_delay_after_presentation() {
        let (delay, most, underruns) = play_out(1024, 0.025, 0.0017);
        assert_eq!(delay, 55_000_000);
        assert!(most < 0.000_2, "{most}");
        assert_eq!(underruns, 0);
    }

    /// Frames 100 ms ahead to a device 5 ms behind its callbacks of 256
    /// frames: each plays at its presentation time.
    #[test]
    fn early_frames_play_at_presentation_time() {
        let (delay, most, underruns) = play_out(256, 0.005, 0.1);
        assert_eq!(delay, 0);
        assert!(most < 0.000_2, "{most}");
        assert_eq!(underruns, 0);
    }

    #[test]
    fn a_tone_fills_every_channel() {
        let mut input = Input::open(
            &Source::Tone {
                hertz: 1000.0,
                level: 0.5,
            },
            2,
            48_000,
        )
        .unwrap();
        let mut out = [0.0; 96];
        input.read(&mut out);
        assert!(out.iter().all(|sample| sample.abs() <= 0.5));
        assert!(
            out.as_chunks::<2>()
                .0
                .iter()
                .all(|frame| frame[0] == frame[1])
        );
        assert!(out.iter().any(|sample| sample.abs() > 0.4));
    }
}
