//! Listeners of AAF or AM824: the frames of a stream, their samples
//! played on the sink, counted as Milan's stream input counters count
//! them; and of a media clock, CRF, its frames counted alike.
//!
//! A group takes several streams on one thread, each with its own
//! socket, sink and counters, waiting on all of their sockets at once. A
//! [`Listener`] is a group of one.

use std::io;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use atdecc::stream_format::StreamFormat;
use avb_net::MacAddress;
use avb_net::stream::{FrameReceiver, avtp_payload};

use crate::audio::{Output, Presented, Sink};
use crate::media::{self, Media};
use crate::{MediaClock, RecoveredClock, crf};

/// How long without a frame before the stream counts as interrupted.
const SILENCE: Duration = Duration::from_millis(100);
/// Presentation times further ahead than this count as early.
const EARLY: i64 = 50_000_000;
/// The longest the thread waits for a frame at once, so that it notices
/// streams that stopped and streams that came.
const LONGEST_WAIT: Duration = Duration::from_millis(20);
/// How often the sockets are asked what they dropped.
const DROPS_EVERY: Duration = Duration::from_millis(250);

/// What a listener receives, and where it goes.
#[derive(Debug, Clone, PartialEq)]
pub struct ListenerConfig {
    pub interface: String,
    pub stream_id: u64,
    /// Where the stream comes to: its multicast address, and in AVB Lite
    /// this computer's own as well.
    pub destinations: Vec<MacAddress>,
    pub format: StreamFormat,
    pub sink: Sink,
    /// The clock the stream carries, which it recovers for talkers to
    /// follow: a media clock's from its timestamps, an audio stream's
    /// from its presentation times.
    pub recovered: Option<RecoveredClock>,
    /// For each of the sink's channels, the stream channel it plays, none
    /// for silence, as a controller mapped them or as a stream of other
    /// channels than the sink's fills it; the stream's own channels, in
    /// order, when left out.
    pub channel_map: Option<Vec<Option<u16>>>,
}

/// A listener's counters (Milan 1.3, Table 5.10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ListenerStats {
    pub frames_received: u64,
    pub media_locked: u64,
    pub media_unlocked: u64,
    pub interrupted: u64,
    pub sequence_mismatches: u64,
    pub unsupported_formats: u64,
    pub late: u64,
    pub early: u64,
    /// Frames the socket had no room for, its thread behind: not Milan's,
    /// but what tells a stream's frames lost here from lost on the way.
    pub socket_drops: u64,
    /// Frames are arriving.
    pub locked: bool,
    /// The thread taking the frames runs real time.
    pub realtime: bool,
    /// How far the audio device's clock runs from gPTP's, in parts per
    /// million.
    pub drift_ppm: i32,
    /// Times the audio device found nothing to play.
    pub device_underruns: u64,
    /// How long after its presentation time each sample plays, once the
    /// device has said when it plays, and how far from that the last
    /// chunk played, in nanoseconds.
    pub playout: Option<(i64, i64)>,
}

struct Counters {
    frames_received: AtomicU64,
    media_locked: AtomicU64,
    media_unlocked: AtomicU64,
    interrupted: AtomicU64,
    sequence_mismatches: AtomicU64,
    unsupported_formats: AtomicU64,
    late: AtomicU64,
    early: AtomicU64,
    socket_drops: AtomicU64,
    locked: AtomicBool,
    peak: AtomicU32,
    drift_ppm: AtomicI32,
    device_underruns: AtomicU64,
    /// `i64::MIN` until chosen.
    delay: AtomicI64,
    playout_error: AtomicI64,
}

impl Default for Counters {
    fn default() -> Self {
        Counters {
            frames_received: AtomicU64::new(0),
            media_locked: AtomicU64::new(0),
            media_unlocked: AtomicU64::new(0),
            interrupted: AtomicU64::new(0),
            sequence_mismatches: AtomicU64::new(0),
            unsupported_formats: AtomicU64::new(0),
            late: AtomicU64::new(0),
            early: AtomicU64::new(0),
            socket_drops: AtomicU64::new(0),
            locked: AtomicBool::new(false),
            peak: AtomicU32::new(0),
            drift_ppm: AtomicI32::new(0),
            device_underruns: AtomicU64::new(0),
            delay: AtomicI64::new(i64::MIN),
            playout_error: AtomicI64::new(0),
        }
    }
}

/// A stream opened for the thread to take: its socket and its sink, none
/// for a media clock.
struct Opened {
    config: ListenerConfig,
    media: Media,
    receiver: FrameReceiver,
    output: Option<Output>,
}

/// What the group's owner and its thread share.
struct Shared {
    /// Each place's stream opened, waiting for the thread, or `None` to
    /// stop it; and how often they changed, which the thread compares
    /// with what it last took.
    waiting: Mutex<Vec<Option<Option<Opened>>>>,
    changes: AtomicU64,
    /// Each place's counters, made anew for each stream.
    counters: Mutex<Vec<Arc<Counters>>>,
    realtime: AtomicBool,
}

/// A running group of listeners. Dropping it stops them.
pub struct ListenerGroup {
    shared: Arc<Shared>,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl ListenerGroup {
    /// Starts the group's thread, with room for `places` streams, none
    /// taken yet.
    pub fn start(places: usize, clock: Arc<MediaClock>) -> io::Result<Self> {
        let stop = Arc::new(AtomicBool::new(false));
        let shared = Arc::new(Shared {
            waiting: Mutex::new((0..places).map(|_| None).collect()),
            changes: AtomicU64::new(0),
            counters: Mutex::new((0..places).map(|_| Arc::default()).collect()),
            realtime: AtomicBool::new(false),
        });
        let handle = {
            let stop = stop.clone();
            let shared = shared.clone();
            std::thread::Builder::new()
                .name("listener".into())
                .spawn(move || run(&shared, &clock, &stop))?
        };
        Ok(ListenerGroup {
            shared,
            stop,
            handle: Some(handle),
        })
    }

    /// Starts taking `config`'s stream in `place`, in place of what was
    /// there, opening its socket and sink, or stops what was there.
    pub fn set_stream(&self, place: usize, config: Option<ListenerConfig>) -> io::Result<()> {
        let opened = match config {
            Some(config) => {
                let media = Media::of(config.format).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::Unsupported,
                        "not an AAF, AM824 or CRF format this listener takes",
                    )
                })?;
                let receiver =
                    FrameReceiver::open(&config.interface, media.subtype(), &config.destinations)?;
                // The sink has the channels its map has, where it has one.
                let channels = config
                    .channel_map
                    .as_ref()
                    .map_or(media.channels(), |map| map.len() as u16);
                let output = if media.is_clock() {
                    None
                } else {
                    Some(Output::open(&config.sink, channels, media.sample_rate())?)
                };
                Some(Opened {
                    config,
                    media,
                    receiver,
                    output,
                })
            }
            None => None,
        };
        let mut waiting = self
            .shared
            .waiting
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        let Some(slot) = waiting.get_mut(place) else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "no such place"));
        };
        if opened.is_some()
            && let Some(counters) = self
                .shared
                .counters
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .get_mut(place)
        {
            *counters = Arc::default();
        }
        *slot = Some(opened);
        self.shared.changes.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn counters(&self, place: usize) -> Arc<Counters> {
        self.shared
            .counters
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(place)
            .cloned()
            .unwrap_or_default()
    }

    /// `place`'s counters.
    pub fn stats(&self, place: usize) -> ListenerStats {
        let counters = self.counters(place);
        let read = |counter: &AtomicU64| counter.load(Ordering::Relaxed);
        ListenerStats {
            frames_received: read(&counters.frames_received),
            media_locked: read(&counters.media_locked),
            media_unlocked: read(&counters.media_unlocked),
            interrupted: read(&counters.interrupted),
            sequence_mismatches: read(&counters.sequence_mismatches),
            unsupported_formats: read(&counters.unsupported_formats),
            late: read(&counters.late),
            early: read(&counters.early),
            socket_drops: read(&counters.socket_drops),
            locked: counters.locked.load(Ordering::Relaxed),
            realtime: self.shared.realtime.load(Ordering::Relaxed),
            drift_ppm: counters.drift_ppm.load(Ordering::Relaxed),
            device_underruns: read(&counters.device_underruns),
            playout: match counters.delay.load(Ordering::Relaxed) {
                i64::MIN => None,
                delay => Some((delay, counters.playout_error.load(Ordering::Relaxed))),
            },
        }
    }

    /// The loudest sample of `place`'s stream since the last time this was
    /// asked, from 0 to 1.
    pub fn take_peak(&self, place: usize) -> f32 {
        f32::from_bits(self.counters(place).peak.swap(0, Ordering::Relaxed))
    }
}

impl Drop for ListenerGroup {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

/// One of a group's streams, taken while it lives: dropping it stops
/// that stream, and the group once none holds it.
pub struct GroupListener {
    group: Arc<ListenerGroup>,
    place: usize,
}

impl GroupListener {
    /// Starts taking `config`'s stream in `group`'s `place`.
    pub fn start(
        group: &Arc<ListenerGroup>,
        place: usize,
        config: ListenerConfig,
    ) -> io::Result<Self> {
        group.set_stream(place, Some(config))?;
        Ok(GroupListener {
            group: group.clone(),
            place,
        })
    }

    pub fn stats(&self) -> ListenerStats {
        self.group.stats(self.place)
    }

    /// The loudest sample since the last time this was asked, from 0 to 1.
    pub fn take_peak(&self) -> f32 {
        self.group.take_peak(self.place)
    }
}

impl Drop for GroupListener {
    fn drop(&mut self) {
        let _ = self.group.set_stream(self.place, None);
    }
}

/// A running listener of one stream. Dropping it stops listening.
pub struct Listener {
    group: GroupListener,
}

impl Listener {
    pub fn start(config: ListenerConfig, clock: Arc<MediaClock>) -> io::Result<Self> {
        let group = Arc::new(ListenerGroup::start(1, clock)?);
        Ok(Listener {
            group: GroupListener::start(&group, 0, config)?,
        })
    }

    pub fn stats(&self) -> ListenerStats {
        self.group.stats()
    }

    /// The loudest sample since the last time this was asked, from 0 to 1.
    pub fn take_peak(&self) -> f32 {
        self.group.take_peak()
    }
}

/// A stream the thread takes.
struct Running {
    opened: Opened,
    counters: Arc<Counters>,
    expected: Option<u8>,
    last: Instant,
    /// Its samples in the order its channel map plays them.
    mapped: Vec<f32>,
}

impl Drop for Running {
    /// The clock a stream recovered goes with it, however it stops: in
    /// place of another, or with the group's thread.
    fn drop(&mut self) {
        if let Some(recovered) = &self.opened.config.recovered {
            recovered.lose();
        }
    }
}

fn run(shared: &Shared, clock: &MediaClock, stop: &AtomicBool) {
    shared
        .realtime
        .store(crate::realtime::raise(), Ordering::Relaxed);
    let places = shared
        .waiting
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .len();
    let mut running: Vec<Option<Running>> = (0..places).map(|_| None).collect();
    let mut seen = u64::MAX;
    let mut frame = vec![0u8; 1522];
    let mut samples = Vec::new();
    let mut drops_read = Instant::now();
    while !stop.load(Ordering::Relaxed) {
        let changes = shared.changes.load(Ordering::Relaxed);
        if changes != seen {
            seen = changes;
            take_changes(shared, &mut running);
        }
        let taking: Vec<usize> = (0..places)
            .filter(|place| running[*place].is_some())
            .collect();
        if taking.is_empty() {
            std::thread::sleep(LONGEST_WAIT);
            continue;
        }
        let receivers: Vec<&FrameReceiver> = taking
            .iter()
            .filter_map(|place| running[*place].as_ref())
            .map(|stream| &stream.opened.receiver)
            .collect();
        let ready = match FrameReceiver::ready(&receivers, Some(LONGEST_WAIT)) {
            Ok(ready) => ready,
            Err(_) => {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
        };
        for at in ready {
            let Some(stream) = running[taking[at]].as_mut() else {
                continue;
            };
            // Every frame the socket holds.
            while let Ok(Some(length)) = stream
                .opened
                .receiver
                .receive(&mut frame, Some(Duration::ZERO))
            {
                take_frame(stream, &frame[..length], &mut samples, clock);
            }
        }
        // A stream gone quiet is interrupted.
        for stream in running.iter_mut().flatten() {
            let counters = &stream.counters;
            if counters.locked.load(Ordering::Relaxed) && stream.last.elapsed() > SILENCE {
                counters.locked.store(false, Ordering::Relaxed);
                counters.media_unlocked.fetch_add(1, Ordering::Relaxed);
                counters.interrupted.fetch_add(1, Ordering::Relaxed);
                stream.expected = None;
                if let Some(recovered) = &stream.opened.config.recovered {
                    recovered.lose();
                }
            }
        }
        if drops_read.elapsed() >= DROPS_EVERY {
            drops_read = Instant::now();
            for stream in running.iter().flatten() {
                let dropped = stream.opened.receiver.dropped();
                if dropped > 0 {
                    stream
                        .counters
                        .socket_drops
                        .fetch_add(dropped, Ordering::Relaxed);
                }
            }
        }
    }
}

/// Takes the streams opened for it and stops those asked to stop.
fn take_changes(shared: &Shared, running: &mut [Option<Running>]) {
    let mut waiting = shared
        .waiting
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    let counters = shared
        .counters
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    for (place, slot) in waiting.iter_mut().enumerate() {
        let Some(change) = slot.take() else {
            continue;
        };
        if let Some(stream) = running.get_mut(place) {
            *stream = change.map(|opened| Running {
                opened,
                counters: counters.get(place).cloned().unwrap_or_default(),
                expected: None,
                last: Instant::now(),
                mapped: Vec::new(),
            });
        }
    }
}

/// Counts one frame of a stream and plays its samples.
fn take_frame(stream: &mut Running, frame: &[u8], samples: &mut Vec<f32>, clock: &MediaClock) {
    let Running {
        opened,
        counters,
        expected,
        last,
        mapped,
    } = stream;
    samples.clear();
    let Some(pdu) = avtp_payload(frame) else {
        return;
    };
    let Some(header) = media::read(pdu, &opened.media, samples) else {
        return;
    };
    if header.stream_id != opened.config.stream_id {
        return;
    }
    if !header.matches {
        counters.unsupported_formats.fetch_add(1, Ordering::Relaxed);
        return;
    }
    let now = clock.now();
    // How far ahead a presentation time is, from the low 32 bits of gPTP
    // time.
    let ahead = |timestamp: u32| i64::from(timestamp.wrapping_sub(now as u32) as i32);
    // The clock the stream carries: a media clock's timestamps, or an
    // audio stream's first sample's presentation time, each frame its
    // samples on, made whole from gPTP time, which a hardware clock must
    // give for its high bits to be right.
    if let Some(recovered) = &opened.config.recovered {
        match &opened.media {
            Media::Crf(layout) => {
                if let Some((crf, timestamps)) = crf::Header::decode(pdu) {
                    let times = timestamps
                        .as_chunks::<8>()
                        .0
                        .iter()
                        .map(|time| u64::from_be_bytes(*time));
                    recovered.take(
                        times,
                        layout.timestamp_interval,
                        layout.base_frequency,
                        crf.media_reset,
                    );
                }
            }
            media => {
                if let Some(first) = header.first_presented.filter(|_| clock.locked()) {
                    recovered.take(
                        [(now + ahead(first)) as u64],
                        media.samples_per_frame(),
                        media.sample_rate(),
                        header.media_reset,
                    );
                }
            }
        }
    }
    *last = Instant::now();
    counters.frames_received.fetch_add(1, Ordering::Relaxed);
    if expected.is_some_and(|wanted| wanted != header.sequence) {
        counters.sequence_mismatches.fetch_add(1, Ordering::Relaxed);
    }
    *expected = Some(header.sequence.wrapping_add(1));
    if !counters.locked.load(Ordering::Relaxed) {
        counters.locked.store(true, Ordering::Relaxed);
        counters.media_locked.fetch_add(1, Ordering::Relaxed);
    }
    if let Some(timestamp) = header.timestamp {
        let ahead = ahead(timestamp);
        if ahead < 0 {
            counters.late.fetch_add(1, Ordering::Relaxed);
        } else if ahead > EARLY {
            counters.early.fetch_add(1, Ordering::Relaxed);
        }
    }
    let loudest = samples
        .iter()
        .fold(0.0f32, |most, sample| most.max(sample.abs()));
    counters
        .peak
        .fetch_max(loudest.to_bits(), Ordering::Relaxed);
    // Without gPTP time from a hardware clock the presentation times mean
    // nothing here, and the device plays as frames come.
    let presented = header
        .first_presented
        .filter(|_| clock.locked())
        .map(|first| Presented {
            first: now + ahead(first),
            offset: clock.offset(),
        });
    let Some(output) = &opened.output else {
        return;
    };
    let samples = match &opened.config.channel_map {
        Some(map) => {
            remap(samples, opened.media.channels(), map, mapped);
            &*mapped
        }
        None => &*samples,
    };
    output.write(samples, presented);
    if header.sequence == 0 {
        counters
            .drift_ppm
            .store(output.drift_ppm(), Ordering::Relaxed);
        counters
            .device_underruns
            .store(output.underruns(), Ordering::Relaxed);
        if let Some((delay, error)) = output.playout() {
            counters.delay.store(delay, Ordering::Relaxed);
            counters.playout_error.store(error, Ordering::Relaxed);
        }
    }
}

/// `samples`, frames of `channels` interleaved, into `out` as `map` plays
/// them: each of its channels the stream channel it names, silence where
/// it names none or one the stream lacks.
fn remap(samples: &[f32], channels: u16, map: &[Option<u16>], out: &mut Vec<f32>) {
    let channels = usize::from(channels.max(1));
    out.clear();
    for frame in samples.chunks_exact(channels) {
        out.extend(map.iter().map(|channel| {
            channel
                .and_then(|channel| frame.get(usize::from(channel)))
                .copied()
                .unwrap_or(0.0)
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A channel map plays the stream's channels where it names them, and
    /// silence elsewhere.
    #[test]
    fn a_channel_map_moves_and_silences_channels() {
        let samples = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6];
        let mut out = Vec::new();
        remap(&samples, 3, &[Some(2), None, Some(0)], &mut out);
        assert_eq!(out, [0.3, 0.0, 0.1, 0.6, 0.0, 0.4]);
    }
}
