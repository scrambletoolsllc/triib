//! MSRP (IEEE 802.1Q-2022, 35): its attributes as an MRP participant
//! declares and registers them, and the talker declarations in a message,
//! as an MRPDU or a CVU SRP message carries them.

use alloc::vec::Vec;

use avb_net::MacAddress;

use crate::mrpdu::{Format, add};

/// MSRP attribute types (35.2.2.4).
pub mod attribute {
    pub const TALKER_ADVERTISE: u8 = 1;
    pub const TALKER_FAILED: u8 = 2;
    pub const LISTENER: u8 = 3;
    pub const DOMAIN: u8 = 4;
}

pub use crate::mrpdu::Event;

/// The format of MSRP's MRPDUs, for a [`crate::Participant`].
pub const FORMAT: Format = Format {
    list_length: true,
    first_value_length,
    key_length,
    four_packed: |attribute_type| attribute_type == attribute::LISTENER,
    nth_value,
};

fn first_value_length(attribute_type: u8) -> Option<usize> {
    match attribute_type {
        attribute::TALKER_ADVERTISE => Some(25),
        attribute::TALKER_FAILED => Some(34),
        attribute::LISTENER => Some(8),
        attribute::DOMAIN => Some(4),
        _ => None,
    }
}

/// Talkers and listeners are named by their stream ID, domains by their
/// SR class.
fn key_length(attribute_type: u8) -> usize {
    match attribute_type {
        attribute::DOMAIN => 1,
        _ => 8,
    }
}

/// Values after the first count on its stream ID, and a talker's
/// destination too (35.2.2.8.x); a domain's class, priority and VLAN.
fn nth_value(attribute_type: u8, first: &[u8], index: u16) -> Vec<u8> {
    let mut value = Vec::from(first);
    match attribute_type {
        attribute::DOMAIN if value.len() >= 4 => {
            value[0] = value[0].wrapping_add(index as u8);
            value[1] = value[1].wrapping_add(index as u8);
            add(&mut value[2..4], index);
        }
        attribute::TALKER_ADVERTISE | attribute::TALKER_FAILED if value.len() >= 14 => {
            add(&mut value[..8], index);
            add(&mut value[8..14], index);
        }
        _ if value.len() >= 8 => add(&mut value[..8], index),
        _ => {}
    }
    value
}

/// A Listener declaration's four-packed event (35.2.2.8.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListenerState {
    Ignore,
    AskingFailed,
    Ready,
    ReadyFailed,
}

impl ListenerState {
    pub fn from_number(number: u8) -> Self {
        match number & 0x3 {
            0 => ListenerState::Ignore,
            1 => ListenerState::AskingFailed,
            2 => ListenerState::Ready,
            _ => ListenerState::ReadyFailed,
        }
    }

    pub fn number(self) -> u8 {
        match self {
            ListenerState::Ignore => 0,
            ListenerState::AskingFailed => 1,
            ListenerState::Ready => 2,
            ListenerState::ReadyFailed => 3,
        }
    }

    /// Whether at least one listener behind it can receive the stream.
    pub fn ready(self) -> bool {
        matches!(self, ListenerState::Ready | ListenerState::ReadyFailed)
    }
}

/// An SR class's domain: its class ID, priority and VLAN (35.2.2.9).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Domain {
    /// 6 for class A, 5 for class B.
    pub class_id: u8,
    pub priority: u8,
    pub vlan_id: u16,
}

impl Domain {
    /// Class A on its default priority and VLAN.
    pub const CLASS_A: Domain = Domain {
        class_id: 6,
        priority: 3,
        vlan_id: 2,
    };

    /// Class B on its default priority and VLAN.
    pub const CLASS_B: Domain = Domain {
        class_id: 5,
        priority: 2,
        vlan_id: 2,
    };

    pub fn value(&self) -> Vec<u8> {
        let vlan = self.vlan_id.to_be_bytes();
        Vec::from([self.class_id, self.priority, vlan[0], vlan[1]])
    }

    pub fn decode(value: &[u8]) -> Option<Self> {
        let [class_id, priority, high, low, ..] = *value else {
            return None;
        };
        Some(Domain {
            class_id,
            priority,
            vlan_id: u16::from_be_bytes([high, low]) & 0x0fff,
        })
    }
}

/// The value of a Talker Advertise for `declaration`, or of a Talker
/// Failed when it says why.
pub fn talker_value(declaration: &TalkerDeclaration) -> Vec<u8> {
    let mut value = Vec::with_capacity(34);
    value.extend_from_slice(&declaration.stream_id.to_be_bytes());
    value.extend_from_slice(&declaration.destination.0);
    value.extend_from_slice(&declaration.vlan_id.to_be_bytes());
    value.extend_from_slice(&declaration.max_frame_size.to_be_bytes());
    value.extend_from_slice(&declaration.max_interval_frames.to_be_bytes());
    value.push((declaration.priority << 5) | if declaration.rank { 0x10 } else { 0 });
    value.extend_from_slice(&declaration.accumulated_latency.to_be_bytes());
    if let Some(failure) = declaration.failure {
        value.extend_from_slice(&failure.bridge_id.to_be_bytes());
        value.push(failure.code);
    }
    value
}

/// The talker declaration a Talker Advertise or Talker Failed value
/// holds.
pub fn decode_talker(attribute_type: u8, value: &[u8], event: Event) -> Option<TalkerDeclaration> {
    declaration(value, attribute_type == attribute::TALKER_FAILED, 0, event)
}

/// Why a talker's reservation failed, and where (35.2.2.8.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TalkerFailure {
    pub bridge_id: u64,
    pub code: u8,
}

/// One stream a talker declares: Talker Advertise, or Talker Failed when
/// `failure` says why (35.2.2.8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TalkerDeclaration {
    pub stream_id: u64,
    pub destination: MacAddress,
    pub vlan_id: u16,
    /// The largest frame, in octets of its payload.
    pub max_frame_size: u16,
    /// The most frames in each class measurement interval.
    pub max_interval_frames: u16,
    pub priority: u8,
    /// Rank 1, a stream that is not an emergency one.
    pub rank: bool,
    /// Nanoseconds.
    pub accumulated_latency: u32,
    pub failure: Option<TalkerFailure>,
    pub event: Event,
}

/// Octets each frame takes on the wire beyond its payload: preamble and
/// start of frame, header with a VLAN tag, frame check and interframe gap.
const FRAME_OVERHEAD: u64 = 8 + 18 + 4 + 12;

impl TalkerDeclaration {
    /// The bandwidth the stream takes in bits per second, at
    /// `intervals_per_second` class measurement intervals: 8000 for class
    /// A, 4000 for class B.
    pub fn bandwidth(&self, intervals_per_second: u32) -> u64 {
        (u64::from(self.max_frame_size) + FRAME_OVERHEAD)
            * 8
            * u64::from(self.max_interval_frames)
            * u64::from(intervals_per_second)
    }
}

/// The talker declarations in an MSRP message, from its AttributeType on.
pub fn talker_declarations(message: &[u8]) -> TalkerDeclarations<'_> {
    let (attribute_type, length, list) = match message {
        [attribute_type, length, list_length @ ..] if list_length.len() >= 2 => {
            let declared = usize::from(u16::from_be_bytes([list_length[0], list_length[1]]));
            let list = &list_length[2..];
            (
                *attribute_type,
                usize::from(*length),
                &list[..declared.min(list.len())],
            )
        }
        _ => (0, 0, &[][..]),
    };
    let talker = matches!(
        attribute_type,
        attribute::TALKER_ADVERTISE | attribute::TALKER_FAILED
    );
    TalkerDeclarations {
        failed: attribute_type == attribute::TALKER_FAILED,
        length,
        list: if talker { list } else { &[] },
        vector: None,
    }
}

/// An iterator over the talker declarations of an MSRP message.
pub struct TalkerDeclarations<'a> {
    failed: bool,
    /// The octets of a first value.
    length: usize,
    /// The vector attributes still to read.
    list: &'a [u8],
    /// The vector being read: its first value, its packed events, how many
    /// values it has and the next one.
    vector: Option<(&'a [u8], &'a [u8], usize, usize)>,
}

impl Iterator for TalkerDeclarations<'_> {
    type Item = TalkerDeclaration;

    fn next(&mut self) -> Option<TalkerDeclaration> {
        loop {
            if let Some((first, events, count, place)) = self.vector {
                if place < count {
                    self.vector = Some((first, events, count, place + 1));
                    let event = events
                        .get(place / 3)
                        .and_then(|&packed| {
                            let shift = [36, 6, 1][place % 3];
                            Event::from_number(packed / shift % 6)
                        })
                        .unwrap_or(Event::Mt);
                    return declaration(first, self.failed, place as u64, event);
                }
                self.vector = None;
            }
            // A vector attribute: its header, its first value, then its
            // events packed three to an octet; an EndMark ends the list.
            let header = self.list.get(..2)?;
            let count = usize::from(u16::from_be_bytes([header[0], header[1]]) & 0x1fff);
            if count == 0 {
                self.list = &[];
                return None;
            }
            let events = count.div_ceil(3);
            let first = self.list.get(2..2 + self.length)?;
            let packed = self.list.get(2 + self.length..2 + self.length + events)?;
            self.list = &self.list[2 + self.length + events..];
            self.vector = Some((first, packed, count, 0));
        }
    }
}

/// The `place`th value of a vector whose first value is `first`: its
/// stream ID and destination counted on from the first's.
fn declaration(first: &[u8], failed: bool, place: u64, event: Event) -> Option<TalkerDeclaration> {
    let needed = if failed { 34 } else { 25 };
    if first.len() < needed {
        return None;
    }
    let read_u64 = |at: usize| {
        let mut octets = [0; 8];
        octets.copy_from_slice(&first[at..at + 8]);
        u64::from_be_bytes(octets)
    };
    let mut mac = [0; 8];
    mac[2..].copy_from_slice(&first[8..14]);
    let mac = (u64::from_be_bytes(mac) + place) & 0xffff_ffff_ffff;
    let mut destination = [0; 6];
    destination.copy_from_slice(&mac.to_be_bytes()[2..]);
    let priority_and_rank = first[20];
    Some(TalkerDeclaration {
        stream_id: read_u64(0).wrapping_add(place),
        destination: MacAddress(destination),
        vlan_id: u16::from_be_bytes([first[14], first[15]]) & 0x0fff,
        max_frame_size: u16::from_be_bytes([first[16], first[17]]),
        max_interval_frames: u16::from_be_bytes([first[18], first[19]]),
        priority: priority_and_rank >> 5,
        rank: priority_and_rank & 0x10 != 0,
        accumulated_latency: u32::from_be_bytes([first[21], first[22], first[23], first[24]]),
        failure: failed.then(|| TalkerFailure {
            bridge_id: read_u64(25),
            code: first[33],
        }),
        event,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A Talker Advertise message for two streams, the first joining and
    /// the second leaving, as a CVU SRP talker message carries it.
    fn advertise() -> Vec<u8> {
        let mut message = vec![attribute::TALKER_ADVERTISE, 25, 0, 0];
        message.extend_from_slice(&[0x00, 0x02]); // two values
        message.extend_from_slice(&0xe8f6_0ae0_9220_0000u64.to_be_bytes());
        message.extend_from_slice(&[0x91, 0xe0, 0xf0, 0x00, 0x6a, 0x20]);
        message.extend_from_slice(&2u16.to_be_bytes()); // VLAN
        message.extend_from_slice(&224u16.to_be_bytes()); // frame size
        message.extend_from_slice(&1u16.to_be_bytes()); // frames per interval
        message.push(0xb0); // priority 5, rank 1
        message.extend_from_slice(&500_000u32.to_be_bytes());
        message.push(36 + 5 * 6); // JoinIn, Lv, New
        message.extend_from_slice(&[0, 0]); // EndMark
        let list = (message.len() - 4) as u16;
        message[2..4].copy_from_slice(&list.to_be_bytes());
        message
    }

    #[test]
    fn talker_advertisements_decode_value_by_value() {
        let declarations: Vec<TalkerDeclaration> = talker_declarations(&advertise()).collect();
        assert_eq!(declarations.len(), 2);
        let first = declarations[0];
        assert_eq!(first.stream_id, 0xe8f6_0ae0_9220_0000);
        assert_eq!(
            first.destination,
            MacAddress([0x91, 0xe0, 0xf0, 0x00, 0x6a, 0x20])
        );
        assert_eq!((first.vlan_id, first.priority, first.rank), (2, 5, true));
        assert_eq!(first.accumulated_latency, 500_000);
        assert_eq!(first.event, Event::JoinIn);
        assert!(first.event.declares() && first.failure.is_none());
        let second = declarations[1];
        assert_eq!(second.stream_id, 0xe8f6_0ae0_9220_0001);
        assert_eq!(second.destination.0[5], 0x21);
        assert_eq!(second.event, Event::Lv);
        assert!(!second.event.declares());
        // 224 octets of payload, one frame each 125 us.
        assert_eq!(first.bandwidth(8000), (224 + 42) * 8 * 8000);
    }

    #[test]
    fn other_attributes_and_short_messages_hold_no_talkers() {
        let mut listener = advertise();
        listener[0] = attribute::LISTENER;
        assert_eq!(talker_declarations(&listener).count(), 0);
        assert_eq!(talker_declarations(&advertise()[..20]).count(), 0);
        assert_eq!(talker_declarations(&[]).count(), 0);
    }
}
