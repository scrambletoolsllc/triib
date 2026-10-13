//! MAAP, the MAC Address Acquisition Protocol (IEEE 1722-2016, Annex B):
//! claiming a block of the multicast addresses talkers stream to. A claim
//! probes three times, then announces the block and defends it; when two
//! claims meet, the lower MAC address keeps the block.
//!
//! The first address tried comes from the interface's address, so a
//! device claims the same block each time it starts unless another holds
//! it, which keeps what bridges remember of its streams.

use alloc::collections::VecDeque;
use core::time::Duration;

use crate::avtp::subtype;
use crate::{Instant, MacAddress};

/// Where MAAP messages go.
pub const DESTINATION: MacAddress = MacAddress([0x91, 0xe0, 0xf0, 0x00, 0xff, 0x00]);
/// Octets of a MAAP PDU.
pub const PDU_LEN: usize = 28;

/// The dynamic allocation pool: 91:E0:F0:00:00:00 to 91:E0:F0:00:FD:FF.
const POOL_START: u64 = 0x91e0_f000_0000;
const POOL_SIZE: u64 = 0xfe00;
const PROBES: u8 = 3;
const PROBE_INTERVAL: Duration = Duration::from_millis(500);
const PROBE_VARIATION: Duration = Duration::from_millis(100);
const ANNOUNCE_INTERVAL: Duration = Duration::from_secs(30);
const ANNOUNCE_VARIATION: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Probe = 1,
    Defend = 2,
    Announce = 3,
}

/// A MAAP PDU's fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Message {
    pub message_type: MessageType,
    pub requested_start: MacAddress,
    pub requested_count: u16,
    /// For a defense: the part of the request that is taken.
    pub conflict_start: MacAddress,
    pub conflict_count: u16,
}

impl Message {
    pub fn encode(&self) -> [u8; PDU_LEN] {
        let mut out = [0; PDU_LEN];
        out[0] = subtype::MAAP;
        // sv 0, version 0, the message type.
        out[1] = self.message_type as u8;
        // MAAP version 1 and 16 octets of control data; no stream ID.
        let word = (1u16 << 11) | 16;
        out[2..4].copy_from_slice(&word.to_be_bytes());
        out[12..18].copy_from_slice(&self.requested_start.0);
        out[18..20].copy_from_slice(&self.requested_count.to_be_bytes());
        out[20..26].copy_from_slice(&self.conflict_start.0);
        out[26..28].copy_from_slice(&self.conflict_count.to_be_bytes());
        out
    }

    pub fn decode(pdu: &[u8]) -> Option<Self> {
        let pdu: &[u8; PDU_LEN] = pdu.get(..PDU_LEN)?.try_into().ok()?;
        if pdu[0] != subtype::MAAP || pdu[1] & 0x80 != 0 {
            return None;
        }
        let message_type = match pdu[1] & 0x0f {
            1 => MessageType::Probe,
            2 => MessageType::Defend,
            3 => MessageType::Announce,
            _ => return None,
        };
        let mac = |at: usize| {
            let mut octets = [0; 6];
            octets.copy_from_slice(&pdu[at..at + 6]);
            MacAddress(octets)
        };
        Some(Message {
            message_type,
            requested_start: mac(12),
            requested_count: u16::from_be_bytes([pdu[18], pdu[19]]),
            conflict_start: mac(20),
            conflict_count: u16::from_be_bytes([pdu[26], pdu[27]]),
        })
    }
}

/// What a claim reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaapEvent {
    /// The block starting at this address is ours.
    Acquired(MacAddress),
    /// Another device holds the block: the addresses must stop being used
    /// while another is claimed.
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Idle,
    Probing { left: u8, at: Instant },
    Defending { at: Instant },
}

/// A claim on a block of `count` addresses.
pub struct Maap {
    mac: MacAddress,
    count: u16,
    /// The block's first address, as an offset into the pool.
    offset: u64,
    /// How many blocks were given up, which moves the next one.
    moves: u64,
    state: State,
    random: u64,
    outgoing: VecDeque<(MacAddress, [u8; PDU_LEN])>,
    events: VecDeque<MaapEvent>,
}

fn as_number(mac: MacAddress) -> u64 {
    let mut octets = [0; 8];
    octets[2..].copy_from_slice(&mac.0);
    u64::from_be_bytes(octets)
}

fn from_number(number: u64) -> MacAddress {
    let mut mac = [0; 6];
    mac.copy_from_slice(&number.to_be_bytes()[2..]);
    MacAddress(mac)
}

impl Maap {
    /// A claim for `count` addresses (1 to the pool's size) by the
    /// interface with `mac`.
    pub fn new(mac: MacAddress, count: u16) -> Self {
        let count = count.clamp(1, POOL_SIZE as u16);
        let seed = as_number(mac);
        Maap {
            mac,
            count,
            offset: 0,
            moves: 0,
            state: State::Idle,
            random: seed | 1,
            outgoing: VecDeque::new(),
            events: VecDeque::new(),
        }
    }

    /// Starts claiming.
    pub fn start(&mut self, now: Instant) {
        self.pick();
        self.probe(now, PROBES);
    }

    /// How many addresses the claim is for.
    pub fn count(&self) -> u16 {
        self.count
    }

    /// The block's addresses, once acquired.
    pub fn address(&self, index: u16) -> Option<MacAddress> {
        (matches!(self.state, State::Defending { .. }) && index < self.count)
            .then(|| from_number(POOL_START + self.offset + u64::from(index)))
    }

    /// Hands the claim a MAAP PDU `source` sent.
    pub fn handle_frame(&mut self, now: Instant, source: MacAddress, pdu: &[u8]) {
        let Some(message) = Message::decode(pdu) else {
            return;
        };
        if source == self.mac || !self.overlaps(message.requested_start, message.requested_count) {
            return;
        }
        // The lower address keeps a block both want.
        let ours = as_number(self.mac) < as_number(source);
        match (self.state, message.message_type) {
            (State::Probing { .. }, MessageType::Probe) if ours => {}
            (State::Probing { .. }, _) => {
                self.pick();
                self.probe(now, PROBES);
            }
            (State::Defending { .. }, MessageType::Probe) => {
                let (start, count) =
                    self.conflict(message.requested_start, message.requested_count);
                self.send(
                    source,
                    Message {
                        message_type: MessageType::Defend,
                        requested_start: message.requested_start,
                        requested_count: message.requested_count,
                        conflict_start: start,
                        conflict_count: count,
                    },
                );
            }
            (State::Defending { .. }, _) if ours => {}
            (State::Defending { .. }, _) => {
                self.events.push_back(MaapEvent::Lost);
                self.pick();
                self.probe(now, PROBES);
            }
            (State::Idle, _) => {}
        }
    }

    pub fn handle_timeout(&mut self, now: Instant) {
        match self.state {
            State::Probing { left, at } if at <= now => {
                if left > 1 {
                    self.probe(now, left - 1);
                } else {
                    self.events
                        .push_back(MaapEvent::Acquired(from_number(POOL_START + self.offset)));
                    self.announce(now);
                }
            }
            State::Defending { at } if at <= now => self.announce(now),
            _ => {}
        }
    }

    pub fn poll_timeout(&self) -> Option<Instant> {
        match self.state {
            State::Probing { at, .. } | State::Defending { at } => Some(at),
            State::Idle => None,
        }
    }

    /// A PDU to send, and where.
    pub fn poll_transmit(&mut self) -> Option<(MacAddress, [u8; PDU_LEN])> {
        self.outgoing.pop_front()
    }

    pub fn poll_event(&mut self) -> Option<MaapEvent> {
        self.events.pop_front()
    }

    /// The next block to try: from the interface's address, moved on by
    /// each block given up.
    fn pick(&mut self) {
        let span = POOL_SIZE - u64::from(self.count) + 1;
        let seed = as_number(self.mac) & 0xff_ffff;
        self.offset = (seed + self.moves * 0x0101) % span;
        self.moves += 1;
    }

    fn overlaps(&self, start: MacAddress, count: u16) -> bool {
        let ours = POOL_START + self.offset;
        let theirs = as_number(start);
        theirs < ours + u64::from(self.count) && ours < theirs + u64::from(count)
    }

    /// The part of a request that is ours.
    fn conflict(&self, start: MacAddress, count: u16) -> (MacAddress, u16) {
        let ours = POOL_START + self.offset;
        let first = ours.max(as_number(start));
        let last = (ours + u64::from(self.count)).min(as_number(start) + u64::from(count));
        (from_number(first), (last - first) as u16)
    }

    fn message(&self, message_type: MessageType) -> Message {
        Message {
            message_type,
            requested_start: from_number(POOL_START + self.offset),
            requested_count: self.count,
            conflict_start: MacAddress([0; 6]),
            conflict_count: 0,
        }
    }

    fn send(&mut self, destination: MacAddress, message: Message) {
        self.outgoing.push_back((destination, message.encode()));
    }

    fn probe(&mut self, now: Instant, left: u8) {
        self.send(DESTINATION, self.message(MessageType::Probe));
        let at = now + PROBE_INTERVAL + self.jitter(PROBE_VARIATION);
        self.state = State::Probing { left, at };
    }

    fn announce(&mut self, now: Instant) {
        self.send(DESTINATION, self.message(MessageType::Announce));
        let at = now + ANNOUNCE_INTERVAL + self.jitter(ANNOUNCE_VARIATION);
        self.state = State::Defending { at };
    }

    fn jitter(&mut self, limit: Duration) -> Duration {
        self.random ^= self.random << 13;
        self.random ^= self.random >> 7;
        self.random ^= self.random << 17;
        let millis = limit.as_millis() as u64;
        Duration::from_millis(self.random % (millis + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOW: MacAddress = MacAddress([0x00, 0x1b, 0x92, 0x01, 0x02, 0x03]);
    const HIGH: MacAddress = MacAddress([0xf0, 0xa7, 0x31, 0xf4, 0x0f, 0x14]);

    /// Runs claims against each other, every frame reaching every other
    /// claim, until `until`.
    fn run(claims: &mut [(MacAddress, &mut Maap)], from: Instant, until: Instant) -> Instant {
        let mut now = from;
        while now < until {
            now += Duration::from_millis(10);
            for (_, claim) in claims.iter_mut() {
                claim.handle_timeout(now);
            }
            loop {
                let mut sent = alloc::vec::Vec::new();
                for (mac, claim) in claims.iter_mut() {
                    while let Some((_, pdu)) = claim.poll_transmit() {
                        sent.push((*mac, pdu));
                    }
                }
                if sent.is_empty() {
                    break;
                }
                for (source, pdu) in sent {
                    for (mac, claim) in claims.iter_mut() {
                        if *mac != source {
                            claim.handle_frame(now, source, &pdu);
                        }
                    }
                }
            }
        }
        now
    }

    #[test]
    fn messages_round_trip() {
        let message = Message {
            message_type: MessageType::Defend,
            requested_start: MacAddress([0x91, 0xe0, 0xf0, 0x00, 0x12, 0x00]),
            requested_count: 4,
            conflict_start: MacAddress([0x91, 0xe0, 0xf0, 0x00, 0x12, 0x02]),
            conflict_count: 2,
        };
        let pdu = message.encode();
        assert_eq!(&pdu[..4], &[0xfe, 0x02, 0x08, 0x10]);
        assert_eq!(Message::decode(&pdu), Some(message));
        assert_eq!(Message::decode(&pdu[..20]), None);
    }

    #[test]
    fn a_claim_alone_takes_its_block_after_three_probes() {
        let mut claim = Maap::new(HIGH, 2);
        claim.start(Instant::ZERO);
        let mut probes = 0;
        let mut now = Instant::ZERO;
        while claim.address(0).is_none() {
            while let Some((destination, pdu)) = claim.poll_transmit() {
                assert_eq!(destination, DESTINATION);
                assert_eq!(
                    Message::decode(&pdu).unwrap().message_type,
                    MessageType::Probe
                );
                probes += 1;
            }
            now = claim.poll_timeout().unwrap();
            claim.handle_timeout(now);
        }
        assert_eq!(probes, 3);
        assert!(now >= Instant::from_millis(1500) && now <= Instant::from_millis(1800));
        let first = claim.address(0).unwrap();
        assert_eq!(claim.poll_event(), Some(MaapEvent::Acquired(first)));
        assert_eq!(&first.0[..4], &[0x91, 0xe0, 0xf0, 0x00]);
        assert_eq!(as_number(claim.address(1).unwrap()), as_number(first) + 1);
        assert_eq!(claim.address(2), None);
        // The same interface claims the same block next time.
        let mut again = Maap::new(HIGH, 2);
        again.start(Instant::ZERO);
        assert_eq!(again.offset, claim.offset);
    }

    #[test]
    fn a_held_block_is_defended_and_the_prober_moves_on() {
        let mut holder = Maap::new(HIGH, 1);
        holder.start(Instant::ZERO);
        let now = run(
            &mut [(HIGH, &mut holder)],
            Instant::ZERO,
            Instant::from_millis(2000),
        );
        let held = holder.address(0).unwrap();
        // A lower address wants the same block: the holder defends it
        // all the same, and the newcomer takes another.
        let mut newcomer = Maap::new(LOW, 1);
        newcomer.offset = holder.offset;
        newcomer.probe(now, PROBES);
        let end = run(
            &mut [(HIGH, &mut holder), (LOW, &mut newcomer)],
            now,
            now + Duration::from_secs(5),
        );
        assert_eq!(holder.address(0), Some(held));
        let other = newcomer.address(0).expect("another block");
        assert_ne!(other, held);
        assert!(end > now);
    }

    #[test]
    fn of_two_holders_of_one_block_the_lower_address_keeps_it() {
        // Two that took the same block apart, now on one network.
        let mut low = Maap::new(LOW, 1);
        let mut high = Maap::new(HIGH, 1);
        high.offset = 0x1234;
        low.offset = 0x1234;
        high.announce(Instant::ZERO);
        low.announce(Instant::ZERO);
        run(
            &mut [(LOW, &mut low), (HIGH, &mut high)],
            Instant::ZERO,
            Instant::from_millis(5000),
        );
        assert_eq!(low.address(0), Some(from_number(POOL_START + 0x1234)));
        assert_eq!(high.poll_event(), Some(MaapEvent::Lost));
        assert!(
            high.address(0)
                .is_some_and(|other| other != low.address(0).unwrap())
        );
    }
}
