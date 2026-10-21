//! AVB Lite (the AVB Lite profile): telling from the Pdelay exchange
//! whether an AVB bridge is between this computer and its peers, falling
//! back to AVB Lite when none is, and the CVU SRP messages that carry
//! MSRP declarations there, as AECP vendor unique commands.

use std::collections::VecDeque;
use std::time::Duration;

use atdecc::aecp::{AecpHeader, AecpMessageType, VendorUniquePdu};
use atdecc::lite::{CVU_PROTOCOL_ID, CvuMessage, FallbackReason};
use atdecc::{EntityId, MacAddress};
use avb_mrp::msrp::{self, ListenerState, TalkerDeclaration};
use avb_mrp::{Event, mrpdu};

/// PTP's ethertype, and where gPTP's peer delay messages go.
pub const PTP_ETHERTYPE: u16 = 0x88f7;
pub const PDELAY_DESTINATION: MacAddress = MacAddress([0x01, 0x80, 0xc2, 0x00, 0x00, 0x0e]);

/// PTP message types.
const PDELAY_REQ: u8 = 0x2;
const PDELAY_RESP: u8 = 0x3;
const PDELAY_RESP_FOLLOW_UP: u8 = 0xa;
/// Octets of a Pdelay message before any TLV.
const PDELAY_LEN: usize = 54;
/// The Endpoint Declaration TLV (profile 2.1): an organization extension
/// under the AVB Lite MA-S, saying the sender is an endpoint, of the
/// do-not-propagate type IEEE 1588-2019 asks of new extensions.
const ENDPOINT_TLV: [u8; 12] = [
    0x80, 0x00, 0x00, 0x08, 0x8c, 0x1f, 0x64, 0x36, 0xc0, 0x01, 0x01, 0x00,
];
/// The organization extension types the TLV comes in: the profile's,
/// and the one its earlier revisions used.
const ORGANIZATION_EXTENSION_DO_NOT_PROPAGATE: u16 = 0x8000;
const ORGANIZATION_EXTENSION: u16 = 0x0003;

/// Where PTP's other messages go, Announce among them.
pub const PTP_DESTINATION: MacAddress = MacAddress([0x01, 0x1b, 0x19, 0x00, 0x00, 0x00]);
/// Announce's message type, and its octets before any TLV.
const ANNOUNCE: u8 = 0xb;
const ANNOUNCE_LEN: usize = 64;
/// The Grandmaster Link TLV's identifiers (profile 5): the AVB Lite MA-S
/// and sub-id 0x004.
const GRANDMASTER_LINK: [u8; 6] = [0x8c, 0x1f, 0x64, 0x36, 0xc0, 0x04];
/// The octets of a Sync or Delay_Req frame on the wire, a minimum-size
/// frame, which the link-speed asymmetry counts.
const PTP_FRAME_OCTETS: f64 = 64.0;

/// How long after starting this waits before its first Pdelay_Req, so it
/// knows first whether ptp4l runs, which an unasked-for answer would make
/// fault.
const START_GRACE: Duration = Duration::from_secs(3);
/// How often a Pdelay_Req goes out, and after fallback the beacon.
const PROBE_INTERVAL: Duration = Duration::from_secs(1);
const BEACON_INTERVAL: Duration = Duration::from_secs(3);
/// Unanswered requests that make the fallback (profile 2.2, condition 2).
const MOST_UNANSWERED: u32 = 9;
/// How long ptp4l may say it has no gPTP peer before that counts as the
/// same.
const PEERLESS: Duration = Duration::from_secs(10);
/// How long after ptp4l last said its port is FAULTY its requests going
/// unanswered still says nothing of the peer: twice its default
/// fault_reset_interval, 2^4 s, for it to reset and ask again.
const FAULT_GRACE: Duration = Duration::from_secs(32);
/// The first wait before trying gPTP again from AVB Lite, the longest
/// after tries that fell back again, and how long gPTP must hold for the
/// next fallback to wait the first again.
const RETRY_FIRST: Duration = Duration::from_secs(60);
const RETRY_LONGEST: Duration = Duration::from_secs(30 * 60);
const RETRY_HELD: Duration = Duration::from_secs(5 * 60);

/// A Pdelay_Req from `identity` (clock identity, port 1) with the
/// Endpoint Declaration TLV, as esp_ptp sends its own and its beacons.
pub fn pdelay_request(identity: u64, sequence: u16) -> Vec<u8> {
    let mut message = vec![0; PDELAY_LEN];
    // gPTP (majorSdoId 1), Pdelay_Req; PTP version 2.
    message[0] = 0x10 | PDELAY_REQ;
    message[1] = 0x02;
    message[2..4].copy_from_slice(&((PDELAY_LEN + ENDPOINT_TLV.len()) as u16).to_be_bytes());
    // The PTP timescale flag.
    message[7] = 0x08;
    message[20..28].copy_from_slice(&identity.to_be_bytes());
    message[28..30].copy_from_slice(&1u16.to_be_bytes());
    message[30..32].copy_from_slice(&sequence.to_be_bytes());
    message[32] = 5;
    message.extend_from_slice(&ENDPOINT_TLV);
    message
}

/// The octets a driver's receive time stamp takes at the end of a frame,
/// as atlantic's does.
const STAMP_LEN: usize = 12;

/// Whether a PTP message heard on the interface, after the ethertype,
/// runs a time stamp's length past its messageLength: the driver left its
/// receive time stamp on the frame, which Ethernet's padding, at most 2
/// octets for the shortest PTP message, cannot explain, and which ptp4l
/// takes for a broken TLV.
pub fn stamped(message: &[u8]) -> bool {
    message.len() >= 4
        && message.len() >= usize::from(u16::from_be_bytes([message[2], message[3]])) + STAMP_LEN
}

/// The grandmaster an Announce names and the link speed in Mb/s its
/// Grandmaster Link TLV gives (profile 5), when it carries one.
pub fn grandmaster_link(message: &[u8]) -> Option<(u64, u32)> {
    if message.first()? & 0x0f != ANNOUNCE || message.len() < ANNOUNCE_LEN {
        return None;
    }
    let grandmaster = u64::from_be_bytes(message[53..61].try_into().ok()?);
    let length = usize::from(u16::from_be_bytes([message[2], message[3]])).min(message.len());
    let mut at = ANNOUNCE_LEN;
    while at + 4 <= length {
        let kind = u16::from_be_bytes([message[at], message[at + 1]]);
        let size = usize::from(u16::from_be_bytes([message[at + 2], message[at + 3]]));
        let body = message.get(at + 4..at + 4 + size)?;
        if kind == ORGANIZATION_EXTENSION_DO_NOT_PROPAGATE
            && body.len() >= 10
            && body[..6] == GRANDMASTER_LINK
        {
            let speed = u32::from_be_bytes(body[6..10].try_into().ok()?);
            return (speed > 0).then_some((grandmaster, speed));
        }
        at += 4 + size;
    }
    None
}

/// How much longer, in nanoseconds, a minimum-size frame takes from a
/// grandmaster linked at `grandmaster` Mb/s to an endpoint linked at
/// `local` Mb/s than back, halved: PTP's delayAsymmetry (IEEE 1588-2019,
/// 7.4.2), which store-and-forward switches make of the two links'
/// speeds whatever lies between (profile 5). An endpoint following the
/// grandmaster settles that much behind it.
pub fn link_asymmetry(grandmaster: u32, local: u32) -> i64 {
    let held = |megabits: u32| PTP_FRAME_OCTETS * 8.0 * 1000.0 / f64::from(megabits.max(1));
    ((held(grandmaster) - held(local)) / 2.0).round() as i64
}

/// Whether a PTP message's TLVs, after its first `start` octets, hold
/// the Endpoint Declaration TLV.
fn has_endpoint_tlv(message: &[u8], start: usize) -> bool {
    let length = message
        .get(2..4)
        .map_or(0, |octets| {
            usize::from(u16::from_be_bytes([octets[0], octets[1]]))
        })
        .min(message.len());
    let mut at = start;
    while at + 4 <= length {
        let kind = u16::from_be_bytes([message[at], message[at + 1]]);
        let size = usize::from(u16::from_be_bytes([message[at + 2], message[at + 3]]));
        let Some(body) = message.get(at + 4..at + 4 + size) else {
            return false;
        };
        // The identifiers and dataField; a pad octet may follow.
        if matches!(
            kind,
            ORGANIZATION_EXTENSION_DO_NOT_PROPAGATE | ORGANIZATION_EXTENSION
        ) && body.len() >= 7
            && body[..7] == ENDPOINT_TLV[4..11]
        {
            return true;
        }
        at += 4 + size;
    }
    false
}

/// What runs PTP on the interface besides this daemon.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ptp4l {
    Absent,
    /// ptp4l running gPTP, peer to peer. It takes any Pdelay_Resp it did
    /// not ask for as a fault, so this then sends no Pdelay_Req of its own.
    Gptp,
    /// ptp4l running the AVB Lite PTP profile, end to end.
    Lite,
}

/// Decides between AVB and AVB Lite from the Pdelay exchange (profile
/// 2.2): an Endpoint Declaration TLV heard, nine requests unanswered, or
/// two responders to one request each make the fallback, which then
/// holds. Where ptp4l runs gPTP on the interface it asks the peer itself,
/// and the answers to its requests, which this hears too, count as they
/// would to this one's: whether ptp4l calls the peer asCapable does not,
/// as it does not while it starts. Where ptp4l runs the AVB Lite profile
/// nobody here asks, so it listens: one gPTP peer asking without the TLV,
/// as an AVB bridge does every second, may be a bridge; two are a switch
/// flooding the bridge-group address, as condition 3 shows by its
/// responders; and none for 10 s, condition 2's silence. A fault of
/// ptp4l's own, as a transmit time stamp coming late under load, stops its
/// requests until it resets, so none answering then does not count.
pub struct Fallback {
    identity: u64,
    sequence: u16,
    /// What else runs PTP on the interface.
    ptp4l: Ptp4l,
    /// The request waiting for answers, and who answered it.
    waiting: Option<u16>,
    responders: Vec<[u8; 10]>,
    unanswered: u32,
    /// Since when ptp4l has run gPTP, when a Pdelay_Resp to this clock last
    /// came, whoever asked, and who answered the latest request ptp4l
    /// sent.
    ptp4l_since: Option<Duration>,
    answered_at: Option<Duration>,
    ptp4l_request: Option<(u16, Vec<[u8; 10]>)>,
    /// When ptp4l last said its port is FAULTY.
    faulty_at: Option<Duration>,
    reason: Option<FallbackReason>,
    /// The link is down, so nothing can answer and nothing is weighed.
    link_down: bool,
    /// Since when it has weighed: its start, or the link coming up.
    armed_at: Duration,
    /// The gPTP peers heard asking for peer delay without the Endpoint
    /// Declaration TLV since then.
    peers: Vec<[u8; 10]>,
    next: Duration,
    outgoing: VecDeque<Vec<u8>>,
}

impl Fallback {
    /// For the interface whose clock identity is `identity`; `configured`
    /// falls back at once, as an operator asked.
    pub fn new(identity: u64, configured: bool, now: Duration) -> Self {
        Fallback {
            identity,
            sequence: 0,
            ptp4l: Ptp4l::Absent,
            waiting: None,
            responders: Vec::new(),
            unanswered: 0,
            ptp4l_since: None,
            answered_at: None,
            ptp4l_request: None,
            faulty_at: None,
            reason: configured.then_some(FallbackReason::CONFIGURED),
            link_down: false,
            armed_at: now,
            peers: Vec::new(),
            next: now + START_GRACE,
            outgoing: VecDeque::new(),
        }
    }

    pub fn reason(&self) -> Option<FallbackReason> {
        self.reason
    }

    /// What runs PTP on the interface. Where ptp4l runs gPTP, it asks the
    /// peer and this listens; where nothing does, this asks the peer
    /// itself; and after falling back this sends its beacon unless ptp4l
    /// still runs gPTP.
    pub fn set_ptp4l(&mut self, now: Duration, ptp4l: Ptp4l) {
        self.ptp4l = ptp4l;
        if ptp4l == Ptp4l::Gptp {
            self.ptp4l_since.get_or_insert(now);
        } else {
            self.ptp4l_since = None;
            self.ptp4l_request = None;
        }
    }

    /// ptp4l says its port is FAULTY: its requests stop until it resets.
    pub fn ptp4l_faulty(&mut self, now: Duration) {
        self.faulty_at = Some(now);
    }

    /// Whether the link is up. While it is down nothing is weighed, and
    /// as it comes up the weighing starts afresh (profile 2.2), a fallback
    /// already made holding.
    pub fn set_link(&mut self, now: Duration, up: bool) {
        if up && self.link_down {
            self.waiting = None;
            self.responders.clear();
            self.unanswered = 0;
            self.answered_at = None;
            self.ptp4l_request = None;
            if self.ptp4l_since.is_some() {
                self.ptp4l_since = Some(now);
            }
            self.armed_at = now;
            self.peers.clear();
            self.next = now + START_GRACE;
        }
        self.link_down = !up;
    }

    /// How many gPTP peers it has heard asking for peer delay without the
    /// Endpoint Declaration TLV since it started weighing.
    pub fn gptp_peers(&self) -> usize {
        self.peers.len()
    }

    /// Whether it has weighed long enough to have heard another
    /// endpoint's beacon, two of them at least: without a fallback by
    /// then, the link has no AVB Lite endpoint saying so.
    pub fn settled(&self, now: Duration) -> bool {
        !self.link_down && now >= self.armed_at + PEERLESS
    }

    fn fall_back(&mut self, reason: FallbackReason) {
        if self.reason.is_none() {
            self.reason = Some(reason);
        }
    }

    /// Hands it a PTP message heard on the interface, after the
    /// ethertype.
    pub fn handle_ptp(&mut self, now: Duration, message: &[u8]) {
        if self.link_down {
            return;
        }
        let Some(&first) = message.first() else {
            return;
        };
        if message.len() < PDELAY_LEN {
            return;
        }
        let kind = first & 0x0f;
        let endpoint = has_endpoint_tlv(message, PDELAY_LEN);
        if matches!(kind, PDELAY_REQ | PDELAY_RESP | PDELAY_RESP_FOLLOW_UP) && endpoint {
            self.fall_back(FallbackReason::ENDPOINT_TLV);
        }
        if kind == PDELAY_REQ && !endpoint {
            let requester = u64::from_be_bytes(message[20..28].try_into().unwrap_or_default());
            let mut peer = [0; 10];
            peer.copy_from_slice(&message[20..30]);
            if requester != self.identity && !self.peers.contains(&peer) {
                self.peers.push(peer);
                // An AVB bridge's port is the only peer that asks; two
                // asking is a switch flooding the bridge-group address.
                if self.peers.len() >= 2 {
                    self.fall_back(FallbackReason::MULTIPLE_RESPONDERS);
                }
            }
        }
        if kind != PDELAY_RESP {
            return;
        }
        let sequence = u16::from_be_bytes([message[30], message[31]]);
        let requester = u64::from_be_bytes(message[44..52].try_into().unwrap_or_default());
        if requester != self.identity {
            return;
        }
        self.answered_at = Some(now);
        let mut responder = [0; 10];
        responder.copy_from_slice(&message[20..30]);
        let responders = if self.waiting == Some(sequence) {
            &mut self.responders
        } else if self.ptp4l == Ptp4l::Gptp {
            // ptp4l's request: who answered it.
            let request = self.ptp4l_request.get_or_insert((sequence, Vec::new()));
            if request.0 != sequence {
                *request = (sequence, Vec::new());
            }
            &mut request.1
        } else {
            return;
        };
        if !responders.contains(&responder) {
            responders.push(responder);
        }
        if responders.len() >= 2 {
            self.fall_back(FallbackReason::MULTIPLE_RESPONDERS);
        }
    }

    pub fn handle_timeout(&mut self, now: Duration) {
        if self.link_down {
            self.next = now + PROBE_INTERVAL;
            return;
        }
        // Where ptp4l asks, nine requests unanswered are 10 s without an
        // answer to it, counted from when ptp4l was first seen; not while
        // its own fault stops it asking.
        let faulted = self.faulty_at.is_some_and(|at| now < at + FAULT_GRACE);
        if let Some(since) = self.ptp4l_since
            && !faulted
        {
            let last = self.answered_at.map_or(since, |at| at.max(since));
            if now.saturating_sub(last) >= PEERLESS {
                self.fall_back(FallbackReason::PDELAY_UNANSWERED);
            }
        }
        // Where ptp4l runs the AVB Lite profile nobody here asks: 10 s
        // with no gPTP peer asking either is condition 2's silence.
        if self.ptp4l == Ptp4l::Lite && self.peers.is_empty() && self.settled(now) {
            self.fall_back(FallbackReason::PDELAY_UNANSWERED);
        }
        if now < self.next {
            return;
        }
        if self.reason.is_some() {
            // An endpoint that fell back says so every 3 s, for peers
            // still in gPTP (profile 2.3), once ptp4l no longer does.
            if self.ptp4l != Ptp4l::Gptp {
                self.send_request();
            }
            self.next = now + BEACON_INTERVAL;
            return;
        }
        if self.ptp4l != Ptp4l::Absent {
            self.waiting = None;
            self.next = now + PROBE_INTERVAL;
            return;
        }
        if self.waiting.is_some() && self.responders.is_empty() {
            self.unanswered += 1;
            if self.unanswered >= MOST_UNANSWERED {
                self.fall_back(FallbackReason::PDELAY_UNANSWERED);
            }
        } else if self.waiting.is_some() {
            self.unanswered = 0;
        }
        self.responders.clear();
        self.waiting = Some(self.send_request());
        self.next = now + PROBE_INTERVAL;
    }

    fn send_request(&mut self) -> u16 {
        let sequence = self.sequence;
        self.sequence = self.sequence.wrapping_add(1);
        self.outgoing
            .push_back(pdelay_request(self.identity, sequence));
        sequence
    }

    pub fn poll_timeout(&self) -> Duration {
        self.next
    }

    /// A PTP message to send to [`PDELAY_DESTINATION`].
    pub fn poll_transmit(&mut self) -> Option<Vec<u8>> {
        self.outgoing.pop_front()
    }
}

/// When to try gPTP again from AVB Lite fallen back to as no gPTP peer
/// answered, which ptp4l's own lasting fault makes too: once one gPTP peer
/// asks for peer delay without the Endpoint Declaration TLV, as an AVB
/// bridge does every second, after a wait each try that falls back again
/// soon doubles.
#[derive(Debug, Default)]
pub struct Retry {
    wait: Duration,
    at: Option<Duration>,
    tried_at: Option<Duration>,
}

impl Retry {
    /// The endpoints fell back to AVB Lite for `reason`: only peers not
    /// answering is tried again.
    pub fn fell_back(&mut self, now: Duration, reason: FallbackReason) {
        if reason != FallbackReason::PDELAY_UNANSWERED {
            self.at = None;
            return;
        }
        let again = self.tried_at.is_some_and(|tried| now < tried + RETRY_HELD);
        self.wait = if again {
            (self.wait * 2).clamp(RETRY_FIRST, RETRY_LONGEST)
        } else {
            RETRY_FIRST
        };
        self.at = Some(now + self.wait);
    }

    /// How long it waits before trying gPTP again, while it waits.
    pub fn wait(&self) -> Option<Duration> {
        self.at.map(|_| self.wait)
    }

    /// Whether to try gPTP now, with `gptp_peers` peers heard asking for
    /// peer delay: one, as an AVB bridge, shows the link has one.
    pub fn due(&self, now: Duration, gptp_peers: usize) -> bool {
        self.at.is_some_and(|at| now >= at) && gptp_peers == 1
    }

    /// It tries gPTP now.
    pub fn tried(&mut self, now: Duration) {
        self.at = None;
        self.tried_at = Some(now);
    }
}

/// An AECP vendor unique command carrying one MSRP message as CVU SRP
/// (profile 6), from `sender`.
pub fn cvu_command(sender: EntityId, sequence: u16, message: &mrpdu::Message) -> Option<Vec<u8>> {
    let pdu = mrpdu::encode(std::slice::from_ref(message), &msrp::FORMAT);
    // The MSRP message alone: no ProtocolVersion before it, nor the
    // MRPDU's EndMark after.
    let msrp = pdu.get(1..pdu.len().checked_sub(2)?)?;
    let mut payload = vec![message.attribute_type];
    payload.extend_from_slice(msrp);
    let command = VendorUniquePdu {
        header: AecpHeader {
            message_type: AecpMessageType::VENDOR_UNIQUE_COMMAND,
            status: 0,
            target_entity_id: EntityId(0),
            controller_entity_id: sender,
            sequence_id: sequence,
        },
        protocol_id: CVU_PROTOCOL_ID,
        payload: &payload,
    };
    let mut out = vec![0; 1500];
    let length = command.encode(&mut out).ok()?;
    out.truncate(length);
    Some(out)
}

/// What one CVU SRP command declares.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declared {
    /// A talker's stream, or that it is gone.
    Talker(TalkerDeclaration),
    TalkerGone(u64),
    /// A listener's word on a stream.
    Listener(u64, ListenerState),
    ListenerGone(u64),
}

/// The declarations of a CVU SRP command, when the AECP PDU is one.
pub fn cvu_declarations(pdu: &[u8]) -> Option<Vec<Declared>> {
    let pdu = VendorUniquePdu::decode(pdu).ok()?;
    if pdu.header.message_type != AecpMessageType::VENDOR_UNIQUE_COMMAND {
        return None;
    }
    let message = CvuMessage::from_pdu(&pdu).ok()?;
    let mut mrpdu = vec![0];
    mrpdu.extend_from_slice(message.msrp);
    mrpdu.extend_from_slice(&[0, 0]);
    let decoded = mrpdu::decode(&mrpdu, &msrp::FORMAT);
    let stream = |value: &[u8]| {
        value
            .get(..8)
            .and_then(|octets| octets.try_into().ok())
            .map(u64::from_be_bytes)
    };
    let declared = decoded
        .values
        .into_iter()
        .filter_map(|value| {
            let declares = matches!(value.event, Event::New | Event::JoinIn | Event::JoinMt);
            let leaves = value.event == Event::Lv;
            match value.attribute_type {
                msrp::attribute::TALKER_ADVERTISE | msrp::attribute::TALKER_FAILED => {
                    if declares {
                        msrp::decode_talker(value.attribute_type, &value.value, value.event)
                            .map(Declared::Talker)
                    } else if leaves {
                        stream(&value.value).map(Declared::TalkerGone)
                    } else {
                        None
                    }
                }
                msrp::attribute::LISTENER => {
                    let id = stream(&value.value)?;
                    if declares {
                        Some(Declared::Listener(
                            id,
                            ListenerState::from_number(value.four_packed.unwrap_or(0)),
                        ))
                    } else if leaves {
                        Some(Declared::ListenerGone(id))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .collect();
    Some(declared)
}

/// The MSRP message declaring a talker's stream, or withdrawing it.
pub fn talker_message(declaration: &TalkerDeclaration, event: Event) -> mrpdu::Message {
    mrpdu::Message {
        attribute_type: if declaration.failure.is_some() {
            msrp::attribute::TALKER_FAILED
        } else {
            msrp::attribute::TALKER_ADVERTISE
        },
        leave_all: false,
        values: vec![(msrp::talker_value(declaration), event, None)],
    }
}

/// The MSRP message giving a listener's word on `stream_id`, or
/// withdrawing it.
pub fn listener_message(stream_id: u64, state: ListenerState, event: Event) -> mrpdu::Message {
    mrpdu::Message {
        attribute_type: msrp::attribute::LISTENER,
        leave_all: false,
        values: vec![(
            stream_id.to_be_bytes().to_vec(),
            event,
            Some(state.number()),
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const IDENTITY: u64 = 0x9c6b_00ff_fe30_9a2b;

    /// A Pdelay_Resp from `responder` to request `sequence` of
    /// `requester`, with the Endpoint Declaration TLV or not.
    fn response(responder: u64, requester: u64, sequence: u16, endpoint: bool) -> Vec<u8> {
        let mut message = vec![0; PDELAY_LEN];
        message[0] = 0x10 | PDELAY_RESP;
        message[1] = 0x02;
        let length = PDELAY_LEN + if endpoint { ENDPOINT_TLV.len() } else { 0 };
        message[2..4].copy_from_slice(&(length as u16).to_be_bytes());
        message[20..28].copy_from_slice(&responder.to_be_bytes());
        message[28..30].copy_from_slice(&1u16.to_be_bytes());
        message[30..32].copy_from_slice(&sequence.to_be_bytes());
        message[44..52].copy_from_slice(&requester.to_be_bytes());
        message[52..54].copy_from_slice(&1u16.to_be_bytes());
        if endpoint {
            message.extend_from_slice(&ENDPOINT_TLV);
        }
        message
    }

    /// Runs the detector from second `from` to `until`, answering each
    /// request with `answer`.
    fn run(fallback: &mut Fallback, from: u64, until: u64, answer: impl Fn(u16) -> Vec<Vec<u8>>) {
        for tick in from * 10..until * 10 {
            let now = Duration::from_millis(tick * 100);
            fallback.handle_timeout(now);
            while let Some(request) = fallback.poll_transmit() {
                let sequence = u16::from_be_bytes([request[30], request[31]]);
                for message in answer(sequence) {
                    fallback.handle_ptp(now, &message);
                }
            }
        }
    }

    /// An Announce from `grandmaster` with a Grandmaster Link TLV giving
    /// `speed`, after a TLV of another kind.
    fn announce(grandmaster: u64, speed: u32) -> Vec<u8> {
        let mut message = vec![0; ANNOUNCE_LEN];
        message[0] = ANNOUNCE;
        message[1] = 0x02;
        message[53..61].copy_from_slice(&grandmaster.to_be_bytes());
        // A path trace TLV, then the Grandmaster Link TLV.
        message.extend_from_slice(&[0x00, 0x08, 0x00, 0x08]);
        message.extend_from_slice(&grandmaster.to_be_bytes());
        message.extend_from_slice(&[0x80, 0x00, 0x00, 0x0a]);
        message.extend_from_slice(&GRANDMASTER_LINK);
        message.extend_from_slice(&speed.to_be_bytes());
        let length = message.len() as u16;
        message[2..4].copy_from_slice(&length.to_be_bytes());
        message
    }

    #[test]
    fn an_announce_tells_the_grandmasters_link_speed() {
        let message = announce(0xe8f6_0aff_fee0_9220, 100);
        assert_eq!(
            grandmaster_link(&message),
            Some((0xe8f6_0aff_fee0_9220, 100))
        );
        // Without the TLV, or from another kind of message, nothing.
        assert_eq!(grandmaster_link(&message[..ANNOUNCE_LEN]), None);
        assert_eq!(grandmaster_link(&pdelay_request(IDENTITY, 1)), None);
    }

    /// The profile's example: a 1 Gb/s endpoint following a 100 Mb/s
    /// grandmaster settles 2.3 us behind it; the other way round, ahead;
    /// at one speed, neither.
    #[test]
    fn mixed_link_speeds_make_an_asymmetry() {
        assert_eq!(link_asymmetry(100, 1000), 2304);
        assert_eq!(link_asymmetry(1000, 100), -2304);
        assert_eq!(link_asymmetry(1000, 1000), 0);
        assert_eq!(link_asymmetry(100, 10_000), 2534);
    }

    #[test]
    fn requests_carry_the_endpoint_declaration() {
        let request = pdelay_request(IDENTITY, 7);
        assert_eq!(request.len(), 66);
        assert_eq!(&request[..4], &[0x12, 0x02, 0x00, 0x42]);
        assert_eq!(&request[54..58], &[0x80, 0x00, 0x00, 0x08]);
        assert!(has_endpoint_tlv(&request, PDELAY_LEN));
        assert!(!has_endpoint_tlv(&request[..PDELAY_LEN], PDELAY_LEN));
        // An endpoint of the profile's first revision, its TLV of type
        // 0x0003 and seven octets long with no pad, is still recognized.
        let mut older = request[..PDELAY_LEN].to_vec();
        older[2..4].copy_from_slice(&65u16.to_be_bytes());
        older.extend_from_slice(&[0x00, 0x03, 0x00, 0x07]);
        older.extend_from_slice(&ENDPOINT_TLV[4..11]);
        assert!(has_endpoint_tlv(&older, PDELAY_LEN));
    }

    #[test]
    fn a_single_bridge_answering_keeps_avb() {
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Absent);
        run(&mut fallback, 0, 30, |sequence| {
            vec![response(0x0001_f2ff_feff_3b14, IDENTITY, sequence, false)]
        });
        assert_eq!(fallback.reason(), None);
    }

    #[test]
    fn nine_unanswered_requests_fall_back() {
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Absent);
        // The first request goes out after the start's grace of 3 s.
        run(&mut fallback, 0, 11, |_| Vec::new());
        assert_eq!(fallback.reason(), None);
        run(&mut fallback, 11, 14, |_| Vec::new());
        assert_eq!(fallback.reason(), Some(FallbackReason::PDELAY_UNANSWERED));
    }

    /// ptp4l running gPTP, answered every second until `faulty_from`,
    /// then FAULTY until `faulty_until` and silent, and answered again from
    /// `answered_again`, if ever; run to `until` seconds.
    fn ptp4l_faulting(
        fallback: &mut Fallback,
        faulty_from: u64,
        faulty_until: u64,
        answered_again: Option<u64>,
        until: u64,
    ) {
        const BRIDGE: u64 = 0x0001_f2ff_feff_3b14;
        for second in 0..until {
            let now = Duration::from_secs(second);
            let answered =
                second < faulty_from || answered_again.is_some_and(|again| second >= again);
            if answered {
                fallback.handle_ptp(now, &response(BRIDGE, IDENTITY, second as u16, false));
            }
            // pmc asks every 2 s.
            if (faulty_from..=faulty_until).contains(&second) && second % 2 == 0 {
                fallback.ptp4l_faulty(now);
            }
            fallback.handle_timeout(now);
        }
    }

    #[test]
    fn a_fault_of_ptp4ls_own_does_not_fall_back() {
        // Unanswered for 10 s, with no fault, falls back as before.
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Gptp);
        ptp4l_faulting(&mut fallback, 20, 0, None, 31);
        assert_eq!(fallback.reason(), Some(FallbackReason::PDELAY_UNANSWERED));
        // FAULTY for 16 s, as ptp4l resets, then answered again.
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Gptp);
        ptp4l_faulting(&mut fallback, 20, 36, Some(38), 120);
        assert_eq!(fallback.reason(), None);
    }

    #[test]
    fn a_lasting_fault_falls_back_after_its_grace() {
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Gptp);
        // Last FAULTY at 36 s; nothing answers after.
        ptp4l_faulting(&mut fallback, 20, 36, None, 68);
        assert_eq!(fallback.reason(), None);
        fallback.handle_timeout(Duration::from_secs(68));
        assert_eq!(fallback.reason(), Some(FallbackReason::PDELAY_UNANSWERED));
    }

    #[test]
    fn gptp_is_tried_again_once_a_bridge_asks() {
        let seconds = Duration::from_secs;
        let mut retry = Retry::default();
        retry.fell_back(seconds(0), FallbackReason::PDELAY_UNANSWERED);
        assert_eq!(retry.wait(), Some(seconds(60)));
        assert!(!retry.due(seconds(59), 1));
        assert!(retry.due(seconds(60), 1));
        // No peer asking, or two as a flooding switch, is no AVB bridge.
        assert!(!retry.due(seconds(60), 0));
        assert!(!retry.due(seconds(60), 2));
        // Falling back again soon after a try waits twice as long, up to
        // half an hour.
        retry.tried(seconds(60));
        assert!(!retry.due(seconds(61), 1));
        retry.fell_back(seconds(100), FallbackReason::PDELAY_UNANSWERED);
        assert_eq!(retry.wait(), Some(seconds(120)));
        let mut now = seconds(100);
        for _ in 0..10 {
            now += retry.wait().unwrap();
            retry.tried(now);
            retry.fell_back(now + seconds(10), FallbackReason::PDELAY_UNANSWERED);
            now += seconds(10);
        }
        assert_eq!(retry.wait(), Some(seconds(30 * 60)));
        // gPTP held five minutes: the next fallback waits a minute again.
        retry.tried(now);
        retry.fell_back(now + seconds(5 * 60), FallbackReason::PDELAY_UNANSWERED);
        assert_eq!(retry.wait(), Some(seconds(60)));
        // An AVB Lite endpoint on the link, or a choice, is not tried again.
        for reason in [
            FallbackReason::ENDPOINT_TLV,
            FallbackReason::MULTIPLE_RESPONDERS,
            FallbackReason::CONFIGURED,
        ] {
            retry.fell_back(now, reason);
            assert_eq!(retry.wait(), None);
            assert!(!retry.due(now + seconds(3600), 1));
        }
    }

    #[test]
    fn two_responders_or_an_endpoint_fall_back() {
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Absent);
        run(&mut fallback, 0, 6, |sequence| {
            vec![
                response(0x1111_11ff_fe11_1111, IDENTITY, sequence, false),
                response(0x2222_22ff_fe22_2222, IDENTITY, sequence, false),
            ]
        });
        assert_eq!(fallback.reason(), Some(FallbackReason::MULTIPLE_RESPONDERS));

        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Gptp);
        // Another endpoint's request, heard through a plain switch.
        fallback.handle_ptp(Duration::ZERO, &pdelay_request(0x3333_33ff_fe33_3333, 1));
        assert_eq!(fallback.reason(), Some(FallbackReason::ENDPOINT_TLV));
    }

    /// A Sync padded to Ethernet's least frame is as it should be; one
    /// carrying the driver's 12-octet time stamp is not.
    #[test]
    fn a_time_stamp_left_on_a_frame_shows() {
        let mut sync = vec![0u8; 44];
        sync[2..4].copy_from_slice(&44u16.to_be_bytes());
        assert!(!stamped(&sync));
        sync.extend([0; 2]);
        assert!(!stamped(&sync));
        sync.extend([0xa5; 12]);
        assert!(stamped(&sync));
        assert!(!stamped(&[0x0b, 0x02]));
    }

    /// Where ptp4l runs the AVB Lite profile it listens: one gPTP peer
    /// asking without the TLV keeps AVB open, its own requests and an
    /// endpoint's do not count, two peers fall back, and so do 10 s
    /// without any.
    #[test]
    fn under_the_lite_profile_it_listens_for_peers() {
        const BRIDGE: u64 = 0x0001_f2ff_feff_3b14;
        let plain = |identity: u64, sequence: u16| {
            let mut message = pdelay_request(identity, sequence);
            message.truncate(PDELAY_LEN);
            message[2..4].copy_from_slice(&(PDELAY_LEN as u16).to_be_bytes());
            message
        };
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Lite);
        for second in 0..20u16 {
            let now = Duration::from_secs(u64::from(second));
            fallback.handle_timeout(now);
            assert_eq!(fallback.poll_transmit(), None);
            fallback.handle_ptp(now, &plain(BRIDGE, second));
            // ptp4l's own requests, as this hears them.
            fallback.handle_ptp(now, &plain(IDENTITY, second));
        }
        assert_eq!(fallback.gptp_peers(), 1);
        assert_eq!(fallback.reason(), None);
        fallback.handle_ptp(Duration::from_secs(20), &plain(0x3333_33ff_fe33_3333, 1));
        assert_eq!(fallback.reason(), Some(FallbackReason::MULTIPLE_RESPONDERS));

        // An endpoint's beacon is no gPTP peer: it falls back for itself.
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Lite);
        fallback.handle_ptp(Duration::ZERO, &pdelay_request(0x3333_33ff_fe33_3333, 1));
        assert_eq!(fallback.gptp_peers(), 0);
        assert_eq!(fallback.reason(), Some(FallbackReason::ENDPOINT_TLV));

        // Nobody asking for 10 s.
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Lite);
        for second in 0..10 {
            fallback.handle_timeout(Duration::from_secs(second));
        }
        assert_eq!(fallback.reason(), None);
        fallback.handle_timeout(Duration::from_secs(10));
        assert_eq!(fallback.reason(), Some(FallbackReason::PDELAY_UNANSWERED));
    }

    /// It has weighed long enough 10 s after it starts or the link comes
    /// up, and never while the link is down.
    #[test]
    fn it_settles_after_ten_seconds_of_link() {
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        assert!(!fallback.settled(Duration::from_secs(9)));
        assert!(fallback.settled(Duration::from_secs(10)));
        fallback.set_link(Duration::from_secs(20), false);
        assert!(!fallback.settled(Duration::from_secs(40)));
        fallback.set_link(Duration::from_secs(40), true);
        assert!(!fallback.settled(Duration::from_secs(49)));
        assert!(fallback.settled(Duration::from_secs(50)));
    }

    /// With the cable out nothing answers, which is no reason to fall back:
    /// nothing is weighed until the link comes up, and then afresh.
    #[test]
    fn nothing_is_weighed_while_the_link_is_down() {
        const BRIDGE: u64 = 0x0001_f2ff_feff_3b14;
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Gptp);
        for second in 0..10u16 {
            let now = Duration::from_secs(u64::from(second));
            fallback.handle_timeout(now);
            fallback.handle_ptp(now, &response(BRIDGE, IDENTITY, second, false));
        }
        fallback.set_link(Duration::from_secs(10), false);
        for second in 10..70 {
            fallback.handle_timeout(Duration::from_secs(second));
            assert!(fallback.poll_timeout() > Duration::from_secs(second));
        }
        assert_eq!(fallback.reason(), None);
        // Up again: the bridge answers from 72 s, within the 10 s ptp4l
        // has from the link coming up.
        fallback.set_link(Duration::from_secs(70), true);
        for second in 70..90u16 {
            let now = Duration::from_secs(u64::from(second));
            fallback.handle_timeout(now);
            if second >= 72 {
                fallback.handle_ptp(now, &response(BRIDGE, IDENTITY, second, false));
            }
        }
        assert_eq!(fallback.reason(), None);

        // Nothing running PTP: requests unanswered while the link is down
        // do not count either.
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Absent);
        fallback.set_link(Duration::ZERO, false);
        run(&mut fallback, 0, 30, |_| Vec::new());
        assert_eq!(fallback.reason(), None);
        fallback.set_link(Duration::from_secs(30), true);
        run(&mut fallback, 30, 36, |_| Vec::new());
        assert_eq!(fallback.reason(), None);
        run(&mut fallback, 36, 45, |_| Vec::new());
        assert_eq!(fallback.reason(), Some(FallbackReason::PDELAY_UNANSWERED));
    }

    /// Where ptp4l runs, this sends nothing of its own and goes by the
    /// answers to ptp4l's requests, asCapable or not: a bridge answering
    /// keeps AVB, two responders to one request fall back, and so do 10 s
    /// without an answer.
    #[test]
    fn where_ptp4l_runs_it_goes_by_the_answers_to_ptp4l() {
        const BRIDGE: u64 = 0x0001_f2ff_feff_3b14;
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Gptp);
        for second in 0..30u16 {
            let now = Duration::from_secs(u64::from(second));
            fallback.handle_timeout(now);
            assert_eq!(fallback.poll_transmit(), None);
            // ptp4l's request `second`, which the bridge answers.
            fallback.handle_ptp(now, &response(BRIDGE, IDENTITY, second, false));
        }
        assert_eq!(fallback.reason(), None);
        // The bridge stops answering: 10 s later, the fallback.
        for second in 30..41 {
            fallback.handle_timeout(Duration::from_secs(second));
        }
        assert_eq!(fallback.reason(), Some(FallbackReason::PDELAY_UNANSWERED));

        // Two responders to one of ptp4l's requests.
        let mut fallback = Fallback::new(IDENTITY, false, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Gptp);
        fallback.handle_ptp(Duration::ZERO, &response(BRIDGE, IDENTITY, 4, false));
        assert_eq!(fallback.reason(), None);
        fallback.handle_ptp(
            Duration::ZERO,
            &response(0x2222_22ff_fe22_2222, IDENTITY, 4, false),
        );
        assert_eq!(fallback.reason(), Some(FallbackReason::MULTIPLE_RESPONDERS));
    }

    /// After falling back, no beacon goes out while ptp4l still runs gPTP:
    /// an answer to it would make ptp4l fault. Once ptp4l runs AVB Lite's
    /// profile, the beacon goes out again.
    #[test]
    fn no_beacon_while_ptp4l_runs_gptp() {
        let mut fallback = Fallback::new(IDENTITY, true, Duration::ZERO);
        fallback.set_ptp4l(Duration::ZERO, Ptp4l::Gptp);
        for second in 0..10 {
            fallback.handle_timeout(Duration::from_secs(second));
        }
        assert_eq!(fallback.poll_transmit(), None);
        fallback.set_ptp4l(Duration::from_secs(10), Ptp4l::Lite);
        fallback.handle_timeout(Duration::from_secs(13));
        assert!(fallback.poll_transmit().is_some());
    }

    #[test]
    fn after_falling_back_a_beacon_goes_out_every_3_s() {
        let mut fallback = Fallback::new(IDENTITY, true, Duration::ZERO);
        assert_eq!(fallback.reason(), Some(FallbackReason::CONFIGURED));
        let mut beacons = 0;
        // At 3, 6 and 9 s, the first after the start's grace.
        for tick in 0..120 {
            fallback.handle_timeout(Duration::from_millis(tick * 100));
            while let Some(beacon) = fallback.poll_transmit() {
                assert!(has_endpoint_tlv(&beacon, PDELAY_LEN));
                beacons += 1;
            }
        }
        assert_eq!(beacons, 3);
    }

    #[test]
    fn cvu_commands_carry_one_msrp_message() {
        let declaration = TalkerDeclaration {
            stream_id: 0x9c6b_0030_9a2b_0000,
            destination: MacAddress([0x91, 0xe0, 0xf0, 0x00, 0x12, 0x34]),
            vlan_id: 2,
            max_frame_size: 224,
            max_interval_frames: 1,
            priority: 5,
            rank: true,
            accumulated_latency: 125_000,
            failure: None,
            event: Event::JoinIn,
        };
        let sender = EntityId(0x9c6b_00ff_0030_9a2b);
        let command = cvu_command(sender, 3, &talker_message(&declaration, Event::JoinIn)).unwrap();
        let pdu = VendorUniquePdu::decode(&command).unwrap();
        assert_eq!(pdu.protocol_id, CVU_PROTOCOL_ID);
        assert_eq!(pdu.header.target_entity_id, EntityId(0));
        // The command type, then the message: type, length, list length.
        assert_eq!(&pdu.payload[..3], &[1, 1, 25]);
        let declared = cvu_declarations(&command).unwrap();
        assert!(
            matches!(&declared[..], [Declared::Talker(found)] if found.stream_id == declaration.stream_id
                && found.destination == declaration.destination
                && found.max_frame_size == 224)
        );
        let ready = cvu_command(
            sender,
            4,
            &listener_message(declaration.stream_id, ListenerState::Ready, Event::JoinIn),
        )
        .unwrap();
        assert_eq!(
            cvu_declarations(&ready).unwrap(),
            [Declared::Listener(
                declaration.stream_id,
                ListenerState::Ready
            )]
        );
        let gone = cvu_command(
            sender,
            5,
            &listener_message(declaration.stream_id, ListenerState::Ready, Event::Lv),
        )
        .unwrap();
        assert_eq!(
            cvu_declarations(&gone).unwrap(),
            [Declared::ListenerGone(declaration.stream_id)]
        );
        // A response, as earlier implementations sent, declares nothing.
        let response = VendorUniquePdu {
            header: AecpHeader {
                message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
                ..pdu.header
            },
            ..pdu
        };
        let mut out = vec![0; 200];
        let length = response.encode(&mut out).unwrap();
        assert_eq!(cvu_declarations(&out[..length]), None);
    }
}
