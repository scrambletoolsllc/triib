//! The Streams tab's meters: how loud the first two channels of each of
//! this computer's running streams are, as triib-endpointd measures them,
//! or on a Mac a helper does from the Mac's own entity's audio device,
//! shown too on the streams of other entities connected to them, which
//! carry the same audio.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use atdecc::EntityId;
use iced::widget::container;
use iced::{Color, Element, Theme};
use scramble_ui::{Scheme, shape};
use triib_endpointd::meters::{EVERY, Meters};

use crate::app::{Message, Triib};

/// The quietest level a meter shows, in dBFS.
pub const FLOOR: f32 = -60.0;
/// Where a bar turns from green to yellow, and to red, as shares of its
/// length: -12 dBFS and -6 dBFS.
const YELLOW_FROM: f32 = 0.8;
const RED_FROM: f32 = 0.9;
/// How fast a meter falls back from a peak, in dB a second.
const FALL: f32 = 24.0;
/// How long without a new reading before the levels shown are forgotten
/// rather than falling from where they stood, and the daemon's meters
/// count as stopped.
const STALE: Duration = Duration::from_millis(500);
/// How often the app asks the daemon to go on writing its meters.
const ASK_AGAIN: Duration = Duration::from_secs(1);

/// A meter's width, its bars' height and the gap between them.
pub const WIDTH: f32 = 32.0;
const BAR: f32 = 4.0;
const GAP: f32 = 2.0;

/// A stream: its entity, whether it is an input, and its index.
pub type StreamKey = (EntityId, bool, u16);

/// Who measures a reading.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Origin {
    /// triib-endpointd, of the entities it runs.
    Daemon,
    /// The helper, of the Mac's own entity.
    #[cfg(target_os = "macos")]
    Mac,
}

/// What the meters show: each running stream's first two channels in
/// dBFS, by who measures them.
#[derive(Debug, Default)]
pub struct Levels {
    taken: BTreeMap<Origin, Taken>,
}

/// One origin's levels, and when they were last read.
#[derive(Debug, Default)]
struct Taken {
    levels: BTreeMap<StreamKey, [f32; 2]>,
    read_at: Option<Instant>,
}

impl Levels {
    /// Takes a reading from `origin`, at `now`: each stream's meter rises
    /// at once to its new peak or falls toward it at [`FALL`], and a
    /// stream no longer running drops out.
    pub fn take(&mut self, origin: Origin, reading: Option<Meters>, now: Instant) {
        self.taken.entry(origin).or_default().take(reading, now);
    }

    /// `stream`'s levels, while it runs.
    pub fn get(&self, stream: StreamKey) -> Option<[f32; 2]> {
        self.taken
            .values()
            .find_map(|taken| taken.levels.get(&stream).copied())
    }
}

impl Taken {
    fn take(&mut self, reading: Option<Meters>, now: Instant) {
        let fall = match self.read_at {
            Some(at) if now.duration_since(at) < STALE => {
                FALL * now.duration_since(at).as_secs_f32()
            }
            _ => f32::INFINITY,
        };
        self.read_at = Some(now);
        let mut levels = BTreeMap::new();
        for stream in reading.into_iter().flat_map(|meters| meters.streams) {
            let Ok(entity_id) = stream.entity_id.parse::<EntityId>() else {
                continue;
            };
            let key = (entity_id, stream.input, stream.index);
            let mut level = stream.peaks.map(decibels);
            if let Some(before) = self.levels.get(&key) {
                for (level, before) in level.iter_mut().zip(before) {
                    *level = level.max(before - fall);
                }
            }
            levels.insert(key, level);
        }
        self.levels = levels;
    }
}

/// The Mac's own entity's meters as the helper measures them, each stream
/// by its place among the device's streams that way, given instead by its
/// index among the entity's audio streams that way: macOS gives its media
/// clock streams no channels.
#[cfg(target_os = "macos")]
pub fn by_stream_index(triib: &Triib, mut meters: Meters) -> Meters {
    meters.streams.retain_mut(|stream| {
        let Some(model) = stream
            .entity_id
            .parse::<EntityId>()
            .ok()
            .and_then(|entity_id| triib.models.get(&entity_id))
        else {
            return false;
        };
        let index = model
            .streams(stream.input)
            .filter(|descriptor| !descriptor.is_media_clock())
            .nth(usize::from(stream.index))
            .map(|descriptor| descriptor.index);
        index.inspect(|&index| stream.index = index).is_some()
    });
    meters
}

/// A peak from 0 to 1 in dBFS, no lower than [`FLOOR`].
fn decibels(peak: f32) -> f32 {
    if peak > 0.0 {
        (20.0 * peak.log10()).max(FLOOR)
    } else {
        FLOOR
    }
}

/// Each new write of the daemon's meters, read every [`EVERY`] while the
/// subscription runs, asking the daemon to go on writing them; `None`
/// while there are none, or once it stopped writing them.
pub fn readings() -> impl iced::futures::Stream<Item = Option<Meters>> {
    let (mut sender, receiver) = iced::futures::channel::mpsc::channel(1);
    let _ = std::thread::Builder::new()
        .name("meters".into())
        .spawn(move || {
            let mut asked: Option<Instant> = None;
            // The write last read and when it was new, and what went to
            // the app last.
            let mut seen: Option<u64> = None;
            let mut new_at = Instant::now();
            let mut sent: Option<Option<u64>> = None;
            while !sender.is_closed() {
                if asked.is_none_or(|at| at.elapsed() >= ASK_AGAIN) {
                    triib_endpointd::meters::want();
                    asked = Some(Instant::now());
                }
                let reading = triib_endpointd::meters::read();
                let sequence = reading.as_ref().map(|meters| meters.sequence);
                if sequence != seen {
                    seen = sequence;
                    new_at = Instant::now();
                }
                let reading = reading.filter(|_| new_at.elapsed() < STALE);
                let sequence = reading.as_ref().map(|meters| meters.sequence);
                // A reading the app has already taken, or one it is busy
                // with, waits for the next.
                if sent != Some(sequence) && sender.try_send(reading).is_ok() {
                    sent = Some(sequence);
                }
                std::thread::sleep(EVERY);
            }
        });
    receiver
}

/// Which of this computer's streams carries the audio of `entity_id`'s
/// stream, for its meter: the stream itself, on one of this computer's
/// entities; for another's input, the output of this computer's it is
/// connected to; for another's output, an input of this computer's
/// connected to it. `None` where none does.
pub fn source(triib: &Triib, entity_id: EntityId, input: bool, index: u16) -> Option<StreamKey> {
    // This computer's entities: those it runs, and the Mac's own.
    let ours = |entity_id| triib.is_host(entity_id) || triib.is_this_mac(entity_id);
    if ours(entity_id) {
        return Some((entity_id, input, index));
    }
    if input {
        let (talker, output) = triib
            .models
            .get(&entity_id)?
            .binding(index)?
            .talker_stream()?;
        return ours(talker).then_some((talker, false, output));
    }
    triib
        .models
        .iter()
        .filter(|(host, _)| ours(**host))
        .find_map(|(host, model)| {
            model
                .streams(true)
                .find(|stream| {
                    model
                        .binding(stream.index)
                        .and_then(|binding| binding.talker_stream())
                        == Some((entity_id, index))
                })
                .map(|stream| (*host, true, stream.index))
        })
}

/// A meter of a stream's first `channels` channels, up to two, a bar each,
/// filled to their `levels` in dBFS from [`FLOOR`]; empty while the stream
/// does not run.
pub fn meter<'a>(levels: Option<[f32; 2]>, channels: u16) -> Element<'a, Message> {
    let levels = levels.unwrap_or([FLOOR; 2]);
    let bars = levels
        .into_iter()
        .take(usize::from(channels.clamp(1, 2)))
        .map(|level| bar(level).into());
    container(iced::widget::Column::with_children(bars).spacing(GAP))
        .width(WIDTH)
        .into()
}

/// One channel's bar, filled to `level`: green, then yellow from
/// [`YELLOW_FROM`] of its length and red from [`RED_FROM`].
fn bar<'a>(level: f32) -> iced::widget::Container<'a, Message> {
    let share = ((level - FLOOR) / -FLOOR).clamp(0.0, 1.0);
    // How much of each color the fill takes.
    let parts = [
        share.min(YELLOW_FROM),
        (share.min(RED_FROM) - YELLOW_FROM).max(0.0),
        (share - RED_FROM).max(0.0),
    ];
    let last = parts.iter().rposition(|part| *part > 0.0);
    let segments = parts
        .into_iter()
        .enumerate()
        .filter(|(_, part)| *part > 0.0)
        .map(|(zone, part)| {
            // The fill's ends are round, as the track's are.
            let mut corners = iced::border::Radius::default();
            if zone == 0 {
                corners = corners.left(shape::EXTRA_SMALL);
            }
            if Some(zone) == last {
                corners = corners.right(shape::EXTRA_SMALL);
            }
            container(iced::widget::space())
                .width(WIDTH * part)
                .height(BAR)
                .style(move |theme: &Theme| container::Style {
                    background: Some(zone_colors(Scheme::of(theme).dark)[zone].into()),
                    border: iced::Border {
                        radius: corners,
                        ..iced::Border::default()
                    },
                    ..container::Style::default()
                })
                .into()
        });
    // The track shows through, on a picked row's color as on the list's.
    container(iced::widget::Row::with_children(segments))
        .width(WIDTH)
        .height(BAR)
        .style(|theme: &Theme| container::Style {
            background: Some(
                Scheme::of(theme)
                    .on_surface_variant
                    .scale_alpha(0.24)
                    .into(),
            ),
            border: iced::border::rounded(shape::EXTRA_SMALL),
            ..container::Style::default()
        })
}

/// A bar's green, yellow and red: Material's, lighter on a dark theme.
fn zone_colors(dark: bool) -> [Color; 3] {
    if dark {
        [
            Color::from_rgb8(0x66, 0xbb, 0x6a),
            Color::from_rgb8(0xff, 0xee, 0x58),
            Color::from_rgb8(0xef, 0x53, 0x50),
        ]
    } else {
        [
            Color::from_rgb8(0x43, 0xa0, 0x47),
            Color::from_rgb8(0xfb, 0xc0, 0x2d),
            Color::from_rgb8(0xe5, 0x39, 0x35),
        ]
    }
}

#[cfg(test)]
mod tests {
    use triib_endpointd::meters::StreamMeter;

    use super::*;

    const ENTITY: EntityId = EntityId(0xa010_a3ff_00a5_1726);

    fn reading(peaks: [f32; 2]) -> Option<Meters> {
        Some(Meters {
            sequence: 1,
            streams: vec![StreamMeter {
                entity_id: ENTITY.to_string(),
                input: false,
                index: 0,
                peaks,
            }],
        })
    }

    /// Whether `levels` are `wanted`, to a hundredth of a decibel.
    fn near(levels: Option<[f32; 2]>, wanted: [f32; 2]) -> bool {
        levels.is_some_and(|levels| {
            levels
                .iter()
                .zip(wanted)
                .all(|(level, wanted)| (level - wanted).abs() < 0.01)
        })
    }

    /// A meter rises at once, falls at 24 dB a second, and empties when
    /// its stream stops.
    #[test]
    fn a_meter_rises_at_once_and_falls_slowly() {
        let mut levels = Levels::default();
        let start = Instant::now();
        levels.take(Origin::Daemon, reading([1.0, 0.1]), start);
        assert!(near(levels.get((ENTITY, false, 0)), [0.0, -20.0]));
        // Silence a quarter second later: 6 dB down.
        levels.take(
            Origin::Daemon,
            reading([0.0, 0.0]),
            start + Duration::from_millis(250),
        );
        assert!(near(levels.get((ENTITY, false, 0)), [-6.0, -26.0]));
        // A peak louder than where it fell to shows at once.
        levels.take(
            Origin::Daemon,
            reading([0.5, 0.0]),
            start + Duration::from_millis(300),
        );
        assert!(near(levels.get((ENTITY, false, 0)), [-6.02, -27.2]));
        // Stopped.
        levels.take(Origin::Daemon, None, start + Duration::from_millis(350));
        assert_eq!(levels.get((ENTITY, false, 0)), None);
    }

    const MAC_MINI: EntityId = EntityId(0xd111_e597_f544_8000);
    const WIRED_ESP: EntityId = EntityId(0xe8f6_0ae0_9220_0000);

    /// The bench, the wired ESP's first input connected to the Mac mini's
    /// first output, with `host` as one of this computer's entities.
    fn bench_with_host(host: EntityId) -> Triib {
        use triib_endpointd::status::{DaemonStatus, EndpointStatus};
        let (entities, mut models) = crate::view::tests::bench();
        let bound = crate::view::tests::reread(
            &entities,
            &models,
            WIRED_ESP,
            |_| {},
            &[],
            &[(0, MAC_MINI, 0)],
            false,
        );
        models.insert(WIRED_ESP, bound);
        let interface = avb_net::Interface {
            name: "enp6s0".to_owned(),
            mac: avb_net::MacAddress([0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b]),
            up: true,
            speed: None,
            physical: true,
            wireless: false,
            hardware_clock: None,
        };
        let settings = crate::settings::Settings::default();
        let mut triib = Triib::sample(settings, interface, entities, models);
        triib.endpoints = Some(DaemonStatus {
            pid: 1,
            started: 0,
            interface: "enp6s0".into(),
            gptp: String::new(),
            foreign_mrp: false,
            config: None,
            endpoints: vec![EndpointStatus {
                entity_id: host.to_string(),
                name: "Host entity 1".into(),
                state: "streaming".into(),
                streams: Vec::new(),
            }],
        });
        triib
    }

    /// A stream shows a meter where this computer carries its audio: one
    /// of its own entities' streams, an input connected to one of their
    /// outputs, an output one of their inputs is connected to.
    #[test]
    fn streams_connected_to_this_computers_take_their_meters() {
        let (listener, input, talker, output) = (WIRED_ESP, 0, MAC_MINI, 0);
        let triib = bench_with_host(ENTITY);
        // Neither end is this computer's.
        assert_eq!(source(&triib, listener, true, input), None);
        assert_eq!(source(&triib, talker, false, output), None);
        // The talker is.
        let triib = bench_with_host(talker);
        assert_eq!(
            source(&triib, talker, false, output),
            Some((talker, false, output))
        );
        assert_eq!(
            source(&triib, listener, true, input),
            Some((talker, false, output))
        );
        // The listener is.
        let triib = bench_with_host(listener);
        assert_eq!(
            source(&triib, talker, false, output),
            Some((listener, true, input))
        );
    }

    /// Levels last read long ago are not where a new reading starts from.
    #[test]
    fn a_stale_meter_starts_again() {
        let mut levels = Levels::default();
        let start = Instant::now();
        levels.take(Origin::Daemon, reading([1.0, 1.0]), start);
        levels.take(
            Origin::Daemon,
            reading([0.0, 0.0]),
            start + Duration::from_secs(5),
        );
        assert!(near(levels.get((ENTITY, false, 0)), [FLOOR, FLOOR]));
    }
}
