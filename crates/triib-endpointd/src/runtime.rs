//! The running endpoints: one thread taking frames from the sockets and
//! ptp4l's state, driving each entity and the MSRP and MVRP participants,
//! and starting and stopping the streams as reservations come and go.

use std::collections::{HashMap, VecDeque};
use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::sync::{Arc, Weak};
use std::time::SystemTime;
use std::time::{Duration, Instant as Clock};

use atdecc::acmp::AcmpFlags;
use atdecc::entity::{
    EndpointModel, Entity, EntityEvent, InputBinding, InputReservation, OutputReservation,
    StreamModel, UNADDRESSED,
};
use atdecc::lite::{FallbackReason, LiteFlags, LiteStatus, PtpProfile};
use atdecc::maap::{self, Maap, MaapEvent};
use atdecc::stream_format::{FormatKind, StreamFormat};
use atdecc::{
    ADP_ACMP_MULTICAST, ClockIdentity, EntityId, EntityModelId, Instant, MacAddress, StreamId,
};
use avb_mrp::Registration;
use avb_mrp::msrp::{self, ListenerState, TalkerDeclaration, TalkerFailure};
use avb_mrp::sharing::{self as sharing_protocol, Hello, Mode, Notice, Refusal, Request};
use avb_net::Socket;
use avb_net::stream::FrameSender;
use avb_stream::audio::{Sink, Source};
use avb_stream::formats;
use avb_stream::{
    GroupConfig, GroupListener, GroupStream, GroupTalker, ListenerConfig, ListenerGroup,
    MediaClock, RecoveredClock, TalkerGroup,
};

use triib_pipewire::{DeviceSpec, VirtualAudio, VirtualDevice};

use crate::config::{
    Config, Direction, EndpointConfig, LiteChoice, MOST_STREAMS, SrClass, StreamConfig,
    binding_text,
};
use crate::gptp;
use crate::lite::{self, Declared, Fallback, Ptp4l};
use crate::meters::{self, Meters, StreamMeter};
use crate::port::{Port, VLAN};
use crate::profile::{self, Profile};
use crate::sharing::{self, Server, Sharers};
use crate::status::{ClockStatus, DaemonStatus, EndpointStatus, StreamStatus, status_path};

/// The entity models of devices, under Scramble Tools' MA-S
/// `8C-1F-64-36-C`: this, and a hash of their streams. triib 1.0's
/// talkers and listeners had `0x8c1f_6436_c000_0002` and `_0004`.
const DEVICE_MODELS: u64 = 0x8c1f_6436_c001_0000;
/// The revision of every device's model, in its hash.
const MODEL_REVISION: u8 = 7;
/// The logo controllers show this computer's entities with: triib's app
/// icon, at 128 pixels.
pub const LOGO: &[u8] =
    include_bytes!("../../../data/icons/hicolor/128x128/apps/run.triib.Triib.png");
/// Their vendor's logo: Scramble Tools' square white logo on a black
/// circle, at 128 pixels.
pub const VENDOR_LOGO: &[u8] = include_bytes!("../../../data/icons/scramble-vendor.png");
/// Class A's priority; its VLAN is the port's.
const PRIORITY: u8 = 3;
/// Class B's priority.
const CLASS_B_PRIORITY: u8 = 2;
/// How long after the last MRP from another participant on the port it
/// is still reported.
const FOREIGN_MRP_QUIET: Duration = Duration::from_secs(30);
/// The latency a talker declares for itself, in nanoseconds.
const TALKER_LATENCY: u32 = 125_000;
/// How often the streams' counters are logged.
const REPORT: Duration = Duration::from_secs(30);
/// How long a talker whose stream changed waits with its declaration
/// withdrawn before declaring the new one: long enough for the bridge to
/// let the old go, as a bridge can keep the old frame size when a
/// declaration changes in place, and drop what no longer fits.
const REDECLARE: Duration = Duration::from_secs(2);

/// The fewest stream addresses MAAP claims, so talkers added while the
/// daemon runs take theirs from the same block.
const MAAP_BLOCK: u16 = 32;
/// AVB Lite's media priority (profile 7), the unicast copies a talker
/// sends before it falls back to multicast, how often CVU SRP declarations
/// AVB Lite's priority for streams and CVU SRP, and the broadcast
/// address talkers declare to.
const LITE_PRIORITY: u8 = 5;
const BROADCAST: MacAddress = MacAddress([0xff; 6]);
/// The unicast fan-out the profile recommends.
const FANOUT: u8 = 5;
/// CVU SRP's timing, MRP's (IEEE 802.1Q-2022, Table 10-7) as the AVB Lite
/// profile carries it over: a declaration goes out twice, JoinTime
/// apart; a withdrawn one lasts LeaveTime more; all are sent again every
/// LeaveAllTime and up to half as much again; and one not sent again
/// within 1.5 LeaveAllTime and LeaveTime is gone.
const JOIN_TIME: Duration = Duration::from_millis(200);
const LEAVE_TIME: Duration = Duration::from_millis(800);
const LEAVE_ALL_TIME: Duration = Duration::from_secs(10);
const AGE_OUT: Duration = Duration::from_secs(16);
/// MSRP failure codes a talker refuses a listener with (IEEE 802.1Q-2022,
/// Table 46-15): its link is full, or it serves as many listeners as it
/// can.
const FAILURE_BANDWIDTH: u8 = 1;
const FAILURE_RESOURCES: u8 = 2;
/// An AVB_INTERFACE's counters by their place in GET_COUNTERS (Milan
/// 1.3, 5.4.2.25): the link coming up and going down, and the grandmaster
/// changing.
const LINK_UP: usize = 0;
const LINK_DOWN: usize = 1;
const GPTP_GM_CHANGED: usize = 5;
/// The least time between starting one ptp4l unit and the next, so a
/// profile switch has time to show.
const SWITCH_AGAIN: Duration = Duration::from_secs(30);
/// The least time between runs of the link unit, which renegotiates the
/// link each time.
const LINK_AGAIN: Duration = Duration::from_secs(600);
/// How long the link unit has before the daemon looks again, the link
/// having gone down and up.
const LINK_SETTLE: Duration = Duration::from_secs(15);
/// How many PTP frames in a row from others carrying the driver's time
/// stamp make the daemon restart the interface: 2 s of Sync and Follow_Up.
const STAMPED_RUN: u32 = 16;
/// How long after running the link unit, where AVB Lite held, the link
/// coming up is its doing, not a new link.
const LINK_OWN: Duration = Duration::from_secs(30);
/// How long ptp4l has to answer for the interface before the daemon starts
/// triib's unit for it: a few of the gPTP watch's turns.
const PTP4L_WAIT: Duration = Duration::from_secs(8);
/// The least time between tries at starting triib's ptp4l unit where none
/// answers.
const PTP4L_AGAIN: Duration = Duration::from_secs(120);
/// How far ptp4l's clock may be off the grandmaster's, in nanoseconds,
/// before the daemon holds it out of sync: far past a servo settling, well
/// short of what a stream's presentation times bear.
const OFF_SYNC: u64 = 1_000_000;
/// How long ptp4l may stay out of sync before the daemon restarts triib's
/// unit running it, as ptp4l steps the clock when it starts.
const OFF_RESTART: Duration = Duration::from_secs(10);
/// How often the daemon looks whether an app asks for the meters.
const METERS_LOOK: Duration = Duration::from_millis(500);

/// What the sockets and ptp4l hand the runtime.
enum Input {
    /// From where, whether sent to a group address rather than to this
    /// computer, and the frame.
    Avtp(MacAddress, bool, Vec<u8>),
    Ptp(MacAddress, Vec<u8>),
    /// From where, and the frame.
    Msrp(MacAddress, Vec<u8>),
    Mvrp(MacAddress, Vec<u8>),
    Gptp(Option<gptp::Status>),
    Link(profile::Link),
    /// Another program on this computer came to share the port's MRP
    /// participant, by its connection; asked it something; or went.
    SharerOpened(u64, sharing::Writer),
    SharerAsked(u64, Request),
    SharerClosed(u64),
}

/// Milan's media clock stream format: CRF, an audio sample clock at
/// 48 kHz, a timestamp each 96 samples, one a frame.
const MEDIA_CLOCK: StreamFormat = StreamFormat(0x0410_6001_0000_bb80);

/// Whether two formats pack samples alike, whatever their channels.
fn same_packing(one: StreamFormat, other: StreamFormat) -> bool {
    match (one.kind(), other.kind()) {
        (FormatKind::Aaf(one), FormatKind::Aaf(other)) => {
            (
                one.nsr,
                one.sample_format,
                one.bit_depth,
                one.samples_per_frame,
            ) == (
                other.nsr,
                other.sample_format,
                other.bit_depth,
                other.samples_per_frame,
            )
        }
        (FormatKind::Iec61883_6(one), FormatKind::Iec61883_6(other)) => {
            (one.packing, one.sfc) == (other.packing, other.sfc)
        }
        _ => false,
    }
}

/// An entity ID from the interface's address and the endpoint's instance,
/// apart from the app's (FF-FE) and triib-cli's (FF-FD).
pub fn entity_id(mac: MacAddress, instance: u8) -> EntityId {
    let [a, b, c, d, e, f] = mac.0;
    EntityId(u64::from_be_bytes([a, b, c, 0xff, instance, d, e, f]))
}

/// Whether an entity model is one of this daemon's devices', telling
/// them apart before the status names them: the block's high bits and the
/// bit their hash always carries, which the app's controller lacks.
pub fn is_device_model(model: EntityModelId) -> bool {
    model.0 & 0xffff_ffff_ff01_0000 == DEVICE_MODELS
}

/// The stream a talker's output sends.
fn output_stream(mac: MacAddress, instance: u8, index: u16) -> StreamId {
    StreamId::new(mac, (u16::from(instance) << 8) | index)
}

/// A talker output: its endpoint, and its index there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Out(usize, u16);

/// A listener input: its endpoint, and its index there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct In(usize, u16);

/// What runs for one of an endpoint's stream outputs.
#[derive(Default)]
struct OutputState {
    /// Its stream in the endpoint's group, while it sends.
    talker: Option<GroupTalker>,
    /// The virtual audio device apps play into for it, where it has one.
    device: Option<VirtualDevice>,
    /// Its place in the MAAP block, kept while it runs, so endpoints
    /// coming and going leave its stream's address as it is.
    slot: u16,
    /// When a talker whose stream changed declares it again.
    redeclare_at: Option<Duration>,
    /// When it last withdrew its declaration.
    withdrawn_at: Option<Duration>,
    /// In AVB Lite, where its copies go and whom it refuses.
    lite: LiteTalker,
}

/// A talker here in AVB Lite, ours or a program's: where its copies go,
/// whether its stream went multicast, when its frames move there, and the
/// listeners it refuses, with the failure code each has.
#[derive(Default)]
struct LiteTalker {
    copies: Vec<MacAddress>,
    escalated: bool,
    escalate_at: Option<Duration>,
    refused: HashMap<MacAddress, u8>,
}

impl LiteTalker {
    /// Moves its copies to `ready`, or for them to `multicast` where it
    /// escalates: the declaration first and the frames JoinTime later to
    /// escalate, the frames first and the declaration after to go back
    /// (profile 6). The copies to send now, and whether its declaration
    /// changed.
    fn step(
        &mut self,
        since: Duration,
        escalate: bool,
        ready: &[MacAddress],
        multicast: MacAddress,
    ) -> (Vec<MacAddress>, bool) {
        let mut announce = false;
        let copies = if escalate {
            if !self.escalated {
                self.escalated = true;
                self.escalate_at = Some(since + JOIN_TIME);
                announce = true;
            }
            match self.escalate_at {
                Some(at) if since < at => self.copies.clone(),
                _ => {
                    self.escalate_at = None;
                    if ready.is_empty() {
                        Vec::new()
                    } else {
                        vec![multicast]
                    }
                }
            }
        } else {
            if self.escalated {
                self.escalated = false;
                self.escalate_at = None;
                announce = true;
            }
            ready.to_vec()
        };
        (copies, announce)
    }
}

/// What runs for one of an endpoint's stream inputs.
#[derive(Default)]
struct InputState {
    /// Its stream in the endpoint's listening group, while it listens.
    listener: Option<GroupListener>,
    /// The virtual audio device apps record it from, where it has one.
    device: Option<VirtualDevice>,
    /// The stream it was last counted among the listeners of.
    counted: Option<u64>,
    /// The stream its listener's thread takes.
    listening_to: Option<atdecc::entity::ProbedStream>,
    /// The clock its stream carries, which the entity's talkers follow
    /// while it is the clock source.
    recovered: RecoveredClock,
    /// The counters its entity was last given.
    counters: [u32; 32],
}

/// One of this computer's entities: a talker, a listener, or a device
/// with any number of stream outputs and inputs.
struct Endpoint {
    config: EndpointConfig,
    entity: Entity,
    /// Its streams as configured, and where each output's and input's is
    /// among them.
    streams: Vec<StreamConfig>,
    out_streams: Vec<usize>,
    in_streams: Vec<usize>,
    /// The thread every output sends from, and the one every input takes
    /// its stream on, while one does: their streams hold them.
    group: Weak<TalkerGroup>,
    listening: Weak<ListenerGroup>,
    outputs: Vec<OutputState>,
    inputs: Vec<InputState>,
    /// Its place in the configuration's list.
    place: usize,
    /// Whether its clock domain is locked to its clock source, and how
    /// often it locked and unlocked, as its counters say.
    clock_locked: bool,
    clock_locks: u32,
    clock_unlocks: u32,
}

/// A device's entity model, one for its streams' every static detail, as
/// controllers keep a model's descriptors by its ID: FNV-1a of each
/// stream's way, channels and formats, in the low 24 bits.
fn model_id(outputs: &[StreamModel], inputs: &[StreamModel]) -> EntityModelId {
    let mut hash: u32 = 0x811c_9dc5;
    let mut take = |bytes: &[u8]| {
        for byte in bytes {
            hash = (hash ^ u32::from(*byte)).wrapping_mul(0x0100_0193);
        }
    };
    // The model's revision, for what the streams leave the same: 1 since
    // the inputs stopped being clock sources, 2 since the model name is
    // "triib entity" and media clock inputs are clock sources, 3 since
    // every input is, 4 since inputs' mappings are dynamic, 5 since
    // streams' classes follow their formats, 6 since it has a logo, 7
    // since its vendor has one too. A class B output's formats come in
    // another order.
    take(&[MODEL_REVISION]);
    for (way, streams) in [(0u8, outputs), (1, inputs)] {
        for stream in streams {
            take(&[way]);
            take(&stream.channels.to_be_bytes());
            for format in &stream.formats {
                take(&format.0.to_be_bytes());
            }
        }
    }
    EntityModelId(DEVICE_MODELS | u64::from(hash & 0x00ff_ffff))
}

impl Endpoint {
    /// Starts advertising, and binds its inputs again to what they were
    /// bound to.
    fn begin(&mut self, now: Instant) {
        // The max transit times and mappings controllers set before.
        for (at, place) in self.out_streams.iter().enumerate() {
            if let Some(nanoseconds) = self.streams[*place].transit {
                self.entity.restore_max_transit_time(at as u16, nanoseconds);
            }
        }
        for (at, place) in self.in_streams.iter().enumerate() {
            if let Some(mappings) = self.streams[*place].mapping_pairs() {
                self.entity.restore_input_mappings(at as u16, &mappings);
            }
        }
        self.entity.start(now);
        // Inputs bind again to what they were bound to, but those carried
        // over bound as they were.
        for input in 0..self.inputs.len() as u16 {
            if self.entity.input_binding(input).is_some() {
                continue;
            }
            if let Some((talker, output)) = self.in_config(input).bound_to() {
                self.entity.restore_binding(
                    now,
                    input,
                    InputBinding {
                        talker,
                        talker_unique_id: output,
                        controller: EntityId(0),
                        flags: AcmpFlags::empty(),
                    },
                );
            }
        }
        log(format!(
            "entity {} as {}, {} out and {} in",
            self.config.name,
            self.entity.entity_id(),
            self.outputs.len(),
            self.inputs.len()
        ));
    }

    fn new(
        config: EndpointConfig,
        instance: u8,
        mac: MacAddress,
        interface: &str,
        clock: ClockIdentity,
    ) -> Self {
        let streams = config.stream_configs();
        let place_of = |direction| {
            streams
                .iter()
                .enumerate()
                .filter(|(_, stream)| stream.direction == direction)
                .map(|(place, _)| place)
                .collect::<Vec<usize>>()
        };
        let (out_streams, in_streams) = (place_of(Direction::Out), place_of(Direction::In));
        // Stream k of a direction: its name, and the format chosen last,
        // with the channels it has now; a media clock's one format.
        let model = |direction: Direction, place: usize| {
            let stream = &streams[place];
            let output = direction == Direction::Out;
            // Numbered among the streams of its way and kind.
            let number = streams[..place]
                .iter()
                .filter(|other| other.direction == direction && other.clock == stream.clock)
                .count()
                + 1;
            let name = if !stream.name.is_empty() {
                stream.name.clone()
            } else {
                let way = if output { "Out" } else { "In" };
                if stream.clock {
                    format!("Media Clock {way} {number}")
                } else {
                    format!("{way} {number}")
                }
            };
            if stream.clock {
                return StreamModel {
                    name,
                    formats: vec![MEDIA_CLOCK],
                    current_format: MEDIA_CLOCK,
                    channels: 0,
                };
            }
            // As many as six samples of each fit in one Ethernet frame,
            // at 48 kHz; the higher rates take the profiles' smaller
            // sizes. A class B output starts in class B's own formats,
            // as the format decides the class.
            let channels = stream.channels.clamp(1, 60);
            let class_b = output && stream.class == SrClass::B;
            let formats = formats::offered(channels, class_b);
            // The format chosen last, or, where the stream's channels
            // changed since, its packing at the channels it has now.
            let current_format = stream
                .stream_format()
                .and_then(|wanted| {
                    formats.contains(&wanted).then_some(wanted).or_else(|| {
                        formats
                            .iter()
                            .copied()
                            .find(|offered| same_packing(*offered, wanted))
                    })
                })
                .unwrap_or(formats[0]);
            StreamModel {
                name,
                current_format,
                formats,
                channels,
            }
        };
        let outputs: Vec<StreamModel> = out_streams
            .iter()
            .map(|place| model(Direction::Out, *place))
            .collect();
        let inputs: Vec<StreamModel> = in_streams
            .iter()
            .map(|place| model(Direction::In, *place))
            .collect();
        // The audio streams share the audio unit's rate: the first one's.
        let first = outputs
            .iter()
            .chain(&inputs)
            .find(|stream| !stream.current_format.is_clock());
        let mut sampling_rates: Vec<u32> = first
            .map(|stream| {
                stream
                    .formats
                    .iter()
                    .filter_map(|format| format.sample_rate())
                    .collect()
            })
            .unwrap_or_default();
        // Each rate once, as the class B formats offer them again.
        sampling_rates.sort_unstable();
        sampling_rates.dedup();
        let current_sampling_rate = first
            .and_then(|stream| stream.current_format.sample_rate())
            .unwrap_or(48_000);
        // Its inputs are clock sources after the internal clock, media
        // clock and audio alike, the one kept as the source chosen.
        let clock_source_inputs: Vec<u16> = (0..in_streams.len() as u16).collect();
        let clock_source = clock_source_inputs
            .iter()
            .position(|&at| streams[in_streams[usize::from(at)]].clock_source)
            .map_or(0, |position| position as u16 + 1);
        // Media clocks alone run at the rate they count.
        if sampling_rates.is_empty() {
            sampling_rates.push(current_sampling_rate);
        }
        // Every audio stream at the entity's rate, as one added to an
        // entity at another rate than its formats' first starts.
        let mut outputs = outputs;
        let mut inputs = inputs;
        for stream in outputs.iter_mut().chain(inputs.iter_mut()) {
            let current = stream.current_format;
            if current.is_clock() || current.sample_rate() == Some(current_sampling_rate) {
                continue;
            }
            // Its own channels, or where they take more than a frame at
            // the rate, the most of its packing that fit, as a change of
            // rate moves a stream.
            let at_rate = || {
                stream
                    .formats
                    .iter()
                    .copied()
                    .filter(|format| format.sample_rate() == Some(current_sampling_rate))
            };
            let same = at_rate().find(|format| format.same_but_rate(current));
            let fewer = || {
                at_rate()
                    .filter(|format| format.same_but_rate_and_channels(current))
                    .max_by_key(|format| format.channels())
            };
            if let Some(format) = same.or_else(fewer) {
                stream.current_format = format;
            }
        }
        let model = EndpointModel {
            entity_id: entity_id(mac, instance),
            entity_model_id: model_id(&outputs, &inputs),
            entity_name: config.name.clone(),
            group_name: String::new(),
            vendor_name: "Scramble Tools".into(),
            model_name: "triib entity".into(),
            firmware_version: env!("CARGO_PKG_VERSION").into(),
            serial_number: String::new(),
            mac,
            interface_name: interface.into(),
            clock_identity: clock,
            sampling_rates,
            current_sampling_rate,
            outputs,
            inputs,
            clock_source,
            clock_source_inputs,
            logo: Some(LOGO.to_vec()),
            vendor_logo: Some(VENDOR_LOGO.to_vec()),
        };
        // A talker's streams have no destination until MAAP gives them
        // one.
        let addressed: Vec<(StreamId, MacAddress)> = (0..out_streams.len() as u16)
            .map(|index| (output_stream(mac, instance, index), UNADDRESSED))
            .collect();
        Endpoint {
            config,
            entity: Entity::new(model, &addressed),
            outputs: out_streams.iter().map(|_| OutputState::default()).collect(),
            inputs: in_streams.iter().map(|_| InputState::default()).collect(),
            streams,
            out_streams,
            in_streams,
            group: Weak::new(),
            listening: Weak::new(),
            place: 0,
            clock_locked: true,
            clock_locks: 1,
            clock_unlocks: 0,
        }
    }

    /// The media clock its talkers follow: its clock source's, none for
    /// the internal clock.
    fn followed(&self) -> Option<RecoveredClock> {
        let model = self.entity.model();
        let input = model.clock_source_input(model.clock_source)?;
        Some(self.inputs.get(usize::from(input))?.recovered.clone())
    }

    /// The name its log lines give its clock source.
    fn clock_source_name(&self) -> String {
        let model = self.entity.model();
        match model.clock_source_input(model.clock_source) {
            Some(input) => self.stream_name(true, input),
            None => "the internal clock".into(),
        }
    }

    /// An output's stream as configured.
    fn out_config(&self, index: u16) -> &StreamConfig {
        &self.streams[self.out_streams[usize::from(index)]]
    }

    /// An input's stream as configured.
    fn in_config(&self, index: u16) -> &StreamConfig {
        &self.streams[self.in_streams[usize::from(index)]]
    }

    /// The name its log lines give a stream: the device's and the
    /// stream's, once where they are the same, as a stream of 1.0's
    /// talkers and listeners is named.
    fn stream_name(&self, input: bool, index: u16) -> String {
        let streams = &self.entity.model();
        let stream = if input {
            streams.inputs.get(usize::from(index))
        } else {
            streams.outputs.get(usize::from(index))
        };
        match stream.map(|stream| stream.name.as_str()) {
            Some(name) if name != self.config.name => format!("{} {name}", self.config.name),
            _ => self.config.name.clone(),
        }
    }

    /// The virtual devices its streams use, made anew for the entity's
    /// sampling rate now; none where `audio` cannot make them.
    fn make_devices(&mut self, audio: Option<&VirtualAudio>) {
        for state in &mut self.outputs {
            state.device = None;
        }
        for state in &mut self.inputs {
            state.device = None;
        }
        self.make_missing_devices(audio);
    }

    /// The virtual devices its streams that have none use, as those
    /// added while the others go on with theirs.
    fn make_missing_devices(&mut self, audio: Option<&VirtualAudio>) {
        let Some(audio) = audio else {
            return;
        };
        let model = self.entity.model();
        let rate = model.current_sampling_rate;
        let entity = model.entity_id.0;
        let name = |input: bool, index: usize| {
            let streams = if input { &model.inputs } else { &model.outputs };
            let stream = &streams[index];
            (
                format!(
                    "triib-{entity:016x}-{}-{}",
                    if input { "in" } else { "out" },
                    index + 1
                ),
                format!("{} {}", model.entity_name, stream.name),
                stream.channels,
            )
        };
        let made = |input: bool, index: usize| {
            let (node, description, channels) = name(input, index);
            match audio.add(DeviceSpec {
                name: node,
                description: description.clone(),
                channels,
                rate,
                playback: !input,
            }) {
                Ok(device) => Some(device),
                Err(error) => {
                    log(format!("{description}: no virtual audio device: {error}"));
                    None
                }
            }
        };
        let outputs: Vec<Option<VirtualDevice>> = (0..self.outputs.len())
            .map(|index| {
                (self.outputs[index].device.is_none()
                    && self.streams[self.out_streams[index]].is_virtual())
                .then(|| made(false, index))
                .flatten()
            })
            .collect();
        let inputs: Vec<Option<VirtualDevice>> = (0..self.inputs.len())
            .map(|index| {
                (self.inputs[index].device.is_none()
                    && self.streams[self.in_streams[index]].is_virtual())
                .then(|| made(true, index))
                .flatten()
            })
            .collect();
        for (state, device) in self.outputs.iter_mut().zip(outputs) {
            if device.is_some() {
                state.device = device;
            }
        }
        for (state, device) in self.inputs.iter_mut().zip(inputs) {
            if device.is_some() {
                state.device = device;
            }
        }
    }

    /// Whether its stream output or input `index` runs as `earlier`'s
    /// does: the same audio, channels, class and format at the same rate,
    /// so it can go on as it is when `earlier` hands over to it.
    fn runs_as(&self, earlier: &Endpoint, input: bool, index: u16) -> bool {
        let at = usize::from(index);
        let (count, before) = if input {
            (self.inputs.len(), earlier.inputs.len())
        } else {
            (self.outputs.len(), earlier.outputs.len())
        };
        if at >= count || at >= before {
            return false;
        }
        let (this, that) = if input {
            (self.in_config(index), earlier.in_config(index))
        } else {
            (self.out_config(index), earlier.out_config(index))
        };
        let model = |endpoint: &Endpoint| {
            let model = endpoint.entity.model();
            let streams = if input { &model.inputs } else { &model.outputs };
            streams.get(at).map(|stream| {
                (
                    stream.current_format,
                    stream.channels,
                    model.current_sampling_rate,
                )
            })
        };
        model(self) == model(earlier)
            && (
                this.clock,
                this.channels,
                &this.audio,
                this.first_channel,
                this.class,
            ) == (
                that.clock,
                that.channels,
                &that.audio,
                that.first_channel,
                that.class,
            )
    }
}

pub struct Runtime {
    interface: String,
    mac: MacAddress,
    start: Clock,
    avtp: Arc<Socket>,
    msrp: Arc<Socket>,
    mvrp: Arc<Socket>,
    endpoints: Vec<Endpoint>,
    /// The virtual audio devices of the streams that have them, while any
    /// do.
    audio: Option<VirtualAudio>,
    /// The port's one MSRP and one MVRP participant, for every endpoint.
    port: Port,
    /// When MRP from this computer's address that this daemon did not send
    /// was last heard: another participant on the port.
    foreign_mrp: Option<Clock>,
    /// The claim on the talkers' destination addresses, one each.
    maap: Option<Maap>,
    /// Peer delay messages, which tell AVB from AVB Lite.
    ptp: Arc<Socket>,
    /// Sends whole frames, VLAN tag and all: AVB Lite's declarations.
    tagged: FrameSender,
    fallback: Fallback,
    /// AVB Lite, once the endpoints fell back to it.
    lite: Option<LiteMode>,
    /// Whether the interface has a PTP hardware clock, which AVB Lite
    /// needs: without one its endpoints stay off it, falling back to it
    /// for nothing and running it for no one.
    hardware_clock: bool,
    /// This interface's clock identity, its link speed in Mb/s, and what
    /// ptp4l says of the grandmaster there.
    own_clock: ClockIdentity,
    link_speed: u32,
    ptp_status: Option<gptp::Status>,
    /// The link speed each grandmaster announced, in Mb/s, and the
    /// correction for it the media clock carries, in nanoseconds.
    grandmaster_links: HashMap<u64, u32>,
    correction: i64,
    /// CVU SRP messages this computer sent its own endpoints, whether to a
    /// group address, taken once the update that sent them is done.
    local_cvu: VecDeque<(bool, Vec<u8>)>,
    /// The link and which of triib's ptp4l units runs there, and when the
    /// daemon last started one.
    link: Option<profile::Link>,
    switched_at: Option<Duration>,
    /// A ptp4l unit switch put off until `SWITCH_AGAIN` has passed, or
    /// until a gPTP peer is heard.
    steer_pending: bool,
    /// PTP frames from others in a row that carried the driver's time
    /// stamp, and when the daemon last restarted the interface for them.
    stamped_run: u32,
    link_reset_at: Option<Duration>,
    /// When the daemon last ran the link unit again.
    link_fixed_at: Option<Duration>,
    /// When the daemon last started triib's ptp4l unit, or found it could
    /// not, where no ptp4l answered for the interface.
    ptp4l_tried_at: Option<Duration>,
    /// Since when ptp4l's clock has been out of sync, more than `OFF_SYNC`
    /// off the grandmaster's, and when the daemon last restarted triib's
    /// unit for it.
    off_since: Option<Duration>,
    off_restarted_at: Option<Duration>,
    /// Until when the link coming up is the link unit's doing, where AVB
    /// Lite held as the daemon ran it.
    link_own_until: Option<Duration>,
    clock: Arc<MediaClock>,
    /// The talker declarations registered, by stream ID.
    talkers: HashMap<u64, TalkerDeclaration>,
    /// Our talkers' streams a listener on the network is ready for.
    ready_remotely: HashMap<u64, bool>,
    /// The listener declarations registered, by stream ID.
    listener_states: HashMap<u64, ListenerState>,
    /// The Listener declarations made on the port by MSRP, one for each
    /// stream for all its listeners here, ours and the programs'.
    port_listeners: HashMap<u64, ListenerState>,
    /// Serves the port's MRP participant to other programs here, and the
    /// programs sharing it.
    sharing: Option<Server>,
    sharers: Sharers,
    stop: Arc<AtomicBool>,
    inputs: Receiver<Input>,
    last_report: Clock,
    last_counters: Clock,
    /// The interface's counters, as every endpoint on it reports them for
    /// its AVB_INTERFACE: how often the link came up and went down, and
    /// the grandmaster changed; and the grandmaster ptp4l last named.
    interface_counters: [u32; 32],
    grandmaster: Option<ClockIdentity>,
    /// The configuration running, its file, and when that last changed.
    config: Config,
    path: Option<PathBuf>,
    modified: Option<SystemTime>,
    last_look: Clock,
    /// Whether an app asked for the meters when the daemon last looked,
    /// when that was, when it last wrote them and how many times, and each
    /// running stream's peaks in that write.
    meters_wanted: bool,
    meters_looked: Clock,
    meters_written: Clock,
    meters_sequence: u64,
    meter_peaks: HashMap<(EntityId, bool, u16), [f32; 2]>,
    gptp_text: String,
    /// When it started on this configuration, in milliseconds since the
    /// Unix epoch.
    started: u64,
}

/// AVB Lite's state: CVU SRP declarations in place of MSRP's.
struct LiteMode {
    reason: FallbackReason,
    sequence: u16,
    /// Talker declarations heard, by stream: what, from where, and when
    /// last.
    talkers: HashMap<u64, (TalkerDeclaration, MacAddress, Duration)>,
    /// Listener declarations heard for our streams, by stream and by the
    /// listener's address.
    listeners: HashMap<u64, HashMap<MacAddress, (ListenerState, Duration)>>,
    next_refresh: Duration,
    /// Declarations to send a second time, JoinTime after the first: when,
    /// in which entity's name, where to, and what.
    repeats: Vec<(Duration, EntityId, MacAddress, avb_mrp::mrpdu::Message)>,
    /// The Listener declarations this computer sends, one for each stream
    /// for all its listeners here, ours and the programs': the talker
    /// told, what, and in which entity's name.
    listening: HashMap<u64, (MacAddress, ListenerState, EntityId)>,
    /// The talkers of the programs sharing the port, by stream.
    shared: HashMap<u64, LiteTalker>,
    /// Declarations withdrawn, kept until LeaveTime after their Lv unless
    /// declared again first.
    leaving_talkers: HashMap<u64, Duration>,
    leaving_listeners: HashMap<(u64, MacAddress), Duration>,
    /// Talker declarations sent to this computer alone, refusing one of
    /// its listeners (profile 6, item 10): by stream, what, from where and
    /// when last; and those withdrawn.
    refusals: HashMap<u64, (TalkerDeclaration, MacAddress, Duration)>,
    leaving_refusals: HashMap<u64, Duration>,
    /// Draws the refresh interval.
    random: u64,
}

impl LiteMode {
    /// When to send every declaration again: from LeaveAllTime to half as
    /// much again from `since`.
    fn refresh_after(&mut self, since: Duration) -> Duration {
        self.random ^= self.random << 13;
        self.random ^= self.random >> 7;
        self.random ^= self.random << 17;
        since + LEAVE_ALL_TIME + Duration::from_millis(self.random % 5000)
    }
}

/// Whether two CVU SRP messages are about the same declaration: the same
/// kind, talker or listener, and the same stream.
fn same_declaration(one: &avb_mrp::mrpdu::Message, other: &avb_mrp::mrpdu::Message) -> bool {
    let listener =
        |message: &avb_mrp::mrpdu::Message| message.attribute_type == msrp::attribute::LISTENER;
    let stream = |message: &avb_mrp::mrpdu::Message| {
        message
            .values
            .first()
            .and_then(|(value, _, _)| value.get(..8))
            .map(<[u8]>::to_vec)
    };
    listener(one) == listener(other) && stream(one) == stream(other)
}

/// Why the runtime ended.
pub enum Exit {
    Stop,
    /// The configuration changed; start again with this one.
    Reload(Config),
}

/// The modification time of `path`.
fn modified(path: Option<&PathBuf>) -> Option<SystemTime> {
    std::fs::metadata(path?).ok()?.modified().ok()
}

fn log(message: impl AsRef<str>) {
    println!("{}", message.as_ref());
}

/// Hands every frame `socket` receives to the runtime as `wrap` makes it.
fn read_into(
    socket: Arc<Socket>,
    stop: Arc<AtomicBool>,
    sender: Sender<Input>,
    name: &str,
    wrap: fn(MacAddress, bool, Vec<u8>) -> Input,
) -> io::Result<()> {
    std::thread::Builder::new()
        .name(name.into())
        .spawn(move || {
            let mut buffer = vec![0u8; 1522];
            while !stop.load(Ordering::Relaxed) {
                match socket.receive(&mut buffer, Some(Duration::from_millis(200))) {
                    Ok(Some(received)) => {
                        let bytes = buffer[..received.length].to_vec();
                        if sender
                            .send(wrap(received.source, received.group, bytes))
                            .is_err()
                        {
                            return;
                        }
                    }
                    Ok(None) => {}
                    Err(_) => std::thread::sleep(Duration::from_millis(200)),
                }
            }
        })?;
    Ok(())
}

impl Runtime {
    pub fn new(config: Config, path: Option<PathBuf>, stop: Arc<AtomicBool>) -> io::Result<Self> {
        let interface = config.interface.clone();
        let found = avb_net::interfaces()
            .into_iter()
            .find(|found| found.name == interface)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("no interface named {interface}"),
                )
            })?;
        let clock = Arc::new(MediaClock::open(found.hardware_clock)?);
        let hardware_clock = found.hardware_clock.is_some();
        if !hardware_clock {
            log(format!(
                "{interface} has no PTP hardware clock: presentation times will mean nothing, \
                 and its endpoints stay off AVB Lite, which needs one"
            ));
        }
        let avtp = Arc::new(Socket::open(&interface, atdecc::ETHERTYPE_AVTP)?);
        avtp.join_multicast(ADP_ACMP_MULTICAST)?;
        avtp.join_multicast(maap::DESTINATION)?;
        let _ =
            avtp.keep_payloads_starting(atdecc::avtp::subtype::ADP..=atdecc::avtp::subtype::MAAP);
        let msrp = Arc::new(Socket::open(&interface, avb_mrp::ETHERTYPE_MSRP)?);
        msrp.join_multicast(avb_mrp::MSRP_DESTINATION)?;
        let mvrp = Arc::new(Socket::open(&interface, avb_mrp::ETHERTYPE_MVRP)?);
        mvrp.join_multicast(avb_mrp::MVRP_DESTINATION)?;
        let ptp = Arc::new(Socket::open(&interface, lite::PTP_ETHERTYPE)?);
        ptp.join_multicast(lite::PDELAY_DESTINATION)?;
        ptp.join_multicast(lite::PTP_DESTINATION)?;

        let (sender, inputs) = mpsc::channel();
        read_into(
            avtp.clone(),
            stop.clone(),
            sender.clone(),
            "avtp",
            Input::Avtp,
        )?;
        read_into(
            msrp.clone(),
            stop.clone(),
            sender.clone(),
            "msrp",
            |source, _, bytes| Input::Msrp(source, bytes),
        )?;
        read_into(
            mvrp.clone(),
            stop.clone(),
            sender.clone(),
            "mvrp",
            |source, _, bytes| Input::Mvrp(source, bytes),
        )?;
        read_into(
            ptp.clone(),
            stop.clone(),
            sender.clone(),
            "ptp",
            |source, _, bytes| Input::Ptp(source, bytes),
        )?;
        let socket = if config.ptp4l_socket.is_empty() {
            "/var/run/ptp4lro".to_owned()
        } else {
            config.ptp4l_socket.clone()
        };
        gptp::watch(socket, stop.clone(), sender.clone(), Input::Gptp);
        let sharing = Server::start(
            &interface,
            sender.clone(),
            Input::SharerOpened,
            Input::SharerAsked,
            Input::SharerClosed,
        );
        profile::watch(interface.clone(), stop.clone(), sender, Input::Link);

        let mac = avtp.mac();
        let own_clock = ClockIdentity(u64::from_be_bytes([
            mac.0[0], mac.0[1], mac.0[2], 0xff, 0xfe, mac.0[3], mac.0[4], mac.0[5],
        ]));
        let mut used = Vec::new();
        let endpoints: Vec<Endpoint> = config
            .endpoints
            .iter()
            .enumerate()
            .filter_map(|(place, endpoint)| {
                let instance = endpoint.instance_at(place);
                if used.contains(&instance) {
                    log(format!(
                        "instance {instance} is taken; leaving out {}",
                        endpoint.name
                    ));
                    return None;
                }
                used.push(instance);
                let mut made =
                    Endpoint::new(endpoint.clone(), instance, mac, &interface, own_clock);
                made.place = place;
                Some(made)
            })
            .collect();
        let seed = mac
            .0
            .iter()
            .fold(0u64, |seed, octet| (seed << 8) | u64::from(*octet));
        let talkers: usize = endpoints
            .iter()
            .map(|endpoint| endpoint.outputs.len())
            .sum();
        // PipeWire, for the streams with virtual audio devices.
        let mut endpoints = endpoints;
        let wanted = endpoints
            .iter()
            .any(|endpoint| endpoint.streams.iter().any(StreamConfig::is_virtual));
        let audio = if wanted {
            match VirtualAudio::start() {
                Ok(audio) => Some(audio),
                Err(error) => {
                    log(format!("no virtual audio devices: {error}"));
                    None
                }
            }
        } else {
            None
        };
        for endpoint in &mut endpoints {
            endpoint.make_devices(audio.as_ref());
        }
        for (slot, output) in endpoints
            .iter_mut()
            .flat_map(|endpoint| endpoint.outputs.iter_mut())
            .enumerate()
        {
            output.slot = slot as u16;
        }
        let maap = (talkers > 0).then(|| Maap::new(mac, (talkers as u16).max(MAAP_BLOCK)));
        let fallback = Fallback::new(
            own_clock.0,
            config.avb_lite == LiteChoice::On && hardware_clock,
            Duration::ZERO,
        );
        let link_speed = found.speed.unwrap_or(0);
        Ok(Runtime {
            interface: interface.clone(),
            mac,
            start: Clock::now(),
            avtp,
            msrp,
            mvrp,
            endpoints,
            audio,
            port: Port::new(seed),
            foreign_mrp: None,
            maap,
            tagged: FrameSender::open(&interface)?,
            ptp,
            fallback,
            lite: None,
            hardware_clock,
            own_clock,
            link_speed,
            ptp_status: None,
            local_cvu: VecDeque::new(),
            grandmaster_links: HashMap::new(),
            correction: 0,
            link: None,
            switched_at: None,
            steer_pending: false,
            stamped_run: 0,
            link_reset_at: None,
            link_fixed_at: None,
            ptp4l_tried_at: None,
            off_since: None,
            off_restarted_at: None,
            link_own_until: None,
            clock,
            talkers: HashMap::new(),
            ready_remotely: HashMap::new(),
            listener_states: HashMap::new(),
            port_listeners: HashMap::new(),
            sharing,
            sharers: Sharers::default(),
            stop,
            inputs,
            last_report: Clock::now(),
            last_counters: Clock::now(),
            interface_counters: [0; 32],
            grandmaster: None,
            modified: modified(path.as_ref()),
            path,
            config,
            last_look: Clock::now(),
            meters_wanted: false,
            meters_looked: Clock::now(),
            meters_written: Clock::now(),
            meters_sequence: 0,
            meter_peaks: HashMap::new(),
            gptp_text: "ptp4l not asked yet".into(),
            started: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_or(0, |since| since.as_millis() as u64),
        })
    }

    fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    fn now(&self) -> Instant {
        Instant::from_nanos(self.elapsed().as_nanos() as u64)
    }

    /// Runs until told to stop or the configuration changes, then says
    /// goodbye.
    pub fn run(mut self) -> io::Result<Exit> {
        let now = self.now();
        let since = self.elapsed();
        for endpoint in &mut self.endpoints {
            endpoint.begin(now);
        }
        // Every endpoint takes part in class A on VLAN 2, declared once for
        // the port; talkers declare their streams once MAAP gives them
        // addresses.
        self.port.join(since);
        if let Some(maap) = &mut self.maap {
            maap.start(now);
        }
        while !self.stop.load(Ordering::Relaxed) {
            let wait = self.next_wait();
            match self.inputs.recv_timeout(wait) {
                Ok(input) => self.handle(input),
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => break,
            }
            self.turn();
            self.write_meters();
            if self.last_look.elapsed() >= Duration::from_secs(2) {
                self.last_look = Clock::now();
                self.write_status();
                if let Some(config) = self.changed_config() {
                    if self.apply(&config) {
                        log("entities.toml changed, applied");
                    } else {
                        log("entities.toml changed, starting again");
                        self.shut_down();
                        return Ok(Exit::Reload(config));
                    }
                }
            }
        }
        self.shut_down();
        if let Some(path) = status_path() {
            let _ = std::fs::remove_file(path);
        }
        Ok(Exit::Stop)
    }

    /// Takes a changed configuration in place where only its endpoints
    /// changed: those it no longer has, or has set up otherwise, stop,
    /// and those new to it start, while the rest go on as they are.
    /// `false` when the daemon must start again, as for another interface
    /// or more talkers than the MAAP block holds.
    fn apply(&mut self, config: &Config) -> bool {
        let port = |config: &Config| Config {
            endpoints: Vec::new(),
            ..config.clone()
        };
        if port(config) != port(&self.config) {
            return false;
        }
        let instances: Vec<u8> = config
            .endpoints
            .iter()
            .enumerate()
            .map(|(place, entry)| entry.instance_at(place))
            .collect();
        if (1..instances.len()).any(|at| instances[..at].contains(&instances[at])) {
            return false;
        }
        // An entry as it stands, with its instance written down, so one
        // left implicit compares equal to the same one made explicit.
        let settled = |entry: &EndpointConfig, instance: u8| EndpointConfig {
            instance: Some(instance),
            ..entry.clone()
        };
        let mac = self.mac;
        // Where each endpoint goes on, by its place in the new entries.
        let goes_on: Vec<Option<usize>> = self
            .endpoints
            .iter()
            .map(|endpoint| {
                let id = endpoint.entity.entity_id();
                let held = self.config.endpoints.get(endpoint.place)?;
                let place = instances
                    .iter()
                    .position(|&instance| entity_id(mac, instance) == id)?;
                let instance = instances[place];
                (settled(&config.endpoints[place], instance) == settled(held, instance))
                    .then_some(place)
            })
            .collect();
        let started: Vec<Endpoint> = config
            .endpoints
            .iter()
            .enumerate()
            .filter(|(place, _)| !goes_on.contains(&Some(*place)))
            .map(|(place, entry)| {
                let mut made = Endpoint::new(
                    entry.clone(),
                    instances[place],
                    mac,
                    &self.interface,
                    self.own_clock,
                );
                made.place = place;
                made
            })
            .collect();
        // A device whose entry changed, as when a stream was added, takes
        // over from itself rather than leaving: its outputs keep their
        // places in the MAAP block, in order, so their streams' addresses
        // stay where listeners take them.
        let kept: HashMap<EntityId, Vec<u16>> = self
            .endpoints
            .iter()
            .zip(&goes_on)
            .filter(|(endpoint, place)| {
                let id = endpoint.entity.entity_id();
                place.is_none() && started.iter().any(|made| made.entity.entity_id() == id)
            })
            .map(|(endpoint, _)| {
                let slots = endpoint.outputs.iter().map(|output| output.slot);
                (endpoint.entity.entity_id(), slots.collect())
            })
            .collect();
        // The other new talkers' places, from those free.
        let taken: Vec<u16> = self
            .endpoints
            .iter()
            .zip(&goes_on)
            .filter(|(_, place)| place.is_some())
            .flat_map(|(endpoint, _)| endpoint.outputs.iter().map(|output| output.slot))
            .chain(kept.values().flatten().copied())
            .collect();
        let reused = |endpoint: &Endpoint| {
            kept.get(&endpoint.entity.entity_id())
                .map_or(0, |slots| slots.len().min(endpoint.outputs.len()))
        };
        let wanted: usize = started
            .iter()
            .map(|endpoint| endpoint.outputs.len() - reused(endpoint))
            .sum();
        let block = self
            .maap
            .as_ref()
            .map_or(wanted.max(usize::from(MAAP_BLOCK)), |maap| {
                usize::from(maap.count())
            });
        let free: Vec<u16> = (0..block as u16)
            .filter(|slot| !taken.contains(slot))
            .take(wanted)
            .collect();
        if free.len() < wanted {
            return false;
        }

        let now = self.now();
        let mut started = started;
        for index in (0..self.endpoints.len()).rev() {
            if goes_on[index].is_none() {
                let id = self.endpoints[index].entity.entity_id();
                match started
                    .iter_mut()
                    .find(|made| made.entity.entity_id() == id)
                {
                    Some(made) => self.hand_over(now, index, made),
                    None => self.stop_endpoint(index),
                }
            }
        }
        for (endpoint, place) in self.endpoints.iter_mut().zip(goes_on.into_iter().flatten()) {
            endpoint.place = place;
        }
        if self.audio.is_none()
            && started
                .iter()
                .any(|endpoint| endpoint.streams.iter().any(StreamConfig::is_virtual))
        {
            match VirtualAudio::start() {
                Ok(audio) => self.audio = Some(audio),
                Err(error) => log(format!("no virtual audio devices: {error}")),
            }
        }
        let since = self.elapsed();
        let mut free = free.into_iter();
        // What ptp4l last said, so a new endpoint advertises the network's
        // grandmaster from the start rather than at the next poll.
        let gptp = self
            .ptp_status
            .as_ref()
            .map(|status| status.gptp.clone())
            .unwrap_or_default();
        for mut endpoint in started {
            let id = endpoint.entity.entity_id();
            endpoint.entity.set_gptp(now, gptp.clone());
            endpoint.make_missing_devices(self.audio.as_ref());
            let mut slots = kept.get(&id).cloned().unwrap_or_default().into_iter();
            for (index, output) in endpoint.outputs.iter_mut().enumerate() {
                output.slot = slots.next().or_else(|| free.next()).unwrap_or_default();
                if let Some(address) = self
                    .maap
                    .as_ref()
                    .and_then(|maap| maap.address(output.slot))
                {
                    endpoint
                        .entity
                        .set_output_destination(index as u16, address);
                    output.redeclare_at = Some(since);
                }
            }
            endpoint.begin(now);
            self.endpoints.push(endpoint);
        }
        self.endpoints.sort_by_key(|endpoint| endpoint.place);
        if self.maap.is_none() && wanted > 0 {
            let mut maap = Maap::new(mac, block as u16);
            maap.start(now);
            self.maap = Some(maap);
        }
        self.config = config.clone();
        true
    }

    /// Stops one endpoint while the others go on: its streams'
    /// declarations withdrawn, its threads and virtual audio devices
    /// ended, and its departure said.
    fn stop_endpoint(&mut self, index: usize) {
        for out in self.outs().into_iter().filter(|out| out.0 == index) {
            self.output_mut(out).talker = None;
            self.withdraw_talker(out);
        }
        for input in self.ins().into_iter().filter(|input| input.0 == index) {
            self.stop_listener(input);
        }
        let mut endpoint = self.endpoints.remove(index);
        endpoint.entity.depart();
        let now = self.now();
        while let Some((destination, bytes)) = endpoint.entity.poll_transmit() {
            let _ = self.avtp.send(destination, &bytes);
            // Our other entities hear it too, as the socket never brings
            // our own frames back.
            if destination == ADP_ACMP_MULTICAST || destination == self.mac {
                for other in &mut self.endpoints {
                    other.entity.handle_frame(now, self.mac, &bytes);
                }
            }
        }
        log(format!("{} stopped", endpoint.config.name));
    }

    /// Hands endpoint `index` over to `made`, the same entity whose
    /// streams changed, in place of stopping it: its streams that run as
    /// before go on in `made`, with their threads, virtual devices,
    /// reservations, bindings and probes; the others stop. The entity
    /// takes over without leaving, as `Entity::take_over` has it.
    fn hand_over(&mut self, now: Instant, index: usize, made: &mut Endpoint) {
        let earlier = &self.endpoints[index];
        let kept = |input: bool, count: usize| -> Vec<u16> {
            (0..count as u16)
                .filter(|&at| made.runs_as(earlier, input, at))
                .collect()
        };
        let outputs = kept(false, earlier.outputs.len());
        let inputs = kept(true, earlier.inputs.len());
        for out in self.outs().into_iter().filter(|out| out.0 == index) {
            if !outputs.contains(&out.1) {
                self.output_mut(out).talker = None;
                self.withdraw_talker(out);
            }
        }
        for input in self.ins().into_iter().filter(|input| input.0 == index) {
            if !inputs.contains(&input.1) {
                self.stop_listener(input);
            }
        }
        let mut earlier = self.endpoints.remove(index);
        // The talkers of the inputs not carried on, at the same index, are
        // let go, as a talker outside Milan keeps a connection until told.
        earlier.entity.release_inputs(&inputs);
        while let Some((destination, bytes)) = earlier.entity.poll_transmit() {
            let _ = self.avtp.send(destination, &bytes);
        }
        for &at in &outputs {
            made.outputs[usize::from(at)] = std::mem::take(&mut earlier.outputs[usize::from(at)]);
        }
        for &at in &inputs {
            made.inputs[usize::from(at)] = std::mem::take(&mut earlier.inputs[usize::from(at)]);
        }
        made.group = earlier.group.clone();
        made.listening = earlier.listening.clone();
        made.clock_locked = earlier.clock_locked;
        made.clock_locks = earlier.clock_locks;
        made.clock_unlocks = earlier.clock_unlocks;
        made.entity.take_over(now, &earlier.entity);
        made.entity
            .carry_streams(&earlier.entity, &outputs, &inputs);
        if outputs.len() + inputs.len() > 0 {
            log(format!(
                "{}: {} streams go on as they were",
                made.config.name,
                outputs.len() + inputs.len()
            ));
        }
    }

    /// The configuration file, when it changed into something else.
    fn changed_config(&mut self) -> Option<Config> {
        let now = modified(self.path.as_ref());
        if now == self.modified {
            return None;
        }
        self.modified = now;
        match triib_store::load::<Config>(self.path.as_ref()?) {
            Ok(config) if config != self.config => Some(config),
            Ok(_) => None,
            Err(error) => {
                log(format!("entities.toml: {error}"));
                None
            }
        }
    }

    /// Keeps the name, stream format and binding a controller gave in
    /// the configuration.
    fn keep_settings(&mut self, index: usize) {
        let endpoint = &self.endpoints[index];
        let model = endpoint.entity.model();
        let name = model.entity_name.clone();
        // Each stream's format and binding, in the configured streams'
        // order.
        let mut streams = endpoint.streams.clone();
        for (at, place) in endpoint.out_streams.iter().enumerate() {
            streams[*place].format = model
                .outputs
                .get(at)
                .map(|stream| format!("{:#018x}", stream.current_format.0));
            streams[*place].transit = (!endpoint.entity.max_transit_time_is_default(at as u16))
                .then(|| endpoint.entity.max_transit_time(at as u16))
                .flatten();
        }
        let source = model.clock_source_input(model.clock_source);
        for (at, place) in endpoint.in_streams.iter().enumerate() {
            streams[*place].format = model
                .inputs
                .get(at)
                .map(|stream| format!("{:#018x}", stream.current_format.0));
            streams[*place].clock_source = source == Some(at as u16);
            let channels = model.inputs.get(at).map_or(0, |stream| stream.channels);
            streams[*place].mappings = (!endpoint.entity.input_mappings_are_default(at as u16))
                .then(|| endpoint.entity.input_mappings(at as u16))
                .flatten()
                .map(|mappings| {
                    let pairs = mappings
                        .iter()
                        .map(|mapping| (mapping.stream_channel, mapping.cluster_offset));
                    crate::config::mappings_text(pairs, channels)
                });
            streams[*place].bound = endpoint
                .entity
                .input_binding(at as u16)
                .map(|binding| binding_text(binding.talker, binding.talker_unique_id));
        }
        let instance = endpoint.config.instance_at(endpoint.place);
        let place = endpoint.place;
        let keep = |entry: &mut EndpointConfig| {
            let before = entry.clone();
            entry.name.clone_from(&name);
            entry.streams.clone_from(&streams);
            *entry != before
        };
        let Some(entry) = self.config.endpoints.get_mut(place) else {
            return;
        };
        if !keep(entry) {
            return;
        }
        let Some(path) = self.path.clone() else {
            return;
        };
        if modified(Some(&path)) != self.modified {
            // The file changed since it was read, as when the app wrote
            // it: the change goes into what it holds now, which the
            // daemon then reads again, rather than over it.
            let Ok(mut now) = triib_store::load::<Config>(&path) else {
                return;
            };
            let found = now
                .endpoints
                .iter()
                .enumerate()
                .position(|(place, entry)| entry.instance_at(place) == instance);
            if let Some(found) = found
                && keep(&mut now.endpoints[found])
                && let Err(error) = triib_store::save(&path, &now)
            {
                log(format!("entities.toml: {error}"));
            }
            return;
        }
        if let Err(error) = triib_store::save(&path, &self.config) {
            log(format!("entities.toml: {error}"));
        }
        self.modified = modified(Some(&path));
    }

    /// Keeps a setting of the whole configuration that a controller gave
    /// in the configuration file.
    fn keep_config(&mut self) {
        let Some(path) = self.path.clone() else {
            return;
        };
        if modified(Some(&path)) != self.modified {
            // As in keep_settings: into what the file holds now.
            let Ok(mut now) = triib_store::load::<Config>(&path) else {
                return;
            };
            now.multicast_escalation = self.config.multicast_escalation;
            if let Err(error) = triib_store::save(&path, &now) {
                log(format!("entities.toml: {error}"));
            }
            return;
        }
        if let Err(error) = triib_store::save(&path, &self.config) {
            log(format!("entities.toml: {error}"));
        }
        self.modified = modified(Some(&path));
    }

    fn write_status(&self) {
        let Some(path) = status_path() else {
            return;
        };
        let endpoints = self
            .endpoints
            .iter()
            .map(|endpoint| {
                let streaming = endpoint
                    .outputs
                    .iter()
                    .any(|output| output.talker.is_some());
                let listening = endpoint.inputs.iter().any(|input| input.listener.is_some());
                let bound = (0..endpoint.inputs.len() as u16)
                    .any(|input| endpoint.entity.input_binding(input).is_some());
                EndpointStatus {
                    entity_id: endpoint.entity.entity_id().to_string(),
                    name: endpoint.entity.model().entity_name.clone(),
                    streams: stream_statuses(endpoint),
                    state: if streaming {
                        "streaming"
                    } else if listening {
                        "listening"
                    } else if bound {
                        "bound"
                    } else if endpoint.inputs.is_empty() {
                        "waiting"
                    } else {
                        "unbound"
                    }
                    .into(),
                }
            })
            .collect();
        let status = DaemonStatus {
            pid: std::process::id(),
            started: self.started,
            interface: self.interface.clone(),
            gptp: self.gptp_text.clone(),
            foreign_mrp: self.foreign_mrp_heard(),
            config: self.path.as_ref().map(|file| file.display().to_string()),
            clock: self.ptp_status.as_ref().map(|status| ClockStatus {
                grandmaster: status.gptp.grandmaster.to_string(),
                offset: status.offset,
                end_to_end: status.end_to_end,
                as_capable: status.gptp.as_capable && !status.faulty,
            }),
            endpoints,
        };
        let _ = triib_store::save(&path, &status);
    }

    /// Writes how loud each running audio stream is each
    /// [`meters::EVERY`] while an app asks, and removes the meters once
    /// none does.
    fn write_meters(&mut self) {
        if self.meters_looked.elapsed() >= METERS_LOOK {
            self.meters_looked = Clock::now();
            let wanted = meters::wanted();
            if self.meters_wanted && !wanted {
                meters::remove();
                self.meter_peaks.clear();
            }
            self.meters_wanted = wanted;
        }
        if !self.meters_wanted || self.meters_written.elapsed() < meters::EVERY {
            return;
        }
        self.meters_written = Clock::now();
        let mut peaks = HashMap::new();
        let mut streams = Vec::new();
        for endpoint in &self.endpoints {
            let entity_id = endpoint.entity.entity_id();
            let audio = |place: Option<&usize>| {
                place
                    .and_then(|&place| endpoint.streams.get(place))
                    .is_some_and(|stream| !stream.clock)
            };
            let outputs = endpoint
                .outputs
                .iter()
                .enumerate()
                .filter(|(at, _)| audio(endpoint.out_streams.get(*at)))
                .filter_map(|(at, output)| Some((false, at, output.talker.as_ref()?.take_peaks())));
            let inputs = endpoint
                .inputs
                .iter()
                .enumerate()
                .filter(|(at, _)| audio(endpoint.in_streams.get(*at)))
                .filter_map(|(at, input)| Some((true, at, input.listener.as_ref()?.take_peaks())));
            for (input, at, now) in outputs.chain(inputs) {
                let key = (entity_id, input, at as u16);
                let before = self.meter_peaks.get(&key).copied().unwrap_or_default();
                peaks.insert(key, now);
                streams.push(StreamMeter {
                    entity_id: entity_id.to_string(),
                    input,
                    index: at as u16,
                    peaks: [now[0].max(before[0]), now[1].max(before[1])],
                });
            }
        }
        self.meter_peaks = peaks;
        self.meters_sequence += 1;
        let _ = meters::write(&Meters {
            sequence: self.meters_sequence,
            streams,
        });
    }

    fn next_wait(&self) -> Duration {
        let now = self.now();
        let since = self.elapsed();
        let entity = self
            .endpoints
            .iter()
            .filter_map(|endpoint| endpoint.entity.poll_timeout())
            .min()
            .map(|at| at.saturating_duration_since(now));
        let mrp = [self.port.msrp.poll_timeout(), self.port.mvrp.poll_timeout()]
            .into_iter()
            .min()
            .map(|at| at.saturating_sub(since));
        let maap = self
            .maap
            .as_ref()
            .and_then(Maap::poll_timeout)
            .map(|at| at.saturating_duration_since(now));
        let lite = [
            Some(self.fallback.poll_timeout()),
            self.lite.as_ref().map(|lite| lite.next_refresh),
            self.lite
                .as_ref()
                .and_then(|lite| lite.repeats.iter().map(|(at, ..)| *at).min()),
            self.lite.as_ref().and_then(|lite| {
                lite.leaving_talkers
                    .values()
                    .chain(lite.leaving_listeners.values())
                    .chain(lite.leaving_refusals.values())
                    .min()
                    .copied()
            }),
            self.endpoints
                .iter()
                .flat_map(|endpoint| &endpoint.outputs)
                .filter_map(|output| output.lite.escalate_at)
                .chain(
                    self.lite
                        .iter()
                        .flat_map(|lite| lite.shared.values())
                        .filter_map(|talker| talker.escalate_at),
                )
                .min(),
        ]
        .into_iter()
        .flatten()
        .min()
        .map(|at| at.saturating_sub(since));
        let meters = self
            .meters_wanted
            .then(|| meters::EVERY.saturating_sub(self.meters_written.elapsed()));
        [
            entity,
            mrp,
            maap,
            lite,
            meters,
            Some(Duration::from_millis(250)),
        ]
        .into_iter()
        .flatten()
        .min()
        .unwrap_or(Duration::from_millis(250))
    }

    fn handle(&mut self, input: Input) {
        let now = self.now();
        let since = self.elapsed();
        match input {
            Input::Avtp(source, _, bytes)
                if bytes.first() == Some(&atdecc::avtp::subtype::MAAP) =>
            {
                if let Some(maap) = &mut self.maap {
                    maap.handle_frame(now, source, &bytes);
                }
            }
            Input::Avtp(source, group, bytes) => {
                if self.lite.is_some() && source != self.mac {
                    self.cvu_heard(source, group, &bytes);
                }
                for endpoint in &mut self.endpoints {
                    endpoint.entity.handle_frame(now, source, &bytes);
                }
            }
            Input::Ptp(source, bytes) => {
                if source != self.mac {
                    self.watch_stamps(&bytes);
                }
                // Without a hardware clock nothing is weighed: AVB Lite is
                // not for it.
                if self.hardware_clock {
                    self.fallback.handle_ptp(since, &bytes);
                }
                if let Some((grandmaster, speed)) = lite::grandmaster_link(&bytes)
                    && self.grandmaster_links.insert(grandmaster, speed) != Some(speed)
                {
                    self.update_correction();
                }
            }
            // MRP from this computer's own address that the socket did not
            // send is another participant on the port, which this one must
            // not take for a peer's.
            Input::Msrp(source, _) | Input::Mvrp(source, _) if source == self.mac => {
                self.foreign_participant();
            }
            Input::Msrp(_, bytes) => self.port.msrp.handle_pdu(since, &bytes),
            Input::Mvrp(_, bytes) => self.port.mvrp.handle_pdu(since, &bytes),
            Input::Gptp(status) => {
                // ptp4l on another interface says nothing of this one: its
                // clock identity comes from that interface's address.
                let status = status.filter(|status| status.own_clock == self.own_clock);
                if status.is_none() && self.ptp_status.is_none() && !self.gptp_text.ends_with("yet")
                {
                    return;
                }
                self.ptp_status.clone_from(&status);
                if let Some(status) = &status {
                    let grandmaster = status.gptp.grandmaster;
                    if self.grandmaster.is_some_and(|known| known != grandmaster) {
                        self.interface_counters[GPTP_GM_CHANGED] += 1;
                    }
                    self.grandmaster = Some(grandmaster);
                }
                self.update_ptp4l();
                self.update_correction();
                // A clock that far off keeps no gPTP time for the streams,
                // whatever the link: the entities say so, as not asCapable,
                // so controllers show them out of sync.
                let off = status
                    .as_ref()
                    .and_then(|status| status.offset)
                    .filter(|offset| offset.unsigned_abs() > OFF_SYNC);
                self.off_since = off.map(|_| self.off_since.unwrap_or(since));
                let state = |status: &gptp::Status| match off {
                    _ if status.faulty => "ptp4l's port FAULTY".into(),
                    Some(offset) => format!("{offset} ns off it, not in sync"),
                    None if status.gptp.as_capable => "asCapable".into(),
                    None => "not asCapable".into(),
                };
                self.gptp_text = match &status {
                    Some(status) => {
                        format!("grandmaster {}, {}", status.gptp.grandmaster, state(status))
                    }
                    None => "ptp4l does not answer for this interface".into(),
                };
                match &status {
                    Some(status) => log(format!(
                        "gPTP: grandmaster {}, peer delay {} ns, {}",
                        status.gptp.grandmaster,
                        status.gptp.propagation_delay,
                        state(status)
                    )),
                    None => log("gPTP: ptp4l does not answer for this interface"),
                }
                let mut gptp = status.map(|status| status.gptp).unwrap_or_default();
                if off.is_some() {
                    gptp.as_capable = false;
                }
                for endpoint in &mut self.endpoints {
                    endpoint.entity.set_gptp(now, gptp.clone());
                }
            }
            Input::SharerOpened(id, writer) => {
                self.sharers.open(id, writer);
                self.sharers.tell(
                    id,
                    Notice::Hello(Hello {
                        version: sharing_protocol::VERSION,
                        features: sharing_protocol::feature::LOCAL_LISTENERS
                            | sharing_protocol::feature::LISTENERS
                            | sharing_protocol::feature::AVB_LITE,
                    }),
                );
                // Whether it runs AVB Lite with the port comes after, with
                // its hello and entity.
                if self.lite.is_some() {
                    self.sharers.tell(id, Notice::Unavailable);
                }
            }
            Input::SharerAsked(id, request) => self.sharer_asked(id, request),
            Input::SharerClosed(id) => {
                let closed = self.sharers.close(id);
                for declaration in closed.streams {
                    self.withdraw_shared(closed.entity.map(EntityId), declaration);
                    self.talker_heard(declaration.stream_id);
                }
                for stream_id in closed.listens {
                    self.talker_heard(stream_id);
                }
            }
            Input::Link(link) => {
                let last = self.link.replace(link);
                let unit = last.and_then(|last| last.unit);
                if unit != link.unit {
                    match link.unit {
                        Some(profile) => log(format!(
                            "ptp4l: {} runs {profile}",
                            profile.unit(&self.interface)
                        )),
                        None if unit.is_some() => log("ptp4l: triib's units stopped"),
                        None => {}
                    }
                }
                // The fallback waits while the link is down, as nothing can
                // answer, and weighs afresh as it comes up (profile 2.2).
                let since = self.elapsed();
                if let Some(carrier) = link.carrier {
                    self.fallback.set_link(since, carrier);
                    let before = last.and_then(|last| last.carrier);
                    if before != Some(carrier) {
                        let counter = if carrier { LINK_UP } else { LINK_DOWN };
                        // Down from not known yet is no change.
                        if carrier || before.is_some() {
                            self.interface_counters[counter] += 1;
                        }
                    }
                }
                // AVB Lite holds until the link comes up again, though not
                // from a link unit taking it down.
                let came_up = link.carrier == Some(true)
                    && last.is_some_and(|last| last.carrier == Some(false))
                    && !self.link_own_until.is_some_and(|until| since < until);
                if came_up && self.lite.is_some() && self.config.avb_lite == LiteChoice::Auto {
                    log("AVB Lite: the link came up again, trying AVB");
                    self.leave_lite();
                }
                self.update_ptp4l();
                self.steer_profile();
                self.keep_link_power(last.and_then(|last| last.power));
            }
        }
    }

    /// Runs the timers, then acts on what the entities and participants
    /// report and sends what they queued.
    fn turn(&mut self) {
        let now = self.now();
        let since = self.elapsed();
        for endpoint in &mut self.endpoints {
            endpoint.entity.handle_timeout(now);
        }
        self.port.handle_timeout(since);
        if let Some(maap) = &mut self.maap {
            maap.handle_timeout(now);
        }
        // ptp4l's own fault stops its requests while it lasts. Without a
        // hardware clock nothing is weighed, nor a beacon sent.
        if self.hardware_clock {
            if self.ptp_status.as_ref().is_some_and(|status| status.faulty) {
                self.fallback.ptp4l_faulty(since);
            }
            self.fallback.handle_timeout(since);
        }
        if self.steer_pending && !self.switched_at.is_some_and(|at| since < at + SWITCH_AGAIN) {
            self.steer_profile();
        }
        self.start_ptp4l();
        self.restart_ptp4l();
        if self.lite.is_none()
            && self.config.avb_lite != LiteChoice::Off
            && let Some(reason) = self.fallback.reason()
        {
            self.enter_lite(reason);
        }
        self.lite_turn();
        while let Some((group, pdu)) = self.local_cvu.pop_front() {
            self.cvu_heard(self.mac, group, &pdu);
        }
        while let Some(event) = self.maap.as_mut().and_then(Maap::poll_event) {
            self.maap_event(event);
        }
        let addressed = self.addressed();
        for out in self.outs() {
            if addressed && self.output(out).redeclare_at.is_some_and(|at| at <= since) {
                self.output_mut(out).redeclare_at = None;
                self.declare_talker(out);
                self.update_talker(out);
            }
        }
        loop {
            for index in 0..self.endpoints.len() {
                while let Some(event) = self.endpoints[index].entity.poll_event() {
                    self.entity_event(index, event);
                }
            }
            while let Some(registration) = self.port.msrp.poll_event() {
                self.registration(registration);
            }
            while self.port.mvrp.poll_event().is_some() {}
            if !self.flush() {
                break;
            }
        }
        if self.last_counters.elapsed() >= Duration::from_secs(1) {
            self.last_counters = Clock::now();
            self.update_counters();
            for index in 0..self.endpoints.len() {
                self.update_clock_lock(index);
            }
            self.update_lite_status();
        }
        if self.last_report.elapsed() >= REPORT {
            self.last_report = Clock::now();
            self.report();
        }
    }

    /// Sends what the entities and participants queued, saying whether an
    /// entity had a frame from another of ours to handle.
    fn flush(&mut self) -> bool {
        let now = self.now();
        let mut handed = false;
        // The socket never brings back its own frames, so our entities'
        // frames go to them here as well as on the wire, the sender's own
        // among them: an input bound to its own entity's output probes it.
        loop {
            let mut sent = Vec::new();
            for endpoint in &mut self.endpoints {
                while let Some((destination, bytes)) = endpoint.entity.poll_transmit() {
                    let _ = self.avtp.send(destination, &bytes);
                    if destination == ADP_ACMP_MULTICAST || destination == self.mac {
                        sent.push(bytes);
                    }
                }
            }
            if sent.is_empty() {
                break;
            }
            for bytes in sent {
                for endpoint in &mut self.endpoints {
                    endpoint.entity.handle_frame(now, self.mac, &bytes);
                }
            }
            handed = true;
        }
        while let Some((destination, pdu)) = self.maap.as_mut().and_then(Maap::poll_transmit) {
            let _ = self.avtp.send(destination, &pdu);
        }
        while let Some(message) = self.fallback.poll_transmit() {
            let _ = self.ptp.send(lite::PDELAY_DESTINATION, &message);
        }
        while let Some(pdu) = self.port.msrp.poll_transmit() {
            let _ = self.msrp.send(avb_mrp::MSRP_DESTINATION, &pdu);
        }
        while let Some(pdu) = self.port.mvrp.poll_transmit() {
            let _ = self.mvrp.send(avb_mrp::MVRP_DESTINATION, &pdu);
        }
        handed
    }

    /// Every endpoint's stream outputs, in order.
    fn outs(&self) -> Vec<Out> {
        self.endpoints
            .iter()
            .enumerate()
            .flat_map(|(index, endpoint)| {
                (0..endpoint.outputs.len() as u16).map(move |output| Out(index, output))
            })
            .collect()
    }

    /// Every endpoint's stream inputs, in order.
    fn ins(&self) -> Vec<In> {
        self.endpoints
            .iter()
            .enumerate()
            .flat_map(|(index, endpoint)| {
                (0..endpoint.inputs.len() as u16).map(move |input| In(index, input))
            })
            .collect()
    }

    fn output(&self, out: Out) -> &OutputState {
        &self.endpoints[out.0].outputs[usize::from(out.1)]
    }

    fn output_mut(&mut self, out: Out) -> &mut OutputState {
        &mut self.endpoints[out.0].outputs[usize::from(out.1)]
    }

    fn input(&self, input: In) -> &InputState {
        &self.endpoints[input.0].inputs[usize::from(input.1)]
    }

    fn input_mut(&mut self, input: In) -> &mut InputState {
        &mut self.endpoints[input.0].inputs[usize::from(input.1)]
    }

    /// The name log lines give an output.
    fn out_name(&self, out: Out) -> String {
        self.endpoints[out.0].stream_name(false, out.1)
    }

    /// The name log lines give an input.
    fn in_name(&self, input: In) -> String {
        self.endpoints[input.0].stream_name(true, input.1)
    }

    /// The priority an output's frames go out on: AVB Lite's, else its SR
    /// class's, as its format decides.
    fn priority(&self, out: Out) -> u8 {
        let class_b = self.endpoints[out.0]
            .entity
            .model()
            .outputs
            .get(usize::from(out.1))
            .is_some_and(StreamModel::class_b);
        if self.lite.is_some() {
            LITE_PRIORITY
        } else if class_b {
            CLASS_B_PRIORITY
        } else {
            PRIORITY
        }
    }

    /// The Talker Advertise for an output's stream, from its current
    /// format.
    fn declaration(&self, out: Out) -> Option<TalkerDeclaration> {
        let endpoint = &self.endpoints[out.0];
        let (stream_id, destination) = endpoint.entity.output_stream(out.1)?;
        let format = endpoint
            .entity
            .model()
            .outputs
            .get(usize::from(out.1))?
            .current_format;
        let media = avb_stream::media::Media::of(format)?;
        Some(TalkerDeclaration {
            stream_id: stream_id.0,
            destination,
            vlan_id: if self.lite.is_some() {
                self.media_vlan()
            } else {
                VLAN
            },
            max_frame_size: media.pdu_length() as u16,
            // One frame each interval of its class, which its format
            // decides.
            max_interval_frames: 1,
            priority: self.priority(out),
            rank: true,
            accumulated_latency: TALKER_LATENCY,
            failure: None,
            event: avb_mrp::Event::New,
        })
    }

    fn declare_talker(&mut self, out: Out) {
        let Some(declaration) = self.declaration(out) else {
            return;
        };
        if self.lite.is_some()
            && let Some(declaration) = self.lite_declaration(out)
        {
            self.declare_cvu(
                self.entity_of(out.0),
                BROADCAST,
                lite::talker_message(&declaration, avb_mrp::Event::New),
            );
            self.endpoints[out.0].entity.set_output_reservation(
                out.1,
                OutputReservation {
                    ready_listeners: 0,
                    registering: true,
                },
            );
            return;
        }
        let since = self.elapsed();
        self.port.msrp.declare(
            since,
            msrp::attribute::TALKER_ADVERTISE,
            msrp::talker_value(&declaration),
            None,
        );
        self.endpoints[out.0].entity.set_output_reservation(
            out.1,
            OutputReservation {
                ready_listeners: 0,
                registering: true,
            },
        );
    }

    /// Whether the talkers have their destination addresses.
    fn addressed(&self) -> bool {
        self.maap
            .as_ref()
            .is_some_and(|maap| maap.address(0).is_some())
    }

    /// Talkers take the addresses MAAP gave them, or stop when it lost
    /// them, and our listeners of them follow.
    fn maap_event(&mut self, event: MaapEvent) {
        let since = self.elapsed();
        let now = self.now();
        let talkers = self.outs();
        match event {
            MaapEvent::Acquired(first) => {
                log(format!("MAAP: streams go to {first} on"));
                for out in talkers {
                    let slot = self.output(out).slot;
                    let Some(address) = self.maap.as_ref().and_then(|maap| maap.address(slot))
                    else {
                        continue;
                    };
                    self.endpoints[out.0]
                        .entity
                        .set_output_destination(out.1, address);
                    // Declared again only once a bridge has let the old
                    // declaration go.
                    let at = self
                        .output(out)
                        .withdrawn_at
                        .map_or(since, |withdrawn| withdrawn + REDECLARE)
                        .max(since);
                    self.output_mut(out).redeclare_at = Some(at);
                }
                // Our inputs of our talkers find where they went.
                for input in self.ins() {
                    self.endpoints[input.0].entity.probe_again(now, input.1);
                }
            }
            MaapEvent::Lost => {
                log("MAAP: another device took the streams' addresses, claiming others");
                for out in talkers {
                    let output = self.output_mut(out);
                    output.talker = None;
                    // Declared again once MAAP gives it an address.
                    output.redeclare_at = None;
                    self.withdraw_talker(out);
                }
            }
        }
    }

    /// Withdraws a talker's declaration, which a stream needs to stop.
    fn withdraw_talker(&mut self, out: Out) {
        let Some((stream_id, _)) = self.endpoints[out.0].entity.output_stream(out.1) else {
            return;
        };
        if self.lite.is_some()
            && let Some(declaration) = self.lite_declaration(out)
        {
            let entity = self.entity_of(out.0);
            self.send_cvu(
                entity,
                BROADCAST,
                &lite::talker_message(&declaration, avb_mrp::Event::Lv),
            );
            let talker = std::mem::take(&mut self.output_mut(out).lite);
            for (listener, code) in talker.refused {
                let refusal = self.refused(declaration, code);
                self.send_cvu(
                    entity,
                    listener,
                    &lite::talker_message(&refusal, avb_mrp::Event::Lv),
                );
            }
        }
        let since = self.elapsed();
        self.port.msrp.withdraw(
            since,
            msrp::attribute::TALKER_ADVERTISE,
            &stream_id.0.to_be_bytes(),
        );
        self.output_mut(out).withdrawn_at = Some(since);
        self.endpoints[out.0].entity.set_output_reservation(
            out.1,
            OutputReservation {
                ready_listeners: 0,
                registering: false,
            },
        );
    }

    fn registration(&mut self, registration: Registration) {
        match registration {
            Registration::Joined {
                attribute_type: msrp::attribute::LISTENER,
                value,
                four_packed,
            } => {
                let state = ListenerState::from_number(four_packed.unwrap_or(0));
                self.listener_heard(stream_of(&value), Some(state));
            }
            Registration::Left {
                attribute_type: msrp::attribute::LISTENER,
                value,
            } => self.listener_heard(stream_of(&value), None),
            Registration::Joined {
                attribute_type,
                value,
                ..
            } if is_talker(attribute_type) => {
                if let Some(declaration) =
                    msrp::decode_talker(attribute_type, &value, avb_mrp::Event::JoinIn)
                {
                    self.talkers.insert(declaration.stream_id, declaration);
                    self.talker_heard(declaration.stream_id);
                }
            }
            Registration::Left {
                attribute_type,
                value,
            } if is_talker(attribute_type) => {
                let stream_id = stream_of(&value);
                if self.talkers.remove(&stream_id).is_some() {
                    self.talker_heard(stream_id);
                }
            }
            _ => {}
        }
    }

    /// Which of our outputs sends `stream_id`.
    fn own_talker(&self, stream_id: u64) -> Option<Out> {
        self.outs().into_iter().find(|out| {
            self.endpoints[out.0]
                .entity
                .output_stream(out.1)
                .is_some_and(|(id, _)| id.0 == stream_id)
        })
    }

    /// The stream an input settled on, its probe having found it.
    fn input_stream(&self, input: In) -> Option<atdecc::entity::ProbedStream> {
        self.endpoints[input.0].entity.input_stream(input.1)
    }

    /// A listener's declaration for one of our talkers' streams, or of a
    /// program sharing the port, changed.
    fn listener_heard(&mut self, stream_id: u64, state: Option<ListenerState>) {
        match state {
            Some(state) => self.listener_states.insert(stream_id, state),
            None => self.listener_states.remove(&stream_id),
        };
        if self.sharers.owner(stream_id).is_some() {
            self.tell_sharer(stream_id);
            return;
        }
        let ready = state.is_some_and(ListenerState::ready);
        self.ready_remotely.insert(stream_id, ready);
        if let Some(out) = self.own_talker(stream_id) {
            self.update_talker(out);
        }
    }

    /// A program sharing the port's participant asks it to declare or
    /// withdraw one of its talkers, or to listen to a stream or stop: not
    /// while the port runs AVB Lite where the program does not run it too,
    /// nor a stream another talker here declares. It hears whether its
    /// stream was declared, and of the talker of one it listens to.
    fn sharer_asked(&mut self, id: u64, request: Request) {
        let refuse = |sharers: &mut Sharers, stream_id, refusal| {
            sharers.tell(id, Notice::Refused { stream_id, refusal });
        };
        let unable = self.lite.is_some() && !self.sharers.lite(id);
        match request {
            Request::Hello(hello) => {
                let was = self.sharers.lite(id);
                self.sharers.said_hello(id, hello);
                self.sharer_runs_lite(id, was);
            }
            Request::Entity(entity) => {
                let was = self.sharers.lite(id);
                self.sharers.named(id, entity);
                self.sharer_runs_lite(id, was);
            }
            Request::Escalation(allowed) => self.set_escalation(allowed),
            Request::Declare(declaration) => {
                let stream_id = declaration.stream_id;
                if unable {
                    // A program of version 0 knows only this.
                    self.sharers.tell(id, Notice::Unavailable);
                    refuse(&mut self.sharers, stream_id, Refusal::Unavailable);
                    return;
                }
                let taken = self.own_talker(stream_id).is_some()
                    || self
                        .sharers
                        .owner(stream_id)
                        .is_some_and(|owner| owner != id);
                if taken {
                    refuse(&mut self.sharers, stream_id, Refusal::Taken);
                    return;
                }
                self.sharers.declared(id, declaration);
                self.sharers.tell(id, Notice::Declared(stream_id));
                self.declare_shared(stream_id);
                // Our inputs settled on it listen, and it hears of them and
                // of listeners already there for it.
                self.talker_heard(stream_id);
                self.tell_sharer(stream_id);
            }
            Request::Withdraw(stream_id) => {
                if self.sharers.owner(stream_id) != Some(id) {
                    return;
                }
                let entity = self.sharers.owner_entity(stream_id).map(EntityId);
                if let Some(declaration) = self.sharers.withdrawn(id, stream_id) {
                    self.withdraw_shared(entity, declaration);
                    self.talker_heard(stream_id);
                }
            }
            Request::Listen(stream_id) => {
                if unable {
                    refuse(&mut self.sharers, stream_id, Refusal::Unavailable);
                    return;
                }
                // Asking twice is asking once.
                if self.sharers.listen(id, stream_id) {
                    self.talker_heard(stream_id);
                }
            }
            Request::Unlisten(stream_id) => {
                if self.sharers.unlisten(id, stream_id) {
                    self.talker_heard(stream_id);
                }
            }
            Request::Unreadable(stream_id) => {
                refuse(
                    &mut self.sharers,
                    stream_id.unwrap_or(0),
                    Refusal::Unreadable,
                );
            }
            // Kinds a later version gives, which the reader skips.
            _ => {}
        }
    }

    /// A program now runs AVB Lite with the port, its hello saying so and
    /// its entity named: it hears the port's mode and, while the port runs
    /// AVB Lite, that it can declare and listen after all.
    fn sharer_runs_lite(&mut self, id: u64, was: bool) {
        if was || !self.sharers.lite(id) {
            return;
        }
        let mode = self.mode();
        self.sharers.tell_mode(mode);
        if self.lite.is_some() {
            self.sharers.tell(id, Notice::Available);
        }
    }

    /// The port's mode and AVB Lite settings, which the programs sharing
    /// it report for their entities.
    fn mode(&self) -> Mode {
        Mode {
            lite: self.lite.as_ref().map(|lite| lite.reason.0),
            media_vlan: self.media_vlan(),
            unicast_fanout: u8::try_from(self.fanout()).unwrap_or(u8::MAX),
            escalation: self.config.multicast_escalation,
        }
    }

    /// Whether streams may escalate to multicast beyond their unicast
    /// fan-out, as a controller set it on one of this computer's entities
    /// or a program's: for every talker here, which are admitted again,
    /// and every program hears of it.
    fn set_escalation(&mut self, allowed: bool) {
        log(format!(
            "AVB Lite: escalating to multicast {}",
            if allowed { "allowed" } else { "not allowed" }
        ));
        self.config.multicast_escalation = allowed;
        self.keep_config();
        self.update_lite_status();
        if self.lite.is_some() {
            self.update_own_talkers();
        }
        let mode = self.mode();
        self.sharers.tell_mode(mode);
    }

    /// Declares a program's talker on the port: by MSRP, or in AVB Lite by
    /// CVU SRP in its entity's name, sent to the listeners it admits.
    fn declare_shared(&mut self, stream_id: u64) {
        let Some(declaration) = self.sharers.declaration(stream_id) else {
            return;
        };
        let since = self.elapsed();
        let Some(lite) = &self.lite else {
            self.port.msrp.declare(
                since,
                msrp::attribute::TALKER_ADVERTISE,
                msrp::talker_value(&declaration),
                None,
            );
            return;
        };
        let escalated = lite
            .shared
            .get(&stream_id)
            .is_some_and(|talker| talker.escalated);
        let Some(entity) = self.sharers.owner_entity(stream_id).map(EntityId) else {
            return;
        };
        let declared = self.shared_lite_declaration(declaration, escalated);
        self.declare_cvu(
            entity,
            BROADCAST,
            lite::talker_message(&declared, avb_mrp::Event::New),
        );
        self.lite_update_shared(stream_id);
    }

    /// Withdraws a program's talker declaration: by MSRP, or in AVB Lite by
    /// CVU SRP in its entity's name, with the refusals it sent.
    fn withdraw_shared(&mut self, entity: Option<EntityId>, declaration: TalkerDeclaration) {
        let since = self.elapsed();
        let stream_id = declaration.stream_id;
        let Some(lite) = &mut self.lite else {
            self.port.msrp.withdraw(
                since,
                msrp::attribute::TALKER_ADVERTISE,
                &stream_id.to_be_bytes(),
            );
            return;
        };
        let talker = lite.shared.remove(&stream_id).unwrap_or_default();
        let Some(entity) = entity else {
            return;
        };
        let declared = self.shared_lite_declaration(declaration, talker.escalated);
        self.send_cvu(
            entity,
            BROADCAST,
            &lite::talker_message(&declared, avb_mrp::Event::Lv),
        );
        for (listener, code) in talker.refused {
            let refusal = self.refused(declared, code);
            self.send_cvu(
                entity,
                listener,
                &lite::talker_message(&refusal, avb_mrp::Event::Lv),
            );
        }
    }

    /// Tells the program sharing the port that declared `stream_id` of its
    /// listeners: those on the network as the port registers them, merged
    /// with those here, ours and the programs', which are ready as its
    /// frames reach them as they leave (35.2.4.4.1); in AVB Lite, as its
    /// copies serve them.
    fn tell_sharer(&mut self, stream_id: u64) {
        if self.lite.is_some() {
            self.lite_update_shared(stream_id);
            return;
        }
        let remote = self.listener_states.get(&stream_id).copied();
        let state = if self.local_listeners(stream_id) == 0 {
            remote
        } else {
            Some(match remote {
                Some(ListenerState::AskingFailed | ListenerState::ReadyFailed) => {
                    ListenerState::ReadyFailed
                }
                _ => ListenerState::Ready,
            })
        };
        self.sharers.tell_listener(stream_id, state);
    }

    /// How many listeners here take `stream_id` as it leaves: our inputs
    /// listening to it, and the programs.
    fn local_listeners(&self, stream_id: u64) -> usize {
        let ours = self
            .ins()
            .into_iter()
            .filter(|input| {
                self.input(*input).listener.is_some()
                    && self
                        .input_stream(*input)
                        .is_some_and(|stream| stream.stream_id.0 == stream_id)
            })
            .count();
        ours + self.sharers.listening(stream_id)
    }

    /// Starts an output's stream when a listener is ready for it, on the
    /// network or here, and stops it when none is.
    fn update_talker(&mut self, out: Out) {
        if self.lite.is_some() {
            self.lite_update_talker(out);
            return;
        }
        let Some((stream_id, _)) = self.endpoints[out.0].entity.output_stream(out.1) else {
            return;
        };
        let remote = self
            .ready_remotely
            .get(&stream_id.0)
            .copied()
            .unwrap_or(false);
        let local = self.local_listeners(stream_id.0);
        let listeners = usize::from(remote) + local;
        let ready = listeners > 0;
        self.endpoints[out.0].entity.set_output_reservation(
            out.1,
            OutputReservation {
                ready_listeners: listeners.min(usize::from(u16::MAX)) as u16,
                registering: true,
            },
        );
        let addressed = self.addressed();
        let name = self.out_name(out);
        let output = self.output_mut(out);
        if ready && output.talker.is_none() && output.redeclare_at.is_none() && addressed {
            log(format!("{name}: a listener is ready, streaming"));
            self.start_talker(out);
        } else if !ready && output.talker.is_some() {
            log(format!("{name}: no listener ready, stopping"));
            output.talker = None;
        }
    }

    fn start_talker(&mut self, out: Out) {
        // What it sent before stops first: its place in the group is the
        // new stream's, which dropping the old one later would stop.
        self.output_mut(out).talker = None;
        let endpoint = &self.endpoints[out.0];
        let Some((stream_id, destination)) = endpoint.entity.output_stream(out.1) else {
            return;
        };
        let Some(stream) = endpoint.entity.model().outputs.get(usize::from(out.1)) else {
            return;
        };
        let transit = endpoint.entity.max_transit_time(out.1).unwrap_or(2_000_000);
        let lite = self.lite.is_some();
        let name = self.out_name(out);
        let format = stream.current_format;
        let source = match &self.output(out).device {
            Some(device) => Source::Shared(device.shared()),
            None if endpoint.out_config(out.1).clock => Source::Silence,
            None => endpoint.out_config(out.1).source(),
        };
        let Some(media) = avb_stream::media::Media::of(format) else {
            log(format!(
                "{name}: could not start the stream: not a format it sends"
            ));
            return;
        };
        // A media clock goes out on the entity's audio clock, from its
        // group.
        let rate = if media.is_clock() {
            endpoint.entity.model().current_sampling_rate
        } else {
            media.sample_rate()
        };
        let wanted = GroupStream {
            stream_id: stream_id.0,
            destinations: if lite {
                self.output(out).lite.copies.clone()
            } else {
                vec![destination]
            },
            vlan_id: if lite { self.media_vlan() } else { VLAN },
            priority: self.priority(out),
            format,
            max_transit_time: Duration::from_nanos(u64::from(transit)),
            source,
            channels: stream.channels,
        };
        // Every output sends from the endpoint's one group; one at another
        // rate, as the entity's moved, starts a new group.
        let group = match endpoint.group.upgrade() {
            Some(group) if group.rate() == rate => Ok(group),
            _ => TalkerGroup::start(
                GroupConfig {
                    interface: self.interface.clone(),
                    mac: self.mac,
                    rate,
                },
                MOST_STREAMS,
                self.clock.clone(),
            )
            .map(Arc::new),
        };
        // An audio device that isn't there, as on another computer than
        // the one its name came from, leaves the stream running silent.
        let followed = endpoint.followed();
        let started = group.and_then(|group| {
            self.endpoints[out.0].group = Arc::downgrade(&group);
            group.follow(followed);
            let place = usize::from(out.1);
            GroupTalker::start(&group, place, wanted.clone()).or_else(|error| {
                if wanted.source == Source::Silence {
                    return Err(error);
                }
                log(format!("{name}: {error}, sending silence"));
                let silent = GroupStream {
                    source: Source::Silence,
                    ..wanted
                };
                GroupTalker::start(&group, place, silent)
            })
        });
        match started {
            Ok(talker) => self.output_mut(out).talker = Some(talker),
            Err(error) => log(format!("{name}: could not start the stream: {error}")),
        }
    }

    /// A stream's talker or listeners here changed: one Listener declaration
    /// on the port for all our inputs settled on it and the programs
    /// listening to it, as its talker declares it; then each input listens
    /// or stops, and each program hears of the talker.
    fn talker_heard(&mut self, stream_id: u64) {
        let declaration = if self.lite.is_some() {
            self.lite_listen(stream_id)
        } else {
            self.listen(stream_id)
        };
        for input in self.settled_on(stream_id) {
            self.input_heard(input, declaration);
        }
        self.sharers.tell_talker(stream_id, declaration);
        self.update_own_talkers();
    }

    /// An input settled on a stream, moved to another, or stopped: the
    /// listeners of each, as their talkers declare them.
    fn update_listener(&mut self, input: In) {
        let stream = self.input_stream(input).map(|stream| stream.stream_id.0);
        let before = std::mem::replace(&mut self.input_mut(input).counted, stream);
        if let Some(before) = before.filter(|before| Some(*before) != stream) {
            self.talker_heard(before);
        }
        if let Some(stream) = stream {
            self.talker_heard(stream);
        }
    }

    /// Our inputs settled on `stream_id`.
    fn settled_on(&self, stream_id: u64) -> Vec<In> {
        self.ins()
            .into_iter()
            .filter(|input| {
                self.input_stream(*input)
                    .is_some_and(|stream| stream.stream_id.0 == stream_id)
            })
            .collect()
    }

    /// How many listeners here `stream_id` has: our inputs settled on it,
    /// and the programs listening to it.
    fn listeners_here(&self, stream_id: u64) -> usize {
        self.settled_on(stream_id).len() + self.sharers.listening(stream_id)
    }

    /// A stream's talker declaration: ours, a program's sharing the port,
    /// or one the port registered. Its listeners here make one Listener
    /// declaration on the port for it, unless its talker is here too: the
    /// bridge never sends our own declarations back to us, and its frames
    /// reach them as they leave.
    fn listen(&mut self, stream_id: u64) -> Option<TalkerDeclaration> {
        let since = self.elapsed();
        let own = self.own_talker(stream_id);
        let shared = self.sharers.declaration(stream_id);
        let here = own.is_some() || shared.is_some();
        let declaration = match own {
            Some(talker) if self.addressed() => self.declaration(talker),
            Some(_) => None,
            None => shared.or_else(|| self.talkers.get(&stream_id).copied()),
        };
        let wanted = declaration
            .filter(|_| !here && self.listeners_here(stream_id) > 0)
            .map(|declaration| {
                if declaration.failure.is_some() {
                    ListenerState::AskingFailed
                } else {
                    ListenerState::Ready
                }
            });
        let key = stream_id.to_be_bytes();
        match wanted {
            Some(state) => {
                if self.port_listeners.insert(stream_id, state) != Some(state) {
                    self.port.msrp.declare(
                        since,
                        msrp::attribute::LISTENER,
                        key.to_vec(),
                        Some(state.number()),
                    );
                }
            }
            None => {
                if self.port_listeners.remove(&stream_id).is_some() {
                    self.port
                        .msrp
                        .withdraw(since, msrp::attribute::LISTENER, &key);
                }
            }
        }
        declaration
    }

    /// An input settled on a stream takes its talker's declaration: its
    /// reservation as its entity reports it, and it listens while the
    /// stream is advertised, at the address its talker declares, and in
    /// AVB Lite at this computer's own, where its copies come.
    fn input_heard(&mut self, input: In, declaration: Option<TalkerDeclaration>) {
        let Some(stream) = self.input_stream(input) else {
            return;
        };
        let reservation = match declaration {
            Some(declaration) => InputReservation {
                talker_registered: declaration.failure.is_none(),
                failure: declaration
                    .failure
                    .map(|failure| (failure.code, failure.bridge_id)),
                accumulated_latency: declaration.accumulated_latency,
                registering: true,
            },
            None => InputReservation::default(),
        };
        self.endpoints[input.0]
            .entity
            .set_input_reservation(input.1, reservation);
        let advertised = declaration.filter(|declaration| declaration.failure.is_none());
        // Where the stream goes, as its talker declares it now: a probe
        // can have answered before the talker had its address, or before
        // it moved. AVB Lite's unicast declares none, and the stream keeps
        // the one probing found, so escalating moves nothing.
        let declared = advertised
            .map(|declaration| declaration.destination)
            .filter(|destination| destination.0 != [0; 6]);
        let stream = atdecc::entity::ProbedStream {
            destination: declared.unwrap_or(stream.destination),
            ..stream
        };
        let lite = self.lite.is_some();
        let mac = self.mac;
        let own = self.own_bridge_id();
        let name = self.in_name(input);
        let state = self.input_mut(input);
        if state.listener.is_some() && state.listening_to != Some(stream) {
            state.listener = None;
        }
        if advertised.is_some() && state.listener.is_none() {
            let mut destinations = vec![stream.destination];
            if lite && stream.destination != mac {
                destinations.insert(0, mac);
            }
            self.start_listener(input, stream, destinations);
        } else if advertised.is_none() && state.listener.take().is_some() {
            let why = match declaration.and_then(|declaration| declaration.failure) {
                Some(failure)
                    if lite && failure.code == FAILURE_BANDWIDTH && failure.bridge_id == own =>
                {
                    "the link has no room".to_owned()
                }
                Some(failure) => format!("the talker failed, code {}", failure.code),
                None => "the talker is gone".to_owned(),
            };
            log(format!("{name}: {why}, not listening"));
        }
    }

    /// Starts a listener's thread on `stream`, taking frames sent to any
    /// of `destinations`.
    fn start_listener(
        &mut self,
        input: In,
        stream: atdecc::entity::ProbedStream,
        destinations: Vec<MacAddress>,
    ) {
        // What it took before stops first, as for a talker.
        self.input_mut(input).listener = None;
        let endpoint = &self.endpoints[input.0];
        let Some(model) = endpoint.entity.model().inputs.get(usize::from(input.1)) else {
            return;
        };
        let name = self.in_name(input);
        let config = ListenerConfig {
            interface: self.interface.clone(),
            stream_id: stream.stream_id.0,
            destinations,
            format: model.current_format,
            sink: match &self.input(input).device {
                Some(device) => Sink::Shared(device.shared()),
                None if endpoint.in_config(input.1).clock => Sink::Discard,
                None => endpoint.in_config(input.1).sink(),
            },
            recovered: Some(self.input(input).recovered.clone()),
            // Each of its audio's channels the stream channel its cluster
            // is mapped from, where a controller mapped them otherwise, or
            // its own where the stream has other channels than its audio,
            // silence past the stream's.
            channel_map: (!endpoint.entity.input_mappings_are_default(input.1))
                .then(|| endpoint.entity.input_mappings(input.1))
                .flatten()
                .map(|mappings| {
                    let mut map = vec![None; usize::from(model.channels)];
                    for mapping in mappings {
                        if let Some(slot) = map.get_mut(usize::from(mapping.cluster_offset)) {
                            *slot = Some(mapping.stream_channel);
                        }
                    }
                    map
                })
                .or_else(|| {
                    model
                        .current_format
                        .channels()
                        .is_some_and(|channels| channels != model.channels)
                        .then(|| (0..model.channels).map(Some).collect())
                }),
        };
        // Every input takes its stream on the endpoint's one listening
        // thread, which the first to listen starts, with room for as many
        // as an entity may have, so streams added go on it too.
        let group = match endpoint.listening.upgrade() {
            Some(group) => Ok(group),
            None => ListenerGroup::start(MOST_STREAMS, self.clock.clone()).map(Arc::new),
        };
        // An audio device that isn't there, as on another computer than
        // the one its name came from, leaves the stream playing nowhere.
        let started = group.and_then(|group| {
            self.endpoints[input.0].listening = Arc::downgrade(&group);
            let place = usize::from(input.1);
            GroupListener::start(&group, place, config.clone()).or_else(|error| {
                if config.sink == Sink::Discard {
                    return Err(error);
                }
                log(format!("{name}: {error}, playing nowhere"));
                let discarding = ListenerConfig {
                    sink: Sink::Discard,
                    ..config
                };
                GroupListener::start(&group, place, discarding)
            })
        });
        match started {
            Ok(listener) => {
                log(format!("{name}: listening to {}", stream.stream_id));
                let state = self.input_mut(input);
                state.listener = Some(listener);
                state.listening_to = Some(stream);
            }
            Err(error) => log(format!("{name}: could not listen: {error}")),
        }
    }

    /// Our talkers, again, after one of our listeners started or stopped,
    /// and the programs sharing the port hear of it.
    fn update_own_talkers(&mut self) {
        for out in self.outs() {
            self.update_talker(out);
        }
        for stream_id in self.sharers.streams() {
            self.tell_sharer(stream_id);
        }
    }

    fn stop_listener(&mut self, input: In) {
        let name = self.in_name(input);
        if self.input_mut(input).listener.take().is_some() {
            log(format!("{name}: not listening"));
        }
        self.endpoints[input.0]
            .entity
            .set_input_reservation(input.1, InputReservation::default());
        // The stream it was settled on has a listener fewer here.
        self.update_listener(input);
        self.update_own_talkers();
    }

    fn entity_event(&mut self, index: usize, event: EntityEvent) {
        let name = self.endpoints[index].config.name.clone();
        match event {
            EntityEvent::Identify(on) => log(format!(
                "{name}: identify {}",
                if on { "on" } else { "off" }
            )),
            EntityEvent::InputBound { index: at, binding } => {
                let name = self.in_name(In(index, at));
                log(format!(
                    "{name}: bound to {} output {}",
                    binding.talker, binding.talker_unique_id
                ));
                self.keep_settings(index);
            }
            EntityEvent::InputSettled { index: at, stream } => {
                let input = In(index, at);
                log(format!(
                    "{}: probed stream {} to {}",
                    self.in_name(input),
                    stream.stream_id,
                    stream.destination
                ));
                // A stream that moved is taken from where it went.
                let state = self.input_mut(input);
                if state.listener.is_some() && state.listening_to != Some(stream) {
                    state.listener = None;
                }
                self.update_listener(input);
            }
            EntityEvent::InputUnsettled { index: at } => self.stop_listener(In(index, at)),
            EntityEvent::InputUnbound { index: at } => {
                log(format!("{}: unbound", self.in_name(In(index, at))));
                self.stop_listener(In(index, at));
                self.keep_settings(index);
            }
            EntityEvent::StreamFormatChanged {
                descriptor_type,
                index: at,
                format,
            } => {
                let input = descriptor_type == atdecc::DescriptorType::STREAM_INPUT;
                log(format!(
                    "{}: format {format}",
                    self.endpoints[index].stream_name(input, at)
                ));
                self.keep_settings(index);
                if input {
                    let input = In(index, at);
                    if self.input_mut(input).listener.take().is_some() {
                        self.update_listener(input);
                    }
                } else {
                    let out = Out(index, at);
                    self.output_mut(out).talker = None;
                    self.withdraw_talker(out);
                    self.output_mut(out).redeclare_at = Some(self.elapsed() + REDECLARE);
                }
            }
            EntityEvent::MaxTransitTimeChanged {
                index: at,
                nanoseconds,
            } => {
                let out = Out(index, at);
                log(format!(
                    "{}: max transit time {nanoseconds} ns",
                    self.out_name(out)
                ));
                if self.output_mut(out).talker.take().is_some() {
                    self.start_talker(out);
                }
                self.keep_settings(index);
            }
            EntityEvent::SamplingRateChanged(rate) => {
                log(format!("{name}: sampling rate {rate}"));
                // The virtual devices run at the entity's rate: made
                // anew, and the streams on them start again.
                self.endpoints[index].make_devices(self.audio.as_ref());
                for at in 0..self.endpoints[index].outputs.len() as u16 {
                    if self.output_mut(Out(index, at)).talker.take().is_some() {
                        self.start_talker(Out(index, at));
                    }
                }
                for at in 0..self.endpoints[index].inputs.len() as u16 {
                    if self.input_mut(In(index, at)).listener.take().is_some() {
                        self.update_listener(In(index, at));
                    }
                }
            }
            EntityEvent::ClockSourceChanged(_) => {
                let endpoint = &self.endpoints[index];
                log(format!(
                    "{name}: clock source {}",
                    endpoint.clock_source_name()
                ));
                if let Some(group) = endpoint.group.upgrade() {
                    group.follow(endpoint.followed());
                }
                self.update_clock_lock(index);
                self.keep_settings(index);
            }
            EntityEvent::LiteConfigChanged { escalation_allowed } => {
                self.set_escalation(escalation_allowed);
            }
            EntityEvent::MappingsChanged { index: at } => {
                let input = In(index, at);
                log(format!("{}: mappings changed", self.in_name(input)));
                self.keep_settings(index);
                // Its audio takes the stream's channels anew.
                if self.input_mut(input).listener.take().is_some() {
                    self.update_listener(input);
                }
            }
            EntityEvent::NameChanged => {
                log(format!(
                    "{name}: renamed {}",
                    self.endpoints[index].entity.model().entity_name
                ));
                self.keep_settings(index);
            }
        }
    }

    /// Notes whether an endpoint's clock domain is locked to its clock
    /// source: always to the internal clock, to a media clock input while
    /// its clock is recovered, its talkers on gPTP time until then. Its
    /// counters count each lock and loss.
    fn update_clock_lock(&mut self, index: usize) {
        let endpoint = &mut self.endpoints[index];
        let locked = endpoint.followed().is_none_or(|clock| clock.locked());
        let changed = locked != endpoint.clock_locked;
        if changed {
            endpoint.clock_locked = locked;
            if locked {
                endpoint.clock_locks += 1;
            } else {
                endpoint.clock_unlocks += 1;
            }
        }
        let mut counters = [0; 32];
        counters[0] = endpoint.clock_locks;
        counters[1] = endpoint.clock_unlocks;
        endpoint
            .entity
            .set_counters(atdecc::DescriptorType::CLOCK_DOMAIN, 0, counters);
        if changed {
            let name = endpoint.config.name.clone();
            let source = endpoint.clock_source_name();
            log(if locked {
                format!("{name}: locked to {source}")
            } else {
                format!("{name}: waiting for {source}, on gPTP time")
            });
        }
    }

    /// Hands each running stream's counters to its entity, in Milan's
    /// places for them.
    fn update_counters(&mut self) {
        let low = |value: u64| value as u32;
        for endpoint in &mut self.endpoints {
            endpoint.entity.set_counters(
                atdecc::DescriptorType::AVB_INTERFACE,
                0,
                self.interface_counters,
            );
            for (at, output) in endpoint.outputs.iter().enumerate() {
                let Some(talker) = &output.talker else {
                    continue;
                };
                let stats = talker.stats();
                let mut counters = [0; 32];
                counters[0] = 1; // stream_start
                counters[3] = low(stats.media_resets);
                counters[5] = low(stats.frames_sent);
                counters[7] = low(stats.frames_sent);
                endpoint.entity.set_counters(
                    atdecc::DescriptorType::STREAM_OUTPUT,
                    at as u16,
                    counters,
                );
            }
            for (at, input) in endpoint.inputs.iter_mut().enumerate() {
                match &input.listener {
                    Some(listener) => {
                        let stats = listener.stats();
                        let counters = &mut input.counters;
                        *counters = [0; 32];
                        counters[0] = low(stats.media_locked);
                        counters[1] = low(stats.media_unlocked);
                        counters[2] = low(stats.interrupted);
                        counters[3] = low(stats.sequence_mismatches);
                        counters[6] = low(stats.frames_received);
                        counters[8] = low(stats.unsupported_formats);
                        counters[9] = low(stats.late);
                        counters[10] = low(stats.early);
                        counters[11] = low(stats.frames_received);
                    }
                    // A listener that stopped while locked unlocked as it
                    // stopped, so a controller reading the counters sees
                    // the stream gone.
                    None if input.counters[0] > input.counters[1] => input.counters[1] += 1,
                    None => continue,
                }
                endpoint.entity.set_counters(
                    atdecc::DescriptorType::STREAM_INPUT,
                    at as u16,
                    input.counters,
                );
            }
        }
    }

    fn report(&self) {
        for endpoint in &self.endpoints {
            for (at, output) in endpoint.outputs.iter().enumerate() {
                let Some(talker) = &output.talker else {
                    continue;
                };
                let stats = talker.stats();
                log(format!(
                    "{}: sent {} frames, {} media resets, {} send errors, device {:+} ppm, {} short{}",
                    endpoint.stream_name(false, at as u16),
                    stats.frames_sent,
                    stats.media_resets,
                    stats.send_errors,
                    stats.drift_ppm,
                    stats.underruns,
                    if stats.realtime { ", real time" } else { "" }
                ));
            }
            for (at, input) in endpoint.inputs.iter().enumerate() {
                let Some(listener) = &input.listener else {
                    continue;
                };
                let stats = listener.stats();
                // The stream's clock, as recovered: a media clock's always
                // said, an audio stream's once it is.
                let clock = match (input.recovered.fit(), endpoint.in_config(at as u16).clock) {
                    (Some(fit), _) => format!(", clock {:+.2} ppm", fit.ppm()),
                    (None, true) => ", clock not recovered".into(),
                    (None, false) => String::new(),
                };
                log(format!(
                    "{}: received {} frames, {} out of sequence, {} dropped here, {} late, {} early, peak {:.3}, device {:+} ppm, {} short{}{}{clock}",
                    endpoint.stream_name(true, at as u16),
                    stats.frames_received,
                    stats.sequence_mismatches,
                    stats.socket_drops,
                    stats.late,
                    stats.early,
                    listener.take_peak(),
                    stats.drift_ppm,
                    stats.device_underruns,
                    match stats.playout {
                        Some((0, error)) =>
                            format!(", plays on time {:+.2} ms", error as f64 / 1e6),
                        Some((delay, error)) => format!(
                            ", plays {:.0} ms after presentation {:+.2} ms",
                            delay as f64 / 1e6,
                            error as f64 / 1e6
                        ),
                        None => String::new(),
                    },
                    if stats.realtime { ", real time" } else { "" }
                ));
            }
        }
    }

    /// Stops the streams, withdraws the declarations and says the entities
    /// are leaving.
    /// Notes another MRP participant on the port, and says so when it is
    /// first heard after a quiet spell.
    fn foreign_participant(&mut self) {
        if !self.foreign_mrp_heard() {
            log(format!(
                "another program declares MSRP or MVRP on {} from this computer's address; \
                 two participants on one port can withdraw what these endpoints need",
                self.interface
            ));
        }
        self.foreign_mrp = Some(Clock::now());
    }

    /// Whether another participant was heard on the port lately.
    fn foreign_mrp_heard(&self) -> bool {
        self.foreign_mrp
            .is_some_and(|heard| heard.elapsed() < FOREIGN_MRP_QUIET)
    }

    fn shut_down(&mut self) {
        let since = self.elapsed();
        meters::remove();
        if self.lite.is_some() {
            // AVB Lite's declarations go too, as no bridge withdraws them.
            for out in self.outs() {
                self.withdraw_talker(out);
            }
            self.withdraw_lite_listening();
            self.withdraw_lite_shared();
        }
        for endpoint in &mut self.endpoints {
            for output in &mut endpoint.outputs {
                output.talker = None;
            }
            for input in &mut endpoint.inputs {
                input.listener = None;
            }
            endpoint.entity.depart();
        }
        let mut streams: Vec<StreamId> = self
            .outs()
            .into_iter()
            .filter_map(|out| self.endpoints[out.0].entity.output_stream(out.1))
            .map(|(stream_id, _)| stream_id)
            .collect();
        // The programs sharing the port go too, their streams with them.
        streams.extend(self.sharers.streams().into_iter().map(StreamId));
        self.sharers.close_all();
        self.sharing = None;
        self.port.withdraw_streams(since, streams);
        // Long enough for the withdrawals to go out.
        let until = since + Duration::from_millis(400);
        while self.elapsed() < until {
            let now = self.elapsed();
            self.port.handle_timeout(now);
            let _ = self.flush();
            std::thread::sleep(Duration::from_millis(20));
        }
        log("stopped");
    }
}

// AVB Lite: CVU SRP in place of MSRP, and streams unicast to each
// listener (AVB Lite profile, 6).
impl Runtime {
    /// Falls back to AVB Lite: MSRP and MVRP give way to CVU SRP, and the
    /// streams start again as its declarations come.
    fn enter_lite(&mut self, reason: FallbackReason) {
        log(format!(
            "AVB Lite: {}, declaring with CVU SRP",
            match reason {
                FallbackReason::ENDPOINT_TLV => "another endpoint answers peer delay",
                FallbackReason::PDELAY_UNANSWERED => "no gPTP peer answers",
                FallbackReason::MULTIPLE_RESPONDERS => "more than one peer answers",
                _ => "configured",
            }
        ));
        let since = self.elapsed();
        // Programs sharing the port declare by MSRP, which AVB Lite does
        // without: those running AVB Lite with it keep their streams and
        // listening, declared by CVU SRP from now on, and the others' go
        // until it leaves.
        for stream_id in self.sharers.streams() {
            self.port.msrp.withdraw(
                since,
                msrp::attribute::TALKER_ADVERTISE,
                &stream_id.to_be_bytes(),
            );
        }
        self.sharers.forget_unable();
        self.sharers.tell_unable(&Notice::Unavailable);
        for out in self.outs() {
            self.output_mut(out).talker = None;
            if let Some((stream_id, _)) = self.endpoints[out.0].entity.output_stream(out.1) {
                self.port.msrp.withdraw(
                    since,
                    msrp::attribute::TALKER_ADVERTISE,
                    &stream_id.0.to_be_bytes(),
                );
            }
        }
        for input in self.ins() {
            self.input_mut(input).listener = None;
        }
        for stream_id in std::mem::take(&mut self.port_listeners).into_keys() {
            self.port
                .msrp
                .withdraw(since, msrp::attribute::LISTENER, &stream_id.to_be_bytes());
        }
        self.port.leave(since);
        self.talkers.clear();
        self.ready_remotely.clear();
        self.listener_states.clear();
        let mut lite = LiteMode {
            reason,
            sequence: 0,
            talkers: HashMap::new(),
            listeners: HashMap::new(),
            next_refresh: since,
            repeats: Vec::new(),
            leaving_talkers: HashMap::new(),
            leaving_listeners: HashMap::new(),
            refusals: HashMap::new(),
            leaving_refusals: HashMap::new(),
            listening: HashMap::new(),
            shared: HashMap::new(),
            random: u64::from_be_bytes([
                0,
                0,
                self.mac.0[0],
                self.mac.0[1],
                self.mac.0[2],
                self.mac.0[3],
                self.mac.0[4],
                self.mac.0[5],
            ]) | 1,
        };
        lite.next_refresh = lite.refresh_after(since);
        self.lite = Some(lite);
        // Other computers' CVU SRP comes tagged in the media VLAN, which an
        // interface filtering VLANs drops unless it is promiscuous.
        if profile::filters_vlans(&self.interface)
            && let Err(error) = self.avtp.set_promiscuous(true)
        {
            log(format!(
                "{}: cannot hear other computers' CVU SRP, not promiscuous: {error}",
                self.interface
            ));
        }
        // Programs running AVB Lite with the port hear it does, before
        // where their copies go.
        let mode = self.mode();
        self.sharers.tell_mode(mode);
        self.declare_all();
        self.update_lite_status();
        self.steer_profile();
    }

    /// Declares every output's stream, once MAAP gave them addresses, and
    /// the programs' streams; and the listeners here of every stream, ours
    /// and the programs', as its talker allows.
    fn declare_all(&mut self) {
        if self.addressed() {
            for out in self.outs() {
                self.declare_talker(out);
            }
        }
        for stream_id in self.sharers.streams() {
            self.declare_shared(stream_id);
        }
        for input in self.ins() {
            self.update_listener(input);
        }
        for stream_id in self.sharers.listened() {
            self.talker_heard(stream_id);
        }
    }

    /// Withdraws AVB Lite's Listener declarations, one for each stream.
    fn withdraw_lite_listening(&mut self) {
        let listening = self
            .lite
            .as_mut()
            .map(|lite| std::mem::take(&mut lite.listening))
            .unwrap_or_default();
        for (stream_id, (talker, state, entity)) in listening {
            self.send_cvu(
                entity,
                talker,
                &lite::listener_message(stream_id, state, avb_mrp::Event::Lv),
            );
        }
    }

    /// Withdraws the programs' talkers' AVB Lite declarations, which they
    /// keep declared.
    fn withdraw_lite_shared(&mut self) {
        for stream_id in self.sharers.streams() {
            if let Some(declaration) = self.sharers.declaration(stream_id) {
                let entity = self.sharers.owner_entity(stream_id).map(EntityId);
                self.withdraw_shared(entity, declaration);
            }
        }
    }

    /// Leaves AVB Lite as the link comes up again (profile 2.2): CVU SRP's
    /// declarations go, the endpoints declare with MSRP and MVRP as at the
    /// start, and the fallback is armed again.
    fn leave_lite(&mut self) {
        let since = self.elapsed();
        for out in self.outs() {
            self.withdraw_talker(out);
            let output = self.output_mut(out);
            output.talker = None;
            output.lite = LiteTalker::default();
        }
        for input in self.ins() {
            self.input_mut(input).listener = None;
        }
        self.withdraw_lite_listening();
        self.withdraw_lite_shared();
        self.lite = None;
        let _ = self.avtp.set_promiscuous(false);
        self.talkers.clear();
        self.ready_remotely.clear();
        self.listener_states.clear();
        self.fallback = Fallback::new(self.own_clock.0, false, since);
        self.update_ptp4l();
        self.port.join(since);
        // Programs running AVB Lite with the port hear it runs AVB, their
        // streams and listening declared by MSRP again; the others, that
        // they can declare and listen again.
        let mode = self.mode();
        self.sharers.tell_mode(mode);
        self.declare_all();
        for stream_id in self.sharers.streams() {
            self.tell_sharer(stream_id);
        }
        self.sharers.tell_unable(&Notice::Available);
        self.update_lite_status();
        self.steer_profile();
    }

    /// Keeps EEE and PAUSE off the link, as the AVB Lite profile asks (6):
    /// should the interface have them on, as after its driver starts
    /// again, runs the link unit once more, at most every ten minutes;
    /// and says so when they go off.
    fn keep_link_power(&mut self, before: Option<avb_net::LinkPower>) {
        let Some(power) = self.link.and_then(|link| link.power) else {
            return;
        };
        let on = power.eee || power.pause;
        let was_on = before.is_some_and(|before| before.eee || before.pause);
        if !on {
            if was_on {
                log(format!("{}: EEE and PAUSE are off", self.interface));
            }
            return;
        }
        // Not where AVB Lite is off, nor runs for want of a hardware clock.
        if self.config.avb_lite == LiteChoice::Off || !self.hardware_clock {
            return;
        }
        let since = self.elapsed();
        if self.link_fixed_at.is_some_and(|at| since < at + LINK_AGAIN) {
            return;
        }
        self.link_fixed_at = Some(since);
        self.link_own_until = self.lite.is_some().then_some(since + LINK_OWN);
        let what = match (power.eee, power.pause) {
            (true, true) => "EEE and PAUSE are",
            (true, false) => "EEE is",
            _ => "PAUSE is",
        };
        let unit = profile::link_unit(&self.interface);
        log(format!(
            "{}: {what} on, which AVB Lite asks to be off; running {unit}",
            self.interface
        ));
        let interface = self.interface.clone();
        let _ = std::thread::Builder::new()
            .name("link unit".into())
            .spawn(move || {
                if let Err(error) = profile::apply_link(&interface) {
                    log(format!(
                        "{interface}: cannot run {unit} ({error}); install triib's units and ethtool, or turn EEE and PAUSE off with ethtool"
                    ));
                    return;
                }
                std::thread::sleep(LINK_SETTLE);
                if profile::power(&interface).is_some_and(|power| power.eee || power.pause) {
                    log(format!(
                        "{interface}: EEE or PAUSE is still on after {unit}; is ethtool installed, and does the interface let them go off?"
                    ));
                }
            });
    }

    /// Restarts the interface when the PTP frames others send keep the
    /// driver's receive time stamp, as atlantic's do after the link
    /// renegotiates, until the interface goes down and up: ptp4l drops
    /// every one of them. At most every ten minutes, should a restart not
    /// help.
    fn watch_stamps(&mut self, message: &[u8]) {
        if !lite::stamped(message) {
            self.stamped_run = 0;
            return;
        }
        self.stamped_run += 1;
        if self.stamped_run != STAMPED_RUN {
            return;
        }
        let since = self.elapsed();
        if self.link_reset_at.is_some_and(|at| since < at + LINK_AGAIN) {
            return;
        }
        self.link_reset_at = Some(since);
        self.link_own_until = self.lite.is_some().then_some(since + LINK_OWN);
        let unit = profile::reset_unit(&self.interface);
        log(format!(
            "{}: PTP frames arrive with the driver's time stamps on them, which ptp4l drops; running {unit}",
            self.interface
        ));
        let interface = self.interface.clone();
        let _ = std::thread::Builder::new()
            .name("link reset".into())
            .spawn(move || {
                if let Err(error) = profile::reset_link(&interface) {
                    log(format!(
                        "{interface}: cannot run {unit} ({error}); install triib's units, or take {interface} down and up"
                    ));
                }
            });
    }

    /// Tells the fallback what runs PTP on the interface: what ptp4l
    /// answers, else what triib's unit runs, as ptp4l does not answer
    /// while its unit starts it.
    fn update_ptp4l(&mut self) {
        let since = self.elapsed();
        let ptp4l = match (&self.ptp_status, self.link.and_then(|link| link.unit)) {
            (Some(status), _) if status.end_to_end => Ptp4l::Lite,
            (Some(_), _) => Ptp4l::Gptp,
            (None, Some(Profile::Gptp)) => Ptp4l::Gptp,
            (None, Some(Profile::Lite)) => Ptp4l::Lite,
            (None, None) => Ptp4l::Absent,
        };
        self.fallback.set_ptp4l(since, ptp4l);
    }

    /// Starts triib's ptp4l unit on the interface where no ptp4l answers
    /// for it and none of triib's units runs one, once ptp4l has had time to
    /// answer, so installing triib is all the network's time takes. A ptp4l
    /// run another way on the interface stays as it is.
    fn start_ptp4l(&mut self) {
        let since = self.elapsed();
        if self.ptp_status.is_some()
            || since < PTP4L_WAIT
            || self
                .ptp4l_tried_at
                .is_some_and(|at| since < at + PTP4L_AGAIN)
        {
            return;
        }
        let Some(link) = self.link else {
            return;
        };
        if link.unit.is_some() || link.carrier == Some(false) {
            return;
        }
        self.ptp4l_tried_at = Some(since);
        let interface = self.interface.clone();
        if profile::ptp4l_on(&interface) {
            log(format!(
                "ptp4l: one runs on {interface} but does not answer on the read-only socket entities.toml names"
            ));
            return;
        }
        if !profile::ptp4l_installed() {
            log(format!(
                "ptp4l: none runs on {interface}, and linuxptp is not installed; install it for the network's time"
            ));
            return;
        }
        let profile = if self.lite.is_some()
            || (self.config.avb_lite == LiteChoice::On && self.hardware_clock)
        {
            Profile::Lite
        } else {
            Profile::Gptp
        };
        let unit = profile.unit(&interface);
        log(format!(
            "ptp4l: none answers for {interface}; starting {unit}"
        ));
        self.switched_at = Some(since);
        let _ = std::thread::Builder::new()
            .name("ptp4l unit".into())
            .spawn(move || {
                if let Err(error) = profile::start(&interface, profile) {
                    log(format!("ptp4l: cannot start {unit}: {error}"));
                }
            });
    }

    /// Restarts triib's unit running ptp4l once its clock has stayed out of
    /// sync for `OFF_RESTART`, as when a driver reset the hardware clock and
    /// ptp4l does not step it: starting, ptp4l does. A ptp4l run another
    /// way stays as it is.
    fn restart_ptp4l(&mut self) {
        let since = self.elapsed();
        let Some(off) = self.off_since else {
            return;
        };
        let Some(profile) = self.link.and_then(|link| link.unit) else {
            return;
        };
        if since < off + OFF_RESTART
            || self
                .off_restarted_at
                .is_some_and(|at| since < at + PTP4L_AGAIN)
        {
            return;
        }
        self.off_restarted_at = Some(since);
        self.off_since = None;
        let interface = self.interface.clone();
        let unit = profile.unit(&interface);
        log(format!(
            "ptp4l: out of sync for {} s; restarting {unit}",
            OFF_RESTART.as_secs()
        ));
        let _ = std::thread::Builder::new()
            .name("ptp4l unit".into())
            .spawn(move || {
                if let Err(error) = profile::restart_ptp4l(&interface, profile) {
                    log(format!("ptp4l: cannot restart {unit}: {error}"));
                }
            });
    }

    /// Starts the ptp4l unit for the profile the endpoints run, where one
    /// of triib's units runs ptp4l on the interface.
    fn steer_profile(&mut self) {
        if self.config.avb_lite == LiteChoice::Off {
            return;
        }
        let Some(unit) = self.link.and_then(|link| link.unit) else {
            return;
        };
        let wanted = if self.lite.is_some() || self.fallback.reason().is_some() {
            Profile::Lite
        } else {
            Profile::Gptp
        };
        let since = self.elapsed();
        self.steer_pending = false;
        if unit == wanted {
            return;
        }
        // From AVB Lite, ptp4l moves to gPTP once one gPTP peer has asked
        // for peer delay without the Endpoint Declaration TLV, as an AVB
        // bridge does every second, and starts while the fallback keeps
        // listening; with no such peer gPTP would have nothing to follow.
        if unit == Profile::Lite && self.fallback.gptp_peers() != 1 {
            self.steer_pending = true;
            return;
        }
        // Too soon after the last switch, the turn makes this one once the
        // wait is over; but back to AVB Lite before the fallback has
        // settled, after a start or the link coming up, does not wait.
        let back = wanted == Profile::Lite && !self.fallback.settled(since);
        if !back && self.switched_at.is_some_and(|at| since < at + SWITCH_AGAIN) {
            self.steer_pending = true;
            return;
        }
        self.switched_at = Some(since);
        log(format!("ptp4l: moving to {wanted}"));
        let interface = self.interface.clone();
        let _ = std::thread::Builder::new()
            .name("ptp4l unit".into())
            .spawn(move || {
                if let Err(error) = profile::start(&interface, wanted) {
                    log(format!(
                        "ptp4l: cannot start {}: {error}",
                        wanted.unit(&interface)
                    ));
                }
            });
    }

    /// Ages out declarations not refreshed, and refreshes ours.
    fn lite_turn(&mut self) {
        let since = self.elapsed();
        let Some(lite) = &mut self.lite else {
            return;
        };
        let stale = |at: &Duration| since.saturating_sub(*at) > AGE_OUT;
        let mut gone_talkers: Vec<u64> = lite
            .talkers
            .iter()
            .filter(|(_, (_, _, at))| stale(at))
            .map(|(stream_id, _)| *stream_id)
            .collect();
        for stream_id in &gone_talkers {
            lite.talkers.remove(stream_id);
        }
        let mut gone_listeners = Vec::new();
        for (stream_id, listeners) in &mut lite.listeners {
            let before = listeners.len();
            listeners.retain(|_, (_, at)| !stale(at));
            if listeners.len() != before {
                gone_listeners.push(*stream_id);
            }
        }
        // Withdrawn declarations not declared again within LeaveTime.
        let left: Vec<u64> = lite
            .leaving_talkers
            .iter()
            .filter(|(_, at)| **at <= since)
            .map(|(stream_id, _)| *stream_id)
            .collect();
        for stream_id in left {
            lite.leaving_talkers.remove(&stream_id);
            if lite.talkers.remove(&stream_id).is_some() {
                gone_talkers.push(stream_id);
            }
        }
        let left: Vec<(u64, MacAddress)> = lite
            .leaving_listeners
            .iter()
            .filter(|(_, at)| **at <= since)
            .map(|(key, _)| *key)
            .collect();
        for (stream_id, listener) in left {
            lite.leaving_listeners.remove(&(stream_id, listener));
            let removed = lite
                .listeners
                .get_mut(&stream_id)
                .and_then(|listeners| listeners.remove(&listener))
                .is_some();
            if removed && !gone_listeners.contains(&stream_id) {
                gone_listeners.push(stream_id);
            }
        }
        let stale_refusals: Vec<u64> = lite
            .refusals
            .iter()
            .filter(|(stream_id, (_, _, at))| {
                stale(at)
                    || lite
                        .leaving_refusals
                        .get(*stream_id)
                        .is_some_and(|at| *at <= since)
            })
            .map(|(stream_id, _)| *stream_id)
            .collect();
        for stream_id in &stale_refusals {
            lite.refusals.remove(stream_id);
            lite.leaving_refusals.remove(stream_id);
        }
        let repeats = std::mem::take(&mut lite.repeats);
        let (due, later): (Vec<_>, Vec<_>) = repeats.into_iter().partition(|(at, ..)| *at <= since);
        lite.repeats = later;
        let refresh = since >= lite.next_refresh;
        if refresh {
            lite.next_refresh = lite.refresh_after(since);
        }
        for (_, entity, destination, message) in due {
            self.send_cvu(entity, destination, &message);
        }
        for stream_id in gone_talkers {
            log(format!("AVB Lite: talker of {stream_id:#018x} is gone"));
            self.lite_talker_changed(stream_id);
        }
        for stream_id in stale_refusals {
            self.lite_talker_changed(stream_id);
        }
        for out in self.outs() {
            if self
                .output(out)
                .lite
                .escalate_at
                .is_some_and(|at| at <= since)
            {
                self.update_talker(out);
            }
        }
        let escalating: Vec<u64> = self.lite.as_ref().map_or_else(Vec::new, |lite| {
            lite.shared
                .iter()
                .filter(|(_, talker)| talker.escalate_at.is_some_and(|at| at <= since))
                .map(|(stream_id, _)| *stream_id)
                .collect()
        });
        for stream_id in escalating {
            self.lite_update_shared(stream_id);
        }
        for stream_id in gone_listeners {
            if let Some(out) = self.own_talker(stream_id) {
                self.update_talker(out);
            } else if self.sharers.owner(stream_id).is_some() {
                self.lite_update_shared(stream_id);
            }
        }
        if refresh {
            for out in self.outs() {
                let entity = self.entity_of(out.0);
                if self.addressed()
                    && let Some(declaration) = self.lite_declaration(out)
                {
                    self.declare_cvu(
                        entity,
                        BROADCAST,
                        lite::talker_message(&declaration, avb_mrp::Event::JoinIn),
                    );
                }
                let refused: Vec<(MacAddress, u8)> = self
                    .output(out)
                    .lite
                    .refused
                    .iter()
                    .map(|(listener, code)| (*listener, *code))
                    .collect();
                for (listener, code) in refused {
                    if let Some(refusal) = self.refusal(out, code) {
                        self.declare_cvu(
                            entity,
                            listener,
                            lite::talker_message(&refusal, avb_mrp::Event::JoinIn),
                        );
                    }
                }
            }
            for stream_id in self.sharers.streams() {
                let Some(declaration) = self.sharers.declaration(stream_id) else {
                    continue;
                };
                let Some(entity) = self.sharers.owner_entity(stream_id).map(EntityId) else {
                    continue;
                };
                let (escalated, refused) = self
                    .lite
                    .as_ref()
                    .and_then(|lite| lite.shared.get(&stream_id))
                    .map_or((false, Vec::new()), |talker| {
                        (
                            talker.escalated,
                            talker
                                .refused
                                .iter()
                                .map(|(listener, code)| (*listener, *code))
                                .collect(),
                        )
                    });
                let declared = self.shared_lite_declaration(declaration, escalated);
                self.declare_cvu(
                    entity,
                    BROADCAST,
                    lite::talker_message(&declared, avb_mrp::Event::JoinIn),
                );
                for (listener, code) in refused {
                    let refusal = self.refused(declared, code);
                    self.declare_cvu(
                        entity,
                        listener,
                        lite::talker_message(&refusal, avb_mrp::Event::JoinIn),
                    );
                }
            }
            let listening: Vec<(u64, (MacAddress, ListenerState, EntityId))> = self
                .lite
                .as_ref()
                .map(|lite| {
                    lite.listening
                        .iter()
                        .map(|(id, sent)| (*id, *sent))
                        .collect()
                })
                .unwrap_or_default();
            for (stream_id, (talker, state, entity)) in listening {
                self.declare_cvu(
                    entity,
                    talker,
                    lite::listener_message(stream_id, state, avb_mrp::Event::JoinIn),
                );
            }
        }
    }

    /// Sends a CVU SRP declaration in entity `sender`'s name to
    /// `destination`, VLAN-tagged in the media VLAN (profile 6, item 5);
    /// our own endpoints take it here, as the computer never hears what it
    /// sends.
    fn send_cvu(
        &mut self,
        sender: EntityId,
        destination: MacAddress,
        message: &avb_mrp::mrpdu::Message,
    ) {
        let Some(lite) = &mut self.lite else {
            return;
        };
        // What goes now supersedes a repeat still to go.
        lite.repeats.retain(|(_, from, to, pending)| {
            !(*from == sender && *to == destination && same_declaration(pending, message))
        });
        lite.sequence = lite.sequence.wrapping_add(1);
        let Some(pdu) = lite::cvu_command(sender, lite.sequence, message) else {
            return;
        };
        let mut frame = Vec::with_capacity(18 + pdu.len());
        frame.extend_from_slice(&destination.0);
        frame.extend_from_slice(&self.mac.0);
        frame.extend_from_slice(&0x8100u16.to_be_bytes());
        frame.extend_from_slice(
            &((u16::from(LITE_PRIORITY) << 13) | self.media_vlan()).to_be_bytes(),
        );
        frame.extend_from_slice(&atdecc::ETHERTYPE_AVTP.to_be_bytes());
        frame.extend_from_slice(&pdu);
        frame.resize(frame.len().max(64), 0);
        let _ = self.tagged.send(&frame);
        // Our own endpoints take it too, as the computer never hears what
        // it sends, but only after the update that sent it.
        if destination == BROADCAST || destination == self.mac {
            self.local_cvu.push_back((destination == BROADCAST, pdu));
        }
    }

    /// Sends a new, changed or refreshed declaration, and the same again
    /// JoinTime later, as an MRP applicant makes sure two go out.
    fn declare_cvu(
        &mut self,
        sender: EntityId,
        destination: MacAddress,
        message: avb_mrp::mrpdu::Message,
    ) {
        self.send_cvu(sender, destination, &message);
        let at = self.elapsed() + JOIN_TIME;
        if let Some(lite) = &mut self.lite {
            lite.repeats.push((at, sender, destination, message));
        }
    }

    /// The entity ID of endpoint `index`, which AVB Lite's declarations
    /// name.
    fn entity_of(&self, index: usize) -> EntityId {
        self.endpoints[index].entity.entity_id()
    }

    /// Takes the declarations of a CVU SRP command `source` sent, to a
    /// group address or to this computer alone. CVU SRP commands are not
    /// answered (profile 6).
    fn cvu_heard(&mut self, source: MacAddress, group: bool, pdu: &[u8]) {
        let Some(declared) = lite::cvu_declarations(pdu) else {
            return;
        };
        let since = self.elapsed();
        for declaration in declared {
            let Some(lite) = &mut self.lite else {
                return;
            };
            match declaration {
                // A talker's declaration to this computer alone refuses
                // one of its listeners, and outranks its broadcast one.
                Declared::Talker(talker) if !group => {
                    lite.leaving_refusals.remove(&talker.stream_id);
                    let changed =
                        lite.refusals
                            .get(&talker.stream_id)
                            .is_none_or(|(known, from, _)| {
                                known.failure != talker.failure || *from != source
                            });
                    lite.refusals
                        .insert(talker.stream_id, (talker, source, since));
                    if changed {
                        self.lite_talker_changed(talker.stream_id);
                    }
                }
                Declared::TalkerGone(stream_id) if !group => {
                    if lite.refusals.contains_key(&stream_id) {
                        lite.leaving_refusals
                            .entry(stream_id)
                            .or_insert(since + LEAVE_TIME);
                    }
                }
                Declared::Talker(talker) => {
                    lite.leaving_talkers.remove(&talker.stream_id);
                    let changed =
                        lite.talkers
                            .get(&talker.stream_id)
                            .is_none_or(|(known, from, _)| {
                                known.destination != talker.destination
                                    || known.failure != talker.failure
                                    || *from != source
                            });
                    lite.talkers
                        .insert(talker.stream_id, (talker, source, since));
                    if changed {
                        self.lite_talker_changed(talker.stream_id);
                    }
                }
                Declared::TalkerGone(stream_id) => {
                    if lite.talkers.contains_key(&stream_id) {
                        lite.leaving_talkers
                            .entry(stream_id)
                            .or_insert(since + LEAVE_TIME);
                    }
                }
                Declared::Listener(stream_id, state) => {
                    lite.leaving_listeners.remove(&(stream_id, source));
                    let listeners = lite.listeners.entry(stream_id).or_default();
                    let changed = listeners
                        .get(&source)
                        .is_none_or(|(known, _)| *known != state);
                    listeners.insert(source, (state, since));
                    if changed {
                        if let Some(out) = self.own_talker(stream_id) {
                            self.update_talker(out);
                        } else if self.sharers.owner(stream_id).is_some() {
                            self.lite_update_shared(stream_id);
                        }
                    }
                }
                Declared::ListenerGone(stream_id) => {
                    let known = lite
                        .listeners
                        .get(&stream_id)
                        .is_some_and(|listeners| listeners.contains_key(&source));
                    if known {
                        lite.leaving_listeners
                            .entry((stream_id, source))
                            .or_insert(since + LEAVE_TIME);
                    }
                }
            }
        }
    }

    /// A talker's declaration came, changed or went: our listeners
    /// settled on its stream follow.
    fn lite_talker_changed(&mut self, stream_id: u64) {
        self.talker_heard(stream_id);
    }

    /// Corrects the media clock for the asymmetry of the grandmaster's and
    /// this computer's link speeds, where ptp4l runs the AVB Lite PTP
    /// profile and the grandmaster announces its speed (profile 5); else
    /// for nothing.
    fn update_correction(&mut self) {
        let wanted = match &self.ptp_status {
            Some(status) if status.end_to_end && self.link_speed > 0 => self
                .grandmaster_links
                .get(&status.gptp.grandmaster.0)
                .map_or(0, |speed| lite::link_asymmetry(*speed, self.link_speed)),
            _ => 0,
        };
        if wanted != self.correction {
            self.correction = wanted;
            self.clock.set_correction(wanted);
            log(format!(
                "PTP: correcting {wanted} ns for the grandmaster's link speed against {} Mb/s here",
                self.link_speed
            ));
        }
    }

    /// The VLAN of AVB Lite's streams and CVU SRP: the configured one, 2
    /// when none is, or 0 for a priority tag only.
    fn media_vlan(&self) -> u16 {
        self.config.media_vlan.unwrap_or(VLAN) & 0x0fff
    }

    /// How many listeners a talker serves unicast.
    fn fanout(&self) -> usize {
        usize::from(self.config.unicast_fanout.unwrap_or(FANOUT).max(1))
    }

    /// The bandwidth the copies of the talkers here take, ours and the
    /// programs', but for stream `except`'s, in bits per second.
    fn committed(&self, except: u64) -> u64 {
        let ours: u64 = self
            .outs()
            .into_iter()
            .filter(|out| {
                self.endpoints[out.0]
                    .entity
                    .output_stream(out.1)
                    .is_some_and(|(stream_id, _)| stream_id.0 != except)
            })
            .filter_map(|out| {
                let copies = self.output(out).lite.copies.len() as u64;
                self.declaration(out)
                    .map(|declaration| copies * declaration.bandwidth(8000))
            })
            .sum();
        let programs: u64 = self.lite.as_ref().map_or(0, |lite| {
            lite.shared
                .iter()
                .filter(|(stream_id, _)| **stream_id != except)
                .filter_map(|(stream_id, talker)| {
                    self.sharers
                        .declaration(*stream_id)
                        .map(|declaration| talker.copies.len() as u64 * declaration.bandwidth(8000))
                })
                .sum()
        });
        ours + programs
    }

    /// A talker's declaration in AVB Lite, its destination where the
    /// stream's frames go now: `00:00:00:00:00:00` while unicast, the
    /// multicast address once escalated (profile 6).
    fn lite_declaration(&self, out: Out) -> Option<TalkerDeclaration> {
        let declaration = self.declaration(out)?;
        Some(TalkerDeclaration {
            destination: if self.output(out).lite.escalated {
                declaration.destination
            } else {
                MacAddress([0; 6])
            },
            ..declaration
        })
    }

    /// A program's talker's declaration in AVB Lite, as ours: where its
    /// frames go now, `00:00:00:00:00:00` while unicast, the multicast
    /// address it declared once escalated; in the media VLAN, at AVB
    /// Lite's priority (profile 6).
    fn shared_lite_declaration(
        &self,
        declaration: TalkerDeclaration,
        escalated: bool,
    ) -> TalkerDeclaration {
        TalkerDeclaration {
            destination: if escalated {
                declaration.destination
            } else {
                MacAddress([0; 6])
            },
            vlan_id: self.media_vlan(),
            priority: LITE_PRIORITY,
            ..declaration
        }
    }

    /// This computer's address as a Talker Failed's Bridge ID, preceded by
    /// a priority of 0 (profile 6, Declaration content).
    fn own_bridge_id(&self) -> u64 {
        let [a, b, c, d, e, f] = self.mac.0;
        u64::from_be_bytes([0, 0, a, b, c, d, e, f])
    }

    /// The Talker Failed of `declaration` with failure `code`, naming this
    /// computer as the system that failed: what a talker here sends a
    /// listener it refuses (profile 6, item 10), and what a stream this
    /// computer's link has no room for comes to (item 6).
    fn refused(&self, declaration: TalkerDeclaration, code: u8) -> TalkerDeclaration {
        TalkerDeclaration {
            failure: Some(TalkerFailure {
                bridge_id: self.own_bridge_id(),
                code,
            }),
            ..declaration
        }
    }

    /// The Talker Failed one of our talkers sends a listener it refuses.
    fn refusal(&self, out: Out, code: u8) -> Option<TalkerDeclaration> {
        Some(self.refused(self.lite_declaration(out)?, code))
    }

    /// Whom a talker here serves in AVB Lite among the listeners that
    /// declared for its stream, within its unicast fan-out and 75% of the
    /// link (profile 6): those it serves keep their place, then those it
    /// refuses, who ask and fail because of it, then newcomers ready for
    /// it. Those it sends to, those it refuses with the failure code each
    /// has, and whether it escalates to multicast for them.
    fn lite_admit(
        &self,
        stream_id: u64,
        declaration: &TalkerDeclaration,
        talker: &LiteTalker,
    ) -> (Vec<MacAddress>, HashMap<MacAddress, u8>, bool) {
        let declared: Vec<(MacAddress, ListenerState)> = self
            .lite
            .as_ref()
            .and_then(|lite| lite.listeners.get(&stream_id))
            .map(|listeners| {
                listeners
                    .iter()
                    .map(|(mac, (state, _))| (*mac, *state))
                    .collect()
            })
            .unwrap_or_default();
        let rank = |mac: &MacAddress| {
            if talker.copies.contains(mac) {
                0
            } else if talker.refused.contains_key(mac) {
                1
            } else {
                2
            }
        };
        let mut wanting: Vec<(MacAddress, ListenerState)> = declared
            .into_iter()
            .filter(|(mac, state)| state.ready() || talker.refused.contains_key(mac))
            .collect();
        wanting.sort_by_key(|(mac, _)| (rank(mac), mac.0));
        let per_copy = declaration.bandwidth(8000).max(1);
        let room = if self.link_speed == 0 {
            usize::MAX
        } else {
            let budget = u64::from(self.link_speed) * 750_000;
            (budget.saturating_sub(self.committed(stream_id)) / per_copy) as usize
        };
        let fanout = self.fanout();
        let escalate = self.config.multicast_escalation && wanting.len() > fanout && room >= 1;
        let limit = if escalate {
            wanting.len()
        } else {
            fanout.min(room)
        };
        let ready: Vec<MacAddress> = wanting
            .iter()
            .take(limit)
            .filter(|(_, state)| state.ready())
            .map(|(mac, _)| *mac)
            .collect();
        let refused: HashMap<MacAddress, u8> = wanting
            .iter()
            .enumerate()
            .skip(limit)
            .map(|(place, (mac, _))| {
                let code = if place < fanout {
                    FAILURE_BANDWIDTH
                } else {
                    FAILURE_RESOURCES
                };
                (*mac, code)
            })
            .collect();
        (ready, refused, escalate)
    }

    /// Sends a talker's stream to the listeners that want it, within its
    /// unicast fan-out and 75% of the link (profile 6): a unicast copy
    /// each, or, where escalation is allowed and they are more, its
    /// multicast address. A listener it cannot serve it refuses alone.
    fn lite_update_talker(&mut self, out: Out) {
        let Some((stream_id, multicast)) = self.endpoints[out.0].entity.output_stream(out.1) else {
            return;
        };
        let Some(declaration) = self.declaration(out) else {
            return;
        };
        let since = self.elapsed();
        let (ready, refused, escalate) =
            self.lite_admit(stream_id.0, &declaration, &self.output(out).lite);
        let (copies, announce) = self
            .output_mut(out)
            .lite
            .step(since, escalate, &ready, multicast);
        let name = self.out_name(out);
        self.endpoints[out.0].entity.set_output_reservation(
            out.1,
            OutputReservation {
                ready_listeners: ready.len().min(usize::from(u16::MAX)) as u16,
                registering: true,
            },
        );
        if copies != self.output(out).lite.copies && !copies.is_empty() {
            log(format!(
                "{name}: AVB Lite, {}",
                if copies == [multicast] {
                    format!("to {multicast} for {} listeners", ready.len())
                } else {
                    let to: Vec<String> = copies.iter().map(ToString::to_string).collect();
                    format!("unicast to {}", to.join(", "))
                }
            ));
        }
        self.output_mut(out).lite.copies.clone_from(&copies);
        let addressed = self.addressed();
        let output = self.output_mut(out);
        if copies.is_empty() {
            if output.talker.take().is_some() {
                log(format!("{name}: no listener ready, stopping"));
            }
        } else if let Some(talker) = &output.talker {
            talker.set_destinations(copies);
        } else if output.redeclare_at.is_none() && addressed {
            log(format!("{name}: a listener is ready, streaming"));
            self.start_talker(out);
        }
        let entity = self.entity_of(out.0);
        let Some(declared) = self.lite_declaration(out) else {
            return;
        };
        if announce {
            self.declare_cvu(
                entity,
                BROADCAST,
                lite::talker_message(&declared, avb_mrp::Event::New),
            );
        }
        let before = std::mem::replace(&mut self.output_mut(out).lite.refused, refused.clone());
        self.lite_refusals(entity, &name, declared, before, &refused);
    }

    /// Sends the listeners a talker here newly refuses, or refuses for
    /// another reason, its Talker Failed, and withdraws it from those it
    /// serves again (profile 6, item 10).
    fn lite_refusals(
        &mut self,
        sender: EntityId,
        name: &str,
        declared: TalkerDeclaration,
        before: HashMap<MacAddress, u8>,
        refused: &HashMap<MacAddress, u8>,
    ) {
        for (listener, code) in refused {
            if before.get(listener) != Some(code) {
                log(format!(
                    "{name}: AVB Lite refuses {listener}, failure code {code}"
                ));
                let refusal = self.refused(declared, *code);
                self.declare_cvu(
                    sender,
                    *listener,
                    lite::talker_message(&refusal, avb_mrp::Event::New),
                );
            }
        }
        for (listener, code) in before {
            if !refused.contains_key(&listener) {
                let refusal = self.refused(declared, code);
                self.send_cvu(
                    sender,
                    listener,
                    &lite::talker_message(&refusal, avb_mrp::Event::Lv),
                );
            }
        }
    }

    /// Sends a program's stream to the listeners that want it in AVB Lite,
    /// admitted with ours: the program hears where to send its copies, and
    /// with which tag, and how its listeners stand, refused or not.
    fn lite_update_shared(&mut self, stream_id: u64) {
        let Some(declaration) = self.sharers.declaration(stream_id) else {
            return;
        };
        let Some(entity) = self.sharers.owner_entity(stream_id).map(EntityId) else {
            return;
        };
        let Some(mut talker) = self
            .lite
            .as_mut()
            .map(|lite| lite.shared.remove(&stream_id).unwrap_or_default())
        else {
            return;
        };
        let since = self.elapsed();
        let (ready, refused, escalate) = self.lite_admit(stream_id, &declaration, &talker);
        let (copies, announce) = talker.step(since, escalate, &ready, declaration.destination);
        talker.copies.clone_from(&copies);
        let before = std::mem::replace(&mut talker.refused, refused.clone());
        let escalated = talker.escalated;
        if let Some(lite) = &mut self.lite {
            lite.shared.insert(stream_id, talker);
        }
        let tag = (u16::from(LITE_PRIORITY) << 13) | self.media_vlan();
        self.sharers.tell_copies(stream_id, tag, copies);
        let state = match (ready.is_empty(), refused.is_empty()) {
            (false, true) => Some(ListenerState::Ready),
            (false, false) => Some(ListenerState::ReadyFailed),
            (true, false) => Some(ListenerState::AskingFailed),
            (true, true) => None,
        };
        self.sharers.tell_listener(stream_id, state);
        let declared = self.shared_lite_declaration(declaration, escalated);
        if announce {
            self.declare_cvu(
                entity,
                BROADCAST,
                lite::talker_message(&declared, avb_mrp::Event::New),
            );
        }
        let name = format!("Stream {stream_id:#018x} of a program here");
        self.lite_refusals(entity, &name, declared, before, &refused);
    }

    /// Whether this computer's link takes a stream of `declaration` on
    /// top of the streams from other computers its listeners are ready
    /// for, within 75% (profile 6, item 6).
    fn ingress_fits(&self, stream_id: u64, declaration: &TalkerDeclaration) -> bool {
        if self.link_speed == 0 {
            return true;
        }
        let Some(lite) = &self.lite else {
            return true;
        };
        let taken: u64 = lite
            .listening
            .iter()
            .filter(|(other, (talker, state, _))| {
                **other != stream_id && *talker != self.mac && state.ready()
            })
            .filter_map(|(other, _)| lite.talkers.get(other))
            .map(|(declaration, _, _)| declaration.bandwidth(8000))
            .sum();
        taken + declaration.bandwidth(8000) <= u64::from(self.link_speed) * 750_000
    }

    /// In whose name to declare a stream's listeners here in AVB Lite:
    /// one of our entities settled on it, else a program's listening to
    /// it.
    fn listening_entity(&self, stream_id: u64) -> Option<EntityId> {
        self.settled_on(stream_id)
            .first()
            .map(|input| self.entity_of(input.0))
            .or_else(|| self.sharers.listening_entity(stream_id).map(EntityId))
    }

    /// A stream's talker declaration in AVB Lite: its CVU SRP one, a
    /// Talker Failed sent to this computer alone outranking its broadcast
    /// one (profile 6, item 10), and failed where this computer's link has
    /// no room for the stream (item 6). Its listeners here, ours and the
    /// programs', make one Listener declaration to the talker for it,
    /// unicast, in the name of one of their entities.
    fn lite_listen(&mut self, stream_id: u64) -> Option<TalkerDeclaration> {
        let lite = self.lite.as_ref()?;
        let heard = lite
            .refusals
            .get(&stream_id)
            .or_else(|| lite.talkers.get(&stream_id))
            .map(|(declaration, from, _)| (*declaration, *from));
        let before = lite.listening.get(&stream_id).copied();
        let full = heard.is_some_and(|(declaration, from)| {
            declaration.failure.is_none()
                && from != self.mac
                && !self.ingress_fits(stream_id, &declaration)
        });
        let wanted =
            heard
                .zip(self.listening_entity(stream_id))
                .map(|((declaration, from), entity)| {
                    let state = if declaration.failure.is_some() || full {
                        ListenerState::AskingFailed
                    } else {
                        ListenerState::Ready
                    };
                    (from, state, entity)
                });
        let state = |sent: Option<(MacAddress, ListenerState, EntityId)>| {
            sent.map(|(talker, state, _)| (talker, state))
        };
        if full && state(wanted) != state(before) {
            log(format!(
                "AVB Lite: the link has no room for {stream_id:#018x}"
            ));
        }
        if let Some((talker, known, entity)) = before
            && wanted.map(|(to, ..)| to) != Some(talker)
        {
            self.send_cvu(
                entity,
                talker,
                &lite::listener_message(stream_id, known, avb_mrp::Event::Lv),
            );
        }
        // Another entity of ours declaring alone sends nothing new: the
        // talker knows this computer by its address.
        if let Some(lite) = &mut self.lite {
            match wanted {
                Some(sent) => lite.listening.insert(stream_id, sent),
                None => lite.listening.remove(&stream_id),
            };
        }
        if let Some((talker, known, entity)) = wanted
            && state(wanted) != state(before)
        {
            self.declare_cvu(
                entity,
                talker,
                lite::listener_message(stream_id, known, avb_mrp::Event::New),
            );
        }
        heard.map(|(declaration, _)| {
            if full {
                self.refused(declaration, FAILURE_BANDWIDTH)
            } else {
                declaration
            }
        })
    }

    /// What each entity's GET_LITE_STATUS answers.
    fn update_lite_status(&mut self) {
        let ptp = self.ptp_status.as_ref();
        let offset = ptp.and_then(|status| status.offset);
        let gptp_runs = ptp.is_some_and(|status| !status.end_to_end);
        // AVB Lite needs a hardware clock: without one an interface is not
        // capable of it.
        let mut flags = if self.hardware_clock {
            LiteFlags::CAPABLE | LiteFlags::EGRESS_VALID
        } else {
            LiteFlags::empty()
        };
        if self.lite.is_some() {
            flags |= LiteFlags::ACTIVE;
        }
        if self.config.multicast_escalation {
            flags |= LiteFlags::ESCALATION_ALLOWED;
        }
        if offset.is_some() {
            flags |= LiteFlags::OFFSET_VALID;
        }
        for index in 0..self.endpoints.len() {
            let talker = !self.endpoints[index].outputs.is_empty();
            let egress: u64 = (0..self.endpoints[index].outputs.len() as u16)
                .map(|at| {
                    let out = Out(index, at);
                    let copies = self.output(out).lite.copies.len() as u64;
                    self.declaration(out)
                        .map_or(0, |declaration| copies * declaration.bandwidth(8000) / 1000)
                })
                .sum();
            let status = LiteStatus {
                interface: 0,
                flags,
                fallback_reason: self
                    .lite
                    .as_ref()
                    .map_or(FallbackReason::NONE, |lite| lite.reason),
                // ptp4l still in gPTP keeps the profile gPTP, fallen
                // back or not.
                ptp_profile: if self.lite.is_some() && !gptp_runs {
                    PtpProfile::AVB_LITE_PTP
                } else {
                    PtpProfile::GPTP
                },
                ptp_domain: ptp.map_or(0, |status| status.gptp.domain),
                media_vlan_id: self.media_vlan(),
                unicast_fanout_limit: if talker { self.fanout() as u8 } else { 0 },
                link_speed: self.link_speed,
                committed_egress: egress.min(u64::from(u32::MAX)) as u32,
                grandmaster: ptp.map_or(ClockIdentity(0), |status| status.gptp.grandmaster),
                offset_from_grandmaster: offset
                    .unwrap_or(0)
                    .clamp(i64::from(i32::MIN), i64::from(i32::MAX))
                    as i32,
            };
            self.endpoints[index].entity.set_lite_status(status);
        }
    }
}

/// What each of a device's streams is doing, in the configured order.
fn stream_statuses(endpoint: &Endpoint) -> Vec<StreamStatus> {
    let model = endpoint.entity.model();
    endpoint
        .streams
        .iter()
        .enumerate()
        .map(|(place, stream)| {
            let output = endpoint.out_streams.iter().position(|at| *at == place);
            let input = endpoint.in_streams.iter().position(|at| *at == place);
            let (name, state) = match (output, input) {
                (Some(at), _) => (
                    model.outputs.get(at).map(|stream| stream.name.clone()),
                    if endpoint.outputs[at].talker.is_some() {
                        "streaming"
                    } else {
                        "waiting"
                    },
                ),
                (_, Some(at)) => (
                    model.inputs.get(at).map(|stream| stream.name.clone()),
                    if endpoint.inputs[at].listener.is_some() {
                        "listening"
                    } else if endpoint.entity.input_binding(at as u16).is_some() {
                        "bound"
                    } else {
                        "unbound"
                    },
                ),
                _ => (None, "waiting"),
            };
            StreamStatus {
                direction: stream.direction,
                clock: stream.clock,
                name: name.unwrap_or_default(),
                channels: stream.channels,
                audio: stream
                    .audio
                    .clone()
                    .unwrap_or_else(|| crate::config::VIRTUAL.into()),
                state: state.into(),
            }
        })
        .collect()
}

fn is_talker(attribute_type: u8) -> bool {
    matches!(
        attribute_type,
        msrp::attribute::TALKER_ADVERTISE | msrp::attribute::TALKER_FAILED
    )
}

/// The stream ID a talker or listener value starts with.
fn stream_of(value: &[u8]) -> u64 {
    let mut octets = [0; 8];
    let length = value.len().min(8);
    octets[..length].copy_from_slice(&value[..length]);
    u64::from_be_bytes(octets)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A format chosen at some channels carries over to others in its
    /// packing, AM824 with "or fewer" set or not, but not to another
    /// rate, bit depth or frame rate.
    #[test]
    fn a_format_carries_over_to_other_channels() {
        let milan =
            |rate, channels, bits| formats::aaf(rate, channels, bits, formats::CLASS_A_FRAMES);
        let wireless = formats::aaf(48_000, 8, 32, formats::CLASS_B_FRAMES);
        // AM824 as triib offered it before, "or fewer" clear.
        let exact_am824 = StreamFormat(0x00a0_0208_4000_0800);
        assert!(same_packing(
            formats::am824(48_000, 2),
            formats::am824(48_000, 8)
        ));
        assert!(same_packing(exact_am824, formats::am824(48_000, 2)));
        assert!(!same_packing(exact_am824, formats::am824(96_000, 8)));
        assert!(same_packing(milan(48_000, 2, 24), milan(48_000, 8, 24)));
        assert!(!same_packing(milan(48_000, 2, 32), milan(48_000, 2, 24)));
        assert!(!same_packing(
            milan(48_000, 8, 32),
            formats::am824(48_000, 8)
        ));
        assert!(!same_packing(milan(48_000, 8, 32), milan(96_000, 8, 32)));
        assert!(!same_packing(milan(48_000, 8, 32), wireless));
    }

    /// A stream offers the formats every program on avb's crates offers,
    /// each size at every rate where it fits, so an entity of many
    /// channels offers the higher rates too, at the smaller sizes.
    #[test]
    fn many_channels_still_offer_every_rate() {
        let device = endpoint(vec![stream(Direction::Out, 60), stream(Direction::In, 60)]);
        let model = device.entity.model();
        assert_eq!(model.outputs[0].formats, formats::offered(60, false));
        assert_eq!(model.inputs[0].formats, formats::offered(60, false));
        assert_eq!(model.sampling_rates, [48_000, 96_000, 192_000]);
        assert_eq!(model.current_sampling_rate, 48_000);
    }

    /// A stream joining an entity at a rate where its own channels take
    /// more than a frame starts at the most channels of its packing that
    /// fit there, as a change of rate would move it.
    #[test]
    fn a_stream_too_large_for_the_rate_takes_fewer_channels() {
        let at_192 = formats::aaf(192_000, 8, 32, formats::CLASS_A_FRAMES);
        let device = endpoint(vec![
            StreamConfig {
                format: Some(format!("{:#018x}", at_192.0)),
                ..stream(Direction::Out, 8)
            },
            stream(Direction::In, 60),
        ]);
        let model = device.entity.model();
        assert_eq!(model.current_sampling_rate, 192_000);
        assert_eq!(model.outputs[0].current_format, at_192);
        assert_eq!(model.inputs[0].current_format, at_192);
    }

    /// A stream keeps the format a controller chose of the profile's other
    /// sizes, and one chosen at other channels than its own, as when they
    /// changed, comes back as its packing at its own; AM824 chosen as
    /// triib offered it before comes back as offered now.
    #[test]
    fn a_chosen_format_comes_back() {
        let milan = |channels, bits| formats::aaf(48_000, channels, bits, formats::CLASS_A_FRAMES);
        let chosen = |channels, format: StreamFormat| {
            let device = endpoint(vec![StreamConfig {
                format: Some(format!("{:#018x}", format.0)),
                ..stream(Direction::In, channels)
            }]);
            device.entity.model().inputs[0].current_format
        };
        assert_eq!(chosen(8, milan(2, 24)), milan(2, 24));
        assert_eq!(chosen(2, milan(8, 32)), milan(8, 32));
        assert_eq!(chosen(6, milan(4, 24)), milan(6, 24));
        let exact_am824 = StreamFormat(0x00a0_0208_4000_0800);
        assert_eq!(chosen(8, exact_am824), formats::am824(48_000, 8));
    }

    /// A class B talker offers class B's formats first, so a frame each
    /// class B interval is its default; others start in class A's.
    #[test]
    fn a_class_b_talker_defaults_to_half_the_frames() {
        let device = endpoint(vec![
            StreamConfig {
                class: SrClass::B,
                ..stream(Direction::Out, 8)
            },
            stream(Direction::Out, 8),
        ]);
        let model = device.entity.model();
        assert_eq!(model.outputs[0].formats, formats::offered(8, true));
        assert_eq!(
            model.outputs[0].current_format,
            formats::aaf(48_000, 8, 32, formats::CLASS_B_FRAMES)
        );
        assert_eq!(
            model.outputs[1].current_format,
            formats::aaf(48_000, 8, 32, formats::CLASS_A_FRAMES)
        );
    }

    /// Every declaration goes out again from LeaveAllTime to half as much
    /// again later, at a different point each time.
    #[test]
    fn declarations_refresh_every_10_to_15_s() {
        let mut lite = LiteMode {
            reason: FallbackReason::CONFIGURED,
            sequence: 0,
            talkers: HashMap::new(),
            listeners: HashMap::new(),
            next_refresh: Duration::ZERO,
            repeats: Vec::new(),
            leaving_talkers: HashMap::new(),
            leaving_listeners: HashMap::new(),
            refusals: HashMap::new(),
            leaving_refusals: HashMap::new(),
            listening: HashMap::new(),
            shared: HashMap::new(),
            random: 0xf0a7_31f4_0f14 | 1,
        };
        let since = Duration::from_secs(100);
        let intervals: Vec<Duration> = (0..50).map(|_| lite.refresh_after(since) - since).collect();
        for interval in &intervals {
            assert!(*interval >= LEAVE_ALL_TIME && *interval < LEAVE_ALL_TIME * 3 / 2);
        }
        assert!(intervals.windows(2).any(|pair| pair[0] != pair[1]));
    }

    /// A talker here escalating declares its multicast address first and
    /// moves its frames there JoinTime later; going back, its frames move
    /// first, and its declaration with them (profile 6).
    #[test]
    fn escalating_declares_before_the_frames_move() {
        let one = MacAddress([0xe8, 0xf6, 0x0a, 0xe0, 0x92, 0x20]);
        let two = MacAddress([0xe8, 0xf6, 0x0a, 0xe0, 0x97, 0x21]);
        let multicast = MacAddress([0x91, 0xe0, 0xf0, 0x00, 0x63, 0x26]);
        let mut talker = LiteTalker::default();
        let start = Duration::from_secs(10);
        assert_eq!(
            talker.step(start, false, &[one], multicast),
            (vec![one], false)
        );
        talker.copies = vec![one];
        assert_eq!(
            talker.step(start, true, &[one, two], multicast),
            (vec![one], true)
        );
        assert_eq!(
            talker.step(start + JOIN_TIME / 2, true, &[one, two], multicast),
            (vec![one], false)
        );
        assert_eq!(
            talker.step(start + JOIN_TIME, true, &[one, two], multicast),
            (vec![multicast], false)
        );
        talker.copies = vec![multicast];
        assert_eq!(
            talker.step(start + JOIN_TIME * 2, false, &[one], multicast),
            (vec![one], true)
        );
        assert!(!talker.escalated);
    }

    /// A repeat still to go is superseded by what goes for the same
    /// stream's declaration of the same kind, not by another's.
    #[test]
    fn a_new_declaration_supersedes_its_repeat() {
        let ready = lite::listener_message(7, ListenerState::Ready, avb_mrp::Event::New);
        let gone = lite::listener_message(7, ListenerState::Ready, avb_mrp::Event::Lv);
        let other = lite::listener_message(8, ListenerState::Ready, avb_mrp::Event::New);
        assert!(same_declaration(&ready, &gone));
        assert!(!same_declaration(&ready, &other));
    }

    #[test]
    fn ids_stay_apart_from_the_apps() {
        let mac = MacAddress([0xf0, 0xa7, 0x31, 0xf4, 0x0f, 0x14]);
        assert_eq!(entity_id(mac, 0), EntityId(0xf0a7_31ff_00f4_0f14));
        assert_eq!(output_stream(mac, 2, 0), StreamId(0xf0a7_31f4_0f14_0200));
    }

    fn endpoint(streams: Vec<StreamConfig>) -> Endpoint {
        let config = EndpointConfig {
            streams,
            ..EndpointConfig::new("Studio", Some(4))
        };
        let mac = MacAddress([0xf0, 0xa7, 0x31, 0xf4, 0x0f, 0x14]);
        Endpoint::new(config, 4, mac, "enp2s0", ClockIdentity(1))
    }

    fn stream(direction: Direction, channels: u16) -> StreamConfig {
        StreamConfig {
            channels,
            ..StreamConfig::new(direction)
        }
    }

    /// A device has its streams in the order listed, each with its own
    /// channels: streams named by number where they have no name, an
    /// output's stream ID from its index among the outputs, and a model of
    /// its own for its streams.
    #[test]
    fn a_device_has_streams_of_its_own() {
        let device = endpoint(vec![
            stream(Direction::Out, 8),
            stream(Direction::In, 2),
            StreamConfig {
                name: "Mix".into(),
                ..stream(Direction::Out, 2)
            },
        ]);
        let model = device.entity.model();
        assert_eq!(model.model_name, "triib entity");
        let names: Vec<&str> = model
            .outputs
            .iter()
            .chain(&model.inputs)
            .map(|stream| stream.name.as_str())
            .collect();
        assert_eq!(names, ["Out 1", "Mix", "In 1"]);
        let channels: Vec<u16> = model.outputs.iter().map(|stream| stream.channels).collect();
        assert_eq!(channels, [8, 2]);
        assert_eq!((device.outputs.len(), device.inputs.len()), (2, 1));
        assert_eq!(device.out_config(1).name, "Mix");
        assert_eq!(
            device.entity.output_stream(1).map(|(stream, _)| stream),
            Some(StreamId(0xf0a7_31f4_0f14_0401))
        );
        assert_eq!(device.stream_name(false, 1), "Studio Mix");
        // Its model is its streams': other channels, another model; the
        // same streams, the same.
        let id = model.entity_model_id;
        assert_eq!(id.0 >> 24, DEVICE_MODELS >> 24);
        let other = endpoint(vec![stream(Direction::Out, 8), stream(Direction::In, 8)]);
        assert_ne!(other.entity.model().entity_model_id, id);
        let again = endpoint(vec![
            stream(Direction::Out, 8),
            stream(Direction::In, 2),
            stream(Direction::Out, 2),
        ]);
        assert_eq!(again.entity.model().entity_model_id, id);
    }

    /// A device's media clock streams are CRF, named apart from its audio
    /// streams, without channels or a virtual device, and leave the
    /// rates to the audio.
    #[test]
    fn media_clock_streams_carry_crf_alone() {
        let device = endpoint(vec![
            StreamConfig::clock(Direction::Out),
            stream(Direction::Out, 2),
            StreamConfig::clock(Direction::In),
        ]);
        let model = device.entity.model();
        let names: Vec<&str> = model
            .outputs
            .iter()
            .chain(&model.inputs)
            .map(|stream| stream.name.as_str())
            .collect();
        assert_eq!(names, ["Media Clock Out 1", "Out 1", "Media Clock In 1"]);
        assert_eq!(model.outputs[0].current_format, MEDIA_CLOCK);
        assert_eq!(model.outputs[0].formats, [MEDIA_CLOCK]);
        assert_eq!(model.outputs[0].channels, 0);
        assert_eq!(model.inputs[0].current_format, MEDIA_CLOCK);
        assert_eq!(model.sampling_rates, [48_000, 96_000, 192_000]);
        assert!(!device.out_config(0).is_virtual());
        assert!(device.out_config(1).is_virtual());
        // Its talker takes no more than its frames' share.
        let media = avb_stream::media::Media::of(MEDIA_CLOCK).unwrap();
        assert_eq!(media.pdu_length(), 28);
        // Media clocks alone run at the rate they count.
        let clocks = endpoint(vec![StreamConfig::clock(Direction::Out)]);
        assert_eq!(clocks.entity.model().sampling_rates, [48_000]);
    }

    /// An entity's inputs are its clock sources after the internal clock,
    /// media clock and audio alike; the one kept as its source is the
    /// source it starts on, and its talkers follow that input's clock.
    #[test]
    fn its_inputs_are_clock_sources() {
        let device = endpoint(vec![
            stream(Direction::In, 2),
            StreamConfig::clock(Direction::In),
            stream(Direction::Out, 2),
            StreamConfig {
                clock_source: true,
                ..StreamConfig::clock(Direction::In)
            },
        ]);
        let model = device.entity.model();
        assert_eq!(model.clock_source_inputs, [0, 1, 2]);
        assert_eq!(model.clock_source, 3, "the second media clock input");
        assert_eq!(model.clock_source_input(3), Some(2));
        assert_eq!(model.clock_source_input(0), None);
        assert_eq!(device.clock_source_name(), "Studio Media Clock In 2");
        assert_eq!(device.followed(), Some(device.inputs[2].recovered.clone()));
        // An audio input, kept as the source.
        let audio = endpoint(vec![
            stream(Direction::Out, 2),
            StreamConfig {
                clock_source: true,
                ..stream(Direction::In, 2)
            },
        ]);
        assert_eq!(audio.entity.model().clock_source, 1);
        assert_eq!(audio.clock_source_name(), "Studio In 1");
        assert_eq!(audio.followed(), Some(audio.inputs[0].recovered.clone()));
        // Without one kept, the internal clock, which nothing follows.
        let plain = endpoint(vec![
            StreamConfig::clock(Direction::In),
            stream(Direction::Out, 2),
        ]);
        assert_eq!(plain.entity.model().clock_source, 0);
        assert_eq!(plain.followed(), None);
        assert_eq!(plain.clock_source_name(), "the internal clock");
    }

    /// A stream added to an entity leaves the others running as before,
    /// so they go on through the handover; one whose channels changed
    /// does not. A stream added runs at the entity's rate.
    #[test]
    fn streams_that_run_as_before_carry_over() {
        let at_192k = |stream: StreamConfig| StreamConfig {
            format: Some("0x0209021802018000".into()),
            ..stream
        };
        let before = endpoint(vec![
            at_192k(stream(Direction::Out, 8)),
            stream(Direction::In, 8),
        ]);
        assert_eq!(before.entity.model().current_sampling_rate, 192_000);
        assert_eq!(
            before.entity.model().inputs[0].current_format.sample_rate(),
            Some(192_000),
            "the input follows the entity's rate"
        );
        let added = endpoint(vec![
            at_192k(stream(Direction::Out, 8)),
            stream(Direction::In, 8),
            stream(Direction::Out, 2),
        ]);
        assert!(added.runs_as(&before, false, 0));
        assert!(added.runs_as(&before, true, 0));
        assert!(!added.runs_as(&before, false, 1), "the new one");
        assert_eq!(
            added.entity.model().outputs[1].current_format.sample_rate(),
            Some(192_000)
        );
        let changed = endpoint(vec![
            at_192k(stream(Direction::Out, 4)),
            stream(Direction::In, 8),
        ]);
        assert!(!changed.runs_as(&before, false, 0));
        assert!(changed.runs_as(&before, true, 0));
    }

    /// Mappings a controller gave an input, kept in the configuration,
    /// come back as the entity starts; written and read again, they stay.
    #[test]
    fn kept_mappings_come_back() {
        let config = EndpointConfig {
            streams: vec![
                stream(Direction::Out, 2),
                StreamConfig {
                    mappings: Some(crate::config::mappings_text([(1, 0), (0, 1)], 2)),
                    ..stream(Direction::In, 2)
                },
            ],
            ..EndpointConfig::new("Studio", Some(4))
        };
        let text = toml::to_string_pretty(&Config {
            endpoints: vec![config.clone()],
            ..Config::default()
        })
        .expect("writes");
        assert!(text.contains("mappings = \"2 1\""), "{text}");
        let read: Config = toml::from_str(&text).expect("reads");
        assert_eq!(read.endpoints[0], config);
        let mac = MacAddress([0xf0, 0xa7, 0x31, 0xf4, 0x0f, 0x14]);
        let mut entity = Endpoint::new(config, 4, mac, "enp2s0", ClockIdentity(1));
        assert!(entity.entity.input_mappings_are_default(0));
        entity.begin(Instant::from_nanos(0));
        assert!(!entity.entity.input_mappings_are_default(0));
        let restored: Vec<(u16, u16)> = entity
            .entity
            .input_mappings(0)
            .unwrap()
            .iter()
            .map(|mapping| (mapping.stream_channel, mapping.cluster_offset))
            .collect();
        assert_eq!(restored, [(0, 1), (1, 0)]);
    }

    /// A max transit time a controller set, kept in the configuration,
    /// comes back as the endpoint starts: a talker's in its entry, a
    /// device's in its stream's. Without one, the class's default stays.
    #[test]
    fn a_kept_max_transit_time_comes_back() {
        let mut device = endpoint(vec![
            stream(Direction::Out, 8),
            StreamConfig {
                transit: Some(1_000_000),
                ..stream(Direction::Out, 2)
            },
        ]);
        device.begin(Instant::from_nanos(0));
        assert_eq!(device.entity.max_transit_time(0), Some(2_000_000));
        assert!(device.entity.max_transit_time_is_default(0));
        assert_eq!(device.entity.max_transit_time(1), Some(1_000_000));
    }

    /// Its models tell this daemon's devices from the app's controller and
    /// anyone else's.
    #[test]
    fn its_models_tell_its_devices() {
        assert!(is_device_model(EntityModelId(DEVICE_MODELS | 0x6d_5b25)));
        assert!(!is_device_model(EntityModelId(0x8c1f_6436_c000_0001)));
        assert!(!is_device_model(EntityModelId(0x0000_0074_6869_6e67)));
        // triib 1.0's talkers' and listeners'.
        assert!(!is_device_model(EntityModelId(0x8c1f_6436_c000_0002)));
    }

    /// A stream of 1.0's talker, read as a device's, keeps its name, its
    /// audio and its class, which its declaration takes.
    #[test]
    fn a_talker_of_old_is_a_device_of_one_stream() {
        let config: Config = toml::from_str(
            "[[endpoint]]\nkind = \"talker\"\nname = \"Studio\"\nsource = \"tone\"\nclass = \"B\"\n",
        )
        .expect("parses");
        let mac = MacAddress([0xf0, 0xa7, 0x31, 0xf4, 0x0f, 0x14]);
        let talker = Endpoint::new(
            config.endpoints[0].clone(),
            4,
            mac,
            "enp2s0",
            ClockIdentity(1),
        );
        let model = talker.entity.model();
        assert!(is_device_model(model.entity_model_id));
        assert_eq!((model.outputs.len(), model.inputs.len()), (1, 0));
        assert_eq!(model.outputs[0].name, "Studio");
        assert!(model.outputs[0].class_b());
        assert_eq!(talker.stream_name(false, 0), "Studio");
        assert_eq!(
            talker.out_config(0).source(),
            Source::Tone {
                hertz: 1000.0,
                level: 0.1
            }
        );
    }
}
