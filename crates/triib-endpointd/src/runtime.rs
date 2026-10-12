//! The running endpoints: one thread taking frames from the sockets and
//! ptp4l's state, driving each entity and the MSRP and MVRP participants,
//! and starting and stopping the streams as reservations come and go.

use std::collections::{HashMap, VecDeque};
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
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
use avb_net::Socket;
use avb_net::stream::FrameSender;
use triib_stream::audio::{Sink, Source};
use triib_stream::{Listener, ListenerConfig, MediaClock, Talker, TalkerConfig};

use crate::config::{Config, EndpointConfig, Kind, LiteChoice, SrClass, binding_text};
use crate::gptp;
use crate::lite::{self, Declared, Fallback, Ptp4l};
use crate::port::{Port, VLAN};
use crate::profile::{self, Profile};
use crate::status::{DaemonStatus, EndpointStatus, status_path};

/// Entity model IDs under Scramble Tools' MA-S `8C-1F-64-36-C`.
const TALKER_MODEL: EntityModelId = EntityModelId(0x8c1f_6436_c000_0002);
const LISTENER_MODEL: EntityModelId = EntityModelId(0x8c1f_6436_c000_0003);
/// Class A's priority; its VLAN is the port's.
const PRIORITY: u8 = 3;
/// Class B's priority.
const CLASS_B_PRIORITY: u8 = 2;
/// Class B's interval, which a reservation counts its frames over.
const CLASS_B_INTERVAL: u64 = 250_000;
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
/// AVB Lite's media priority (profile 7), the unicast copies a talker
/// sends before it falls back to multicast, how often CVU SRP declarations
/// AVB Lite's priority for streams and CVU SRP, and the broadcast
/// address talkers declare to.
const LITE_PRIORITY: u8 = 5;
const BROADCAST: MacAddress = MacAddress([0xff; 6]);
/// The unicast fan-out the profile recommends.
const FANOUT: u8 = 2;
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
}

/// The rates this computer's endpoints offer: those class A's 8000
/// frames a second divide.
const RATES: [u32; 3] = [48_000, 96_000, 192_000];
/// Frames a second: one each class A interval, as Milan's formats send,
/// or one each class B interval, as a stream across a Wi-Fi hop sends at
/// half the frames.
const CLASS_A_FRAMES: u32 = 8000;
const CLASS_B_FRAMES: u32 = 4000;

/// The most octets an AVTPDU takes in one Ethernet frame.
const MOST_PDU: usize = 1500;

/// AAF at `rate`, sent `frames` times a second, 32-bit, in `bits` of
/// them.
fn aaf(rate: u32, channels: u16, bits: u8, frames: u32) -> StreamFormat {
    let nsr: u64 = match rate {
        96_000 => 0x07,
        192_000 => 0x09,
        _ => 0x05,
    };
    StreamFormat(
        (0x02 << 56)
            | (nsr << 48)
            | (0x02 << 40)
            | (u64::from(bits) << 32)
            | (u64::from(channels & 0x3ff) << 22)
            | (u64::from(rate / frames) << 12),
    )
}

/// IEC 61883-6 AM824 at `rate`, non-blocking, every quadlet multi-bit
/// linear audio, as the MOTU 8D and macOS use.
fn am824(rate: u32, channels: u16) -> StreamFormat {
    let sfc: u64 = match rate {
        96_000 => 0x04,
        192_000 => 0x06,
        _ => 0x02,
    };
    let quadlets = u64::from(channels.min(255));
    StreamFormat((0xa0 << 48) | (sfc << 40) | (quadlets << 32) | (0x40 << 24) | (quadlets << 8))
}

/// The formats offered for `channels`: each packing at every rate where
/// all of them fit one Ethernet frame, so a change of rate finds the
/// same packing there. Milan's, a frame each class A interval, come
/// first; AAF a frame each class B interval follows, at half the frames,
/// where it fits, and comes first for a class B talker, whose default it
/// is.
fn offered_formats(channels: u16, class_b: bool) -> Vec<StreamFormat> {
    let fits = |formats: &[StreamFormat]| {
        formats.iter().all(|format| {
            triib_stream::media::Media::of(*format)
                .is_some_and(|media| media.pdu_length() <= MOST_PDU)
        })
    };
    let milan: Vec<StreamFormat> = RATES
        .iter()
        .map(|rate| {
            [
                aaf(*rate, channels, 32, CLASS_A_FRAMES),
                aaf(*rate, channels, 24, CLASS_A_FRAMES),
                am824(*rate, channels),
            ]
        })
        .filter(|formats| fits(formats))
        .flatten()
        .collect();
    let wireless: Vec<StreamFormat> = RATES
        .iter()
        .map(|rate| {
            [
                aaf(*rate, channels, 32, CLASS_B_FRAMES),
                aaf(*rate, channels, 24, CLASS_B_FRAMES),
            ]
        })
        .filter(|formats| fits(formats))
        .flatten()
        .collect();
    if class_b {
        [wireless, milan].concat()
    } else {
        [milan, wireless].concat()
    }
}

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

/// The stream a talker's output sends.
fn output_stream(mac: MacAddress, instance: u8, index: u16) -> StreamId {
    StreamId::new(mac, (u16::from(instance) << 8) | index)
}

struct Endpoint {
    config: EndpointConfig,
    entity: Entity,
    talker: Option<Talker>,
    listener: Option<Listener>,
    /// The stream its Listener declaration names, while it declares one.
    declared: Option<Vec<u8>>,
    /// When a talker whose stream changed declares it again.
    redeclare_at: Option<Duration>,
    /// When a talker last withdrew its declaration.
    withdrawn_at: Option<Duration>,
    /// The stream a listener's thread takes.
    listening_to: Option<atdecc::entity::ProbedStream>,
    /// In AVB Lite, the stream a listener declares itself for, the
    /// talker it tells and what; for a talker, where its copies go.
    cvu_to: Option<(u64, MacAddress, ListenerState)>,
    copies: Vec<MacAddress>,
    /// For a talker in AVB Lite: whether its stream went multicast, when
    /// its frames move there, and the listeners it refuses, with the
    /// failure code each has.
    escalated: bool,
    escalate_at: Option<Duration>,
    refused: HashMap<MacAddress, u8>,
    /// Its place in the configuration's list.
    place: usize,
}

impl Endpoint {
    fn new(
        config: EndpointConfig,
        instance: u8,
        mac: MacAddress,
        interface: &str,
        clock: ClockIdentity,
    ) -> Self {
        // As many as six samples of each fit in one Ethernet frame, at
        // 48 kHz; fewer offer the higher rates too.
        let channels = config.channels.clamp(1, 60);
        let talker = config.kind == Kind::Talker;
        let formats = offered_formats(channels, talker && config.class == SrClass::B);
        // The format chosen last, with the channels there are now.
        let current_format = config
            .stream_format()
            .and_then(|wanted| {
                formats
                    .iter()
                    .copied()
                    .find(|offered| same_packing(*offered, wanted))
            })
            .unwrap_or(formats[0]);
        let mut sampling_rates: Vec<u32> = formats
            .iter()
            .filter_map(|format| format.sample_rate())
            .collect();
        sampling_rates.dedup();
        let current_sampling_rate = current_format.sample_rate().unwrap_or(48_000);
        let stream = StreamModel {
            name: config.name.clone(),
            current_format,
            formats,
            channels,
            class_b: talker && config.class == SrClass::B,
        };
        let model = EndpointModel {
            entity_id: entity_id(mac, instance),
            entity_model_id: if talker { TALKER_MODEL } else { LISTENER_MODEL },
            entity_name: config.name.clone(),
            group_name: String::new(),
            vendor_name: "Scramble Tools".into(),
            model_name: if talker {
                "triib talker"
            } else {
                "triib listener"
            }
            .into(),
            firmware_version: env!("CARGO_PKG_VERSION").into(),
            serial_number: String::new(),
            mac,
            interface_name: interface.into(),
            clock_identity: clock,
            sampling_rates,
            current_sampling_rate,
            outputs: if talker { vec![stream.clone()] } else { vec![] },
            inputs: if talker { vec![] } else { vec![stream] },
            clock_source: 0,
        };
        // A talker's stream has no destination until MAAP gives it one.
        let outputs = if talker {
            vec![(output_stream(mac, instance, 0), UNADDRESSED)]
        } else {
            vec![]
        };
        Endpoint {
            config,
            entity: Entity::new(model, &outputs),
            talker: None,
            listener: None,
            declared: None,
            redeclare_at: None,
            withdrawn_at: None,
            listening_to: None,
            cvu_to: None,
            copies: Vec::new(),
            escalated: false,
            escalate_at: None,
            refused: HashMap::new(),
            place: 0,
        }
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
    /// Until when the link coming up is the link unit's doing, where AVB
    /// Lite held as the daemon ran it.
    link_own_until: Option<Duration>,
    clock: Arc<MediaClock>,
    /// The talker declarations registered, by stream ID.
    talkers: HashMap<u64, TalkerDeclaration>,
    /// Our talkers' streams a listener on the network is ready for.
    ready_remotely: HashMap<u64, bool>,
    stop: Arc<AtomicBool>,
    inputs: Receiver<Input>,
    last_report: Clock,
    last_counters: Clock,
    /// The configuration running, its file, and when that last changed.
    config: Config,
    path: Option<PathBuf>,
    modified: Option<SystemTime>,
    last_look: Clock,
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
    /// from which endpoint, where to, and what.
    repeats: Vec<(Duration, usize, MacAddress, avb_mrp::mrpdu::Message)>,
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
        if found.hardware_clock.is_none() {
            log(format!(
                "{interface} has no PTP hardware clock: presentation times will mean nothing"
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
        let talkers = endpoints
            .iter()
            .filter(|endpoint| endpoint.config.kind == Kind::Talker)
            .count();
        let maap = (talkers > 0).then(|| Maap::new(mac, talkers as u16));
        let fallback = Fallback::new(
            own_clock.0,
            config.avb_lite == LiteChoice::On,
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
            port: Port::new(seed),
            foreign_mrp: None,
            maap,
            tagged: FrameSender::open(&interface)?,
            ptp,
            fallback,
            lite: None,
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
            link_own_until: None,
            clock,
            talkers: HashMap::new(),
            ready_remotely: HashMap::new(),
            stop,
            inputs,
            last_report: Clock::now(),
            last_counters: Clock::now(),
            modified: modified(path.as_ref()),
            path,
            config,
            last_look: Clock::now(),
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
            endpoint.entity.start(now);
            // A listener binds again to what it was bound to.
            if let Some((talker, output)) = endpoint.config.bound_to() {
                endpoint.entity.restore_binding(
                    now,
                    0,
                    InputBinding {
                        talker,
                        talker_unique_id: output,
                        controller: EntityId(0),
                        flags: AcmpFlags::empty(),
                    },
                );
            }
            log(format!(
                "{} {} as {}",
                match endpoint.config.kind {
                    Kind::Talker => "talker",
                    Kind::Listener => "listener",
                },
                endpoint.config.name,
                endpoint.entity.entity_id()
            ));
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
            if self.last_look.elapsed() >= Duration::from_secs(2) {
                self.last_look = Clock::now();
                self.write_status();
                if let Some(config) = self.changed_config() {
                    log("endpoints.toml changed, starting again");
                    self.shut_down();
                    return Ok(Exit::Reload(config));
                }
            }
        }
        self.shut_down();
        if let Some(path) = status_path() {
            let _ = std::fs::remove_file(path);
        }
        Ok(Exit::Stop)
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
                log(format!("endpoints.toml: {error}"));
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
        let format = model
            .outputs
            .iter()
            .chain(&model.inputs)
            .next()
            .map(|stream| format!("{:#018x}", stream.current_format.0));
        let bound = endpoint
            .entity
            .input_binding(0)
            .map(|binding| binding_text(binding.talker, binding.talker_unique_id));
        let instance = endpoint.config.instance_at(endpoint.place);
        let place = endpoint.place;
        let keep = |entry: &mut EndpointConfig| {
            let same = (&entry.name, &entry.format, &entry.bound) == (&name, &format, &bound);
            entry.name.clone_from(&name);
            entry.format.clone_from(&format);
            entry.bound.clone_from(&bound);
            !same
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
                log(format!("endpoints.toml: {error}"));
            }
            return;
        }
        if let Err(error) = triib_store::save(&path, &self.config) {
            log(format!("endpoints.toml: {error}"));
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
                log(format!("endpoints.toml: {error}"));
            }
            return;
        }
        if let Err(error) = triib_store::save(&path, &self.config) {
            log(format!("endpoints.toml: {error}"));
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
            .map(|endpoint| EndpointStatus {
                entity_id: endpoint.entity.entity_id().to_string(),
                name: endpoint.entity.model().entity_name.clone(),
                kind: endpoint.config.kind,
                audio: match endpoint.config.kind {
                    Kind::Talker => endpoint
                        .config
                        .source
                        .clone()
                        .unwrap_or_else(|| "silence".into()),
                    Kind::Listener => endpoint
                        .config
                        .sink
                        .clone()
                        .unwrap_or_else(|| "discard".into()),
                },
                channels: endpoint
                    .entity
                    .model()
                    .outputs
                    .iter()
                    .chain(&endpoint.entity.model().inputs)
                    .next()
                    .map_or(0, |stream| stream.channels),
                state: match endpoint.config.kind {
                    Kind::Talker if endpoint.talker.is_some() => "streaming",
                    Kind::Talker => "waiting",
                    Kind::Listener if endpoint.listener.is_some() => "listening",
                    Kind::Listener if endpoint.entity.input_binding(0).is_some() => "bound",
                    Kind::Listener => "unbound",
                }
                .into(),
            })
            .collect();
        let status = DaemonStatus {
            pid: std::process::id(),
            started: self.started,
            interface: self.interface.clone(),
            gptp: self.gptp_text.clone(),
            foreign_mrp: self.foreign_mrp_heard(),
            endpoints,
        };
        let _ = triib_store::save(&path, &status);
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
                .filter_map(|endpoint| endpoint.escalate_at)
                .min(),
        ]
        .into_iter()
        .flatten()
        .min()
        .map(|at| at.saturating_sub(since));
        [entity, mrp, maap, lite, Some(Duration::from_millis(250))]
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
                self.fallback.handle_ptp(since, &bytes);
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
                self.update_ptp4l();
                self.update_correction();
                self.gptp_text = match &status {
                    Some(status) => format!(
                        "grandmaster {}, {}",
                        status.gptp.grandmaster,
                        if status.gptp.as_capable {
                            "asCapable"
                        } else {
                            "not asCapable"
                        }
                    ),
                    None => "ptp4l does not answer for this interface".into(),
                };
                match &status {
                    Some(status) => log(format!(
                        "gPTP: grandmaster {}, peer delay {} ns, {}",
                        status.gptp.grandmaster,
                        status.gptp.propagation_delay,
                        if status.gptp.as_capable {
                            "asCapable"
                        } else {
                            "not asCapable"
                        }
                    )),
                    None => log("gPTP: ptp4l does not answer for this interface"),
                }
                let gptp = status.map(|status| status.gptp).unwrap_or_default();
                for endpoint in &mut self.endpoints {
                    endpoint.entity.set_gptp(now, gptp.clone());
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
                }
                // AVB Lite holds until the link comes up again, though not
                // from a link unit taking it down.
                let came_up = link.carrier == Some(true)
                    && last.is_some_and(|last| last.carrier == Some(false))
                    && !self.link_own_until.is_some_and(|until| since < until);
                if came_up && self.lite.is_some() && self.config.avb_lite == LiteChoice::Auto {
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
        self.fallback.handle_timeout(since);
        if self.steer_pending && !self.switched_at.is_some_and(|at| since < at + SWITCH_AGAIN) {
            self.steer_profile();
        }
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
        for index in 0..self.endpoints.len() {
            if addressed
                && self.endpoints[index]
                    .redeclare_at
                    .is_some_and(|at| at <= since)
            {
                self.endpoints[index].redeclare_at = None;
                self.declare_talker(index);
                self.update_talker(index);
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
        // frames for each other go to them here as well as on the wire.
        loop {
            let mut sent = Vec::new();
            for (index, endpoint) in self.endpoints.iter_mut().enumerate() {
                while let Some((destination, bytes)) = endpoint.entity.poll_transmit() {
                    let _ = self.avtp.send(destination, &bytes);
                    if destination == ADP_ACMP_MULTICAST || destination == self.mac {
                        sent.push((index, bytes));
                    }
                }
            }
            if sent.is_empty() || self.endpoints.len() < 2 {
                break;
            }
            for (from, bytes) in sent {
                for (index, endpoint) in self.endpoints.iter_mut().enumerate() {
                    if index != from {
                        endpoint.entity.handle_frame(now, self.mac, &bytes);
                    }
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

    /// The priority a talker endpoint's frames go out on: AVB Lite's, else
    /// its SR class's.
    fn priority(&self, index: usize) -> u8 {
        if self.lite.is_some() {
            LITE_PRIORITY
        } else if self.endpoints[index].config.class == SrClass::B {
            CLASS_B_PRIORITY
        } else {
            PRIORITY
        }
    }

    /// The Talker Advertise for a talker endpoint's stream, from its
    /// current format.
    fn declaration(&self, index: usize) -> Option<TalkerDeclaration> {
        let endpoint = &self.endpoints[index];
        let (stream_id, destination) = endpoint.entity.output_stream(0)?;
        let format = endpoint.entity.model().outputs.first()?.current_format;
        let media = triib_stream::media::Media::of(format)?;
        Some(TalkerDeclaration {
            stream_id: stream_id.0,
            destination,
            vlan_id: if self.lite.is_some() {
                self.media_vlan()
            } else {
                VLAN
            },
            max_frame_size: media.pdu_length() as u16,
            // A class B interval holds as many of the stream's frames as
            // go out in 250 µs, two at 6 samples a frame and 48 kHz.
            max_interval_frames: if endpoint.config.class == SrClass::B {
                (CLASS_B_INTERVAL / media.interval_nanos().max(1)).max(1) as u16
            } else {
                1
            },
            priority: self.priority(index),
            rank: true,
            accumulated_latency: TALKER_LATENCY,
            failure: None,
            event: avb_mrp::Event::New,
        })
    }

    fn declare_talker(&mut self, index: usize) {
        let Some(declaration) = self.declaration(index) else {
            return;
        };
        if self.lite.is_some()
            && let Some(declaration) = self.lite_declaration(index)
        {
            self.declare_cvu(
                index,
                BROADCAST,
                lite::talker_message(&declaration, avb_mrp::Event::New),
            );
            self.endpoints[index].entity.set_output_reservation(
                0,
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
        self.endpoints[index].entity.set_output_reservation(
            0,
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
        let talkers: Vec<usize> = (0..self.endpoints.len())
            .filter(|index| self.endpoints[*index].config.kind == Kind::Talker)
            .collect();
        match event {
            MaapEvent::Acquired(first) => {
                log(format!("MAAP: streams go to {first} on"));
                for (place, index) in talkers.into_iter().enumerate() {
                    let Some(address) = self
                        .maap
                        .as_ref()
                        .and_then(|maap| maap.address(place as u16))
                    else {
                        continue;
                    };
                    self.endpoints[index]
                        .entity
                        .set_output_destination(0, address);
                    // Declared again only once a bridge has let the old
                    // declaration go.
                    let at = self.endpoints[index]
                        .withdrawn_at
                        .map_or(since, |withdrawn| withdrawn + REDECLARE)
                        .max(since);
                    self.endpoints[index].redeclare_at = Some(at);
                }
                // Our listeners of our talkers find where they went.
                for endpoint in &mut self.endpoints {
                    if endpoint.config.kind == Kind::Listener {
                        endpoint.entity.probe_again(now, 0);
                    }
                }
            }
            MaapEvent::Lost => {
                log("MAAP: another device took the streams' addresses, claiming others");
                for index in talkers {
                    self.endpoints[index].talker = None;
                    // Declared again once MAAP gives it an address.
                    self.endpoints[index].redeclare_at = None;
                    self.withdraw_talker(index);
                }
            }
        }
    }

    /// Withdraws a talker's declaration, which a stream needs to stop.
    fn withdraw_talker(&mut self, index: usize) {
        let Some((stream_id, _)) = self.endpoints[index].entity.output_stream(0) else {
            return;
        };
        if self.lite.is_some()
            && let Some(declaration) = self.lite_declaration(index)
        {
            self.send_cvu(
                index,
                BROADCAST,
                &lite::talker_message(&declaration, avb_mrp::Event::Lv),
            );
            let refused = std::mem::take(&mut self.endpoints[index].refused);
            for (listener, code) in refused {
                if let Some(refusal) = self.refusal(index, code) {
                    self.send_cvu(
                        index,
                        listener,
                        &lite::talker_message(&refusal, avb_mrp::Event::Lv),
                    );
                }
            }
            let endpoint = &mut self.endpoints[index];
            endpoint.copies.clear();
            endpoint.escalated = false;
            endpoint.escalate_at = None;
        }
        let since = self.elapsed();
        self.port.msrp.withdraw(
            since,
            msrp::attribute::TALKER_ADVERTISE,
            &stream_id.0.to_be_bytes(),
        );
        self.endpoints[index].withdrawn_at = Some(since);
        self.endpoints[index].entity.set_output_reservation(
            0,
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

    /// Which of our talkers sends `stream_id`.
    fn own_talker(&self, stream_id: u64) -> Option<usize> {
        self.endpoints.iter().position(|endpoint| {
            endpoint
                .entity
                .output_stream(0)
                .is_some_and(|(id, _)| id.0 == stream_id)
        })
    }

    /// A listener's declaration for one of our talkers' streams changed.
    fn listener_heard(&mut self, stream_id: u64, state: Option<ListenerState>) {
        let ready = state.is_some_and(ListenerState::ready);
        self.ready_remotely.insert(stream_id, ready);
        if let Some(index) = self.own_talker(stream_id) {
            self.update_talker(index);
        }
    }

    /// Starts a talker's stream when a listener is ready for it, on the
    /// network or here, and stops it when none is.
    fn update_talker(&mut self, index: usize) {
        if self.lite.is_some() {
            self.lite_update_talker(index);
            return;
        }
        let Some((stream_id, _)) = self.endpoints[index].entity.output_stream(0) else {
            return;
        };
        let remote = self
            .ready_remotely
            .get(&stream_id.0)
            .copied()
            .unwrap_or(false);
        let local = self
            .endpoints
            .iter()
            .filter(|endpoint| {
                endpoint.listener.is_some()
                    && endpoint
                        .entity
                        .input_stream(0)
                        .is_some_and(|stream| stream.stream_id == stream_id)
            })
            .count();
        let listeners = usize::from(remote) + local;
        let ready = listeners > 0;
        self.endpoints[index].entity.set_output_reservation(
            0,
            OutputReservation {
                ready_listeners: listeners.min(usize::from(u16::MAX)) as u16,
                registering: true,
            },
        );
        let addressed = self.addressed();
        let endpoint = &mut self.endpoints[index];
        if ready && endpoint.talker.is_none() && endpoint.redeclare_at.is_none() && addressed {
            log(format!(
                "{}: a listener is ready, streaming",
                endpoint.config.name
            ));
            self.start_talker(index);
        } else if !ready && endpoint.talker.is_some() {
            log(format!(
                "{}: no listener ready, stopping",
                endpoint.config.name
            ));
            endpoint.talker = None;
        }
    }

    fn start_talker(&mut self, index: usize) {
        let endpoint = &self.endpoints[index];
        let Some((stream_id, destination)) = endpoint.entity.output_stream(0) else {
            return;
        };
        let Some(stream) = endpoint.entity.model().outputs.first() else {
            return;
        };
        let transit = endpoint.entity.max_transit_time(0).unwrap_or(2_000_000);
        let lite = self.lite.is_some();
        let config = TalkerConfig {
            interface: self.interface.clone(),
            mac: self.mac,
            stream_id: stream_id.0,
            destinations: if lite {
                endpoint.copies.clone()
            } else {
                vec![destination]
            },
            vlan_id: if lite { self.media_vlan() } else { VLAN },
            priority: self.priority(index),
            format: stream.current_format,
            max_transit_time: Duration::from_nanos(u64::from(transit)),
            source: endpoint.config.source(),
        };
        // An audio device that isn't there, as on another computer than
        // the one its name came from, leaves the stream running silent.
        let started = Talker::start(config.clone(), self.clock.clone()).or_else(|error| {
            if config.source == Source::Silence {
                return Err(error);
            }
            log(format!(
                "{}: {error}, sending silence",
                endpoint.config.name
            ));
            let silent = TalkerConfig {
                source: Source::Silence,
                ..config
            };
            Talker::start(silent, self.clock.clone())
        });
        match started {
            Ok(talker) => self.endpoints[index].talker = Some(talker),
            Err(error) => log(format!(
                "{}: could not start the stream: {error}",
                endpoint.config.name
            )),
        }
    }

    /// A talker declaration came or went: inputs settled on its stream
    /// declare themselves and listen, or stop.
    fn talker_heard(&mut self, stream_id: u64) {
        for index in 0..self.endpoints.len() {
            let settled = self.endpoints[index]
                .entity
                .input_stream(0)
                .is_some_and(|stream| stream.stream_id.0 == stream_id);
            if settled {
                self.update_listener(index);
            }
        }
    }

    /// Declares a listener endpoint's input as its talker's declaration
    /// allows, and listens when it is advertised. A talker of ours needs
    /// no reservation, as the bridge never sends our own declarations back
    /// to us; its frames reach the listener as they leave.
    fn update_listener(&mut self, index: usize) {
        if self.lite.is_some() {
            self.lite_update_listener(index);
            return;
        }
        let since = self.elapsed();
        let Some(stream) = self.endpoints[index].entity.input_stream(0) else {
            return;
        };
        let own = self.own_talker(stream.stream_id.0);
        let declaration = match own {
            Some(talker) if self.addressed() => self.declaration(talker),
            Some(_) => None,
            None => self.talkers.get(&stream.stream_id.0).copied(),
        };
        let key = stream.stream_id.0.to_be_bytes().to_vec();
        let (state, reservation) = match declaration {
            Some(declaration) => (
                Some(if declaration.failure.is_some() {
                    ListenerState::AskingFailed
                } else {
                    ListenerState::Ready
                }),
                InputReservation {
                    talker_registered: declaration.failure.is_none(),
                    failure: declaration
                        .failure
                        .map(|failure| (failure.code, failure.bridge_id)),
                    accumulated_latency: declaration.accumulated_latency,
                    registering: true,
                },
            ),
            None => (None, InputReservation::default()),
        };
        match state {
            Some(_) if own.is_some() => {
                if let Some(key) = self.endpoints[index].declared.take() {
                    self.port
                        .msrp
                        .withdraw(since, msrp::attribute::LISTENER, &key);
                }
            }
            Some(state) => {
                self.port.msrp.declare(
                    since,
                    msrp::attribute::LISTENER,
                    key.clone(),
                    Some(state.number()),
                );
                self.endpoints[index].declared = Some(key);
            }
            None => {
                self.port
                    .msrp
                    .withdraw(since, msrp::attribute::LISTENER, &key);
                self.endpoints[index].declared = None;
            }
        }
        self.endpoints[index]
            .entity
            .set_input_reservation(0, reservation);
        let advertised = state == Some(ListenerState::Ready);
        // Where the stream goes, as its talker declares it now: a probe
        // can have answered before the talker had its address, or before
        // it moved.
        let stream = atdecc::entity::ProbedStream {
            destination: declaration.map_or(stream.destination, |declared| declared.destination),
            ..stream
        };
        let endpoint = &mut self.endpoints[index];
        if endpoint.listener.is_some() && endpoint.listening_to != Some(stream) {
            endpoint.listener = None;
        }
        if advertised && endpoint.listener.is_none() {
            self.start_listener(index, stream, vec![stream.destination]);
        } else if !advertised && endpoint.listener.is_some() {
            log(format!(
                "{}: the talker is gone, not listening",
                endpoint.config.name
            ));
            endpoint.listener = None;
        }
        self.update_own_talkers();
    }

    /// Starts a listener's thread on `stream`, taking frames sent to any
    /// of `destinations`.
    fn start_listener(
        &mut self,
        index: usize,
        stream: atdecc::entity::ProbedStream,
        destinations: Vec<MacAddress>,
    ) {
        let endpoint = &self.endpoints[index];
        let Some(model) = endpoint.entity.model().inputs.first() else {
            return;
        };
        let config = ListenerConfig {
            interface: self.interface.clone(),
            stream_id: stream.stream_id.0,
            destinations,
            format: model.current_format,
            sink: endpoint.config.sink(),
        };
        let started = Listener::start(config.clone(), self.clock.clone()).or_else(|error| {
            if config.sink == Sink::Discard {
                return Err(error);
            }
            log(format!(
                "{}: {error}, playing nowhere",
                endpoint.config.name
            ));
            let discarding = ListenerConfig {
                sink: Sink::Discard,
                ..config
            };
            Listener::start(discarding, self.clock.clone())
        });
        let endpoint = &mut self.endpoints[index];
        match started {
            Ok(listener) => {
                log(format!(
                    "{}: listening to {}",
                    endpoint.config.name, stream.stream_id
                ));
                endpoint.listener = Some(listener);
                endpoint.listening_to = Some(stream);
            }
            Err(error) => log(format!(
                "{}: could not listen: {error}",
                endpoint.config.name
            )),
        }
    }

    /// Our talkers, again, after one of our listeners started or stopped.
    fn update_own_talkers(&mut self) {
        for index in 0..self.endpoints.len() {
            if self.endpoints[index].config.kind == Kind::Talker {
                self.update_talker(index);
            }
        }
    }

    fn stop_listener(&mut self, index: usize) {
        let since = self.elapsed();
        let endpoint = &mut self.endpoints[index];
        if endpoint.listener.take().is_some() {
            log(format!("{}: not listening", endpoint.config.name));
        }
        endpoint
            .entity
            .set_input_reservation(0, InputReservation::default());
        if let Some(key) = endpoint.declared.take() {
            self.port
                .msrp
                .withdraw(since, msrp::attribute::LISTENER, &key);
        }
        if let Some((stream_id, talker, state)) = endpoint.cvu_to.take() {
            self.send_cvu(
                index,
                talker,
                &lite::listener_message(stream_id, state, avb_mrp::Event::Lv),
            );
        }
        self.update_own_talkers();
    }

    fn entity_event(&mut self, index: usize, event: EntityEvent) {
        let name = self.endpoints[index].config.name.clone();
        match event {
            EntityEvent::Identify(on) => log(format!(
                "{name}: identify {}",
                if on { "on" } else { "off" }
            )),
            EntityEvent::InputBound { binding, .. } => {
                log(format!(
                    "{name}: bound to {} output {}",
                    binding.talker, binding.talker_unique_id
                ));
                self.keep_settings(index);
            }
            EntityEvent::InputSettled { stream, .. } => {
                log(format!(
                    "{name}: probed stream {} to {}",
                    stream.stream_id, stream.destination
                ));
                // A stream that moved is taken from where it went.
                let endpoint = &mut self.endpoints[index];
                if endpoint.listener.is_some() && endpoint.listening_to != Some(stream) {
                    endpoint.listener = None;
                }
                self.update_listener(index);
            }
            EntityEvent::InputUnsettled { .. } => self.stop_listener(index),
            EntityEvent::InputUnbound { .. } => {
                log(format!("{name}: unbound"));
                self.stop_listener(index);
                self.keep_settings(index);
            }
            EntityEvent::StreamFormatChanged { format, .. } => {
                log(format!("{name}: format {format}"));
                self.keep_settings(index);
                if self.endpoints[index].config.kind == Kind::Talker {
                    self.endpoints[index].talker = None;
                    self.withdraw_talker(index);
                    self.endpoints[index].redeclare_at = Some(self.elapsed() + REDECLARE);
                }
                if self.endpoints[index].listener.take().is_some() {
                    self.update_listener(index);
                }
            }
            EntityEvent::MaxTransitTimeChanged { nanoseconds, .. } => {
                log(format!("{name}: max transit time {nanoseconds} ns"));
                if self.endpoints[index].talker.take().is_some() {
                    self.start_talker(index);
                }
            }
            EntityEvent::SamplingRateChanged(rate) => log(format!("{name}: sampling rate {rate}")),
            EntityEvent::ClockSourceChanged(source) => {
                log(format!("{name}: clock source {source}"))
            }
            EntityEvent::LiteConfigChanged { escalation_allowed } => {
                log(format!(
                    "AVB Lite: escalating to multicast {}",
                    if escalation_allowed {
                        "allowed"
                    } else {
                        "not allowed"
                    }
                ));
                self.config.multicast_escalation = escalation_allowed;
                self.keep_config();
                self.update_lite_status();
                if self.lite.is_some() {
                    for index in 0..self.endpoints.len() {
                        if self.endpoints[index].config.kind == Kind::Talker {
                            self.update_talker(index);
                        }
                    }
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

    /// Hands each running stream's counters to its entity, in Milan's
    /// places for them.
    fn update_counters(&mut self) {
        let low = |value: u64| value as u32;
        for endpoint in &mut self.endpoints {
            if let Some(talker) = &endpoint.talker {
                let stats = talker.stats();
                let mut counters = [0; 32];
                counters[0] = 1; // stream_start
                counters[3] = low(stats.media_resets);
                counters[5] = low(stats.frames_sent);
                counters[7] = low(stats.frames_sent);
                endpoint.entity.set_stream_counters(
                    atdecc::DescriptorType::STREAM_OUTPUT,
                    0,
                    counters,
                );
            }
            if let Some(listener) = &endpoint.listener {
                let stats = listener.stats();
                let mut counters = [0; 32];
                counters[0] = low(stats.media_locked);
                counters[1] = low(stats.media_unlocked);
                counters[2] = low(stats.interrupted);
                counters[3] = low(stats.sequence_mismatches);
                counters[6] = low(stats.frames_received);
                counters[8] = low(stats.unsupported_formats);
                counters[9] = low(stats.late);
                counters[10] = low(stats.early);
                counters[11] = low(stats.frames_received);
                endpoint.entity.set_stream_counters(
                    atdecc::DescriptorType::STREAM_INPUT,
                    0,
                    counters,
                );
            }
        }
    }

    fn report(&self) {
        for endpoint in &self.endpoints {
            if let Some(talker) = &endpoint.talker {
                let stats = talker.stats();
                log(format!(
                    "{}: sent {} frames, {} media resets, {} send errors, device {:+} ppm, {} short{}",
                    endpoint.config.name,
                    stats.frames_sent,
                    stats.media_resets,
                    stats.send_errors,
                    stats.drift_ppm,
                    stats.underruns,
                    if stats.realtime { ", real time" } else { "" }
                ));
            }
            if let Some(listener) = &endpoint.listener {
                let stats = listener.stats();
                log(format!(
                    "{}: received {} frames, {} out of sequence, {} late, {} early, peak {:.3}, device {:+} ppm, {} short{}{}",
                    endpoint.config.name,
                    stats.frames_received,
                    stats.sequence_mismatches,
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
        if self.lite.is_some() {
            // AVB Lite's declarations go too, as no bridge withdraws them.
            for index in 0..self.endpoints.len() {
                if self.endpoints[index].config.kind == Kind::Talker {
                    self.withdraw_talker(index);
                } else if let Some((stream_id, talker, state)) = self.endpoints[index].cvu_to.take()
                {
                    self.send_cvu(
                        index,
                        talker,
                        &lite::listener_message(stream_id, state, avb_mrp::Event::Lv),
                    );
                }
            }
        }
        for endpoint in &mut self.endpoints {
            endpoint.talker = None;
            endpoint.listener = None;
            endpoint.entity.depart();
        }
        let streams: Vec<StreamId> = self
            .endpoints
            .iter()
            .filter_map(|endpoint| endpoint.entity.output_stream(0))
            .map(|(stream_id, _)| stream_id)
            .collect();
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
        for index in 0..self.endpoints.len() {
            let endpoint = &mut self.endpoints[index];
            endpoint.talker = None;
            endpoint.listener = None;
            if let Some(key) = endpoint.declared.take() {
                self.port
                    .msrp
                    .withdraw(since, msrp::attribute::LISTENER, &key);
            }
            if let Some((stream_id, _)) = self.endpoints[index].entity.output_stream(0) {
                self.port.msrp.withdraw(
                    since,
                    msrp::attribute::TALKER_ADVERTISE,
                    &stream_id.0.to_be_bytes(),
                );
            }
        }
        self.port.leave(since);
        self.talkers.clear();
        self.ready_remotely.clear();
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
        for index in 0..self.endpoints.len() {
            match self.endpoints[index].config.kind {
                Kind::Talker if self.addressed() => self.declare_talker(index),
                Kind::Talker => {}
                Kind::Listener => self.update_listener(index),
            }
        }
        self.update_lite_status();
        self.steer_profile();
    }

    /// Leaves AVB Lite as the link comes up again (profile 2.2): CVU SRP's
    /// declarations go, the endpoints declare with MSRP and MVRP as at the
    /// start, and the fallback is armed again.
    fn leave_lite(&mut self) {
        log("AVB Lite: the link came up again, trying AVB");
        let since = self.elapsed();
        for index in 0..self.endpoints.len() {
            if self.endpoints[index].config.kind == Kind::Talker {
                self.withdraw_talker(index);
            } else if let Some((stream_id, talker, state)) = self.endpoints[index].cvu_to.take() {
                self.send_cvu(
                    index,
                    talker,
                    &lite::listener_message(stream_id, state, avb_mrp::Event::Lv),
                );
            }
            let endpoint = &mut self.endpoints[index];
            endpoint.talker = None;
            endpoint.listener = None;
            endpoint.copies.clear();
        }
        self.lite = None;
        self.talkers.clear();
        self.ready_remotely.clear();
        self.fallback = Fallback::new(self.own_clock.0, false, since);
        self.port.join(since);
        for index in 0..self.endpoints.len() {
            match self.endpoints[index].config.kind {
                Kind::Talker if self.addressed() => self.declare_talker(index),
                Kind::Talker => {}
                Kind::Listener => self.update_listener(index),
            }
        }
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
        if self.config.avb_lite == LiteChoice::Off {
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
        for (_, index, destination, message) in due {
            self.send_cvu(index, destination, &message);
        }
        for stream_id in gone_talkers {
            log(format!("AVB Lite: talker of {stream_id:#018x} is gone"));
            self.lite_talker_changed(stream_id);
        }
        for stream_id in stale_refusals {
            self.lite_talker_changed(stream_id);
        }
        for index in 0..self.endpoints.len() {
            if self.endpoints[index]
                .escalate_at
                .is_some_and(|at| at <= since)
            {
                self.update_talker(index);
            }
        }
        for stream_id in gone_listeners {
            if let Some(index) = self.own_talker(stream_id) {
                self.update_talker(index);
            }
        }
        if refresh {
            for index in 0..self.endpoints.len() {
                let endpoint = &self.endpoints[index];
                if endpoint.config.kind == Kind::Talker {
                    if self.addressed()
                        && let Some(declaration) = self.lite_declaration(index)
                    {
                        self.declare_cvu(
                            index,
                            BROADCAST,
                            lite::talker_message(&declaration, avb_mrp::Event::JoinIn),
                        );
                    }
                    let refused: Vec<(MacAddress, u8)> = self.endpoints[index]
                        .refused
                        .iter()
                        .map(|(listener, code)| (*listener, *code))
                        .collect();
                    for (listener, code) in refused {
                        if let Some(refusal) = self.refusal(index, code) {
                            self.declare_cvu(
                                index,
                                listener,
                                lite::talker_message(&refusal, avb_mrp::Event::JoinIn),
                            );
                        }
                    }
                } else if let Some((stream_id, talker, state)) = endpoint.cvu_to {
                    self.declare_cvu(
                        index,
                        talker,
                        lite::listener_message(stream_id, state, avb_mrp::Event::JoinIn),
                    );
                }
            }
        }
    }

    /// Sends a CVU SRP declaration from endpoint `index` to `destination`,
    /// VLAN-tagged in the media VLAN (profile 6, item 5); our own
    /// endpoints take it here, as the computer never hears what it sends.
    fn send_cvu(
        &mut self,
        index: usize,
        destination: MacAddress,
        message: &avb_mrp::mrpdu::Message,
    ) {
        let Some(lite) = &mut self.lite else {
            return;
        };
        // What goes now supersedes a repeat still to go.
        lite.repeats.retain(|(_, from, to, pending)| {
            !(*from == index && *to == destination && same_declaration(pending, message))
        });
        lite.sequence = lite.sequence.wrapping_add(1);
        let sender = self.endpoints[index].entity.entity_id();
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
        index: usize,
        destination: MacAddress,
        message: avb_mrp::mrpdu::Message,
    ) {
        self.send_cvu(index, destination, &message);
        let at = self.elapsed() + JOIN_TIME;
        if let Some(lite) = &mut self.lite {
            lite.repeats.push((at, index, destination, message));
        }
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
                    if changed && let Some(index) = self.own_talker(stream_id) {
                        self.update_talker(index);
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
        for index in 0..self.endpoints.len() {
            let settled = self.endpoints[index]
                .entity
                .input_stream(0)
                .is_some_and(|stream| stream.stream_id.0 == stream_id);
            if settled {
                self.update_listener(index);
            }
        }
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

    /// The bandwidth our talkers' copies take, but for talker `except`'s,
    /// in bits per second.
    fn committed(&self, except: Option<usize>) -> u64 {
        (0..self.endpoints.len())
            .filter(|index| Some(*index) != except)
            .filter_map(|index| {
                let copies = self.endpoints[index].copies.len() as u64;
                self.declaration(index)
                    .map(|declaration| copies * declaration.bandwidth(8000))
            })
            .sum()
    }

    /// A talker's declaration in AVB Lite, its destination where the
    /// stream's frames go now: `00:00:00:00:00:00` while unicast, the
    /// multicast address once escalated (profile 6).
    fn lite_declaration(&self, index: usize) -> Option<TalkerDeclaration> {
        let declaration = self.declaration(index)?;
        Some(TalkerDeclaration {
            destination: if self.endpoints[index].escalated {
                declaration.destination
            } else {
                MacAddress([0; 6])
            },
            ..declaration
        })
    }

    /// The Talker Failed a talker sends one listener it refuses (profile
    /// 6, item 10), naming this computer as the system that failed.
    fn refusal(&self, index: usize, code: u8) -> Option<TalkerDeclaration> {
        let declaration = self.lite_declaration(index)?;
        let [a, b, c, d, e, f] = self.mac.0;
        Some(TalkerDeclaration {
            failure: Some(TalkerFailure {
                bridge_id: u64::from_be_bytes([0, 0, a, b, c, d, e, f]),
                code,
            }),
            ..declaration
        })
    }

    /// Sends a talker's stream to the listeners that want it, within its
    /// unicast fan-out and 75% of the link (profile 6): a unicast copy
    /// each, or, where escalation is allowed and they are more, its
    /// multicast address. A listener it cannot serve it refuses alone.
    fn lite_update_talker(&mut self, index: usize) {
        let Some((stream_id, multicast)) = self.endpoints[index].entity.output_stream(0) else {
            return;
        };
        let Some(declaration) = self.declaration(index) else {
            return;
        };
        let since = self.elapsed();
        let declared: Vec<(MacAddress, ListenerState)> = self
            .lite
            .as_ref()
            .and_then(|lite| lite.listeners.get(&stream_id.0))
            .map(|listeners| {
                listeners
                    .iter()
                    .map(|(mac, (state, _))| (*mac, *state))
                    .collect()
            })
            .unwrap_or_default();
        // Those it serves keep their place, then those it refuses, who ask
        // and fail because of it, then newcomers ready for the stream.
        let endpoint = &self.endpoints[index];
        let rank = |mac: &MacAddress| {
            if endpoint.copies.contains(mac) {
                0
            } else if endpoint.refused.contains_key(mac) {
                1
            } else {
                2
            }
        };
        let mut wanting: Vec<(MacAddress, ListenerState)> = declared
            .into_iter()
            .filter(|(mac, state)| state.ready() || endpoint.refused.contains_key(mac))
            .collect();
        wanting.sort_by_key(|(mac, _)| (rank(mac), mac.0));
        let per_copy = declaration.bandwidth(8000).max(1);
        let room = if self.link_speed == 0 {
            usize::MAX
        } else {
            let budget = u64::from(self.link_speed) * 750_000;
            (budget.saturating_sub(self.committed(Some(index))) / per_copy) as usize
        };
        let fanout = self.fanout();
        let go_multicast = self.config.multicast_escalation && wanting.len() > fanout && room >= 1;
        let limit = if go_multicast {
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
        // To escalate, the declaration goes first and the frames JoinTime
        // later; to go back, the frames go first and the declaration after.
        let endpoint = &mut self.endpoints[index];
        let mut announce = false;
        let copies = if go_multicast {
            if !endpoint.escalated {
                endpoint.escalated = true;
                endpoint.escalate_at = Some(since + JOIN_TIME);
                announce = true;
            }
            match endpoint.escalate_at {
                Some(at) if since < at => endpoint.copies.clone(),
                _ => {
                    endpoint.escalate_at = None;
                    if ready.is_empty() {
                        Vec::new()
                    } else {
                        vec![multicast]
                    }
                }
            }
        } else {
            if endpoint.escalated {
                endpoint.escalated = false;
                endpoint.escalate_at = None;
                announce = true;
            }
            ready.clone()
        };
        let name = self.endpoints[index].config.name.clone();
        self.endpoints[index].entity.set_output_reservation(
            0,
            OutputReservation {
                ready_listeners: ready.len().min(usize::from(u16::MAX)) as u16,
                registering: true,
            },
        );
        if copies != self.endpoints[index].copies && !copies.is_empty() {
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
        self.endpoints[index].copies.clone_from(&copies);
        let addressed = self.addressed();
        let endpoint = &mut self.endpoints[index];
        if copies.is_empty() {
            if endpoint.talker.take().is_some() {
                log(format!("{name}: no listener ready, stopping"));
            }
        } else if let Some(talker) = &endpoint.talker {
            talker.set_destinations(copies);
        } else if endpoint.redeclare_at.is_none() && addressed {
            log(format!("{name}: a listener is ready, streaming"));
            self.start_talker(index);
        }
        if announce && let Some(declaration) = self.lite_declaration(index) {
            self.declare_cvu(
                index,
                BROADCAST,
                lite::talker_message(&declaration, avb_mrp::Event::New),
            );
        }
        let before = std::mem::replace(&mut self.endpoints[index].refused, refused.clone());
        for (listener, code) in &refused {
            if before.get(listener) != Some(code)
                && let Some(refusal) = self.refusal(index, *code)
            {
                log(format!(
                    "{name}: AVB Lite refuses {listener}, failure code {code}"
                ));
                self.declare_cvu(
                    index,
                    *listener,
                    lite::talker_message(&refusal, avb_mrp::Event::New),
                );
            }
        }
        for (listener, code) in before {
            if !refused.contains_key(&listener)
                && let Some(refusal) = self.refusal(index, code)
            {
                self.send_cvu(
                    index,
                    listener,
                    &lite::talker_message(&refusal, avb_mrp::Event::Lv),
                );
            }
        }
    }

    /// Whether this computer's link takes a stream of `declaration` for
    /// listener `index` on top of the streams its other listeners are
    /// ready for from other computers, within 75% (profile 6, item 6).
    fn ingress_fits(&self, index: usize, declaration: &TalkerDeclaration) -> bool {
        if self.link_speed == 0 {
            return true;
        }
        let Some(lite) = &self.lite else {
            return true;
        };
        let taken: u64 = (0..self.endpoints.len())
            .filter(|other| *other != index)
            .filter_map(|other| self.endpoints[other].cvu_to)
            .filter(|(_, talker, state)| *talker != self.mac && state.ready())
            .filter_map(|(stream_id, _, _)| lite.talkers.get(&stream_id))
            .map(|(declaration, _, _)| declaration.bandwidth(8000))
            .sum();
        taken + declaration.bandwidth(8000) <= u64::from(self.link_speed) * 750_000
    }

    /// Declares a listener to the talker whose declaration names its
    /// stream, unicast, and takes the stream sent to this computer or to
    /// the stream's multicast address.
    fn lite_update_listener(&mut self, index: usize) {
        let Some(stream) = self.endpoints[index].entity.input_stream(0) else {
            return;
        };
        // A Talker Failed sent to this computer alone outranks the
        // talker's broadcast declaration (profile 6, item 10).
        let heard = self.lite.as_ref().and_then(|lite| {
            lite.refusals
                .get(&stream.stream_id.0)
                .or_else(|| lite.talkers.get(&stream.stream_id.0))
                .map(|(declaration, from, _)| (*declaration, *from))
        });
        // A stream from another computer the link has no room for is
        // asked for and failed (profile 6, item 6).
        let full = heard.is_some_and(|(declaration, from)| {
            declaration.failure.is_none()
                && from != self.mac
                && !self.ingress_fits(index, &declaration)
        });
        let wanted = heard.map(|(declaration, from)| {
            let state = if declaration.failure.is_some() || full {
                ListenerState::AskingFailed
            } else {
                ListenerState::Ready
            };
            (stream.stream_id.0, from, state)
        });
        if full && wanted != self.endpoints[index].cvu_to {
            log(format!(
                "{}: AVB Lite, the link has no room for {:#018x}",
                self.endpoints[index].config.name, stream.stream_id.0
            ));
        }
        let before = self.endpoints[index].cvu_to;
        if let Some((stream_id, talker, state)) = before
            && wanted.map(|(id, to, _)| (id, to)) != Some((stream_id, talker))
        {
            self.send_cvu(
                index,
                talker,
                &lite::listener_message(stream_id, state, avb_mrp::Event::Lv),
            );
        }
        self.endpoints[index].cvu_to = wanted;
        if wanted != before
            && let Some((stream_id, talker, state)) = wanted
        {
            self.declare_cvu(
                index,
                talker,
                lite::listener_message(stream_id, state, avb_mrp::Event::New),
            );
        }
        let [a, b, c, d, e, f] = self.mac.0;
        let own = u64::from_be_bytes([0, 0, a, b, c, d, e, f]);
        let reservation = match heard {
            Some((declaration, _)) => InputReservation {
                talker_registered: declaration.failure.is_none() && !full,
                failure: declaration
                    .failure
                    .map(|failure| (failure.code, failure.bridge_id))
                    .or(full.then_some((FAILURE_BANDWIDTH, own))),
                accumulated_latency: declaration.accumulated_latency,
                registering: true,
            },
            None => InputReservation::default(),
        };
        self.endpoints[index]
            .entity
            .set_input_reservation(0, reservation);
        let advertised = heard.filter(|(declaration, _)| declaration.failure.is_none() && !full);
        // The stream comes to this computer's own address, and to the
        // multicast address the talker declares once escalated; until
        // then, to the one probing found, so escalating moves nothing.
        let declared = advertised
            .map(|(declaration, _)| declaration.destination)
            .filter(|destination| destination.0 != [0; 6]);
        let stream = atdecc::entity::ProbedStream {
            destination: declared.unwrap_or(stream.destination),
            ..stream
        };
        let endpoint = &mut self.endpoints[index];
        if endpoint.listener.is_some() && endpoint.listening_to != Some(stream) {
            endpoint.listener = None;
        }
        if advertised.is_some() && endpoint.listener.is_none() {
            let mut destinations = vec![self.mac];
            if stream.destination != self.mac {
                destinations.push(stream.destination);
            }
            self.start_listener(index, stream, destinations);
        } else if advertised.is_none() && endpoint.listener.take().is_some() {
            let why = match heard {
                Some((declaration, _)) => match declaration.failure {
                    Some(failure) => format!("the talker failed, code {}", failure.code),
                    None => "the link has no room".to_owned(),
                },
                None => "the talker is gone".to_owned(),
            };
            log(format!("{}: {why}, not listening", endpoint.config.name));
        }
    }

    /// What each entity's GET_LITE_STATUS answers.
    fn update_lite_status(&mut self) {
        let ptp = self.ptp_status.as_ref();
        let offset = ptp.and_then(|status| status.offset);
        let gptp_runs = ptp.is_some_and(|status| !status.end_to_end);
        let mut flags = LiteFlags::CAPABLE | LiteFlags::EGRESS_VALID;
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
            let talker = self.endpoints[index].config.kind == Kind::Talker;
            let egress = if talker {
                let copies = self.endpoints[index].copies.len() as u64;
                self.declaration(index)
                    .map_or(0, |declaration| copies * declaration.bandwidth(8000) / 1000)
            } else {
                0
            };
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

    #[test]
    fn the_formats_are_the_bench_ones() {
        let milan = |rate, channels, bits| aaf(rate, channels, bits, CLASS_A_FRAMES);
        assert_eq!(milan(48_000, 8, 32), StreamFormat(0x0205_0220_0200_6000));
        assert_eq!(milan(48_000, 8, 24), StreamFormat(0x0205_0218_0200_6000));
        assert_eq!(milan(96_000, 8, 32), StreamFormat(0x0207_0220_0200_c000));
        assert_eq!(am824(48_000, 8), StreamFormat(0x00a0_0208_4000_0800));
        assert_eq!(am824(96_000, 8), StreamFormat(0x00a0_0408_4000_0800));
        // A frame each class B interval: 12 samples at 48 kHz.
        let wireless = aaf(48_000, 8, 32, CLASS_B_FRAMES);
        assert_eq!(wireless, StreamFormat(0x0205_0220_0200_c000));
        let media = triib_stream::media::Media::of(wireless).unwrap();
        assert_eq!(media.interval_nanos(), CLASS_B_INTERVAL);
        // A format chosen for eight channels carries over to two.
        assert!(same_packing(am824(48_000, 2), am824(48_000, 8)));
        assert!(same_packing(milan(48_000, 2, 24), milan(48_000, 8, 24)));
        assert!(!same_packing(milan(48_000, 2, 32), milan(48_000, 2, 24)));
        assert!(!same_packing(milan(48_000, 8, 32), am824(48_000, 8)));
        assert!(!same_packing(milan(48_000, 8, 32), milan(96_000, 8, 32)));
        // Nor does one frame rate pass for the other.
        assert!(!same_packing(milan(48_000, 8, 32), wireless));
    }

    /// The rates `formats` offer at `frames` a second, in order.
    fn rates_at(formats: &[StreamFormat], frames: u32) -> Vec<u32> {
        let mut rates: Vec<u32> = formats
            .iter()
            .filter(|format| {
                let Some(rate) = format.sample_rate() else {
                    return false;
                };
                match format.kind() {
                    FormatKind::Aaf(aaf) => u32::from(aaf.samples_per_frame) * frames == rate,
                    _ => frames == CLASS_A_FRAMES,
                }
            })
            .filter_map(|format| format.sample_rate())
            .collect();
        rates.dedup();
        rates
    }

    /// Eight channels go at every rate, 16 to 96 kHz, and 60 only at
    /// 48 kHz, each rate with all three packings; a frame each class B
    /// interval holds twice the samples, so fewer channels fit.
    #[test]
    fn more_channels_take_fewer_rates() {
        let rates = |channels| rates_at(&offered_formats(channels, false), CLASS_A_FRAMES);
        assert_eq!(rates(8), [48_000, 96_000, 192_000]);
        assert_eq!(rates(16), [48_000, 96_000]);
        assert_eq!(rates(60), [48_000]);
        let wireless = |channels| rates_at(&offered_formats(channels, false), CLASS_B_FRAMES);
        assert_eq!(wireless(8), [48_000, 96_000]);
        assert_eq!(wireless(60), Vec::<u32>::new());
    }

    /// A class B talker offers its own formats first, so a frame each
    /// class B interval is its default; others offer Milan's first.
    #[test]
    fn a_class_b_talker_defaults_to_half_the_frames() {
        assert_eq!(
            offered_formats(8, true)[0],
            aaf(48_000, 8, 32, CLASS_B_FRAMES)
        );
        assert_eq!(
            offered_formats(8, false)[0],
            aaf(48_000, 8, 32, CLASS_A_FRAMES)
        );
        assert_eq!(
            offered_formats(8, true).len(),
            offered_formats(8, false).len()
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
            random: 0xf0a7_31f4_0f14 | 1,
        };
        let since = Duration::from_secs(100);
        let intervals: Vec<Duration> = (0..50).map(|_| lite.refresh_after(since) - since).collect();
        for interval in &intervals {
            assert!(*interval >= LEAVE_ALL_TIME && *interval < LEAVE_ALL_TIME * 3 / 2);
        }
        assert!(intervals.windows(2).any(|pair| pair[0] != pair[1]));
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
}
