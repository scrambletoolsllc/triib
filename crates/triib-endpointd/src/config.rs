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
    /// This computer as one device with stream outputs and inputs, as
    /// hardware presents itself: its outputs send its source's channels
    /// and its inputs play to its sink.
    Device,
}

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
    /// The talker's stream output a listener is bound to, as
    /// `<entity ID>:<output>`, bound again when it starts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound: Option<String>,
    /// A talker's SR class, `"A"` when left out.
    #[serde(default, skip_serializing_if = "SrClass::is_a")]
    pub class: SrClass,
    /// How many stream outputs and inputs it has, each of `channels`:
    /// when left out, one output for a talker, one input for a listener,
    /// and one of each for a device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<u16>,
    /// With more than one stream, the format a controller last chose for
    /// each, outputs then inputs, in hex, empty for the first offered; one
    /// stream keeps its in `format`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub formats: Vec<String>,
    /// With more than one stream, what each input is bound to, as `bound`
    /// keeps it, empty for none; one input keeps its in `bound`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bindings: Vec<String>,
}

/// A binding as `bound` keeps it.
pub fn binding_text(talker: EntityId, output: u16) -> String {
    format!("{talker}:{output}")
}

fn eight() -> u16 {
    8
}

/// The most streams each way: the outputs' stream IDs take an octet.
pub const MOST_STREAMS: u16 = 64;

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
    /// How many stream outputs it has.
    pub fn output_count(&self) -> u16 {
        let default = u16::from(matches!(self.kind, Kind::Talker | Kind::Device));
        match self.kind {
            Kind::Listener => 0,
            _ => self.outputs.unwrap_or(default).min(MOST_STREAMS),
        }
    }

    /// How many stream inputs it has.
    pub fn input_count(&self) -> u16 {
        let default = u16::from(matches!(self.kind, Kind::Listener | Kind::Device));
        match self.kind {
            Kind::Talker => 0,
            _ => self.inputs.unwrap_or(default).min(MOST_STREAMS),
        }
    }

    /// Whether it has more than one stream, so that `formats` and
    /// `bindings` keep its streams' settings.
    pub fn several(&self) -> bool {
        self.output_count() + self.input_count() > 1
    }

    /// The talker and stream output input 0 is bound to.
    pub fn bound_to(&self) -> Option<(EntityId, u16)> {
        self.input_bound_to(0)
    }

    /// The talker and stream output an input is bound to.
    pub fn input_bound_to(&self, input: u16) -> Option<(EntityId, u16)> {
        if self.several() {
            return parse_binding(self.bindings.get(usize::from(input))?);
        }
        if input > 0 {
            return None;
        }
        parse_binding(self.bound.as_deref()?)
    }

    /// The stream format a controller last chose, for its first stream.
    pub fn stream_format(&self) -> Option<StreamFormat> {
        self.stream_format_at(0)
    }

    /// The stream format a controller last chose for a stream, counting
    /// the outputs then the inputs.
    pub fn stream_format_at(&self, stream: u16) -> Option<StreamFormat> {
        if self.several() {
            return parse_format(self.formats.get(usize::from(stream))?);
        }
        if stream > 0 {
            return None;
        }
        parse_format(self.format.as_deref()?)
    }

    /// Its instance, as its entity ID takes it, when it is `place` in the
    /// list.
    pub fn instance_at(&self, place: usize) -> u8 {
        self.instance.unwrap_or(place as u8).min(239)
    }

    /// Where its first output's audio comes from.
    pub fn source(&self) -> Source {
        self.source_at(0, self.channels)
    }

    /// Where output `output`'s audio comes from: output k takes the
    /// device's channels from k × `channels` on.
    pub fn source_at(&self, output: u16, channels: u16) -> Source {
        let first_channel = self.first_channel + output * channels;
        match self.source.as_deref() {
            None | Some("silence") => Source::Silence,
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

    /// Where its first input's audio goes.
    pub fn sink(&self) -> Sink {
        self.sink_at(0, self.channels)
    }

    /// Where input `input`'s audio goes: input k plays on the device's
    /// channels from k × `channels` on.
    pub fn sink_at(&self, input: u16, channels: u16) -> Sink {
        let first_channel = self.first_channel + input * channels;
        match self.sink.as_deref() {
            None | Some("discard") => Sink::Discard,
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

    /// Today's files read as before: a talker has one output, a listener
    /// one input, and a device one of each unless told more.
    #[test]
    fn counts_default_by_kind() {
        let talker = entry("[[endpoint]]\nkind = \"talker\"\nname = \"T\"\n");
        assert_eq!((talker.output_count(), talker.input_count()), (1, 0));
        let listener = entry("[[endpoint]]\nkind = \"listener\"\nname = \"L\"\ninputs = 3\n");
        assert_eq!((listener.output_count(), listener.input_count()), (0, 3));
        let device = entry("[[endpoint]]\nkind = \"device\"\nname = \"D\"\noutputs = 2\n");
        assert_eq!((device.output_count(), device.input_count()), (2, 1));
    }

    /// With several streams, `formats` and `bindings` keep each stream's,
    /// and stream k's audio is k streams of channels further along.
    #[test]
    fn several_streams_keep_their_own_settings() {
        let device = entry(
            "[[endpoint]]\nkind = \"device\"\nname = \"D\"\nchannels = 2\nfirst_channel = 4\n\
             sink = \"default\"\nformats = [\"\", \"0x0205022002006000\"]\n\
             bindings = [\"0xf0a731ff00f40f14:0\"]\n",
        );
        assert_eq!(device.stream_format_at(0), None);
        assert_eq!(
            device.stream_format_at(1),
            Some(StreamFormat(0x0205_0220_0200_6000))
        );
        assert_eq!(
            device.input_bound_to(0),
            Some((EntityId(0xf0a7_31ff_00f4_0f14), 0))
        );
        assert_eq!(device.input_bound_to(1), None);
        assert_eq!(
            device.sink_at(1, 2),
            Sink::Device {
                name: None,
                first_channel: 6
            }
        );
    }
}
