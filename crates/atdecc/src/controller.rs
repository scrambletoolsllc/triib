//! The controller role, without I/O: the caller passes in received frames
//! and the time, and takes back frames to send, events and the next
//! deadline.
//!
//! It discovers entities (IEEE 1722.1-2021, 6.2.6), reads the descriptors
//! of those that support AEM, asks Milan entities for their Milan
//! information, registers for unsolicited notifications and re-reads what
//! they report changed, answers the CONTROLLER_AVAILABLE checks of the
//! entities it registered with (Milan 1.3, 5.4.5.4), and can advertise
//! itself.

use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec::Vec;
use core::time::Duration;

use avb_mrp::msrp;
use avb_net::MacAddress;

use crate::acmp::{AcmpFlags, AcmpMessageType, AcmpStatus, Acmpdu};
use crate::adp::{AdpMessageType, Adpdu, ControllerCapabilities, EntityCapabilities};
use crate::aecp::{AecpMessageType, AemCommandType, AemPdu, AemStatus, VendorUniquePdu};
use crate::aem::{
    self, Addressing, AsPath, AudioMap, AudioMapping, AudioMappings, AvbInfo, Counters,
    DynamicQuery, MappingChange, MaxTransitTime, ReadDescriptorResponse, SetClockSource, SetName,
    SetSamplingRate, SetStreamFormat, StreamInfo, StreamInfoFlags,
};
use crate::cache::{CachedModel, ModelKey};
use crate::descriptor::{
    AudioUnitDescriptor, ConfigurationDescriptor, DescriptorType, EntityDescriptor,
    LocaleDescriptor, SamplingRate, StreamPortDescriptor,
};
use crate::error::{DecodeError, EncodeError};
use crate::id::{EntityId, EntityModelId};
use crate::lite::{self, CvuMessage, LiteCommandType, LiteMessage, LiteStatus};
use crate::model::{Binding, EntityModel, EnumerationFailure, EnumerationState, TxState, Wireless};
use crate::mvu::{self, MediaClockReference, MilanInfo, MvuCommandType, MvuMessage};
use crate::pdu::{self, Pdu};
use crate::stream_format::StreamFormat;
use crate::time::Instant;
use crate::wireless::{self, WirelessCommandType, WirelessMessage, WirelessStatus};

/// How long a command waits for its response before its one retry
/// (IEEE 1722.1-2021, 9.3.2.6).
pub const COMMAND_TIMEOUT: Duration = Duration::from_millis(250);

/// How long a CVU SRP declaration lasts without a refresh: well past
/// MSRP's LeaveAll period of 10 s (IEEE 802.1Q-2022, 10.7.11).
const CVU_LIFETIME: Duration = Duration::from_secs(30);

/// The most descriptors read from one entity, against counts no entity
/// could mean.
const MAX_DESCRIPTORS: usize = 4096;

/// Top level descriptors read from each configuration; STREAM_PORT,
/// AUDIO_CLUSTER, AUDIO_MAP and STRINGS are read as children of these.
const READ_TYPES: [DescriptorType; 11] = [
    DescriptorType::AUDIO_UNIT,
    DescriptorType::STREAM_INPUT,
    DescriptorType::STREAM_OUTPUT,
    DescriptorType::JACK_INPUT,
    DescriptorType::JACK_OUTPUT,
    DescriptorType::AVB_INTERFACE,
    DescriptorType::CLOCK_SOURCE,
    DescriptorType::MEMORY_OBJECT,
    DescriptorType::LOCALE,
    DescriptorType::CONTROL,
    DescriptorType::CLOCK_DOMAIN,
];

/// Descriptors whose every field stays the same for entities of a model,
/// which a cached model can stand in for without asking again.
const STATIC_TYPES: [DescriptorType; 5] = [
    DescriptorType::STRINGS,
    DescriptorType::LOCALE,
    DescriptorType::AUDIO_MAP,
    DescriptorType::STREAM_PORT_INPUT,
    DescriptorType::STREAM_PORT_OUTPUT,
];

/// Descriptors holding what differs between entities of a model, such as
/// an interface's MAC address and clock identity or a clock source's
/// identifier; always read from the entity.
const OWN_TYPES: [DescriptorType; 4] = [
    DescriptorType::ENTITY,
    DescriptorType::CONFIGURATION,
    DescriptorType::AVB_INTERFACE,
    DescriptorType::CLOCK_SOURCE,
];

/// Unanswered AVB Wireless status queries before an entity is taken not
/// to answer them: a Wi-Fi hop loses frames, and a bridge under load can
/// stall for seconds, so one silence is not an answer.
const WIRELESS_MISSES: u8 = 3;

/// The most values one GET_DYNAMIC_INFO asks for.
const MAX_DYNAMIC_QUERIES: usize = 16;

/// The most parts a stream port's dynamic mappings are read in, against
/// counts no entity could mean; Milan's parts hold up to 176 mappings.
const MAX_MAP_PARTS: u16 = 256;

/// How the controller identifies and behaves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// The controller's entity ID, in the commands it sends.
    pub entity_id: EntityId,
    /// Read the descriptors of each entity that supports AEM.
    pub enumerate: bool,
    /// Register for unsolicited notifications once an entity is read.
    pub register_unsolicited: bool,
    /// Ask each AVB interface for its gPTP and SRP state, its gPTP path
    /// and its counters once an entity is read.
    pub network_info: bool,
    /// Read each stream port's dynamic mappings once an entity is read.
    pub read_mappings: bool,
    /// Ask each Milan entity's clock domains for their media clock
    /// reference priority and domain name once it is read.
    pub media_clock_info: bool,
    /// Ask each clock domain, stream input and stream output for its
    /// counters once an entity is read; Milan entities then report
    /// changes.
    pub read_counters: bool,
    /// Ask each Milan entity's stream outputs for their max transit time
    /// once it is read.
    pub read_transit_times: bool,
    /// Read an entity's other configurations and their streams once it is
    /// read, to tell them apart.
    pub read_configurations: bool,
    /// Ask each entity for its AVB Lite status once it is read, and keep
    /// asking those that answer every `lite_poll`, zero for never, to
    /// follow their PTP offset.
    pub lite_status: bool,
    pub lite_poll: Duration,
    /// Ask each entity's AVB interfaces for their AVB Wireless status once
    /// it is read, and keep asking the wireless ones every
    /// `wireless_poll`, zero for never, to follow their link and time.
    pub wireless_status: bool,
    pub wireless_poll: Duration,
    /// The controller's own MAC address, where entities advertised from
    /// it are out of its reach: it does not read them.
    pub own_mac: Option<MacAddress>,
    /// Advertise the controller with ADP.
    pub advertise: Option<Advertise>,
    /// The first sequence ID of AEM and of MVU commands.
    pub first_sequence_id: u16,
    /// Seeds the random delays ADP calls for.
    pub random_seed: u64,
}

impl Config {
    pub fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            enumerate: true,
            register_unsolicited: true,
            network_info: true,
            read_mappings: true,
            media_clock_info: true,
            read_counters: true,
            read_transit_times: true,
            read_configurations: true,
            lite_status: true,
            lite_poll: Duration::from_secs(5),
            wireless_status: true,
            wireless_poll: Duration::from_secs(5),
            own_mac: None,
            advertise: None,
            first_sequence_id: 0,
            random_seed: entity_id.0,
        }
    }
}

/// How the controller advertises itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Advertise {
    pub entity_model_id: EntityModelId,
    /// In units of 2 seconds, 1 to 31.
    pub valid_time: u8,
}

/// An entity the controller has heard advertise itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiscoveredEntity {
    /// Its latest ENTITY_AVAILABLE.
    pub adp: Adpdu,
    /// The source address of that advertisement.
    pub mac: MacAddress,
    pub first_seen: Instant,
    pub last_seen: Instant,
    /// When it is forgotten unless it advertises again.
    pub expires: Instant,
}

impl DiscoveredEntity {
    pub fn entity_id(&self) -> EntityId {
        self.adp.entity_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineReason {
    /// It sent ENTITY_DEPARTING.
    Departed,
    /// It stopped advertising for longer than its valid time.
    TimedOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// A new entity, or one back after going offline.
    EntityOnline(EntityId),
    /// An entity advertised different capabilities, BTC, configuration or
    /// address.
    EntityChanged(EntityId),
    /// An entity's available_index went back, so it restarted; its model
    /// is read again.
    EntityRestarted(EntityId),
    EntityOffline(EntityId, OfflineReason),
    /// Reading an entity's descriptors began.
    EnumerationStarted(EntityId),
    /// All of an entity's descriptors that could be read were.
    EntityEnumerated(EntityId),
    /// The ENTITY or CONFIGURATION descriptor could not be read.
    EnumerationFailed(EntityId, EnumerationFailure),
    /// A descriptor read again after a notification changed, or a stream's
    /// binding or info did.
    EntityModelChanged(EntityId),
    /// A command the caller asked for finished.
    CommandFinished(CommandId, Outcome),
}

/// Identifies a command the caller asked for, in
/// [`Event::CommandFinished`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CommandId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Done,
    /// The entity answered with an error.
    Refused(Refusal),
    /// No response, after a retry.
    NoResponse,
    /// The entity is not online, went away before answering, or lacks what
    /// the command needs, such as an identify control.
    NotPossible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    Acmp(AcmpStatus),
    Aem(AemStatus),
}

/// A frame for the caller to send: `length` octets at the start of the
/// buffer given to [`Controller::poll_transmit`], on the AVTP ethertype.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Transmit {
    pub destination: MacAddress,
    pub length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Request {
    ReadDescriptor {
        descriptor_type: DescriptorType,
        index: u16,
        /// Read again after a notification, not as part of enumeration.
        refresh: bool,
    },
    GetMilanInfo,
    /// GET_MEDIA_CLOCK_REFERENCE_INFO for a CLOCK_DOMAIN.
    GetMediaClockReference {
        domain: u16,
    },
    RegisterUnsolicited,
    GetRxState {
        input: u16,
    },
    GetStreamInfo {
        descriptor_type: DescriptorType,
        index: u16,
    },
    GetAvbInfo {
        index: u16,
    },
    GetAsPath {
        index: u16,
    },
    GetCounters {
        descriptor_type: DescriptorType,
        index: u16,
    },
    Bind {
        talker: EntityId,
        talker_unique_id: u16,
        input: u16,
        /// The talker's stream is class B, so the connect says so.
        class_b: bool,
        command: CommandId,
    },
    Unbind {
        talker: EntityId,
        talker_unique_id: u16,
        input: u16,
        command: CommandId,
    },
    Identify {
        control: u16,
        on: bool,
        command: Option<CommandId>,
    },
    /// SET_NAME; its configuration is the entity's when it is sent.
    SetName {
        set: SetName,
        command: CommandId,
    },
    SetStreamFormat {
        set: SetStreamFormat,
        command: CommandId,
    },
    SetSamplingRate {
        set: SetSamplingRate,
        command: CommandId,
    },
    SetClockSource {
        set: SetClockSource,
        command: CommandId,
    },
    SetConfiguration {
        configuration: u16,
        command: CommandId,
    },
    /// A descriptor of another configuration than the one read.
    ReadOther {
        configuration: u16,
        descriptor_type: DescriptorType,
        index: u16,
    },
    /// One part of a stream port's dynamic mappings.
    GetAudioMap {
        descriptor_type: DescriptorType,
        index: u16,
        map_index: u16,
    },
    /// ADD_AUDIO_MAPPINGS or REMOVE_AUDIO_MAPPINGS on a stream port; the
    /// mappings wait in the controller's `mapping_changes`.
    ChangeMappings {
        change: MappingChange,
        descriptor_type: DescriptorType,
        index: u16,
        command: CommandId,
    },
    /// SET_CONTROL; the values wait in the controller's `control_values`.
    SetControl {
        index: u16,
        command: CommandId,
    },
    /// GET_TX_STATE to a talker about a stream output.
    GetTxState {
        output: u16,
        command: Option<CommandId>,
    },
    /// DISCONNECT_TX to a talker, for a listener's stream input, without
    /// the listener taking part.
    DisconnectTx {
        output: u16,
        listener: EntityId,
        listener_unique_id: u16,
        command: CommandId,
    },
    GetMaxTransitTime {
        output: u16,
        command: Option<CommandId>,
    },
    SetMaxTransitTime {
        set: MaxTransitTime,
        command: CommandId,
    },
    /// The AVB Lite status of an AVB_INTERFACE (AVB Lite profile, 2.4).
    GetLiteStatus {
        interface: u16,
    },
    /// How an AVB_INTERFACE runs AVB Lite (SET_LITE_CONFIG).
    SetLiteConfig {
        interface: u16,
        flags: u8,
        command: CommandId,
    },
    /// The AVB Wireless status of an AVB_INTERFACE (AVB Wireless profile,
    /// 5.1).
    GetWirelessStatus {
        interface: u16,
    },
    /// An AVB_INTERFACE's AVB Wireless configuration (SET_WIRELESS_CONFIG).
    SetWirelessConfig {
        interface: u16,
        flags: u8,
        command: CommandId,
    },
    /// READ_DESCRIPTOR the caller asked for, outside enumeration.
    ReadForCaller {
        descriptor_type: DescriptorType,
        index: u16,
        command: CommandId,
    },
    /// What changes on an entity whose descriptors came from the cache.
    GetDynamicInfo {
        queries: [DynamicQuery; MAX_DYNAMIC_QUERIES],
        count: u8,
    },
}

/// The protocols whose commands are numbered apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Channel {
    Aem,
    Mvu,
    Acmp,
}

impl Request {
    fn channel(self) -> Channel {
        match self {
            // Vendor unique commands, numbered apart from AEM.
            Request::GetMilanInfo
            | Request::GetMediaClockReference { .. }
            | Request::GetLiteStatus { .. }
            | Request::SetLiteConfig { .. }
            | Request::GetWirelessStatus { .. }
            | Request::SetWirelessConfig { .. } => Channel::Mvu,
            Request::GetRxState { .. }
            | Request::Bind { .. }
            | Request::Unbind { .. }
            | Request::GetTxState { .. }
            | Request::DisconnectTx { .. } => Channel::Acmp,
            _ => Channel::Aem,
        }
    }

    /// The ACMP message the request is sent as.
    fn acmp_message(self) -> Option<AcmpMessageType> {
        match self {
            Request::GetRxState { .. } => Some(AcmpMessageType::GET_RX_STATE_COMMAND),
            Request::Bind { .. } => Some(AcmpMessageType::CONNECT_RX_COMMAND),
            Request::GetTxState { .. } => Some(AcmpMessageType::GET_TX_STATE_COMMAND),
            Request::DisconnectTx { .. } => Some(AcmpMessageType::DISCONNECT_TX_COMMAND),
            Request::Unbind { .. } => Some(AcmpMessageType::DISCONNECT_RX_COMMAND),
            _ => None,
        }
    }

    /// The caller's command, for requests the caller asked for.
    fn command(self) -> Option<CommandId> {
        match self {
            Request::Bind { command, .. }
            | Request::Unbind { command, .. }
            | Request::SetName { command, .. }
            | Request::SetStreamFormat { command, .. }
            | Request::SetSamplingRate { command, .. }
            | Request::SetClockSource { command, .. }
            | Request::SetConfiguration { command, .. }
            | Request::SetControl { command, .. }
            | Request::DisconnectTx { command, .. }
            | Request::SetMaxTransitTime { command, .. }
            | Request::ReadForCaller { command, .. }
            | Request::SetLiteConfig { command, .. }
            | Request::SetWirelessConfig { command, .. }
            | Request::ChangeMappings { command, .. } => Some(command),
            Request::Identify { command, .. }
            | Request::GetTxState { command, .. }
            | Request::GetMaxTransitTime { command, .. } => command,
            _ => None,
        }
    }
}

/// A command waiting for its response.
#[derive(Debug, Clone, Copy)]
struct Inflight {
    entity_id: EntityId,
    request: Request,
    deadline: Instant,
    retried: bool,
}

/// The commands queued for one entity; one is in flight at a time.
#[derive(Debug, Default)]
struct Session {
    queue: VecDeque<Request>,
    inflight: Option<(Channel, u16)>,
    /// Descriptors queued or read during this enumeration.
    requested: usize,
}

/// A small xorshift generator for ADP's random delays.
#[derive(Debug, Clone, Copy)]
struct Random(u64);

impl Random {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    /// A duration from zero to `limit`.
    fn up_to(&mut self, limit: Duration) -> Duration {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        let millis = limit.as_millis() as u64;
        Duration::from_millis(if millis == 0 {
            0
        } else {
            self.0 % (millis + 1)
        })
    }
}

#[derive(Debug)]
struct Advertiser {
    config: Advertise,
    available_index: u32,
    next: Option<Instant>,
    started: bool,
}

impl Advertiser {
    fn valid(&self) -> Duration {
        Duration::from_secs(2 * u64::from(self.config.valid_time.clamp(1, 31)))
    }

    /// The longest random delay before advertising: a fifth of the valid
    /// time (6.2.4.2.2), and no more than a second, so other controllers
    /// see triib promptly.
    fn delay_limit(&self) -> Duration {
        (self.valid() / 5).min(Duration::from_secs(1))
    }
}

pub struct Controller {
    config: Config,
    entities: BTreeMap<EntityId, DiscoveredEntity>,
    models: BTreeMap<EntityId, EntityModel>,
    sessions: BTreeMap<EntityId, Session>,
    inflight: BTreeMap<(Channel, u16), Inflight>,
    next_aem_sequence: u16,
    next_mvu_sequence: u16,
    next_acmp_sequence: u16,
    next_command: u32,
    /// When to turn each identifying entity's identify control off.
    identify_off: BTreeMap<EntityId, (Instant, u16)>,
    /// When to ask the entities that answer it for their AVB Lite status
    /// again.
    next_lite_poll: Option<Instant>,
    /// When to ask the wireless interfaces for their AVB Wireless status
    /// again.
    next_wireless_poll: Option<Instant>,
    /// Entity models read before, by what they are for.
    cache: BTreeMap<ModelKey, CachedModel>,
    /// The mappings of each mapping change queued or in flight.
    mapping_changes: BTreeMap<CommandId, Vec<AudioMapping>>,
    /// The values of each SET_CONTROL waiting to go.
    control_values: BTreeMap<CommandId, Vec<u8>>,
    advertiser: Option<Advertiser>,
    random: Random,
    outgoing: VecDeque<(MacAddress, Vec<u8>)>,
    events: VecDeque<Event>,
    malformed: u64,
}

impl Controller {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            entities: BTreeMap::new(),
            models: BTreeMap::new(),
            sessions: BTreeMap::new(),
            inflight: BTreeMap::new(),
            next_aem_sequence: config.first_sequence_id,
            next_mvu_sequence: config.first_sequence_id,
            next_acmp_sequence: config.first_sequence_id,
            next_command: 0,
            identify_off: BTreeMap::new(),
            next_lite_poll: None,
            next_wireless_poll: None,
            cache: BTreeMap::new(),
            mapping_changes: BTreeMap::new(),
            control_values: BTreeMap::new(),
            advertiser: config.advertise.map(|advertise| Advertiser {
                config: advertise,
                available_index: 0,
                next: None,
                started: false,
            }),
            random: Random::new(config.random_seed),
            outgoing: VecDeque::new(),
            events: VecDeque::new(),
            malformed: 0,
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.config.entity_id
    }

    /// Asks `entity`, or every entity when `None`, to advertise itself
    /// now rather than at its next periodic advertisement.
    pub fn discover(&mut self, entity: Option<EntityId>) {
        let adpdu = Adpdu::discover(entity.unwrap_or_default());
        self.queue_adp(&adpdu);
    }

    /// Reads an entity's descriptors again.
    pub fn enumerate(&mut self, now: Instant, entity_id: EntityId) {
        if self.entities.contains_key(&entity_id) {
            self.start_enumeration(now, entity_id);
        }
    }

    /// Advertises that the controller is leaving and stops advertising.
    pub fn depart(&mut self) {
        let Some(advertiser) = &mut self.advertiser else {
            return;
        };
        advertiser.next = None;
        advertiser.available_index = 0;
        let adpdu = self.own_adpdu(AdpMessageType::ENTITY_DEPARTING, 0);
        self.queue_adp(&adpdu);
    }

    /// Handles the payload of a frame received on the AVTP ethertype from
    /// `source`. Frames that do not decode are counted and their error
    /// returned; the controller's state is unchanged by them.
    pub fn handle_frame(
        &mut self,
        now: Instant,
        source: MacAddress,
        bytes: &[u8],
    ) -> Result<(), DecodeError> {
        self.start_advertising(now);
        let result = match pdu::decode(bytes) {
            Ok(Pdu::Adp(adpdu)) => {
                self.handle_adpdu(now, source, &adpdu);
                Ok(())
            }
            Ok(Pdu::Aem(aem)) => {
                self.handle_aem(now, source, &aem);
                Ok(())
            }
            Ok(Pdu::Acmp(acmpdu)) => {
                self.handle_acmp(&acmpdu);
                Ok(())
            }
            Ok(Pdu::VendorUnique(vendor_unique)) => {
                self.handle_vendor_unique(now, &vendor_unique);
                Ok(())
            }
            Ok(_) => Ok(()),
            Err(error) => {
                self.malformed += 1;
                Err(error)
            }
        };
        self.pump(now);
        result
    }

    /// Handles an ADPDU received from `source`.
    pub fn handle_adpdu(&mut self, now: Instant, source: MacAddress, adpdu: &Adpdu) {
        let entity_id = adpdu.entity_id;
        if entity_id == self.config.entity_id {
            return;
        }
        match adpdu.message_type {
            AdpMessageType::ENTITY_AVAILABLE if entity_id.is_valid() => {
                self.entity_available(now, source, adpdu);
            }
            AdpMessageType::ENTITY_DEPARTING => {
                let known = self.entities.remove(&entity_id).is_some();
                if known {
                    self.forget(entity_id);
                    self.events
                        .push_back(Event::EntityOffline(entity_id, OfflineReason::Departed));
                }
            }
            AdpMessageType::ENTITY_DISCOVER => {
                let asked = !entity_id.is_valid() || entity_id == self.config.entity_id;
                if let Some(advertiser) = self.advertiser.as_mut().filter(|_| asked) {
                    let soon = now + self.random.up_to(advertiser.delay_limit());
                    advertiser.next = Some(advertiser.next.map_or(soon, |next| next.min(soon)));
                }
            }
            _ => {}
        }
    }

    fn entity_available(&mut self, now: Instant, source: MacAddress, adpdu: &Adpdu) {
        let entity_id = adpdu.entity_id;
        // Valid times run from 1 to 31 units of 2 s; zero would expire the
        // entity at once, so it counts as one unit.
        let expires = now + Duration::from_secs(2 * u64::from(adpdu.valid_time.max(1)));
        let Some(known) = self.entities.get_mut(&entity_id) else {
            self.entities.insert(
                entity_id,
                DiscoveredEntity {
                    adp: *adpdu,
                    mac: source,
                    first_seen: now,
                    last_seen: now,
                    expires,
                },
            );
            self.events.push_back(Event::EntityOnline(entity_id));
            if self.wants_model(adpdu) {
                self.start_enumeration(now, entity_id);
            }
            return;
        };
        let restarted = adpdu.available_index < known.adp.available_index;
        let changed = !same_advertisement(&known.adp, adpdu) || known.mac != source;
        // A new grandmaster or domain moves the entity in the network.
        let moved = adpdu.gptp_grandmaster_id != known.adp.gptp_grandmaster_id
            || adpdu.gptp_domain_number != known.adp.gptp_domain_number;
        let reconfigured = adpdu.current_configuration_index
            != known.adp.current_configuration_index
            && adpdu
                .entity_capabilities
                .contains(EntityCapabilities::AEM_CONFIGURATION_INDEX_VALID);
        // Another model, as when an entity's streams change while it
        // stays, has other descriptors to read.
        let remodeled = adpdu.entity_model_id != known.adp.entity_model_id;
        known.adp = *adpdu;
        known.mac = source;
        known.last_seen = now;
        known.expires = expires;
        if restarted {
            self.events.push_back(Event::EntityRestarted(entity_id));
        }
        if changed {
            self.events.push_back(Event::EntityChanged(entity_id));
        }
        if (restarted || reconfigured || remodeled) && self.wants_model(adpdu) {
            self.start_enumeration(now, entity_id);
        } else if moved {
            self.read_network_info(now, entity_id);
        }
    }

    /// Reads again where an entity sits in the network, each AVB
    /// interface's gPTP state and path, as not every entity says unasked
    /// when they change.
    fn read_network_info(&mut self, now: Instant, entity_id: EntityId) {
        if !self.config.network_info {
            return;
        }
        let Some(model) = self.models.get(&entity_id) else {
            return;
        };
        let interfaces: Vec<u16> = model
            .descriptors(DescriptorType::AVB_INTERFACE)
            .map(|(index, _)| index)
            .collect();
        for index in interfaces {
            self.queue_query(entity_id, Request::GetAvbInfo { index });
            self.queue_query(entity_id, Request::GetAsPath { index });
        }
        self.pump(now);
    }

    fn wants_model(&self, adpdu: &Adpdu) -> bool {
        self.config.enumerate
            && adpdu
                .entity_capabilities
                .contains(EntityCapabilities::AEM_SUPPORTED)
    }

    /// Drops everything kept about an entity apart from its discovery,
    /// finishing the caller's commands to it as not possible.
    fn forget(&mut self, entity_id: EntityId) {
        self.models.remove(&entity_id);
        self.identify_off.remove(&entity_id);
        let mut abandoned: Vec<CommandId> = Vec::new();
        if let Some(session) = self.sessions.remove(&entity_id) {
            abandoned.extend(session.queue.iter().filter_map(|request| request.command()));
        }
        self.inflight.retain(|_, inflight| {
            let keep = inflight.entity_id != entity_id;
            if !keep {
                abandoned.extend(inflight.request.command());
            }
            keep
        });
        for command in abandoned {
            self.drop_command_data(command);
            self.events
                .push_back(Event::CommandFinished(command, Outcome::NotPossible));
        }
    }

    fn start_enumeration(&mut self, now: Instant, entity_id: EntityId) {
        self.forget(entity_id);
        let Some(entity) = self.entities.get(&entity_id) else {
            return;
        };
        if self.config.own_mac == Some(entity.mac) {
            let mut model = EntityModel::reading();
            model.state = EnumerationState::Failed(EnumerationFailure::OnThisComputer);
            self.models.insert(entity_id, model);
            self.events.push_back(Event::EnumerationFailed(
                entity_id,
                EnumerationFailure::OnThisComputer,
            ));
            return;
        }
        let mut session = Session::default();
        if entity
            .adp
            .entity_capabilities
            .contains(EntityCapabilities::VENDOR_UNIQUE_SUPPORTED)
        {
            session.queue.push_back(Request::GetMilanInfo);
        }
        session.queue.push_back(Request::ReadDescriptor {
            descriptor_type: DescriptorType::ENTITY,
            index: 0,
            refresh: false,
        });
        self.sessions.insert(entity_id, session);
        self.models.insert(entity_id, EntityModel::reading());
        self.events.push_back(Event::EnumerationStarted(entity_id));
        self.pump(now);
    }

    fn handle_aem(&mut self, now: Instant, source: MacAddress, aem: &AemPdu<'_>) {
        let header = &aem.header;
        match header.message_type {
            AecpMessageType::AEM_COMMAND if header.target_entity_id == self.config.entity_id => {
                // Entities check on the controllers registered with them; a
                // controller answers commands it does not implement too.
                let status = match aem.command_type {
                    AemCommandType::CONTROLLER_AVAILABLE => AemStatus::SUCCESS,
                    _ => AemStatus::NOT_IMPLEMENTED,
                };
                let mut out = [0; 600];
                if let Ok(length) = aem::encode_response(aem, status, &mut out) {
                    self.outgoing.push_back((source, out[..length].to_vec()));
                }
            }
            AecpMessageType::AEM_RESPONSE
                if header.controller_entity_id == self.config.entity_id =>
            {
                if aem.unsolicited {
                    // Another controller chose another configuration.
                    if aem.command_type == AemCommandType::SET_CONFIGURATION {
                        self.reconfigured(now, header.target_entity_id, aem.payload);
                        return;
                    }
                    self.handle_notification(header.target_entity_id, aem);
                    return;
                }
                let key = (Channel::Aem, header.sequence_id);
                let Some(inflight) = self.inflight.get_mut(&key) else {
                    return;
                };
                if inflight.entity_id != header.target_entity_id
                    || aem.command_type != command_type_of(inflight.request)
                {
                    return;
                }
                if aem.status() == AemStatus::IN_PROGRESS {
                    inflight.deadline = now + COMMAND_TIMEOUT;
                    return;
                }
                let inflight = *inflight;
                self.complete(key, inflight);
                if let Request::SetConfiguration { command, .. } = inflight.request {
                    self.events
                        .push_back(Event::CommandFinished(command, aem_outcome(aem)));
                    if aem.status().is_success() {
                        self.reconfigured(now, inflight.entity_id, aem.payload);
                    }
                    return;
                }
                self.handle_response(inflight, aem);
            }
            _ => {}
        }
    }

    /// Reads an entity again in the configuration a SET_CONFIGURATION
    /// response or notification names, taking it as advertised so the
    /// advertisement that follows does not start the read over.
    fn reconfigured(&mut self, now: Instant, entity_id: EntityId, payload: &[u8]) {
        if let (Some(configuration), Some(entity)) = (
            aem::configuration_of(payload),
            self.entities.get_mut(&entity_id),
        ) {
            entity.adp.current_configuration_index = configuration;
        }
        if self.entities.contains_key(&entity_id) && self.config.enumerate {
            self.start_enumeration(now, entity_id);
        }
    }

    fn handle_vendor_unique(&mut self, now: Instant, pdu: &VendorUniquePdu<'_>) {
        // CVU SRP declarations are commands every AVB Lite endpoint
        // broadcasts; the controller listens.
        if pdu.header.message_type == AecpMessageType::VENDOR_UNIQUE_COMMAND {
            if let Ok(cvu) = CvuMessage::from_pdu(pdu) {
                self.store_cvu(now, &cvu);
            }
            return;
        }
        if pdu.header.message_type != AecpMessageType::VENDOR_UNIQUE_RESPONSE
            || pdu.header.controller_entity_id != self.config.entity_id
        {
            return;
        }
        if let Ok(message) = LiteMessage::from_pdu(pdu) {
            self.handle_lite(now, pdu, &message);
            return;
        }
        if let Ok(message) = WirelessMessage::from_pdu(pdu) {
            self.handle_wireless(now, pdu, &message);
            return;
        }
        let Ok(message) = MvuMessage::from_pdu(pdu) else {
            return;
        };
        let entity_id = pdu.header.target_entity_id;
        if message.unsolicited {
            // Another controller set a clock domain's media clock reference
            // information.
            if pdu.header.status == 0
                && message.command_type == MvuCommandType::SET_MEDIA_CLOCK_REFERENCE_INFO
                && let Ok(reference) = MediaClockReference::decode(message.data)
            {
                self.store_media_clock_reference(entity_id, reference);
            }
            return;
        }
        let key = (Channel::Mvu, pdu.header.sequence_id);
        let Some(inflight) = self.inflight.get(&key).copied() else {
            return;
        };
        if inflight.entity_id != entity_id {
            return;
        }
        match (inflight.request, message.command_type) {
            (Request::GetMilanInfo, MvuCommandType::GET_MILAN_INFO) => {
                self.complete(key, inflight);
                let info = (pdu.header.status == 0)
                    .then(|| MilanInfo::decode(message.data).ok())
                    .flatten();
                if let Some(model) = self.models.get_mut(&entity_id) {
                    model.milan = info;
                }
            }
            (
                Request::GetMediaClockReference { domain },
                MvuCommandType::GET_MEDIA_CLOCK_REFERENCE_INFO,
            ) => {
                self.complete(key, inflight);
                if pdu.header.status == 0
                    && let Ok(reference) = MediaClockReference::decode(message.data)
                    && reference.domain == domain
                {
                    self.store_media_clock_reference(entity_id, reference);
                }
            }
            _ => return,
        }
        self.check_complete(entity_id);
    }

    /// An AVB Lite status response, or one an entity sent unsolicited.
    fn handle_lite(&mut self, now: Instant, pdu: &VendorUniquePdu<'_>, message: &LiteMessage<'_>) {
        let entity_id = pdu.header.target_entity_id;
        let status = (pdu.header.status == 0
            && message.command_type == LiteCommandType::GET_LITE_STATUS)
            .then(|| LiteStatus::decode(message.data).ok())
            .flatten();
        if message.unsolicited {
            if let Some(status) = status {
                self.store_lite_status(entity_id, status);
            }
            return;
        }
        let key = (Channel::Mvu, pdu.header.sequence_id);
        let Some(inflight) = self.inflight.get(&key).copied() else {
            return;
        };
        if let Request::SetLiteConfig {
            interface, command, ..
        } = inflight.request
        {
            if inflight.entity_id != entity_id {
                return;
            }
            self.complete(key, inflight);
            let status = AemStatus(pdu.header.status);
            self.events.push_back(Event::CommandFinished(
                command,
                if status.is_success() {
                    Outcome::Done
                } else {
                    Outcome::Refused(Refusal::Aem(status))
                },
            ));
            // The status shows the change.
            self.queue_query(entity_id, Request::GetLiteStatus { interface });
            return;
        }
        let Request::GetLiteStatus { interface } = inflight.request else {
            return;
        };
        if inflight.entity_id != entity_id {
            return;
        }
        self.complete(key, inflight);
        match status {
            Some(status) if status.interface == interface => {
                if let Some(model) = self.models.get_mut(&entity_id) {
                    model.lite_supported = Some(true);
                }
                // Asked again from here on, to follow its offset.
                if !self.config.lite_poll.is_zero() {
                    self.next_lite_poll
                        .get_or_insert(now + self.config.lite_poll);
                }
                self.store_lite_status(entity_id, status);
            }
            // An entity without AVB Lite says it does not implement the
            // query; it is not asked again.
            _ if pdu.header.status == AemStatus::NOT_IMPLEMENTED.0 => {
                if let Some(model) = self.models.get_mut(&entity_id) {
                    model.lite_supported = Some(false);
                }
            }
            _ => {}
        }
        self.check_complete(entity_id);
    }

    fn store_lite_status(&mut self, entity_id: EntityId, status: LiteStatus) {
        if let Some(model) = self.models.get_mut(&entity_id)
            && model.has(DescriptorType::AVB_INTERFACE, status.interface)
            && model.set_lite_status(status)
            && model.state == EnumerationState::Complete
        {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    /// An AVB Wireless status response, or one an entity sent unsolicited.
    fn handle_wireless(
        &mut self,
        now: Instant,
        pdu: &VendorUniquePdu<'_>,
        message: &WirelessMessage<'_>,
    ) {
        let entity_id = pdu.header.target_entity_id;
        let wireless = (pdu.header.status == 0
            && message.command_type == WirelessCommandType::GET_WIRELESS_STATUS)
            .then(|| WirelessStatus::decode(message.data).ok())
            .flatten()
            .map(|status| Wireless {
                status,
                stations: status.stations(message.data).collect(),
            });
        if message.unsolicited {
            if let Some(wireless) = wireless {
                self.store_wireless(entity_id, wireless);
            }
            return;
        }
        let key = (Channel::Mvu, pdu.header.sequence_id);
        let Some(inflight) = self.inflight.get(&key).copied() else {
            return;
        };
        if inflight.entity_id != entity_id {
            return;
        }
        if let Request::SetWirelessConfig {
            interface, command, ..
        } = inflight.request
        {
            self.complete(key, inflight);
            let status = AemStatus(pdu.header.status);
            self.events.push_back(Event::CommandFinished(
                command,
                if status.is_success() {
                    Outcome::Done
                } else {
                    Outcome::Refused(Refusal::Aem(status))
                },
            ));
            // The status shows the change.
            self.queue_query(entity_id, Request::GetWirelessStatus { interface });
            return;
        }
        let Request::GetWirelessStatus { interface } = inflight.request else {
            return;
        };
        self.complete(key, inflight);
        let status = AemStatus(pdu.header.status);
        match wireless {
            Some(wireless) if wireless.status.interface == interface => {
                if let Some(model) = self.models.get_mut(&entity_id) {
                    model.wireless_supported = Some(true);
                }
                // Asked again from here on, to follow its link and time.
                if !self.config.wireless_poll.is_zero() {
                    self.next_wireless_poll
                        .get_or_insert(now + self.config.wireless_poll);
                }
                self.store_wireless(entity_id, wireless);
            }
            // The entity answers the query, but the interface is wired.
            _ if status == AemStatus::NOT_SUPPORTED => {
                if let Some(model) = self.models.get_mut(&entity_id) {
                    model.wireless_supported = Some(true);
                }
            }
            // An entity without AVB Wireless says it does not implement
            // the query; it is not asked again.
            _ if status == AemStatus::NOT_IMPLEMENTED => {
                if let Some(model) = self.models.get_mut(&entity_id) {
                    model.wireless_supported = Some(false);
                }
            }
            _ => {}
        }
        self.check_complete(entity_id);
    }

    fn store_wireless(&mut self, entity_id: EntityId, wireless: Wireless) {
        if let Some(model) = self.models.get_mut(&entity_id)
            && model.has(DescriptorType::AVB_INTERFACE, wireless.status.interface)
            && model.set_wireless(wireless)
            && model.state == EnumerationState::Complete
        {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    /// Keeps what a CVU SRP talker message declares, and forgets what it
    /// withdraws, in the talker's model.
    fn store_cvu(&mut self, now: Instant, cvu: &CvuMessage<'_>) {
        let Some(model) = self.models.get_mut(&cvu.sender) else {
            return;
        };
        let mut changed = false;
        for declaration in msrp::talker_declarations(cvu.msrp) {
            changed |= if declaration.event.declares() {
                model.set_cvu_talker(declaration, now)
            } else {
                model.remove_cvu_talker(declaration.stream_id)
            };
        }
        if changed {
            self.events.push_back(Event::EntityModelChanged(cvu.sender));
        }
    }

    fn store_media_clock_reference(&mut self, entity_id: EntityId, reference: MediaClockReference) {
        if let Some(model) = self.models.get_mut(&entity_id)
            && model.has(DescriptorType::CLOCK_DOMAIN, reference.domain)
            && model.set_media_clock_reference(reference)
        {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    /// Clears a command from flight, so the entity's next one can go.
    fn complete(&mut self, key: (Channel, u16), inflight: Inflight) {
        self.inflight.remove(&key);
        if let Some(session) = self.sessions.get_mut(&inflight.entity_id)
            && session.inflight == Some(key)
        {
            session.inflight = None;
        }
    }

    fn handle_response(&mut self, inflight: Inflight, aem: &AemPdu<'_>) {
        let entity_id = inflight.entity_id;
        match inflight.request {
            Request::ReadDescriptor {
                descriptor_type,
                index,
                refresh,
            } => {
                if !aem.status().is_success() {
                    self.read_failed(
                        entity_id,
                        descriptor_type,
                        refresh,
                        EnumerationFailure::Refused(aem.status()),
                    );
                    return;
                }
                let response = ReadDescriptorResponse::decode(aem.payload);
                let Some(response) = matching(response, descriptor_type, index) else {
                    self.read_failed(
                        entity_id,
                        descriptor_type,
                        refresh,
                        EnumerationFailure::Malformed,
                    );
                    return;
                };
                self.descriptor_read(entity_id, response, refresh);
            }
            Request::RegisterUnsolicited => {
                if let Some(model) = self.models.get_mut(&entity_id) {
                    model.registered = aem.status().is_success();
                }
            }
            Request::GetStreamInfo {
                descriptor_type,
                index,
            } => {
                if let Ok(info) = StreamInfo::decode(aem.payload)
                    && aem.status().is_success()
                    && info.descriptor_type == descriptor_type
                    && info.index == index
                {
                    self.store_stream_info(entity_id, info);
                }
            }
            Request::GetAvbInfo { index } => {
                if let Ok(info) = AvbInfo::decode(aem.payload)
                    && aem.status().is_success()
                    && info.index == index
                {
                    self.store_avb_info(entity_id, info);
                }
            }
            Request::GetAsPath { index } => {
                if let Ok(path) = AsPath::decode(aem.payload)
                    && aem.status().is_success()
                    && path.index == index
                {
                    self.store_as_path(entity_id, &path);
                }
            }
            Request::GetCounters {
                descriptor_type,
                index,
            } => {
                if let Ok(counters) = Counters::decode(aem.payload)
                    && aem.status().is_success()
                    && counters.descriptor_type == descriptor_type
                    && counters.index == index
                {
                    self.store_counters(entity_id, counters);
                }
            }
            Request::Identify { command, .. } => {
                if let Some(command) = command {
                    self.events
                        .push_back(Event::CommandFinished(command, aem_outcome(aem)));
                }
            }
            Request::SetControl { index, command } => {
                self.control_values.remove(&command);
                // The response holds the values the control has after,
                // set or not.
                if let Some(values) = control_values(aem.payload, index) {
                    self.store_control(entity_id, index, values);
                }
                self.events
                    .push_back(Event::CommandFinished(command, aem_outcome(aem)));
            }
            Request::GetDynamicInfo { queries, count } => {
                self.dynamic_info_answered(entity_id, &queries[..usize::from(count)], Some(aem));
            }
            Request::GetMaxTransitTime { output, command } => {
                if aem.status().is_success() {
                    self.store_max_transit_time(entity_id, output, aem.payload);
                    self.read_other_transit_times(entity_id, output);
                }
                if let Some(command) = command {
                    self.events
                        .push_back(Event::CommandFinished(command, aem_outcome(aem)));
                }
            }
            Request::SetMaxTransitTime { set, command } => {
                // The response holds the time the output has after; a
                // refusal may hold the time asked for instead, as macOS's
                // does, so the time the model has stays.
                if aem.status().is_success() {
                    self.store_max_transit_time(entity_id, set.output, aem.payload);
                }
                self.events
                    .push_back(Event::CommandFinished(command, aem_outcome(aem)));
            }
            Request::ReadForCaller {
                descriptor_type,
                index,
                command,
            } => {
                let response = ReadDescriptorResponse::decode(aem.payload);
                if aem.status().is_success()
                    && let Some(response) = matching(response, descriptor_type, index)
                    && let Some(model) = self.models.get_mut(&entity_id)
                    && model.store(descriptor_type, index, response.descriptor)
                    && model.state == EnumerationState::Complete
                {
                    self.events.push_back(Event::EntityModelChanged(entity_id));
                }
                self.events
                    .push_back(Event::CommandFinished(command, aem_outcome(aem)));
            }
            Request::GetAudioMap {
                descriptor_type,
                index,
                map_index,
            } => {
                if let Ok(map) = AudioMap::decode(aem.payload)
                    && aem.status().is_success()
                    && (map.descriptor_type, map.index, map.map_index)
                        == (descriptor_type, index, map_index)
                {
                    self.store_audio_map(entity_id, &map);
                }
            }
            Request::ChangeMappings {
                change,
                descriptor_type,
                index,
                command,
            } => {
                self.mapping_changes.remove(&command);
                if aem.status().is_success() {
                    if let Ok(changed) = AudioMappings::decode(aem.payload) {
                        self.store_mapping_change(entity_id, change, &changed);
                    }
                    // Entities may change more than asked, such as removing
                    // the mapping a new one replaces; read the map again.
                    self.queue_query(
                        entity_id,
                        Request::GetAudioMap {
                            descriptor_type,
                            index,
                            map_index: 0,
                        },
                    );
                }
                self.events
                    .push_back(Event::CommandFinished(command, aem_outcome(aem)));
            }
            Request::ReadOther {
                configuration,
                descriptor_type,
                index,
            } => {
                let response = ReadDescriptorResponse::decode(aem.payload);
                let read = aem
                    .status()
                    .is_success()
                    .then(|| matching(response, descriptor_type, index))
                    .flatten()
                    .map(|response| response.descriptor);
                self.other_read(entity_id, configuration, descriptor_type, index, read);
            }
            Request::SetName { command, .. }
            | Request::SetStreamFormat { command, .. }
            | Request::SetSamplingRate { command, .. }
            | Request::SetClockSource { command, .. } => {
                if aem.status().is_success() {
                    self.applied(entity_id, inflight.request, aem.payload);
                }
                self.events
                    .push_back(Event::CommandFinished(command, aem_outcome(aem)));
            }
            // Finished as it is answered, where the read starts over.
            Request::SetConfiguration { .. }
            | Request::GetMilanInfo
            | Request::GetMediaClockReference { .. }
            | Request::GetLiteStatus { .. }
            | Request::SetLiteConfig { .. }
            | Request::GetWirelessStatus { .. }
            | Request::SetWirelessConfig { .. }
            | Request::GetRxState { .. }
            | Request::Bind { .. }
            | Request::Unbind { .. }
            | Request::GetTxState { .. }
            | Request::DisconnectTx { .. } => {}
        }
        self.check_complete(entity_id);
    }

    /// Keeps a descriptor of another configuration, when it was read; for
    /// its CONFIGURATION descriptor, reads its streams; once the last is
    /// answered, says the model changed.
    fn other_read(
        &mut self,
        entity_id: EntityId,
        configuration: u16,
        descriptor_type: DescriptorType,
        index: u16,
        bytes: Option<&[u8]>,
    ) {
        let Some(model) = self.models.get_mut(&entity_id) else {
            return;
        };
        if let Some(bytes) = bytes {
            model.store_other(configuration, descriptor_type, index, bytes);
        }
        if descriptor_type == DescriptorType::CONFIGURATION
            && let Some(Ok(descriptor)) = bytes.map(ConfigurationDescriptor::decode)
        {
            for (stream_type, count) in descriptor.descriptor_counts().filter(|(found, _)| {
                matches!(
                    *found,
                    DescriptorType::STREAM_INPUT | DescriptorType::STREAM_OUTPUT
                )
            }) {
                for stream in 0..count {
                    self.queue_query(
                        entity_id,
                        Request::ReadOther {
                            configuration,
                            descriptor_type: stream_type,
                            index: stream,
                        },
                    );
                }
            }
        }
        let pending = self.sessions.get(&entity_id).is_some_and(|session| {
            session
                .queue
                .iter()
                .any(|request| matches!(request, Request::ReadOther { .. }))
        });
        if !pending {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    /// Updates the model with what a SET command's response says the
    /// entity now has, reading again what the response cannot tell: a
    /// descriptor not stored as expected, and the streams after a new
    /// sampling rate, whose formats may follow it.
    fn applied(&mut self, entity_id: EntityId, request: Request, payload: &[u8]) {
        let Some(model) = self.models.get_mut(&entity_id) else {
            return;
        };
        let (changed, target) = match request {
            Request::SetName { set, .. } => (
                SetName::decode(payload).is_ok_and(|set| model.apply_name(&set)),
                (set.descriptor_type, set.index),
            ),
            Request::SetStreamFormat { set, .. } => (
                SetStreamFormat::decode(payload).is_ok_and(|set| model.apply_stream_format(&set)),
                (set.descriptor_type, set.index),
            ),
            Request::SetSamplingRate { set, .. } => (
                SetSamplingRate::decode(payload).is_ok_and(|set| model.apply_sampling_rate(&set)),
                (set.descriptor_type, set.index),
            ),
            Request::SetClockSource { set, .. } => (
                SetClockSource::decode(payload).is_ok_and(|set| model.apply_clock_source(&set)),
                (DescriptorType::CLOCK_DOMAIN, set.domain),
            ),
            _ => return,
        };
        let streams: Vec<(DescriptorType, u16)> = match request {
            Request::SetSamplingRate { .. } => {
                [DescriptorType::STREAM_INPUT, DescriptorType::STREAM_OUTPUT]
                    .into_iter()
                    .flat_map(|descriptor_type| {
                        model
                            .descriptors(descriptor_type)
                            .map(move |(index, _)| (descriptor_type, index))
                    })
                    .collect()
            }
            _ => Vec::new(),
        };
        if changed {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        } else {
            self.queue_read(entity_id, target.0, target.1, true);
        }
        for (descriptor_type, index) in streams {
            self.queue_read(entity_id, descriptor_type, index, true);
        }
        if let Request::SetStreamFormat { set, .. } = request {
            self.queue_query(
                entity_id,
                Request::GetStreamInfo {
                    descriptor_type: set.descriptor_type,
                    index: set.index,
                },
            );
        }
        // Mappings to channels a stream no longer has may be dropped.
        if matches!(
            request,
            Request::SetStreamFormat { .. } | Request::SetSamplingRate { .. }
        ) {
            self.queue_audio_map_reads(entity_id);
        }
    }

    /// Queues reading each of an entity's dynamic stream port maps again.
    fn queue_audio_map_reads(&mut self, entity_id: EntityId) {
        if !self.config.read_mappings {
            return;
        }
        let Some(model) = self.models.get(&entity_id) else {
            return;
        };
        for request in audio_map_reads(model) {
            self.queue_query(entity_id, request);
        }
    }

    fn store_audio_map(&mut self, entity_id: EntityId, map: &AudioMap<'_>) {
        let Some(model) = self.models.get_mut(&entity_id) else {
            return;
        };
        if !model.has(map.descriptor_type, map.index) {
            return;
        }
        if model.set_audio_map(map) {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
        // The next part goes next, after the caller's commands, so the parts
        // are read together.
        let next = map.map_index.saturating_add(1);
        if next < map.number_of_maps.min(MAX_MAP_PARTS)
            && let Some(session) = self.sessions.get_mut(&entity_id)
        {
            let request = Request::GetAudioMap {
                descriptor_type: map.descriptor_type,
                index: map.index,
                map_index: next,
            };
            if !session.queue.contains(&request) {
                let place = session
                    .queue
                    .iter()
                    .position(|queued| queued.command().is_none())
                    .unwrap_or(session.queue.len());
                session.queue.insert(place, request);
            }
        }
    }

    fn store_mapping_change(
        &mut self,
        entity_id: EntityId,
        change: MappingChange,
        changed: &AudioMappings<'_>,
    ) {
        if let Some(model) = self.models.get_mut(&entity_id)
            && model.apply_mappings(change, changed)
        {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    fn store_stream_info(&mut self, entity_id: EntityId, info: StreamInfo) {
        if let Some(model) = self.models.get_mut(&entity_id)
            && model.has(info.descriptor_type, info.index)
            && model.set_stream_info(info)
        {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    fn store_avb_info(&mut self, entity_id: EntityId, info: AvbInfo) {
        if let Some(model) = self.models.get_mut(&entity_id)
            && model.has(DescriptorType::AVB_INTERFACE, info.index)
            && model.set_avb_info(info)
        {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    fn store_as_path(&mut self, entity_id: EntityId, path: &AsPath<'_>) {
        if let Some(model) = self.models.get_mut(&entity_id)
            && model.has(DescriptorType::AVB_INTERFACE, path.index)
            && model.set_as_path(path)
        {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    fn store_counters(&mut self, entity_id: EntityId, counters: Counters) {
        if let Some(model) = self.models.get_mut(&entity_id)
            && model.has(counters.descriptor_type, counters.index)
            && model.set_counters(counters)
        {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    fn handle_acmp(&mut self, acmpdu: &Acmpdu) {
        if !acmpdu.message_type.is_response() {
            return;
        }
        let key = (Channel::Acmp, acmpdu.sequence_id);
        // TX commands go to the talker, RX ones to the listener.
        let addressed = if to_talker(acmpdu.message_type) {
            acmpdu.talker_entity_id
        } else {
            acmpdu.listener_entity_id
        };
        if acmpdu.controller_entity_id == self.config.entity_id
            && let Some(inflight) = self.inflight.get(&key).copied()
            && inflight.entity_id == addressed
            && inflight
                .request
                .acmp_message()
                .map(AcmpMessageType::response)
                == Some(acmpdu.message_type)
        {
            self.complete(key, inflight);
            if let Request::GetTxState { output, .. } = inflight.request
                && acmpdu.status.is_success()
                && acmpdu.talker_unique_id == output
            {
                let state = TxState {
                    stream_id: acmpdu.stream_id,
                    destination: acmpdu.stream_dest_mac,
                    connection_count: acmpdu.connection_count,
                    vlan_id: acmpdu.stream_vlan_id,
                };
                if let Some(model) = self.models.get_mut(&addressed)
                    && model.set_tx_state(output, state)
                {
                    self.events.push_back(Event::EntityModelChanged(addressed));
                }
            }
            if let Some(command) = inflight.request.command() {
                let outcome = if acmpdu.status.is_success() {
                    Outcome::Done
                } else {
                    Outcome::Refused(Refusal::Acmp(acmpdu.status))
                };
                self.events
                    .push_back(Event::CommandFinished(command, outcome));
            }
        }
        // Responses to every controller's commands tell the listener's
        // bindings, so changes made elsewhere show too.
        if acmpdu.status.is_success() {
            self.observe_binding(acmpdu);
        }
    }

    fn observe_binding(&mut self, acmpdu: &Acmpdu) {
        let binding = match acmpdu.message_type {
            AcmpMessageType::CONNECT_RX_RESPONSE => Binding {
                talker: acmpdu.talker_entity_id,
                talker_unique_id: acmpdu.talker_unique_id,
                connection_count: acmpdu.connection_count.max(1),
                flags: acmpdu.flags,
            },
            AcmpMessageType::DISCONNECT_RX_RESPONSE => Binding::default(),
            AcmpMessageType::GET_RX_STATE_RESPONSE => Binding {
                talker: acmpdu.talker_entity_id,
                talker_unique_id: acmpdu.talker_unique_id,
                connection_count: acmpdu.connection_count,
                flags: acmpdu.flags,
            },
            _ => return,
        };
        let entity_id = acmpdu.listener_entity_id;
        let input = acmpdu.listener_unique_id;
        let Some(model) = self.models.get_mut(&entity_id) else {
            return;
        };
        if !model.has(DescriptorType::STREAM_INPUT, input) {
            return;
        }
        if model.set_binding(input, binding) {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
        // The input's stream info changes with its binding.
        if acmpdu.message_type != AcmpMessageType::GET_RX_STATE_RESPONSE {
            self.queue_query(
                entity_id,
                Request::GetStreamInfo {
                    descriptor_type: DescriptorType::STREAM_INPUT,
                    index: input,
                },
            );
        }
    }

    /// Queues a query about the entity's current state, unless the same
    /// one is queued already.
    fn queue_query(&mut self, entity_id: EntityId, request: Request) {
        if let Some(session) = self.sessions.get_mut(&entity_id)
            && !session.queue.contains(&request)
        {
            session.queue.push_back(request);
        }
    }

    fn read_failed(
        &mut self,
        entity_id: EntityId,
        descriptor_type: DescriptorType,
        refresh: bool,
        failure: EnumerationFailure,
    ) {
        let Some(model) = self.models.get_mut(&entity_id) else {
            return;
        };
        let essential = matches!(
            descriptor_type,
            DescriptorType::ENTITY | DescriptorType::CONFIGURATION
        );
        if essential && !refresh {
            model.state = EnumerationState::Failed(failure);
            if let Some(session) = self.sessions.get_mut(&entity_id) {
                session.queue.clear();
            }
            self.events
                .push_back(Event::EnumerationFailed(entity_id, failure));
        } else {
            model.failed_reads = model.failed_reads.saturating_add(1);
        }
    }

    fn descriptor_read(
        &mut self,
        entity_id: EntityId,
        response: ReadDescriptorResponse<'_>,
        refresh: bool,
    ) {
        let descriptor = response.descriptor;
        let mut follow_ups: Vec<(DescriptorType, u16, u16)> = Vec::new();
        let Some(model) = self.models.get_mut(&entity_id) else {
            return;
        };
        if !refresh {
            match response.descriptor_type {
                DescriptorType::ENTITY => match EntityDescriptor::decode(descriptor) {
                    Ok(entity) => {
                        model.configuration = entity.current_configuration;
                        follow_ups.push((
                            DescriptorType::CONFIGURATION,
                            entity.current_configuration,
                            1,
                        ));
                    }
                    Err(_) => {
                        self.read_failed(
                            entity_id,
                            DescriptorType::ENTITY,
                            false,
                            EnumerationFailure::Malformed,
                        );
                        return;
                    }
                },
                DescriptorType::CONFIGURATION => {
                    match ConfigurationDescriptor::decode(descriptor) {
                        Ok(configuration) => {
                            let key = model.entity().map(|entity| ModelKey {
                                entity_model_id: entity.entity_model_id,
                                firmware: entity.firmware_version.into(),
                                configuration: response.index,
                            });
                            // A model cached for the same model, firmware
                            // and configuration, with the same descriptors.
                            let cached =
                                key.and_then(|key| self.cache.get(&key)).filter(|cached| {
                                    cached
                                        .descriptor(DescriptorType::CONFIGURATION, response.index)
                                        .and_then(|bytes| {
                                            ConfigurationDescriptor::decode(bytes).ok()
                                        })
                                        .is_some_and(|known| {
                                            known
                                                .descriptor_counts()
                                                .eq(configuration.descriptor_counts())
                                        })
                                });
                            if let Some(cached) = cached {
                                let cached = cached.clone();
                                model.store(response.descriptor_type, response.index, descriptor);
                                self.use_cached(entity_id, &cached);
                                return;
                            }
                            for (descriptor_type, count) in configuration.descriptor_counts() {
                                if READ_TYPES.contains(&descriptor_type) {
                                    follow_ups.push((descriptor_type, 0, count));
                                }
                            }
                        }
                        Err(_) => {
                            self.read_failed(
                                entity_id,
                                DescriptorType::CONFIGURATION,
                                false,
                                EnumerationFailure::Malformed,
                            );
                            return;
                        }
                    }
                }
                DescriptorType::AUDIO_UNIT => {
                    if let Ok(unit) = AudioUnitDescriptor::decode(descriptor) {
                        follow_ups.push((
                            DescriptorType::STREAM_PORT_INPUT,
                            unit.base_stream_input_port,
                            unit.number_of_stream_input_ports,
                        ));
                        follow_ups.push((
                            DescriptorType::STREAM_PORT_OUTPUT,
                            unit.base_stream_output_port,
                            unit.number_of_stream_output_ports,
                        ));
                    }
                }
                DescriptorType::STREAM_PORT_INPUT | DescriptorType::STREAM_PORT_OUTPUT => {
                    if let Ok(port) = StreamPortDescriptor::decode(descriptor) {
                        follow_ups.push((
                            DescriptorType::AUDIO_CLUSTER,
                            port.base_cluster,
                            port.number_of_clusters,
                        ));
                        follow_ups.push((
                            DescriptorType::AUDIO_MAP,
                            port.base_map,
                            port.number_of_maps,
                        ));
                    }
                }
                DescriptorType::LOCALE => {
                    if let Ok(locale) = LocaleDescriptor::decode(descriptor) {
                        follow_ups.push((
                            DescriptorType::STRINGS,
                            locale.base_strings,
                            locale.number_of_strings,
                        ));
                    }
                }
                _ => {}
            }
        }
        let changed = model.store(response.descriptor_type, response.index, descriptor);
        if refresh && changed {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
        for (descriptor_type, first, count) in follow_ups {
            for index in first..first.saturating_add(count) {
                self.queue_read(entity_id, descriptor_type, index, false);
            }
        }
    }

    /// Takes an entity's descriptors from a cached model of its entity
    /// model: what never changes as it is, what differs between entities
    /// read from the entity, and what changes on an entity asked for with
    /// GET_DYNAMIC_INFO when it is a Milan entity, read otherwise.
    fn use_cached(&mut self, entity_id: EntityId, cached: &CachedModel) {
        let Some(model) = self.models.get_mut(&entity_id) else {
            return;
        };
        model.from_cache = true;
        let milan = model.milan.is_some();
        let mut own = Vec::new();
        let mut queries = Vec::new();
        for (descriptor_type, index, bytes) in &cached.descriptors {
            let (descriptor_type, index) = (*descriptor_type, *index);
            if OWN_TYPES.contains(&descriptor_type) {
                if !matches!(
                    descriptor_type,
                    DescriptorType::ENTITY | DescriptorType::CONFIGURATION
                ) {
                    own.push((descriptor_type, index));
                }
                continue;
            }
            model.store(descriptor_type, index, bytes);
            if STATIC_TYPES.contains(&descriptor_type) {
                continue;
            }
            queries.extend(dynamic_queries(descriptor_type, index));
        }
        for (descriptor_type, index) in own {
            self.queue_read(entity_id, descriptor_type, index, false);
        }
        if milan {
            self.queue_dynamic_info(entity_id, &queries);
        } else {
            self.read_instead(entity_id, &queries);
        }
    }

    /// Queues GET_DYNAMIC_INFO for `queries`, as many to a command as
    /// their answers fit.
    fn queue_dynamic_info(&mut self, entity_id: EntityId, queries: &[DynamicQuery]) {
        let Some(session) = self.sessions.get_mut(&entity_id) else {
            return;
        };
        let mut batch = [DynamicQuery::ClockSource { domain: 0 }; MAX_DYNAMIC_QUERIES];
        let (mut count, mut room) = (0, 0);
        for &query in queries {
            if count == MAX_DYNAMIC_QUERIES || room + query.answer_len() > aem::MAX_AEM_PAYLOAD {
                session.queue.push_back(Request::GetDynamicInfo {
                    queries: batch,
                    count: count as u8,
                });
                (count, room) = (0, 0);
            }
            batch[count] = query;
            count += 1;
            room += query.answer_len();
        }
        if count > 0 {
            session.queue.push_back(Request::GetDynamicInfo {
                queries: batch,
                count: count as u8,
            });
        }
    }

    /// Reads again the descriptors `queries` ask about, for an entity that
    /// does not answer GET_DYNAMIC_INFO.
    fn read_instead(&mut self, entity_id: EntityId, queries: &[DynamicQuery]) {
        let mut targets: Vec<(DescriptorType, u16)> =
            queries.iter().map(|query| query.target()).collect();
        targets.dedup();
        for (descriptor_type, index) in targets {
            self.queue_read(entity_id, descriptor_type, index, true);
        }
    }

    /// Applies a GET_DYNAMIC_INFO response, reading again what it did not
    /// answer; when the entity refused it, reads instead what this one and
    /// the ones still queued ask about.
    fn dynamic_info_answered(
        &mut self,
        entity_id: EntityId,
        asked: &[DynamicQuery],
        aem: Option<&AemPdu<'_>>,
    ) {
        let Some(aem) = aem.filter(|aem| aem.status().is_success()) else {
            let mut queries = asked.to_vec();
            if let Some(session) = self.sessions.get_mut(&entity_id) {
                session.queue.retain(|request| match request {
                    Request::GetDynamicInfo {
                        queries: more,
                        count,
                    } => {
                        queries.extend_from_slice(&more[..usize::from(*count)]);
                        false
                    }
                    _ => true,
                });
            }
            self.read_instead(entity_id, &queries);
            return;
        };
        let Some(model) = self.models.get_mut(&entity_id) else {
            return;
        };
        let mut unanswered: Vec<DynamicQuery> = asked.to_vec();
        let mut changed = false;
        for info in aem::dynamic_infos(aem.payload) {
            if !info.status.is_success() {
                continue;
            }
            let applied = match info.command_type {
                AemCommandType::GET_NAME => SetName::decode(info.data).ok().map(|got| {
                    let query = DynamicQuery::Name {
                        descriptor_type: got.descriptor_type,
                        index: got.index,
                        name_index: got.name_index,
                    };
                    (query, model.apply_name(&got))
                }),
                AemCommandType::GET_STREAM_FORMAT => {
                    SetStreamFormat::decode(info.data).ok().map(|got| {
                        let query = DynamicQuery::StreamFormat {
                            descriptor_type: got.descriptor_type,
                            index: got.index,
                        };
                        (query, model.apply_stream_format(&got))
                    })
                }
                AemCommandType::GET_SAMPLING_RATE => {
                    SetSamplingRate::decode(info.data).ok().map(|got| {
                        let query = DynamicQuery::SamplingRate {
                            descriptor_type: got.descriptor_type,
                            index: got.index,
                        };
                        (query, model.apply_sampling_rate(&got))
                    })
                }
                AemCommandType::GET_CLOCK_SOURCE => {
                    SetClockSource::decode(info.data).ok().map(|got| {
                        (
                            DynamicQuery::ClockSource { domain: got.domain },
                            model.apply_clock_source(&got),
                        )
                    })
                }
                _ => None,
            };
            if let Some((query, applied)) = applied {
                unanswered.retain(|asked| *asked != query);
                changed |= applied;
            }
        }
        if changed && model.state == EnumerationState::Complete {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
        self.read_instead(entity_id, &unanswered);
    }

    /// Remembers an entity model read before, so entities of that model are
    /// read faster; the caller decides which, as from
    /// [`static_model`](Self::static_model) once an entity is read.
    pub fn remember_model(&mut self, model: CachedModel) {
        self.cache.insert(model.key.clone(), model);
    }

    /// An entity's model to keep for next time: its descriptors as read,
    /// once all of them were and none came from the cache.
    pub fn static_model(&self, entity_id: EntityId) -> Option<CachedModel> {
        let model = self.models.get(&entity_id)?;
        if model.state != EnumerationState::Complete || model.failed_reads > 0 || model.from_cache {
            return None;
        }
        let entity = model.entity()?;
        Some(CachedModel {
            key: ModelKey {
                entity_model_id: entity.entity_model_id,
                firmware: entity.firmware_version.into(),
                configuration: model.configuration,
            },
            descriptors: model
                .all_descriptors()
                .filter(|(descriptor_type, _, _)| *descriptor_type != DescriptorType::ENTITY)
                .map(|(descriptor_type, index, bytes)| (descriptor_type, index, bytes.to_vec()))
                .collect(),
        })
    }

    /// Queues a descriptor read unless it is read or queued already, or
    /// the entity has asked for more than any could need.
    fn queue_read(
        &mut self,
        entity_id: EntityId,
        descriptor_type: DescriptorType,
        index: u16,
        refresh: bool,
    ) {
        let (Some(session), Some(model)) = (
            self.sessions.get_mut(&entity_id),
            self.models.get(&entity_id),
        ) else {
            return;
        };
        let request = Request::ReadDescriptor {
            descriptor_type,
            index,
            refresh,
        };
        if session.queue.contains(&request) {
            return;
        }
        if !refresh {
            if model.has(descriptor_type, index) || session.requested >= MAX_DESCRIPTORS {
                return;
            }
            session.requested += 1;
        }
        session.queue.push_back(request);
    }

    /// Marks an entity read once nothing is queued or in flight for it,
    /// and registers for its notifications.
    fn check_complete(&mut self, entity_id: EntityId) {
        let (Some(session), Some(model)) = (
            self.sessions.get_mut(&entity_id),
            self.models.get_mut(&entity_id),
        ) else {
            return;
        };
        if model.state != EnumerationState::Reading
            || !session.queue.is_empty()
            || session.inflight.is_some()
        {
            return;
        }
        model.state = EnumerationState::Complete;
        // What the streams are doing now.
        for (index, _) in model.descriptors(DescriptorType::STREAM_INPUT) {
            session
                .queue
                .push_back(Request::GetRxState { input: index });
            session.queue.push_back(Request::GetStreamInfo {
                descriptor_type: DescriptorType::STREAM_INPUT,
                index,
            });
        }
        for (index, _) in model.descriptors(DescriptorType::STREAM_OUTPUT) {
            session.queue.push_back(Request::GetStreamInfo {
                descriptor_type: DescriptorType::STREAM_OUTPUT,
                index,
            });
        }
        // How channels map to and from the streams.
        if self.config.read_mappings {
            session.queue.extend(audio_map_reads(model));
        }
        // How the clocks and streams have fared.
        if self.config.read_counters {
            for descriptor_type in [
                DescriptorType::CLOCK_DOMAIN,
                DescriptorType::STREAM_INPUT,
                DescriptorType::STREAM_OUTPUT,
            ] {
                for (index, _) in model.descriptors(descriptor_type) {
                    session.queue.push_back(Request::GetCounters {
                        descriptor_type,
                        index,
                    });
                }
            }
        }
        // How long each stream output's frames may take, which Milan
        // talkers report and others may, as macOS does: an entity outside
        // Milan is asked for its first output's, and for the rest once it
        // answers.
        if self.config.read_transit_times {
            let asked = if model.milan.is_some() { usize::MAX } else { 1 };
            for (output, _) in model.descriptors(DescriptorType::STREAM_OUTPUT).take(asked) {
                session.queue.push_back(Request::GetMaxTransitTime {
                    output,
                    command: None,
                });
            }
        }
        // What its other configurations hold, to choose among them: each
        // one's CONFIGURATION descriptor, then its streams.
        if self.config.read_configurations
            && let Some(entity) = model.entity()
        {
            for configuration in
                (0..entity.configurations_count).filter(|&other| other != model.configuration)
            {
                session.queue.push_back(Request::ReadOther {
                    configuration,
                    descriptor_type: DescriptorType::CONFIGURATION,
                    index: configuration,
                });
            }
        }
        // Whether each interface runs AVB or AVB Lite, from entities that
        // answer the query.
        if self.config.lite_status && model.lite_supported != Some(false) {
            for (interface, _) in model.descriptors(DescriptorType::AVB_INTERFACE) {
                session
                    .queue
                    .push_back(Request::GetLiteStatus { interface });
            }
        }
        // Which interfaces are wireless, and how their link and time hold,
        // from entities that answer the query.
        if self.config.wireless_status && model.wireless_supported != Some(false) {
            for (interface, _) in model.descriptors(DescriptorType::AVB_INTERFACE) {
                session
                    .queue
                    .push_back(Request::GetWirelessStatus { interface });
            }
        }
        // How each clock domain stands in media clock management.
        if self.config.media_clock_info && model.milan.is_some() {
            for (domain, _) in model.descriptors(DescriptorType::CLOCK_DOMAIN) {
                session
                    .queue
                    .push_back(Request::GetMediaClockReference { domain });
            }
        }
        // Where the entity sits in the network.
        if self.config.network_info {
            for (index, _) in model.descriptors(DescriptorType::AVB_INTERFACE) {
                session.queue.push_back(Request::GetAvbInfo { index });
                session.queue.push_back(Request::GetAsPath { index });
                session.queue.push_back(Request::GetCounters {
                    descriptor_type: DescriptorType::AVB_INTERFACE,
                    index,
                });
            }
        }
        if self.config.register_unsolicited && !model.registered {
            session.queue.push_back(Request::RegisterUnsolicited);
        }
        self.events.push_back(Event::EntityEnumerated(entity_id));
    }

    fn handle_notification(&mut self, entity_id: EntityId, aem: &AemPdu<'_>) {
        let Some(model) = self.models.get_mut(&entity_id) else {
            return;
        };
        if aem.command_type == AemCommandType::DEREGISTER_UNSOLICITED_NOTIFICATION {
            model.registered = false;
            if self.config.register_unsolicited
                && model.state == EnumerationState::Complete
                && let Some(session) = self.sessions.get_mut(&entity_id)
            {
                session.queue.push_back(Request::RegisterUnsolicited);
            }
            return;
        }
        match aem.command_type {
            AemCommandType::GET_STREAM_INFO | AemCommandType::SET_STREAM_INFO => {
                if let Ok(info) = StreamInfo::decode(aem.payload) {
                    self.store_stream_info(entity_id, info);
                }
                return;
            }
            AemCommandType::GET_AVB_INFO => {
                if let Ok(info) = AvbInfo::decode(aem.payload) {
                    self.store_avb_info(entity_id, info);
                }
                return;
            }
            AemCommandType::GET_AS_PATH => {
                if let Ok(path) = AsPath::decode(aem.payload) {
                    self.store_as_path(entity_id, &path);
                }
                return;
            }
            AemCommandType::GET_COUNTERS => {
                if let Ok(counters) = Counters::decode(aem.payload) {
                    self.store_counters(entity_id, counters);
                }
                return;
            }
            AemCommandType::ADD_AUDIO_MAPPINGS | AemCommandType::REMOVE_AUDIO_MAPPINGS => {
                let change = if aem.command_type == AemCommandType::ADD_AUDIO_MAPPINGS {
                    MappingChange::Add
                } else {
                    MappingChange::Remove
                };
                if let Ok(changed) = AudioMappings::decode(aem.payload) {
                    self.store_mapping_change(entity_id, change, &changed);
                }
                return;
            }
            // Reading a map changes nothing; its descriptor needs no read.
            AemCommandType::GET_AUDIO_MAP => return,
            AemCommandType::SET_MAX_TRANSIT_TIME | AemCommandType::GET_MAX_TRANSIT_TIME => {
                if let Ok(time) = MaxTransitTime::decode(aem.payload) {
                    self.store_max_transit_time(entity_id, time.output, aem.payload);
                }
                return;
            }
            AemCommandType::SET_CONTROL | AemCommandType::GET_CONTROL => {
                if let Some((DescriptorType::CONTROL, index)) = aem::target_descriptor(aem.payload)
                    && let Some(values) = control_values(aem.payload, index)
                {
                    self.store_control(entity_id, index, values);
                }
                return;
            }
            _ => {}
        }
        // Whatever the notification changed, the descriptor it names holds
        // the new value; read it again.
        if let Some((descriptor_type, index)) = aem::target_descriptor(aem.payload)
            && model.has(descriptor_type, index)
        {
            self.queue_read(entity_id, descriptor_type, index, true);
        }
    }

    /// Sends the next queued command of each entity with none in flight.
    fn pump(&mut self, now: Instant) {
        let ready: Vec<EntityId> = self
            .sessions
            .iter()
            .filter(|(_, session)| session.inflight.is_none() && !session.queue.is_empty())
            .map(|(&entity_id, _)| entity_id)
            .collect();
        for entity_id in ready {
            let Some(request) = self
                .sessions
                .get_mut(&entity_id)
                .and_then(|session| session.queue.pop_front())
            else {
                continue;
            };
            let counter = match request.channel() {
                Channel::Aem => &mut self.next_aem_sequence,
                Channel::Mvu => &mut self.next_mvu_sequence,
                Channel::Acmp => &mut self.next_acmp_sequence,
            };
            let sequence = *counter;
            *counter = sequence.wrapping_add(1);
            let key = (request.channel(), sequence);
            let inflight = Inflight {
                entity_id,
                request,
                deadline: now + self.timeout(entity_id, request),
                retried: false,
            };
            if !self.send(key, inflight) {
                continue;
            }
            self.inflight.insert(key, inflight);
            if let Some(session) = self.sessions.get_mut(&entity_id) {
                session.inflight = Some(key);
            }
        }
    }

    /// Queues a command for sending, returning whether it could be built.
    fn send(&mut self, (_, sequence_id): (Channel, u16), inflight: Inflight) -> bool {
        let Some(entity) = self.entities.get(&inflight.entity_id) else {
            return false;
        };
        let addressing = Addressing {
            target: inflight.entity_id,
            controller: self.config.entity_id,
            sequence_id,
        };
        // Room for the longest command, GET_DYNAMIC_INFO.
        let mut out = [0; 600];
        let acmp = |message_type, talker: (EntityId, u16), input: u16, out: &mut [u8]| {
            Acmpdu::command(
                message_type,
                addressing.controller,
                talker,
                (addressing.target, input),
                sequence_id,
            )
            .encode(out)
        };
        let destination = match inflight.request.channel() {
            Channel::Acmp => crate::ADP_ACMP_MULTICAST,
            Channel::Aem | Channel::Mvu => entity.mac,
        };
        let length = match inflight.request {
            Request::ReadDescriptor {
                descriptor_type,
                index,
                ..
            } => {
                let configuration = self
                    .models
                    .get(&inflight.entity_id)
                    .map_or(0, |model| model.configuration);
                aem::encode_read_descriptor(
                    addressing,
                    configuration,
                    descriptor_type,
                    index,
                    &mut out,
                )
            }
            Request::RegisterUnsolicited => aem::encode_register_unsolicited(addressing, &mut out),
            Request::GetLiteStatus { interface } => lite::encode_get_lite_status(
                addressing.target,
                addressing.controller,
                sequence_id,
                interface,
                &mut out,
            ),
            Request::SetLiteConfig {
                interface, flags, ..
            } => lite::encode_set_lite_config(
                addressing.target,
                addressing.controller,
                sequence_id,
                interface,
                lite::LiteConfigFlags(flags),
                &mut out,
            ),
            Request::GetWirelessStatus { interface } => wireless::encode_get_wireless_status(
                addressing.target,
                addressing.controller,
                sequence_id,
                interface,
                &mut out,
            ),
            Request::SetWirelessConfig {
                interface, flags, ..
            } => wireless::encode_set_wireless_config(
                addressing.target,
                addressing.controller,
                sequence_id,
                interface,
                wireless::WirelessConfigFlags(flags),
                &mut out,
            ),
            Request::GetMilanInfo => mvu::encode_get_milan_info(
                addressing.target,
                addressing.controller,
                sequence_id,
                &mut out,
            ),
            Request::GetMediaClockReference { domain } => mvu::encode_get_media_clock_reference(
                addressing.target,
                addressing.controller,
                sequence_id,
                domain,
                &mut out,
            ),
            Request::GetStreamInfo {
                descriptor_type,
                index,
            } => aem::encode_get_stream_info(addressing, descriptor_type, index, &mut out),
            Request::GetAvbInfo { index } => aem::encode_get_avb_info(addressing, index, &mut out),
            Request::GetAsPath { index } => aem::encode_get_as_path(addressing, index, &mut out),
            Request::GetCounters {
                descriptor_type,
                index,
            } => aem::encode_get_counters(addressing, descriptor_type, index, &mut out),
            Request::Identify { control, on, .. } => {
                aem::encode_set_control(addressing, control, &[if on { 255 } else { 0 }], &mut out)
            }
            Request::SetName { set, .. } => {
                let configuration = match set.descriptor_type {
                    DescriptorType::ENTITY | DescriptorType::CONFIGURATION => 0,
                    _ => self
                        .models
                        .get(&inflight.entity_id)
                        .map_or(0, |model| model.configuration),
                };
                SetName {
                    configuration,
                    ..set
                }
                .encode(addressing, &mut out)
            }
            Request::SetStreamFormat { set, .. } => set.encode(addressing, &mut out),
            Request::SetSamplingRate { set, .. } => set.encode(addressing, &mut out),
            Request::SetClockSource { set, .. } => set.encode(addressing, &mut out),
            Request::SetConfiguration { configuration, .. } => {
                aem::encode_set_configuration(addressing, configuration, &mut out)
            }
            Request::ReadOther {
                configuration,
                descriptor_type,
                index,
            } => aem::encode_read_descriptor(
                addressing,
                configuration,
                descriptor_type,
                index,
                &mut out,
            ),
            Request::GetMaxTransitTime { output, .. } => {
                aem::encode_get_max_transit_time(addressing, output, &mut out)
            }
            Request::SetMaxTransitTime { set, .. } => set.encode(addressing, &mut out),
            Request::ReadForCaller {
                descriptor_type,
                index,
                ..
            } => {
                let configuration = self
                    .models
                    .get(&inflight.entity_id)
                    .map_or(0, |model| model.configuration);
                aem::encode_read_descriptor(
                    addressing,
                    configuration,
                    descriptor_type,
                    index,
                    &mut out,
                )
            }
            Request::GetTxState { output, .. } => Acmpdu::command(
                AcmpMessageType::GET_TX_STATE_COMMAND,
                addressing.controller,
                (addressing.target, output),
                (EntityId(0), 0),
                sequence_id,
            )
            .encode(&mut out),
            Request::DisconnectTx {
                output,
                listener,
                listener_unique_id,
                ..
            } => Acmpdu::command(
                AcmpMessageType::DISCONNECT_TX_COMMAND,
                addressing.controller,
                (addressing.target, output),
                (listener, listener_unique_id),
                sequence_id,
            )
            .encode(&mut out),
            Request::SetControl { index, command } => aem::encode_set_control(
                addressing,
                index,
                self.control_values
                    .get(&command)
                    .map_or(&[][..], Vec::as_slice),
                &mut out,
            ),
            Request::GetAudioMap {
                descriptor_type,
                index,
                map_index,
            } => aem::encode_get_audio_map(addressing, descriptor_type, index, map_index, &mut out),
            Request::ChangeMappings {
                change,
                descriptor_type,
                index,
                command,
            } => aem::encode_audio_mappings(
                addressing,
                change,
                descriptor_type,
                index,
                self.mapping_changes
                    .get(&command)
                    .map_or(&[][..], Vec::as_slice),
                &mut out,
            ),
            Request::GetDynamicInfo { queries, count } => {
                let configuration = self
                    .models
                    .get(&inflight.entity_id)
                    .map_or(0, |model| model.configuration);
                aem::encode_get_dynamic_info(
                    addressing,
                    configuration,
                    &queries[..usize::from(count)],
                    &mut out,
                )
            }
            Request::GetRxState { input } => acmp(
                AcmpMessageType::GET_RX_STATE_COMMAND,
                (EntityId(0), 0),
                input,
                &mut out,
            ),
            Request::Bind {
                talker,
                talker_unique_id,
                input,
                class_b,
                ..
            } => {
                let mut connect = Acmpdu::command(
                    AcmpMessageType::CONNECT_RX_COMMAND,
                    addressing.controller,
                    (talker, talker_unique_id),
                    (addressing.target, input),
                    sequence_id,
                );
                if class_b {
                    connect.flags |= AcmpFlags::CLASS_B;
                }
                connect.encode(&mut out)
            }
            Request::Unbind {
                talker,
                talker_unique_id,
                input,
                ..
            } => acmp(
                AcmpMessageType::DISCONNECT_RX_COMMAND,
                (talker, talker_unique_id),
                input,
                &mut out,
            ),
        };
        match length {
            Ok(length) => {
                self.outgoing
                    .push_back((destination, out[..length].to_vec()));
                true
            }
            Err(_) => false,
        }
    }

    /// How long to wait for the response to `request`: ACMP's timeouts
    /// for the message, Milan's for Milan entities, else 250 ms.
    fn timeout(&self, entity_id: EntityId, request: Request) -> Duration {
        let Some(message_type) = request.acmp_message() else {
            return COMMAND_TIMEOUT;
        };
        let milan = self
            .models
            .get(&entity_id)
            .is_some_and(|model| model.milan.is_some());
        let millis = if milan {
            message_type.milan_timeout_ms()
        } else {
            message_type.timeout_ms()
        };
        Duration::from_millis(u64::from(millis.unwrap_or(250)))
    }

    fn next_command(&mut self) -> CommandId {
        self.next_command = self.next_command.wrapping_add(1);
        CommandId(self.next_command)
    }

    /// Queues a caller's request ahead of the entity's queries, after the
    /// caller's earlier ones so they happen in order, or finishes it at
    /// once if the entity is not online.
    fn queue_command(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        command: CommandId,
        request: Request,
    ) {
        if !self.entities.contains_key(&entity_id) {
            self.events
                .push_back(Event::CommandFinished(command, Outcome::NotPossible));
            return;
        }
        let queue = &mut self.sessions.entry(entity_id).or_default().queue;
        let place = queue
            .iter()
            .position(|queued| queued.command().is_none())
            .unwrap_or(queue.len());
        queue.insert(place, request);
        self.pump(now);
    }

    /// Renames a descriptor (SET_NAME): for the ENTITY, `name_index` 0 is
    /// its entity_name and 1 its group_name; for other descriptors 0 is
    /// the object_name. A name longer than 64 octets of UTF-8 is not
    /// possible.
    pub fn set_name(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        descriptor_type: DescriptorType,
        index: u16,
        name_index: u16,
        name: &str,
    ) -> CommandId {
        let command = self.next_command();
        let Some(name) = aem::aem_name(name) else {
            self.events
                .push_back(Event::CommandFinished(command, Outcome::NotPossible));
            return command;
        };
        let set = SetName {
            descriptor_type,
            index,
            name_index,
            configuration: 0,
            name,
        };
        self.queue_command(now, entity_id, command, Request::SetName { set, command });
        command
    }

    /// Changes a STREAM_INPUT's or STREAM_OUTPUT's format
    /// (SET_STREAM_FORMAT). Entities refuse while the stream runs.
    pub fn set_stream_format(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        descriptor_type: DescriptorType,
        index: u16,
        format: StreamFormat,
    ) -> CommandId {
        let command = self.next_command();
        let set = SetStreamFormat {
            descriptor_type,
            index,
            format,
        };
        let request = Request::SetStreamFormat { set, command };
        self.queue_command(now, entity_id, command, request);
        command
    }

    /// Changes an AUDIO_UNIT's sampling rate (SET_SAMPLING_RATE), then
    /// reads its streams again, whose formats may follow it.
    pub fn set_sampling_rate(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        descriptor_type: DescriptorType,
        index: u16,
        rate: SamplingRate,
    ) -> CommandId {
        let command = self.next_command();
        let set = SetSamplingRate {
            descriptor_type,
            index,
            rate,
        };
        let request = Request::SetSamplingRate { set, command };
        self.queue_command(now, entity_id, command, request);
        command
    }

    /// Allows or disallows an AVB_INTERFACE's talkers escalating streams to
    /// multicast in AVB Lite (SET_LITE_CONFIG, AVB Lite profile 2.4).
    pub fn set_lite_config(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        interface: u16,
        escalation_allowed: bool,
    ) -> CommandId {
        let command = self.next_command();
        let flags = if escalation_allowed {
            lite::LiteConfigFlags::ESCALATION_ALLOWED.0
        } else {
            0
        };
        let request = Request::SetLiteConfig {
            interface,
            flags,
            command,
        };
        self.queue_command(now, entity_id, command, request);
        command
    }

    /// Allows an access point to admit Class A toward its wireless port,
    /// where it delivers 12000 frames a second, or keeps Class A off the hop
    /// (SET_WIRELESS_CONFIG, AVB Wireless profile 4 and 5.2).
    pub fn set_wireless_config(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        interface: u16,
        class_a_allowed: bool,
    ) -> CommandId {
        let command = self.next_command();
        let flags = if class_a_allowed {
            wireless::WirelessConfigFlags::CLASS_A_ALLOWED.0
        } else {
            0
        };
        let request = Request::SetWirelessConfig {
            interface,
            flags,
            command,
        };
        self.queue_command(now, entity_id, command, request);
        command
    }

    /// Picks a CLOCK_DOMAIN's clock source (SET_CLOCK_SOURCE).
    pub fn set_clock_source(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        domain: u16,
        source: u16,
    ) -> CommandId {
        let command = self.next_command();
        let request = Request::SetClockSource {
            set: SetClockSource { domain, source },
            command,
        };
        self.queue_command(now, entity_id, command, request);
        command
    }

    /// Makes `configuration` the entity's current configuration
    /// (SET_CONFIGURATION); once it answers, the entity is read again in
    /// it.
    pub fn set_configuration(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        configuration: u16,
    ) -> CommandId {
        let command = self.next_command();
        let request = Request::SetConfiguration {
            configuration,
            command,
        };
        self.queue_command(now, entity_id, command, request);
        command
    }

    /// Sets a control's current values (SET_CONTROL), encoded as its value
    /// type keeps them (see [`control::encode_values`]); the entity answers
    /// with the values it has after, set or not, and the model takes them.
    /// Up to [`aem::MAX_CONTROL_VALUES`] octets.
    ///
    /// [`control::encode_values`]: crate::control::encode_values
    pub fn set_control(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        index: u16,
        values: &[u8],
    ) -> CommandId {
        let command = self.next_command();
        if values.is_empty() || values.len() > aem::MAX_CONTROL_VALUES {
            self.events
                .push_back(Event::CommandFinished(command, Outcome::NotPossible));
            return command;
        }
        self.control_values.insert(command, values.to_vec());
        self.queue_command(
            now,
            entity_id,
            command,
            Request::SetControl { index, command },
        );
        if !self.entities.contains_key(&entity_id) {
            self.control_values.remove(&command);
        }
        command
    }

    /// Asks a talker what it has of a stream output (GET_TX_STATE): its
    /// stream ID, destination, VLAN and how many listen, which the model
    /// then holds.
    pub fn tx_state(&mut self, now: Instant, talker: (EntityId, u16)) -> CommandId {
        let command = self.next_command();
        let request = Request::GetTxState {
            output: talker.1,
            command: Some(command),
        };
        self.queue_command(now, talker.0, command, request);
        command
    }

    /// Tells a talker to stop sending a stream output to a listener's
    /// stream input (DISCONNECT_TX), without the listener: for a talker
    /// still sending to a listener that went away.
    pub fn disconnect_talker(
        &mut self,
        now: Instant,
        talker: (EntityId, u16),
        listener: (EntityId, u16),
    ) -> CommandId {
        let command = self.next_command();
        let request = Request::DisconnectTx {
            output: talker.1,
            listener: listener.0,
            listener_unique_id: listener.1,
            command,
        };
        self.queue_command(now, talker.0, command, request);
        command
    }

    /// Reads a stream output's max transit time (GET_MAX_TRANSIT_TIME)
    /// into the model.
    pub fn max_transit_time(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        output: u16,
    ) -> CommandId {
        let command = self.next_command();
        let request = Request::GetMaxTransitTime {
            output,
            command: Some(command),
        };
        self.queue_command(now, entity_id, command, request);
        command
    }

    /// Sets a stream output's max transit time in nanoseconds
    /// (SET_MAX_TRANSIT_TIME), zero for the entity's default; only while
    /// the stream is not running.
    pub fn set_max_transit_time(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        output: u16,
        nanoseconds: u64,
    ) -> CommandId {
        let command = self.next_command();
        let request = Request::SetMaxTransitTime {
            set: MaxTransitTime {
                output,
                nanoseconds,
            },
            command,
        };
        self.queue_command(now, entity_id, command, request);
        command
    }

    /// Reads a descriptor again (READ_DESCRIPTOR) into the model, whatever
    /// enumeration read; the command finishes when the entity answers.
    pub fn read_descriptor(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        descriptor_type: DescriptorType,
        index: u16,
    ) -> CommandId {
        let command = self.next_command();
        let request = Request::ReadForCaller {
            descriptor_type,
            index,
            command,
        };
        self.queue_command(now, entity_id, command, request);
        command
    }

    /// Takes the max transit time a response or notification carries for
    /// a stream output into the model.
    fn store_max_transit_time(&mut self, entity_id: EntityId, output: u16, payload: &[u8]) {
        if let Ok(time) = MaxTransitTime::decode(payload)
            && time.output == output
            && let Some(model) = self.models.get_mut(&entity_id)
            && model.has(DescriptorType::STREAM_OUTPUT, output)
            && model.set_max_transit_time(output, time.nanoseconds)
            && model.state == EnumerationState::Complete
        {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    /// Asks an entity outside Milan, asked first for its first stream
    /// output's max transit time alone, for the others it has not told,
    /// once it answers for that one.
    fn read_other_transit_times(&mut self, entity_id: EntityId, answered: u16) {
        let Some(model) = self.models.get(&entity_id) else {
            return;
        };
        if model.milan.is_some() {
            return;
        }
        let mut outputs = model
            .descriptors(DescriptorType::STREAM_OUTPUT)
            .map(|(output, _)| output);
        if outputs.next() != Some(answered) {
            return;
        }
        let others: Vec<u16> = outputs
            .filter(|&output| model.max_transit_time(output).is_none())
            .collect();
        let queue = &mut self.sessions.entry(entity_id).or_default().queue;
        for output in others {
            queue.push_back(Request::GetMaxTransitTime {
                output,
                command: None,
            });
        }
    }

    /// Forgets what a caller's command was to send, once it is answered or
    /// given up.
    fn drop_command_data(&mut self, command: CommandId) {
        self.mapping_changes.remove(&command);
        self.control_values.remove(&command);
    }

    /// Takes a control's current values into its entity's model.
    fn store_control(&mut self, entity_id: EntityId, index: u16, values: &[u8]) {
        if let Some(model) = self.models.get_mut(&entity_id)
            && model.apply_control(index, values)
            && model.state == EnumerationState::Complete
        {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    /// Adds mappings to a stream port's dynamic mappings
    /// (ADD_AUDIO_MAPPINGS): stream channels to an input's cluster
    /// channels, or an output's cluster channels to stream channels. The
    /// entity refuses the whole change if any mapping is not valid, and
    /// may replace a mapping to the same channel; the map is read again
    /// after. From 1 to [`aem::MAX_MAPPINGS_PER_CHANGE`] mappings.
    pub fn add_audio_mappings(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        port: (DescriptorType, u16),
        mappings: &[AudioMapping],
    ) -> CommandId {
        self.change_mappings(now, entity_id, MappingChange::Add, port, mappings)
    }

    /// Removes mappings from a stream port's dynamic mappings
    /// (REMOVE_AUDIO_MAPPINGS), as for
    /// [`add_audio_mappings`](Self::add_audio_mappings).
    pub fn remove_audio_mappings(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        port: (DescriptorType, u16),
        mappings: &[AudioMapping],
    ) -> CommandId {
        self.change_mappings(now, entity_id, MappingChange::Remove, port, mappings)
    }

    fn change_mappings(
        &mut self,
        now: Instant,
        entity_id: EntityId,
        change: MappingChange,
        (descriptor_type, index): (DescriptorType, u16),
        mappings: &[AudioMapping],
    ) -> CommandId {
        let command = self.next_command();
        if mappings.is_empty() || mappings.len() > aem::MAX_MAPPINGS_PER_CHANGE {
            self.events
                .push_back(Event::CommandFinished(command, Outcome::NotPossible));
            return command;
        }
        self.mapping_changes.insert(command, mappings.to_vec());
        let request = Request::ChangeMappings {
            change,
            descriptor_type,
            index,
            command,
        };
        self.queue_command(now, entity_id, command, request);
        command
    }

    /// Binds a listener's stream input to a talker's stream output
    /// (CONNECT_RX, which Milan calls BIND_RX). Where the talker reports
    /// its output in class B, the connect carries CLASS_B, which the
    /// listener passes on to the talker: a talker that takes its class
    /// from the connect would otherwise go to class A.
    pub fn connect(
        &mut self,
        now: Instant,
        talker: (EntityId, u16),
        listener: (EntityId, u16),
    ) -> CommandId {
        let command = self.next_command();
        let class_b = self
            .models
            .get(&talker.0)
            .and_then(|model| model.stream_info(DescriptorType::STREAM_OUTPUT, talker.1))
            .is_some_and(|info| info.flags.contains(StreamInfoFlags::CLASS_B));
        let request = Request::Bind {
            talker: talker.0,
            talker_unique_id: talker.1,
            input: listener.1,
            class_b,
            command,
        };
        self.queue_command(now, listener.0, command, request);
        command
    }

    /// Unbinds a listener's stream input (DISCONNECT_RX, which Milan calls
    /// UNBIND_RX), naming the talker it is bound to when that is known.
    pub fn disconnect(&mut self, now: Instant, listener: (EntityId, u16)) -> CommandId {
        let command = self.next_command();
        let talker = self
            .models
            .get(&listener.0)
            .and_then(|model| model.binding(listener.1))
            .map_or((EntityId(0), 0), |binding| {
                (binding.talker, binding.talker_unique_id)
            });
        let request = Request::Unbind {
            talker: talker.0,
            talker_unique_id: talker.1,
            input: listener.1,
            command,
        };
        self.queue_command(now, listener.0, command, request);
        command
    }

    /// Turns on an entity's identify control, the one its advertisement
    /// names, and turns it off again after `duration`.
    pub fn identify(&mut self, now: Instant, entity_id: EntityId, duration: Duration) -> CommandId {
        let command = self.next_command();
        let control = self.entities.get(&entity_id).and_then(|entity| {
            entity
                .adp
                .entity_capabilities
                .contains(EntityCapabilities::AEM_IDENTIFY_CONTROL_INDEX_VALID)
                .then_some(entity.adp.identify_control_index)
        });
        let Some(control) = control else {
            self.events
                .push_back(Event::CommandFinished(command, Outcome::NotPossible));
            return command;
        };
        self.identify_off
            .insert(entity_id, (now + duration, control));
        let request = Request::Identify {
            control,
            on: true,
            command: Some(command),
        };
        self.queue_command(now, entity_id, command, request);
        command
    }

    /// Forgets entities whose valid time ran out by `now`, retries or
    /// gives up on commands not answered in time, and advertises when due.
    pub fn handle_timeout(&mut self, now: Instant) {
        self.start_advertising(now);
        let mut expired = Vec::new();
        self.entities.retain(|&entity_id, entity| {
            let keep = entity.expires > now;
            if !keep {
                expired.push(entity_id);
            }
            keep
        });
        for entity_id in expired {
            self.forget(entity_id);
            self.events
                .push_back(Event::EntityOffline(entity_id, OfflineReason::TimedOut));
        }

        let late: Vec<((Channel, u16), Inflight)> = self
            .inflight
            .iter()
            .filter(|(_, inflight)| inflight.deadline <= now)
            .map(|(&key, &inflight)| (key, inflight))
            .collect();
        for (key, inflight) in late {
            if !inflight.retried {
                let retry = Inflight {
                    deadline: now + self.timeout(inflight.entity_id, inflight.request),
                    retried: true,
                    ..inflight
                };
                self.inflight.insert(key, retry);
                self.send(key, retry);
                continue;
            }
            self.complete(key, inflight);
            if let Request::ReadDescriptor {
                descriptor_type,
                refresh,
                ..
            } = inflight.request
            {
                self.read_failed(
                    inflight.entity_id,
                    descriptor_type,
                    refresh,
                    EnumerationFailure::NoResponse,
                );
            }
            // An entity that never answered the query is not asked again.
            if let Request::GetLiteStatus { .. } = inflight.request
                && let Some(model) = self.models.get_mut(&inflight.entity_id)
                && model.lite_supported.is_none()
            {
                model.lite_supported = Some(false);
            }
            // An entity that does not answer is asked again at the next
            // poll, and taken not to answer only after a few silences.
            if let Request::GetWirelessStatus { .. } = inflight.request
                && let Some(model) = self.models.get_mut(&inflight.entity_id)
                && model.wireless_supported.is_none()
            {
                model.wireless_misses += 1;
                if model.wireless_misses >= WIRELESS_MISSES {
                    model.wireless_supported = Some(false);
                } else if !self.config.wireless_poll.is_zero() {
                    self.next_wireless_poll
                        .get_or_insert(now + self.config.wireless_poll);
                }
            }
            if let Request::GetDynamicInfo { queries, count } = inflight.request {
                self.dynamic_info_answered(
                    inflight.entity_id,
                    &queries[..usize::from(count)],
                    None,
                );
            }
            if let Some(command) = inflight.request.command() {
                self.drop_command_data(command);
                self.events
                    .push_back(Event::CommandFinished(command, Outcome::NoResponse));
            }
            self.check_complete(inflight.entity_id);
        }

        let due: Vec<(EntityId, u16)> = self
            .identify_off
            .iter()
            .filter(|(_, (when, _))| *when <= now)
            .map(|(&entity_id, &(_, control))| (entity_id, control))
            .collect();
        for (entity_id, control) in due {
            self.identify_off.remove(&entity_id);
            self.queue_query(
                entity_id,
                Request::Identify {
                    control,
                    on: false,
                    command: None,
                },
            );
        }

        self.poll_lite(now);
        self.poll_wireless(now);
        self.age_cvu_talkers(now);

        if let Some(advertiser) = &mut self.advertiser
            && advertiser.next.is_some_and(|next| next <= now)
        {
            let index = advertiser.available_index;
            advertiser.available_index = index.wrapping_add(1);
            advertiser.next = Some(now + advertiser.valid() / 4);
            let adpdu = self.own_adpdu(AdpMessageType::ENTITY_AVAILABLE, index);
            self.queue_adp(&adpdu);
        }
        self.pump(now);
    }

    /// Asks each entity that answers the AVB Lite status query again, every
    /// `lite_poll`.
    fn poll_lite(&mut self, now: Instant) {
        if self.next_lite_poll.is_none_or(|next| next > now) {
            return;
        }
        let asked: Vec<(EntityId, u16)> = self
            .models
            .iter()
            .filter(|(_, model)| model.lite_supported == Some(true))
            .flat_map(|(&entity_id, model)| {
                model
                    .descriptors(DescriptorType::AVB_INTERFACE)
                    .map(move |(interface, _)| (entity_id, interface))
            })
            .collect();
        // The timer stops once no entity answers the query.
        self.next_lite_poll = (!asked.is_empty()).then(|| now + self.config.lite_poll);
        for (entity_id, interface) in asked {
            self.queue_query(entity_id, Request::GetLiteStatus { interface });
        }
    }

    /// Asks each wireless interface for its AVB Wireless status again,
    /// every `wireless_poll`.
    fn poll_wireless(&mut self, now: Instant) {
        if self.next_wireless_poll.is_none_or(|next| next > now) {
            return;
        }
        let mut asked: Vec<(EntityId, u16)> = self
            .models
            .iter()
            .flat_map(|(&entity_id, model)| {
                model
                    .wireless_interfaces()
                    .map(move |(interface, _)| (entity_id, interface))
            })
            .collect();
        // Entities whose queries went unanswered are asked again, every
        // AVB interface, until they answer or have missed too often.
        asked.extend(
            self.models
                .iter()
                .filter(|(_, model)| {
                    model.wireless_supported.is_none() && model.wireless_misses > 0
                })
                .flat_map(|(&entity_id, model)| {
                    model
                        .descriptors(DescriptorType::AVB_INTERFACE)
                        .map(move |(interface, _)| (entity_id, interface))
                }),
        );
        // The timer stops once no interface is wireless or waits for an
        // answer.
        self.next_wireless_poll = (!asked.is_empty()).then(|| now + self.config.wireless_poll);
        for (entity_id, interface) in asked {
            self.queue_query(entity_id, Request::GetWirelessStatus { interface });
        }
    }

    /// Forgets CVU SRP declarations not refreshed for [`CVU_LIFETIME`].
    fn age_cvu_talkers(&mut self, now: Instant) {
        let Some(before) = now.checked_sub(CVU_LIFETIME) else {
            return;
        };
        let mut aged = Vec::new();
        for (&entity_id, model) in &mut self.models {
            if model.age_cvu_talkers(before) {
                aged.push(entity_id);
            }
        }
        for entity_id in aged {
            self.events.push_back(Event::EntityModelChanged(entity_id));
        }
    }

    /// Schedules the first advertisement, a random moment after the
    /// controller first sees the time.
    fn start_advertising(&mut self, now: Instant) {
        if let Some(advertiser) = &mut self.advertiser
            && !advertiser.started
        {
            advertiser.started = true;
            advertiser.next = Some(now + self.random.up_to(advertiser.delay_limit()));
        }
    }

    fn own_adpdu(&self, message_type: AdpMessageType, available_index: u32) -> Adpdu {
        let advertise = self.advertiser.as_ref().map(|advertiser| advertiser.config);
        Adpdu {
            message_type,
            valid_time: if message_type == AdpMessageType::ENTITY_AVAILABLE {
                advertise.map_or(31, |advertise| advertise.valid_time.clamp(1, 31))
            } else {
                0
            },
            entity_id: self.config.entity_id,
            entity_model_id: advertise
                .map_or_else(Default::default, |advertise| advertise.entity_model_id),
            controller_capabilities: ControllerCapabilities::IMPLEMENTED,
            available_index,
            ..Adpdu::default()
        }
    }

    fn queue_adp(&mut self, adpdu: &Adpdu) {
        if let Ok(bytes) = adpdu.to_bytes() {
            self.outgoing
                .push_back((crate::ADP_ACMP_MULTICAST, bytes.to_vec()));
        }
    }

    /// When [`handle_timeout`](Self::handle_timeout) next has work.
    pub fn poll_timeout(&self) -> Option<Instant> {
        let expiries = self.entities.values().map(|entity| entity.expires);
        let deadlines = self.inflight.values().map(|inflight| inflight.deadline);
        let advertise = self
            .advertiser
            .as_ref()
            .and_then(|advertiser| advertiser.next);
        let identify = self.identify_off.values().map(|(when, _)| *when);
        let lite = self.next_lite_poll;
        let wireless = self.next_wireless_poll;
        let declarations = self
            .models
            .values()
            .filter_map(EntityModel::oldest_cvu_talker)
            .map(|heard| heard + CVU_LIFETIME);
        expiries
            .chain(deadlines)
            .chain(advertise)
            .chain(identify)
            .chain(lite)
            .chain(wireless)
            .chain(declarations)
            .min()
    }

    /// Copies the next frame to send into `out`. A frame that does not
    /// fit stays queued; 1500 octets always suffice.
    pub fn poll_transmit(&mut self, out: &mut [u8]) -> Result<Option<Transmit>, EncodeError> {
        let Some((destination, bytes)) = self.outgoing.front() else {
            return Ok(None);
        };
        let length = bytes.len();
        let Some(target) = out.get_mut(..length) else {
            return Err(EncodeError::BufferTooSmall {
                needed: length,
                available: out.len(),
            });
        };
        target.copy_from_slice(bytes);
        let destination = *destination;
        self.outgoing.pop_front();
        Ok(Some(Transmit {
            destination,
            length,
        }))
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        self.events.pop_front()
    }

    pub fn entity(&self, entity_id: EntityId) -> Option<&DiscoveredEntity> {
        self.entities.get(&entity_id)
    }

    /// The entities online, by entity ID.
    pub fn entities(&self) -> impl Iterator<Item = &DiscoveredEntity> {
        self.entities.values()
    }

    /// What has been read of an entity's model.
    pub fn model(&self, entity_id: EntityId) -> Option<&EntityModel> {
        self.models.get(&entity_id)
    }

    /// Whether commands to the entity are queued or waiting for an answer,
    /// such as the queries that follow reading its descriptors.
    pub fn busy(&self, entity_id: EntityId) -> bool {
        self.sessions
            .get(&entity_id)
            .is_some_and(|session| session.inflight.is_some() || !session.queue.is_empty())
    }

    /// Frames received that did not decode.
    pub fn malformed_frames(&self) -> u64 {
        self.malformed
    }
}

/// Whether an ACMP message goes to or comes from the talker: the TX
/// commands and their responses.
fn to_talker(message: AcmpMessageType) -> bool {
    matches!(
        message.0 & !1,
        0 | 2 | 4 | 12 // CONNECT_TX, DISCONNECT_TX, GET_TX_STATE, GET_TX_CONNECTION
    )
}

/// The values a SET_CONTROL or GET_CONTROL response or notification for
/// CONTROL `index` carries.
fn control_values(payload: &[u8], index: u16) -> Option<&[u8]> {
    let (descriptor_type, named) = aem::target_descriptor(payload)?;
    (descriptor_type == DescriptorType::CONTROL && named == index)
        .then(|| payload.get(4..))
        .flatten()
        .filter(|values| !values.is_empty())
}

fn command_type_of(request: Request) -> AemCommandType {
    match request {
        Request::ReadDescriptor { .. } => AemCommandType::READ_DESCRIPTOR,
        Request::RegisterUnsolicited => AemCommandType::REGISTER_UNSOLICITED_NOTIFICATION,
        Request::GetStreamInfo { .. } => AemCommandType::GET_STREAM_INFO,
        Request::GetAvbInfo { .. } => AemCommandType::GET_AVB_INFO,
        Request::GetAsPath { .. } => AemCommandType::GET_AS_PATH,
        Request::GetCounters { .. } => AemCommandType::GET_COUNTERS,
        Request::Identify { .. } => AemCommandType::SET_CONTROL,
        Request::SetName { .. } => AemCommandType::SET_NAME,
        Request::SetStreamFormat { .. } => AemCommandType::SET_STREAM_FORMAT,
        Request::SetSamplingRate { .. } => AemCommandType::SET_SAMPLING_RATE,
        Request::SetClockSource { .. } => AemCommandType::SET_CLOCK_SOURCE,
        Request::SetConfiguration { .. } => AemCommandType::SET_CONFIGURATION,
        Request::SetControl { .. } => AemCommandType::SET_CONTROL,
        Request::GetMaxTransitTime { .. } => AemCommandType::GET_MAX_TRANSIT_TIME,
        Request::SetMaxTransitTime { .. } => AemCommandType::SET_MAX_TRANSIT_TIME,
        Request::ReadForCaller { .. } | Request::ReadOther { .. } => {
            AemCommandType::READ_DESCRIPTOR
        }
        Request::GetDynamicInfo { .. } => AemCommandType::GET_DYNAMIC_INFO,
        Request::GetAudioMap { .. } => AemCommandType::GET_AUDIO_MAP,
        Request::ChangeMappings { change, .. } => change.command_type(),
        // Not AEM commands; never match an AEM response.
        Request::GetMilanInfo
        | Request::GetMediaClockReference { .. }
        | Request::GetLiteStatus { .. }
        | Request::SetLiteConfig { .. }
        | Request::GetWirelessStatus { .. }
        | Request::SetWirelessConfig { .. }
        | Request::GetRxState { .. }
        | Request::Bind { .. }
        | Request::Unbind { .. }
        | Request::GetTxState { .. }
        | Request::DisconnectTx { .. } => AemCommandType(0xffff),
    }
}

/// What changes on an entity in a descriptor of this type: its name, and
/// for a stream its format, an audio unit its sampling rate and a clock
/// domain its clock source.
fn dynamic_queries(descriptor_type: DescriptorType, index: u16) -> Vec<DynamicQuery> {
    let mut queries = Vec::new();
    if descriptor_type.has_object_name() {
        queries.push(DynamicQuery::Name {
            descriptor_type,
            index,
            name_index: 0,
        });
    }
    match descriptor_type {
        DescriptorType::STREAM_INPUT | DescriptorType::STREAM_OUTPUT => {
            queries.push(DynamicQuery::StreamFormat {
                descriptor_type,
                index,
            });
        }
        DescriptorType::AUDIO_UNIT => queries.push(DynamicQuery::SamplingRate {
            descriptor_type,
            index,
        }),
        DescriptorType::CLOCK_DOMAIN => queries.push(DynamicQuery::ClockSource { domain: index }),
        _ => {}
    }
    queries
}

/// The first GET_AUDIO_MAP of each of an entity's stream ports with
/// dynamic mappings.
fn audio_map_reads(model: &EntityModel) -> Vec<Request> {
    [true, false]
        .into_iter()
        .flat_map(|input| model.stream_ports(input))
        .filter(|port| port.has_dynamic_mappings())
        .map(|port| Request::GetAudioMap {
            descriptor_type: port.descriptor_type,
            index: port.index,
            map_index: 0,
        })
        .collect()
}

/// How an AEM command the caller asked for went, by its response.
fn aem_outcome(aem: &AemPdu<'_>) -> Outcome {
    if aem.status().is_success() {
        Outcome::Done
    } else {
        Outcome::Refused(Refusal::Aem(aem.status()))
    }
}

/// The response, if it decoded and is for the descriptor asked for.
fn matching<'a>(
    response: Result<ReadDescriptorResponse<'a>, DecodeError>,
    descriptor_type: DescriptorType,
    index: u16,
) -> Option<ReadDescriptorResponse<'a>> {
    response
        .ok()
        .filter(|response| response.descriptor_type == descriptor_type && response.index == index)
}

/// The same advertisement apart from the fields that change on every one
/// or do not describe the entity.
fn same_advertisement(left: &Adpdu, right: &Adpdu) -> bool {
    let normalized = |adpdu: &Adpdu| Adpdu {
        available_index: 0,
        valid_time: 0,
        ..*adpdu
    };
    normalized(left) == normalized(right)
}

#[cfg(test)]
mod tests;
