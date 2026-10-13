//! AEM descriptors (IEEE 1722.1-2021, 7.2): their types, and views of the
//! ones a controller reads to show an entity. A view borrows the
//! descriptor's octets, from its descriptor_type field on, and checks
//! their length when made; arrays are found through the offsets the
//! descriptor gives, as their position changed between revisions.

use avb_net::MacAddress;

use crate::adp::{
    ControllerCapabilities, EntityCapabilities, ListenerCapabilities, TalkerCapabilities,
};
use crate::aem::AudioMapping;
use crate::avtp::{read_array, read_u16, read_u32, read_u64};
use crate::error::DecodeError;
use crate::id::{ClockIdentity, EntityId, EntityModelId};
use crate::macros::{code, flags};
pub use crate::stream_format::StreamFormat;

code! {
    /// descriptor_type (Table 7-1).
    pub struct DescriptorType(u16) {
        const ENTITY = 0x0000;
        const CONFIGURATION = 0x0001;
        const AUDIO_UNIT = 0x0002;
        const VIDEO_UNIT = 0x0003;
        const SENSOR_UNIT = 0x0004;
        const STREAM_INPUT = 0x0005;
        const STREAM_OUTPUT = 0x0006;
        const JACK_INPUT = 0x0007;
        const JACK_OUTPUT = 0x0008;
        const AVB_INTERFACE = 0x0009;
        const CLOCK_SOURCE = 0x000a;
        const MEMORY_OBJECT = 0x000b;
        const LOCALE = 0x000c;
        const STRINGS = 0x000d;
        const STREAM_PORT_INPUT = 0x000e;
        const STREAM_PORT_OUTPUT = 0x000f;
        const EXTERNAL_PORT_INPUT = 0x0010;
        const EXTERNAL_PORT_OUTPUT = 0x0011;
        const INTERNAL_PORT_INPUT = 0x0012;
        const INTERNAL_PORT_OUTPUT = 0x0013;
        const AUDIO_CLUSTER = 0x0014;
        const VIDEO_CLUSTER = 0x0015;
        const SENSOR_CLUSTER = 0x0016;
        const AUDIO_MAP = 0x0017;
        const VIDEO_MAP = 0x0018;
        const SENSOR_MAP = 0x0019;
        const CONTROL = 0x001a;
        const SIGNAL_SELECTOR = 0x001b;
        const MIXER = 0x001c;
        const MATRIX = 0x001d;
        const MATRIX_SIGNAL = 0x001e;
        const SIGNAL_SPLITTER = 0x001f;
        const SIGNAL_COMBINER = 0x0020;
        const SIGNAL_DEMULTIPLEXER = 0x0021;
        const SIGNAL_MULTIPLEXER = 0x0022;
        const SIGNAL_TRANSCODER = 0x0023;
        const CLOCK_DOMAIN = 0x0024;
        const CONTROL_BLOCK = 0x0025;
        const TIMING = 0x0026;
        const PTP_INSTANCE = 0x0027;
        const PTP_PORT = 0x0028;
        const INVALID = 0xffff;
    }
}

impl DescriptorType {
    /// Whether descriptors of this type start with object_name and
    /// localized_description after their type and index.
    pub const fn has_object_name(self) -> bool {
        !matches!(
            self,
            Self::ENTITY
                | Self::LOCALE
                | Self::STRINGS
                | Self::STREAM_PORT_INPUT
                | Self::STREAM_PORT_OUTPUT
                | Self::AUDIO_MAP
                | Self::VIDEO_MAP
                | Self::SENSOR_MAP
                | Self::INVALID
        )
    }
}

/// A pointer into an entity's localized strings (7.3.7): which STRINGS
/// descriptor, counted from the LOCALE's base_strings, and which of its
/// seven strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct LocalizedStringRef(pub u16);

impl LocalizedStringRef {
    pub const NONE: Self = Self(0xffff);

    pub const fn offset(self) -> u16 {
        self.0 >> 3
    }

    pub const fn index(self) -> u8 {
        (self.0 & 0x07) as u8
    }

    /// No string: an index of 7.
    pub const fn is_none(self) -> bool {
        self.index() == 7
    }
}

/// The text of a 64-octet string field: up to the first zero, cut back to
/// the longest valid UTF-8 if a device sent something else.
pub fn aem_string(field: &[u8]) -> &str {
    let end = field
        .iter()
        .position(|&octet| octet == 0)
        .unwrap_or(field.len());
    let field = &field[..end];
    match core::str::from_utf8(field) {
        Ok(text) => text,
        Err(error) => core::str::from_utf8(&field[..error.valid_up_to()]).unwrap_or_default(),
    }
}

/// A sampling rate (7.3.1): a 3-bit pull and a 29-bit base frequency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct SamplingRate(pub u32);

impl SamplingRate {
    pub const fn pull(self) -> u8 {
        (self.0 >> 29) as u8
    }

    /// The base frequency in hertz.
    pub const fn base_frequency(self) -> u32 {
        self.0 & 0x1fff_ffff
    }
}

flags! {
    /// stream_flags (Table 7-9). The standard numbers bits from the most
    /// significant, so its bit 15 is `0x0001`.
    pub struct StreamFlags(u16) {
        const CLOCK_SYNC_SOURCE = 0x0001;
        const CLASS_A = 0x0002;
        const CLASS_B = 0x0004;
        const SUPPORTS_ENCRYPTED = 0x0008;
        const PRIMARY_BACKUP_SUPPORTED = 0x0010;
        const PRIMARY_BACKUP_VALID = 0x0020;
        const SECONDARY_BACKUP_SUPPORTED = 0x0040;
        const SECONDARY_BACKUP_VALID = 0x0080;
        const TERTIARY_BACKUP_SUPPORTED = 0x0100;
        const TERTIARY_BACKUP_VALID = 0x0200;
        const SUPPORTS_AVTP_UDPV4 = 0x0400;
        const SUPPORTS_AVTP_UDPV6 = 0x0800;
        const NO_SUPPORT_AVTP_NATIVE = 0x1000;
        const TIMING_FIELD_VALID = 0x2000;
        const NO_MEDIA_CLOCK = 0x4000;
        const SUPPORTS_NO_SRP = 0x8000;
    }
}

flags! {
    /// interface_flags of an AVB_INTERFACE (Table 7-14).
    pub struct InterfaceFlags(u16) {
        const GPTP_GRANDMASTER_SUPPORTED = 0x0001;
        const GPTP_SUPPORTED = 0x0002;
        const SRP_SUPPORTED = 0x0004;
        const FQTSS_NOT_SUPPORTED = 0x0008;
        const SCHEDULED_TRAFFIC_SUPPORTED = 0x0010;
        const CAN_LISTEN_TO_SELF = 0x0020;
        const CAN_LISTEN_TO_OTHER_SELF = 0x0040;
    }
}

flags! {
    /// clock_source_flags (Table 7-16).
    pub struct ClockSourceFlags(u16) {
        const STREAM_ID = 0x0001;
        const LOCAL_ID = 0x0002;
    }
}

code! {
    /// clock_source_type (Table 7-17).
    pub struct ClockSourceType(u16) {
        const INTERNAL = 0x0000;
        const EXTERNAL = 0x0001;
        const INPUT_STREAM = 0x0002;
        const EXPANSION = 0xffff;
    }
}

fn need(bytes: &[u8], length: usize) -> Result<(), DecodeError> {
    if bytes.len() < length {
        return Err(DecodeError::Truncated {
            needed: length,
            available: bytes.len(),
        });
    }
    Ok(())
}

/// `count` items of `size` octets at `offset`, checked to be within the
/// descriptor.
fn array(bytes: &[u8], offset: u16, count: u16, size: usize) -> Result<&[u8], DecodeError> {
    let start = usize::from(offset);
    let end = start + usize::from(count) * size;
    need(bytes, end)?;
    Ok(&bytes[start..end])
}

/// The type and index every descriptor starts with.
pub fn header(bytes: &[u8]) -> Result<(DescriptorType, u16), DecodeError> {
    need(bytes, 4)?;
    Ok((DescriptorType(read_u16(bytes, 0)), read_u16(bytes, 2)))
}

/// The object_name and localized_description of descriptors that have
/// them.
pub fn names(bytes: &[u8]) -> Option<(&str, LocalizedStringRef)> {
    let (descriptor_type, _) = header(bytes).ok()?;
    if !descriptor_type.has_object_name() || bytes.len() < 70 {
        return None;
    }
    Some((
        aem_string(&bytes[4..68]),
        LocalizedStringRef(read_u16(bytes, 68)),
    ))
}

/// The ENTITY descriptor (Table 7-2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntityDescriptor<'a> {
    pub entity_id: EntityId,
    pub entity_model_id: EntityModelId,
    pub entity_capabilities: EntityCapabilities,
    pub talker_stream_sources: u16,
    pub talker_capabilities: TalkerCapabilities,
    pub listener_stream_sinks: u16,
    pub listener_capabilities: ListenerCapabilities,
    pub controller_capabilities: ControllerCapabilities,
    pub available_index: u32,
    pub association_id: u64,
    pub entity_name: &'a str,
    pub vendor_name: LocalizedStringRef,
    pub model_name: LocalizedStringRef,
    pub firmware_version: &'a str,
    pub group_name: &'a str,
    pub serial_number: &'a str,
    pub configurations_count: u16,
    pub current_configuration: u16,
}

impl<'a> EntityDescriptor<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        need(bytes, 312)?;
        Ok(Self {
            entity_id: EntityId(read_u64(bytes, 4)),
            entity_model_id: EntityModelId(read_u64(bytes, 12)),
            entity_capabilities: EntityCapabilities(read_u32(bytes, 20)),
            talker_stream_sources: read_u16(bytes, 24),
            talker_capabilities: TalkerCapabilities(read_u16(bytes, 26)),
            listener_stream_sinks: read_u16(bytes, 28),
            listener_capabilities: ListenerCapabilities(read_u16(bytes, 30)),
            controller_capabilities: ControllerCapabilities(read_u32(bytes, 32)),
            available_index: read_u32(bytes, 36),
            association_id: read_u64(bytes, 40),
            entity_name: aem_string(&bytes[48..112]),
            vendor_name: LocalizedStringRef(read_u16(bytes, 112)),
            model_name: LocalizedStringRef(read_u16(bytes, 114)),
            firmware_version: aem_string(&bytes[116..180]),
            group_name: aem_string(&bytes[180..244]),
            serial_number: aem_string(&bytes[244..308]),
            configurations_count: read_u16(bytes, 308),
            current_configuration: read_u16(bytes, 310),
        })
    }
}

/// The CONFIGURATION descriptor (Table 7-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfigurationDescriptor<'a> {
    pub index: u16,
    pub object_name: &'a str,
    pub localized_description: LocalizedStringRef,
    counts: &'a [u8],
}

impl<'a> ConfigurationDescriptor<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        need(bytes, 74)?;
        Ok(Self {
            index: read_u16(bytes, 2),
            object_name: aem_string(&bytes[4..68]),
            localized_description: LocalizedStringRef(read_u16(bytes, 68)),
            counts: array(bytes, read_u16(bytes, 72), read_u16(bytes, 70), 4)?,
        })
    }

    /// How many descriptors of each top level type the configuration has.
    pub fn descriptor_counts(&self) -> impl Iterator<Item = (DescriptorType, u16)> + 'a {
        self.counts
            .as_chunks::<4>()
            .0
            .iter()
            .map(|count| (DescriptorType(read_u16(count, 0)), read_u16(count, 2)))
    }
}

/// The AUDIO_UNIT descriptor (Table 7-5), the fields a controller uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioUnitDescriptor<'a> {
    pub index: u16,
    pub object_name: &'a str,
    pub localized_description: LocalizedStringRef,
    pub clock_domain_index: u16,
    pub number_of_stream_input_ports: u16,
    pub base_stream_input_port: u16,
    pub number_of_stream_output_ports: u16,
    pub base_stream_output_port: u16,
    pub current_sampling_rate: SamplingRate,
    sampling_rates: &'a [u8],
}

impl<'a> AudioUnitDescriptor<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        need(bytes, 144)?;
        Ok(Self {
            index: read_u16(bytes, 2),
            object_name: aem_string(&bytes[4..68]),
            localized_description: LocalizedStringRef(read_u16(bytes, 68)),
            clock_domain_index: read_u16(bytes, 70),
            number_of_stream_input_ports: read_u16(bytes, 72),
            base_stream_input_port: read_u16(bytes, 74),
            number_of_stream_output_ports: read_u16(bytes, 76),
            base_stream_output_port: read_u16(bytes, 78),
            current_sampling_rate: SamplingRate(read_u32(bytes, 136)),
            sampling_rates: array(bytes, read_u16(bytes, 140), read_u16(bytes, 142), 4)?,
        })
    }

    pub fn sampling_rates(&self) -> impl Iterator<Item = SamplingRate> + 'a {
        self.sampling_rates
            .as_chunks::<4>()
            .0
            .iter()
            .map(|rate| SamplingRate(u32::from_be_bytes(*rate)))
    }
}

/// A STREAM_INPUT or STREAM_OUTPUT descriptor (Table 7-8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamDescriptor<'a> {
    pub descriptor_type: DescriptorType,
    pub index: u16,
    pub object_name: &'a str,
    pub localized_description: LocalizedStringRef,
    pub clock_domain_index: u16,
    pub stream_flags: StreamFlags,
    pub current_format: StreamFormat,
    pub avb_interface_index: u16,
    /// Nanoseconds of buffering.
    pub buffer_length: u32,
    formats: &'a [u8],
}

impl<'a> StreamDescriptor<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        need(bytes, 132)?;
        Ok(Self {
            descriptor_type: DescriptorType(read_u16(bytes, 0)),
            index: read_u16(bytes, 2),
            object_name: aem_string(&bytes[4..68]),
            localized_description: LocalizedStringRef(read_u16(bytes, 68)),
            clock_domain_index: read_u16(bytes, 70),
            stream_flags: StreamFlags(read_u16(bytes, 72)),
            current_format: StreamFormat(read_u64(bytes, 74)),
            avb_interface_index: read_u16(bytes, 126),
            buffer_length: read_u32(bytes, 128),
            formats: array(bytes, read_u16(bytes, 82), read_u16(bytes, 84), 8)?,
        })
    }

    pub fn is_input(&self) -> bool {
        self.descriptor_type == DescriptorType::STREAM_INPUT
    }

    /// The formats the stream supports.
    pub fn formats(&self) -> impl Iterator<Item = StreamFormat> + 'a {
        self.formats
            .as_chunks::<8>()
            .0
            .iter()
            .map(|format| StreamFormat(u64::from_be_bytes(*format)))
    }

    /// Whether the stream carries a media clock: CRF now, or a stream
    /// that offers CRF, which an audio stream does not, sending its clock
    /// as audio whose presentation times carry it, as the Mac's media
    /// clock outputs can.
    pub fn is_media_clock(&self) -> bool {
        self.current_format.is_clock() || self.formats().any(StreamFormat::is_clock)
    }
}

/// The AVB_INTERFACE descriptor (Table 7-13).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvbInterfaceDescriptor<'a> {
    pub index: u16,
    pub object_name: &'a str,
    pub localized_description: LocalizedStringRef,
    pub mac_address: MacAddress,
    pub interface_flags: InterfaceFlags,
    pub clock_identity: ClockIdentity,
    pub priority1: u8,
    pub clock_class: u8,
    pub offset_scaled_log_variance: u16,
    pub clock_accuracy: u8,
    pub priority2: u8,
    pub domain_number: u8,
    pub log_sync_interval: i8,
    pub log_announce_interval: i8,
    pub log_pdelay_interval: i8,
    pub port_number: u16,
}

impl<'a> AvbInterfaceDescriptor<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        need(bytes, 98)?;
        Ok(Self {
            index: read_u16(bytes, 2),
            object_name: aem_string(&bytes[4..68]),
            localized_description: LocalizedStringRef(read_u16(bytes, 68)),
            mac_address: MacAddress(read_array(bytes, 70)),
            interface_flags: InterfaceFlags(read_u16(bytes, 76)),
            clock_identity: ClockIdentity(read_u64(bytes, 78)),
            priority1: bytes[86],
            clock_class: bytes[87],
            offset_scaled_log_variance: read_u16(bytes, 88),
            clock_accuracy: bytes[90],
            priority2: bytes[91],
            domain_number: bytes[92],
            log_sync_interval: bytes[93] as i8,
            log_announce_interval: bytes[94] as i8,
            log_pdelay_interval: bytes[95] as i8,
            port_number: read_u16(bytes, 96),
        })
    }
}

/// The CLOCK_SOURCE descriptor (Table 7-15).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockSourceDescriptor<'a> {
    pub index: u16,
    pub object_name: &'a str,
    pub localized_description: LocalizedStringRef,
    pub flags: ClockSourceFlags,
    pub clock_source_type: ClockSourceType,
    pub identifier: u64,
    /// The descriptor the clock comes from, such as a STREAM_INPUT.
    pub location_type: DescriptorType,
    pub location_index: u16,
}

impl<'a> ClockSourceDescriptor<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        need(bytes, 86)?;
        Ok(Self {
            index: read_u16(bytes, 2),
            object_name: aem_string(&bytes[4..68]),
            localized_description: LocalizedStringRef(read_u16(bytes, 68)),
            flags: ClockSourceFlags(read_u16(bytes, 70)),
            clock_source_type: ClockSourceType(read_u16(bytes, 72)),
            identifier: read_u64(bytes, 74),
            location_type: DescriptorType(read_u16(bytes, 82)),
            location_index: read_u16(bytes, 84),
        })
    }
}

/// The CLOCK_DOMAIN descriptor (Table 7-61).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockDomainDescriptor<'a> {
    pub index: u16,
    pub object_name: &'a str,
    pub localized_description: LocalizedStringRef,
    /// The CLOCK_SOURCE in use.
    pub clock_source_index: u16,
    sources: &'a [u8],
}

impl<'a> ClockDomainDescriptor<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        need(bytes, 76)?;
        Ok(Self {
            index: read_u16(bytes, 2),
            object_name: aem_string(&bytes[4..68]),
            localized_description: LocalizedStringRef(read_u16(bytes, 68)),
            clock_source_index: read_u16(bytes, 70),
            sources: array(bytes, read_u16(bytes, 72), read_u16(bytes, 74), 2)?,
        })
    }

    /// The CLOCK_SOURCE indexes the domain can use.
    pub fn clock_sources(&self) -> impl Iterator<Item = u16> + 'a {
        self.sources
            .as_chunks::<2>()
            .0
            .iter()
            .map(|source| u16::from_be_bytes(*source))
    }
}

/// The LOCALE descriptor (Table 7-21).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocaleDescriptor<'a> {
    pub index: u16,
    /// Such as "en-US".
    pub locale_identifier: &'a str,
    pub number_of_strings: u16,
    pub base_strings: u16,
}

impl<'a> LocaleDescriptor<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        need(bytes, 72)?;
        Ok(Self {
            index: read_u16(bytes, 2),
            locale_identifier: aem_string(&bytes[4..68]),
            number_of_strings: read_u16(bytes, 68),
            base_strings: read_u16(bytes, 70),
        })
    }
}

/// The STRINGS descriptor (Table 7-22): seven localized strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringsDescriptor<'a> {
    pub index: u16,
    pub strings: [&'a str; 7],
}

impl<'a> StringsDescriptor<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        need(bytes, 452)?;
        Ok(Self {
            index: read_u16(bytes, 2),
            strings: core::array::from_fn(|string| {
                let start = 4 + 64 * string;
                aem_string(&bytes[start..start + 64])
            }),
        })
    }
}

/// A STREAM_PORT_INPUT or STREAM_PORT_OUTPUT descriptor (Table 7-23).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamPortDescriptor {
    pub descriptor_type: DescriptorType,
    pub index: u16,
    pub clock_domain_index: u16,
    pub port_flags: u16,
    pub number_of_controls: u16,
    pub base_control: u16,
    pub number_of_clusters: u16,
    pub base_cluster: u16,
    pub number_of_maps: u16,
    pub base_map: u16,
}

impl StreamPortDescriptor {
    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        need(bytes, 20)?;
        Ok(Self {
            descriptor_type: DescriptorType(read_u16(bytes, 0)),
            index: read_u16(bytes, 2),
            clock_domain_index: read_u16(bytes, 4),
            port_flags: read_u16(bytes, 6),
            number_of_controls: read_u16(bytes, 8),
            base_control: read_u16(bytes, 10),
            number_of_clusters: read_u16(bytes, 12),
            base_cluster: read_u16(bytes, 14),
            number_of_maps: read_u16(bytes, 16),
            base_map: read_u16(bytes, 18),
        })
    }

    pub fn is_input(&self) -> bool {
        self.descriptor_type == DescriptorType::STREAM_PORT_INPUT
    }

    /// Its mappings are set with ADD_AUDIO_MAPPINGS and
    /// REMOVE_AUDIO_MAPPINGS and read with GET_AUDIO_MAP, as it has no
    /// AUDIO_MAP descriptors (7.2.13).
    pub fn has_dynamic_mappings(&self) -> bool {
        self.number_of_maps == 0
    }
}

/// An AUDIO_CLUSTER descriptor (Table 7-27): channels of a stream port,
/// the unit stream channels map to and from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioClusterDescriptor<'a> {
    pub index: u16,
    pub object_name: &'a str,
    pub localized_description: LocalizedStringRef,
    pub channel_count: u16,
}

impl<'a> AudioClusterDescriptor<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        need(bytes, 86)?;
        Ok(Self {
            index: read_u16(bytes, 2),
            object_name: aem_string(&bytes[4..68]),
            localized_description: LocalizedStringRef(read_u16(bytes, 68)),
            channel_count: read_u16(bytes, 84),
        })
    }
}

/// An AUDIO_MAP descriptor (Table 7-32): mappings fixed by the entity, on
/// a stream port without dynamic mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AudioMapDescriptor<'a> {
    pub index: u16,
    mappings: &'a [u8],
}

impl<'a> AudioMapDescriptor<'a> {
    pub fn decode(bytes: &'a [u8]) -> Result<Self, DecodeError> {
        need(bytes, 8)?;
        Ok(Self {
            index: read_u16(bytes, 2),
            mappings: array(bytes, read_u16(bytes, 4), read_u16(bytes, 6), 8)?,
        })
    }

    pub fn mappings(&self) -> impl Iterator<Item = AudioMapping> + use<'a> {
        crate::aem::each_mapping(self.mappings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_name(descriptor_type: DescriptorType, length: usize, name: &str) -> [u8; 512] {
        let mut bytes = [0; 512];
        bytes[0..2].copy_from_slice(&descriptor_type.0.to_be_bytes());
        bytes[4..4 + name.len()].copy_from_slice(name.as_bytes());
        bytes[length..].fill(0xee);
        bytes
    }

    #[test]
    fn strings_stop_at_zero_and_survive_bad_utf8() {
        assert_eq!(aem_string(b"Stage Left\0\0junk"), "Stage Left");
        assert_eq!(aem_string(&[b'a'; 64]).len(), 64);
        assert_eq!(aem_string(&[b'o', b'k', 0xff, b'x']), "ok");
    }

    #[test]
    fn string_references() {
        let reference = LocalizedStringRef((3 << 3) | 5);
        assert_eq!(reference.offset(), 3);
        assert_eq!(reference.index(), 5);
        assert!(!reference.is_none());
        assert!(LocalizedStringRef::NONE.is_none());
    }

    #[test]
    fn entity_descriptor_fields() {
        let mut bytes = with_name(DescriptorType::ENTITY, 312, "");
        bytes[4..12].copy_from_slice(&0xe8f6_0ae0_9220_0000u64.to_be_bytes());
        bytes[48..53].copy_from_slice(b"Synth");
        bytes[112..114].copy_from_slice(&8u16.to_be_bytes());
        bytes[116..121].copy_from_slice(b"2.1.0");
        bytes[180..185].copy_from_slice(b"Stage");
        bytes[310..312].copy_from_slice(&1u16.to_be_bytes());
        let entity = EntityDescriptor::decode(&bytes[..312]).unwrap();
        assert_eq!(entity.entity_id, EntityId(0xe8f6_0ae0_9220_0000));
        assert_eq!(entity.entity_name, "Synth");
        assert_eq!(entity.vendor_name, LocalizedStringRef(8));
        assert_eq!(entity.firmware_version, "2.1.0");
        assert_eq!(entity.group_name, "Stage");
        assert_eq!(entity.current_configuration, 1);
        assert!(EntityDescriptor::decode(&bytes[..311]).is_err());
    }

    #[test]
    fn configuration_counts_follow_their_offset() {
        let mut bytes = with_name(DescriptorType::CONFIGURATION, 82, "Main");
        bytes[70..72].copy_from_slice(&2u16.to_be_bytes());
        bytes[72..74].copy_from_slice(&74u16.to_be_bytes());
        bytes[74..82].copy_from_slice(&[0x00, 0x05, 0x00, 0x02, 0x00, 0x24, 0x00, 0x01]);
        let configuration = ConfigurationDescriptor::decode(&bytes[..82]).unwrap();
        assert_eq!(configuration.object_name, "Main");
        let counts: Vec<_> = configuration.descriptor_counts().collect();
        assert_eq!(
            counts,
            [
                (DescriptorType::STREAM_INPUT, 2),
                (DescriptorType::CLOCK_DOMAIN, 1)
            ]
        );
        assert!(ConfigurationDescriptor::decode(&bytes[..81]).is_err());
    }

    #[test]
    fn stream_formats_follow_their_offset() {
        // A 2013 layout: formats at 132, without the redundancy fields.
        let mut bytes = with_name(DescriptorType::STREAM_OUTPUT, 148, "Out 1");
        bytes[74..82].copy_from_slice(&0x0205_0220_0040_6000u64.to_be_bytes());
        bytes[82..84].copy_from_slice(&132u16.to_be_bytes());
        bytes[84..86].copy_from_slice(&2u16.to_be_bytes());
        bytes[132..140].copy_from_slice(&1u64.to_be_bytes());
        bytes[140..148].copy_from_slice(&2u64.to_be_bytes());
        let stream = StreamDescriptor::decode(&bytes[..148]).unwrap();
        assert!(!stream.is_input());
        assert_eq!(stream.object_name, "Out 1");
        assert_eq!(stream.current_format, StreamFormat(0x0205_0220_0040_6000));
        let formats: Vec<_> = stream.formats().collect();
        assert_eq!(formats, [StreamFormat(1), StreamFormat(2)]);
        assert!(StreamDescriptor::decode(&bytes[..147]).is_err());
    }

    #[test]
    fn clusters_and_static_maps() {
        let mut bytes = with_name(DescriptorType::AUDIO_CLUSTER, 90, "Kick");
        bytes[84..86].copy_from_slice(&2u16.to_be_bytes());
        let cluster = AudioClusterDescriptor::decode(&bytes[..90]).unwrap();
        assert_eq!((cluster.object_name, cluster.channel_count), ("Kick", 2));
        assert!(AudioClusterDescriptor::decode(&bytes[..85]).is_err());

        let mut bytes = with_name(DescriptorType::AUDIO_MAP, 24, "");
        bytes[4..6].copy_from_slice(&8u16.to_be_bytes());
        bytes[6..8].copy_from_slice(&2u16.to_be_bytes());
        bytes[8..16].copy_from_slice(&[0, 0, 0, 1, 0, 0, 0, 0]);
        bytes[16..24].copy_from_slice(&[0, 0, 0, 0, 0, 1, 0, 1]);
        let map = AudioMapDescriptor::decode(&bytes[..24]).unwrap();
        assert_eq!(
            map.mappings().collect::<Vec<_>>(),
            [
                AudioMapping {
                    stream_index: 0,
                    stream_channel: 1,
                    cluster_offset: 0,
                    cluster_channel: 0,
                },
                AudioMapping {
                    stream_index: 0,
                    stream_channel: 0,
                    cluster_offset: 1,
                    cluster_channel: 1,
                },
            ]
        );
        assert!(AudioMapDescriptor::decode(&bytes[..23]).is_err());

        let mut bytes = with_name(DescriptorType::STREAM_PORT_INPUT, 20, "");
        let port = StreamPortDescriptor::decode(&bytes[..20]).unwrap();
        assert!(port.is_input() && port.has_dynamic_mappings());
        bytes[16..18].copy_from_slice(&1u16.to_be_bytes());
        let port = StreamPortDescriptor::decode(&bytes[..20]).unwrap();
        assert!(!port.has_dynamic_mappings());
    }

    #[test]
    fn names_only_where_they_exist() {
        let bytes = with_name(DescriptorType::CLOCK_DOMAIN, 76, "Domain");
        assert_eq!(names(&bytes).map(|(name, _)| name), Some("Domain"));
        let bytes = with_name(DescriptorType::STREAM_PORT_INPUT, 20, "");
        assert_eq!(names(&bytes), None);
    }
}
