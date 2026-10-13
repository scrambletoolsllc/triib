//! An ATDECC talker or listener entity (IEEE 1722.1-2021 with Milan 1.3):
//! it advertises itself with ADP, answers the AEM commands controllers
//! send, and connects its streams with Milan's ACMP, a listener probing
//! the talker it is bound to.
//!
//! Like the controller it does no I/O: the caller hands it frames and the
//! time, takes back frames to send and events, and tells it what it knows
//! of gPTP and stream reservation, which other parts of the program run.

pub mod model;

use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;
use core::time::Duration;

use avb_net::MacAddress;

use crate::ADP_ACMP_MULTICAST;
use crate::acmp::{AcmpFlags, AcmpMessageType, AcmpStatus, Acmpdu};
use crate::adp::{
    AdpMessageType, Adpdu, EntityCapabilities, ListenerCapabilities, TalkerCapabilities,
};
use crate::aecp::{
    AecpHeader, AecpMessageType, AemCommandType, AemPdu, AemStatus, VendorUniquePdu,
};
use crate::aem::NAME_LENGTH;
use crate::aem::{AudioMapping, AvbInfoFlags, StreamInfoFlags};
use crate::avtp::{read_u16, read_u32, read_u64};
use crate::descriptor::DescriptorType;
use crate::id::{ClockIdentity, EntityId, StreamId};
use crate::lite::{LiteCommandType, LiteConfigFlags, LiteFlags, LiteStatus, STATUS_PROTOCOL_ID};
use crate::mvu::{MVU_PROTOCOL_ID, MilanFeatures, MvuCommandType, MvuStatus};
use crate::pdu::{self, Pdu};
use crate::stream_format::StreamFormat;
use crate::time::Instant;

pub use model::{EndpointModel, StreamModel};
use model::{EntityState, name_field};

/// How long the advertisement stays valid, in units of 2 s.
const VALID_TIME: u8 = 31;
/// How often ENTITY_AVAILABLE goes out, well inside the valid time.
const ADVERTISE_INTERVAL: Duration = Duration::from_secs(10);
/// The longest wait before answering ENTITY_DISCOVER.
const DISCOVER_DELAY: Duration = Duration::from_millis(500);
/// How long a listener waits for its talker's PROBE_TX_RESPONSE, and
/// between probes once it has given up waiting (Milan 1.3, 5.5.3).
const PROBE_TIMEOUT: Duration = Duration::from_millis(200);
/// An output's destination before it has one, as while MAAP claims it.
pub const UNADDRESSED: MacAddress = MacAddress([0; 6]);
const PROBE_RETRY: Duration = Duration::from_secs(4);
/// How long a lock lasts without being renewed (Milan 1.3, 5.4.2.2).
const LOCK_TIME: Duration = Duration::from_secs(60);
/// The default max transit time of a class A stream.
const MAX_TRANSIT_TIME: u32 = 2_000_000;
/// A class B stream output's max transit time when none is set, in
/// nanoseconds, room for a Wi-Fi hop.
const MAX_TRANSIT_TIME_CLASS_B: u32 = 12_000_000;
/// The registered controllers an entity keeps (Milan 1.3, 5.4.2.20).
const MOST_REGISTERED: usize = 16;
/// The Milan specification this entity implements.
const MILAN_VERSION: [u8; 4] = [1, 3, 0, 0];

/// What the entity knows of gPTP on its interface, from whatever runs it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Gptp {
    pub grandmaster: ClockIdentity,
    pub domain: u8,
    /// Peer delay to the neighbor, in nanoseconds.
    pub propagation_delay: u32,
    pub as_capable: bool,
    /// The path from the grandmaster, as PATH_TRACE carries it.
    pub path: Vec<ClockIdentity>,
}

/// A stream output's reservation, from MSRP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct OutputReservation {
    /// Listeners ready for the stream behind the port.
    pub ready_listeners: u16,
    /// The talker declares the stream.
    pub registering: bool,
}

/// A stream input's reservation, from MSRP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct InputReservation {
    /// The talker's advertisement is registered.
    pub talker_registered: bool,
    /// Why and where the reservation failed, from a Talker Failed.
    pub failure: Option<(u8, u64)>,
    pub accumulated_latency: u32,
    /// The listener declares the stream.
    pub registering: bool,
}

/// What a stream input is bound to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputBinding {
    pub talker: EntityId,
    pub talker_unique_id: u16,
    pub controller: EntityId,
    pub flags: AcmpFlags,
}

/// The stream a probe found for a bound input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProbedStream {
    pub stream_id: StreamId,
    pub destination: MacAddress,
    /// Zero for the SR class's VLAN.
    pub vlan_id: u16,
}

/// What happened that the program running the entity acts on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityEvent {
    /// Identification starts or stops.
    Identify(bool),
    /// A controller bound a stream input to a talker's stream output.
    InputBound {
        index: u16,
        binding: InputBinding,
    },
    /// The probe found the bound talker's stream, or found it again.
    InputSettled {
        index: u16,
        stream: ProbedStream,
    },
    /// The input lost its talker's stream: unbound, or the talker left.
    InputUnsettled {
        index: u16,
    },
    InputUnbound {
        index: u16,
    },
    StreamFormatChanged {
        descriptor_type: DescriptorType,
        index: u16,
        format: StreamFormat,
    },
    SamplingRateChanged(u32),
    ClockSourceChanged(u16),
    /// A controller allowed or disallowed escalating streams to
    /// multicast in AVB Lite (SET_LITE_CONFIG).
    LiteConfigChanged {
        escalation_allowed: bool,
    },
    MaxTransitTimeChanged {
        index: u16,
        nanoseconds: u32,
    },
    /// A name changed, to be kept.
    NameChanged,
    /// A controller changed which of an input's stream channels its
    /// clusters take (ADD_AUDIO_MAPPINGS, REMOVE_AUDIO_MAPPINGS).
    MappingsChanged {
        index: u16,
    },
}

/// Where a bound input's probe stands (Milan 1.3, 5.5.3.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Probe {
    /// Waiting to send a probe at this time.
    Due(Instant),
    /// Sent with this sequence ID, answered by this time.
    Waiting {
        sequence: u16,
        until: Instant,
    },
    Settled(ProbedStream),
}

#[derive(Debug, Clone)]
struct Input {
    binding: Option<InputBinding>,
    probe: Option<Probe>,
    /// The status of the last probe.
    probe_status: AcmpStatus,
    reservation: InputReservation,
    counters: [u32; 32],
}

#[derive(Debug, Clone)]
struct Output {
    stream_id: StreamId,
    destination: MacAddress,
    reservation: OutputReservation,
    max_transit_time: u32,
    class_b: bool,
    counters: [u32; 32],
}

impl Output {
    /// The max transit time when none is set, by the stream's class.
    fn default_transit(&self) -> u32 {
        if self.class_b {
            MAX_TRANSIT_TIME_CLASS_B
        } else {
            MAX_TRANSIT_TIME
        }
    }
}

/// An entity this one has heard advertise.
#[derive(Debug, Clone, Copy)]
struct Known {
    available_index: u32,
    expires: Instant,
}

/// A controller registered for unsolicited notifications.
#[derive(Debug, Clone, Copy)]
struct Registered {
    controller: EntityId,
    mac: MacAddress,
}

/// A small xorshift generator for the random delays ADP calls for.
#[derive(Debug, Clone, Copy)]
struct Random(u64);

impl Random {
    fn up_to(&mut self, limit: Duration) -> Duration {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        let millis = limit.as_millis() as u64;
        Duration::from_millis(self.0 % (millis + 1))
    }
}

/// A talker or listener entity.
pub struct Entity {
    model: EndpointModel,
    available_index: u32,
    advertise_at: Option<Instant>,
    gptp: Gptp,
    identifying: bool,
    lock: Option<(EntityId, Instant)>,
    registered: Vec<Registered>,
    unsolicited_sequence: u16,
    acmp_sequence: u16,
    /// What GET_LITE_STATUS answers, for an entity that supports AVB Lite.
    lite: Option<LiteStatus>,
    outputs: Vec<Output>,
    inputs: Vec<Input>,
    /// The clock domain's counters: how often it locked and unlocked.
    clock_counters: [u32; 32],
    /// Each stream port input's dynamic mappings, by port.
    input_mappings: Vec<Vec<AudioMapping>>,
    known: BTreeMap<EntityId, Known>,
    random: Random,
    outgoing: VecDeque<(MacAddress, Vec<u8>)>,
    events: VecDeque<EntityEvent>,
}

impl Entity {
    /// An entity for `model`, with each stream output sent as the stream
    /// ID and to the destination given, in order.
    pub fn new(model: EndpointModel, outputs: &[(StreamId, MacAddress)]) -> Self {
        let outputs = model
            .outputs
            .iter()
            .zip(outputs)
            .map(|(stream, &(stream_id, destination))| Output {
                stream_id,
                destination,
                reservation: OutputReservation::default(),
                max_transit_time: if stream.class_b {
                    MAX_TRANSIT_TIME_CLASS_B
                } else {
                    MAX_TRANSIT_TIME
                },
                class_b: stream.class_b,
                counters: [0; 32],
            })
            .collect();
        let inputs = model
            .inputs
            .iter()
            .map(|_| Input {
                binding: None,
                probe: None,
                probe_status: AcmpStatus::SUCCESS,
                reservation: InputReservation::default(),
                counters: [0; 32],
            })
            .collect();
        // Each input's channels to its clusters one to one, until a
        // controller maps them otherwise.
        let input_mappings = model
            .ports()
            .into_iter()
            .filter(|port| port.input)
            .map(|port| one_to_one(port.stream, port.channels))
            .collect();
        Entity {
            random: Random(model.entity_id.0 | 1),
            input_mappings,
            model,
            available_index: 0,
            advertise_at: None,
            gptp: Gptp::default(),
            identifying: false,
            lock: None,
            registered: Vec::new(),
            unsolicited_sequence: 0,
            acmp_sequence: 0,
            lite: None,
            outputs,
            inputs,
            clock_counters: [0; 32],
            known: BTreeMap::new(),
            outgoing: VecDeque::new(),
            events: VecDeque::new(),
        }
    }

    pub fn model(&self) -> &EndpointModel {
        &self.model
    }

    pub fn entity_id(&self) -> EntityId {
        self.model.entity_id
    }

    /// The stream ID and destination of a stream output.
    pub fn output_stream(&self, index: u16) -> Option<(StreamId, MacAddress)> {
        let output = self.outputs.get(usize::from(index))?;
        Some((output.stream_id, output.destination))
    }

    /// The max transit time of a stream output, in nanoseconds: how far
    /// ahead of sending its presentation times are.
    pub fn max_transit_time(&self, index: u16) -> Option<u32> {
        Some(self.outputs.get(usize::from(index))?.max_transit_time)
    }

    /// Whether a stream output's max transit time is its class's default.
    pub fn max_transit_time_is_default(&self, index: u16) -> bool {
        self.outputs
            .get(usize::from(index))
            .is_none_or(|output| output.max_transit_time == output.default_transit())
    }

    /// Gives a stream output back the max transit time a controller set
    /// before, as the entity starts; zero is the default.
    pub fn restore_max_transit_time(&mut self, index: u16, nanoseconds: u32) {
        if let Some(output) = self.outputs.get_mut(usize::from(index)) {
            output.max_transit_time = match nanoseconds {
                0 => output.default_transit(),
                nanoseconds => nanoseconds,
            };
        }
    }

    /// The mappings of stream input `index`'s port: which of its channels
    /// each of its clusters takes. None for an input without a port, as a
    /// media clock's.
    pub fn input_mappings(&self, index: u16) -> Option<&[AudioMapping]> {
        let port = self.input_port(index)?;
        self.input_mappings
            .get(usize::from(port))
            .map(Vec::as_slice)
    }

    /// Whether stream input `index`'s mappings are one to one, as they
    /// start.
    pub fn input_mappings_are_default(&self, index: u16) -> bool {
        let Some(port) = self
            .model
            .ports()
            .into_iter()
            .find(|port| port.input && port.stream == index)
        else {
            return true;
        };
        self.input_mappings(index)
            .is_none_or(|mappings| mappings == one_to_one(port.stream, port.channels))
    }

    /// Takes the mappings a controller gave stream input `index` before,
    /// as `(stream channel, cluster)`, those that fit its channels.
    pub fn restore_input_mappings(&mut self, index: u16, mappings: &[(u16, u16)]) {
        let Some(port) = self
            .model
            .ports()
            .into_iter()
            .find(|port| port.input && port.stream == index)
        else {
            return;
        };
        let mut kept: Vec<AudioMapping> = Vec::new();
        for &(stream_channel, cluster) in mappings {
            let mapping = AudioMapping {
                stream_index: index,
                stream_channel,
                cluster_offset: cluster,
                cluster_channel: 0,
            };
            if stream_channel < port.channels
                && cluster < port.channels
                && !kept.iter().any(|other| other.cluster_offset == cluster)
            {
                kept.push(mapping);
            }
        }
        kept.sort();
        self.input_mappings[usize::from(port.index)] = kept;
    }

    /// The stream port input of stream input `index`.
    fn input_port(&self, index: u16) -> Option<u16> {
        self.model
            .ports()
            .into_iter()
            .find(|port| port.input && port.stream == index)
            .map(|port| port.index)
    }

    pub fn input_binding(&self, index: u16) -> Option<InputBinding> {
        self.inputs.get(usize::from(index))?.binding
    }

    /// What GET_LITE_STATUS answers now, for an entity that supports AVB
    /// Lite.
    pub fn lite_status(&self) -> Option<&LiteStatus> {
        self.lite.as_ref()
    }

    /// Sets what GET_LITE_STATUS answers for the entity's interface,
    /// telling registered controllers when anything but the offset
    /// changes, or the offset crosses the AVB Lite alarm of 50 us (AVB
    /// Lite profile, 2.4).
    pub fn set_lite_status(&mut self, status: LiteStatus) {
        const ALARM: i32 = 50_000;
        let alarmed = |status: &LiteStatus| {
            status
                .offset()
                .is_some_and(|offset| offset.unsigned_abs() > ALARM.unsigned_abs())
        };
        let tell = self.lite.is_none_or(|before| {
            let quiet = |status: &LiteStatus| LiteStatus {
                offset_from_grandmaster: 0,
                ..*status
            };
            quiet(&before) != quiet(&status) || alarmed(&before) != alarmed(&status)
        });
        self.lite = Some(status);
        if tell {
            let registered = self.registered.clone();
            for registered in registered {
                self.unsolicited_sequence = self.unsolicited_sequence.wrapping_add(1);
                let header = AecpHeader {
                    message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
                    status: 0,
                    target_entity_id: self.model.entity_id,
                    controller_entity_id: registered.controller,
                    sequence_id: self.unsolicited_sequence,
                };
                let mut data = (0x8000 | LiteCommandType::GET_LITE_STATUS.0)
                    .to_be_bytes()
                    .to_vec();
                data.extend_from_slice(&status.to_bytes());
                let pdu = VendorUniquePdu {
                    header,
                    protocol_id: STATUS_PROTOCOL_ID,
                    payload: &data,
                };
                let mut out = [0; 200];
                if let Ok(length) = pdu.encode(&mut out) {
                    self.outgoing
                        .push_back((registered.mac, out[..length].to_vec()));
                }
            }
        }
    }

    /// Sends output `index`'s stream to `destination` from now on, as MAAP
    /// gave it, telling registered controllers.
    pub fn set_output_destination(&mut self, index: u16, destination: MacAddress) {
        let Some(output) = self.outputs.get_mut(usize::from(index)) else {
            return;
        };
        if output.destination != destination {
            output.destination = destination;
            self.notify_stream_info(DescriptorType::STREAM_OUTPUT, index);
        }
    }

    /// Probes the talker of bound input `index` again, as when its stream
    /// may have moved.
    pub fn probe_again(&mut self, now: Instant, index: u16) {
        if let Some(input) = self.inputs.get_mut(usize::from(index))
            && input.binding.is_some()
            && !matches!(input.probe, Some(Probe::Waiting { .. }))
        {
            input.probe = Some(Probe::Due(now));
        }
    }

    /// Binds stream input `index` as it was bound before the entity last
    /// stopped, as a Milan listener keeps its binding across a restart,
    /// and probes the talker at once.
    pub fn restore_binding(&mut self, now: Instant, index: u16, binding: InputBinding) {
        let Some(input) = self.inputs.get_mut(usize::from(index)) else {
            return;
        };
        input.binding = Some(binding);
        input.probe = Some(Probe::Due(now));
        self.events
            .push_back(EntityEvent::InputBound { index, binding });
    }

    /// The stream a bound input's probe found.
    pub fn input_stream(&self, index: u16) -> Option<ProbedStream> {
        match self.inputs.get(usize::from(index))?.probe {
            Some(Probe::Settled(stream)) => Some(stream),
            _ => None,
        }
    }

    /// Starts advertising, a random moment from now.
    pub fn start(&mut self, now: Instant) {
        if self.advertise_at.is_none() {
            self.advertise_at = Some(now + self.random.up_to(DISCOVER_DELAY));
        }
    }

    /// Takes over from `earlier`, this entity before its model changed,
    /// in place of starting: its advertisements count on from that one's,
    /// so controllers read the new model rather than see it leave or
    /// restart; it advertises at once, and keeps the controllers
    /// registered with it, its lock and the entities it heard.
    pub fn take_over(&mut self, now: Instant, earlier: &Entity) {
        self.available_index = earlier.available_index;
        self.registered.clone_from(&earlier.registered);
        self.lock = earlier.lock;
        self.known.clone_from(&earlier.known);
        self.advertise_at = Some(now);
    }

    /// Carries the state of `earlier`'s stream outputs and inputs at the
    /// indexes given into this entity's, those streams running on as
    /// they were: their reservations and counters, and an input's
    /// binding, the stream its probe found and its mappings, so it need
    /// not probe again.
    pub fn carry_streams(&mut self, earlier: &Entity, outputs: &[u16], inputs: &[u16]) {
        for &index in outputs {
            let at = usize::from(index);
            if let (Some(output), Some(before)) =
                (self.outputs.get_mut(at), earlier.outputs.get(at))
            {
                output.reservation = before.reservation;
                output.counters = before.counters;
            }
        }
        for &index in inputs {
            let at = usize::from(index);
            if let (Some(input), Some(before)) = (self.inputs.get_mut(at), earlier.inputs.get(at)) {
                input.binding = before.binding;
                input.probe = before.probe;
                input.probe_status = before.probe_status;
                input.reservation = before.reservation;
                input.counters = before.counters;
            }
            if let (Some(port), Some(mappings)) =
                (self.input_port(index), earlier.input_mappings(index))
                && let Some(slot) = self.input_mappings.get_mut(usize::from(port))
            {
                slot.clone_from(&mappings.to_vec());
            }
        }
    }

    /// Says the entity is leaving and stops advertising.
    pub fn depart(&mut self) {
        self.advertise_at = None;
        let mut adpdu = self.adpdu();
        adpdu.message_type = AdpMessageType::ENTITY_DEPARTING;
        adpdu.valid_time = 0;
        self.send_adp(&adpdu);
    }

    /// Takes what gPTP says: a new grandmaster is advertised at once, and
    /// registered controllers hear the change.
    pub fn set_gptp(&mut self, now: Instant, gptp: Gptp) {
        if gptp == self.gptp {
            return;
        }
        let new_grandmaster =
            gptp.grandmaster != self.gptp.grandmaster || gptp.domain != self.gptp.domain;
        self.gptp = gptp;
        if new_grandmaster && self.advertise_at.is_some() {
            self.advertise_at = Some(now);
        }
        let payload = self.avb_info(0);
        self.notify(AemCommandType::GET_AVB_INFO, &payload, None);
    }

    /// Takes a stream output's reservation, from MSRP.
    pub fn set_output_reservation(&mut self, index: u16, reservation: OutputReservation) {
        let Some(output) = self.outputs.get_mut(usize::from(index)) else {
            return;
        };
        if output.reservation != reservation {
            output.reservation = reservation;
            self.notify_stream_info(DescriptorType::STREAM_OUTPUT, index);
        }
    }

    /// Takes a stream input's reservation, from MSRP.
    pub fn set_input_reservation(&mut self, index: u16, reservation: InputReservation) {
        let Some(input) = self.inputs.get_mut(usize::from(index)) else {
            return;
        };
        if input.reservation != reservation {
            input.reservation = reservation;
            self.notify_stream_info(DescriptorType::STREAM_INPUT, index);
        }
    }

    /// Takes a stream's or the clock domain's counters, by their place in
    /// GET_COUNTERS (Milan 1.3, 5.4.2.25): registered controllers hear
    /// when they change. Call it no more than once a second.
    pub fn set_counters(
        &mut self,
        descriptor_type: DescriptorType,
        index: u16,
        counters: [u32; 32],
    ) {
        let slot = match descriptor_type {
            DescriptorType::CLOCK_DOMAIN if index == 0 => Some(&mut self.clock_counters),
            DescriptorType::STREAM_INPUT => self
                .inputs
                .get_mut(usize::from(index))
                .map(|input| &mut input.counters),
            DescriptorType::STREAM_OUTPUT => self
                .outputs
                .get_mut(usize::from(index))
                .map(|output| &mut output.counters),
            _ => None,
        };
        let Some(slot) = slot else {
            return;
        };
        if *slot != counters {
            *slot = counters;
            if let Some(payload) = self.counters(descriptor_type, index) {
                self.notify(AemCommandType::GET_COUNTERS, &payload, None);
            }
        }
    }

    fn entity_state(&self) -> EntityState {
        EntityState {
            capabilities: self.capabilities().0,
            talker_capabilities: self.talker_capabilities().0,
            listener_capabilities: self.listener_capabilities().0,
            available_index: self.available_index,
            identifying: self.identifying,
        }
    }

    fn capabilities(&self) -> EntityCapabilities {
        let capabilities = EntityCapabilities::AEM_SUPPORTED
            | EntityCapabilities::VENDOR_UNIQUE_SUPPORTED
            | EntityCapabilities::CLASS_A_SUPPORTED
            | EntityCapabilities::GPTP_SUPPORTED
            | EntityCapabilities::AEM_IDENTIFY_CONTROL_INDEX_VALID
            | EntityCapabilities::AEM_INTERFACE_INDEX_VALID;
        if self.outputs.iter().any(|output| output.class_b) {
            capabilities | EntityCapabilities::CLASS_B_SUPPORTED
        } else {
            capabilities
        }
    }

    /// A talker, if it has outputs, of the kinds they send: audio, media
    /// clock or both.
    fn talker_capabilities(&self) -> TalkerCapabilities {
        let outputs = &self.model.outputs;
        let mut capabilities = TalkerCapabilities::default();
        if !outputs.is_empty() {
            capabilities |= TalkerCapabilities::IMPLEMENTED;
        }
        if outputs
            .iter()
            .any(|output| !output.current_format.is_clock())
        {
            capabilities |= TalkerCapabilities::AUDIO_SOURCE;
        }
        if outputs
            .iter()
            .any(|output| output.current_format.is_clock())
        {
            capabilities |= TalkerCapabilities::MEDIA_CLOCK_SOURCE;
        }
        capabilities
    }

    /// A listener, if it has inputs, of the kinds they take.
    fn listener_capabilities(&self) -> ListenerCapabilities {
        let inputs = &self.model.inputs;
        let mut capabilities = ListenerCapabilities::default();
        if !inputs.is_empty() {
            capabilities |= ListenerCapabilities::IMPLEMENTED;
        }
        if inputs.iter().any(|input| !input.current_format.is_clock()) {
            capabilities |= ListenerCapabilities::AUDIO_SINK;
        }
        if inputs.iter().any(|input| input.current_format.is_clock()) {
            capabilities |= ListenerCapabilities::MEDIA_CLOCK_SINK;
        }
        capabilities
    }

    fn adpdu(&self) -> Adpdu {
        Adpdu {
            message_type: AdpMessageType::ENTITY_AVAILABLE,
            valid_time: VALID_TIME,
            entity_id: self.model.entity_id,
            entity_model_id: self.model.entity_model_id,
            entity_capabilities: self.capabilities(),
            talker_stream_sources: self.model.outputs.len() as u16,
            talker_capabilities: self.talker_capabilities(),
            listener_stream_sinks: self.model.inputs.len() as u16,
            listener_capabilities: self.listener_capabilities(),
            available_index: self.available_index,
            gptp_grandmaster_id: self.gptp.grandmaster,
            gptp_domain_number: self.gptp.domain,
            ..Adpdu::default()
        }
    }

    fn send_adp(&mut self, adpdu: &Adpdu) {
        let mut out = [0; Adpdu::LEN];
        if let Ok(length) = adpdu.encode(&mut out) {
            self.outgoing
                .push_back((ADP_ACMP_MULTICAST, out[..length].to_vec()));
        }
    }

    fn send_acmp(&mut self, acmpdu: &Acmpdu) {
        let mut out = [0; Acmpdu::FULL_LEN];
        if let Ok(length) = acmpdu.encode(&mut out) {
            self.outgoing
                .push_back((ADP_ACMP_MULTICAST, out[..length].to_vec()));
        }
    }

    /// Handles a frame received on the AVTP ethertype from `source`.
    pub fn handle_frame(&mut self, now: Instant, source: MacAddress, bytes: &[u8]) {
        let Ok(pdu) = pdu::decode(bytes) else {
            return;
        };
        match pdu {
            Pdu::Adp(adpdu) => self.handle_adp(now, &adpdu),
            Pdu::Acmp(acmpdu) => self.handle_acmp(now, &acmpdu),
            Pdu::Aem(aem)
                if aem.header.target_entity_id == self.model.entity_id
                    && aem.header.message_type == AecpMessageType::AEM_COMMAND =>
            {
                self.handle_command(now, source, &aem);
            }
            Pdu::VendorUnique(vendor)
                if vendor.header.target_entity_id == self.model.entity_id
                    && vendor.header.message_type == AecpMessageType::VENDOR_UNIQUE_COMMAND =>
            {
                self.handle_vendor_unique(source, &vendor);
            }
            _ => {}
        }
    }

    fn handle_adp(&mut self, now: Instant, adpdu: &Adpdu) {
        match adpdu.message_type {
            AdpMessageType::ENTITY_DISCOVER => {
                let wanted = adpdu.entity_id;
                if (wanted == EntityId(0) || wanted == self.model.entity_id)
                    && let Some(at) = self.advertise_at
                {
                    let soon = now + self.random.up_to(DISCOVER_DELAY);
                    self.advertise_at = Some(at.min(soon));
                }
            }
            AdpMessageType::ENTITY_AVAILABLE if adpdu.entity_id != self.model.entity_id => {
                let expires = now + Duration::from_secs(u64::from(adpdu.valid_seconds().max(2)));
                let restarted = self
                    .known
                    .get(&adpdu.entity_id)
                    .is_none_or(|known| adpdu.available_index < known.available_index);
                self.known.insert(
                    adpdu.entity_id,
                    Known {
                        available_index: adpdu.available_index,
                        expires,
                    },
                );
                if restarted {
                    self.talker_appeared(now, adpdu.entity_id);
                }
            }
            AdpMessageType::ENTITY_DEPARTING => {
                self.known.remove(&adpdu.entity_id);
                self.talker_left(now, adpdu.entity_id);
                self.registered
                    .retain(|registered| registered.controller != adpdu.entity_id);
            }
            _ => {}
        }
    }

    /// A talker came online, or restarted: inputs bound to it probe again.
    fn talker_appeared(&mut self, now: Instant, talker: EntityId) {
        for input in &mut self.inputs {
            if input
                .binding
                .is_some_and(|binding| binding.talker == talker)
                && !matches!(input.probe, Some(Probe::Waiting { .. }))
            {
                input.probe = Some(Probe::Due(now));
            }
        }
    }

    /// A talker went away: inputs bound to it lose their stream and wait
    /// for it.
    fn talker_left(&mut self, now: Instant, talker: EntityId) {
        for index in 0..self.inputs.len() {
            let input = &mut self.inputs[index];
            if input
                .binding
                .is_some_and(|binding| binding.talker == talker)
            {
                let settled = matches!(input.probe, Some(Probe::Settled(_)));
                input.probe = Some(Probe::Due(now + PROBE_RETRY));
                if settled {
                    self.events.push_back(EntityEvent::InputUnsettled {
                        index: index as u16,
                    });
                    self.notify_stream_info(DescriptorType::STREAM_INPUT, index as u16);
                }
            }
        }
    }

    fn handle_acmp(&mut self, now: Instant, acmpdu: &Acmpdu) {
        let us = self.model.entity_id;
        let message = acmpdu.message_type;
        if message.is_command() && acmpdu.talker_entity_id == us && is_talker_command(message) {
            self.answer_talker(acmpdu);
        } else if message.is_command()
            && acmpdu.listener_entity_id == us
            && !is_talker_command(message)
        {
            // A talker command naming this entity's input is its own probe,
            // as it hears its own frames: its talker answers it.
            self.answer_listener(now, acmpdu);
        } else if message == AcmpMessageType::PROBE_TX_RESPONSE && acmpdu.listener_entity_id == us {
            self.probe_answered(now, acmpdu);
        }
    }

    fn answer_talker(&mut self, command: &Acmpdu) {
        let mut response = Acmpdu {
            message_type: command.message_type.response(),
            ip: None,
            ..*command
        };
        match self.outputs.get(usize::from(command.talker_unique_id)) {
            None => response.status = AcmpStatus::TALKER_UNKNOWN_ID,
            Some(output) => match command.message_type {
                AcmpMessageType::PROBE_TX_COMMAND | AcmpMessageType::GET_TX_STATE_COMMAND => {
                    response.stream_id = output.stream_id;
                    response.stream_dest_mac = output.destination;
                    response.stream_vlan_id = 0;
                    response.connection_count = output.reservation.ready_listeners;
                    response.flags = AcmpFlags::default();
                    // No address yet, as while MAAP claims one: a probing
                    // listener tries again later.
                    if output.destination == UNADDRESSED
                        && command.message_type == AcmpMessageType::PROBE_TX_COMMAND
                    {
                        response.status = AcmpStatus::TALKER_DEST_MAC_FAIL;
                    }
                }
                AcmpMessageType::DISCONNECT_TX_COMMAND => {}
                _ => response.status = AcmpStatus::NOT_SUPPORTED,
            },
        }
        self.send_acmp(&response);
    }

    fn answer_listener(&mut self, now: Instant, command: &Acmpdu) {
        let index = command.listener_unique_id;
        let mut response = Acmpdu {
            message_type: command.message_type.response(),
            ip: None,
            ..*command
        };
        if usize::from(index) >= self.inputs.len() {
            response.status = AcmpStatus::LISTENER_UNKNOWN_ID;
            self.send_acmp(&response);
            return;
        }
        match command.message_type {
            AcmpMessageType::BIND_RX_COMMAND => {
                let binding = InputBinding {
                    talker: command.talker_entity_id,
                    talker_unique_id: command.talker_unique_id,
                    controller: command.controller_entity_id,
                    flags: command.flags,
                };
                let input = &mut self.inputs[usize::from(index)];
                let same = input.binding.is_some_and(|bound| {
                    bound.talker == binding.talker
                        && bound.talker_unique_id == binding.talker_unique_id
                });
                if !same {
                    let settled = matches!(input.probe, Some(Probe::Settled(_)));
                    input.binding = Some(binding);
                    input.probe = Some(Probe::Due(now));
                    if settled {
                        self.events.push_back(EntityEvent::InputUnsettled { index });
                    }
                    self.events
                        .push_back(EntityEvent::InputBound { index, binding });
                } else {
                    input.binding = Some(binding);
                }
                response.connection_count = 1;
                self.fill_rx_state(index, &mut response);
                self.send_acmp(&response);
                self.notify_stream_info(DescriptorType::STREAM_INPUT, index);
            }
            AcmpMessageType::UNBIND_RX_COMMAND => {
                let input = &mut self.inputs[usize::from(index)];
                if input.binding.take().is_some() {
                    let settled = matches!(input.probe, Some(Probe::Settled(_)));
                    input.probe = None;
                    if settled {
                        self.events.push_back(EntityEvent::InputUnsettled { index });
                    }
                    self.events.push_back(EntityEvent::InputUnbound { index });
                }
                response.connection_count = 0;
                self.send_acmp(&response);
                self.notify_stream_info(DescriptorType::STREAM_INPUT, index);
            }
            AcmpMessageType::GET_RX_STATE_COMMAND => {
                self.fill_rx_state(index, &mut response);
                self.send_acmp(&response);
            }
            _ => {
                response.status = AcmpStatus::NOT_SUPPORTED;
                self.send_acmp(&response);
            }
        }
    }

    /// The binding and stream of an input, in an ACMP response.
    fn fill_rx_state(&self, index: u16, response: &mut Acmpdu) {
        let input = &self.inputs[usize::from(index)];
        match input.binding {
            Some(binding) => {
                response.talker_entity_id = binding.talker;
                response.talker_unique_id = binding.talker_unique_id;
                response.controller_entity_id = binding.controller;
                response.connection_count = 1;
                response.flags = binding.flags;
            }
            None => {
                response.talker_entity_id = EntityId(0);
                response.talker_unique_id = 0;
                response.connection_count = 0;
            }
        }
        if let Some(Probe::Settled(stream)) = input.probe {
            response.stream_id = stream.stream_id;
            response.stream_dest_mac = stream.destination;
            response.stream_vlan_id = stream.vlan_id;
        }
    }

    fn probe_answered(&mut self, now: Instant, response: &Acmpdu) {
        let index = response.listener_unique_id;
        let Some(input) = self.inputs.get_mut(usize::from(index)) else {
            return;
        };
        let Some(Probe::Waiting { sequence, .. }) = input.probe else {
            return;
        };
        if sequence != response.sequence_id
            || input.binding.is_none_or(|binding| {
                binding.talker != response.talker_entity_id
                    || binding.talker_unique_id != response.talker_unique_id
            })
        {
            return;
        }
        input.probe_status = response.status;
        if response.status.is_success() {
            let stream = ProbedStream {
                stream_id: response.stream_id,
                destination: response.stream_dest_mac,
                vlan_id: response.stream_vlan_id,
            };
            input.probe = Some(Probe::Settled(stream));
            self.events
                .push_back(EntityEvent::InputSettled { index, stream });
            self.notify_stream_info(DescriptorType::STREAM_INPUT, index);
        } else {
            input.probe = Some(Probe::Due(now + PROBE_RETRY));
        }
    }

    /// Sends the probes that are due, and gives up on those unanswered.
    fn probe(&mut self, now: Instant) {
        for index in 0..self.inputs.len() {
            let input = &self.inputs[index];
            let (Some(binding), Some(probe)) = (input.binding, input.probe) else {
                continue;
            };
            match probe {
                Probe::Due(at) if at <= now => {
                    self.acmp_sequence = self.acmp_sequence.wrapping_add(1);
                    let sequence = self.acmp_sequence;
                    let command = Acmpdu {
                        flags: binding.flags,
                        ..Acmpdu::command(
                            AcmpMessageType::PROBE_TX_COMMAND,
                            binding.controller,
                            (binding.talker, binding.talker_unique_id),
                            (self.model.entity_id, index as u16),
                            sequence,
                        )
                    };
                    self.send_acmp(&command);
                    self.inputs[index].probe = Some(Probe::Waiting {
                        sequence,
                        until: now + PROBE_TIMEOUT,
                    });
                }
                Probe::Waiting { until, .. } if until <= now => {
                    self.inputs[index].probe_status = AcmpStatus::LISTENER_TALKER_TIMEOUT;
                    self.inputs[index].probe = Some(Probe::Due(now + PROBE_RETRY));
                }
                _ => {}
            }
        }
    }

    /// Handles the timers due by `now`.
    pub fn handle_timeout(&mut self, now: Instant) {
        if self.advertise_at.is_some_and(|at| at <= now) {
            self.available_index = self.available_index.wrapping_add(1);
            let adpdu = self.adpdu();
            self.send_adp(&adpdu);
            self.advertise_at = Some(now + ADVERTISE_INTERVAL);
        }
        if self.lock.is_some_and(|(_, until)| until <= now) {
            self.lock = None;
        }
        let expired: Vec<EntityId> = self
            .known
            .iter()
            .filter(|(_, known)| known.expires <= now)
            .map(|(entity_id, _)| *entity_id)
            .collect();
        for entity_id in expired {
            self.known.remove(&entity_id);
            self.talker_left(now, entity_id);
        }
        self.probe(now);
    }

    /// When `handle_timeout` should next be called.
    pub fn poll_timeout(&self) -> Option<Instant> {
        let probes = self.inputs.iter().filter_map(|input| match input.probe {
            Some(Probe::Due(at)) => Some(at),
            Some(Probe::Waiting { until, .. }) => Some(until),
            _ => None,
        });
        let known = self.known.values().map(|known| known.expires);
        let lock = self.lock.map(|(_, until)| until);
        probes
            .chain(known)
            .chain(lock)
            .chain(self.advertise_at)
            .min()
    }

    /// The next frame to send: its destination and the octets after the
    /// Ethernet header.
    pub fn poll_transmit(&mut self) -> Option<(MacAddress, Vec<u8>)> {
        self.outgoing.pop_front()
    }

    pub fn poll_event(&mut self) -> Option<EntityEvent> {
        self.events.pop_front()
    }

    /// Sends an AEM response, or an unsolicited notification.
    fn respond(
        &mut self,
        destination: MacAddress,
        header: AecpHeader,
        command_type: AemCommandType,
        status: AemStatus,
        payload: &[u8],
        unsolicited: bool,
    ) {
        let pdu = AemPdu {
            header: AecpHeader {
                message_type: AecpMessageType::AEM_RESPONSE,
                status: status.0,
                target_entity_id: self.model.entity_id,
                ..header
            },
            unsolicited,
            controller_request: false,
            command_type,
            payload,
        };
        let mut out = [0; 1500];
        if let Ok(length) = pdu.encode(&mut out) {
            self.outgoing
                .push_back((destination, out[..length].to_vec()));
        }
    }

    /// Tells every registered controller but `except` what changed, as
    /// the response to `command_type` with `payload`.
    fn notify(&mut self, command_type: AemCommandType, payload: &[u8], except: Option<EntityId>) {
        let registered = self.registered.clone();
        for registered in registered {
            if Some(registered.controller) == except {
                continue;
            }
            self.unsolicited_sequence = self.unsolicited_sequence.wrapping_add(1);
            let header = AecpHeader {
                message_type: AecpMessageType::AEM_RESPONSE,
                status: 0,
                target_entity_id: self.model.entity_id,
                controller_entity_id: registered.controller,
                sequence_id: self.unsolicited_sequence,
            };
            self.respond(
                registered.mac,
                header,
                command_type,
                AemStatus::SUCCESS,
                payload,
                true,
            );
        }
    }

    fn notify_stream_info(&mut self, descriptor_type: DescriptorType, index: u16) {
        if let Some(payload) = self.stream_info(descriptor_type, index) {
            self.notify(AemCommandType::GET_STREAM_INFO, &payload, None);
        }
    }

    /// Whether `controller` may change the entity: no other controller
    /// holds its lock.
    fn may_change(&self, controller: EntityId) -> bool {
        self.lock.is_none_or(|(holder, _)| holder == controller)
    }

    fn handle_command(&mut self, now: Instant, source: MacAddress, command: &AemPdu<'_>) {
        let controller = command.header.controller_entity_id;
        let changes = matches!(
            command.command_type,
            AemCommandType::SET_STREAM_FORMAT
                | AemCommandType::SET_NAME
                | AemCommandType::SET_SAMPLING_RATE
                | AemCommandType::SET_CLOCK_SOURCE
                | AemCommandType::SET_CONTROL
                | AemCommandType::SET_MAX_TRANSIT_TIME
                | AemCommandType::ADD_AUDIO_MAPPINGS
                | AemCommandType::REMOVE_AUDIO_MAPPINGS
        );
        if changes && !self.may_change(controller) {
            let payload = Vec::from(command.payload);
            self.respond(
                source,
                command.header,
                command.command_type,
                AemStatus::ENTITY_LOCKED,
                &payload,
                false,
            );
            return;
        }
        let (status, payload, notify) = self.answer(now, source, command);
        self.respond(
            source,
            command.header,
            command.command_type,
            status,
            &payload,
            false,
        );
        if status.is_success() && notify {
            self.notify(command.command_type, &payload, Some(controller));
        }
    }

    /// The status and payload answering an AEM command, and whether the
    /// other registered controllers hear of it.
    fn answer(
        &mut self,
        now: Instant,
        source: MacAddress,
        command: &AemPdu<'_>,
    ) -> (AemStatus, Vec<u8>, bool) {
        let payload = command.payload;
        let echo = || Vec::from(payload);
        let short = |needed: usize| payload.len() < needed;
        let controller = command.header.controller_entity_id;
        match command.command_type {
            AemCommandType::ENTITY_AVAILABLE | AemCommandType::CONTROLLER_AVAILABLE => {
                (AemStatus::SUCCESS, echo(), false)
            }
            AemCommandType::READ_DESCRIPTOR => {
                if short(8) {
                    return (AemStatus::BAD_ARGUMENTS, echo(), false);
                }
                let descriptor_type = DescriptorType(read_u16(payload, 4));
                let index = read_u16(payload, 6);
                match self
                    .model
                    .descriptor(descriptor_type, index, &self.entity_state())
                {
                    Some(descriptor) => {
                        let mut answer = Vec::from(&payload[..4]);
                        answer.extend_from_slice(&descriptor);
                        (AemStatus::SUCCESS, answer, false)
                    }
                    None => (AemStatus::NO_SUCH_DESCRIPTOR, echo(), false),
                }
            }
            AemCommandType::LOCK_ENTITY => {
                if short(16) {
                    return (AemStatus::BAD_ARGUMENTS, echo(), false);
                }
                let unlock = read_u32(payload, 0) & 1 != 0;
                let mut answer = echo();
                let status = match (unlock, self.lock) {
                    (_, Some((holder, _))) if holder != controller => {
                        answer[4..12].copy_from_slice(&holder.to_bytes());
                        AemStatus::ENTITY_LOCKED
                    }
                    (true, _) => {
                        self.lock = None;
                        answer[4..12].copy_from_slice(&[0; 8]);
                        AemStatus::SUCCESS
                    }
                    (false, _) => {
                        self.lock = Some((controller, now + LOCK_TIME));
                        answer[4..12].copy_from_slice(&controller.to_bytes());
                        AemStatus::SUCCESS
                    }
                };
                (status, answer, false)
            }
            AemCommandType::REGISTER_UNSOLICITED_NOTIFICATION => {
                if !self
                    .registered
                    .iter()
                    .any(|registered| registered.controller == controller)
                {
                    if self.registered.len() >= MOST_REGISTERED {
                        return (AemStatus::NO_RESOURCES, echo(), false);
                    }
                    self.registered.push(Registered {
                        controller,
                        mac: source,
                    });
                }
                (AemStatus::SUCCESS, echo(), false)
            }
            AemCommandType::DEREGISTER_UNSOLICITED_NOTIFICATION => {
                self.registered
                    .retain(|registered| registered.controller != controller);
                (AemStatus::SUCCESS, echo(), false)
            }
            AemCommandType::GET_STREAM_FORMAT | AemCommandType::SET_STREAM_FORMAT => {
                self.stream_format(command.command_type, payload)
            }
            AemCommandType::GET_STREAM_INFO => {
                if short(4) {
                    return (AemStatus::BAD_ARGUMENTS, echo(), false);
                }
                let descriptor_type = DescriptorType(read_u16(payload, 0));
                match self.stream_info(descriptor_type, read_u16(payload, 2)) {
                    Some(answer) => (AemStatus::SUCCESS, answer, false),
                    None => (AemStatus::NO_SUCH_DESCRIPTOR, echo(), false),
                }
            }
            AemCommandType::GET_NAME | AemCommandType::SET_NAME => {
                self.name(command.command_type, payload)
            }
            AemCommandType::GET_SAMPLING_RATE | AemCommandType::SET_SAMPLING_RATE => {
                if short(8) || DescriptorType(read_u16(payload, 0)) != DescriptorType::AUDIO_UNIT {
                    return (AemStatus::NO_SUCH_DESCRIPTOR, echo(), false);
                }
                let mut answer = echo();
                if command.command_type == AemCommandType::SET_SAMPLING_RATE {
                    let rate = read_u32(payload, 4);
                    if !self.model.sampling_rates.contains(&rate) {
                        return (AemStatus::BAD_ARGUMENTS, answer, false);
                    }
                    if rate != self.model.current_sampling_rate {
                        self.model.current_sampling_rate = rate;
                        self.events
                            .push_back(EntityEvent::SamplingRateChanged(rate));
                        self.follow_rate(rate, None);
                    }
                }
                answer[4..8].copy_from_slice(&self.model.current_sampling_rate.to_be_bytes());
                (AemStatus::SUCCESS, answer, true)
            }
            AemCommandType::GET_CLOCK_SOURCE | AemCommandType::SET_CLOCK_SOURCE => {
                if short(6) || DescriptorType(read_u16(payload, 0)) != DescriptorType::CLOCK_DOMAIN
                {
                    return (AemStatus::NO_SUCH_DESCRIPTOR, echo(), false);
                }
                let mut answer = echo();
                answer.resize(8, 0);
                if command.command_type == AemCommandType::SET_CLOCK_SOURCE {
                    let source = read_u16(payload, 4);
                    if source >= self.model.clock_sources() {
                        return (AemStatus::BAD_ARGUMENTS, answer, false);
                    }
                    if source != self.model.clock_source {
                        self.model.clock_source = source;
                        self.events
                            .push_back(EntityEvent::ClockSourceChanged(source));
                    }
                }
                answer[4..6].copy_from_slice(&self.model.clock_source.to_be_bytes());
                (AemStatus::SUCCESS, answer, true)
            }
            AemCommandType::GET_CONTROL | AemCommandType::SET_CONTROL => {
                if short(4)
                    || DescriptorType(read_u16(payload, 0)) != DescriptorType::CONTROL
                    || read_u16(payload, 2) != 0
                {
                    return (AemStatus::NO_SUCH_DESCRIPTOR, echo(), false);
                }
                if command.command_type == AemCommandType::SET_CONTROL {
                    let Some(&value) = payload.get(4) else {
                        return (AemStatus::BAD_ARGUMENTS, echo(), false);
                    };
                    let identifying = value != 0;
                    if identifying != self.identifying {
                        self.identifying = identifying;
                        self.events.push_back(EntityEvent::Identify(identifying));
                    }
                }
                let mut answer = Vec::from(&payload[..4]);
                answer.push(if self.identifying { 255 } else { 0 });
                (AemStatus::SUCCESS, answer, true)
            }
            AemCommandType::GET_AVB_INFO => {
                if short(4)
                    || DescriptorType(read_u16(payload, 0)) != DescriptorType::AVB_INTERFACE
                    || read_u16(payload, 2) != 0
                {
                    return (AemStatus::NO_SUCH_DESCRIPTOR, echo(), false);
                }
                (AemStatus::SUCCESS, self.avb_info(0), false)
            }
            AemCommandType::GET_AS_PATH => {
                if short(2) || read_u16(payload, 0) != 0 {
                    return (AemStatus::NO_SUCH_DESCRIPTOR, echo(), false);
                }
                let mut answer = Vec::from([0, 0]);
                answer.extend_from_slice(&(self.gptp.path.len() as u16).to_be_bytes());
                for identity in &self.gptp.path {
                    answer.extend_from_slice(&identity.0.to_be_bytes());
                }
                (AemStatus::SUCCESS, answer, false)
            }
            AemCommandType::GET_COUNTERS => {
                if short(4) {
                    return (AemStatus::BAD_ARGUMENTS, echo(), false);
                }
                let descriptor_type = DescriptorType(read_u16(payload, 0));
                let index = read_u16(payload, 2);
                match self.counters(descriptor_type, index) {
                    Some(answer) => (AemStatus::SUCCESS, answer, false),
                    None => (AemStatus::NO_SUCH_DESCRIPTOR, echo(), false),
                }
            }
            AemCommandType::GET_MAX_TRANSIT_TIME | AemCommandType::SET_MAX_TRANSIT_TIME => {
                if short(4) || DescriptorType(read_u16(payload, 0)) != DescriptorType::STREAM_OUTPUT
                {
                    return (AemStatus::NO_SUCH_DESCRIPTOR, echo(), false);
                }
                let index = read_u16(payload, 2);
                let Some(output) = self.outputs.get_mut(usize::from(index)) else {
                    return (AemStatus::NO_SUCH_DESCRIPTOR, echo(), false);
                };
                let mut answer = Vec::from(&payload[..4]);
                if command.command_type == AemCommandType::SET_MAX_TRANSIT_TIME {
                    if short(12) {
                        return (AemStatus::BAD_ARGUMENTS, echo(), false);
                    }
                    // Zero asks for the default.
                    let wanted = match read_u64(payload, 4) {
                        0 => output.default_transit(),
                        nanoseconds => u32::try_from(nanoseconds).unwrap_or(u32::MAX),
                    };
                    if wanted != output.max_transit_time {
                        output.max_transit_time = wanted;
                        self.events.push_back(EntityEvent::MaxTransitTimeChanged {
                            index,
                            nanoseconds: wanted,
                        });
                    }
                }
                let output = &self.outputs[usize::from(index)];
                answer.extend_from_slice(&u64::from(output.max_transit_time).to_be_bytes());
                (AemStatus::SUCCESS, answer, true)
            }
            AemCommandType::GET_AUDIO_MAP => self.audio_map(payload),
            AemCommandType::ADD_AUDIO_MAPPINGS | AemCommandType::REMOVE_AUDIO_MAPPINGS => {
                self.change_mappings(command.command_type, payload)
            }
            _ => (AemStatus::NOT_IMPLEMENTED, echo(), false),
        }
    }

    /// GET_AUDIO_MAP: a stream port input's dynamic mappings, in parts of
    /// as many as fit one response; a port with a static map has none to
    /// give, its AUDIO_MAP descriptor saying them.
    fn audio_map(&self, payload: &[u8]) -> (AemStatus, Vec<u8>, bool) {
        let echo = Vec::from(payload);
        if payload.len() < 8 {
            return (AemStatus::BAD_ARGUMENTS, echo, false);
        }
        let descriptor_type = DescriptorType(read_u16(payload, 0));
        let index = read_u16(payload, 2);
        let part = read_u16(payload, 4);
        let mappings = match descriptor_type {
            DescriptorType::STREAM_PORT_INPUT => self.input_mappings.get(usize::from(index)),
            DescriptorType::STREAM_PORT_OUTPUT => return (AemStatus::NOT_SUPPORTED, echo, false),
            _ => None,
        };
        let Some(mappings) = mappings else {
            return (AemStatus::NO_SUCH_DESCRIPTOR, echo, false);
        };
        const PER_PART: usize = (crate::aem::MAX_AEM_PAYLOAD - 12) / 8;
        let parts = mappings.len().div_ceil(PER_PART).max(1);
        if usize::from(part) >= parts {
            return (AemStatus::BAD_ARGUMENTS, echo, false);
        }
        let these: Vec<AudioMapping> = mappings
            .iter()
            .skip(usize::from(part) * PER_PART)
            .take(PER_PART)
            .copied()
            .collect();
        let mut answer = Vec::from(&payload[..6]);
        answer.extend_from_slice(&(parts as u16).to_be_bytes());
        answer.extend_from_slice(&(these.len() as u16).to_be_bytes());
        answer.extend_from_slice(&[0, 0]);
        for mapping in these {
            let mut octets = [0; 8];
            mapping.encode(&mut octets);
            answer.extend_from_slice(&octets);
        }
        (AemStatus::SUCCESS, answer, false)
    }

    /// ADD_AUDIO_MAPPINGS and REMOVE_AUDIO_MAPPINGS on a stream port
    /// input: each mapping its own stream's channel to one of its
    /// clusters, a cluster taking one channel at most; all of them or
    /// none.
    fn change_mappings(
        &mut self,
        command_type: AemCommandType,
        payload: &[u8],
    ) -> (AemStatus, Vec<u8>, bool) {
        let echo = Vec::from(payload);
        let Ok(changed) = crate::aem::AudioMappings::decode(payload) else {
            return (AemStatus::BAD_ARGUMENTS, echo, false);
        };
        if changed.descriptor_type == DescriptorType::STREAM_PORT_OUTPUT {
            return (AemStatus::NOT_SUPPORTED, echo, false);
        }
        let port = self
            .model
            .ports()
            .into_iter()
            .find(|port| port.input && port.index == changed.index);
        let (Some(port), DescriptorType::STREAM_PORT_INPUT) = (port, changed.descriptor_type)
        else {
            return (AemStatus::NO_SUCH_DESCRIPTOR, echo, false);
        };
        let add = command_type == AemCommandType::ADD_AUDIO_MAPPINGS;
        let mut mappings = self.input_mappings[usize::from(port.index)].clone();
        for mapping in changed.mappings() {
            let fits = mapping.stream_index == port.stream
                && mapping.stream_channel < port.channels
                && mapping.cluster_offset < port.channels
                && mapping.cluster_channel == 0;
            if !fits {
                return (AemStatus::BAD_ARGUMENTS, echo, false);
            }
            if add {
                let taken = mappings.iter().any(|other| {
                    other.cluster_offset == mapping.cluster_offset && *other != mapping
                });
                if taken {
                    return (AemStatus::BAD_ARGUMENTS, echo, false);
                }
                if !mappings.contains(&mapping) {
                    mappings.push(mapping);
                }
            } else {
                mappings.retain(|other| *other != mapping);
            }
        }
        mappings.sort();
        if mappings != self.input_mappings[usize::from(port.index)] {
            self.input_mappings[usize::from(port.index)] = mappings;
            self.events
                .push_back(EntityEvent::MappingsChanged { index: port.stream });
        }
        (AemStatus::SUCCESS, echo, true)
    }

    fn stream_format(
        &mut self,
        command_type: AemCommandType,
        payload: &[u8],
    ) -> (AemStatus, Vec<u8>, bool) {
        let echo = Vec::from(payload);
        if payload.len() < 4 {
            return (AemStatus::BAD_ARGUMENTS, echo, false);
        }
        let descriptor_type = DescriptorType(read_u16(payload, 0));
        let index = read_u16(payload, 2);
        let streams = match descriptor_type {
            DescriptorType::STREAM_INPUT => &mut self.model.inputs,
            DescriptorType::STREAM_OUTPUT => &mut self.model.outputs,
            _ => return (AemStatus::NO_SUCH_DESCRIPTOR, echo, false),
        };
        let Some(stream) = streams.get_mut(usize::from(index)) else {
            return (AemStatus::NO_SUCH_DESCRIPTOR, echo, false);
        };
        let mut answer = Vec::from(&payload[..4]);
        if command_type == AemCommandType::SET_STREAM_FORMAT {
            if payload.len() < 12 {
                return (AemStatus::BAD_ARGUMENTS, echo, false);
            }
            let format = StreamFormat(read_u64(payload, 4));
            if !stream.formats.contains(&format) {
                answer.extend_from_slice(&stream.current_format.0.to_be_bytes());
                return (AemStatus::BAD_ARGUMENTS, answer, false);
            }
            if format != stream.current_format {
                stream.current_format = format;
                self.events.push_back(EntityEvent::StreamFormatChanged {
                    descriptor_type,
                    index,
                    format,
                });
                // A format at another rate takes the entity to that rate,
                // and its other streams with it; a media clock's base
                // frequency is no audio rate.
                if let Some(rate) = format.sample_rate()
                    && !format.is_clock()
                    && rate != self.model.current_sampling_rate
                    && self.model.sampling_rates.contains(&rate)
                {
                    self.model.current_sampling_rate = rate;
                    self.events
                        .push_back(EntityEvent::SamplingRateChanged(rate));
                    let mut payload = Vec::from(DescriptorType::AUDIO_UNIT.0.to_be_bytes());
                    payload.extend_from_slice(&[0, 0]);
                    payload.extend_from_slice(&rate.to_be_bytes());
                    self.notify(AemCommandType::SET_SAMPLING_RATE, &payload, None);
                    self.follow_rate(rate, Some((descriptor_type, index)));
                }
            }
        }
        let stream = match descriptor_type {
            DescriptorType::STREAM_INPUT => &self.model.inputs,
            _ => &self.model.outputs,
        };
        answer.extend_from_slice(&stream[usize::from(index)].current_format.0.to_be_bytes());
        (AemStatus::SUCCESS, answer, true)
    }

    /// Moves every stream but `except` to the format that packs samples as
    /// its current one does at `rate`, telling registered controllers.
    fn follow_rate(&mut self, rate: u32, except: Option<(DescriptorType, u16)>) {
        let mut moved = Vec::new();
        for (descriptor_type, streams) in [
            (DescriptorType::STREAM_INPUT, &mut self.model.inputs),
            (DescriptorType::STREAM_OUTPUT, &mut self.model.outputs),
        ] {
            for (index, stream) in streams.iter_mut().enumerate() {
                let index = index as u16;
                if except == Some((descriptor_type, index))
                    || stream.current_format.sample_rate() == Some(rate)
                {
                    continue;
                }
                let current = stream.current_format;
                if let Some(format) = stream.formats.iter().copied().find(|format| {
                    format.sample_rate() == Some(rate) && format.same_but_rate(current)
                }) {
                    stream.current_format = format;
                    moved.push((descriptor_type, index, format));
                }
            }
        }
        for (descriptor_type, index, format) in moved {
            self.events.push_back(EntityEvent::StreamFormatChanged {
                descriptor_type,
                index,
                format,
            });
            let mut payload = Vec::from(descriptor_type.0.to_be_bytes());
            payload.extend_from_slice(&index.to_be_bytes());
            payload.extend_from_slice(&format.0.to_be_bytes());
            self.notify(AemCommandType::SET_STREAM_FORMAT, &payload, None);
        }
    }

    fn name(&mut self, command_type: AemCommandType, payload: &[u8]) -> (AemStatus, Vec<u8>, bool) {
        let echo = Vec::from(payload);
        if payload.len() < 8 {
            return (AemStatus::BAD_ARGUMENTS, echo, false);
        }
        let descriptor_type = DescriptorType(read_u16(payload, 0));
        let index = read_u16(payload, 2);
        let name_index = read_u16(payload, 4);
        let model = &mut self.model;
        let slot = match (descriptor_type, index, name_index) {
            (DescriptorType::ENTITY, 0, 0) => Some(&mut model.entity_name),
            (DescriptorType::ENTITY, 0, 1) => Some(&mut model.group_name),
            (DescriptorType::STREAM_INPUT, index, 0) => model
                .inputs
                .get_mut(usize::from(index))
                .map(|stream| &mut stream.name),
            (DescriptorType::STREAM_OUTPUT, index, 0) => model
                .outputs
                .get_mut(usize::from(index))
                .map(|stream| &mut stream.name),
            _ => None,
        };
        let Some(slot) = slot else {
            return (AemStatus::NOT_SUPPORTED, echo, false);
        };
        if command_type == AemCommandType::SET_NAME {
            let Some(field) = payload.get(8..8 + NAME_LENGTH) else {
                return (AemStatus::BAD_ARGUMENTS, echo, false);
            };
            let text = crate::descriptor::aem_string(field);
            if text != slot.as_str() {
                *slot = text.into();
                self.events.push_back(EntityEvent::NameChanged);
            }
        }
        let mut answer = Vec::from(&payload[..8]);
        answer.extend_from_slice(&name_field(slot));
        (AemStatus::SUCCESS, answer, true)
    }

    /// A GET_AVB_INFO answer for AVB_INTERFACE `index`.
    fn avb_info(&self, index: u16) -> Vec<u8> {
        let mut flags = AvbInfoFlags::GPTP_ENABLED | AvbInfoFlags::SRP_ENABLED;
        if self.gptp.as_capable {
            flags |= AvbInfoFlags::AS_CAPABLE;
        }
        let mut answer = Vec::with_capacity(28);
        answer.extend_from_slice(&DescriptorType::AVB_INTERFACE.0.to_be_bytes());
        answer.extend_from_slice(&index.to_be_bytes());
        answer.extend_from_slice(&self.gptp.grandmaster.0.to_be_bytes());
        answer.extend_from_slice(&self.gptp.propagation_delay.to_be_bytes());
        answer.push(self.gptp.domain);
        answer.push(flags.0);
        // The SR classes: A on priority 3 and B on 2, both on VLAN 2.
        answer.extend_from_slice(&2u16.to_be_bytes());
        answer.extend_from_slice(&[1, 3, 0, 2, 0, 2, 0, 2]);
        answer
    }

    /// A GET_STREAM_INFO answer, in Milan's form (Milan 1.3, 5.4.2.10).
    fn stream_info(&self, descriptor_type: DescriptorType, index: u16) -> Option<Vec<u8>> {
        let mut flags = StreamInfoFlags::STREAM_FORMAT_VALID;
        let mut stream_id = StreamId(0);
        let mut destination = MacAddress([0; 6]);
        let mut latency = 0;
        let mut failure = (0, 0);
        let mut flags_ex = 0u32;
        let mut probing = 0u8;
        let format;
        match descriptor_type {
            DescriptorType::STREAM_OUTPUT => {
                let output = self.outputs.get(usize::from(index))?;
                format = self.model.outputs[usize::from(index)].current_format;
                stream_id = output.stream_id;
                destination = output.destination;
                flags |= StreamInfoFlags::STREAM_ID_VALID | StreamInfoFlags::STREAM_VLAN_ID_VALID;
                if destination != UNADDRESSED {
                    flags |= StreamInfoFlags::STREAM_DEST_MAC_VALID;
                }
                if output.class_b {
                    flags |= StreamInfoFlags::CLASS_B;
                }
                if output.reservation.registering {
                    flags_ex |= 1;
                }
            }
            DescriptorType::STREAM_INPUT => {
                let input = self.inputs.get(usize::from(index))?;
                format = self.model.inputs[usize::from(index)].current_format;
                if let Some(binding) = input.binding {
                    flags |= StreamInfoFlags::CONNECTED;
                    flags |= StreamInfoFlags(u32::from(binding.flags.0));
                    // Probing ACTIVE until it settles, then COMPLETED.
                    probing = 2;
                }
                if let Some(Probe::Settled(stream)) = input.probe {
                    probing = 3;
                    stream_id = stream.stream_id;
                    destination = stream.destination;
                    flags |= StreamInfoFlags::STREAM_ID_VALID
                        | StreamInfoFlags::STREAM_DEST_MAC_VALID
                        | StreamInfoFlags::STREAM_VLAN_ID_VALID;
                    if input.reservation.talker_registered {
                        flags |= StreamInfoFlags::MSRP_ACC_LAT_VALID;
                        latency = input.reservation.accumulated_latency;
                    }
                    if let Some(found) = input.reservation.failure {
                        flags |= StreamInfoFlags::MSRP_FAILURE_VALID;
                        failure = found;
                    }
                }
                if input.reservation.registering {
                    flags_ex |= 1;
                }
            }
            _ => return None,
        }
        let acmp_status = match descriptor_type {
            DescriptorType::STREAM_INPUT => self.inputs[usize::from(index)].probe_status.0 & 0x1f,
            _ => 0,
        };
        let mut answer = Vec::with_capacity(56);
        answer.extend_from_slice(&descriptor_type.0.to_be_bytes());
        answer.extend_from_slice(&index.to_be_bytes());
        answer.extend_from_slice(&flags.0.to_be_bytes());
        answer.extend_from_slice(&format.0.to_be_bytes());
        answer.extend_from_slice(&stream_id.0.to_be_bytes());
        answer.extend_from_slice(&latency.to_be_bytes());
        answer.extend_from_slice(&destination.0);
        answer.push(failure.0);
        answer.push(0);
        answer.extend_from_slice(&failure.1.to_be_bytes());
        answer.extend_from_slice(&2u16.to_be_bytes()); // stream_vlan_id
        answer.extend_from_slice(&[0, 0]);
        answer.extend_from_slice(&flags_ex.to_be_bytes());
        answer.push((probing << 5) | acmp_status);
        answer.extend_from_slice(&[0, 0, 0]);
        Some(answer)
    }

    /// A GET_COUNTERS answer. Counters this entity does not keep read
    /// zero but are marked valid where Milan requires them.
    fn counters(&self, descriptor_type: DescriptorType, index: u16) -> Option<Vec<u8>> {
        let valid: u32 = match descriptor_type {
            DescriptorType::AVB_INTERFACE if index == 0 => 0b10_0011,
            DescriptorType::CLOCK_DOMAIN if index == 0 => 0b11,
            DescriptorType::STREAM_INPUT if usize::from(index) < self.inputs.len() => 0x0fff,
            DescriptorType::STREAM_OUTPUT if usize::from(index) < self.outputs.len() => 0xff,
            _ => return None,
        };
        let block = match descriptor_type {
            DescriptorType::STREAM_INPUT => self.inputs[usize::from(index)].counters,
            DescriptorType::STREAM_OUTPUT => self.outputs[usize::from(index)].counters,
            DescriptorType::CLOCK_DOMAIN => self.clock_counters,
            _ => [0; 32],
        };
        let mut answer = Vec::with_capacity(136);
        answer.extend_from_slice(&descriptor_type.0.to_be_bytes());
        answer.extend_from_slice(&index.to_be_bytes());
        answer.extend_from_slice(&valid.to_be_bytes());
        for counter in block {
            answer.extend_from_slice(&counter.to_be_bytes());
        }
        Some(answer)
    }

    fn handle_vendor_unique(&mut self, source: MacAddress, command: &VendorUniquePdu<'_>) {
        let mut data = Vec::new();
        let mut status = MvuStatus::NOT_IMPLEMENTED.0;
        if command.protocol_id == STATUS_PROTOCOL_ID && command.payload.len() >= 4 {
            let command_type = LiteCommandType(read_u16(command.payload, 0) & 0x7fff);
            let interface = read_u16(command.payload, 2);
            data.extend_from_slice(&command.payload[..2]);
            match self.lite {
                Some(lite) if command_type == LiteCommandType::GET_LITE_STATUS => {
                    if interface == lite.interface {
                        status = MvuStatus::SUCCESS.0;
                        data.extend_from_slice(&lite.to_bytes());
                    } else {
                        status = AemStatus::NO_SUCH_DESCRIPTOR.0;
                        data.extend_from_slice(&command.payload[2..]);
                    }
                }
                Some(lite) if command_type == LiteCommandType::SET_LITE_CONFIG => {
                    let flags = command.payload.get(4).copied().map(LiteConfigFlags);
                    let known = LiteConfigFlags::ESCALATION_ALLOWED.0;
                    status = if interface != lite.interface {
                        AemStatus::NO_SUCH_DESCRIPTOR.0
                    } else if command.payload.len() < 8
                        || flags.is_none_or(|flags| flags.0 & !known != 0)
                    {
                        AemStatus::BAD_ARGUMENTS.0
                    } else if !self.may_change(command.header.controller_entity_id) {
                        AemStatus::ENTITY_LOCKED.0
                    } else {
                        MvuStatus::SUCCESS.0
                    };
                    match flags {
                        Some(flags) if status == MvuStatus::SUCCESS.0 => {
                            let allowed = flags.contains(LiteConfigFlags::ESCALATION_ALLOWED);
                            let mut changed = lite;
                            changed.flags = if allowed {
                                changed.flags.union(LiteFlags::ESCALATION_ALLOWED)
                            } else {
                                changed.flags.difference(LiteFlags::ESCALATION_ALLOWED)
                            };
                            if changed != lite {
                                self.events.push_back(EntityEvent::LiteConfigChanged {
                                    escalation_allowed: allowed,
                                });
                                self.set_lite_status(changed);
                            }
                            data.extend_from_slice(&interface.to_be_bytes());
                            data.extend_from_slice(&[flags.0, 0, 0, 0]);
                        }
                        _ => data.extend_from_slice(&command.payload[2..]),
                    }
                }
                _ => data.extend_from_slice(&command.payload[2..]),
            }
        } else if command.protocol_id == MVU_PROTOCOL_ID && command.payload.len() >= 2 {
            let command_type = MvuCommandType(read_u16(command.payload, 0) & 0x7fff);
            data.extend_from_slice(&command.payload[..2]);
            if command_type == MvuCommandType::GET_MILAN_INFO {
                status = MvuStatus::SUCCESS.0;
                data.extend_from_slice(&[0, 0]);
                data.extend_from_slice(&1u32.to_be_bytes());
                data.extend_from_slice(&MilanFeatures::default().0.to_be_bytes());
                data.extend_from_slice(&[0; 4]);
                data.extend_from_slice(&MILAN_VERSION);
            } else {
                data.extend_from_slice(&command.payload[2..]);
            }
        } else {
            data.extend_from_slice(command.payload);
        }
        let pdu = VendorUniquePdu {
            header: AecpHeader {
                message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
                status,
                ..command.header
            },
            protocol_id: command.protocol_id,
            payload: &data,
        };
        let mut out = [0; 200];
        if let Ok(length) = pdu.encode(&mut out) {
            self.outgoing.push_back((source, out[..length].to_vec()));
        }
    }
}

/// ACMP commands addressed to a talker.
fn is_talker_command(message: AcmpMessageType) -> bool {
    matches!(
        message,
        AcmpMessageType::PROBE_TX_COMMAND
            | AcmpMessageType::DISCONNECT_TX_COMMAND
            | AcmpMessageType::GET_TX_STATE_COMMAND
            | AcmpMessageType::GET_TX_CONNECTION_COMMAND
    )
}

/// Stream `stream`'s `channels` channels to its clusters one to one.
fn one_to_one(stream: u16, channels: u16) -> Vec<AudioMapping> {
    (0..channels)
        .map(|channel| AudioMapping {
            stream_index: stream,
            stream_channel: channel,
            cluster_offset: channel,
            cluster_channel: 0,
        })
        .collect()
}
