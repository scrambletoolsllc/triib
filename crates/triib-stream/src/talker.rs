//! Talkers of AAF or AM824: one frame each class interval, paced on gPTP
//! time, its presentation time the max transit time ahead.
//!
//! A group sends several streams from one source on one thread, each
//! taking some of the source's channels. Their samples are counted on one
//! clock, sample `s` going out at the group's origin plus `s` sample
//! periods, so the streams present together, sample for sample, and the
//! source keeps one buffer and one drift estimate for all of them. A
//! [`Talker`] is a group of one.

use std::collections::VecDeque;
use std::io;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;
use std::time::Duration;

use atdecc::stream_format::StreamFormat;
use avb_net::MacAddress;
use avb_net::clock::{monotonic_now, precise_sleeps};
use avb_net::stream::FrameSender;

use crate::MediaClock;
use crate::audio::{Input, Source};
use crate::media::{Media, Packetizer};

/// The VLAN tag's ethertype, and AVTP's.
const ETHERTYPE_VLAN: u16 = 0x8100;
const ETHERTYPE_AVTP: u16 = 0x22f0;
/// The Ethernet header with its VLAN tag.
const ETHERNET_LEN: usize = 18;
/// How long a frame may take to cross the network to its listeners, in
/// nanoseconds: a frame sent later than this before its presentation time
/// would be late.
const CROSSING: i64 = 500_000;
/// The longest the pacing thread sleeps at once, so that it follows a
/// clock that moved.
const LONGEST_SLEEP: Duration = Duration::from_millis(20);
/// Streams start on a sample a multiple of this, sent on a millisecond:
/// every frame size of 6 to 48 samples divides it, so each stream's
/// frames go out on its class interval's boundaries.
const ALIGN: u64 = 48;
const NANOS: u128 = 1_000_000_000;

/// What a talker sends, and from where.
#[derive(Debug, Clone, PartialEq)]
pub struct TalkerConfig {
    pub interface: String,
    /// The interface's address, the frames' source.
    pub mac: MacAddress,
    pub stream_id: u64,
    /// Where the stream goes: its multicast address, or in AVB Lite a
    /// unicast copy to each listener. [`Talker::set_destinations`] moves
    /// it while it runs.
    pub destinations: Vec<MacAddress>,
    pub vlan_id: u16,
    pub priority: u8,
    pub format: StreamFormat,
    /// How far ahead of sending the presentation times are.
    pub max_transit_time: Duration,
    pub source: Source,
}

/// Where a group's streams take their samples: the interface they go
/// out on, and a source of `channels` channels at `rate` hertz.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupConfig {
    pub interface: String,
    /// The interface's address, the frames' source.
    pub mac: MacAddress,
    pub source: Source,
    pub channels: u16,
    pub rate: u32,
}

/// One of a group's streams.
#[derive(Debug, Clone, PartialEq)]
pub struct GroupStream {
    pub stream_id: u64,
    /// Where the stream goes: its multicast address, or in AVB Lite a
    /// unicast copy to each listener.
    pub destinations: Vec<MacAddress>,
    pub vlan_id: u16,
    pub priority: u8,
    /// Its format, at the group's rate.
    pub format: StreamFormat,
    /// How far ahead of sending the presentation times are.
    pub max_transit_time: Duration,
    /// The group's channel that is its first.
    pub first_channel: u16,
}

/// A talker's counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TalkerStats {
    pub frames_sent: u64,
    /// Frames that went out too late and started the media clock again.
    pub media_resets: u64,
    /// Times the input dropped frames as its device ran far ahead, and
    /// reads that found it empty.
    pub adjustments: u64,
    pub underruns: u64,
    /// Frames the interface would not take.
    pub send_errors: u64,
    /// The thread pacing the frames runs real time.
    pub realtime: bool,
    /// How far the audio device's clock runs from gPTP's, in parts per
    /// million.
    pub drift_ppm: i32,
}

/// A stream's own counters.
#[derive(Default)]
struct StreamCounters {
    frames_sent: AtomicU64,
    send_errors: AtomicU64,
}

/// The group's counters, which its streams share.
#[derive(Default)]
struct GroupCounters {
    media_resets: AtomicU64,
    adjustments: AtomicU64,
    underruns: AtomicU64,
    realtime: AtomicBool,
    drift_ppm: AtomicI32,
}

/// What the group's owner and its thread share.
struct Shared {
    /// The streams wanted, by place, and how often they changed, which
    /// the thread compares with what it last took.
    streams: Mutex<Vec<Option<GroupStream>>>,
    changes: AtomicU64,
    counters: Vec<StreamCounters>,
    group: GroupCounters,
}

impl Shared {
    fn lock(&self) -> std::sync::MutexGuard<'_, Vec<Option<GroupStream>>> {
        self.streams.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// A running group of streams. Dropping it stops them.
pub struct TalkerGroup {
    shared: Arc<Shared>,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
    rate: u32,
}

impl TalkerGroup {
    /// Opens the group's source and starts its thread, with room for
    /// `places` streams, none sending yet.
    pub fn start(config: GroupConfig, places: usize, clock: Arc<MediaClock>) -> io::Result<Self> {
        let sender = FrameSender::open(&config.interface)?;
        let input = Input::open(&config.source, config.channels.max(1), config.rate)?;
        let stop = Arc::new(AtomicBool::new(false));
        let shared = Arc::new(Shared {
            streams: Mutex::new(vec![None; places]),
            changes: AtomicU64::new(0),
            counters: (0..places).map(|_| StreamCounters::default()).collect(),
            group: GroupCounters::default(),
        });
        let rate = config.rate;
        let handle = {
            let stop = stop.clone();
            let shared = shared.clone();
            std::thread::Builder::new()
                .name("talker".into())
                .spawn(move || run(&config, &shared, &sender, &clock, &stop, input))?
        };
        Ok(TalkerGroup {
            shared,
            stop,
            handle: Some(handle),
            rate,
        })
    }

    /// The sampling rate its source runs at.
    pub fn rate(&self) -> u32 {
        self.rate
    }

    /// Starts sending `stream` in `place`, in place of what went there,
    /// or stops what went there.
    pub fn set_stream(&self, place: usize, stream: Option<GroupStream>) -> io::Result<()> {
        if let Some(stream) = &stream {
            let media = Media::of(stream.format).ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Unsupported,
                    "not an AAF or AM824 format this talker sends",
                )
            })?;
            if media.sample_rate() != self.rate {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "a stream at another rate than its group's",
                ));
            }
        }
        let mut streams = self.shared.lock();
        let Some(slot) = streams.get_mut(place) else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "no such place"));
        };
        *slot = stream;
        self.shared.changes.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// Sends `place`'s stream to `destinations` from its next frame on,
    /// none pausing it.
    pub fn set_destinations(&self, place: usize, destinations: Vec<MacAddress>) {
        let mut streams = self.shared.lock();
        if let Some(Some(stream)) = streams.get_mut(place) {
            stream.destinations = destinations;
            self.shared.changes.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Whether any of its streams sends.
    pub fn sending(&self) -> bool {
        self.shared.lock().iter().any(Option::is_some)
    }

    /// Whether `place`'s stream sends.
    pub fn sends(&self, place: usize) -> bool {
        self.shared.lock().get(place).is_some_and(Option::is_some)
    }

    /// `place`'s counters, with the group's it shares.
    pub fn stats(&self, place: usize) -> TalkerStats {
        let read = |counter: &AtomicU64| counter.load(Ordering::Relaxed);
        let group = &self.shared.group;
        let (frames_sent, send_errors) =
            self.shared.counters.get(place).map_or((0, 0), |counters| {
                (read(&counters.frames_sent), read(&counters.send_errors))
            });
        TalkerStats {
            frames_sent,
            media_resets: read(&group.media_resets),
            adjustments: read(&group.adjustments),
            underruns: read(&group.underruns),
            send_errors,
            realtime: group.realtime.load(Ordering::Relaxed),
            drift_ppm: group.drift_ppm.load(Ordering::Relaxed),
        }
    }
}

impl Drop for TalkerGroup {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

/// One of a group's streams, sending while it lives: dropping it stops
/// that stream, and the group once none holds it.
pub struct GroupTalker {
    group: Arc<TalkerGroup>,
    place: usize,
}

impl GroupTalker {
    /// Starts `stream` in `group`'s `place`.
    pub fn start(group: &Arc<TalkerGroup>, place: usize, stream: GroupStream) -> io::Result<Self> {
        group.set_stream(place, Some(stream))?;
        Ok(GroupTalker {
            group: group.clone(),
            place,
        })
    }

    /// Sends the stream to `destinations` from its next frame on, none
    /// pausing it.
    pub fn set_destinations(&self, destinations: Vec<MacAddress>) {
        self.group.set_destinations(self.place, destinations);
    }

    pub fn stats(&self) -> TalkerStats {
        self.group.stats(self.place)
    }
}

impl Drop for GroupTalker {
    fn drop(&mut self) {
        let _ = self.group.set_stream(self.place, None);
    }
}

/// A running talker of one stream. Dropping it stops the stream.
pub struct Talker {
    group: TalkerGroup,
}

impl Talker {
    /// Starts sending `config`'s stream on `clock`'s time.
    pub fn start(config: TalkerConfig, clock: Arc<MediaClock>) -> io::Result<Self> {
        let media = Media::of(config.format).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Unsupported,
                "not an AAF or AM824 format this talker sends",
            )
        })?;
        let group = TalkerGroup::start(
            GroupConfig {
                interface: config.interface,
                mac: config.mac,
                source: config.source,
                channels: media.channels(),
                rate: media.sample_rate(),
            },
            1,
            clock,
        )?;
        group.set_stream(
            0,
            Some(GroupStream {
                stream_id: config.stream_id,
                destinations: config.destinations,
                vlan_id: config.vlan_id,
                priority: config.priority,
                format: config.format,
                max_transit_time: config.max_transit_time,
                first_channel: 0,
            }),
        )?;
        Ok(Talker { group })
    }

    /// Sends the stream to `destinations` from the next frame on, none
    /// pausing it.
    pub fn set_destinations(&self, destinations: Vec<MacAddress>) {
        self.group.set_destinations(0, destinations);
    }

    pub fn stats(&self) -> TalkerStats {
        self.group.stats(0)
    }
}

/// The frame's Ethernet header: destination, source, VLAN tag, ethertype.
fn ethernet_header(mac: MacAddress, stream: &GroupStream) -> [u8; ETHERNET_LEN] {
    let mut header = [0; ETHERNET_LEN];
    header[..6].copy_from_slice(&stream.destinations.first().map_or([0; 6], |mac| mac.0));
    header[6..12].copy_from_slice(&mac.0);
    header[12..14].copy_from_slice(&ETHERTYPE_VLAN.to_be_bytes());
    let tag = (u16::from(stream.priority & 0x7) << 13) | (stream.vlan_id & 0x0fff);
    header[14..16].copy_from_slice(&tag.to_be_bytes());
    header[16..18].copy_from_slice(&ETHERTYPE_AVTP.to_be_bytes());
    header
}

/// A stream the thread sends.
struct Running {
    stream: GroupStream,
    media: Media,
    packetizer: Packetizer,
    frame: Vec<u8>,
    samples: Vec<f32>,
    /// The first sample of its next frame.
    next: u64,
}

impl Running {
    fn new(mac: MacAddress, stream: GroupStream, media: Media) -> Self {
        let mut frame = vec![0u8; ETHERNET_LEN + media.pdu_length()];
        frame[..ETHERNET_LEN].copy_from_slice(&ethernet_header(mac, &stream));
        let samples =
            vec![0.0f32; usize::from(media.channels()) * usize::from(media.samples_per_frame())];
        Running {
            packetizer: Packetizer::new(media, stream.stream_id),
            stream,
            media,
            frame,
            samples,
            next: 0,
        }
    }

    fn frame_samples(&self) -> u64 {
        u64::from(self.media.samples_per_frame().max(1))
    }

    /// Its next frame's first sample at or after sample `from`.
    fn first_from(&self, from: u64) -> u64 {
        from.div_ceil(self.frame_samples()) * self.frame_samples()
    }
}

/// The group's clock: sample `s` goes out at gPTP time `origin` plus `s`
/// sample periods.
struct Timeline {
    rate: u64,
    origin: i64,
}

impl Timeline {
    fn at(&self, sample: u64) -> i64 {
        self.origin + (u128::from(sample) * NANOS / u128::from(self.rate)) as i64
    }

    /// The first sample sent at or after gPTP time `time`.
    fn sample_at(&self, time: i64) -> u64 {
        let since = (time - self.origin).max(0);
        (since as u128 * u128::from(self.rate)).div_ceil(NANOS) as u64
    }

    /// Starts again so that sample `from`, rounded up to the alignment,
    /// goes out on a millisecond at least a millisecond after `now`;
    /// gives that sample.
    fn restart(&mut self, from: u64, now: i64) -> u64 {
        let first = from.div_ceil(ALIGN) * ALIGN;
        let boundary = (now / 1_000_000 + 2) * 1_000_000;
        self.origin = boundary - (u128::from(first) * NANOS / u128::from(self.rate)) as i64;
        first
    }
}

/// Takes the streams wanted: those gone stop, new ones start on their
/// next frame a millisecond on, changed ones start again, and those whose
/// destinations moved send there.
fn take_changes(
    config: &GroupConfig,
    shared: &Shared,
    running: &mut [Option<Running>],
    timeline: &mut Timeline,
    read: u64,
    base: u64,
    clock: &MediaClock,
) {
    let wanted = shared.lock().clone();
    let idle = running.iter().all(Option::is_none);
    let mut from = None;
    for (place, wanted) in wanted.into_iter().enumerate() {
        let Some(slot) = running.get_mut(place) else {
            continue;
        };
        match (wanted, slot.as_mut()) {
            (None, _) => *slot = None,
            (Some(stream), Some(now)) if same_but_destinations(&stream, &now.stream) => {
                if stream.destinations != now.stream.destinations {
                    now.frame[..ETHERNET_LEN]
                        .copy_from_slice(&ethernet_header(config.mac, &stream));
                    now.stream.destinations = stream.destinations;
                }
            }
            (Some(stream), _) => {
                let Some(media) = Media::of(stream.format) else {
                    *slot = None;
                    continue;
                };
                // The first stream after none starts the clock again, so
                // the source is read from now on.
                let start = *from.get_or_insert_with(|| {
                    if idle {
                        timeline.restart(read, clock.now())
                    } else {
                        timeline.sample_at(clock.now() + 1_000_000).max(base)
                    }
                });
                let mut made = Running::new(config.mac, stream, media);
                made.next = made.first_from(start);
                *slot = Some(made);
            }
        }
    }
}

/// Whether two streams differ in their destinations alone.
fn same_but_destinations(one: &GroupStream, other: &GroupStream) -> bool {
    GroupStream {
        destinations: Vec::new(),
        ..one.clone()
    } == GroupStream {
        destinations: Vec::new(),
        ..other.clone()
    }
}

fn run(
    config: &GroupConfig,
    shared: &Shared,
    sender: &FrameSender,
    clock: &MediaClock,
    stop: &AtomicBool,
    mut input: Input,
) {
    precise_sleeps();
    shared
        .group
        .realtime
        .store(crate::realtime::raise(), Ordering::Relaxed);
    let channels = usize::from(config.channels.max(1));
    let mut running: Vec<Option<Running>> = (0..shared.counters.len()).map(|_| None).collect();
    let mut timeline = Timeline {
        rate: u64::from(config.rate.max(1)),
        origin: 0,
    };
    let mut seen = u64::MAX;
    // The samples read from the source, from `base` up to `read`, every
    // channel interleaved.
    let mut held: VecDeque<f32> = VecDeque::new();
    let mut base: u64 = 0;
    let mut read: u64 = 0;
    let mut chunk: Vec<f32> = Vec::new();
    let mut media_reset = false;
    let mut frames: u64 = 0;
    while !stop.load(Ordering::Relaxed) {
        let changes = shared.changes.load(Ordering::Relaxed);
        if changes != seen {
            seen = changes;
            take_changes(
                config,
                shared,
                &mut running,
                &mut timeline,
                read,
                base,
                clock,
            );
        }
        // The stream whose frame is due first.
        let Some((place, due)) = running
            .iter()
            .enumerate()
            .filter_map(|(place, stream)| {
                stream
                    .as_ref()
                    .map(|stream| (place, timeline.at(stream.next)))
            })
            .min_by_key(|(_, due)| *due)
        else {
            std::thread::sleep(LONGEST_SLEEP);
            continue;
        };
        let wake = clock.monotonic_at(due);
        let now = monotonic_now();
        if wake > now {
            let wait = wake - now;
            std::thread::sleep(wait.min(LONGEST_SLEEP));
            if wait > LONGEST_SLEEP {
                // Too far ahead for the clock to have only drifted: it
                // went back, and the streams start again from it.
                if due - clock.now() > 2 * LONGEST_SLEEP.as_nanos() as i64 {
                    restart(&mut running, &mut timeline, read, clock);
                    media_reset = !media_reset;
                    shared.group.media_resets.fetch_add(1, Ordering::Relaxed);
                }
                continue;
            }
        }
        let Some(stream) = running[place].as_mut() else {
            continue;
        };
        // A frame sent late still arrives in time while its presentation
        // time is far enough ahead.
        let transit = stream.stream.max_transit_time.as_nanos() as i64;
        let late = clock.now() - due;
        if late > (transit - CROSSING).max(transit / 2) {
            // Too late for the presentation time to mean anything: the
            // media clock starts again, which the mr bit tells.
            restart(&mut running, &mut timeline, read, clock);
            media_reset = !media_reset;
            shared.group.media_resets.fetch_add(1, Ordering::Relaxed);
            continue;
        }
        // The source read as far as this frame's last sample.
        let end = stream.next + stream.frame_samples();
        if end > read {
            chunk.resize((end - read) as usize * channels, 0.0);
            input.read(&mut chunk);
            held.extend(chunk.iter().copied());
            read = end;
        }
        // Its channels of the frame's samples.
        let stream_channels = usize::from(stream.media.channels());
        let first = usize::from(stream.stream.first_channel);
        let offset = (stream.next - base) as usize * channels;
        for (index, sample) in stream.samples.iter_mut().enumerate() {
            let (frame, channel) = (index / stream_channels, index % stream_channels);
            *sample = if first + channel < channels {
                held.get(offset + frame * channels + first + channel)
                    .copied()
                    .unwrap_or(0.0)
            } else {
                0.0
            };
        }
        stream.packetizer.write(
            due + transit,
            media_reset,
            &stream.samples,
            &mut stream.frame[ETHERNET_LEN..],
        );
        let counters = &shared.counters[place];
        for destination in &stream.stream.destinations {
            stream.frame[..6].copy_from_slice(&destination.0);
            match sender.send(&stream.frame) {
                Ok(()) => counters.frames_sent.fetch_add(1, Ordering::Relaxed),
                Err(_) => counters.send_errors.fetch_add(1, Ordering::Relaxed),
            };
        }
        stream.next = end;
        // The samples every stream has sent go.
        let needed = running
            .iter()
            .flatten()
            .map(|stream| stream.next)
            .min()
            .unwrap_or(read)
            .clamp(base, read);
        held.drain(..(needed - base) as usize * channels);
        base = needed;
        frames += 1;
        if frames.is_multiple_of(800) {
            // Each tenth of a second at 8000 frames a second, the input's
            // own counts.
            let (made, empty) = input.counts();
            shared.group.adjustments.store(made, Ordering::Relaxed);
            shared
                .group
                .drift_ppm
                .store(input.drift_ppm(), Ordering::Relaxed);
            shared.group.underruns.store(empty, Ordering::Relaxed);
        }
    }
}

/// Starts every stream again from the samples read so far, on the clock
/// as it is now.
fn restart(
    running: &mut [Option<Running>],
    timeline: &mut Timeline,
    read: u64,
    clock: &MediaClock,
) {
    let first = timeline.restart(read, clock.now());
    for stream in running.iter_mut().flatten() {
        stream.next = stream.first_from(first);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Samples map to gPTP time exactly, every stream's frames on its
    /// interval: 6 samples at 48 kHz each 125 µs, 12 each 250 µs.
    #[test]
    fn the_timeline_counts_samples_on_gptp_time() {
        let mut timeline = Timeline {
            rate: 48_000,
            origin: 0,
        };
        let first = timeline.restart(5, 7_400_000);
        assert_eq!(first, 48);
        assert_eq!(timeline.at(first), 9_000_000);
        assert_eq!(timeline.at(first + 6) - timeline.at(first), 125_000);
        assert_eq!(timeline.at(first + 12) - timeline.at(first), 250_000);
        assert_eq!(timeline.sample_at(timeline.at(first + 6)), first + 6);
        // A time between samples gives the next sample.
        assert_eq!(timeline.sample_at(timeline.at(first) + 1), first + 1);
    }
}
