//! The AEM commands the controller uses, built on [`AemPdu`].

use avb_net::MacAddress;

use crate::acmp::AcmpStatus;
use crate::aecp::{AecpHeader, AecpMessageType, AemCommandType, AemPdu, AemStatus};
use crate::avtp::{read_array, read_u16, read_u32, read_u64};
use crate::descriptor::{DescriptorType, SamplingRate};
use crate::error::{DecodeError, EncodeError};
use crate::id::{ClockIdentity, EntityId, StreamId};
use crate::macros::flags;
use crate::stream_format::StreamFormat;

/// Who a command goes to and comes from, and its sequence ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Addressing {
    pub target: EntityId,
    pub controller: EntityId,
    pub sequence_id: u16,
}

fn command<'a>(
    addressing: Addressing,
    command_type: AemCommandType,
    payload: &'a [u8],
) -> AemPdu<'a> {
    AemPdu {
        header: AecpHeader {
            message_type: AecpMessageType::AEM_COMMAND,
            status: AemStatus::SUCCESS.0,
            target_entity_id: addressing.target,
            controller_entity_id: addressing.controller,
            sequence_id: addressing.sequence_id,
        },
        unsolicited: false,
        controller_request: false,
        command_type,
        payload,
    }
}

/// Encodes READ_DESCRIPTOR for a descriptor of `configuration` (ignored
/// for ENTITY and CONFIGURATION, which the entity has once).
pub fn encode_read_descriptor(
    addressing: Addressing,
    configuration: u16,
    descriptor_type: DescriptorType,
    index: u16,
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    let configuration = match descriptor_type {
        DescriptorType::ENTITY | DescriptorType::CONFIGURATION => 0,
        _ => configuration,
    };
    let mut payload = [0; 8];
    payload[0..2].copy_from_slice(&configuration.to_be_bytes());
    payload[4..6].copy_from_slice(&descriptor_type.0.to_be_bytes());
    payload[6..8].copy_from_slice(&index.to_be_bytes());
    command(addressing, AemCommandType::READ_DESCRIPTOR, &payload).encode(out)
}

/// Encodes REGISTER_UNSOLICITED_NOTIFICATION in the 2013 form, without
/// the flags field, which 2021 entities must also accept.
pub fn encode_register_unsolicited(
    addressing: Addressing,
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    command(
        addressing,
        AemCommandType::REGISTER_UNSOLICITED_NOTIFICATION,
        &[],
    )
    .encode(out)
}

/// Encodes the response to a command sent to the controller, such as the
/// CONTROLLER_AVAILABLE entities send to check on registered controllers:
/// the command echoed back with `status`.
pub fn encode_response(
    command: &AemPdu<'_>,
    status: AemStatus,
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    AemPdu {
        header: AecpHeader {
            message_type: AecpMessageType::AEM_RESPONSE,
            status: status.0,
            ..command.header
        },
        unsolicited: false,
        controller_request: false,
        ..*command
    }
    .encode(out)
}

/// Encodes GET_STREAM_INFO for a STREAM_INPUT or STREAM_OUTPUT.
pub fn encode_get_stream_info(
    addressing: Addressing,
    descriptor_type: DescriptorType,
    index: u16,
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut payload = [0; 4];
    payload[0..2].copy_from_slice(&descriptor_type.0.to_be_bytes());
    payload[2..4].copy_from_slice(&index.to_be_bytes());
    command(addressing, AemCommandType::GET_STREAM_INFO, &payload).encode(out)
}

/// The most octets of values a control has (IEEE 1722.1-2021, Table
/// 7-38: value_details' maximum).
pub const MAX_CONTROL_VALUES: usize = 404;

/// Encodes SET_CONTROL with the control's new `values`, encoded as its
/// value type requires.
pub fn encode_set_control(
    addressing: Addressing,
    index: u16,
    values: &[u8],
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut payload = [0; 4 + MAX_CONTROL_VALUES];
    let length = 4 + values.len();
    let Some(body) = payload.get_mut(..length) else {
        return Err(EncodeError::OutOfRange("values"));
    };
    body[0..2].copy_from_slice(&DescriptorType::CONTROL.0.to_be_bytes());
    body[2..4].copy_from_slice(&index.to_be_bytes());
    body[4..].copy_from_slice(values);
    command(addressing, AemCommandType::SET_CONTROL, body).encode(out)
}

/// Encodes GET_AVB_INFO for an AVB_INTERFACE.
pub fn encode_get_avb_info(
    addressing: Addressing,
    index: u16,
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut payload = [0; 4];
    payload[0..2].copy_from_slice(&DescriptorType::AVB_INTERFACE.0.to_be_bytes());
    payload[2..4].copy_from_slice(&index.to_be_bytes());
    command(addressing, AemCommandType::GET_AVB_INFO, &payload).encode(out)
}

/// Encodes GET_AS_PATH for an AVB_INTERFACE.
pub fn encode_get_as_path(
    addressing: Addressing,
    index: u16,
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut payload = [0; 4];
    payload[0..2].copy_from_slice(&index.to_be_bytes());
    command(addressing, AemCommandType::GET_AS_PATH, &payload).encode(out)
}

/// Encodes GET_COUNTERS for an ENTITY, AVB_INTERFACE, CLOCK_DOMAIN,
/// STREAM_INPUT or STREAM_OUTPUT.
pub fn encode_get_counters(
    addressing: Addressing,
    descriptor_type: DescriptorType,
    index: u16,
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut payload = [0; 4];
    payload[0..2].copy_from_slice(&descriptor_type.0.to_be_bytes());
    payload[2..4].copy_from_slice(&index.to_be_bytes());
    command(addressing, AemCommandType::GET_COUNTERS, &payload).encode(out)
}

/// The longest name an entity holds, in octets of UTF-8.
pub const NAME_LENGTH: usize = 64;

/// `text` as AEM carries a name: its UTF-8 in 64 octets, padded with
/// zeros; `None` when it is longer.
pub fn aem_name(text: &str) -> Option<[u8; NAME_LENGTH]> {
    let bytes = text.as_bytes();
    let mut name = [0; NAME_LENGTH];
    name.get_mut(..bytes.len())?.copy_from_slice(bytes);
    Some(name)
}

fn truncated(needed: usize, payload: &[u8]) -> Result<(), DecodeError> {
    if payload.len() < needed {
        return Err(DecodeError::Truncated {
            needed,
            available: payload.len(),
        });
    }
    Ok(())
}

/// What SET_NAME carries, in its command and its response (7.4.17).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetName {
    pub descriptor_type: DescriptorType,
    pub index: u16,
    /// For the ENTITY, 0 is its entity_name and 1 its group_name; for
    /// other descriptors 0 is their object_name.
    pub name_index: u16,
    /// The configuration holding the descriptor; 0 for the ENTITY and
    /// CONFIGURATION.
    pub configuration: u16,
    pub name: [u8; NAME_LENGTH],
}

impl SetName {
    const LEN: usize = 8 + NAME_LENGTH;

    pub fn decode(payload: &[u8]) -> Result<Self, DecodeError> {
        truncated(Self::LEN, payload)?;
        Ok(Self {
            descriptor_type: DescriptorType(read_u16(payload, 0)),
            index: read_u16(payload, 2),
            name_index: read_u16(payload, 4),
            configuration: read_u16(payload, 6),
            name: read_array(payload, 8),
        })
    }

    pub fn encode(&self, addressing: Addressing, out: &mut [u8]) -> Result<usize, EncodeError> {
        let mut payload = [0; Self::LEN];
        payload[0..2].copy_from_slice(&self.descriptor_type.0.to_be_bytes());
        payload[2..4].copy_from_slice(&self.index.to_be_bytes());
        payload[4..6].copy_from_slice(&self.name_index.to_be_bytes());
        payload[6..8].copy_from_slice(&self.configuration.to_be_bytes());
        payload[8..].copy_from_slice(&self.name);
        command(addressing, AemCommandType::SET_NAME, &payload).encode(out)
    }
}

/// What SET_STREAM_FORMAT carries for a STREAM_INPUT or STREAM_OUTPUT
/// (7.4.9); the response holds the format the entity now has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetStreamFormat {
    pub descriptor_type: DescriptorType,
    pub index: u16,
    pub format: StreamFormat,
}

impl SetStreamFormat {
    const LEN: usize = 12;

    pub fn decode(payload: &[u8]) -> Result<Self, DecodeError> {
        truncated(Self::LEN, payload)?;
        Ok(Self {
            descriptor_type: DescriptorType(read_u16(payload, 0)),
            index: read_u16(payload, 2),
            format: StreamFormat(read_u64(payload, 4)),
        })
    }

    pub fn encode(&self, addressing: Addressing, out: &mut [u8]) -> Result<usize, EncodeError> {
        let mut payload = [0; Self::LEN];
        payload[0..2].copy_from_slice(&self.descriptor_type.0.to_be_bytes());
        payload[2..4].copy_from_slice(&self.index.to_be_bytes());
        payload[4..12].copy_from_slice(&self.format.0.to_be_bytes());
        command(addressing, AemCommandType::SET_STREAM_FORMAT, &payload).encode(out)
    }
}

/// What SET_SAMPLING_RATE carries for an AUDIO_UNIT, VIDEO_CLUSTER or
/// SENSOR_CLUSTER (7.4.21); the response holds the rate it now has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetSamplingRate {
    pub descriptor_type: DescriptorType,
    pub index: u16,
    pub rate: SamplingRate,
}

impl SetSamplingRate {
    const LEN: usize = 8;

    pub fn decode(payload: &[u8]) -> Result<Self, DecodeError> {
        truncated(Self::LEN, payload)?;
        Ok(Self {
            descriptor_type: DescriptorType(read_u16(payload, 0)),
            index: read_u16(payload, 2),
            rate: SamplingRate(read_u32(payload, 4)),
        })
    }

    pub fn encode(&self, addressing: Addressing, out: &mut [u8]) -> Result<usize, EncodeError> {
        let mut payload = [0; Self::LEN];
        payload[0..2].copy_from_slice(&self.descriptor_type.0.to_be_bytes());
        payload[2..4].copy_from_slice(&self.index.to_be_bytes());
        payload[4..8].copy_from_slice(&self.rate.0.to_be_bytes());
        command(addressing, AemCommandType::SET_SAMPLING_RATE, &payload).encode(out)
    }
}

/// What SET_CLOCK_SOURCE carries for a CLOCK_DOMAIN (7.4.23); the
/// response holds the source it now has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SetClockSource {
    /// The CLOCK_DOMAIN.
    pub domain: u16,
    /// Its CLOCK_SOURCE.
    pub source: u16,
}

impl SetClockSource {
    const LEN: usize = 8;

    pub fn decode(payload: &[u8]) -> Result<Self, DecodeError> {
        truncated(Self::LEN, payload)?;
        Ok(Self {
            domain: read_u16(payload, 2),
            source: read_u16(payload, 4),
        })
    }

    pub fn encode(&self, addressing: Addressing, out: &mut [u8]) -> Result<usize, EncodeError> {
        let mut payload = [0; Self::LEN];
        payload[0..2].copy_from_slice(&DescriptorType::CLOCK_DOMAIN.0.to_be_bytes());
        payload[2..4].copy_from_slice(&self.domain.to_be_bytes());
        payload[4..6].copy_from_slice(&self.source.to_be_bytes());
        command(addressing, AemCommandType::SET_CLOCK_SOURCE, &payload).encode(out)
    }
}

/// A stream output's max transit time (7.4.77 and 7.4.78): how long its
/// frames may take to reach the listeners, which sets their presentation
/// time. SET_MAX_TRANSIT_TIME carries the new one; both responses hold
/// the one the output now has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxTransitTime {
    /// The STREAM_OUTPUT.
    pub output: u16,
    pub nanoseconds: u64,
}

impl MaxTransitTime {
    const LEN: usize = 12;

    pub fn decode(payload: &[u8]) -> Result<Self, DecodeError> {
        truncated(Self::LEN, payload)?;
        Ok(Self {
            output: read_u16(payload, 2),
            nanoseconds: read_u64(payload, 4),
        })
    }

    /// Encodes SET_MAX_TRANSIT_TIME.
    pub fn encode(&self, addressing: Addressing, out: &mut [u8]) -> Result<usize, EncodeError> {
        let mut payload = [0; Self::LEN];
        payload[0..2].copy_from_slice(&DescriptorType::STREAM_OUTPUT.0.to_be_bytes());
        payload[2..4].copy_from_slice(&self.output.to_be_bytes());
        payload[4..12].copy_from_slice(&self.nanoseconds.to_be_bytes());
        command(addressing, AemCommandType::SET_MAX_TRANSIT_TIME, &payload).encode(out)
    }
}

/// Encodes GET_MAX_TRANSIT_TIME for a STREAM_OUTPUT.
pub fn encode_get_max_transit_time(
    addressing: Addressing,
    output: u16,
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut payload = [0; 4];
    payload[0..2].copy_from_slice(&DescriptorType::STREAM_OUTPUT.0.to_be_bytes());
    payload[2..4].copy_from_slice(&output.to_be_bytes());
    command(addressing, AemCommandType::GET_MAX_TRANSIT_TIME, &payload).encode(out)
}

/// The most octets of command specific data an AEM response carries: a
/// control_data_length of at most 524 (9.2.1.1.7), less the controller
/// ID, sequence ID and command type.
pub const MAX_AEM_PAYLOAD: usize = 512;

/// One channel of a stream carried to or from one channel of an audio
/// cluster (Table 7-162). On a stream port input the stream's channel
/// feeds the cluster's; on an output the cluster's feeds the stream's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct AudioMapping {
    /// The STREAM_INPUT or STREAM_OUTPUT.
    pub stream_index: u16,
    pub stream_channel: u16,
    /// The AUDIO_CLUSTER, counted from the stream port's base_cluster.
    pub cluster_offset: u16,
    pub cluster_channel: u16,
}

impl AudioMapping {
    const LEN: usize = 8;

    fn decode(bytes: &[u8; Self::LEN]) -> Self {
        Self {
            stream_index: read_u16(bytes, 0),
            stream_channel: read_u16(bytes, 2),
            cluster_offset: read_u16(bytes, 4),
            cluster_channel: read_u16(bytes, 6),
        }
    }

    pub(crate) fn encode(&self, out: &mut [u8]) {
        out[0..2].copy_from_slice(&self.stream_index.to_be_bytes());
        out[2..4].copy_from_slice(&self.stream_channel.to_be_bytes());
        out[4..6].copy_from_slice(&self.cluster_offset.to_be_bytes());
        out[6..8].copy_from_slice(&self.cluster_channel.to_be_bytes());
    }
}

/// The mappings in `count` entries from the start of `bytes`, or an error
/// when fewer are there.
fn audio_mappings(bytes: &[u8], count: u16) -> Result<&[u8], DecodeError> {
    let needed = usize::from(count) * AudioMapping::LEN;
    bytes.get(..needed).ok_or(DecodeError::Truncated {
        needed,
        available: bytes.len(),
    })
}

pub(crate) fn each_mapping(bytes: &[u8]) -> impl Iterator<Item = AudioMapping> + '_ {
    bytes
        .as_chunks::<{ AudioMapping::LEN }>()
        .0
        .iter()
        .map(AudioMapping::decode)
}

/// Encodes GET_AUDIO_MAP for a STREAM_PORT_INPUT or STREAM_PORT_OUTPUT,
/// asking for the part of its dynamic mappings at `map_index`.
pub fn encode_get_audio_map(
    addressing: Addressing,
    descriptor_type: DescriptorType,
    index: u16,
    map_index: u16,
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut payload = [0; 8];
    payload[0..2].copy_from_slice(&descriptor_type.0.to_be_bytes());
    payload[2..4].copy_from_slice(&index.to_be_bytes());
    payload[4..6].copy_from_slice(&map_index.to_be_bytes());
    command(addressing, AemCommandType::GET_AUDIO_MAP, &payload).encode(out)
}

/// A GET_AUDIO_MAP response (7.4.44.2): one part of a stream port's
/// dynamic mappings. Milan entities answer with up to 176 mappings, more
/// than 1722.1's 524 octet limit holds, so its length is the frame's.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioMap<'a> {
    pub descriptor_type: DescriptorType,
    pub index: u16,
    pub map_index: u16,
    /// How many parts the stream port's mappings come in.
    pub number_of_maps: u16,
    mappings: &'a [u8],
}

impl<'a> AudioMap<'a> {
    pub fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        truncated(12, payload)?;
        Ok(Self {
            descriptor_type: DescriptorType(read_u16(payload, 0)),
            index: read_u16(payload, 2),
            map_index: read_u16(payload, 4),
            number_of_maps: read_u16(payload, 6),
            mappings: audio_mappings(&payload[12..], read_u16(payload, 8))?,
        })
    }

    pub fn mappings(&self) -> impl Iterator<Item = AudioMapping> + use<'a> {
        each_mapping(self.mappings)
    }
}

/// Whether a change adds mappings or removes them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MappingChange {
    Add,
    Remove,
}

impl MappingChange {
    pub fn command_type(self) -> AemCommandType {
        match self {
            MappingChange::Add => AemCommandType::ADD_AUDIO_MAPPINGS,
            MappingChange::Remove => AemCommandType::REMOVE_AUDIO_MAPPINGS,
        }
    }
}

/// The most mappings one ADD_AUDIO_MAPPINGS or REMOVE_AUDIO_MAPPINGS
/// carries within 1722.1's limit.
pub const MAX_MAPPINGS_PER_CHANGE: usize = (MAX_AEM_PAYLOAD - 8) / AudioMapping::LEN;

/// What ADD_AUDIO_MAPPINGS and REMOVE_AUDIO_MAPPINGS carry, in their
/// commands, responses and notifications (7.4.45, 7.4.46).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioMappings<'a> {
    pub descriptor_type: DescriptorType,
    pub index: u16,
    mappings: &'a [u8],
}

impl<'a> AudioMappings<'a> {
    pub fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        truncated(8, payload)?;
        Ok(Self {
            descriptor_type: DescriptorType(read_u16(payload, 0)),
            index: read_u16(payload, 2),
            mappings: audio_mappings(&payload[8..], read_u16(payload, 4))?,
        })
    }

    pub fn mappings(&self) -> impl Iterator<Item = AudioMapping> + use<'a> {
        each_mapping(self.mappings)
    }
}

/// Encodes ADD_AUDIO_MAPPINGS or REMOVE_AUDIO_MAPPINGS for a
/// STREAM_PORT_INPUT or STREAM_PORT_OUTPUT; at most
/// [`MAX_MAPPINGS_PER_CHANGE`] mappings.
pub fn encode_audio_mappings(
    addressing: Addressing,
    change: MappingChange,
    descriptor_type: DescriptorType,
    index: u16,
    mappings: &[AudioMapping],
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    if mappings.len() > MAX_MAPPINGS_PER_CHANGE {
        return Err(EncodeError::OutOfRange("mappings"));
    }
    let mut payload = [0; MAX_AEM_PAYLOAD];
    payload[0..2].copy_from_slice(&descriptor_type.0.to_be_bytes());
    payload[2..4].copy_from_slice(&index.to_be_bytes());
    payload[4..6].copy_from_slice(&(mappings.len() as u16).to_be_bytes());
    let length = 8 + mappings.len() * AudioMapping::LEN;
    for (mapping, slot) in mappings.iter().zip(
        payload[8..length]
            .as_chunks_mut::<{ AudioMapping::LEN }>()
            .0,
    ) {
        mapping.encode(slot);
    }
    command(addressing, change.command_type(), &payload[..length]).encode(out)
}

/// A value GET_DYNAMIC_INFO fetches (7.4.76), as the GET command it
/// stands for asks it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DynamicQuery {
    /// GET_NAME; for the ENTITY, `name_index` 1 is its group name.
    Name {
        descriptor_type: DescriptorType,
        index: u16,
        name_index: u16,
    },
    /// GET_STREAM_FORMAT, for a STREAM_INPUT or STREAM_OUTPUT.
    StreamFormat {
        descriptor_type: DescriptorType,
        index: u16,
    },
    /// GET_SAMPLING_RATE, for an AUDIO_UNIT.
    SamplingRate {
        descriptor_type: DescriptorType,
        index: u16,
    },
    /// GET_CLOCK_SOURCE, for a CLOCK_DOMAIN.
    ClockSource { domain: u16 },
}

impl DynamicQuery {
    pub fn command_type(self) -> AemCommandType {
        match self {
            DynamicQuery::Name { .. } => AemCommandType::GET_NAME,
            DynamicQuery::StreamFormat { .. } => AemCommandType::GET_STREAM_FORMAT,
            DynamicQuery::SamplingRate { .. } => AemCommandType::GET_SAMPLING_RATE,
            DynamicQuery::ClockSource { .. } => AemCommandType::GET_CLOCK_SOURCE,
        }
    }

    /// The descriptor it asks about.
    pub fn target(self) -> (DescriptorType, u16) {
        match self {
            DynamicQuery::Name {
                descriptor_type,
                index,
                ..
            }
            | DynamicQuery::StreamFormat {
                descriptor_type,
                index,
            }
            | DynamicQuery::SamplingRate {
                descriptor_type,
                index,
            } => (descriptor_type, index),
            DynamicQuery::ClockSource { domain } => (DescriptorType::CLOCK_DOMAIN, domain),
        }
    }

    /// The length of the GET command's data, and of its response's.
    fn lengths(self) -> (usize, usize) {
        match self {
            DynamicQuery::Name { .. } => (8, SetName::LEN),
            DynamicQuery::StreamFormat { .. } => (4, SetStreamFormat::LEN),
            DynamicQuery::SamplingRate { .. } => (4, SetSamplingRate::LEN),
            DynamicQuery::ClockSource { .. } => (4, SetClockSource::LEN),
        }
    }

    /// The room its answer takes in a GET_DYNAMIC_INFO response.
    pub fn answer_len(self) -> usize {
        DYNAMIC_INFO_HEADER + self.lengths().1
    }
}

/// Before each dynamic_info: its data length, then its status and command
/// type (Figure 7-94).
const DYNAMIC_INFO_HEADER: usize = 8;

/// Encodes GET_DYNAMIC_INFO asking for `queries`, names in
/// `configuration`. The caller keeps the answers within
/// [`MAX_AEM_PAYLOAD`], as entities leave out what does not fit.
pub fn encode_get_dynamic_info(
    addressing: Addressing,
    configuration: u16,
    queries: &[DynamicQuery],
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    let mut payload = [0; MAX_AEM_PAYLOAD];
    let mut at = 0;
    for query in queries {
        let (length, _) = query.lengths();
        let Some(entry) = payload.get_mut(at..at + DYNAMIC_INFO_HEADER + length) else {
            return Err(EncodeError::OutOfRange("dynamic_infos"));
        };
        let (descriptor_type, index) = query.target();
        entry[0..2].copy_from_slice(&(length as u16).to_be_bytes());
        entry[6..8].copy_from_slice(&query.command_type().0.to_be_bytes());
        entry[8..10].copy_from_slice(&descriptor_type.0.to_be_bytes());
        entry[10..12].copy_from_slice(&index.to_be_bytes());
        if let DynamicQuery::Name { name_index, .. } = query {
            let configuration = match descriptor_type {
                DescriptorType::ENTITY | DescriptorType::CONFIGURATION => 0,
                _ => configuration,
            };
            entry[12..14].copy_from_slice(&name_index.to_be_bytes());
            entry[14..16].copy_from_slice(&configuration.to_be_bytes());
        }
        at += DYNAMIC_INFO_HEADER + length;
    }
    command(addressing, AemCommandType::GET_DYNAMIC_INFO, &payload[..at]).encode(out)
}

/// One answer in a GET_DYNAMIC_INFO response: how its GET went and the
/// GET response's data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DynamicInfo<'a> {
    pub status: AemStatus,
    pub command_type: AemCommandType,
    pub data: &'a [u8],
}

/// The answers in a GET_DYNAMIC_INFO response's data, up to where it ends
/// or stops making sense.
pub fn dynamic_infos(payload: &[u8]) -> impl Iterator<Item = DynamicInfo<'_>> {
    let mut rest = payload;
    core::iter::from_fn(move || {
        if rest.len() < DYNAMIC_INFO_HEADER {
            return None;
        }
        let length = usize::from(read_u16(rest, 0));
        let end = DYNAMIC_INFO_HEADER + length;
        if rest.len() < end {
            return None;
        }
        let info = DynamicInfo {
            status: AemStatus(rest[4] >> 3),
            command_type: AemCommandType(read_u16(rest, 6) & 0x3fff),
            data: &rest[DYNAMIC_INFO_HEADER..end],
        };
        rest = &rest[end..];
        Some(info)
    })
}

flags! {
    /// GET_AVB_INFO flags (Table 7-148).
    pub struct AvbInfoFlags(u8) {
        const AS_CAPABLE = 0x01;
        const GPTP_ENABLED = 0x02;
        const SRP_ENABLED = 0x04;
        const AVTP_DOWN = 0x08;
        const AVTP_DOWN_VALID = 0x10;
    }
}

/// The VLAN and priority an SR class's traffic is sent with.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MsrpMapping {
    pub traffic_class: u8,
    pub priority: u8,
    pub vlan_id: u16,
}

/// The most MSRP mappings kept from a GET_AVB_INFO response, one for each
/// traffic class.
pub const MAX_MSRP_MAPPINGS: usize = 8;

/// A GET_AVB_INFO response, or the same fields in a notification: the
/// interface's gPTP and SRP state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvbInfo {
    pub index: u16,
    /// The grandmaster elected on the interface.
    pub gptp_grandmaster_id: ClockIdentity,
    /// The link's delay to its neighbor, from gPTP's peer delay, in
    /// nanoseconds.
    pub propagation_delay: u32,
    pub gptp_domain_number: u8,
    pub flags: AvbInfoFlags,
    mappings: [MsrpMapping; MAX_MSRP_MAPPINGS],
    mapping_count: u8,
}

impl AvbInfo {
    pub fn decode(payload: &[u8]) -> Result<Self, DecodeError> {
        if payload.len() < 20 {
            return Err(DecodeError::Truncated {
                needed: 20,
                available: payload.len(),
            });
        }
        let count = usize::from(read_u16(payload, 18));
        let mut mappings = [MsrpMapping::default(); MAX_MSRP_MAPPINGS];
        let available = payload[20..]
            .as_chunks::<4>()
            .0
            .iter()
            .take(count.min(MAX_MSRP_MAPPINGS));
        let mut mapping_count = 0;
        for (slot, mapping) in mappings.iter_mut().zip(available) {
            *slot = MsrpMapping {
                traffic_class: mapping[0],
                priority: mapping[1],
                vlan_id: read_u16(mapping, 2),
            };
            mapping_count += 1;
        }
        Ok(Self {
            index: read_u16(payload, 2),
            gptp_grandmaster_id: ClockIdentity(read_u64(payload, 4)),
            propagation_delay: read_u32(payload, 12),
            gptp_domain_number: payload[16],
            flags: AvbInfoFlags(payload[17]),
            mappings,
            mapping_count,
        })
    }

    /// The SR classes' VLANs and priorities, as many as fit.
    pub fn msrp_mappings(&self) -> &[MsrpMapping] {
        &self.mappings[..usize::from(self.mapping_count)]
    }

    /// gPTP runs on the link: the neighbor answers peer delay in time and
    /// is gPTP capable.
    pub fn as_capable(&self) -> bool {
        self.flags.contains(AvbInfoFlags::AS_CAPABLE)
    }
}

/// A GET_AS_PATH response, or the same fields in a notification: the gPTP
/// instances from the grandmaster to the interface, from the path trace of
/// the last Announce it received.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AsPath<'a> {
    pub index: u16,
    sequence: &'a [u8],
}

impl<'a> AsPath<'a> {
    pub fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        if payload.len() < 4 {
            return Err(DecodeError::Truncated {
                needed: 4,
                available: payload.len(),
            });
        }
        let needed = 4 + usize::from(read_u16(payload, 2)) * 8;
        let Some(sequence) = payload.get(4..needed) else {
            return Err(DecodeError::Truncated {
                needed,
                available: payload.len(),
            });
        };
        Ok(Self {
            index: read_u16(payload, 0),
            sequence,
        })
    }

    /// The clock identities, the grandmaster first.
    pub fn clock_identities(&self) -> impl Iterator<Item = ClockIdentity> + 'a {
        self.sequence
            .as_chunks::<8>()
            .0
            .iter()
            .map(|identity| ClockIdentity(u64::from_be_bytes(*identity)))
    }

    pub fn len(&self) -> usize {
        self.sequence.len() / 8
    }

    pub fn is_empty(&self) -> bool {
        self.sequence.is_empty()
    }
}

/// A GET_COUNTERS response, or the same fields in a notification: up to
/// 32 counters, whose meaning depends on the descriptor type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Counters {
    pub descriptor_type: DescriptorType,
    pub index: u16,
    /// One bit for each counter that is valid, counter 0 in the lowest.
    pub valid: u32,
    pub block: [u32; 32],
}

impl Counters {
    pub fn decode(payload: &[u8]) -> Result<Self, DecodeError> {
        if payload.len() < 136 {
            return Err(DecodeError::Truncated {
                needed: 136,
                available: payload.len(),
            });
        }
        let mut block = [0; 32];
        for (index, counter) in block.iter_mut().enumerate() {
            *counter = read_u32(payload, 8 + 4 * index);
        }
        Ok(Self {
            descriptor_type: DescriptorType(read_u16(payload, 0)),
            index: read_u16(payload, 2),
            valid: read_u32(payload, 4),
            block,
        })
    }

    /// The counter at `position` in the block, when it is valid.
    pub fn get(&self, position: usize) -> Option<u32> {
        let bit = 1u32.checked_shl(u32::try_from(position).ok()?)?;
        (self.valid & bit != 0).then(|| self.block[position])
    }

    /// An AVB_INTERFACE's counters.
    pub fn avb_interface(&self) -> Option<AvbInterfaceCounters> {
        (self.descriptor_type == DescriptorType::AVB_INTERFACE).then_some(AvbInterfaceCounters {
            link_up: self.get(0),
            link_down: self.get(1),
            frames_tx: self.get(2),
            frames_rx: self.get(3),
            rx_crc_error: self.get(4),
            gptp_gm_changed: self.get(5),
        })
    }
}

impl Counters {
    /// A CLOCK_DOMAIN's counters.
    pub fn clock_domain(&self) -> Option<ClockDomainCounters> {
        (self.descriptor_type == DescriptorType::CLOCK_DOMAIN).then_some(ClockDomainCounters {
            locked: self.get(0),
            unlocked: self.get(1),
        })
    }

    /// A STREAM_INPUT's counters.
    pub fn stream_input(&self) -> Option<StreamInputCounters> {
        (self.descriptor_type == DescriptorType::STREAM_INPUT).then_some(StreamInputCounters {
            media_locked: self.get(0),
            media_unlocked: self.get(1),
            stream_interrupted: self.get(2),
            seq_num_mismatch: self.get(3),
            media_reset: self.get(4),
            timestamp_uncertain: self.get(5),
            timestamp_valid: self.get(6),
            timestamp_not_valid: self.get(7),
            unsupported_format: self.get(8),
            late_timestamp: self.get(9),
            early_timestamp: self.get(10),
            frames_rx: self.get(11),
        })
    }

    /// A STREAM_OUTPUT's counters. Milan before 1.3 numbered them apart
    /// from 1722.1 (Milan 1.3, 5.4.2.25), stream start, stop, media reset,
    /// timestamp uncertain and frames sent in a row from 0; an entity
    /// telling no specification_version in GET_MILAN_INFO is one of those.
    pub fn stream_output(&self, before_milan_1_3: bool) -> Option<StreamOutputCounters> {
        if self.descriptor_type != DescriptorType::STREAM_OUTPUT {
            return None;
        }
        Some(if before_milan_1_3 {
            StreamOutputCounters {
                stream_start: self.get(0),
                stream_stop: self.get(1),
                media_reset: self.get(2),
                timestamp_uncertain: self.get(3),
                frames_tx: self.get(4),
                ..StreamOutputCounters::default()
            }
        } else {
            StreamOutputCounters {
                stream_start: self.get(0),
                stream_stop: self.get(1),
                stream_interrupted: self.get(2),
                media_reset: self.get(3),
                timestamp_uncertain: self.get(4),
                timestamp_valid: self.get(5),
                timestamp_not_valid: self.get(6),
                frames_tx: self.get(7),
            }
        })
    }
}

/// A CLOCK_DOMAIN's counters (Table 7-155), which Milan requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ClockDomainCounters {
    pub locked: Option<u32>,
    pub unlocked: Option<u32>,
}

/// A STREAM_INPUT's counters (Table 7-157), each `None` when the entity
/// does not keep it. Milan requires all but the two timestamp validity
/// counters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StreamInputCounters {
    pub media_locked: Option<u32>,
    pub media_unlocked: Option<u32>,
    pub stream_interrupted: Option<u32>,
    pub seq_num_mismatch: Option<u32>,
    pub media_reset: Option<u32>,
    pub timestamp_uncertain: Option<u32>,
    pub timestamp_valid: Option<u32>,
    pub timestamp_not_valid: Option<u32>,
    pub unsupported_format: Option<u32>,
    pub late_timestamp: Option<u32>,
    pub early_timestamp: Option<u32>,
    pub frames_rx: Option<u32>,
}

/// A STREAM_OUTPUT's counters (Table 7-159), each `None` when the entity
/// does not keep it. Milan requires the starts, stops, media resets,
/// uncertain timestamps and frames sent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct StreamOutputCounters {
    pub stream_start: Option<u32>,
    pub stream_stop: Option<u32>,
    pub stream_interrupted: Option<u32>,
    pub media_reset: Option<u32>,
    pub timestamp_uncertain: Option<u32>,
    pub timestamp_valid: Option<u32>,
    pub timestamp_not_valid: Option<u32>,
    pub frames_tx: Option<u32>,
}

/// An AVB_INTERFACE's counters (Table 7-153), each `None` when the entity
/// does not keep it. Milan requires the link and grandmaster ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvbInterfaceCounters {
    pub link_up: Option<u32>,
    pub link_down: Option<u32>,
    pub frames_tx: Option<u32>,
    pub frames_rx: Option<u32>,
    pub rx_crc_error: Option<u32>,
    pub gptp_gm_changed: Option<u32>,
}

flags! {
    /// GET_STREAM_INFO flags (Table 7-145). The low 16 bits are the ACMP
    /// flags (Table 8-4), as Milan 1.3, 5.4.2.10 clarifies.
    pub struct StreamInfoFlags(u32) {
        const CLASS_B = 0x0000_0001;
        const FAST_CONNECT = 0x0000_0002;
        const SAVED_STATE = 0x0000_0004;
        const STREAMING_WAIT = 0x0000_0008;
        const SUPPORTS_ENCRYPTED = 0x0000_0010;
        const ENCRYPTED_PDU = 0x0000_0020;
        const SRP_REGISTRATION_FAILED = 0x0000_0040;
        const CL_ENTRIES_VALID = 0x0000_0080;
        const NO_SRP = 0x0000_0100;
        const UDP = 0x0000_0200;
        const IP_FLAGS_VALID = 0x0008_0000;
        const IP_SRC_PORT_VALID = 0x0010_0000;
        const IP_DST_PORT_VALID = 0x0020_0000;
        const IP_SRC_ADDR_VALID = 0x0040_0000;
        const IP_DST_ADDR_VALID = 0x0080_0000;
        const NOT_REGISTERING_SRP = 0x0100_0000;
        const STREAM_VLAN_ID_VALID = 0x0200_0000;
        const CONNECTED = 0x0400_0000;
        const MSRP_FAILURE_VALID = 0x0800_0000;
        const STREAM_DEST_MAC_VALID = 0x1000_0000;
        const MSRP_ACC_LAT_VALID = 0x2000_0000;
        const STREAM_ID_VALID = 0x4000_0000;
        const STREAM_FORMAT_VALID = 0x8000_0000;
    }
}

/// A GET_STREAM_INFO response, or the same fields in a notification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamInfo {
    pub descriptor_type: DescriptorType,
    pub index: u16,
    pub flags: StreamInfoFlags,
    pub stream_format: StreamFormat,
    pub stream_id: StreamId,
    /// Nanoseconds.
    pub msrp_accumulated_latency: u32,
    pub stream_dest_mac: MacAddress,
    pub msrp_failure_code: u8,
    pub msrp_failure_bridge_id: u64,
    pub stream_vlan_id: u16,
    /// Milan's extension, for a stream input, where the entity gives it:
    /// its probe's state (0 disabled, 1 passive, 2 active, 3 completed)
    /// and the status of the ACMP answer it last had.
    pub probing: Option<(u8, AcmpStatus)>,
}

impl StreamInfo {
    /// Decodes the command specific data, in the 2013 or 2021 form.
    pub fn decode(payload: &[u8]) -> Result<Self, DecodeError> {
        if payload.len() < 46 {
            return Err(DecodeError::Truncated {
                needed: 46,
                available: payload.len(),
            });
        }
        Ok(Self {
            descriptor_type: DescriptorType(read_u16(payload, 0)),
            index: read_u16(payload, 2),
            flags: StreamInfoFlags(read_u32(payload, 4)),
            stream_format: StreamFormat(read_u64(payload, 8)),
            stream_id: StreamId(read_u64(payload, 16)),
            msrp_accumulated_latency: read_u32(payload, 24),
            stream_dest_mac: MacAddress(read_array(payload, 28)),
            msrp_failure_code: payload[34],
            msrp_failure_bridge_id: read_u64(payload, 36),
            stream_vlan_id: read_u16(payload, 44),
            probing: payload
                .get(52)
                .map(|&octet| (octet >> 5, AcmpStatus(octet & 0x1f))),
        })
    }

    /// For a stream input: bound to a talker.
    pub fn bound(&self) -> bool {
        self.flags.contains(StreamInfoFlags::CONNECTED)
    }

    /// For a stream input: it has the talker's stream parameters, so the
    /// stream can flow.
    pub fn settled(&self) -> bool {
        self.flags.contains(StreamInfoFlags::STREAM_ID_VALID)
    }

    /// The talker's MSRP declaration failed somewhere on the path.
    pub fn talker_failed(&self) -> bool {
        self.flags
            .contains(StreamInfoFlags::SRP_REGISTRATION_FAILED)
            || self.flags.contains(StreamInfoFlags::MSRP_FAILURE_VALID)
    }

    /// For a stream input: hearing the talker's MSRP declaration.
    pub fn registering(&self) -> bool {
        self.flags.contains(StreamInfoFlags::MSRP_ACC_LAT_VALID)
    }
}

/// The command specific data of a READ_DESCRIPTOR response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadDescriptorResponse<'a> {
    pub configuration: u16,
    pub descriptor_type: DescriptorType,
    pub index: u16,
    /// The whole descriptor, from its descriptor_type field on.
    pub descriptor: &'a [u8],
}

impl<'a> ReadDescriptorResponse<'a> {
    pub fn decode(payload: &'a [u8]) -> Result<Self, DecodeError> {
        if payload.len() < 8 {
            return Err(DecodeError::Truncated {
                needed: 8,
                available: payload.len(),
            });
        }
        Ok(Self {
            configuration: read_u16(payload, 0),
            descriptor_type: DescriptorType(read_u16(payload, 4)),
            index: read_u16(payload, 6),
            descriptor: &payload[4..],
        })
    }
}

/// The descriptor a command or notification is about, for those whose
/// data starts with descriptor_type and descriptor_index, as most do.
pub fn target_descriptor(payload: &[u8]) -> Option<(DescriptorType, u16)> {
    (payload.len() >= 4).then(|| (DescriptorType(read_u16(payload, 0)), read_u16(payload, 2)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADDRESSING: Addressing = Addressing {
        target: EntityId(0xe8f6_0ae0_9220_0000),
        controller: EntityId(0x9c6b_00ff_fe30_9a2b),
        sequence_id: 9,
    };

    #[test]
    fn read_descriptor_command() {
        let mut out = [0; 64];
        let length =
            encode_read_descriptor(ADDRESSING, 1, DescriptorType::STREAM_INPUT, 2, &mut out)
                .unwrap();
        let pdu = AemPdu::decode(&out[..length]).unwrap();
        assert_eq!(pdu.command_type, AemCommandType::READ_DESCRIPTOR);
        assert_eq!(pdu.payload, &[0, 1, 0, 0, 0, 5, 0, 2]);
        let length =
            encode_read_descriptor(ADDRESSING, 1, DescriptorType::ENTITY, 0, &mut out).unwrap();
        assert_eq!(AemPdu::decode(&out[..length]).unwrap().payload, &[0; 8]);
    }

    #[test]
    fn responses_echo_the_command() {
        let mut out = [0; 64];
        let length = command(ADDRESSING, AemCommandType::CONTROLLER_AVAILABLE, &[])
            .encode(&mut out)
            .unwrap();
        let received = AemPdu::decode(&out[..length]).unwrap();
        let mut reply = [0; 64];
        let length = encode_response(&received, AemStatus::SUCCESS, &mut reply).unwrap();
        let response = AemPdu::decode(&reply[..length]).unwrap();
        assert_eq!(response.header.message_type, AecpMessageType::AEM_RESPONSE);
        assert_eq!(response.header.sequence_id, 9);
        assert_eq!(response.command_type, AemCommandType::CONTROLLER_AVAILABLE);
    }

    #[test]
    fn stream_info_in_both_forms() {
        let mut payload = [0u8; 48];
        payload[0..2].copy_from_slice(&DescriptorType::STREAM_INPUT.0.to_be_bytes());
        payload[4..8].copy_from_slice(&0x6600_0000u32.to_be_bytes());
        payload[8..16].copy_from_slice(&0x0205_0220_0200_6000u64.to_be_bytes());
        payload[16..24].copy_from_slice(&0xe8f6_0ae0_9220_0000u64.to_be_bytes());
        payload[24..28].copy_from_slice(&500_000u32.to_be_bytes());
        payload[28..34].copy_from_slice(&[0x91, 0xe0, 0xf0, 0x00, 0x6a, 0x20]);
        payload[44..46].copy_from_slice(&2u16.to_be_bytes());
        let info = StreamInfo::decode(&payload).unwrap();
        assert!(info.bound() && info.settled() && info.registering());
        assert!(!info.talker_failed());
        assert_eq!(info.stream_format, StreamFormat(0x0205_0220_0200_6000));
        assert_eq!(info.msrp_accumulated_latency, 500_000);
        assert_eq!(info.stream_vlan_id, 2);
        assert!(StreamInfo::decode(&payload[..45]).is_err());
    }

    #[test]
    fn names_fit_in_64_octets() {
        let name = aem_name("Stage box").unwrap();
        assert_eq!(&name[..9], b"Stage box");
        assert!(name[9..].iter().all(|&octet| octet == 0));
        assert!(aem_name(&"x".repeat(64)).is_some());
        assert!(aem_name(&"x".repeat(65)).is_none());
        // Octets count, not characters: 22 three-octet characters are 66.
        assert!(aem_name(&"音".repeat(22)).is_none());
    }

    #[test]
    fn set_commands_round_trip() {
        let mut out = [0; 128];
        let name = SetName {
            descriptor_type: DescriptorType::ENTITY,
            index: 0,
            name_index: 1,
            configuration: 0,
            name: aem_name("Front of house").unwrap(),
        };
        let length = name.encode(ADDRESSING, &mut out).unwrap();
        let pdu = AemPdu::decode(&out[..length]).unwrap();
        assert_eq!(pdu.command_type, AemCommandType::SET_NAME);
        assert_eq!(&pdu.payload[..8], &[0, 0, 0, 0, 0, 1, 0, 0]);
        assert_eq!(SetName::decode(pdu.payload), Ok(name));

        let format = SetStreamFormat {
            descriptor_type: DescriptorType::STREAM_INPUT,
            index: 1,
            format: StreamFormat(0x0205_0220_0200_6000),
        };
        let length = format.encode(ADDRESSING, &mut out).unwrap();
        let pdu = AemPdu::decode(&out[..length]).unwrap();
        assert_eq!(pdu.command_type, AemCommandType::SET_STREAM_FORMAT);
        assert_eq!(SetStreamFormat::decode(pdu.payload), Ok(format));

        let rate = SetSamplingRate {
            descriptor_type: DescriptorType::AUDIO_UNIT,
            index: 0,
            rate: SamplingRate(96_000),
        };
        let length = rate.encode(ADDRESSING, &mut out).unwrap();
        let pdu = AemPdu::decode(&out[..length]).unwrap();
        assert_eq!(pdu.command_type, AemCommandType::SET_SAMPLING_RATE);
        assert_eq!(pdu.payload, &[0, 2, 0, 0, 0, 1, 0x77, 0]);
        assert_eq!(SetSamplingRate::decode(pdu.payload), Ok(rate));

        let source = SetClockSource {
            domain: 0,
            source: 2,
        };
        let length = source.encode(ADDRESSING, &mut out).unwrap();
        let pdu = AemPdu::decode(&out[..length]).unwrap();
        assert_eq!(pdu.command_type, AemCommandType::SET_CLOCK_SOURCE);
        assert_eq!(pdu.payload, &[0, 0x24, 0, 0, 0, 2, 0, 0]);
        assert_eq!(SetClockSource::decode(pdu.payload), Ok(source));

        assert!(SetName::decode(&[0; 71]).is_err());
        assert!(SetStreamFormat::decode(&[0; 11]).is_err());
    }

    #[test]
    fn audio_map_commands() {
        let mut out = [0; 600];
        let length = encode_get_audio_map(
            ADDRESSING,
            DescriptorType::STREAM_PORT_INPUT,
            0,
            1,
            &mut out,
        )
        .unwrap();
        let pdu = AemPdu::decode(&out[..length]).unwrap();
        assert_eq!(pdu.command_type, AemCommandType::GET_AUDIO_MAP);
        assert_eq!(pdu.payload, &[0, 0x0e, 0, 0, 0, 1, 0, 0]);

        let mappings = [
            AudioMapping {
                stream_index: 1,
                stream_channel: 3,
                cluster_offset: 0,
                cluster_channel: 0,
            },
            AudioMapping {
                stream_index: 0,
                stream_channel: 0,
                cluster_offset: 7,
                cluster_channel: 1,
            },
        ];
        let length = encode_audio_mappings(
            ADDRESSING,
            MappingChange::Remove,
            DescriptorType::STREAM_PORT_OUTPUT,
            2,
            &mappings,
            &mut out,
        )
        .unwrap();
        let pdu = AemPdu::decode(&out[..length]).unwrap();
        assert_eq!(pdu.command_type, AemCommandType::REMOVE_AUDIO_MAPPINGS);
        assert_eq!(&pdu.payload[..8], &[0, 0x0f, 0, 2, 0, 2, 0, 0]);
        assert_eq!(&pdu.payload[8..16], &[0, 1, 0, 3, 0, 0, 0, 0]);
        let decoded = AudioMappings::decode(pdu.payload).unwrap();
        assert_eq!(decoded.descriptor_type, DescriptorType::STREAM_PORT_OUTPUT);
        assert_eq!(decoded.index, 2);
        assert_eq!(decoded.mappings().collect::<Vec<_>>(), mappings);
        // A count the frame does not hold is an error, not a shorter list.
        assert!(AudioMappings::decode(&pdu.payload[..15]).is_err());

        let too_many = [AudioMapping::default(); MAX_MAPPINGS_PER_CHANGE + 1];
        assert!(
            encode_audio_mappings(
                ADDRESSING,
                MappingChange::Add,
                DescriptorType::STREAM_PORT_INPUT,
                0,
                &too_many,
                &mut out,
            )
            .is_err()
        );
        let most = [AudioMapping::default(); MAX_MAPPINGS_PER_CHANGE];
        let length = encode_audio_mappings(
            ADDRESSING,
            MappingChange::Add,
            DescriptorType::STREAM_PORT_INPUT,
            0,
            &most,
            &mut out,
        )
        .unwrap();
        assert_eq!(AemPdu::decode(&out[..length]).unwrap().payload.len(), 512);
    }

    #[test]
    fn audio_map_responses_longer_than_1722_1_allows() {
        // 176 mappings, as Milan entities send in one response.
        let mut payload = vec![0u8; 12 + 176 * 8];
        payload[0..2].copy_from_slice(&DescriptorType::STREAM_PORT_INPUT.0.to_be_bytes());
        payload[4..6].copy_from_slice(&1u16.to_be_bytes());
        payload[6..8].copy_from_slice(&2u16.to_be_bytes());
        payload[8..10].copy_from_slice(&176u16.to_be_bytes());
        for (channel, slot) in payload[12..].as_chunks_mut::<8>().0.iter_mut().enumerate() {
            slot[2..4].copy_from_slice(&(channel as u16).to_be_bytes());
            slot[6..8].copy_from_slice(&(channel as u16).to_be_bytes());
        }
        let map = AudioMap::decode(&payload).unwrap();
        assert_eq!((map.map_index, map.number_of_maps), (1, 2));
        assert_eq!(map.mappings().count(), 176);
        assert_eq!(
            map.mappings().last(),
            Some(AudioMapping {
                stream_index: 0,
                stream_channel: 175,
                cluster_offset: 0,
                cluster_channel: 175,
            })
        );
        assert!(AudioMap::decode(&payload[..payload.len() - 1]).is_err());
        assert!(AudioMap::decode(&payload[..11]).is_err());
    }

    #[test]
    fn set_control_for_identify() {
        let mut out = [0; 64];
        let length = encode_set_control(ADDRESSING, 3, &[255], &mut out).unwrap();
        let pdu = AemPdu::decode(&out[..length]).unwrap();
        assert_eq!(pdu.command_type, AemCommandType::SET_CONTROL);
        assert_eq!(pdu.payload, &[0x00, 0x1a, 0x00, 0x03, 0xff]);
    }

    #[test]
    fn network_queries() {
        let mut out = [0; 64];
        let length = encode_get_avb_info(ADDRESSING, 1, &mut out).unwrap();
        let pdu = AemPdu::decode(&out[..length]).unwrap();
        assert_eq!(pdu.command_type, AemCommandType::GET_AVB_INFO);
        assert_eq!(pdu.payload, &[0x00, 0x09, 0x00, 0x01]);
        let length = encode_get_as_path(ADDRESSING, 1, &mut out).unwrap();
        let pdu = AemPdu::decode(&out[..length]).unwrap();
        assert_eq!(pdu.command_type, AemCommandType::GET_AS_PATH);
        assert_eq!(pdu.payload, &[0x00, 0x01, 0x00, 0x00]);
        let length =
            encode_get_counters(ADDRESSING, DescriptorType::AVB_INTERFACE, 0, &mut out).unwrap();
        let pdu = AemPdu::decode(&out[..length]).unwrap();
        assert_eq!(pdu.command_type, AemCommandType::GET_COUNTERS);
        assert_eq!(pdu.payload, &[0x00, 0x09, 0x00, 0x00]);
    }

    #[test]
    fn avb_info() {
        let mut payload = [0u8; 28];
        payload[0..2].copy_from_slice(&DescriptorType::AVB_INTERFACE.0.to_be_bytes());
        payload[4..12].copy_from_slice(&0x0001_f2ff_fe3b_1400u64.to_be_bytes());
        payload[12..16].copy_from_slice(&312u32.to_be_bytes());
        payload[17] = 0x07;
        payload[18..20].copy_from_slice(&2u16.to_be_bytes());
        payload[20..24].copy_from_slice(&[0, 3, 0, 2]);
        payload[24..28].copy_from_slice(&[1, 2, 0, 2]);
        let info = AvbInfo::decode(&payload).unwrap();
        assert_eq!(
            info.gptp_grandmaster_id,
            ClockIdentity(0x0001_f2ff_fe3b_1400)
        );
        assert_eq!(info.propagation_delay, 312);
        assert!(info.as_capable());
        assert!(info.flags.contains(AvbInfoFlags::SRP_ENABLED));
        assert_eq!(
            info.msrp_mappings(),
            &[
                MsrpMapping {
                    traffic_class: 0,
                    priority: 3,
                    vlan_id: 2
                },
                MsrpMapping {
                    traffic_class: 1,
                    priority: 2,
                    vlan_id: 2
                },
            ]
        );
        // A count the frame does not hold keeps the mappings it does.
        assert_eq!(
            AvbInfo::decode(&payload[..24])
                .unwrap()
                .msrp_mappings()
                .len(),
            1
        );
        assert!(AvbInfo::decode(&payload[..19]).is_err());
    }

    #[test]
    fn as_path() {
        let mut payload = [0u8; 20];
        payload[2..4].copy_from_slice(&2u16.to_be_bytes());
        payload[4..12].copy_from_slice(&0x0001_f2ff_fe3b_1400u64.to_be_bytes());
        payload[12..20].copy_from_slice(&0xe8f6_0aff_fee0_9220u64.to_be_bytes());
        let path = AsPath::decode(&payload).unwrap();
        assert_eq!(path.len(), 2);
        assert_eq!(
            path.clock_identities().collect::<Vec<_>>(),
            [
                ClockIdentity(0x0001_f2ff_fe3b_1400),
                ClockIdentity(0xe8f6_0aff_fee0_9220)
            ]
        );
        assert!(AsPath::decode(&payload[..19]).is_err());
    }

    #[test]
    fn avb_interface_counters() {
        let mut payload = [0u8; 136];
        payload[0..2].copy_from_slice(&DescriptorType::AVB_INTERFACE.0.to_be_bytes());
        payload[4..8].copy_from_slice(&0x0000_0023u32.to_be_bytes());
        payload[8..12].copy_from_slice(&3u32.to_be_bytes());
        payload[12..16].copy_from_slice(&2u32.to_be_bytes());
        payload[16..20].copy_from_slice(&99u32.to_be_bytes());
        payload[28..32].copy_from_slice(&4u32.to_be_bytes());
        let counters = Counters::decode(&payload).unwrap();
        let interface = counters.avb_interface().unwrap();
        assert_eq!(interface.link_up, Some(3));
        assert_eq!(interface.link_down, Some(2));
        // Not marked valid, so not read.
        assert_eq!(interface.frames_tx, None);
        assert_eq!(interface.gptp_gm_changed, Some(4));
        assert_eq!(counters.get(32), None);
        assert!(Counters::decode(&payload[..135]).is_err());
    }

    #[test]
    fn stream_and_clock_domain_counters() {
        let counters = |descriptor_type: DescriptorType, valid: u32| {
            let mut block = [0; 32];
            for (position, counter) in block.iter_mut().enumerate() {
                *counter = 100 + position as u32;
            }
            Counters {
                descriptor_type,
                index: 0,
                valid,
                block,
            }
        };
        let domain = counters(DescriptorType::CLOCK_DOMAIN, 0x3)
            .clock_domain()
            .unwrap();
        assert_eq!((domain.locked, domain.unlocked), (Some(100), Some(101)));

        // Milan's mandatory stream input counters (Table 5.13).
        let input = counters(DescriptorType::STREAM_INPUT, 0x0000_0f3f)
            .stream_input()
            .unwrap();
        assert_eq!(input.stream_interrupted, Some(102));
        assert_eq!(input.unsupported_format, Some(108));
        assert_eq!(input.frames_rx, Some(111));
        assert_eq!(input.timestamp_valid, None);
        assert!(
            counters(DescriptorType::STREAM_INPUT, 0)
                .clock_domain()
                .is_none()
        );

        // Milan 1.3's mandatory stream output counters (Table 5.14).
        let output = counters(DescriptorType::STREAM_OUTPUT, 0x0000_009b);
        let now = output.stream_output(false).unwrap();
        assert_eq!(now.media_reset, Some(103));
        assert_eq!(now.timestamp_uncertain, Some(104));
        assert_eq!(now.frames_tx, Some(107));
        assert_eq!(now.stream_interrupted, None);
        // Before 1.3 the same counters sat at 0 to 4.
        let before = counters(DescriptorType::STREAM_OUTPUT, 0x1f)
            .stream_output(true)
            .unwrap();
        assert_eq!(before.media_reset, Some(102));
        assert_eq!(before.timestamp_uncertain, Some(103));
        assert_eq!(before.frames_tx, Some(104));
        assert_eq!(before.stream_interrupted, None);
    }

    #[test]
    fn read_descriptor_response_and_targets() {
        let payload = [0, 0, 0, 0, 0, 0x24, 0, 1, 0xaa];
        let response = ReadDescriptorResponse::decode(&payload).unwrap();
        assert_eq!(response.descriptor_type, DescriptorType::CLOCK_DOMAIN);
        assert_eq!(response.index, 1);
        assert_eq!(response.descriptor, &payload[4..]);
        assert!(ReadDescriptorResponse::decode(&payload[..7]).is_err());
        assert_eq!(
            target_descriptor(&[0, 5, 0, 3]),
            Some((DescriptorType::STREAM_INPUT, 3))
        );
        assert_eq!(target_descriptor(&[0, 5]), None);
    }
}
