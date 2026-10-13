//! Controllers that do not advertise, known from the commands they send:
//! their connection commands, which go to every station, and their AECP
//! commands to this computer's entities.

use std::collections::BTreeMap;
use std::time::{Duration, SystemTime};

use atdecc::adp::{Adpdu, ControllerCapabilities};
use atdecc::blocking::Frame;
use atdecc::controller::DiscoveredEntity;
use atdecc::pdu::{self, Pdu};
use atdecc::{AcmpMessageType, AecpHeader, EntityId, Instant};

/// How long a controller stays listed after its last command heard: one
/// that only reads now and then would come and go if it were the minute
/// an advertisement lasts.
pub const KEPT: Duration = Duration::from_secs(5 * 60);

/// A controller heard, as an advertisement would show it: a controller of
/// no model, from the address its commands came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Heard {
    pub entity: DiscoveredEntity,
    /// When its last command was heard.
    pub at: SystemTime,
}

/// The controller that sent `frame`, by what only a controller sends:
/// ACMP commands to a listener or about a talker, and AECP commands.
pub fn sender(frame: &Frame) -> Option<EntityId> {
    if frame.sent {
        return None;
    }
    let controller = match pdu::decode(&frame.bytes).ok()? {
        Pdu::Acmp(acmpdu) => matches!(
            acmpdu.message_type,
            AcmpMessageType::CONNECT_RX_COMMAND
                | AcmpMessageType::DISCONNECT_RX_COMMAND
                | AcmpMessageType::GET_RX_STATE_COMMAND
                | AcmpMessageType::GET_TX_STATE_COMMAND
                | AcmpMessageType::GET_TX_CONNECTION_COMMAND
        )
        .then_some(acmpdu.controller_entity_id)?,
        Pdu::Aem(_) | Pdu::VendorUnique(_) | Pdu::Aecp(..) => {
            let (header, _) = AecpHeader::decode(&frame.bytes).ok()?;
            (!header.message_type.is_response()).then_some(header.controller_entity_id)?
        }
        Pdu::Adp(_) | Pdu::Other { .. } => return None,
    };
    (controller.0 != 0).then_some(controller)
}

/// Notes the controllers `frames` show, but those `known` (triib's own,
/// and those that advertise), and forgets those not heard for [`KEPT`]
/// before the newest frame.
pub fn note(
    heard: &mut BTreeMap<EntityId, Heard>,
    frames: &[Frame],
    known: impl Fn(EntityId) -> bool,
) {
    for frame in frames {
        let Some(controller) = sender(frame).filter(|controller| !known(*controller)) else {
            continue;
        };
        heard.insert(
            controller,
            Heard {
                entity: DiscoveredEntity {
                    adp: Adpdu {
                        entity_id: controller,
                        controller_capabilities: ControllerCapabilities::IMPLEMENTED,
                        ..Adpdu::default()
                    },
                    mac: frame.peer,
                    first_seen: Instant::ZERO,
                    last_seen: Instant::ZERO,
                    expires: Instant::ZERO,
                },
                at: frame.at,
            },
        );
    }
    if let Some(newest) = frames.iter().map(|frame| frame.at).max() {
        heard.retain(|_, controller| {
            newest
                .duration_since(controller.at)
                .is_ok_and(|since| since <= KEPT)
                || controller.at > newest
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atdecc::MacAddress;

    const APPLE: EntityId = EntityId(0xd111_e597_f544_0000);
    const MAC: MacAddress = MacAddress([0xd0, 0x11, 0xe5, 0x97, 0xf5, 0x44]);

    fn frame(at: u64, sent: bool, bytes: Vec<u8>) -> Frame {
        Frame {
            at: SystemTime::UNIX_EPOCH + Duration::from_secs(at),
            sent,
            peer: MAC,
            bytes,
        }
    }

    /// An AECP PDU of `message_type` from `controller`, as the Mac's
    /// controller sends LOCK_ENTITY.
    fn aecp(message_type: u8, controller: EntityId) -> Vec<u8> {
        let mut bytes = vec![0xfb, message_type, 0x00, 0x1c];
        bytes.extend(0xf0a7_31ff_04f4_0f14u64.to_be_bytes());
        bytes.extend(controller.0.to_be_bytes());
        bytes.extend([0x6d, 0xf5, 0x00, 0x01]);
        bytes.resize(4 + 28 + 8, 0);
        bytes
    }

    /// An ACMP PDU of `message_type` from `controller`.
    fn acmp(message_type: u8, controller: EntityId) -> Vec<u8> {
        let mut bytes = vec![0xfc, message_type, 0x00, 0x2c];
        bytes.extend(0u64.to_be_bytes());
        bytes.extend(controller.0.to_be_bytes());
        // The stream ID, then the 44 octets control_data_length counts.
        bytes.resize(4 + 8 + 44, 0);
        bytes
    }

    /// Commands name their controller; responses, frames triib sent and
    /// a listener's probe do not count.
    #[test]
    fn controllers_are_known_by_their_commands() {
        assert_eq!(sender(&frame(0, false, aecp(0, APPLE))), Some(APPLE));
        assert_eq!(sender(&frame(0, false, aecp(1, APPLE))), None);
        assert_eq!(sender(&frame(0, true, aecp(0, APPLE))), None);
        assert_eq!(sender(&frame(0, false, aecp(0, EntityId(0)))), None);
        assert_eq!(sender(&frame(0, false, acmp(6, APPLE))), Some(APPLE));
        assert_eq!(sender(&frame(0, false, acmp(7, APPLE))), None);
        // PROBE_TX_COMMAND, which a listener sends.
        assert_eq!(sender(&frame(0, false, acmp(0, APPLE))), None);
    }

    /// A controller is listed from its address until [`KEPT`] passes
    /// without a command, and never if it is known.
    #[test]
    fn controllers_heard_are_kept_for_a_while() {
        let mut heard = BTreeMap::new();
        note(&mut heard, &[frame(10, false, aecp(0, APPLE))], |_| false);
        let entity = heard[&APPLE].entity;
        assert_eq!(entity.mac, MAC);
        assert!(crate::describe::is_controller(&entity.adp));
        note(
            &mut heard,
            &[frame(10 + 299, false, aecp(1, APPLE))],
            |_| false,
        );
        assert!(heard.contains_key(&APPLE));
        note(
            &mut heard,
            &[frame(10 + 301, false, aecp(1, APPLE))],
            |_| false,
        );
        assert!(heard.is_empty());
        note(&mut heard, &[frame(400, false, aecp(0, APPLE))], |id| {
            id == APPLE
        });
        assert!(heard.is_empty());
    }
}
