//! AVB Lite (the AVB Lite profile): the CVU SRP messages that carry
//! MSRP declarations, as AECP vendor unique commands, once gptpd has
//! fallen back to AVB Lite; and spotting PTP frames that a driver left its
//! receive time stamp on, which gptpd cannot use.

use atdecc::aecp::{AecpHeader, AecpMessageType, VendorUniquePdu};
use atdecc::lite::{CVU_PROTOCOL_ID, CvuMessage};
use atdecc::{EntityId, MacAddress};
use avb_mrp::msrp::{self, ListenerState, TalkerDeclaration};
use avb_mrp::{Event, mrpdu};

/// PTP's ethertype, and where gPTP's peer delay messages go.
pub const PTP_ETHERTYPE: u16 = 0x88f7;
pub const PDELAY_DESTINATION: MacAddress = MacAddress([0x01, 0x80, 0xc2, 0x00, 0x00, 0x0e]);
/// Where the AVB Lite PTP profile's messages go.
pub const PTP_DESTINATION: MacAddress = MacAddress([0x01, 0x1b, 0x19, 0x00, 0x00, 0x00]);

/// The octets a driver's receive time stamp takes at the end of a frame,
/// as atlantic's does.
const STAMP_LEN: usize = 12;

/// Whether a PTP message heard on the interface, after the ethertype,
/// runs a time stamp's length past its messageLength: the driver left its
/// receive time stamp on the frame, which Ethernet's padding, at most 2
/// octets for the shortest PTP message, cannot explain, and which a PTP
/// daemon takes for a broken TLV.
pub fn stamped(message: &[u8]) -> bool {
    message.len() >= 4
        && message.len() >= usize::from(u16::from_be_bytes([message[2], message[3]])) + STAMP_LEN
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
