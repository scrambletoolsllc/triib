//! This Mac's own AVB entities, through Apple's AudioVideoBridging
//! framework. macOS never shows its AVB stack the frames a program writes
//! to BPF, so commands to an entity advertised from this Mac go through
//! the framework instead, and its answers come back as the frames the
//! entity would have sent.
//!
//! The framework answers AECP commands without putting anything on the
//! wire and sends ACMP ones on the wire as well, where BPF sees the
//! copies; those carry the framework's own sequence IDs, so the
//! controller takes them for another controller's traffic. Unsolicited
//! notifications reach only a handler registered for the controller.

#![allow(unsafe_code)]

use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};

use avb_net::MacAddress;
use block2::RcBlock;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::{AnyClass, AnyObject, Bool, NSObject};
use objc2::{AnyThread, DefinedClass, define_class, msg_send, sel};
use objc2_foundation::{NSData, NSError, NSString};

use crate::acmp::{AcmpFlags, AcmpMessageType, AcmpStatus, Acmpdu};
use crate::aecp::{AecpHeader, AecpMessageType, AemPdu, VendorUniquePdu};
use crate::id::{EntityId, StreamId};
use crate::pdu::{self, Pdu};

#[link(name = "AudioVideoBridging", kind = "framework")]
unsafe extern "C" {}

/// How long a vendor unique command waits for its answer, in
/// milliseconds; the controller retries on its own.
const VENDOR_UNIQUE_TIMEOUT: i64 = 250;

/// Room for any frame.
const FRAME: usize = 1536;

/// Whether AVB is on for interface `name`, as Audio MIDI Setup's Network
/// Device Browser turns it on and off; `None` where the framework is
/// missing.
pub(super) fn avb_enabled(name: &str) -> Option<bool> {
    let base = AnyClass::get(c"AVBInterface")?;
    let ns_name = NSString::from_str(name);
    // SAFETY: the selector and its types are AVBInterface's, as the
    // framework's headers declare them.
    let enabled: Bool = unsafe { msg_send![base, isAVBEnabledOnInterfaceNamed: &*ns_name] };
    Some(enabled.as_bool())
}

/// The framework's view of one interface, sending commands to this Mac's
/// own entities and turning their answers into frames.
pub(super) struct Local {
    _interface: Retained<AnyObject>,
    aecp: Retained<AnyObject>,
    acmp: Retained<AnyObject>,
    /// The interface's own address, where this Mac's entities are.
    mac: Retained<AnyObject>,
    controller: EntityId,
    sender: Sender<Vec<u8>>,
    answers: Receiver<Vec<u8>>,
    /// Commands sent and not yet answered or timed out.
    waiting: Arc<AtomicUsize>,
    /// The entities commands went to, whose unsolicited notifications are
    /// passed on.
    entities: Arc<Mutex<HashSet<EntityId>>>,
    handler: Option<Retained<Responses>>,
    /// The framework sends vendor unique commands (macOS 14 and later).
    vendor_unique: bool,
}

impl Local {
    /// The framework on interface `name`, for controller `controller`, or
    /// `None` where it is missing or AVB is off on the interface.
    pub(super) fn open(name: &str, controller: EntityId) -> Option<Self> {
        let base = AnyClass::get(c"AVBInterface")?;
        let ethernet = AnyClass::get(c"AVBEthernetInterface")?;
        let ns_name = NSString::from_str(name);
        // SAFETY: the selectors and their types are AVBInterface's, as the
        // framework's headers declare them.
        unsafe {
            let enabled: Bool = msg_send![base, isAVBEnabledOnInterfaceNamed: &*ns_name];
            if !enabled.as_bool() {
                return None;
            }
            let allocated: Allocated<AnyObject> = msg_send![ethernet, alloc];
            let interface: Option<Retained<AnyObject>> =
                msg_send![allocated, initWithInterfaceName: &*ns_name];
            let interface = interface?;
            let aecp: Option<Retained<AnyObject>> = msg_send![&*interface, aecp];
            let acmp: Option<Retained<AnyObject>> = msg_send![&*interface, acmp];
            let mac: Option<Retained<AnyObject>> =
                msg_send![base, macAddressForInterfaceNamed: &*ns_name];
            let (aecp, acmp, mac) = (aecp?, acmp?, mac?);
            let vendor_unique: Bool = msg_send![
                &*aecp,
                respondsToSelector: sel!(sendVendorUniqueCommand:toMACAddress:expectResponseWithinTimeout:completionHandler:)
            ];
            let (sender, answers) = mpsc::channel();
            let entities = Arc::new(Mutex::new(HashSet::new()));
            let handler = Responses::new(sender.clone(), entities.clone());
            let registered: Bool = msg_send![
                &*aecp,
                setResponseHandler: &*handler,
                forControllerEntityID: controller.0
            ];
            Some(Self {
                _interface: interface,
                aecp,
                acmp,
                mac,
                controller,
                sender,
                answers,
                waiting: Arc::new(AtomicUsize::new(0)),
                entities,
                handler: registered.as_bool().then_some(handler),
                vendor_unique: vendor_unique.as_bool(),
            })
        }
    }

    /// Sends `bytes` through the framework when they are a command to an
    /// entity `is_local` says is on this Mac, returning whether it did.
    pub(super) fn send(&self, bytes: &[u8], is_local: impl Fn(EntityId) -> bool) -> bool {
        let (target, sent) = match pdu::decode(bytes) {
            Ok(Pdu::Aem(aem))
                if aem.header.message_type == AecpMessageType::AEM_COMMAND
                    && is_local(aem.header.target_entity_id) =>
            {
                (aem.header.target_entity_id, self.send_aem(&aem))
            }
            Ok(Pdu::VendorUnique(command))
                if self.vendor_unique
                    && command.header.message_type == AecpMessageType::VENDOR_UNIQUE_COMMAND
                    && is_local(command.header.target_entity_id) =>
            {
                (
                    command.header.target_entity_id,
                    self.send_vendor_unique(&command),
                )
            }
            Ok(Pdu::Acmp(command))
                if command.message_type.is_command() && is_local(addressed(&command)) =>
            {
                (addressed(&command), self.send_acmp(&command))
            }
            _ => return false,
        };
        if sent && let Ok(mut entities) = self.entities.lock() {
            entities.insert(target);
        }
        sent
    }

    /// Whether commands wait for their answers.
    pub(super) fn waiting(&self) -> bool {
        self.waiting.load(Ordering::Relaxed) > 0
    }

    /// The next answer, as the frame's octets after the Ethernet header.
    pub(super) fn poll(&self) -> Option<Vec<u8>> {
        self.answers.try_recv().ok()
    }

    fn send_aem(&self, command: &AemPdu<'_>) -> bool {
        let Some(class) = AnyClass::get(c"AVB17221AECPAEMMessage") else {
            return false;
        };
        let header = command.header;
        let command_type = command.command_type;
        let answers = self.sender.clone();
        let waiting = self.waiting.clone();
        let completion = RcBlock::new(move |error: *mut NSError, response: *mut AnyObject| {
            waiting.fetch_sub(1, Ordering::Relaxed);
            // SAFETY: the framework hands over a valid response or none.
            let Some(response) = (unsafe { answered(error, response, 1) }) else {
                return;
            };
            // SAFETY: an AVB17221AECPAEMMessage's properties.
            let (status, payload) = unsafe {
                let status: u8 = msg_send![response, status];
                let data: Option<Retained<NSData>> = msg_send![response, commandSpecificData];
                (status, data.map(|data| data.to_vec()).unwrap_or_default())
            };
            let answer = AemPdu {
                header: AecpHeader {
                    message_type: AecpMessageType::AEM_RESPONSE,
                    status,
                    ..header
                },
                unsolicited: false,
                controller_request: false,
                command_type,
                payload: &payload,
            };
            let mut out = [0; FRAME];
            if let Ok(length) = answer.encode(&mut out) {
                let _ = answers.send(out[..length].to_vec());
            }
        });
        self.waiting.fetch_add(1, Ordering::Relaxed);
        // SAFETY: the selectors and their types are the framework's; it
        // copies the block.
        let sent: Bool = unsafe {
            let message: Retained<AnyObject> = msg_send![class, commandMessage];
            let _: () = msg_send![&*message, setMessageType: AecpMessageType::AEM_COMMAND.0];
            let _: () = msg_send![&*message, setCommandType: command_type.0];
            let _: () = msg_send![&*message, setTargetEntityID: header.target_entity_id.0];
            let _: () = msg_send![&*message, setControllerEntityID: header.controller_entity_id.0];
            let _: () = msg_send![&*message, setSequenceID: header.sequence_id];
            let _: () = msg_send![&*message, setUnsolicited: Bool::new(command.unsolicited)];
            let data = NSData::with_bytes(command.payload);
            let _: () = msg_send![&*message, setCommandSpecificData: &*data];
            msg_send![
                &*self.aecp,
                sendCommand: &*message,
                toMACAddress: &*self.mac,
                completionHandler: &*completion
            ]
        };
        self.sent(sent)
    }

    fn send_vendor_unique(&self, command: &VendorUniquePdu<'_>) -> bool {
        let Some(class) = AnyClass::get(c"AVB17221AECPVendorMessage") else {
            return false;
        };
        let header = command.header;
        let protocol_id = command.protocol_id;
        let answers = self.sender.clone();
        let waiting = self.waiting.clone();
        let completion = RcBlock::new(move |error: *mut NSError, response: *mut AnyObject| {
            waiting.fetch_sub(1, Ordering::Relaxed);
            // SAFETY: the framework hands over a valid response or none.
            let Some(response) = (unsafe { answered(error, response, 7) }) else {
                return;
            };
            // SAFETY: an AVB17221AECPVendorMessage's properties.
            let (status, payload) = unsafe {
                let status: u8 = msg_send![response, status];
                let data: Option<Retained<NSData>> = msg_send![response, protocolSpecificData];
                (status, data.map(|data| data.to_vec()).unwrap_or_default())
            };
            let answer = VendorUniquePdu {
                header: AecpHeader {
                    message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
                    status,
                    ..header
                },
                protocol_id,
                payload: &payload,
            };
            let mut out = [0; FRAME];
            if let Ok(length) = answer.encode(&mut out) {
                let _ = answers.send(out[..length].to_vec());
            }
        });
        let mut id = [0; 8];
        id[2..].copy_from_slice(&protocol_id);
        self.waiting.fetch_add(1, Ordering::Relaxed);
        // SAFETY: as in send_aem; the class has a plain init.
        let sent: Bool = unsafe {
            let allocated: Allocated<AnyObject> = msg_send![class, alloc];
            let message: Retained<AnyObject> = msg_send![allocated, init];
            let _: () = msg_send![
                &*message,
                setMessageType: AecpMessageType::VENDOR_UNIQUE_COMMAND.0
            ];
            let _: () = msg_send![&*message, setTargetEntityID: header.target_entity_id.0];
            let _: () = msg_send![&*message, setControllerEntityID: header.controller_entity_id.0];
            let _: () = msg_send![&*message, setSequenceID: header.sequence_id];
            let _: () = msg_send![&*message, setProtocolID: u64::from_be_bytes(id)];
            let data = NSData::with_bytes(command.payload);
            let _: () = msg_send![&*message, setProtocolSpecificData: &*data];
            msg_send![
                &*self.aecp,
                sendVendorUniqueCommand: &*message,
                toMACAddress: &*self.mac,
                expectResponseWithinTimeout: VENDOR_UNIQUE_TIMEOUT,
                completionHandler: &*completion
            ]
        };
        self.sent(sent)
    }

    fn send_acmp(&self, command: &Acmpdu) -> bool {
        let (Some(class), Some(mac_class)) = (
            AnyClass::get(c"AVB17221ACMPMessage"),
            AnyClass::get(c"AVBMACAddress"),
        ) else {
            return false;
        };
        let sent_command = *command;
        let answers = self.sender.clone();
        let waiting = self.waiting.clone();
        let expected = command.message_type.response().0;
        let completion = RcBlock::new(move |error: *mut NSError, response: *mut AnyObject| {
            waiting.fetch_sub(1, Ordering::Relaxed);
            // SAFETY: the framework hands over a valid response or none.
            let Some(response) = (unsafe { answered(error, response, expected) }) else {
                return;
            };
            // SAFETY: an AVB17221ACMPMessage's properties.
            let answer = unsafe {
                let destination: Option<Retained<AnyObject>> = msg_send![response, destinationMAC];
                let destination = destination
                    .and_then(|mac| {
                        let data: Option<Retained<NSData>> = msg_send![&*mac, dataRepresentation];
                        data
                    })
                    .and_then(|data| <[u8; 6]>::try_from(data.to_vec().as_slice()).ok())
                    .map_or(sent_command.stream_dest_mac, MacAddress);
                let message_type: u8 = msg_send![response, messageType];
                let status: u8 = msg_send![response, status];
                let stream_id: u64 = msg_send![response, streamID];
                let talker: u64 = msg_send![response, talkerEntityID];
                let listener: u64 = msg_send![response, listenerEntityID];
                let talker_unique_id: u16 = msg_send![response, talkerUniqueID];
                let listener_unique_id: u16 = msg_send![response, listenerUniqueID];
                let connection_count: u16 = msg_send![response, connectionCount];
                let flags: u16 = msg_send![response, flags];
                let vlan: u16 = msg_send![response, vlanID];
                Acmpdu {
                    message_type: AcmpMessageType(message_type),
                    status: AcmpStatus(status),
                    stream_id: StreamId(stream_id),
                    talker_entity_id: EntityId(talker),
                    listener_entity_id: EntityId(listener),
                    talker_unique_id,
                    listener_unique_id,
                    stream_dest_mac: destination,
                    connection_count,
                    flags: AcmpFlags(flags),
                    stream_vlan_id: vlan,
                    ..sent_command
                }
            };
            let mut out = [0; FRAME];
            if let Ok(length) = answer.encode(&mut out) {
                let _ = answers.send(out[..length].to_vec());
            }
        });
        self.waiting.fetch_add(1, Ordering::Relaxed);
        // SAFETY: as in send_aem; the message class has a plain init, and
        // the address is six octets.
        let sent: Bool = unsafe {
            let allocated: Allocated<AnyObject> = msg_send![class, alloc];
            let message: Retained<AnyObject> = msg_send![allocated, init];
            let allocated: Allocated<AnyObject> = msg_send![mac_class, alloc];
            let destination: Retained<AnyObject> =
                msg_send![allocated, initWithBytes: command.stream_dest_mac.0.as_ptr()];
            let _: () = msg_send![&*message, setMessageType: command.message_type.0];
            let _: () = msg_send![&*message, setStreamID: command.stream_id.0];
            let _: () = msg_send![&*message, setControllerEntityID: command.controller_entity_id.0];
            let _: () = msg_send![&*message, setTalkerEntityID: command.talker_entity_id.0];
            let _: () = msg_send![&*message, setListenerEntityID: command.listener_entity_id.0];
            let _: () = msg_send![&*message, setTalkerUniqueID: command.talker_unique_id];
            let _: () = msg_send![&*message, setListenerUniqueID: command.listener_unique_id];
            let _: () = msg_send![&*message, setDestinationMAC: &*destination];
            let _: () = msg_send![&*message, setConnectionCount: command.connection_count];
            let _: () = msg_send![&*message, setSequenceID: command.sequence_id];
            let _: () = msg_send![&*message, setFlags: command.flags.0];
            let _: () = msg_send![&*message, setVlanID: command.stream_vlan_id];
            msg_send![
                &*self.acmp,
                sendACMPCommandMessage: &*message,
                completionHandler: &*completion
            ]
        };
        self.sent(sent)
    }

    /// Whether the framework took a command; one it did not take will not
    /// be answered.
    fn sent(&self, sent: Bool) -> bool {
        if !sent.as_bool() {
            self.waiting.fetch_sub(1, Ordering::Relaxed);
        }
        sent.as_bool()
    }
}

impl Drop for Local {
    fn drop(&mut self) {
        if self.handler.is_some() {
            // SAFETY: AVB17221AECPInterface's method.
            unsafe {
                let _: () = msg_send![
                    &*self.aecp,
                    removeResponseHandlerForControllerEntityID: self.controller.0
                ];
            }
        }
    }
}

/// The entity an ACMP command goes to: TX commands to the talker, RX ones
/// to the listener.
fn addressed(command: &Acmpdu) -> EntityId {
    match command.message_type {
        AcmpMessageType::CONNECT_TX_COMMAND
        | AcmpMessageType::DISCONNECT_TX_COMMAND
        | AcmpMessageType::GET_TX_STATE_COMMAND
        | AcmpMessageType::GET_TX_CONNECTION_COMMAND => command.talker_entity_id,
        _ => command.listener_entity_id,
    }
}

/// The response a completion handler got, when it is one: the framework
/// passes an error of code 0 along with answers, and on a timeout a
/// nonzero one, perhaps with the command itself.
///
/// # Safety
///
/// `error` and `response` are null or valid objects.
unsafe fn answered<'a>(
    error: *mut NSError,
    response: *mut AnyObject,
    message_type: u8,
) -> Option<&'a AnyObject> {
    // SAFETY: the caller's.
    let (error, response) = unsafe { (error.as_ref(), response.as_ref()?) };
    if error.is_some_and(|error| error.code() != 0) {
        return None;
    }
    // SAFETY: every AVB message has a messageType.
    let found: u8 = unsafe { msg_send![response, messageType] };
    (found == message_type).then_some(response)
}

struct Ivars {
    answers: Sender<Vec<u8>>,
    entities: Arc<Mutex<HashSet<EntityId>>>,
}

define_class!(
    // SAFETY: NSObject has no subclassing requirements, and Responses
    // does not implement Drop.
    #[unsafe(super(NSObject))]
    #[name = "AtdeccLocalEntityResponses"]
    #[ivars = Ivars]
    struct Responses;

    /// The AVB17221AECPClient methods the framework calls with the
    /// responses for the controller.
    impl Responses {
        #[unsafe(method(AECPDidReceiveCommand:onInterface:))]
        fn received_command(&self, _message: *mut AnyObject, _interface: *mut AnyObject) -> Bool {
            Bool::NO
        }

        #[unsafe(method(AECPDidReceiveResponse:onInterface:))]
        fn received_response(&self, message: *mut AnyObject, _interface: *mut AnyObject) -> Bool {
            // SAFETY: the framework passes a valid message.
            if let Some(message) = unsafe { message.as_ref() } {
                self.unsolicited(message);
            }
            Bool::YES
        }
    }
);

impl Responses {
    fn new(answers: Sender<Vec<u8>>, entities: Arc<Mutex<HashSet<EntityId>>>) -> Retained<Self> {
        let this = Self::alloc().set_ivars(Ivars { answers, entities });
        // SAFETY: NSObject's init.
        unsafe { msg_send![super(this), init] }
    }

    /// Passes on an unsolicited notification from one of this Mac's
    /// entities; solicited answers come to their command's handler.
    fn unsolicited(&self, message: &AnyObject) {
        let Some(class) = AnyClass::get(c"AVB17221AECPAEMMessage") else {
            return;
        };
        // SAFETY: NSObject's isKindOfClass:, then an
        // AVB17221AECPAEMMessage's properties.
        unsafe {
            let aem: Bool = msg_send![message, isKindOfClass: class];
            if !aem.as_bool() {
                return;
            }
            let unsolicited: Bool = msg_send![message, isUnsolicited];
            let target: u64 = msg_send![message, targetEntityID];
            let known = self
                .ivars()
                .entities
                .lock()
                .is_ok_and(|entities| entities.contains(&EntityId(target)));
            if !unsolicited.as_bool() || !known {
                return;
            }
            let status: u8 = msg_send![message, status];
            let controller: u64 = msg_send![message, controllerEntityID];
            let sequence_id: u16 = msg_send![message, sequenceID];
            let command_type: u16 = msg_send![message, commandType];
            let data: Option<Retained<NSData>> = msg_send![message, commandSpecificData];
            let payload = data.map(|data| data.to_vec()).unwrap_or_default();
            let answer = AemPdu {
                header: AecpHeader {
                    message_type: AecpMessageType::AEM_RESPONSE,
                    status,
                    target_entity_id: EntityId(target),
                    controller_entity_id: EntityId(controller),
                    sequence_id,
                },
                unsolicited: true,
                controller_request: false,
                command_type: crate::aecp::AemCommandType(command_type),
                payload: &payload,
            };
            let mut out = [0; FRAME];
            if let Ok(length) = answer.encode(&mut out) {
                let _ = self.ivars().answers.send(out[..length].to_vec());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_framework_links_and_its_classes_are_found() {
        for name in [
            c"AVBInterface",
            c"AVBEthernetInterface",
            c"AVBMACAddress",
            c"AVB17221AECPAEMMessage",
            c"AVB17221AECPVendorMessage",
            c"AVB17221ACMPMessage",
        ] {
            assert!(AnyClass::get(name).is_some(), "{name:?}");
        }
        assert!(Local::open("no-such-interface", EntityId(1)).is_none());
    }

    #[test]
    fn acmp_commands_go_to_the_talker_or_the_listener() {
        let command = |message_type| Acmpdu {
            message_type,
            talker_entity_id: EntityId(1),
            listener_entity_id: EntityId(2),
            ..Acmpdu::default()
        };
        assert_eq!(
            addressed(&command(AcmpMessageType::GET_TX_STATE_COMMAND)),
            EntityId(1)
        );
        assert_eq!(
            addressed(&command(AcmpMessageType::DISCONNECT_TX_COMMAND)),
            EntityId(1)
        );
        assert_eq!(
            addressed(&command(AcmpMessageType::CONNECT_RX_COMMAND)),
            EntityId(2)
        );
        assert_eq!(
            addressed(&command(AcmpMessageType::GET_RX_STATE_COMMAND)),
            EntityId(2)
        );
    }
}
