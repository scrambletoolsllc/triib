//! Test signals, for trying streams with devices that send their own: a
//! tone of its own on each of an output's channels, and what an input
//! receives checked against the tones it carries, for clicks and gaps.
//!
//! Both pass through a buffer nothing steers, so the stream takes and
//! gives their samples at its own rate, none resampled.

use std::f64::consts::TAU;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use atdecc::stream_format::StreamFormat;
use avb_stream::ListenerStats;
use avb_stream::audio::{FrameBuffer, SharedBuffer};

/// An output's tones: channel n at 500 + 125 n hertz, all at a tenth of
/// full scale, -20 dBFS.
const FIRST_HERTZ: f64 = 500.0;
const STEP_HERTZ: f64 = 125.0;
const LEVEL: f64 = 0.1;

/// How much the tones keep ahead of the stream, and how often they look:
/// often, as a talker starting empties the buffer and takes from it at
/// once, finding silence until they fill it again.
const KEPT: Duration = Duration::from_millis(40);
const LOOK_EVERY: Duration = Duration::from_millis(1);

/// How often the analysis takes what came, and how much it holds before
/// the oldest goes: far more than it leaves between.
const TAKE_EVERY: Duration = Duration::from_millis(5);
const HELD: Duration = Duration::from_millis(100);

/// The first tenth of a second of each channel only finds its tone, and
/// a channel is silent unless it peaks above this.
const FINDING: u32 = 10;
const LEAST_TONE: f64 = 1e-3;

/// A sample further than this share of its tone's level from the tone
/// the samples before it predict is a click; samples closer than this to
/// nothing, two or more in a row, are a gap.
const CLICK_SHARE: f64 = 0.005;
const SILENT: f64 = 1e-6;

/// The clicks and gaps whose times and channels a run keeps: its first.
const MOST_EVENTS: usize = 8;

/// A thread of its own, stopped and joined when dropped. It runs real
/// time just below the streams' threads: they wait on the buffer they
/// share while it holds it, so it must not be put off for the computer's
/// other threads, but it must never keep them from running either.
struct Worker {
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn spawn(name: &str, mut work: impl FnMut() + Send + 'static) -> io::Result<Self> {
        let stop = Arc::new(AtomicBool::new(false));
        let stopping = stop.clone();
        let thread = std::thread::Builder::new()
            .name(name.into())
            .spawn(move || {
                below_streams();
                while !stopping.load(Ordering::Relaxed) {
                    work();
                }
            })?;
        Ok(Worker {
            stop,
            thread: Some(thread),
        })
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// The priority RealtimeKit gives the calling thread, one below the 15 the
/// streams' threads ask for; it first asks as they do, which sets the
/// CPU time RealtimeKit wants promised.
#[cfg(target_os = "linux")]
fn below_streams() {
    if !avb_stream::realtime::raise() {
        return;
    }
    let Some(thread) = std::fs::read_link("/proc/thread-self")
        .ok()
        .and_then(|task| task.file_name()?.to_str().map(str::to_owned))
    else {
        return;
    };
    let _ = std::process::Command::new("busctl")
        .args([
            "call",
            "--system",
            "org.freedesktop.RealtimeKit1",
            "/org/freedesktop/RealtimeKit1",
            "org.freedesktop.RealtimeKit1",
            "MakeThreadRealtimeWithPID",
            "ttu",
        ])
        .arg(std::process::id().to_string())
        .arg(thread)
        .arg("14")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
}

#[cfg(not(target_os = "linux"))]
fn below_streams() {
    avb_stream::realtime::raise();
}

/// An output's tones, made while it is kept.
pub struct Tones {
    _worker: Worker,
}

impl Tones {
    /// Starts the tones of `channels` at `rate`, into the buffer returned
    /// for the talker to take from.
    pub fn start(channels: u16, rate: u32) -> io::Result<(Self, SharedBuffer)> {
        let kept = (rate as usize * KEPT.as_millis() as usize / 1000).max(1);
        let buffer = Arc::new(FrameBuffer::new(channels, kept));
        let filling = buffer.clone();
        let mut tones = ToneMaker::new(channels, rate);
        let mut made = Vec::new();
        let worker = Worker::spawn("tones", move || {
            let level = filling.level();
            if level < kept {
                tones.make(&mut made, kept - level);
                filling.push(&made);
            }
            std::thread::sleep(LOOK_EVERY);
        })?;
        Ok((
            Tones { _worker: worker },
            SharedBuffer {
                buffer,
                channels,
                rate,
            },
        ))
    }
}

/// Each channel's tone, as far as it has gone.
struct ToneMaker {
    phases: Vec<f64>,
    steps: Vec<f64>,
}

impl ToneMaker {
    fn new(channels: u16, rate: u32) -> Self {
        let steps = (0..channels)
            .map(|channel| TAU * tone_hertz(channel) / f64::from(rate.max(1)))
            .collect();
        ToneMaker {
            phases: vec![0.0; usize::from(channels)],
            steps,
        }
    }

    /// The next `frames`, interleaved, into `out`.
    fn make(&mut self, out: &mut Vec<f32>, frames: usize) {
        out.clear();
        for _ in 0..frames {
            for (phase, step) in self.phases.iter_mut().zip(&self.steps) {
                out.push((phase.sin() * LEVEL) as f32);
                *phase = (*phase + step) % TAU;
            }
        }
    }
}

/// The tone an output sends on `channel`, counted from 0.
pub fn tone_hertz(channel: u16) -> f64 {
    FIRST_HERTZ + STEP_HERTZ * f64::from(channel)
}

/// An input's audio, checked as it comes while it is kept.
pub struct Analysis {
    totals: Arc<Mutex<Totals>>,
    buffer: Arc<FrameBuffer>,
    _worker: Worker,
}

impl Analysis {
    /// Starts checking `channels` at `rate`, from the buffer returned for
    /// the listener to give to.
    pub fn start(channels: u16, rate: u32) -> io::Result<(Self, SharedBuffer)> {
        let held = (rate as usize * HELD.as_millis() as usize / 1000).max(1);
        let buffer = Arc::new(FrameBuffer::new(channels, held));
        let totals = Arc::new(Mutex::new(Totals::default()));
        let (draining, adding) = (buffer.clone(), totals.clone());
        let mut checker = Checker::new(channels, rate);
        let mut taken = Vec::new();
        let mut first = None;
        let worker = Worker::spawn("analysis", move || {
            let level = draining.level();
            if level > 0 {
                taken.resize(level * usize::from(channels.max(1)), 0.0);
                draining.pull(&mut taken);
                checker.take(&taken);
                let now = SystemTime::now();
                *adding.lock().unwrap_or_else(PoisonError::into_inner) = Totals {
                    first: Some(*first.get_or_insert(now)),
                    last: Some(now),
                    ..checker.totals()
                };
            }
            std::thread::sleep(TAKE_EVERY);
        })?;
        Ok((
            Analysis {
                totals,
                buffer: buffer.clone(),
                _worker: worker,
            },
            SharedBuffer {
                buffer,
                channels,
                rate,
            },
        ))
    }

    /// What it found so far.
    pub fn totals(&self) -> Totals {
        let mut totals = self
            .totals
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        totals.overrun = self.buffer.adjustments.load(Ordering::Relaxed);
        totals
    }
}

/// What an analysis found since it started.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Totals {
    /// Sample frames checked, and how many a second.
    pub samples: u64,
    pub rate: u32,
    pub channels: Vec<ChannelTotals>,
    /// Times the analysis fell so far behind that frames were lost
    /// before it took them, which the counts then miss.
    pub overrun: u64,
    /// When it took its first frames and its last.
    pub first: Option<SystemTime>,
    pub last: Option<SystemTime>,
    /// Its first clicks and gaps.
    pub events: Vec<Event>,
    /// The silence its tones end in so far, in frames: as a talker that
    /// stops may send, so no gap.
    pub silent_tail: u64,
}

/// A click or gap, from the frame it starts at, on the channels it is on,
/// counted from 0.
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub from: u64,
    /// A gap's frames, none for a click.
    pub gap: Option<u64>,
    pub channels: Vec<u16>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ChannelTotals {
    /// Its tone's frequency, none for a silent channel, and its loudest
    /// sample but for clicks.
    pub hertz: Option<f64>,
    pub peak: f64,
    pub clicks: u64,
    pub gaps: u64,
}

/// Checks each channel against the tone its samples carry: a sine's next
/// sample is `c` times the last less the one before, `c` fitted to them.
struct Checker {
    rate: u32,
    samples: u64,
    channels: Vec<Channel>,
    events: Vec<Event>,
}

#[derive(Default)]
struct Channel {
    /// The two samples before the next, the latest last.
    before: [f64; 2],
    /// Sums fitting `c` over the clean samples.
    across: f64,
    squares: f64,
    peak: f64,
    clicks: u64,
    gaps: u64,
    /// Silent samples in a row up to the last, whether they count as a
    /// gap once the tone comes back, and the frame the last gap or click
    /// was at.
    silent: u64,
    gap_counts: bool,
    gap_at: Option<u64>,
    click_at: Option<u64>,
}

/// What a channel found at a frame.
enum Found {
    Click,
    /// The tone came back after these silent frames.
    Gap(u64),
}

impl Checker {
    fn new(channels: u16, rate: u32) -> Self {
        Checker {
            rate: rate.max(1),
            samples: 0,
            channels: (0..channels).map(|_| Channel::default()).collect(),
            events: Vec::new(),
        }
    }

    fn finding(&self) -> u64 {
        u64::from(self.rate / FINDING)
    }

    /// Checks interleaved frames.
    fn take(&mut self, samples: &[f32]) {
        let finding = self.finding();
        let click_apart = u64::from(self.rate / 1000);
        for frame in samples.chunks_exact(self.channels.len().max(1)) {
            let at = self.samples;
            for (index, sample) in frame.iter().enumerate() {
                let found =
                    self.channels[index].take(f64::from(*sample), at, at < finding, click_apart);
                if let Some(found) = found {
                    self.note(found, at, index as u16, click_apart);
                }
            }
            self.samples += 1;
        }
    }

    /// Keeps one of the first clicks or gaps, one on other channels at
    /// the same time joining it.
    fn note(&mut self, found: Found, at: u64, channel: u16, apart: u64) {
        let (from, gap) = match found {
            Found::Click => (at, None),
            Found::Gap(frames) => (at - frames, Some(frames)),
        };
        let room = self.events.len() < MOST_EVENTS;
        let same = self.events.iter_mut().rev().find(|event| {
            event.gap.is_some() == gap.is_some() && event.from.abs_diff(from) <= apart
        });
        match same {
            Some(event) => event.channels.push(channel),
            None if room => self.events.push(Event {
                from,
                gap,
                channels: vec![channel],
            }),
            None => {}
        }
    }

    fn totals(&self) -> Totals {
        let found = self.samples >= self.finding();
        Totals {
            samples: self.samples,
            rate: self.rate,
            channels: self
                .channels
                .iter()
                .map(|channel| {
                    let tone = found && channel.peak > LEAST_TONE;
                    ChannelTotals {
                        hertz: tone.then(|| channel.hertz(self.rate)).flatten(),
                        peak: channel.peak,
                        clicks: if tone { channel.clicks } else { 0 },
                        gaps: if tone { channel.gaps } else { 0 },
                    }
                })
                .collect(),
            events: self.events.clone(),
            silent_tail: self
                .channels
                .iter()
                .filter(|channel| channel.peak > LEAST_TONE && channel.silent >= 2)
                .map(|channel| channel.silent)
                .max()
                .unwrap_or(0),
            ..Totals::default()
        }
    }
}

impl Channel {
    fn take(&mut self, sample: f64, at: u64, finding: bool, click_apart: u64) -> Option<Found> {
        let [two_back, last] = self.before;
        self.before = [last, sample];
        if sample.abs() < SILENT {
            self.silent += 1;
            if self.silent == 2 {
                self.gap_counts = !finding && self.peak > LEAST_TONE;
            }
            if self.silent >= 2 {
                self.gap_at = Some(at);
            }
            return None;
        }
        // Silence is a gap once the tone comes back after it.
        let mut found = None;
        if self.silent >= 2 && self.gap_counts {
            self.gaps += 1;
            found = Some(Found::Gap(self.silent));
        }
        self.silent = 0;
        // The samples around a gap are its own, not clicks.
        let near_gap = self.gap_at.is_some_and(|gap| at - gap <= 2);
        let off = self.fitted().map(|c| sample - c * last + two_back);
        if !finding
            && !near_gap
            && let Some(off) = off
            && off.abs() > CLICK_SHARE * self.peak.max(LEAST_TONE)
        {
            if self.click_at.is_none_or(|click| at - click > click_apart) {
                self.clicks += 1;
                found = Some(Found::Click);
            }
            self.click_at = Some(at);
            return found;
        }
        if !near_gap && at >= 2 {
            self.across += last * (sample + two_back);
            self.squares += last * last;
        }
        self.peak = self.peak.max(sample.abs());
        found
    }

    fn fitted(&self) -> Option<f64> {
        (self.squares > 0.0).then(|| self.across / self.squares)
    }

    /// The tone's frequency, as `c` is twice the cosine of its step.
    fn hertz(&self, rate: u32) -> Option<f64> {
        let c = self.fitted()?;
        Some((c / 2.0).clamp(-1.0, 1.0).acos() * f64::from(rate) / TAU)
    }
}

/// One input's listening while its audio is analyzed: in what format,
/// and its stream's counters and clock as last seen.
pub struct Run {
    pub analysis: Analysis,
    format: StreamFormat,
    pub stats: ListenerStats,
    pub clock_ppm: Option<f64>,
    /// When it last said what it found, and when its listener stopped.
    pub said_at: Instant,
    pub stopped: Option<Instant>,
}

impl Run {
    pub fn new(analysis: Analysis, format: StreamFormat) -> Self {
        Run {
            analysis,
            format,
            stats: ListenerStats::default(),
            clock_ppm: None,
            said_at: Instant::now(),
            stopped: None,
        }
    }

    /// What it found, for the log: so far, or all of it once it ended,
    /// from its first frames to its last.
    pub fn line(&self, name: &str, ended: bool) -> String {
        let totals = self.analysis.totals();
        let (Some(first), Some(last)) = (totals.first, totals.last) else {
            return format!("{name}: analyzed nothing, as nothing came");
        };
        let seconds = last.duration_since(first).unwrap_or_default().as_secs_f64();
        let each = |pick: fn(&ChannelTotals) -> String| {
            totals
                .channels
                .iter()
                .map(|channel| {
                    if channel.hertz.is_some() {
                        pick(channel)
                    } else {
                        "-".to_owned()
                    }
                })
                .collect::<Vec<_>>()
                .join(" ")
        };
        let stats = &self.stats;
        let rate = f64::from(totals.rate.max(1));
        format!(
            "{name}: analyzed {} {} to {} UTC, {seconds:.1} s, format {:#018x}, {} samples, clicks {}, gaps {}, hertz {}, peak {}, clock {}, {} frames received, {} out of sequence, {} dropped here, {} late, {} early, {} interrupted{}{}{}",
            if ended { "from" } else { "so far, from" },
            utc(first),
            utc(last),
            self.format.0,
            totals.samples,
            each(|channel| channel.clicks.to_string()),
            each(|channel| channel.gaps.to_string()),
            each(|channel| format!("{:.1}", channel.hertz.unwrap_or_default())),
            each(|channel| format!("{:.3}", channel.peak)),
            self.clock_ppm.map_or_else(
                || "not recovered".to_owned(),
                |ppm| format!("{ppm:+.2} ppm")
            ),
            stats.frames_received,
            stats.sequence_mismatches,
            stats.socket_drops,
            stats.late,
            stats.early,
            stats.interrupted,
            match totals.overrun {
                0 => String::new(),
                times => format!(", analysis behind {times} times"),
            },
            match totals.silent_tail {
                0 => String::new(),
                frames => format!(", silent for its last {:.3} s", frames as f64 / rate),
            },
            events(&totals.events, rate)
        )
    }
}

/// The first clicks and gaps, for the log: when each starts from the first
/// frame, and on which channels, from 1.
fn events(events: &[Event], rate: f64) -> String {
    if events.is_empty() {
        return String::new();
    }
    let each: Vec<String> = events
        .iter()
        .map(|event| {
            let channels: Vec<String> = event
                .channels
                .iter()
                .map(|channel| (channel + 1).to_string())
                .collect();
            let what = match event.gap {
                Some(frames) => format!("gap of {:.1} ms", frames as f64 * 1000.0 / rate),
                None => "click".to_owned(),
            };
            format!(
                "{what} at {:.3} s on {}",
                event.from as f64 / rate,
                channels.join(" ")
            )
        })
        .collect();
    format!("; first {}", each.join(", "))
}

/// A time of day in UTC, to the second.
fn utc(time: SystemTime) -> String {
    let seconds = time
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        % 86_400;
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 3600,
        seconds / 60 % 60,
        seconds % 60
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `seconds` of each of `hertz` at `level`, interleaved, at `rate`.
    fn sines(hertz: &[f64], level: f64, rate: u32, seconds: f64) -> Vec<f32> {
        let frames = (f64::from(rate) * seconds) as usize;
        (0..frames)
            .flat_map(|frame| {
                hertz.iter().map(move |hertz| {
                    let phase = TAU * hertz * frame as f64 / f64::from(rate);
                    (phase.sin() * level) as f32
                })
            })
            .collect()
    }

    fn check(samples: &[f32], channels: u16, rate: u32) -> Totals {
        let mut checker = Checker::new(channels, rate);
        // In pieces, as the buffer gives them.
        for piece in samples.chunks(240 * usize::from(channels)) {
            checker.take(piece);
        }
        checker.totals()
    }

    #[test]
    fn clean_tones_have_no_clicks_or_gaps_and_their_frequencies() {
        for rate in [48_000, 96_000, 192_000] {
            let hertz = [1000.0, 1250.0, 2750.0];
            let totals = check(&sines(&hertz, 0.07, rate, 1.0), 3, rate);
            assert_eq!(totals.samples, u64::from(rate));
            for (channel, hertz) in totals.channels.iter().zip(hertz) {
                assert_eq!((channel.clicks, channel.gaps), (0, 0), "at {rate}");
                assert!((channel.hertz.unwrap() - hertz).abs() < 0.01, "at {rate}");
            }
        }
    }

    #[test]
    fn a_frame_lost_or_twice_is_a_click() {
        let rate = 48_000;
        let mut samples = sines(&[1000.0, 1250.0], 0.07, rate, 1.0);
        // One frame dropped near 0.5 s, and one played twice near 0.75 s,
        // neither where a tone crosses nothing.
        let twice: Vec<f32> = samples[72_022..72_024].to_vec();
        samples.splice(72_022..72_022, twice);
        samples.drain(48_010..48_012);
        let totals = check(&samples, 2, rate);
        assert_eq!(totals.channels[0].clicks, 2);
        assert_eq!(totals.channels[1].clicks, 2);
        assert_eq!(totals.channels[0].gaps, 0);
        // Each kept once, on both channels, where it happened.
        let [lost, twice] = totals.events.as_slice() else {
            panic!("{:?}", totals.events);
        };
        assert_eq!(
            (lost.from, lost.gap, lost.channels.as_slice()),
            (24_005, None, &[0, 1][..])
        );
        assert_eq!(twice.channels, [0, 1]);
        assert!(twice.from.abs_diff(36_010) <= 1, "{}", twice.from);
    }

    /// Silence in place of frames is a gap, not clicks at its edges.
    #[test]
    fn silence_in_place_is_a_gap() {
        let rate = 48_000;
        let mut samples = sines(&[1000.0], 0.07, rate, 1.0);
        samples[30_000..30_006].fill(0.0);
        let totals = check(&samples, 1, rate);
        assert_eq!((totals.channels[0].clicks, totals.channels[0].gaps), (0, 1));
        assert_eq!(
            totals.events,
            [Event {
                from: 30_000,
                gap: Some(6),
                channels: vec![0]
            }]
        );
        assert_eq!(totals.silent_tail, 0);
    }

    /// Silence a stream ends in, as a talker stopping may send, is no gap.
    #[test]
    fn silence_at_the_end_is_its_tail() {
        let rate = 48_000;
        let mut samples = sines(&[1000.0, 1250.0], 0.07, rate, 1.0);
        let end = samples.len();
        samples[end - 2 * 480..].fill(0.0);
        let totals = check(&samples, 2, rate);
        assert!(totals.channels.iter().all(|channel| channel.gaps == 0));
        assert!(totals.events.is_empty());
        assert_eq!(totals.silent_tail, 480);
    }

    /// A 2-channel stream into 8 channels leaves 6 silent, which have no
    /// tone, rather than one long gap.
    #[test]
    fn silent_channels_have_no_tone() {
        let rate = 48_000;
        let tones = sines(&[1000.0, 1250.0], 0.07, rate, 0.5);
        let samples: Vec<f32> = tones
            .as_chunks::<2>()
            .0
            .iter()
            .flat_map(|[left, right]| [*left, *right, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])
            .collect();
        let totals = check(&samples, 8, rate);
        assert!(
            totals.channels[..2]
                .iter()
                .all(|channel| channel.hertz.is_some())
        );
        assert!(
            totals.channels[2..]
                .iter()
                .all(|channel| channel.hertz.is_none())
        );
    }

    /// The tones made carry each channel's own frequency, with no clicks
    /// from one piece to the next.
    #[test]
    fn tones_are_each_channels_own() {
        let rate = 96_000;
        let mut maker = ToneMaker::new(8, rate);
        let mut checker = Checker::new(8, rate);
        let mut piece = Vec::new();
        for frames in [1, 7, 480, 96_000] {
            maker.make(&mut piece, frames);
            checker.take(&piece);
        }
        for (channel, totals) in checker.totals().channels.iter().enumerate() {
            let hertz = tone_hertz(channel as u16);
            assert!((totals.hertz.unwrap() - hertz).abs() < 0.01);
            assert_eq!((totals.clicks, totals.gaps), (0, 0));
        }
        assert_eq!(tone_hertz(7), 1375.0);
    }

    #[test]
    fn times_of_day_are_utc() {
        let time = UNIX_EPOCH + Duration::from_secs(86_400 * 3 + 4 * 3600 + 58 * 60 + 7);
        assert_eq!(utc(time), "04:58:07");
    }
}
