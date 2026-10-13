use alloc::vec;
use std::collections::BTreeMap as Map;

use super::*;
use crate::acmp::{AcmpFlags, AcmpMessageType, AcmpStatus, Acmpdu};
use crate::aecp::AecpHeader;
use crate::aem::{AudioMapping, StreamInfoFlags};
use crate::avtp::read_u16;
use crate::cache::CachedModel;
use crate::descriptor::SamplingRate;
use crate::id::ClockIdentity;
use crate::mvu::MVU_PROTOCOL_ID;
use crate::stream_format::StreamFormat;

const CONTROLLER: EntityId = EntityId(0x9c6b_00ff_fe30_9a2b);
const TALKER: EntityId = EntityId(0xe8f6_0ae0_9220_0000);
const TALKER_MAC: MacAddress = MacAddress([0xe8, 0xf6, 0x0a, 0xe0, 0x92, 0x20]);
const GRANDMASTER: ClockIdentity = ClockIdentity(0x0001_f2ff_fe00_0001);
const BRIDGE: ClockIdentity = ClockIdentity(0x0001_f2ff_fe3b_1400);

fn controller() -> Controller {
    Controller::new(Config::new(CONTROLLER))
}

fn available(entity_id: EntityId, available_index: u32) -> Adpdu {
    Adpdu {
        message_type: AdpMessageType::ENTITY_AVAILABLE,
        valid_time: 10,
        entity_id,
        available_index,
        ..Adpdu::default()
    }
}

/// An advertisement from an entity with an AEM model and MVU.
fn aem_available(available_index: u32) -> Adpdu {
    Adpdu {
        entity_capabilities: EntityCapabilities::AEM_SUPPORTED
            | EntityCapabilities::VENDOR_UNIQUE_SUPPORTED,
        ..available(TALKER, available_index)
    }
}

fn events(controller: &mut Controller) -> Vec<Event> {
    core::iter::from_fn(|| controller.poll_event()).collect()
}

fn at(seconds: u64) -> Instant {
    Instant::from_millis(seconds * 1000)
}

fn after(instant: Instant, millis: u64) -> Instant {
    instant + Duration::from_millis(millis)
}

/// The frames the controller wants to send now.
fn transmits(controller: &mut Controller) -> Vec<(MacAddress, Vec<u8>)> {
    let mut buffer = [0; 1500];
    core::iter::from_fn(|| {
        controller
            .poll_transmit(&mut buffer)
            .unwrap()
            .map(|transmit| (transmit.destination, buffer[..transmit.length].to_vec()))
    })
    .collect()
}

/// A pretend entity: descriptors by type and index, answering commands as
/// a real one would.
struct FakeEntity {
    descriptors: Map<(u16, u16), Vec<u8>>,
    /// Commands seen, by command type.
    commands: Vec<AemCommandType>,
    /// What each stream input is bound to.
    bindings: Map<u16, (EntityId, u16)>,
    /// The status to answer binds with.
    bind_status: AcmpStatus,
    /// Values set on controls, by index.
    controls: Vec<(u16, Vec<u8>)>,
    /// Answers ACMP commands.
    answers_acmp: bool,
    /// Answers GET_AVB_INFO, GET_AS_PATH and GET_COUNTERS.
    answers_network: bool,
    /// Its streams run, so it refuses to change their formats.
    streaming: bool,
    /// Answers GET_DYNAMIC_INFO.
    answers_dynamic_info: bool,
    /// The dynamic mappings of its stream port input.
    mappings: Vec<AudioMapping>,
    /// How many parts GET_AUDIO_MAP answers in, split by cluster.
    map_parts: u16,
    /// Its clock domains' user media clock reference priority.
    user_priority: u8,
    /// Its stream output's max transit time, in nanoseconds.
    transit: u64,
    /// DISCONNECT_TX commands seen: the output and the listener's input.
    tx_disconnects: Vec<(u16, EntityId, u16)>,
    /// What it reports of AVB Lite, when it has it.
    lite: Option<crate::lite::LiteStatus>,
    /// What it reports of its wireless interface, when it has AVB
    /// Wireless; its other interfaces are wired.
    wireless: Option<Wireless>,
    /// AVB Wireless status queries it leaves unanswered before it answers,
    /// as over a Wi-Fi hop that loses them.
    wireless_silent: u8,
    /// Its stream output sends in class B.
    class_b: bool,
    /// Answers GET_MILAN_INFO, as a Milan entity; without it NOT_IMPLEMENTED.
    milan: bool,
    /// Answers GET_MAX_TRANSIT_TIME and SET_MAX_TRANSIT_TIME; without it
    /// NOT_IMPLEMENTED.
    answers_transit: bool,
}

fn descriptor(descriptor_type: DescriptorType, index: u16, length: usize) -> Vec<u8> {
    let mut bytes = vec![0; length];
    bytes[0..2].copy_from_slice(&descriptor_type.0.to_be_bytes());
    bytes[2..4].copy_from_slice(&index.to_be_bytes());
    bytes
}

fn named(
    descriptor_type: DescriptorType,
    index: u16,
    length: usize,
    name: &str,
    description: u16,
) -> Vec<u8> {
    let mut bytes = descriptor(descriptor_type, index, length);
    bytes[4..4 + name.len()].copy_from_slice(name.as_bytes());
    bytes[68..70].copy_from_slice(&description.to_be_bytes());
    bytes
}

fn put(bytes: &mut [u8], at: usize, value: u16) {
    bytes[at..at + 2].copy_from_slice(&value.to_be_bytes());
}

impl FakeEntity {
    fn new() -> Self {
        let mut descriptors = Map::new();
        let mut add = |bytes: Vec<u8>| {
            let key = (
                u16::from_be_bytes([bytes[0], bytes[1]]),
                u16::from_be_bytes([bytes[2], bytes[3]]),
            );
            descriptors.insert(key, bytes);
        };

        let mut entity = descriptor(DescriptorType::ENTITY, 0, 312);
        entity[4..12].copy_from_slice(&TALKER.to_bytes());
        entity[48..57].copy_from_slice(b"Stage box");
        put(&mut entity, 112, 0); // vendor: strings 0, string 0
        put(&mut entity, 114, 1); // model: strings 0, string 1
        entity[116..121].copy_from_slice(b"2.1.0");
        add(entity);

        // Six top level types, one the controller does not read.
        let mut configuration = named(DescriptorType::CONFIGURATION, 0, 102, "Main", 0xffff);
        put(&mut configuration, 70, 7);
        put(&mut configuration, 72, 74);
        let counts = [
            (DescriptorType::AUDIO_UNIT, 1),
            (DescriptorType::STREAM_INPUT, 2),
            (DescriptorType::STREAM_OUTPUT, 1),
            (DescriptorType::LOCALE, 1),
            (DescriptorType::VIDEO_UNIT, 3),
            (DescriptorType::AVB_INTERFACE, 1),
            (DescriptorType::CLOCK_DOMAIN, 1),
        ];
        for (slot, (descriptor_type, count)) in counts.iter().enumerate() {
            put(&mut configuration, 74 + 4 * slot, descriptor_type.0);
            put(&mut configuration, 76 + 4 * slot, *count);
        }
        add(configuration);

        let mut unit = named(DescriptorType::AUDIO_UNIT, 0, 144, "", 0xffff);
        put(&mut unit, 72, 1); // one stream input port, at 0
        put(&mut unit, 76, 1); // one stream output port, at 0
        put(&mut unit, 140, 144); // no sampling rates
        add(unit);
        // The input's mappings are dynamic, as Milan requires.
        let mut port = descriptor(DescriptorType::STREAM_PORT_INPUT, 0, 20);
        put(&mut port, 12, 2); // two clusters at 0
        add(port);
        // The output's are fixed by one map.
        let mut port = descriptor(DescriptorType::STREAM_PORT_OUTPUT, 0, 20);
        put(&mut port, 12, 1); // one cluster at 2
        put(&mut port, 14, 2);
        put(&mut port, 16, 1); // one map at 0
        add(port);
        for (index, name) in [(0, "Left"), (1, "Right"), (2, "Out")] {
            let mut cluster = named(DescriptorType::AUDIO_CLUSTER, index, 90, name, 0xffff);
            put(&mut cluster, 84, 1); // one channel
            add(cluster);
        }
        let mut map = descriptor(DescriptorType::AUDIO_MAP, 0, 16);
        put(&mut map, 4, 8); // one mapping at 8: cluster 0 to stream 0
        put(&mut map, 6, 1);
        add(map);

        for (index, name, description) in [(0, "", 2), (1, "Mic", 0xffff)] {
            let mut stream = named(DescriptorType::STREAM_INPUT, index, 138, name, description);
            put(&mut stream, 82, 138);
            add(stream);
        }
        let mut output = named(DescriptorType::STREAM_OUTPUT, 0, 138, "Out", 0xffff);
        put(&mut output, 82, 138);
        add(output);

        add(named(DescriptorType::AVB_INTERFACE, 0, 98, "", 0xffff));
        let mut domain = named(DescriptorType::CLOCK_DOMAIN, 0, 80, "Domain", 0xffff);
        put(&mut domain, 72, 76); // sources 0 and 1
        put(&mut domain, 74, 2);
        put(&mut domain, 78, 1);
        add(domain);

        let mut locale = descriptor(DescriptorType::LOCALE, 0, 72);
        locale[4..9].copy_from_slice(b"en-US");
        put(&mut locale, 68, 1);
        add(locale);
        let mut strings = descriptor(DescriptorType::STRINGS, 0, 452);
        for (slot, text) in ["Acme", "Box 1", "Input A"].iter().enumerate() {
            strings[4 + 64 * slot..4 + 64 * slot + text.len()].copy_from_slice(text.as_bytes());
        }
        add(strings);

        Self {
            descriptors,
            commands: Vec::new(),
            bindings: Map::new(),
            bind_status: AcmpStatus::SUCCESS,
            controls: Vec::new(),
            answers_acmp: true,
            answers_network: true,
            streaming: false,
            answers_dynamic_info: true,
            mappings: Vec::new(),
            map_parts: 1,
            user_priority: 192,
            transit: 2_000_000,
            tx_disconnects: Vec::new(),
            lite: None,
            wireless: None,
            wireless_silent: 0,
            class_b: false,
            milan: true,
            answers_transit: true,
        }
    }

    /// Answers GET_AUDIO_MAP, ADD_AUDIO_MAPPINGS and REMOVE_AUDIO_MAPPINGS
    /// for its stream port input. An added mapping replaces one to the
    /// same cluster channel, as Milan allows; the response's payload is
    /// the command's.
    fn audio_map(&mut self, command_type: AemCommandType, payload: &mut Vec<u8>) -> AemStatus {
        let read = |at: usize, payload: &[u8]| u16::from_be_bytes([payload[at], payload[at + 1]]);
        if read(0, payload) != DescriptorType::STREAM_PORT_INPUT.0 || read(2, payload) != 0 {
            return AemStatus::NOT_SUPPORTED;
        }
        let mappings = |payload: &[u8]| {
            let count = usize::from(read(4, payload));
            payload[8..8 + 8 * count]
                .as_chunks::<8>()
                .0
                .iter()
                .map(|bytes| AudioMapping {
                    stream_index: read(0, bytes),
                    stream_channel: read(2, bytes),
                    cluster_offset: read(4, bytes),
                    cluster_channel: read(6, bytes),
                })
                .collect::<Vec<_>>()
        };
        match command_type {
            AemCommandType::GET_AUDIO_MAP => {
                let part = read(4, payload);
                if part >= self.map_parts {
                    return AemStatus::BAD_ARGUMENTS;
                }
                let answer: Vec<&AudioMapping> = self
                    .mappings
                    .iter()
                    .filter(|mapping| mapping.cluster_offset % self.map_parts == part)
                    .collect();
                payload.truncate(6);
                payload.extend_from_slice(&self.map_parts.to_be_bytes());
                payload.extend_from_slice(&(answer.len() as u16).to_be_bytes());
                payload.extend_from_slice(&[0, 0]);
                for mapping in answer {
                    for field in [
                        mapping.stream_index,
                        mapping.stream_channel,
                        mapping.cluster_offset,
                        mapping.cluster_channel,
                    ] {
                        payload.extend_from_slice(&field.to_be_bytes());
                    }
                }
            }
            AemCommandType::ADD_AUDIO_MAPPINGS => {
                let added = mappings(payload);
                let valid = added.iter().all(|mapping| {
                    mapping.stream_index < 2
                        && mapping.cluster_offset < 2
                        && mapping.cluster_channel == 0
                });
                if !valid {
                    return AemStatus::BAD_ARGUMENTS;
                }
                for mapping in added {
                    self.mappings.retain(|known| {
                        (known.cluster_offset, known.cluster_channel)
                            != (mapping.cluster_offset, mapping.cluster_channel)
                    });
                    self.mappings.push(mapping);
                }
            }
            AemCommandType::REMOVE_AUDIO_MAPPINGS => {
                let removed = mappings(payload);
                self.mappings.retain(|known| !removed.contains(known));
            }
            _ => {}
        }
        AemStatus::SUCCESS
    }

    /// The answers to a GET_DYNAMIC_INFO command's queries, as an entity
    /// builds them, with the status of the whole.
    fn dynamic_info(&mut self, payload: &[u8]) -> (AemStatus, Vec<u8>) {
        if !self.answers_dynamic_info {
            return (AemStatus::NOT_IMPLEMENTED, payload.to_vec());
        }
        let mut answers = Vec::new();
        let mut rest = payload;
        while rest.len() >= 8 {
            let length = usize::from(u16::from_be_bytes([rest[0], rest[1]]));
            let command_type = AemCommandType(u16::from_be_bytes([rest[6], rest[7]]));
            let data = &rest[8..8 + length];
            rest = &rest[8 + length..];
            let key = (
                u16::from_be_bytes([data[0], data[1]]),
                u16::from_be_bytes([data[2], data[3]]),
            );
            let stored = self.descriptors.get(&key);
            let (status, answer) = match (command_type, stored) {
                (_, None) => (AemStatus::NO_SUCH_DESCRIPTOR, data.to_vec()),
                (AemCommandType::GET_NAME, Some(bytes)) => {
                    let at = match (key.0, u16::from_be_bytes([data[4], data[5]])) {
                        (0, 0) => 48,
                        (0, _) => 180,
                        _ => 4,
                    };
                    let mut answer = data[..8].to_vec();
                    answer.extend_from_slice(&bytes[at..at + 64]);
                    (AemStatus::SUCCESS, answer)
                }
                (AemCommandType::GET_STREAM_FORMAT, Some(bytes)) => {
                    let mut answer = data[..4].to_vec();
                    answer.extend_from_slice(&bytes[74..82]);
                    (AemStatus::SUCCESS, answer)
                }
                (AemCommandType::GET_SAMPLING_RATE, Some(bytes)) => {
                    let mut answer = data[..4].to_vec();
                    answer.extend_from_slice(&bytes[136..140]);
                    (AemStatus::SUCCESS, answer)
                }
                (AemCommandType::GET_CLOCK_SOURCE, Some(bytes)) => {
                    let mut answer = data[..4].to_vec();
                    answer.extend_from_slice(&bytes[70..72]);
                    answer.extend_from_slice(&[0, 0]);
                    (AemStatus::SUCCESS, answer)
                }
                _ => return (AemStatus::BAD_ARGUMENTS, payload.to_vec()),
            };
            answers.extend_from_slice(&(answer.len() as u16).to_be_bytes());
            answers.extend_from_slice(&[0, 0, status.0 << 3, 0]);
            answers.extend_from_slice(&command_type.0.to_be_bytes());
            answers.extend_from_slice(&answer);
        }
        (AemStatus::SUCCESS, answers)
    }

    fn stored(&mut self, descriptor_type: DescriptorType, index: u16) -> &mut Vec<u8> {
        self.descriptors
            .get_mut(&(descriptor_type.0, index))
            .unwrap()
    }

    /// Applies a SET command as an entity would, returning its status;
    /// the response's payload is the command's, holding what is set now.
    fn set(&mut self, command_type: AemCommandType, payload: &mut [u8]) -> AemStatus {
        let descriptor_type = DescriptorType(u16::from_be_bytes([payload[0], payload[1]]));
        let index = u16::from_be_bytes([payload[2], payload[3]]);
        if !self.descriptors.contains_key(&(descriptor_type.0, index)) {
            return AemStatus::NO_SUCH_DESCRIPTOR;
        }
        match command_type {
            AemCommandType::SET_NAME => {
                let name_index = u16::from_be_bytes([payload[4], payload[5]]);
                let at = match (descriptor_type, name_index) {
                    (DescriptorType::ENTITY, 0) => 48,
                    (DescriptorType::ENTITY, 1) => 180,
                    (_, 0) => 4,
                    _ => return AemStatus::BAD_ARGUMENTS,
                };
                self.stored(descriptor_type, index)[at..at + 64].copy_from_slice(&payload[8..72]);
            }
            AemCommandType::SET_STREAM_FORMAT => {
                let streaming = self.streaming;
                let stored = self.stored(descriptor_type, index);
                if streaming {
                    let current: [u8; 8] = stored[74..82].try_into().unwrap();
                    payload[4..12].copy_from_slice(&current);
                    return AemStatus::STREAM_IS_RUNNING;
                }
                stored[74..82].copy_from_slice(&payload[4..12]);
            }
            AemCommandType::SET_SAMPLING_RATE => {
                self.stored(descriptor_type, index)[136..140].copy_from_slice(&payload[4..8]);
                // The streams follow the rate.
                for (key, stream) in &mut self.descriptors {
                    if key.0 == DescriptorType::STREAM_INPUT.0
                        || key.0 == DescriptorType::STREAM_OUTPUT.0
                    {
                        stream[74..82].copy_from_slice(&0x0205_0420_0200_6000u64.to_be_bytes());
                    }
                }
            }
            AemCommandType::SET_CLOCK_SOURCE => {
                let source = u16::from_be_bytes([payload[4], payload[5]]);
                if source > 1 {
                    return AemStatus::BAD_ARGUMENTS;
                }
                self.stored(descriptor_type, index)[70..72].copy_from_slice(&payload[4..6]);
            }
            _ => {}
        }
        AemStatus::SUCCESS
    }

    fn rename(&mut self, descriptor_type: DescriptorType, index: u16, name: &str) {
        let bytes = self
            .descriptors
            .get_mut(&(descriptor_type.0, index))
            .unwrap();
        bytes[4..68].fill(0);
        bytes[4..4 + name.len()].copy_from_slice(name.as_bytes());
    }

    /// The response to an ACMP command for this entity's stream inputs.
    fn respond_acmp(&mut self, command: &Acmpdu) -> Option<Acmpdu> {
        // TX commands go to the talker, RX ones to the listener; the
        // entity is both.
        let addressed = match command.message_type {
            AcmpMessageType::GET_TX_STATE_COMMAND | AcmpMessageType::DISCONNECT_TX_COMMAND => {
                command.talker_entity_id
            }
            _ => command.listener_entity_id,
        };
        if addressed != TALKER || !self.answers_acmp {
            return None;
        }
        let mut response = Acmpdu {
            message_type: command.message_type.response(),
            ..*command
        };
        match command.message_type {
            AcmpMessageType::CONNECT_RX_COMMAND => {
                response.status = self.bind_status;
                if self.bind_status.is_success() {
                    self.bindings.insert(
                        command.listener_unique_id,
                        (command.talker_entity_id, command.talker_unique_id),
                    );
                    response.connection_count = 1;
                }
            }
            AcmpMessageType::DISCONNECT_RX_COMMAND => {
                self.bindings.remove(&command.listener_unique_id);
                response.connection_count = 0;
            }
            AcmpMessageType::GET_RX_STATE_COMMAND => {
                let bound = self.bindings.get(&command.listener_unique_id);
                let (talker, talker_unique_id) = bound.copied().unwrap_or_default();
                response.talker_entity_id = talker;
                response.talker_unique_id = talker_unique_id;
                response.connection_count = u16::from(bound.is_some());
            }
            AcmpMessageType::GET_TX_STATE_COMMAND => {
                response.stream_id = crate::id::StreamId(0x0011_2233_4455_0000);
                response.stream_dest_mac = MacAddress([0x91, 0xe0, 0xf0, 0x00, 0xfe, 0x01]);
                response.connection_count = 2;
                response.stream_vlan_id = 2;
            }
            AcmpMessageType::DISCONNECT_TX_COMMAND => {
                self.tx_disconnects.push((
                    command.talker_unique_id,
                    command.listener_entity_id,
                    command.listener_unique_id,
                ));
                response.connection_count = 1;
            }
            _ => return None,
        }
        Some(response)
    }

    /// The response to a frame the controller sent, if it is a command.
    fn respond(&mut self, frame: &[u8]) -> Option<Vec<u8>> {
        let mut out = [0; 600];
        if let Ok(command) = Acmpdu::decode(frame) {
            let response = self.respond_acmp(&command)?;
            let length = response.encode(&mut out).unwrap();
            return Some(out[..length].to_vec());
        }
        if let Ok(command) = AemPdu::decode(frame) {
            self.commands.push(command.command_type);
            let mut payload = command.payload.to_vec();
            let mut status = AemStatus::SUCCESS;
            if command.command_type == AemCommandType::READ_DESCRIPTOR {
                let key = (
                    u16::from_be_bytes([payload[4], payload[5]]),
                    u16::from_be_bytes([payload[6], payload[7]]),
                );
                match self.descriptors.get(&key) {
                    Some(bytes) => {
                        payload.truncate(4);
                        payload.extend_from_slice(bytes);
                    }
                    None => status = AemStatus::NO_SUCH_DESCRIPTOR,
                }
            }
            if command.command_type == AemCommandType::GET_STREAM_INFO {
                let index = u16::from_be_bytes([payload[2], payload[3]]);
                let input =
                    u16::from_be_bytes([payload[0], payload[1]]) == DescriptorType::STREAM_INPUT.0;
                let mut flags = StreamInfoFlags::STREAM_FORMAT_VALID;
                if input && self.bindings.contains_key(&index) {
                    flags |= StreamInfoFlags::CONNECTED;
                }
                if !input && self.class_b {
                    flags |= StreamInfoFlags::CLASS_B;
                }
                payload.resize(48, 0);
                payload[4..8].copy_from_slice(&flags.0.to_be_bytes());
            }
            match command.command_type {
                _ if !self.answers_network => {}
                AemCommandType::GET_AVB_INFO => {
                    payload.resize(24, 0);
                    payload[4..12].copy_from_slice(&GRANDMASTER.0.to_be_bytes());
                    payload[12..16].copy_from_slice(&310u32.to_be_bytes());
                    payload[17] = 0x07;
                    payload[18..20].copy_from_slice(&1u16.to_be_bytes());
                    payload[20..24].copy_from_slice(&[0, 3, 0, 2]);
                }
                AemCommandType::GET_AS_PATH => {
                    payload.truncate(2);
                    payload.extend_from_slice(&2u16.to_be_bytes());
                    payload.extend_from_slice(&GRANDMASTER.0.to_be_bytes());
                    payload.extend_from_slice(&BRIDGE.0.to_be_bytes());
                }
                AemCommandType::GET_COUNTERS => {
                    payload.resize(136, 0);
                    payload[4..8].copy_from_slice(&0x23u32.to_be_bytes());
                    payload[8..12].copy_from_slice(&1u32.to_be_bytes());
                    payload[28..32].copy_from_slice(&2u32.to_be_bytes());
                }
                _ => {}
            }
            if matches!(
                command.command_type,
                AemCommandType::GET_AVB_INFO
                    | AemCommandType::GET_AS_PATH
                    | AemCommandType::GET_COUNTERS
            ) && !self.answers_network
            {
                status = AemStatus::NOT_IMPLEMENTED;
            }
            if matches!(
                command.command_type,
                AemCommandType::SET_NAME
                    | AemCommandType::SET_STREAM_FORMAT
                    | AemCommandType::SET_SAMPLING_RATE
                    | AemCommandType::SET_CLOCK_SOURCE
            ) {
                status = self.set(command.command_type, &mut payload);
            }
            if command.command_type == AemCommandType::GET_DYNAMIC_INFO {
                let (whole, answers) = self.dynamic_info(&payload);
                status = whole;
                payload = answers;
            }
            if matches!(
                command.command_type,
                AemCommandType::GET_AUDIO_MAP
                    | AemCommandType::ADD_AUDIO_MAPPINGS
                    | AemCommandType::REMOVE_AUDIO_MAPPINGS
            ) {
                status = self.audio_map(command.command_type, &mut payload);
            }
            if command.command_type == AemCommandType::SET_CONTROL {
                let index = u16::from_be_bytes([payload[2], payload[3]]);
                self.controls.push((index, payload[4..].to_vec()));
            }
            if matches!(
                command.command_type,
                AemCommandType::GET_MAX_TRANSIT_TIME | AemCommandType::SET_MAX_TRANSIT_TIME
            ) {
                if !self.answers_transit {
                    status = AemStatus::NOT_IMPLEMENTED;
                } else if command.command_type == AemCommandType::SET_MAX_TRANSIT_TIME
                    && self.streaming
                {
                    // A refusal holds the time asked for, as macOS's does.
                    status = AemStatus::STREAM_IS_RUNNING;
                } else {
                    if command.command_type == AemCommandType::SET_MAX_TRANSIT_TIME {
                        self.transit = u64::from_be_bytes(payload[4..12].try_into().unwrap());
                    }
                    // Otherwise the response holds the time the output has.
                    payload.truncate(4);
                    payload.extend_from_slice(&self.transit.to_be_bytes());
                }
            }
            let response = AemPdu {
                header: AecpHeader {
                    message_type: AecpMessageType::AEM_RESPONSE,
                    status: status.0,
                    ..command.header
                },
                payload: &payload,
                ..command
            };
            let length = response.encode(&mut out).unwrap();
            return Some(out[..length].to_vec());
        }
        let command = VendorUniquePdu::decode(frame).ok()?;
        if let Ok(asked) = crate::lite::LiteMessage::from_pdu(&command) {
            // The AVB Lite status of the interface asked about, from an
            // entity with AVB Lite; else not implemented.
            let (status, data) = match self.lite {
                Some(status) => {
                    let mut data = asked.command_type.0.to_be_bytes().to_vec();
                    data.extend_from_slice(&status.to_bytes());
                    (0, data)
                }
                None => (
                    AemStatus::NOT_IMPLEMENTED.0,
                    [&asked.command_type.0.to_be_bytes()[..], asked.data].concat(),
                ),
            };
            let response = VendorUniquePdu {
                header: AecpHeader {
                    message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
                    status,
                    ..command.header
                },
                protocol_id: crate::lite::STATUS_PROTOCOL_ID,
                payload: &data,
            };
            let length = response.encode(&mut out).unwrap();
            return Some(out[..length].to_vec());
        }
        if let Ok(asked) = WirelessMessage::from_pdu(&command) {
            if self.wireless_silent > 0 {
                self.wireless_silent -= 1;
                return None;
            }
            return Some(self.respond_wireless(&command, &asked));
        }
        let asked = crate::mvu::MvuMessage::from_pdu(&command).ok()?;
        if !self.milan {
            let response = VendorUniquePdu {
                header: AecpHeader {
                    message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
                    status: AemStatus::NOT_IMPLEMENTED.0,
                    ..command.header
                },
                ..command
            };
            let length = response.encode(&mut out).unwrap();
            return Some(out[..length].to_vec());
        }
        let data = match asked.command_type {
            crate::mvu::MvuCommandType::GET_MEDIA_CLOCK_REFERENCE_INFO => {
                // Priority 192, for an audio interface, both changeable.
                let mut data = vec![0u8; 76];
                data[1] = 0x04;
                data[2..4].copy_from_slice(&asked.data[..2]);
                data[4] = 0x03;
                data[6] = 192;
                data[7] = self.user_priority;
                data[12..19].copy_from_slice(b"DEFAULT");
                data
            }
            _ => {
                let mut data = vec![0u8; 20];
                data[1] = 0x00; // GET_MILAN_INFO
                data[4..8].copy_from_slice(&1u32.to_be_bytes());
                data[16..20].copy_from_slice(&[1, 3, 0, 0]);
                data
            }
        };
        let response = VendorUniquePdu {
            header: AecpHeader {
                message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
                ..command.header
            },
            protocol_id: MVU_PROTOCOL_ID,
            payload: &data,
        };
        let length = response.encode(&mut out).unwrap();
        Some(out[..length].to_vec())
    }
}

impl FakeEntity {
    /// The entity with `count` stream outputs, each as the first.
    fn with_outputs(mut self, count: u16) -> Self {
        let configuration = self
            .descriptors
            .get_mut(&(DescriptorType::CONFIGURATION.0, 0))
            .unwrap();
        // The outputs' count, third of the configuration's.
        put(configuration, 84, count);
        let first = self.descriptors[&(DescriptorType::STREAM_OUTPUT.0, 0)].clone();
        for index in 1..count {
            let mut output = first.clone();
            put(&mut output, 2, index);
            self.descriptors
                .insert((DescriptorType::STREAM_OUTPUT.0, index), output);
        }
        self
    }

    /// Answers the AVB Wireless status query and configuration: for its
    /// wireless interface when it has AVB Wireless, NOT_SUPPORTED for its
    /// wired ones, and NOT_IMPLEMENTED without AVB Wireless.
    fn respond_wireless(
        &mut self,
        command: &VendorUniquePdu<'_>,
        asked: &WirelessMessage<'_>,
    ) -> Vec<u8> {
        let interface = read_u16(asked.data, 0);
        let mut data = asked.command_type.0.to_be_bytes().to_vec();
        let status = match &mut self.wireless {
            None => AemStatus::NOT_IMPLEMENTED,
            Some(wireless) if wireless.status.interface != interface => AemStatus::NOT_SUPPORTED,
            Some(wireless) => {
                if asked.command_type == WirelessCommandType::SET_WIRELESS_CONFIG {
                    let allowed = asked.data[2] & 0x01 != 0;
                    let flag = crate::wireless::WirelessFlags::CLASS_A_ALLOWED;
                    wireless.status.flags = if allowed {
                        wireless.status.flags | flag
                    } else {
                        crate::wireless::WirelessFlags(wireless.status.flags.0 & !flag.0)
                    };
                    data.extend_from_slice(&asked.data[..6]);
                } else {
                    data.extend_from_slice(&wireless.status.to_bytes());
                    for station in &wireless.stations {
                        data.extend_from_slice(&station.to_bytes());
                    }
                }
                AemStatus::SUCCESS
            }
        };
        if status != AemStatus::SUCCESS {
            data.extend_from_slice(asked.data);
        }
        let response = VendorUniquePdu {
            header: AecpHeader {
                message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
                status: status.0,
                ..command.header
            },
            protocol_id: crate::wireless::WIRELESS_PROTOCOL_ID,
            payload: &data,
        };
        let mut out = [0; 1500];
        let length = response.encode(&mut out).unwrap();
        out[..length].to_vec()
    }
}

/// Lets the controller and the entity talk until neither has more to
/// say, returning the frames the controller sent.
fn exchange(controller: &mut Controller, entity: &mut FakeEntity, now: Instant) -> usize {
    let mut sent = 0;
    loop {
        let frames = transmits(controller);
        if frames.is_empty() {
            return sent;
        }
        for (destination, frame) in frames {
            sent += 1;
            let acmp = destination == crate::ADP_ACMP_MULTICAST && frame[0] == 0xfc;
            if destination != TALKER_MAC && !acmp {
                continue;
            }
            if let Some(response) = entity.respond(&frame) {
                controller.handle_frame(now, TALKER_MAC, &response).unwrap();
            }
        }
    }
}

fn enumerated() -> (Controller, FakeEntity) {
    let mut controller = controller();
    let mut entity = FakeEntity::new();
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    controller.pump(at(0));
    exchange(&mut controller, &mut entity, at(0));
    (controller, entity)
}

#[test]
fn an_advertisement_brings_an_entity_online_once() {
    let mut controller = controller();
    controller.handle_adpdu(at(0), TALKER_MAC, &available(TALKER, 7));
    controller.handle_adpdu(at(5), TALKER_MAC, &available(TALKER, 8));
    assert_eq!(events(&mut controller), [Event::EntityOnline(TALKER)]);
    let entity = controller.entity(TALKER).unwrap();
    assert_eq!(entity.mac, TALKER_MAC);
    assert_eq!(entity.first_seen, at(0));
    assert_eq!(entity.last_seen, at(5));
    assert_eq!(entity.expires, at(25));
    // Without AEM there is nothing to read.
    assert!(controller.model(TALKER).is_none());
}

#[test]
fn changes_and_restarts_are_reported() {
    let mut controller = controller();
    controller.handle_adpdu(at(0), TALKER_MAC, &available(TALKER, 7));
    let mut moved = available(TALKER, 8);
    moved.gptp_grandmaster_id = ClockIdentity(0x0001_f2ff_feff_3b14);
    controller.handle_adpdu(at(5), TALKER_MAC, &moved);
    let mut rebooted = moved;
    rebooted.available_index = 0;
    controller.handle_adpdu(at(9), TALKER_MAC, &rebooted);
    assert_eq!(
        events(&mut controller),
        [
            Event::EntityOnline(TALKER),
            Event::EntityChanged(TALKER),
            Event::EntityRestarted(TALKER),
        ]
    );
}

#[test]
fn expiry_happens_at_the_deadline_and_not_before() {
    let mut controller = controller();
    controller.handle_adpdu(at(0), TALKER_MAC, &available(TALKER, 1));
    events(&mut controller);
    assert_eq!(controller.poll_timeout(), Some(at(20)));
    controller.handle_timeout(Instant::from_nanos(20_000_000_000 - 1));
    assert!(controller.entity(TALKER).is_some());
    assert!(events(&mut controller).is_empty());
    controller.handle_timeout(at(20));
    assert!(controller.entity(TALKER).is_none());
    assert_eq!(
        events(&mut controller),
        [Event::EntityOffline(TALKER, OfflineReason::TimedOut)]
    );
    assert_eq!(controller.poll_timeout(), None);
}

#[test]
fn departing_entities_go_offline() {
    let mut controller = controller();
    controller.handle_adpdu(at(0), TALKER_MAC, &available(TALKER, 1));
    let mut departing = available(TALKER, 0);
    departing.message_type = AdpMessageType::ENTITY_DEPARTING;
    departing.valid_time = 0;
    controller.handle_adpdu(at(1), TALKER_MAC, &departing);
    controller.handle_adpdu(at(2), TALKER_MAC, &departing);
    assert_eq!(
        events(&mut controller),
        [
            Event::EntityOnline(TALKER),
            Event::EntityOffline(TALKER, OfflineReason::Departed),
        ]
    );
}

#[test]
fn ignores_itself_and_invalid_ids() {
    let mut controller = controller();
    controller.handle_adpdu(at(0), TALKER_MAC, &available(CONTROLLER, 1));
    controller.handle_adpdu(at(0), TALKER_MAC, &available(EntityId(0), 1));
    controller.handle_adpdu(at(0), TALKER_MAC, &Adpdu::discover(EntityId(0)));
    assert!(events(&mut controller).is_empty());
    assert_eq!(controller.entities().count(), 0);
}

#[test]
fn discover_queues_a_multicast_entity_discover() {
    let mut controller = controller();
    controller.discover(None);
    let mut small = [0; 10];
    assert!(controller.poll_transmit(&mut small).is_err());
    let frames = transmits(&mut controller);
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0].0, crate::ADP_ACMP_MULTICAST);
    assert_eq!(
        Adpdu::decode(&frames[0].1),
        Ok(Adpdu::discover(EntityId(0)))
    );
}

#[test]
fn malformed_frames_are_counted() {
    let mut controller = controller();
    assert!(
        controller
            .handle_frame(at(0), TALKER_MAC, &[0xfa, 0x00])
            .is_err()
    );
    let frame = available(TALKER, 1).to_bytes().unwrap();
    assert!(controller.handle_frame(at(0), TALKER_MAC, &frame).is_ok());
    assert_eq!(controller.malformed_frames(), 1);
    assert_eq!(events(&mut controller), [Event::EntityOnline(TALKER)]);
}

#[test]
fn enumerates_the_whole_model() {
    let (mut controller, entity) = enumerated();
    let events = events(&mut controller);
    assert!(events.contains(&Event::EnumerationStarted(TALKER)));
    assert!(events.contains(&Event::EntityEnumerated(TALKER)));
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.state, EnumerationState::Complete);
    assert_eq!(model.failed_reads, 0);
    assert!(model.registered);
    assert_eq!(
        model.milan.unwrap().specification_version,
        Some([1, 3, 0, 0])
    );
    assert_eq!(model.entity_name(), Some("Stage box"));
    let entity_descriptor = model.entity().unwrap();
    assert_eq!(model.localized(entity_descriptor.vendor_name), Some("Acme"));
    assert_eq!(model.localized(entity_descriptor.model_name), Some("Box 1"));
    assert_eq!(model.configuration().unwrap().object_name, "Main");
    assert_eq!(model.streams(true).count(), 2);
    assert_eq!(model.streams(false).count(), 1);
    // An unset object_name falls back to the localized description.
    assert_eq!(
        model.name_of(DescriptorType::STREAM_INPUT, 0),
        Some("Input A")
    );
    assert_eq!(model.name_of(DescriptorType::STREAM_INPUT, 1), Some("Mic"));
    // Children of the audio unit and its port are read too; types the
    // controller does not use are not.
    assert_eq!(model.descriptors(DescriptorType::AUDIO_CLUSTER).count(), 3);
    assert_eq!(model.descriptors(DescriptorType::AUDIO_MAP).count(), 1);
    assert_eq!(model.descriptors(DescriptorType::VIDEO_UNIT).count(), 0);
    assert_eq!(model.descriptor_count(), entity.descriptors.len());
    // Registration comes last, after everything is read.
    assert_eq!(
        entity.commands.last(),
        Some(&AemCommandType::REGISTER_UNSOLICITED_NOTIFICATION)
    );
}

#[test]
fn one_command_in_flight_per_entity() {
    let mut controller = controller();
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    let first = transmits(&mut controller);
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].0, TALKER_MAC);
    assert!(
        VendorUniquePdu::decode(&first[0].1).is_ok(),
        "GET_MILAN_INFO first"
    );
    assert!(transmits(&mut controller).is_empty());
}

#[test]
fn unanswered_commands_retry_once_then_fail() {
    let mut controller = controller();
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    let milan = transmits(&mut controller);
    controller.handle_timeout(after(at(0), 250));
    assert_eq!(
        transmits(&mut controller),
        milan,
        "the retry repeats the command"
    );
    controller.handle_timeout(after(at(0), 500));
    let read_entity = transmits(&mut controller);
    let command = AemPdu::decode(&read_entity[0].1).unwrap();
    assert_eq!(command.command_type, AemCommandType::READ_DESCRIPTOR);
    controller.handle_timeout(after(at(0), 750));
    assert_eq!(transmits(&mut controller), read_entity);
    controller.handle_timeout(after(at(0), 1000));
    assert!(events(&mut controller).contains(&Event::EnumerationFailed(
        TALKER,
        EnumerationFailure::NoResponse
    )));
    let model = controller.model(TALKER).unwrap();
    assert_eq!(
        model.state,
        EnumerationState::Failed(EnumerationFailure::NoResponse)
    );
    assert!(model.milan.is_none());
}

#[test]
fn entities_on_the_own_mac_are_not_read() {
    let mut config = Config::new(CONTROLLER);
    config.own_mac = Some(TALKER_MAC);
    let mut controller = Controller::new(config);
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    assert!(transmits(&mut controller).is_empty());
    assert!(events(&mut controller).contains(&Event::EnumerationFailed(
        TALKER,
        EnumerationFailure::OnThisComputer
    )));
    assert_eq!(
        controller.model(TALKER).unwrap().state,
        EnumerationState::Failed(EnumerationFailure::OnThisComputer)
    );
}

#[test]
fn in_progress_holds_off_the_timeout() {
    let mut controller = controller();
    let mut adpdu = aem_available(1);
    adpdu.entity_capabilities = EntityCapabilities::AEM_SUPPORTED;
    controller.handle_adpdu(at(0), TALKER_MAC, &adpdu);
    let read_entity = transmits(&mut controller);
    let command = AemPdu::decode(&read_entity[0].1).unwrap();
    let in_progress = AemPdu {
        header: AecpHeader {
            message_type: AecpMessageType::AEM_RESPONSE,
            status: AemStatus::IN_PROGRESS.0,
            ..command.header
        },
        ..command
    };
    let mut out = [0; 64];
    let length = in_progress.encode(&mut out).unwrap();
    controller
        .handle_frame(after(at(0), 200), TALKER_MAC, &out[..length])
        .unwrap();
    controller.handle_timeout(after(at(0), 300));
    assert!(
        transmits(&mut controller).is_empty(),
        "no retry while in progress"
    );
    controller.handle_timeout(after(at(0), 450));
    assert_eq!(
        transmits(&mut controller),
        read_entity,
        "retry once it lapses"
    );
}

#[test]
fn answers_controller_available() {
    let mut controller = controller();
    let check = |command_type| AemPdu {
        header: AecpHeader {
            message_type: AecpMessageType::AEM_COMMAND,
            status: 0,
            target_entity_id: CONTROLLER,
            controller_entity_id: TALKER,
            sequence_id: 77,
        },
        unsolicited: false,
        controller_request: false,
        command_type,
        payload: &[],
    };
    let mut out = [0; 64];
    let length = check(AemCommandType::CONTROLLER_AVAILABLE)
        .encode(&mut out)
        .unwrap();
    controller
        .handle_frame(at(0), TALKER_MAC, &out[..length])
        .unwrap();
    let length = check(AemCommandType::REBOOT).encode(&mut out).unwrap();
    controller
        .handle_frame(at(0), TALKER_MAC, &out[..length])
        .unwrap();
    let frames = transmits(&mut controller);
    assert_eq!(frames.len(), 2);
    let available = AemPdu::decode(&frames[0].1).unwrap();
    assert_eq!(frames[0].0, TALKER_MAC);
    assert_eq!(available.header.message_type, AecpMessageType::AEM_RESPONSE);
    assert_eq!(available.status(), AemStatus::SUCCESS);
    assert_eq!(available.header.sequence_id, 77);
    assert_eq!(available.header.controller_entity_id, TALKER);
    let reboot = AemPdu::decode(&frames[1].1).unwrap();
    assert_eq!(reboot.status(), AemStatus::NOT_IMPLEMENTED);
}

/// An unsolicited notification from the entity, for `command_type` about
/// a descriptor.
fn notification(
    command_type: AemCommandType,
    descriptor_type: DescriptorType,
    index: u16,
) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    put(&mut payload, 0, descriptor_type.0);
    put(&mut payload, 2, index);
    let pdu = AemPdu {
        header: AecpHeader {
            message_type: AecpMessageType::AEM_RESPONSE,
            status: 0,
            target_entity_id: TALKER,
            controller_entity_id: CONTROLLER,
            sequence_id: 3,
        },
        unsolicited: true,
        controller_request: false,
        command_type,
        payload: &payload,
    };
    let mut out = [0; 64];
    let length = pdu.encode(&mut out).unwrap();
    out[..length].to_vec()
}

#[test]
fn notifications_read_the_descriptor_again() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    entity.rename(DescriptorType::STREAM_INPUT, 1, "Vocal");
    let frame = notification(AemCommandType::SET_NAME, DescriptorType::STREAM_INPUT, 1);
    controller.handle_frame(at(1), TALKER_MAC, &frame).unwrap();
    assert_eq!(exchange(&mut controller, &mut entity, at(1)), 1);
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    let model = controller.model(TALKER).unwrap();
    assert_eq!(
        model.name_of(DescriptorType::STREAM_INPUT, 1),
        Some("Vocal")
    );
    assert_eq!(model.state, EnumerationState::Complete);
}

#[test]
fn reads_where_each_interface_sits() {
    let (controller, _) = enumerated();
    let model = controller.model(TALKER).unwrap();
    let info = model.avb_info(0).unwrap();
    assert_eq!(info.gptp_grandmaster_id, GRANDMASTER);
    assert_eq!(info.propagation_delay, 310);
    assert!(info.as_capable());
    assert_eq!(model.as_path(0), Some(&[GRANDMASTER, BRIDGE][..]));
    let counters = model
        .counters(DescriptorType::AVB_INTERFACE, 0)
        .and_then(Counters::avb_interface)
        .unwrap();
    assert_eq!((counters.link_up, counters.link_down), (Some(1), Some(0)));
    assert_eq!(counters.gptp_gm_changed, Some(2));
}

#[test]
fn a_new_grandmaster_reads_where_the_entity_sits_again() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let from = entity.commands.len();
    let mut moved = aem_available(2);
    moved.gptp_grandmaster_id = ClockIdentity(0x0001_f2ff_feff_3b14);
    controller.handle_adpdu(at(1), TALKER_MAC, &moved);
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(
        commands_since(&entity, from),
        [AemCommandType::GET_AVB_INFO, AemCommandType::GET_AS_PATH]
    );
    // The same grandmaster again reads nothing.
    let from = entity.commands.len();
    controller.handle_adpdu(
        at(2),
        TALKER_MAC,
        &Adpdu {
            available_index: 3,
            ..moved
        },
    );
    exchange(&mut controller, &mut entity, at(2));
    assert!(commands_since(&entity, from).is_empty());
}

#[test]
fn entities_without_network_queries_are_still_read() {
    let mut controller = controller();
    let mut entity = FakeEntity::new();
    entity.answers_network = false;
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    controller.pump(at(0));
    exchange(&mut controller, &mut entity, at(0));
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.state, EnumerationState::Complete);
    assert!(model.registered);
    assert_eq!(model.avb_info(0), None);
    assert_eq!(model.as_path(0), None);
}

#[test]
fn a_new_path_updates_the_model_without_reading_again() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let mut payload = vec![0, 0, 0, 1];
    payload.extend_from_slice(&BRIDGE.0.to_be_bytes());
    let pdu = AemPdu {
        header: AecpHeader {
            message_type: AecpMessageType::AEM_RESPONSE,
            status: 0,
            target_entity_id: TALKER,
            controller_entity_id: CONTROLLER,
            sequence_id: 77,
        },
        unsolicited: true,
        controller_request: false,
        command_type: AemCommandType::GET_AS_PATH,
        payload: &payload,
    };
    let mut out = [0; 64];
    let length = pdu.encode(&mut out).unwrap();
    controller
        .handle_frame(at(1), TALKER_MAC, &out[..length])
        .unwrap();
    assert_eq!(exchange(&mut controller, &mut entity, at(1)), 0);
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    assert_eq!(
        controller.model(TALKER).unwrap().as_path(0),
        Some(&[BRIDGE][..])
    );
}

#[test]
fn a_dropped_registration_is_renewed() {
    let (mut controller, mut entity) = enumerated();
    let frame = notification(
        AemCommandType::DEREGISTER_UNSOLICITED_NOTIFICATION,
        DescriptorType::ENTITY,
        0,
    );
    controller.handle_frame(at(1), TALKER_MAC, &frame).unwrap();
    entity.commands.clear();
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(
        entity.commands,
        [AemCommandType::REGISTER_UNSOLICITED_NOTIFICATION]
    );
    assert!(controller.model(TALKER).unwrap().registered);
}

#[test]
fn restarts_read_again_and_departures_forget() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    controller.handle_adpdu(at(2), TALKER_MAC, &aem_available(0));
    assert_eq!(
        controller.model(TALKER).unwrap().state,
        EnumerationState::Reading
    );
    exchange(&mut controller, &mut entity, at(2));
    assert!(events(&mut controller).contains(&Event::EntityEnumerated(TALKER)));
    let mut departing = aem_available(0);
    departing.message_type = AdpMessageType::ENTITY_DEPARTING;
    controller.handle_adpdu(at(3), TALKER_MAC, &departing);
    assert!(controller.model(TALKER).is_none());
}

#[test]
fn advertises_itself_and_answers_discovers() {
    let mut config = Config::new(CONTROLLER);
    config.advertise = Some(Advertise {
        entity_model_id: EntityModelId(0x8c1f_6436_c000_0001),
        valid_time: 31,
    });
    let mut controller = Controller::new(config);
    controller.handle_timeout(at(0));
    let first = controller.poll_timeout().unwrap();
    assert!(first <= at(1), "first advertisement within a second");
    controller.handle_timeout(first);
    let frames = transmits(&mut controller);
    let adpdu = Adpdu::decode(&frames[0].1).unwrap();
    assert_eq!(adpdu.message_type, AdpMessageType::ENTITY_AVAILABLE);
    assert_eq!(adpdu.entity_id, CONTROLLER);
    assert_eq!(adpdu.valid_time, 31);
    assert!(
        adpdu
            .controller_capabilities
            .contains(ControllerCapabilities::IMPLEMENTED)
    );
    // Re-announced at a quarter of the valid time.
    assert_eq!(
        controller.poll_timeout(),
        Some(first + Duration::from_millis(15_500))
    );
    // A discover brings the next one forward.
    controller.handle_adpdu(at(5), TALKER_MAC, &Adpdu::discover(EntityId(0)));
    assert!(controller.poll_timeout().unwrap() <= at(6));
    controller.depart();
    let frames = transmits(&mut controller);
    let departing = Adpdu::decode(&frames[0].1).unwrap();
    assert_eq!(departing.message_type, AdpMessageType::ENTITY_DEPARTING);
    assert_eq!(departing.available_index, 0);
    assert_eq!(controller.poll_timeout(), None);
}

#[test]
fn reads_bindings_and_stream_info() {
    let (controller, _) = enumerated();
    let model = controller.model(TALKER).unwrap();
    for input in [0, 1] {
        assert_eq!(model.binding(input).unwrap().talker_stream(), None);
        let info = model
            .stream_info(DescriptorType::STREAM_INPUT, input)
            .unwrap();
        assert!(!info.bound());
    }
    assert!(
        model
            .stream_info(DescriptorType::STREAM_OUTPUT, 0)
            .is_some()
    );
}

/// The CommandFinished events since the last call.
fn finished(controller: &mut Controller) -> Vec<(CommandId, Outcome)> {
    events(controller)
        .into_iter()
        .filter_map(|event| match event {
            Event::CommandFinished(command, outcome) => Some((command, outcome)),
            _ => None,
        })
        .collect()
}

const OTHER_TALKER: EntityId = EntityId(0xd111_e597_f544_8000);

#[test]
fn connects_and_disconnects() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let connect = controller.connect(at(1), (OTHER_TALKER, 2), (TALKER, 1));
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(finished(&mut controller), [(connect, Outcome::Done)]);
    let model = controller.model(TALKER).unwrap();
    assert_eq!(
        model.binding(1).unwrap().talker_stream(),
        Some((OTHER_TALKER, 2))
    );
    assert!(
        model
            .stream_info(DescriptorType::STREAM_INPUT, 1)
            .unwrap()
            .bound()
    );

    let disconnect = controller.disconnect(at(2), (TALKER, 1));
    let frames = transmits(&mut controller);
    let unbind = Acmpdu::decode(&frames[0].1).unwrap();
    assert_eq!(unbind.message_type, AcmpMessageType::UNBIND_RX_COMMAND);
    assert_eq!(
        (unbind.talker_entity_id, unbind.talker_unique_id),
        (OTHER_TALKER, 2)
    );
    let response = entity.respond(&frames[0].1).unwrap();
    controller
        .handle_frame(at(2), TALKER_MAC, &response)
        .unwrap();
    exchange(&mut controller, &mut entity, at(2));
    assert_eq!(finished(&mut controller), [(disconnect, Outcome::Done)]);
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.binding(1).unwrap().talker_stream(), None);
}

#[test]
fn refused_and_impossible_connections() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    entity.bind_status = AcmpStatus::CONTROLLER_NOT_AUTHORIZED;
    let refused = controller.connect(at(1), (OTHER_TALKER, 0), (TALKER, 0));
    exchange(&mut controller, &mut entity, at(1));
    let unknown = controller.connect(at(1), (TALKER, 0), (EntityId(42), 0));
    assert_eq!(
        finished(&mut controller),
        [
            (
                refused,
                Outcome::Refused(Refusal::Acmp(AcmpStatus::CONTROLLER_NOT_AUTHORIZED))
            ),
            (unknown, Outcome::NotPossible),
        ]
    );
}

/// A connect to a talker whose output reports class B says CLASS_B, so a
/// talker that takes its class from the connect keeps class B; one to a
/// talker in class A leaves it clear.
#[test]
fn connects_carry_the_talkers_class() {
    let class_of_connect = |class_b: bool| {
        let mut controller = controller();
        let mut entity = FakeEntity::new();
        entity.class_b = class_b;
        controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
        controller.pump(at(0));
        exchange(&mut controller, &mut entity, at(0));
        events(&mut controller);
        controller.connect(at(1), (TALKER, 0), (TALKER, 1));
        let frames = transmits(&mut controller);
        let connect = Acmpdu::decode(&frames[0].1).unwrap();
        assert_eq!(connect.message_type, AcmpMessageType::BIND_RX_COMMAND);
        connect.flags.contains(AcmpFlags::CLASS_B)
    };
    assert!(class_of_connect(true));
    assert!(!class_of_connect(false));
}

#[test]
fn unanswered_binds_use_milan_timeouts() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    entity.answers_acmp = false;
    let connect = controller.connect(at(1), (OTHER_TALKER, 0), (TALKER, 0));
    assert_eq!(transmits(&mut controller).len(), 1);
    // A Milan entity: 200 ms, then a retry, then the outcome.
    controller.handle_timeout(after(at(1), 199));
    assert!(transmits(&mut controller).is_empty());
    controller.handle_timeout(after(at(1), 200));
    assert_eq!(transmits(&mut controller).len(), 1);
    controller.handle_timeout(after(at(1), 400));
    assert_eq!(finished(&mut controller), [(connect, Outcome::NoResponse)]);
}

#[test]
fn other_controllers_bindings_are_seen() {
    let (mut controller, _) = enumerated();
    events(&mut controller);
    let response = Acmpdu {
        message_type: AcmpMessageType::BIND_RX_RESPONSE,
        controller_entity_id: EntityId(0x0001_f2ff_fe00_0001),
        talker_entity_id: OTHER_TALKER,
        talker_unique_id: 3,
        listener_entity_id: TALKER,
        listener_unique_id: 0,
        connection_count: 1,
        flags: AcmpFlags::empty(),
        sequence_id: 900,
        ..Acmpdu::default()
    };
    let mut out = [0; 96];
    let length = response.encode(&mut out).unwrap();
    controller
        .handle_frame(at(1), TALKER_MAC, &out[..length])
        .unwrap();
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    let model = controller.model(TALKER).unwrap();
    assert_eq!(
        model.binding(0).unwrap().talker_stream(),
        Some((OTHER_TALKER, 3))
    );
    // The input's stream info is read again.
    let frames = transmits(&mut controller);
    let query = AemPdu::decode(&frames[0].1).unwrap();
    assert_eq!(query.command_type, AemCommandType::GET_STREAM_INFO);
}

#[test]
fn identify_turns_the_control_on_then_off() {
    let mut controller = controller();
    let mut entity = FakeEntity::new();
    let mut adpdu = aem_available(1);
    adpdu.entity_capabilities |= EntityCapabilities::AEM_IDENTIFY_CONTROL_INDEX_VALID;
    adpdu.identify_control_index = 4;
    controller.handle_adpdu(at(0), TALKER_MAC, &adpdu);
    exchange(&mut controller, &mut entity, at(0));
    events(&mut controller);
    let identify = controller.identify(at(1), TALKER, Duration::from_secs(3));
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(finished(&mut controller), [(identify, Outcome::Done)]);
    assert_eq!(entity.controls, [(4, vec![255])]);
    controller.handle_timeout(at(4));
    exchange(&mut controller, &mut entity, at(4));
    assert_eq!(entity.controls, [(4, vec![255]), (4, vec![0])]);
    // Without an advertised identify control there is nothing to do.
    let mut plain = controller_with(available(OTHER_TALKER, 1));
    let none = plain.identify(at(1), OTHER_TALKER, Duration::from_secs(3));
    assert_eq!(finished(&mut plain), [(none, Outcome::NotPossible)]);
}

fn controller_with(adpdu: Adpdu) -> Controller {
    let mut controller = controller();
    controller.handle_adpdu(at(0), TALKER_MAC, &adpdu);
    events(&mut controller);
    controller
}

#[test]
fn stream_info_notifications_update_the_model() {
    let (mut controller, _) = enumerated();
    events(&mut controller);
    let mut payload = vec![0u8; 48];
    put(&mut payload, 0, DescriptorType::STREAM_INPUT.0);
    put(&mut payload, 2, 1);
    payload[4..8].copy_from_slice(
        &(StreamInfoFlags::CONNECTED | StreamInfoFlags::SRP_REGISTRATION_FAILED)
            .0
            .to_be_bytes(),
    );
    let pdu = AemPdu {
        header: AecpHeader {
            message_type: AecpMessageType::AEM_RESPONSE,
            status: 0,
            target_entity_id: TALKER,
            controller_entity_id: CONTROLLER,
            sequence_id: 5,
        },
        unsolicited: true,
        controller_request: false,
        command_type: AemCommandType::GET_STREAM_INFO,
        payload: &payload,
    };
    let mut out = [0; 96];
    let length = pdu.encode(&mut out).unwrap();
    controller
        .handle_frame(at(1), TALKER_MAC, &out[..length])
        .unwrap();
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    let info = *controller
        .model(TALKER)
        .unwrap()
        .stream_info(DescriptorType::STREAM_INPUT, 1)
        .unwrap();
    assert!(info.bound() && info.talker_failed());
    assert!(
        transmits(&mut controller).is_empty(),
        "nothing to read again"
    );
}

/// The AEM commands the entity saw from `from` on.
fn commands_since(entity: &FakeEntity, from: usize) -> Vec<AemCommandType> {
    entity.commands[from..].to_vec()
}

#[test]
fn renames_the_entity_its_group_and_descriptors() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let rename = controller.set_name(at(1), TALKER, DescriptorType::ENTITY, 0, 0, "Stage left");
    let group = controller.set_name(at(1), TALKER, DescriptorType::ENTITY, 0, 1, "Stage");
    let input = controller.set_name(at(1), TALKER, DescriptorType::STREAM_INPUT, 1, 0, "Vocal");
    let seen = entity.commands.len();
    exchange(&mut controller, &mut entity, at(1));
    // The response tells the new names; nothing is read again.
    assert_eq!(commands_since(&entity, seen), [AemCommandType::SET_NAME; 3]);
    let events = events(&mut controller);
    assert!(events.contains(&Event::EntityModelChanged(TALKER)));
    let outcomes: Vec<(CommandId, Outcome)> = events
        .into_iter()
        .filter_map(|event| match event {
            Event::CommandFinished(command, outcome) => Some((command, outcome)),
            _ => None,
        })
        .collect();
    assert_eq!(
        outcomes,
        [
            (rename, Outcome::Done),
            (group, Outcome::Done),
            (input, Outcome::Done)
        ]
    );
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.entity_name(), Some("Stage left"));
    assert_eq!(model.entity().unwrap().group_name, "Stage");
    assert_eq!(
        model.name_of(DescriptorType::STREAM_INPUT, 1),
        Some("Vocal")
    );
    // An empty name falls back to the localized description again.
    controller.set_name(at(2), TALKER, DescriptorType::STREAM_INPUT, 1, 0, "");
    exchange(&mut controller, &mut entity, at(2));
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.name_of(DescriptorType::STREAM_INPUT, 1), None);
}

#[test]
fn names_too_long_and_entities_gone_are_not_possible() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let long = controller.set_name(at(1), TALKER, DescriptorType::ENTITY, 0, 0, &"x".repeat(65));
    let gone = controller.set_clock_source(at(1), EntityId(42), 0, 1);
    let missing = controller.set_name(at(1), TALKER, DescriptorType::JACK_INPUT, 9, 0, "Jack");
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(
        finished(&mut controller),
        [
            (long, Outcome::NotPossible),
            (gone, Outcome::NotPossible),
            (
                missing,
                Outcome::Refused(Refusal::Aem(AemStatus::NO_SUCH_DESCRIPTOR))
            ),
        ]
    );
}

#[test]
fn changes_a_stream_format_unless_it_runs() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let format = StreamFormat(0x0205_0220_0200_6000);
    let changed =
        controller.set_stream_format(at(1), TALKER, DescriptorType::STREAM_INPUT, 0, format);
    let seen = entity.commands.len();
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(finished(&mut controller), [(changed, Outcome::Done)]);
    // The stream's info is asked for again, as its format is part of it,
    // and the mappings, which may lose channels the new format lacks.
    assert_eq!(
        commands_since(&entity, seen),
        [
            AemCommandType::SET_STREAM_FORMAT,
            AemCommandType::GET_STREAM_INFO,
            AemCommandType::GET_AUDIO_MAP
        ]
    );
    let input = |controller: &Controller| {
        controller
            .model(TALKER)
            .unwrap()
            .streams(true)
            .find(|stream| stream.index == 0)
            .unwrap()
            .current_format
    };
    assert_eq!(input(&controller), format);

    entity.streaming = true;
    let refused = controller.set_stream_format(
        at(2),
        TALKER,
        DescriptorType::STREAM_INPUT,
        0,
        StreamFormat(0x0205_0420_0200_6000),
    );
    exchange(&mut controller, &mut entity, at(2));
    assert_eq!(
        finished(&mut controller),
        [(
            refused,
            Outcome::Refused(Refusal::Aem(AemStatus::STREAM_IS_RUNNING))
        )]
    );
    assert_eq!(input(&controller), format);
}

#[test]
fn a_new_sampling_rate_reads_the_streams_again() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let rate = SamplingRate(96_000);
    let changed = controller.set_sampling_rate(at(1), TALKER, DescriptorType::AUDIO_UNIT, 0, rate);
    let seen = entity.commands.len();
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(finished(&mut controller), [(changed, Outcome::Done)]);
    let reads = commands_since(&entity, seen)
        .iter()
        .filter(|&&command| command == AemCommandType::READ_DESCRIPTOR)
        .count();
    assert_eq!(reads, 3, "two stream inputs and a stream output");
    let model = controller.model(TALKER).unwrap();
    assert_eq!(
        model.audio_units().next().unwrap().current_sampling_rate,
        rate
    );
    assert!(
        model
            .streams(true)
            .chain(model.streams(false))
            .all(|stream| stream.current_format == StreamFormat(0x0205_0420_0200_6000))
    );
}

#[test]
fn picks_a_clock_source() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let picked = controller.set_clock_source(at(1), TALKER, 0, 1);
    let refused = controller.set_clock_source(at(1), TALKER, 0, 7);
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(
        finished(&mut controller),
        [
            (picked, Outcome::Done),
            (
                refused,
                Outcome::Refused(Refusal::Aem(AemStatus::BAD_ARGUMENTS))
            ),
        ]
    );
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.clock_domains().next().unwrap().clock_source_index, 1);
}

impl FakeEntity {
    /// Gives the entity a speaker volume control, CONTROL 0: a linear
    /// 16-bit gain from -100 dB to 6 dB in tenths, at -6 dB.
    fn add_volume(&mut self) {
        let configuration = self
            .descriptors
            .get_mut(&(DescriptorType::CONFIGURATION.0, 0))
            .unwrap();
        let slot = configuration.len();
        configuration.extend([0; 4]);
        let count = u16::from_be_bytes([configuration[70], configuration[71]]);
        put(configuration, 70, count + 1);
        put(configuration, slot, DescriptorType::CONTROL.0);
        put(configuration, slot + 2, 1);
        let mut volume = named(DescriptorType::CONTROL, 0, 118, "Volume", 0xffff);
        put(&mut volume, 80, crate::control::ValueType::LINEAR_INT16.0);
        volume[82..90].copy_from_slice(&crate::control::ControlType::GAIN.0.to_be_bytes());
        put(&mut volume, 94, 104);
        put(&mut volume, 96, 1);
        put(&mut volume, 98, DescriptorType::INVALID.0);
        volume[104..118].copy_from_slice(&[
            0xfc, 0x18, 0x00, 0x3c, 0x00, 0x05, 0x00, 0x00, 0xff, 0xc4, 0xff, 0xb0, 0xff, 0xff,
        ]);
        self.descriptors
            .insert((DescriptorType::CONTROL.0, 0), volume);
    }
}

/// An unsolicited SET_CONTROL from the entity, with a control's new values.
fn control_notification(index: u16, values: &[u8]) -> Vec<u8> {
    let mut payload = vec![0u8; 4];
    put(&mut payload, 0, DescriptorType::CONTROL.0);
    put(&mut payload, 2, index);
    payload.extend_from_slice(values);
    let pdu = AemPdu {
        header: AecpHeader {
            message_type: AecpMessageType::AEM_RESPONSE,
            status: 0,
            target_entity_id: TALKER,
            controller_entity_id: CONTROLLER,
            sequence_id: 4,
        },
        unsolicited: true,
        controller_request: false,
        command_type: AemCommandType::SET_CONTROL,
        payload: &payload,
    };
    let mut out = [0; 64];
    let length = pdu.encode(&mut out).unwrap();
    out[..length].to_vec()
}

#[test]
fn controls_are_set_and_kept_current_without_reading() {
    use crate::control::Number;

    let mut controller = controller();
    let mut entity = FakeEntity::new();
    entity.add_volume();
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    controller.pump(at(0));
    exchange(&mut controller, &mut entity, at(0));
    events(&mut controller);
    let current = |controller: &Controller| {
        let model = controller.model(TALKER).unwrap();
        model.control(0).unwrap().current().collect::<Vec<_>>()
    };
    assert_eq!(current(&controller), [Number::Int(-60)]);

    let set = controller.set_control(at(1), TALKER, 0, &(-120i16).to_be_bytes());
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(finished(&mut controller), [(set, Outcome::Done)]);
    assert_eq!(current(&controller), [Number::Int(-120)]);
    assert_eq!(
        entity.controls.last(),
        Some(&(0, (-120i16).to_be_bytes().to_vec()))
    );

    // Another controller turns it down: the notification carries the
    // value, so nothing is read again.
    let frame = control_notification(0, &(-300i16).to_be_bytes());
    controller.handle_frame(at(2), TALKER_MAC, &frame).unwrap();
    assert_eq!(exchange(&mut controller, &mut entity, at(2)), 0);
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    assert_eq!(current(&controller), [Number::Int(-300)]);

    // No values, or more than any control holds, are not sent.
    let empty = controller.set_control(at(3), TALKER, 0, &[]);
    let long = controller.set_control(at(3), TALKER, 0, &[0; 405]);
    assert_eq!(
        finished(&mut controller),
        [(empty, Outcome::NotPossible), (long, Outcome::NotPossible)]
    );
    assert!(controller.control_values.is_empty());
}

/// A listener that went away, the talker still sending to it.
const GONE_LISTENER: EntityId = EntityId(0x0011_22ff_fe33_4455);

#[test]
fn talkers_say_what_they_send_and_stop_when_told() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let asked = controller.tx_state(at(1), (TALKER, 0));
    let stopped = controller.disconnect_talker(at(1), (TALKER, 0), (GONE_LISTENER, 1));
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(
        finished(&mut controller),
        [(asked, Outcome::Done), (stopped, Outcome::Done)]
    );
    let state = controller
        .model(TALKER)
        .unwrap()
        .tx_state(0)
        .copied()
        .unwrap();
    assert_eq!(state.connection_count, 2);
    assert_eq!(state.vlan_id, 2);
    assert_eq!(
        state.destination,
        MacAddress([0x91, 0xe0, 0xf0, 0x00, 0xfe, 0x01])
    );
    assert_eq!(entity.tx_disconnects, [(0, GONE_LISTENER, 1)]);
}

#[test]
fn max_transit_times_are_read_and_set_while_not_streaming() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let time = |controller: &Controller| controller.model(TALKER).unwrap().max_transit_time(0);
    // Read with the rest of a Milan entity.
    assert_eq!(time(&controller), Some(2_000_000));
    let set = controller.set_max_transit_time(at(1), TALKER, 0, 1_500_000);
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(finished(&mut controller), [(set, Outcome::Done)]);
    assert_eq!(time(&controller), Some(1_500_000));
    entity.streaming = true;
    let refused = controller.set_max_transit_time(at(2), TALKER, 0, 500_000);
    exchange(&mut controller, &mut entity, at(2));
    assert_eq!(
        finished(&mut controller),
        [(
            refused,
            Outcome::Refused(Refusal::Aem(AemStatus::STREAM_IS_RUNNING))
        )]
    );
    assert_eq!(time(&controller), Some(1_500_000), "the old time kept");
}

/// An entity outside Milan may answer GET_MAX_TRANSIT_TIME too, as
/// macOS's does: asked for its first output's, then, once it answers, the
/// others'. One that does not is asked no more.
#[test]
fn max_transit_times_are_read_outside_milan_once_answered() {
    let read = |answers: bool| {
        let mut controller = controller();
        let mut entity = FakeEntity::new().with_outputs(3);
        entity.milan = false;
        entity.answers_transit = answers;
        controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
        controller.pump(at(0));
        exchange(&mut controller, &mut entity, at(0));
        let asked = entity
            .commands
            .iter()
            .filter(|&&command| command == AemCommandType::GET_MAX_TRANSIT_TIME)
            .count();
        let model = controller.model(TALKER).unwrap();
        assert!(model.milan.is_none());
        let times: Vec<Option<u64>> = (0..3)
            .map(|output| model.max_transit_time(output))
            .collect();
        (asked, times)
    };
    assert_eq!(read(true), (3, vec![Some(2_000_000); 3]));
    assert_eq!(read(false), (1, vec![None; 3]));
}

#[test]
fn descriptors_are_read_on_request() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    entity.rename(DescriptorType::STREAM_INPUT, 1, "Vocal");
    let read = controller.read_descriptor(at(1), TALKER, DescriptorType::STREAM_INPUT, 1);
    let missing = controller.read_descriptor(at(1), TALKER, DescriptorType::STREAM_INPUT, 9);
    exchange(&mut controller, &mut entity, at(1));
    assert_eq!(
        finished(&mut controller),
        [
            (read, Outcome::Done),
            (
                missing,
                Outcome::Refused(Refusal::Aem(AemStatus::NO_SUCH_DESCRIPTOR))
            ),
        ]
    );
    let model = controller.model(TALKER).unwrap();
    assert_eq!(
        model.name_of(DescriptorType::STREAM_INPUT, 1),
        Some("Vocal")
    );
}

fn lite_status() -> crate::lite::LiteStatus {
    use crate::lite::{FallbackReason, LiteFlags, PtpProfile};
    crate::lite::LiteStatus {
        interface: 0,
        flags: LiteFlags::CAPABLE | LiteFlags::ACTIVE | LiteFlags::OFFSET_VALID,
        fallback_reason: FallbackReason::MULTIPLE_RESPONDERS,
        ptp_profile: PtpProfile::AVB_LITE_PTP,
        ptp_domain: 0,
        media_vlan_id: 2,
        unicast_fanout_limit: 2,
        link_speed: 1000,
        committed_egress: 0,
        grandmaster: GRANDMASTER,
        offset_from_grandmaster: 180,
    }
}

/// Reads the fake entity, with AVB Lite when `lite` says what it reports.
fn read_lite(lite: Option<crate::lite::LiteStatus>) -> (Controller, FakeEntity) {
    let mut controller = controller();
    let mut entity = FakeEntity::new();
    entity.lite = lite;
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    controller.pump(at(0));
    exchange(&mut controller, &mut entity, at(0));
    events(&mut controller);
    (controller, entity)
}

#[test]
fn lite_endpoints_report_their_mode_and_are_asked_again() {
    let (mut controller, mut entity) = read_lite(Some(lite_status()));
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.lite_supported, Some(true));
    assert_eq!(model.lite_status(0), Some(&lite_status()));
    // Asked again every 5 s, to follow the offset.
    let next = controller.poll_timeout().unwrap();
    assert!(next <= at(5), "{next:?}");
    entity.lite = Some(crate::lite::LiteStatus {
        offset_from_grandmaster: 72_000,
        ..lite_status()
    });
    controller.handle_timeout(next);
    exchange(&mut controller, &mut entity, next);
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    let offset = controller
        .model(TALKER)
        .unwrap()
        .lite_status(0)
        .unwrap()
        .offset();
    assert_eq!(offset, Some(72_000));
}

#[test]
fn entities_without_lite_are_asked_once() {
    let (controller, _) = read_lite(None);
    assert_eq!(
        controller.model(TALKER).unwrap().lite_supported,
        Some(false)
    );
    assert!(controller.model(TALKER).unwrap().lite_status(0).is_none());
    // Nothing to poll: the entity's advertisement expiry comes first.
    assert_eq!(controller.poll_timeout(), Some(at(20)));
}

#[test]
fn unsolicited_lite_status_updates_the_model() {
    let (mut controller, _) = read_lite(Some(lite_status()));
    let changed = crate::lite::LiteStatus {
        fallback_reason: crate::lite::FallbackReason::CONFIGURED,
        ..lite_status()
    };
    let mut payload = 0x8000u16.to_be_bytes().to_vec();
    payload.extend_from_slice(&changed.to_bytes());
    let notification = VendorUniquePdu {
        header: AecpHeader {
            message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
            status: 0,
            target_entity_id: TALKER,
            controller_entity_id: CONTROLLER,
            sequence_id: 40,
        },
        protocol_id: crate::lite::STATUS_PROTOCOL_ID,
        payload: &payload,
    };
    let mut out = [0; 128];
    let length = notification.encode(&mut out).unwrap();
    controller
        .handle_frame(at(1), TALKER_MAC, &out[..length])
        .unwrap();
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    assert_eq!(
        controller.model(TALKER).unwrap().lite_status(0),
        Some(&changed)
    );
}

fn wireless_station() -> Wireless {
    use crate::wireless::{AsCapableReason, Band, TimeMode, WirelessFlags};
    Wireless {
        status: WirelessStatus {
            interface: 0,
            flags: WirelessFlags::LOCKED | WirelessFlags::RTT_VALID,
            time_mode: TimeMode::MODE_A_FTM,
            band: Band::GHZ_5,
            channel: 36,
            channel_width: 80,
            phy_generation: 6,
            rssi: -52,
            phy_rate: 866,
            ftm_success: 98,
            as_capable_reason: AsCapableReason::NONE,
            ftm_burst_frames: 3,
            listeners_unserved: 0,
            ftm_burst_duration: 7,
            ftm_min_delta: 100,
            ftm_rtt: 42,
            servo_error: 0,
            association_age: 60,
            time_age: 50,
            ap_resets: 0,
            bssid: MacAddress([0x30, 0xed, 0xa0, 0x11, 0x22, 0x33]),
            ap_clock: BRIDGE,
            ap_port: 2,
            downlink_readdressed: 0,
            downlink_unmapped: 0,
            downlink_dropped: 0,
            uplink_restored: 0,
            frame_budget: 0,
            frames_committed: 0,
            station_count: 0,
        },
        stations: Vec::new(),
    }
}

/// An access point's wireless port, with one station associated.
fn wireless_access_point() -> Wireless {
    use crate::wireless::{Station, StationFlags, WirelessFlags};
    let station = wireless_station().status;
    Wireless {
        status: WirelessStatus {
            flags: WirelessFlags::ACCESS_POINT,
            rssi: 0x7f,
            listeners_unserved: 1,
            downlink_readdressed: 40_000,
            station_count: 1,
            ..station
        },
        stations: vec![Station {
            mac: MacAddress([0xfc, 0x01, 0x2c, 0xfd, 0x80, 0x00]),
            rssi: -48,
            flags: StationFlags::FTM_INITIATOR | StationFlags::FTM_KNOWN,
        }],
    }
}

/// Reads the fake entity, with AVB Wireless on interface 0 when
/// `wireless` says what it reports.
fn read_wireless(wireless: Option<Wireless>) -> (Controller, FakeEntity) {
    let mut controller = controller();
    let mut entity = FakeEntity::new();
    entity.wireless = wireless;
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    controller.pump(at(0));
    exchange(&mut controller, &mut entity, at(0));
    events(&mut controller);
    (controller, entity)
}

#[test]
fn wireless_stations_report_their_link_and_are_asked_again() {
    let (mut controller, mut entity) = read_wireless(Some(wireless_station()));
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.wireless_supported, Some(true));
    assert_eq!(model.wireless(0), Some(&wireless_station()));
    assert_eq!(model.wireless_interfaces().count(), 1);
    // Asked again every 5 s, to follow the link and the time.
    let next = controller.poll_timeout().unwrap();
    assert!(next <= at(5), "{next:?}");
    entity.wireless.as_mut().unwrap().status.flags = crate::wireless::WirelessFlags::HOLDOVER;
    controller.handle_timeout(next);
    exchange(&mut controller, &mut entity, next);
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    let status = controller
        .model(TALKER)
        .unwrap()
        .wireless(0)
        .unwrap()
        .status;
    assert!(
        !status
            .flags
            .contains(crate::wireless::WirelessFlags::LOCKED)
    );
}

#[test]
fn entities_without_wireless_are_asked_once() {
    let (controller, _) = read_wireless(None);
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.wireless_supported, Some(false));
    assert!(model.wireless(0).is_none());
    // Nothing to poll: the entity's advertisement expiry comes first.
    assert_eq!(controller.poll_timeout(), Some(at(20)));
}

/// A station whose answers the Wi-Fi hop loses at first is asked again at
/// the next poll, and followed once it answers.
#[test]
fn unanswered_wireless_queries_are_asked_again() {
    let mut controller = controller();
    let mut entity = FakeEntity::new();
    entity.wireless = Some(wireless_station());
    // Its first query and the retry go unanswered.
    entity.wireless_silent = 2;
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    controller.pump(at(0));
    let mut now = at(0);
    for _ in 0..40 {
        exchange(&mut controller, &mut entity, now);
        let Some(next) = controller.poll_timeout().filter(|next| *next <= at(12)) else {
            break;
        };
        now = next;
        controller.handle_timeout(now);
    }
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.wireless_supported, Some(true));
    assert_eq!(model.wireless(0), Some(&wireless_station()));
}

/// An entity that never answers the query is given up after a few polls.
#[test]
fn silent_entities_are_given_up_on() {
    let mut controller = controller();
    let mut entity = FakeEntity::new();
    entity.wireless = Some(wireless_station());
    entity.wireless_silent = u8::MAX;
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    controller.pump(at(0));
    let mut now = at(0);
    for _ in 0..200 {
        exchange(&mut controller, &mut entity, now);
        let Some(next) = controller.poll_timeout().filter(|next| *next <= at(19)) else {
            break;
        };
        now = next;
        controller.handle_timeout(now);
    }
    assert_eq!(
        controller.model(TALKER).unwrap().wireless_supported,
        Some(false)
    );
}

#[test]
fn wired_interfaces_are_not_asked_again() {
    // The entity has AVB Wireless, but its interface 0 is wired.
    let mut elsewhere = wireless_station();
    elsewhere.status.interface = 1;
    let (controller, _) = read_wireless(Some(elsewhere));
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.wireless_supported, Some(true));
    assert_eq!(model.wireless_interfaces().count(), 0);
    assert_eq!(controller.poll_timeout(), Some(at(20)));
}

#[test]
fn an_access_point_reports_its_stations_and_unsolicited_changes() {
    let (mut controller, _) = read_wireless(Some(wireless_access_point()));
    let wireless = controller
        .model(TALKER)
        .unwrap()
        .wireless(0)
        .unwrap()
        .clone();
    assert_eq!(wireless, wireless_access_point());
    assert_eq!(wireless.status.unserved(), Some(1));
    // A station leaves: the access point says so unsolicited.
    let mut changed = wireless_access_point();
    changed.status.station_count = 0;
    changed.stations.clear();
    let mut payload = 0x8000u16.to_be_bytes().to_vec();
    payload.extend_from_slice(&changed.status.to_bytes());
    let notification = VendorUniquePdu {
        header: AecpHeader {
            message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
            status: 0,
            target_entity_id: TALKER,
            controller_entity_id: CONTROLLER,
            sequence_id: 41,
        },
        protocol_id: crate::wireless::WIRELESS_PROTOCOL_ID,
        payload: &payload,
    };
    let mut out = [0; 128];
    let length = notification.encode(&mut out).unwrap();
    controller
        .handle_frame(at(1), TALKER_MAC, &out[..length])
        .unwrap();
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    assert_eq!(
        controller.model(TALKER).unwrap().wireless(0),
        Some(&changed)
    );
}

#[test]
fn class_a_is_allowed_over_the_wireless_port() {
    let (mut controller, mut entity) = read_wireless(Some(wireless_access_point()));
    let command = controller.set_wireless_config(at(1), TALKER, 0, true);
    exchange(&mut controller, &mut entity, at(1));
    let events = events(&mut controller);
    assert!(
        events.contains(&Event::CommandFinished(command, Outcome::Done)),
        "{events:?}"
    );
    // The status, read again, shows it.
    let status = controller
        .model(TALKER)
        .unwrap()
        .wireless(0)
        .unwrap()
        .status;
    assert!(
        status
            .flags
            .contains(crate::wireless::WirelessFlags::CLASS_A_ALLOWED)
    );
}

/// A CVU SRP talker message from the entity declaring its stream output,
/// joining or, with `leave`, withdrawing it.
fn cvu_talker(leave: bool) -> Vec<u8> {
    let mut msrp = vec![1, 25, 0, 0, 0x00, 0x01];
    msrp.extend_from_slice(&0xe8f6_0ae0_9220_0000u64.to_be_bytes());
    msrp.extend_from_slice(&[0x91, 0xe0, 0xf0, 0x00, 0x6a, 0x20]);
    msrp.extend_from_slice(&2u16.to_be_bytes());
    msrp.extend_from_slice(&224u16.to_be_bytes());
    msrp.extend_from_slice(&1u16.to_be_bytes());
    msrp.push(0xb0);
    msrp.extend_from_slice(&500_000u32.to_be_bytes());
    msrp.push(if leave { 5 * 36 } else { 36 });
    msrp.extend_from_slice(&[0, 0]);
    let list = (msrp.len() - 4) as u16;
    msrp[2..4].copy_from_slice(&list.to_be_bytes());
    let mut payload = vec![1];
    payload.extend_from_slice(&msrp);
    let pdu = VendorUniquePdu {
        header: AecpHeader {
            message_type: AecpMessageType::VENDOR_UNIQUE_COMMAND,
            status: 0,
            target_entity_id: EntityId(0),
            controller_entity_id: TALKER,
            sequence_id: 9,
        },
        protocol_id: crate::lite::CVU_PROTOCOL_ID,
        payload: &payload,
    };
    let mut out = [0; 128];
    let length = pdu.encode(&mut out).unwrap();
    out[..length].to_vec()
}

#[test]
fn cvu_declarations_are_kept_until_withdrawn_or_stale() {
    let (mut controller, _) = read_lite(Some(lite_status()));
    let streams = |controller: &Controller| controller.model(TALKER).unwrap().cvu_talkers().count();
    controller
        .handle_frame(at(1), TALKER_MAC, &cvu_talker(false))
        .unwrap();
    assert_eq!(streams(&controller), 1);
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    let declared = controller
        .model(TALKER)
        .unwrap()
        .cvu_talkers()
        .next()
        .copied()
        .unwrap();
    assert_eq!(declared.vlan_id, 2);
    // A refresh changes nothing.
    controller
        .handle_frame(at(2), TALKER_MAC, &cvu_talker(false))
        .unwrap();
    assert!(events(&mut controller).is_empty());
    // Withdrawn.
    controller
        .handle_frame(at(3), TALKER_MAC, &cvu_talker(true))
        .unwrap();
    assert_eq!(streams(&controller), 0);
    // Declared again, then not refreshed for 30 s.
    controller
        .handle_frame(at(4), TALKER_MAC, &cvu_talker(false))
        .unwrap();
    // The entity keeps advertising meanwhile.
    controller.handle_adpdu(at(19), TALKER_MAC, &aem_available(1));
    events(&mut controller);
    controller.handle_timeout(at(33));
    assert_eq!(streams(&controller), 1);
    controller.handle_timeout(at(34));
    assert_eq!(streams(&controller), 0);
}

/// Reads the entity with a controller that knows a model cached from an
/// earlier reading, returning the controller and the commands the entity
/// saw while being read.
fn read_with_cache(
    cached: CachedModel,
    entity: &mut FakeEntity,
    adpdu: Adpdu,
) -> (Controller, Vec<AemCommandType>) {
    let mut controller = controller();
    controller.remember_model(cached);
    let seen = entity.commands.len();
    controller.handle_adpdu(at(0), TALKER_MAC, &adpdu);
    controller.pump(at(0));
    exchange(&mut controller, entity, at(0));
    let commands = commands_since(entity, seen);
    (controller, commands)
}

fn reads_of(commands: &[AemCommandType]) -> usize {
    commands
        .iter()
        .filter(|&&command| command == AemCommandType::READ_DESCRIPTOR)
        .count()
}

#[test]
fn cached_models_read_only_what_changes() {
    let (first, mut entity) = enumerated();
    let cached = first
        .static_model(TALKER)
        .expect("a complete model to keep");
    let full_reads = reads_of(&entity.commands);
    // Changed while the controller was away: a stream's name and format.
    entity.rename(DescriptorType::STREAM_INPUT, 1, "Vocal");
    let format = StreamFormat(0x0205_0220_0200_6000);
    entity.stored(DescriptorType::STREAM_INPUT, 1)[74..82].copy_from_slice(&format.0.to_be_bytes());

    let (controller, commands) = read_with_cache(cached, &mut entity, aem_available(1));
    let model = controller.model(TALKER).unwrap();
    assert!(model.from_cache);
    assert_eq!(model.state, EnumerationState::Complete);
    // The ENTITY, the CONFIGURATION and the AVB interface, which holds what
    // differs between entities; the rest with GET_DYNAMIC_INFO, two of them
    // as the answers for eight names do not fit in one response.
    assert_eq!(reads_of(&commands), 3, "{commands:?}");
    assert!(reads_of(&commands) < full_reads);
    assert_eq!(
        commands
            .iter()
            .filter(|&&command| command == AemCommandType::GET_DYNAMIC_INFO)
            .count(),
        2
    );
    assert_eq!(
        model.name_of(DescriptorType::STREAM_INPUT, 1),
        Some("Vocal")
    );
    let input = model
        .streams(true)
        .find(|stream| stream.index == 1)
        .unwrap();
    assert_eq!(input.current_format, format);
    // Everything else is as a full reading gives it.
    assert_eq!(
        model.descriptor_count(),
        first.model(TALKER).unwrap().descriptor_count()
    );
    assert_eq!(
        model.localized(model.entity().unwrap().vendor_name),
        Some("Acme")
    );
}

#[test]
fn entities_without_dynamic_info_read_what_changes_instead() {
    let (first, mut entity) = enumerated();
    let cached = first.static_model(TALKER).unwrap();
    entity.answers_dynamic_info = false;
    entity.rename(DescriptorType::AUDIO_CLUSTER, 0, "Kick");
    let (controller, commands) = read_with_cache(cached, &mut entity, aem_available(1));
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.state, EnumerationState::Complete);
    assert_eq!(
        model.name_of(DescriptorType::AUDIO_CLUSTER, 0),
        Some("Kick")
    );
    // Strings, locale, map and port were not read again.
    assert!(
        reads_of(&commands) < reads_of(&entity.commands[..entity.commands.len() - commands.len()])
    );
    assert!(model.from_cache);
}

#[test]
fn a_model_that_does_not_match_is_read_in_full() {
    let (first, mut entity) = enumerated();
    let mut cached = first.static_model(TALKER).unwrap();
    // The cached configuration lists one stream input fewer.
    for (descriptor_type, index, bytes) in &mut cached.descriptors {
        if *descriptor_type == DescriptorType::CONFIGURATION && *index == 0 {
            put(bytes, 82, 1);
        }
    }
    let full_reads = reads_of(&entity.commands);
    let (controller, commands) = read_with_cache(cached, &mut entity, aem_available(1));
    let model = controller.model(TALKER).unwrap();
    assert!(!model.from_cache);
    assert_eq!(reads_of(&commands), full_reads);
}

#[test]
fn entities_not_milan_read_what_changes() {
    let (first, mut entity) = enumerated();
    let cached = first.static_model(TALKER).unwrap();
    let mut adpdu = aem_available(1);
    adpdu.entity_capabilities = EntityCapabilities::AEM_SUPPORTED;
    let (controller, commands) = read_with_cache(cached, &mut entity, adpdu);
    assert!(controller.model(TALKER).unwrap().from_cache);
    assert!(!commands.contains(&AemCommandType::GET_DYNAMIC_INFO));
}

const INPUT_PORT: (DescriptorType, u16) = (DescriptorType::STREAM_PORT_INPUT, 0);

fn mapping(stream_index: u16, stream_channel: u16, cluster_offset: u16) -> AudioMapping {
    AudioMapping {
        stream_index,
        stream_channel,
        cluster_offset,
        cluster_channel: 0,
    }
}

fn input_mappings(controller: &Controller) -> Option<Vec<AudioMapping>> {
    let model = controller.model(TALKER)?;
    Some(
        model
            .dynamic_mappings(INPUT_PORT.0, INPUT_PORT.1)?
            .collect(),
    )
}

#[test]
fn reads_dynamic_mappings_in_parts_after_enumeration() {
    let mut controller = controller();
    let mut entity = FakeEntity::new();
    entity.mappings = vec![mapping(0, 1, 0), mapping(1, 0, 1)];
    entity.map_parts = 2;
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    controller.pump(at(0));
    exchange(&mut controller, &mut entity, at(0));
    assert_eq!(
        input_mappings(&controller),
        Some(vec![mapping(0, 1, 0), mapping(1, 0, 1)])
    );
    // Both parts of the input's map; the output's is fixed, so not asked.
    let reads = entity
        .commands
        .iter()
        .filter(|&&command| command == AemCommandType::GET_AUDIO_MAP)
        .count();
    assert_eq!(reads, 2);
    let model = controller.model(TALKER).unwrap();
    assert!(
        model
            .dynamic_mappings(DescriptorType::STREAM_PORT_OUTPUT, 0)
            .is_none()
    );
    let output = model.stream_ports(false).next().unwrap();
    assert_eq!(
        model.static_mappings(&output).collect::<Vec<_>>(),
        [mapping(0, 0, 0)]
    );
    let clusters: Vec<_> = model
        .audio_clusters(&output)
        .map(|(offset, cluster)| (offset, cluster.object_name, cluster.channel_count))
        .collect();
    assert_eq!(clusters, [(0, "Out", 1)]);
    assert_eq!(
        entity.commands.last(),
        Some(&AemCommandType::REGISTER_UNSOLICITED_NOTIFICATION)
    );
}

#[test]
fn mappings_are_not_read_when_turned_off() {
    let mut config = Config::new(CONTROLLER);
    config.read_mappings = false;
    let mut controller = Controller::new(config);
    let mut entity = FakeEntity::new();
    controller.handle_adpdu(at(0), TALKER_MAC, &aem_available(1));
    controller.pump(at(0));
    exchange(&mut controller, &mut entity, at(0));
    assert!(!entity.commands.contains(&AemCommandType::GET_AUDIO_MAP));
    assert_eq!(input_mappings(&controller), None);
}

#[test]
fn adds_and_removes_mappings() {
    let (mut controller, mut entity) = enumerated();
    assert_eq!(input_mappings(&controller), Some(vec![]));
    events(&mut controller);

    let added = controller.add_audio_mappings(at(1), TALKER, INPUT_PORT, &[mapping(1, 3, 0)]);
    exchange(&mut controller, &mut entity, at(1));
    let events = events(&mut controller);
    assert!(events.contains(&Event::CommandFinished(added, Outcome::Done)));
    assert!(events.contains(&Event::EntityModelChanged(TALKER)));
    assert_eq!(input_mappings(&controller), Some(vec![mapping(1, 3, 0)]));

    // A new source for the same cluster channel replaces the old one; the
    // map read after the change shows it.
    let replaced = controller.add_audio_mappings(at(2), TALKER, INPUT_PORT, &[mapping(0, 5, 0)]);
    exchange(&mut controller, &mut entity, at(2));
    assert_eq!(finished(&mut controller), [(replaced, Outcome::Done)]);
    assert_eq!(input_mappings(&controller), Some(vec![mapping(0, 5, 0)]));

    let removed = controller.remove_audio_mappings(at(3), TALKER, INPUT_PORT, &[mapping(0, 5, 0)]);
    exchange(&mut controller, &mut entity, at(3));
    assert_eq!(finished(&mut controller), [(removed, Outcome::Done)]);
    assert_eq!(input_mappings(&controller), Some(vec![]));
    assert!(entity.mappings.is_empty());
}

#[test]
fn refused_and_impossible_mapping_changes() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let sent = entity.commands.len();
    let empty = controller.add_audio_mappings(at(1), TALKER, INPUT_PORT, &[]);
    let too_many = controller.add_audio_mappings(
        at(1),
        TALKER,
        INPUT_PORT,
        &[mapping(0, 0, 0); aem::MAX_MAPPINGS_PER_CHANGE + 1],
    );
    assert_eq!(
        finished(&mut controller),
        [
            (empty, Outcome::NotPossible),
            (too_many, Outcome::NotPossible)
        ]
    );
    assert_eq!(entity.commands.len(), sent, "nothing sent");

    // A cluster the port does not have.
    let refused = controller.add_audio_mappings(at(2), TALKER, INPUT_PORT, &[mapping(0, 0, 7)]);
    // A port whose mappings are fixed.
    let fixed = controller.add_audio_mappings(
        at(2),
        TALKER,
        (DescriptorType::STREAM_PORT_OUTPUT, 0),
        &[mapping(0, 0, 0)],
    );
    exchange(&mut controller, &mut entity, at(2));
    assert_eq!(
        finished(&mut controller),
        [
            (
                refused,
                Outcome::Refused(Refusal::Aem(AemStatus::BAD_ARGUMENTS))
            ),
            (
                fixed,
                Outcome::Refused(Refusal::Aem(AemStatus::NOT_SUPPORTED))
            ),
        ]
    );
    assert_eq!(input_mappings(&controller), Some(vec![]));
    assert!(controller.mapping_changes.is_empty());
}

fn mappings_notification(command_type: AemCommandType, mappings: &[AudioMapping]) -> Vec<u8> {
    let mut out = [0; 600];
    let length = aem::encode_audio_mappings(
        Addressing {
            target: TALKER,
            controller: CONTROLLER,
            sequence_id: 9,
        },
        if command_type == AemCommandType::ADD_AUDIO_MAPPINGS {
            MappingChange::Add
        } else {
            MappingChange::Remove
        },
        INPUT_PORT.0,
        INPUT_PORT.1,
        mappings,
        &mut out,
    )
    .unwrap();
    let mut frame = out[..length].to_vec();
    // A response, sent unsolicited.
    let command = AemPdu::decode(&frame).unwrap();
    let pdu = AemPdu {
        header: AecpHeader {
            message_type: AecpMessageType::AEM_RESPONSE,
            ..command.header
        },
        unsolicited: true,
        ..command
    };
    let length = pdu.encode(&mut out).unwrap();
    frame = out[..length].to_vec();
    frame
}

#[test]
fn mapping_notifications_change_the_model_without_reading() {
    let (mut controller, mut entity) = enumerated();
    events(&mut controller);
    let added = mappings_notification(
        AemCommandType::ADD_AUDIO_MAPPINGS,
        &[mapping(0, 2, 0), mapping(0, 3, 1)],
    );
    controller.handle_frame(at(1), TALKER_MAC, &added).unwrap();
    assert_eq!(exchange(&mut controller, &mut entity, at(1)), 0);
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    assert_eq!(
        input_mappings(&controller),
        Some(vec![mapping(0, 2, 0), mapping(0, 3, 1)])
    );
    let removed = mappings_notification(AemCommandType::REMOVE_AUDIO_MAPPINGS, &[mapping(0, 2, 0)]);
    controller
        .handle_frame(at(2), TALKER_MAC, &removed)
        .unwrap();
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    assert_eq!(input_mappings(&controller), Some(vec![mapping(0, 3, 1)]));
    // Hearing the same again changes nothing.
    controller
        .handle_frame(at(3), TALKER_MAC, &removed)
        .unwrap();
    assert!(events(&mut controller).is_empty());
}

#[test]
fn a_new_stream_format_reads_the_mappings_again() {
    let (mut controller, mut entity) = enumerated();
    let seen = entity.commands.len();
    controller.set_stream_format(
        at(1),
        TALKER,
        DescriptorType::STREAM_INPUT,
        0,
        StreamFormat(0x0205_0220_0200_6000),
    );
    exchange(&mut controller, &mut entity, at(1));
    assert!(commands_since(&entity, seen).contains(&AemCommandType::GET_AUDIO_MAP));
}

#[test]
fn reads_each_domains_media_clock_reference() {
    let (mut controller, entity) = enumerated();
    let model = controller.model(TALKER).unwrap();
    let reference = model.media_clock_reference(0).expect("read");
    assert_eq!(reference.default_priority, 192);
    assert_eq!(reference.priority(), 192);
    assert_eq!(reference.domain_name(), Some("DEFAULT"));
    assert_eq!(
        entity.commands.last(),
        Some(&AemCommandType::REGISTER_UNSOLICITED_NOTIFICATION)
    );
    events(&mut controller);

    // Another controller raises the priority.
    let mut data = vec![0u8; 76];
    data[1] = 0x03; // SET_MEDIA_CLOCK_REFERENCE_INFO
    data[0] = 0x80; // unsolicited
    data[4] = 0x03;
    data[6] = 192;
    data[7] = 240;
    data[12..19].copy_from_slice(b"DEFAULT");
    let notification = VendorUniquePdu {
        header: AecpHeader {
            message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
            status: 0,
            target_entity_id: TALKER,
            controller_entity_id: CONTROLLER,
            sequence_id: 4,
        },
        protocol_id: MVU_PROTOCOL_ID,
        payload: &data,
    };
    let mut out = [0; 128];
    let length = notification.encode(&mut out).unwrap();
    controller
        .handle_frame(at(1), TALKER_MAC, &out[..length])
        .unwrap();
    assert_eq!(events(&mut controller), [Event::EntityModelChanged(TALKER)]);
    let reference = controller
        .model(TALKER)
        .unwrap()
        .media_clock_reference(0)
        .unwrap();
    assert_eq!(reference.priority(), 240);
}

#[test]
fn entities_not_milan_are_not_asked_for_media_clock_references() {
    let mut adpdu = aem_available(1);
    adpdu.entity_capabilities = EntityCapabilities::AEM_SUPPORTED;
    let mut controller = controller_with(adpdu);
    let mut entity = FakeEntity::new();
    exchange(&mut controller, &mut entity, at(0));
    let model = controller.model(TALKER).unwrap();
    assert_eq!(model.state, EnumerationState::Complete);
    assert!(model.media_clock_reference(0).is_none());
}

#[test]
fn reads_the_counters_of_clocks_and_streams() {
    let (controller, _) = enumerated();
    let model = controller.model(TALKER).unwrap();
    for descriptor_type in [
        DescriptorType::CLOCK_DOMAIN,
        DescriptorType::STREAM_INPUT,
        DescriptorType::STREAM_OUTPUT,
    ] {
        assert!(
            model.counters(descriptor_type, 0).is_some(),
            "{descriptor_type:?}"
        );
    }
    // The fake marks counters 0, 1 and 5 valid.
    let input = model.stream_input_counters(0).unwrap();
    assert_eq!(input.media_locked, Some(1));
    assert_eq!(input.timestamp_uncertain, Some(2));
    assert_eq!(input.stream_interrupted, None);
    // A Milan 1.3 entity's output counters, numbered as 1722.1 has them.
    let output = model.stream_output_counters(0).unwrap();
    assert_eq!(output.stream_start, Some(1));
    assert_eq!(output.timestamp_uncertain, None);
    assert_eq!(model.clock_domain_counters(0).unwrap().locked, Some(1));
}
