//! The endpoints to run, from `endpoints.toml` in triib's data folder:
//! the interface, then each talker or listener with its channels and
//! where its audio comes from or goes.

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Talker,
    Listener,
    /// This computer as one device with any number of stream outputs and
    /// inputs, as hardware presents itself, each set up on its own.
    Device,
}

/// Which way a device's stream goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Out,
    In,
}

/// One of a device's streams.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamConfig {
    pub direction: Direction,
    /// Its name, "Out 1", "In 1" and on when empty.
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
    /// For an input, the talker's stream output it is bound to, as
    /// The max transit time a controller set for an output, in
    /// nanoseconds; its class's default when left out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit: Option<u32>,
    /// `bound` keeps it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound: Option<String>,
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
            name: String::new(),
            channels: 8,
            audio: None,
            first_channel: 0,
            format: None,
            transit: None,
            bound: None,
            class: SrClass::A,
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

    /// Whether its audio is its own virtual device.
    pub fn is_virtual(&self) -> bool {
        self.audio.as_deref().is_none_or(|audio| audio == VIRTUAL)
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointConfig {
    pub kind: Kind,
    /// Kept apart from the others' to keep its entity ID stable, from 0 to
    /// 239; its place in the list when left out.
    #[serde(default)]
    pub instance: Option<u8>,
    pub name: String,
    #[serde(default = "eight")]
    pub channels: u16,
    /// A talker's audio: "silence", "tone", "default" for the default
    /// input, or an input's name.
    #[serde(default)]
    pub source: Option<String>,
    /// A listener's audio: "discard", "default" for the default output,
    /// or an output's name.
    #[serde(default)]
    pub sink: Option<String>,
    /// The device channel the stream's first channel goes to or comes from.
    #[serde(default)]
    pub first_channel: u16,
    /// The stream format a controller last chose, in hex; the first one
    /// offered when left out.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// The max transit time a controller set for a talker's stream, in
    /// nanoseconds; its class's default when left out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit: Option<u32>,
    /// The talker's stream output a listener is bound to, as
    /// `<entity ID>:<output>`, bound again when it starts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound: Option<String>,
    /// A talker's SR class, `"A"` when left out.
    #[serde(default, skip_serializing_if = "SrClass::is_a")]
    pub class: SrClass,
    /// A device's streams, in order; one output and one input, each with
    /// its own virtual audio device, when there are none.
    #[serde(rename = "stream", default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<StreamConfig>,
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
    /// Its streams: a talker's one output and a listener's one input from
    /// their own settings, a device's as listed, no more than the most
    /// each way.
    pub fn stream_configs(&self) -> Vec<StreamConfig> {
        let streams = match self.kind {
            Kind::Talker => vec![StreamConfig {
                name: self.name.clone(),
                channels: self.channels,
                audio: Some(self.source.clone().unwrap_or_else(|| "silence".into())),
                first_channel: self.first_channel,
                format: self.format.clone(),
                transit: self.transit,
                class: self.class,
                ..StreamConfig::new(Direction::Out)
            }],
            Kind::Listener => vec![StreamConfig {
                name: self.name.clone(),
                channels: self.channels,
                audio: Some(self.sink.clone().unwrap_or_else(|| "discard".into())),
                first_channel: self.first_channel,
                format: self.format.clone(),
                bound: self.bound.clone(),
                ..StreamConfig::new(Direction::In)
            }],
            Kind::Device if self.streams.is_empty() => vec![
                StreamConfig::new(Direction::Out),
                StreamConfig::new(Direction::In),
            ],
            Kind::Device => self.streams.clone(),
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

    /// The talker and stream output a talker's or listener's input is
    /// bound to.
    pub fn bound_to(&self) -> Option<(EntityId, u16)> {
        parse_binding(self.bound.as_deref()?)
    }

    /// The stream format a controller last chose for a talker's or
    /// listener's stream.
    pub fn stream_format(&self) -> Option<StreamFormat> {
        parse_format(self.format.as_deref()?)
    }

    /// Its instance, as its entity ID takes it, when it is `place` in the
    /// list.
    pub fn instance_at(&self, place: usize) -> u8 {
        self.instance.unwrap_or(place as u8).min(239)
    }

    pub fn source(&self) -> Source {
        source_of(self.source.as_deref(), self.first_channel)
    }

    pub fn sink(&self) -> Sink {
        sink_of(self.sink.as_deref(), self.first_channel)
    }
}

/// A source as endpoints.toml names it.
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

/// A sink as endpoints.toml names it.
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

/// Where the endpoints are listed: `endpoints.toml` in the data folder.
pub fn path() -> Option<PathBuf> {
    triib_store::paths::data_dir().map(|dir| dir.join("endpoints.toml"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(text: &str) -> EndpointConfig {
        let config: Config = toml::from_str(text).expect("parses");
        config.endpoints.into_iter().next().expect("an endpoint")
    }

    /// Today's talkers and listeners read as one stream each, from their
    /// own settings; a device without streams has one each way, virtual.
    #[test]
    fn talkers_and_listeners_are_one_stream() {
        let talker = entry("[[endpoint]]\nkind = \"talker\"\nname = \"T\"\nsource = \"tone\"\n");
        let streams = talker.stream_configs();
        assert_eq!(streams.len(), 1);
        assert_eq!(
            (streams[0].direction, streams[0].name.as_str()),
            (Direction::Out, "T")
        );
        assert!(!streams[0].is_virtual());
        let device = entry("[[endpoint]]\nkind = \"device\"\nname = \"D\"\n");
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

    /// A device's streams each keep their own settings.
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
}
