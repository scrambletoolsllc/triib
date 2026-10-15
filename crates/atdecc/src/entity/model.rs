//! The entity model a talker or listener entity serves: its descriptors,
//! built from a few settings and encoded as READ_DESCRIPTOR answers them
//! (IEEE 1722.1-2021, 7.2), with the current names, formats, sampling
//! rate and clock source.
//!
//! One configuration with one audio unit, one AVB interface and one clock
//! domain. Each audio stream has a stream port of its own on the audio
//! unit, with an audio cluster of one channel for each of the stream's
//! channels: an output's mapped to them by a static map, an input's by
//! dynamic mappings controllers change, as Milan has inputs' (the entity
//! keeps those); a media clock stream, CRF, has none. The clock sources are the internal clock, then the stream inputs
//! the model offers as clock sources.

use alloc::string::String;
use alloc::vec::Vec;

use avb_net::MacAddress;

use crate::aem::NAME_LENGTH;
use crate::descriptor::{ClockSourceType, DescriptorType, InterfaceFlags, StreamFlags};
use crate::id::{ClockIdentity, EntityId, EntityModelId};
use crate::stream_format::StreamFormat;

/// No localized string (7.3.6).
const NO_STRING: u16 = 0xffff;
/// The audio cluster format of PCM samples (Table 7-37, MBLA).
const MBLA: u8 = 0x40;
/// The IDENTIFY control type (7.3.4, Avnu's OUI-24 90-E0-F0).
pub const IDENTIFY_CONTROL_TYPE: u64 = 0x90e0_f000_0000_0001;
/// CONTROL_LINEAR_UINT8 (Table 7-41).
const LINEAR_UINT8: u16 = 0x0001;
/// The clock source location type of the internal clock: the entity.
const LOCATION_ENTITY: u16 = 0x0000;

/// A stream input or output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamModel {
    pub name: String,
    /// The formats it can take, the first the default.
    pub formats: Vec<StreamFormat>,
    pub current_format: StreamFormat,
    /// Its channels, each an audio cluster; none for a media clock.
    pub channels: u16,
}

impl StreamModel {
    /// Sent in SR class B rather than A, as its format decides: a frame
    /// each class B interval, as a stream crossing a Wi-Fi hop sends
    /// (AVB Wireless profile, 4).
    pub fn class_b(&self) -> bool {
        self.current_format.is_class_b()
    }
}

/// What a talker or listener entity is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndpointModel {
    pub entity_id: EntityId,
    pub entity_model_id: EntityModelId,
    pub entity_name: String,
    pub group_name: String,
    pub vendor_name: String,
    pub model_name: String,
    pub firmware_version: String,
    pub serial_number: String,
    pub mac: MacAddress,
    pub interface_name: String,
    /// The interface's gPTP clock identity.
    pub clock_identity: ClockIdentity,
    pub sampling_rates: Vec<u32>,
    pub current_sampling_rate: u32,
    pub outputs: Vec<StreamModel>,
    pub inputs: Vec<StreamModel>,
    /// The clock domain's current clock source: 0 the internal clock,
    /// then those of `clock_source_inputs`, in order.
    pub clock_source: u16,
    /// The stream inputs that are clock sources too, beside the internal
    /// clock: those whose media clock the entity can follow.
    pub clock_source_inputs: Vec<u16>,
}

/// Fixed-size AEM text: UTF-8, zero padded, cut at a character boundary.
pub(crate) fn name_field(text: &str) -> [u8; NAME_LENGTH] {
    let mut field = [0; NAME_LENGTH];
    let mut end = text.len().min(NAME_LENGTH);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    field[..end].copy_from_slice(&text.as_bytes()[..end]);
    field
}

/// A descriptor being written: its type and index, then fields in order.
struct Writer(Vec<u8>);

impl Writer {
    fn new(descriptor_type: DescriptorType, index: u16) -> Self {
        let mut bytes = Vec::with_capacity(320);
        bytes.extend_from_slice(&descriptor_type.0.to_be_bytes());
        bytes.extend_from_slice(&index.to_be_bytes());
        Writer(bytes)
    }

    fn u8(&mut self, value: u8) -> &mut Self {
        self.0.push(value);
        self
    }

    fn u16(&mut self, value: u16) -> &mut Self {
        self.0.extend_from_slice(&value.to_be_bytes());
        self
    }

    fn u32(&mut self, value: u32) -> &mut Self {
        self.0.extend_from_slice(&value.to_be_bytes());
        self
    }

    fn u64(&mut self, value: u64) -> &mut Self {
        self.0.extend_from_slice(&value.to_be_bytes());
        self
    }

    fn bytes(&mut self, value: &[u8]) -> &mut Self {
        self.0.extend_from_slice(value);
        self
    }

    fn name(&mut self, text: &str) -> &mut Self {
        self.bytes(&name_field(text))
    }

    /// Zero padding up to `length` octets in all.
    fn pad_to(&mut self, length: usize) -> &mut Self {
        if self.0.len() < length {
            self.0.resize(length, 0);
        }
        self
    }

    fn done(&mut self) -> Vec<u8> {
        core::mem::take(&mut self.0)
    }
}

/// Which stream port, cluster and map belong to which audio stream:
/// inputs' ports first, then outputs'.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Port {
    pub(crate) input: bool,
    /// The stream port's index, counting its way's audio streams.
    pub(crate) index: u16,
    /// The stream's index.
    pub(crate) stream: u16,
    base_cluster: u16,
    /// Its channels, each a cluster.
    pub(crate) channels: u16,
    /// Its map, by AUDIO_MAP index; none for an input's, whose mappings
    /// are dynamic.
    map: Option<u16>,
}

impl EndpointModel {
    pub(crate) fn ports(&self) -> Vec<Port> {
        let mut ports = Vec::new();
        let mut cluster = 0;
        let mut map = 0;
        for (input, streams) in [(true, &self.inputs), (false, &self.outputs)] {
            let audio = streams
                .iter()
                .enumerate()
                .filter(|(_, stream)| !stream.current_format.is_clock() && stream.channels > 0);
            for (index, (stream, model)) in audio.enumerate() {
                ports.push(Port {
                    input,
                    index: index as u16,
                    stream: stream as u16,
                    base_cluster: cluster,
                    channels: model.channels,
                    map: (!input).then_some(map),
                });
                cluster += model.channels;
                if !input {
                    map += 1;
                }
            }
        }
        ports
    }

    /// How many clock sources the clock domain has.
    pub(crate) fn clock_sources(&self) -> u16 {
        1 + self.clock_source_inputs.len() as u16
    }

    /// The stream input clock source `source` follows, none for the
    /// internal clock.
    pub fn clock_source_input(&self, source: u16) -> Option<u16> {
        let at = usize::from(source.checked_sub(1)?);
        self.clock_source_inputs.get(at).copied()
    }

    /// The descriptor types the configuration counts, with how many of
    /// each.
    pub fn descriptor_counts(&self) -> Vec<(DescriptorType, u16)> {
        let mut counts = Vec::from([(DescriptorType::AUDIO_UNIT, 1)]);
        if !self.inputs.is_empty() {
            counts.push((DescriptorType::STREAM_INPUT, self.inputs.len() as u16));
        }
        if !self.outputs.is_empty() {
            counts.push((DescriptorType::STREAM_OUTPUT, self.outputs.len() as u16));
        }
        counts.extend([
            (DescriptorType::AVB_INTERFACE, 1),
            (DescriptorType::CLOCK_SOURCE, self.clock_sources()),
            (DescriptorType::LOCALE, 1),
            (DescriptorType::CONTROL, 1),
            (DescriptorType::CLOCK_DOMAIN, 1),
        ]);
        counts
    }

    /// The localized strings: the vendor and model names.
    fn strings(&self) -> [&str; 7] {
        [&self.vendor_name, &self.model_name, "", "", "", "", ""]
    }

    /// The descriptor of `descriptor_type` at `index`, as READ_DESCRIPTOR
    /// answers it, or `None` when the entity has none.
    pub fn descriptor(
        &self,
        descriptor_type: DescriptorType,
        index: u16,
        entity: &EntityState,
    ) -> Option<Vec<u8>> {
        let ports = self.ports();
        let mut writer = Writer::new(descriptor_type, index);
        let bytes = match descriptor_type {
            DescriptorType::ENTITY if index == 0 => writer
                .u64(self.entity_id.0)
                .u64(self.entity_model_id.0)
                .u32(entity.capabilities)
                .u16(self.outputs.len() as u16)
                .u16(entity.talker_capabilities)
                .u16(self.inputs.len() as u16)
                .u16(entity.listener_capabilities)
                .u32(0)
                .u32(entity.available_index)
                .u64(0)
                .name(&self.entity_name)
                .u16(0) // vendor_name_string: strings 0
                .u16(1) // model_name_string: strings 1
                .name(&self.firmware_version)
                .name(&self.group_name)
                .name(&self.serial_number)
                .u16(1) // configurations_count
                .u16(0) // current_configuration
                .done(),
            DescriptorType::CONFIGURATION if index == 0 => {
                let counts = self.descriptor_counts();
                writer
                    .name("Configuration")
                    .u16(NO_STRING)
                    .u16(counts.len() as u16)
                    .u16(74);
                for (kind, count) in counts {
                    writer.u16(kind.0).u16(count);
                }
                writer.done()
            }
            DescriptorType::AUDIO_UNIT if index == 0 => {
                let inputs = ports.iter().filter(|port| port.input).count() as u16;
                let outputs = ports.iter().filter(|port| !port.input).count() as u16;
                writer
                    .name("Audio")
                    .u16(NO_STRING)
                    .u16(0) // clock_domain_index
                    .u16(inputs)
                    .u16(0)
                    .u16(outputs)
                    .u16(0)
                    .pad_to(136)
                    .u32(self.current_sampling_rate)
                    .u16(144)
                    .u16(self.sampling_rates.len() as u16);
                for rate in &self.sampling_rates {
                    writer.u32(*rate);
                }
                writer.done()
            }
            DescriptorType::STREAM_INPUT | DescriptorType::STREAM_OUTPUT => {
                let input = descriptor_type == DescriptorType::STREAM_INPUT;
                let streams = if input { &self.inputs } else { &self.outputs };
                let stream = streams.get(usize::from(index))?;
                // The classes its formats go in.
                let mut flags = StreamFlags::empty();
                if stream.formats.iter().any(|format| !format.is_class_b()) {
                    flags |= StreamFlags::CLASS_A;
                }
                if stream.formats.iter().any(|format| format.is_class_b()) {
                    flags |= StreamFlags::CLASS_B;
                }
                if input && self.clock_source_inputs.contains(&index) {
                    flags |= StreamFlags::CLOCK_SYNC_SOURCE;
                }
                writer
                    .name(&stream.name)
                    .u16(NO_STRING)
                    .u16(0) // clock_domain_index
                    .u16(flags.0)
                    .u64(stream.current_format.0)
                    .u16(138)
                    .u16(stream.formats.len() as u16)
                    .pad_to(126) // backup talkers, none
                    .u16(0) // avb_interface_index
                    .u32(0) // buffer_length
                    .u16(138 + 8 * stream.formats.len() as u16)
                    .u16(0) // number_of_redundant_streams
                    .u16(0); // timing
                for format in &stream.formats {
                    writer.u64(format.0);
                }
                writer.done()
            }
            DescriptorType::AVB_INTERFACE if index == 0 => writer
                .name(&self.interface_name)
                .u16(NO_STRING)
                .bytes(&self.mac.0)
                .u16((InterfaceFlags::GPTP_SUPPORTED | InterfaceFlags::SRP_SUPPORTED).0)
                .u64(self.clock_identity.0)
                .u8(255) // priority1
                .u8(248) // clock_class
                .u16(0x436a) // offset_scaled_log_variance
                .u8(0xfe) // clock_accuracy
                .u8(255) // priority2
                .u8(0) // domain_number
                .u8(0xfd) // log_sync_interval, -3
                .u8(0) // log_announce_interval
                .u8(0) // log_pdelay_interval
                .u16(1) // port_number
                .pad_to(110)
                .done(),
            DescriptorType::CLOCK_SOURCE if index < self.clock_sources() => {
                if index == 0 {
                    writer
                        .name("Internal")
                        .u16(NO_STRING)
                        .u16(0)
                        .u16(ClockSourceType::INTERNAL.0)
                        .u64(0)
                        .u16(LOCATION_ENTITY)
                        .u16(0)
                } else {
                    let input = self.clock_source_input(index)?;
                    let name = self.inputs.get(usize::from(input))?.name.as_str();
                    writer
                        .name(name)
                        .u16(NO_STRING)
                        .u16(0)
                        .u16(ClockSourceType::INPUT_STREAM.0)
                        .u64(0)
                        .u16(DescriptorType::STREAM_INPUT.0)
                        .u16(input)
                };
                writer.pad_to(94).done()
            }
            DescriptorType::CLOCK_DOMAIN if index == 0 => {
                writer
                    .name("Clock Domain")
                    .u16(NO_STRING)
                    .u16(self.clock_source)
                    .u16(76)
                    .u16(self.clock_sources());
                for source in 0..self.clock_sources() {
                    writer.u16(source);
                }
                writer.pad_to(88).done()
            }
            DescriptorType::LOCALE if index == 0 => writer
                .name("en")
                .u16(1) // number_of_strings
                .u16(0) // base_strings
                .pad_to(80)
                .done(),
            DescriptorType::STRINGS if index == 0 => {
                for text in self.strings() {
                    writer.name(text);
                }
                writer.pad_to(460).done()
            }
            DescriptorType::STREAM_PORT_INPUT | DescriptorType::STREAM_PORT_OUTPUT => {
                let input = descriptor_type == DescriptorType::STREAM_PORT_INPUT;
                let port = ports
                    .iter()
                    .find(|port| port.input == input && port.index == index)?;
                writer
                    .u16(0) // clock_domain_index
                    .u16(0) // port_flags
                    .u16(0) // number_of_controls
                    .u16(0) // base_control
                    .u16(port.channels)
                    .u16(port.base_cluster)
                    .u16(u16::from(port.map.is_some())) // number_of_maps
                    .u16(port.map.unwrap_or(0))
                    .pad_to(28)
                    .done()
            }
            DescriptorType::AUDIO_CLUSTER => {
                let port = ports.iter().find(|port| {
                    (port.base_cluster..port.base_cluster + port.channels).contains(&index)
                })?;
                let channel = index - port.base_cluster + 1;
                let mut name = String::from("Channel ");
                push_number(&mut name, channel);
                writer
                    .name(&name)
                    .u16(NO_STRING)
                    .u16(0xffff) // signal_type: none
                    .u16(0) // signal_index
                    .u16(0) // signal_output
                    .u32(0) // path_latency
                    .u32(0) // block_latency
                    .u16(1) // channel_count
                    .u8(MBLA)
                    .pad_to(100)
                    .done()
            }
            DescriptorType::AUDIO_MAP => {
                let port = ports.iter().find(|port| port.map == Some(index))?;
                writer.u16(8).u16(port.channels);
                for channel in 0..port.channels {
                    writer.u16(port.stream).u16(channel).u16(channel).u16(0);
                }
                writer.pad_to(24).done()
            }
            DescriptorType::CONTROL if index == 0 => writer
                .name("Identify")
                .u16(NO_STRING)
                .u32(0) // block_latency
                .u32(0) // control_latency
                .u16(0) // control_domain
                .u16(LINEAR_UINT8)
                .u64(IDENTIFY_CONTROL_TYPE)
                .u32(0) // reset_time
                .u16(104) // values_offset
                .u16(1) // number_of_values
                .u16(DescriptorType::ENTITY.0) // signal_type
                .u16(0) // signal_index
                .u16(0) // signal_output
                .u8(0) // minimum
                .u8(255) // maximum
                .u8(255) // step
                .u8(0) // default
                .u8(if entity.identifying { 255 } else { 0 })
                .u16(0) // unit
                .u16(NO_STRING)
                .pad_to(212)
                .done(),
            _ => return None,
        };
        Some(bytes)
    }
}

/// The parts of the ENTITY descriptor that come from the running entity
/// rather than its model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EntityState {
    pub capabilities: u32,
    pub talker_capabilities: u16,
    pub listener_capabilities: u16,
    pub available_index: u32,
    pub identifying: bool,
}

/// Appends the decimal digits of `number`.
fn push_number(text: &mut String, number: u16) {
    let mut digits = [0u8; 5];
    let mut place = digits.len();
    let mut rest = number;
    loop {
        place -= 1;
        digits[place] = b'0' + (rest % 10) as u8;
        rest /= 10;
        if rest == 0 {
            break;
        }
    }
    for digit in &digits[place..] {
        text.push(char::from(*digit));
    }
}
