//! The entities to run, from `entities.toml` in triib's data folder, or
//! the file the app names:
//! the interface, then each of this computer's entities with its streams,
//! their channels and where their audio comes from or goes.

use std::path::PathBuf;

use atdecc::EntityId;
use atdecc::stream_format::StreamFormat;
use serde::{Deserialize, Serialize};
use triib_stream::audio::{Sink, Source};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    /// The interface the entities are on: wired, with a PTP hardware clock
    /// that ptp4l keeps on gPTP time.
    pub interface: String,
    /// ptp4l's read-only management socket.
    pub ptp4l_socket: String,
    /// Whether the endpoints may fall back to AVB Lite.
    pub avb_lite: LiteChoice,
    /// Whether, in AVB Lite, a talker with more listeners than it serves
    /// unicast escalates the stream to multicast rather than refusing
    /// them: only where the media VLAN is confined (AVB Lite profile, 6).
    /// Controllers set it with SET_LITE_CONFIG.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub multicast_escalation: bool,
    /// How many listeners a talker serves a stream to unicast in AVB Lite,
    /// 2 when not given, as the profile recommends; at least 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicast_fanout: Option<u8>,
    /// The VLAN AVB Lite's streams and CVU SRP go in, 2 when not given,
    /// as in AVB; 0 tags them with their priority only, for switches
    /// whose VLANs cannot be set up (AVB Lite profile, 7).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_vlan: Option<u16>,
    #[serde(rename = "endpoint")]
    pub endpoints: Vec<EndpointConfig>,
}

/// When the endpoints run AVB Lite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LiteChoice {
    /// When nothing on the interface answers as an AVB bridge would
    /// (AVB Lite profile, 2.2).
    #[default]
    Auto,
    /// Never.
    Off,
    /// Always, as an operator may set it.
    On,
}

/// The SR class a talker sends its stream in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SrClass {
    /// 8000 frames a second's reservation on priority 3, 2 ms.
    #[default]
    A,
    /// 4000 a second's reservation on priority 2, 12 ms, as a stream to a
    /// Wi-Fi station must be (AVB Wireless profile, 4).
    B,
}

impl SrClass {
    fn is_a(&self) -> bool {
        *self == SrClass::A
    }
}

/// Which way an entity's stream goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Out,
    In,
}

/// One of an entity's streams.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamConfig {
    pub direction: Direction,
    /// A media clock stream, CRF, rather than audio: it has no channels
    /// and no audio.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub clock: bool,
    /// For an input, the entity's clock source: its talkers follow the
    /// clock the input's stream carries.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub clock_source: bool,
    /// Its name, "Out 1", "In 1", "Media Clock Out 1" and on when empty.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default = "eight")]
    pub channels: u16,
    /// Where its audio comes from or goes: "virtual", its own virtual
    /// audio device, when left out; "silence" or "tone" for an output,
    /// "discard" for an input; "default"; or a device's name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// The device channel its first channel comes from or goes to, where
    /// its audio is a device.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub first_channel: u16,
    /// The stream format a controller last chose, in hex.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The max transit time a controller set for an output, in
    /// nanoseconds; its class's default when left out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit: Option<u32>,
    /// For an input, the talker's stream output it is bound to, as
    /// `bound` keeps it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound: Option<String>,
    /// For an input, the stream channel each of its audio's channels
    /// plays, as controllers mapped them: a number from 1 for each audio
    /// channel in turn, or - for silence, such as "1 2 2 -"; one to one
    /// when left out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<String>,
    /// An output's SR class, `"A"` when left out.
    #[serde(default, skip_serializing_if = "SrClass::is_a")]
    pub class: SrClass,
}

fn is_zero(value: &u16) -> bool {
    *value == 0
}

impl StreamConfig {
    /// A new stream of `direction`, with its own virtual audio device.
    pub fn new(direction: Direction) -> Self {
        StreamConfig {
            direction,
            clock: false,
            clock_source: false,
            name: String::new(),
            channels: 8,
            audio: None,
            first_channel: 0,
            format: None,
            transit: None,
            bound: None,
            mappings: None,
            class: SrClass::A,
        }
    }

    /// A new media clock stream of `direction`.
    pub fn clock(direction: Direction) -> Self {
        StreamConfig {
            clock: true,
            channels: 0,
            ..StreamConfig::new(direction)
        }
    }

    /// The stream format a controller last chose.
    pub fn stream_format(&self) -> Option<StreamFormat> {
        parse_format(self.format.as_deref()?)
    }

    /// The talker and stream output it is bound to.
    pub fn bound_to(&self) -> Option<(EntityId, u16)> {
        parse_binding(self.bound.as_deref()?)
    }

    /// An input's mappings, as `(stream channel, audio channel)` from 0.
    pub fn mapping_pairs(&self) -> Option<Vec<(u16, u16)>> {
        let text = self.mappings.as_deref()?;
        Some(
            text.split_whitespace()
                .enumerate()
                .filter_map(|(audio, stream)| {
                    let stream: u16 = stream.parse().ok()?;
                    Some((stream.checked_sub(1)?, audio as u16))
                })
                .collect(),
        )
    }
}

/// Mappings as `mappings` keeps them, for `channels` audio channels.
pub fn mappings_text(pairs: impl IntoIterator<Item = (u16, u16)>, channels: u16) -> String {
    let mut played = vec![None; usize::from(channels)];
    for (stream, audio) in pairs {
        if let Some(slot) = played.get_mut(usize::from(audio)) {
            *slot = Some(stream);
        }
    }
    played
        .iter()
        .map(|stream| stream.map_or_else(|| "-".to_owned(), |stream| (stream + 1).to_string()))
        .collect::<Vec<_>>()
        .join(" ")
}

impl StreamConfig {
    /// Whether its audio is its own virtual device; never a media
    /// clock's.
    pub fn is_virtual(&self) -> bool {
        !self.clock && self.audio.as_deref().is_none_or(|audio| audio == VIRTUAL)
    }

    /// Where an output's audio comes from, but for a virtual device.
    pub fn source(&self) -> Source {
        source_of(self.audio.as_deref(), self.first_channel)
    }

    /// Where an input's audio goes, but for a virtual device.
    pub fn sink(&self) -> Sink {
        sink_of(self.audio.as_deref(), self.first_channel)
    }
}

/// The audio of a stream's own virtual device.
pub const VIRTUAL: &str = "virtual";

/// This computer as an entity, with any number of stream outputs and
/// inputs, as hardware presents itself, each set up on its own.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "EndpointEntry")]
pub struct EndpointConfig {
    /// Kept apart from the others' to keep its entity ID stable, from 0 to
    /// 239; its place in the list when left out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<u8>,
    pub name: String,
    /// Its streams, in order; one output and one input, each with its own
    /// virtual audio device, when there are none.
    #[serde(rename = "stream", skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<StreamConfig>,
}

/// An endpoint as files have it: an entity of any streams, or triib
/// 1.0's talker or listener of one stream, whose stream's settings are
/// the entry's own.
#[derive(Deserialize)]
struct EndpointEntry {
    #[serde(default)]
    kind: EntryKind,
    #[serde(default)]
    instance: Option<u8>,
    name: String,
    #[serde(default = "eight")]
    channels: u16,
    #[serde(default)]
    source: Option<String>,
    #[serde(default)]
    sink: Option<String>,
    #[serde(default)]
    first_channel: u16,
    #[serde(default)]
    format: Option<String>,
    #[serde(default)]
    transit: Option<u32>,
    #[serde(default)]
    bound: Option<String>,
    #[serde(default)]
    class: SrClass,
    #[serde(rename = "stream", default)]
    streams: Vec<StreamConfig>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
enum EntryKind {
    Talker,
    Listener,
    #[default]
    Device,
}

impl From<EndpointEntry> for EndpointConfig {
    /// A talker or listener is an entity of its one stream, named as the
    /// entity is, with the audio, format, binding and class it had.
    fn from(entry: EndpointEntry) -> Self {
        let one = |direction, audio: Option<String>, otherwise: &str| StreamConfig {
            name: entry.name.clone(),
            channels: entry.channels,
            audio: Some(audio.unwrap_or_else(|| otherwise.into())),
            first_channel: entry.first_channel,
            format: entry.format.clone(),
            ..StreamConfig::new(direction)
        };
        let streams = match entry.kind {
            EntryKind::Talker => vec![StreamConfig {
                transit: entry.transit,
                class: entry.class,
                ..one(Direction::Out, entry.source.clone(), "silence")
            }],
            EntryKind::Listener => vec![StreamConfig {
                bound: entry.bound.clone(),
                ..one(Direction::In, entry.sink.clone(), "discard")
            }],
            EntryKind::Device => entry.streams,
        };
        EndpointConfig {
            instance: entry.instance,
            name: entry.name,
            streams,
        }
    }
}

/// A binding as `bound` keeps it.
pub fn binding_text(talker: EntityId, output: u16) -> String {
    format!("{talker}:{output}")
}

fn eight() -> u16 {
    8
}

/// The most streams each way: the outputs' stream IDs take an octet.
pub const MOST_STREAMS: usize = 64;

fn parse_binding(text: &str) -> Option<(EntityId, u16)> {
    let (talker, output) = text.rsplit_once(':')?;
    Some((talker.parse().ok()?, output.parse().ok()?))
}

fn parse_format(text: &str) -> Option<StreamFormat> {
    u64::from_str_radix(text.trim_start_matches("0x"), 16)
        .ok()
        .map(StreamFormat)
}

impl EndpointConfig {
    /// An entity named `name` with a stream each way, each with its own
    /// virtual audio device.
    pub fn new(name: impl Into<String>, instance: Option<u8>) -> Self {
        EndpointConfig {
            instance,
            name: name.into(),
            streams: vec![
                StreamConfig::new(Direction::Out),
                StreamConfig::new(Direction::In),
            ],
        }
    }

    /// Its streams as listed, a stream each way when none are, no more
    /// than the most each way.
    pub fn stream_configs(&self) -> Vec<StreamConfig> {
        let streams = match self.streams.is_empty() {
            true => EndpointConfig::new("", None).streams,
            false => self.streams.clone(),
        };
        let (mut outputs, mut inputs) = (0, 0);
        streams
            .into_iter()
            .filter(|stream| {
                let count = match stream.direction {
                    Direction::Out => &mut outputs,
                    Direction::In => &mut inputs,
                };
                *count += 1;
                *count <= MOST_STREAMS
            })
            .collect()
    }

    /// Its instance, as its entity ID takes it, when it is `place` in the
    /// list.
    pub fn instance_at(&self, place: usize) -> u8 {
        self.instance.unwrap_or(place as u8).min(239)
    }
}

/// A source as entities.toml names it.
fn source_of(audio: Option<&str>, first_channel: u16) -> Source {
    match audio {
        None | Some("silence" | VIRTUAL) => Source::Silence,
        Some("tone") => Source::Tone {
            hertz: 1000.0,
            level: 0.1,
        },
        Some("default") => Source::Device {
            name: None,
            first_channel,
        },
        Some(name) => Source::Device {
            name: Some(name.to_owned()),
            first_channel,
        },
    }
}

/// A sink as entities.toml names it.
fn sink_of(audio: Option<&str>, first_channel: u16) -> Sink {
    match audio {
        None | Some("discard" | VIRTUAL) => Sink::Discard,
        Some("default") => Sink::Device {
            name: None,
            first_channel,
        },
        Some(name) => Sink::Device {
            name: Some(name.to_owned()),
            first_channel,
        },
    }
}

/// Where the entities are listed unless the app names another file:
/// `entities.toml` in the data folder.
pub fn path() -> Option<PathBuf> {
    triib_store::paths::data_dir().map(|dir| dir.join("entities.toml"))
}

/// Moves triib 1.0's `endpoints.toml` in the data folder to
/// `entities.toml`, where that is not there yet; the daemon and the app
/// do as they start.
pub fn move_endpoints_file() {
    if let Some(dir) = triib_store::paths::data_dir() {
        entities_in(&dir);
    }
}

/// `entities.toml` in `dir`, `endpoints.toml` moved to it when that is
/// there alone.
fn entities_in(dir: &std::path::Path) -> PathBuf {
    let path = dir.join("entities.toml");
    let before = dir.join("endpoints.toml");
    if !path.exists() && before.exists() {
        let _ = std::fs::rename(&before, &path);
    }
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(text: &str) -> EndpointConfig {
        let config: Config = toml::from_str(text).expect("parses");
        config.endpoints.into_iter().next().expect("an endpoint")
    }

    /// triib 1.0's talkers and listeners read as entities of their one
    /// stream, named as they are, with their audio, format, binding,
    /// transit time and class; written again as such. An entity without
    /// streams has one each way, virtual.
    #[test]
    fn talkers_and_listeners_read_as_devices() {
        let talker = entry(
            "[[endpoint]]\nkind = \"talker\"\ninstance = 3\nname = \"T\"\nsource = \"tone\"\n\
             channels = 2\nformat = \"0x0205022002006000\"\ntransit = 5000000\nclass = \"B\"\n",
        );
        assert_eq!((talker.instance, talker.name.as_str()), (Some(3), "T"));
        let [stream] = talker.streams.as_slice() else {
            panic!("{talker:?}");
        };
        assert_eq!(
            (stream.direction, stream.name.as_str(), stream.channels),
            (Direction::Out, "T", 2)
        );
        assert_eq!(stream.audio.as_deref(), Some("tone"));
        assert_eq!(
            stream.stream_format(),
            Some(StreamFormat(0x0205_0220_0200_6000))
        );
        assert_eq!(
            (stream.transit, stream.class),
            (Some(5_000_000), SrClass::B)
        );
        let listener = entry(
            "[[endpoint]]\nkind = \"listener\"\nname = \"L\"\nbound = \"0xf0a731ff00f40f14:0\"\n",
        );
        let [stream] = listener.streams.as_slice() else {
            panic!("{listener:?}");
        };
        assert_eq!(stream.direction, Direction::In);
        assert_eq!(stream.sink(), Sink::Discard);
        assert_eq!(
            stream.bound_to(),
            Some((EntityId(0xf0a7_31ff_00f4_0f14), 0))
        );
        let text = toml::to_string(&Config {
            endpoints: vec![talker.clone()],
            ..Config::default()
        })
        .expect("writes");
        assert!(!text.contains("kind"), "{text}");
        assert_eq!(entry(&text), talker);
        let device = entry("[[endpoint]]\nname = \"D\"\n");
        let streams = device.stream_configs();
        assert_eq!(
            streams
                .iter()
                .map(|stream| stream.direction)
                .collect::<Vec<_>>(),
            [Direction::Out, Direction::In]
        );
        assert!(streams.iter().all(StreamConfig::is_virtual));
    }

    /// An entity's streams each keep their own settings.
    #[test]
    fn a_devices_streams_keep_their_own_settings() {
        let device = entry(
            "[[endpoint]]\nkind = \"device\"\nname = \"D\"\n\
             [[endpoint.stream]]\ndirection = \"out\"\nchannels = 2\nname = \"Mix\"\n\
             [[endpoint.stream]]\ndirection = \"in\"\naudio = \"default\"\nfirst_channel = 4\n\
             format = \"0x0205022002006000\"\nbound = \"0xf0a731ff00f40f14:0\"\n",
        );
        let streams = device.stream_configs();
        assert_eq!((streams[0].channels, streams[0].name.as_str()), (2, "Mix"));
        assert!(streams[0].is_virtual());
        assert_eq!(
            streams[1].stream_format(),
            Some(StreamFormat(0x0205_0220_0200_6000))
        );
        assert_eq!(
            streams[1].bound_to(),
            Some((EntityId(0xf0a7_31ff_00f4_0f14), 0))
        );
        assert_eq!(
            streams[1].sink(),
            Sink::Device {
                name: None,
                first_channel: 4
            }
        );
        let text = toml::to_string(&Config {
            endpoints: vec![device.clone()],
            ..Config::default()
        })
        .expect("writes");
        assert_eq!(entry(&text), device);
    }

    /// triib 1.0's endpoints.toml moves to entities.toml, where that is
    /// not there yet; one there already stays as it is.
    #[test]
    fn the_endpoints_file_moves_to_entities() {
        let dir = std::env::temp_dir().join(format!("triib-entities-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("endpoints.toml"), "interface = \"enp2s0\"\n").unwrap();
        let path = entities_in(&dir);
        assert_eq!(path, dir.join("entities.toml"));
        assert!(path.exists() && !dir.join("endpoints.toml").exists());
        std::fs::write(dir.join("endpoints.toml"), "interface = \"enp6s0\"\n").unwrap();
        entities_in(&dir);
        assert!(std::fs::read_to_string(&path).unwrap().contains("enp2s0"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A media clock stream says so, and an audio stream says nothing of
    /// it.
    #[test]
    fn a_media_clock_stream_is_marked() {
        let device = entry(
            "[[endpoint]]\nkind = \"device\"\nname = \"D\"\n\
             [[endpoint.stream]]\ndirection = \"out\"\nclock = true\n\
             [[endpoint.stream]]\ndirection = \"in\"\n",
        );
        let streams = device.stream_configs();
        assert!(streams[0].clock && !streams[0].is_virtual());
        assert!(!streams[1].clock && streams[1].is_virtual());
        let text = toml::to_string(&Config {
            endpoints: vec![device.clone()],
            ..Config::default()
        })
        .expect("writes");
        assert_eq!(text.matches("clock = true").count(), 1, "{text}");
        assert_eq!(entry(&text), device);
    }
}
