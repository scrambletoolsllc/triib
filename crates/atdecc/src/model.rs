//! What the controller has read of an entity: its descriptors, kept as
//! the octets the entity sent, with typed views over them, and its Milan
//! information.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

use avb_mrp::msrp::TalkerDeclaration;
use avb_net::MacAddress;

use crate::acmp::AcmpFlags;
use crate::aecp::AemStatus;
use crate::aem::{
    AsPath, AudioMap, AudioMapping, AudioMappings, AvbInfo, ClockDomainCounters, Counters,
    MappingChange, SetClockSource, SetName, SetSamplingRate, SetStreamFormat, StreamInfo,
    StreamInputCounters, StreamOutputCounters,
};
use crate::control::ControlDescriptor;
use crate::descriptor::{
    AudioClusterDescriptor, AudioMapDescriptor, AudioUnitDescriptor, AvbInterfaceDescriptor,
    ClockDomainDescriptor, ClockSourceDescriptor, ConfigurationDescriptor, DescriptorType,
    EntityDescriptor, LocaleDescriptor, LocalizedStringRef, SamplingRate, StreamDescriptor,
    StreamPortDescriptor, StringsDescriptor, names,
};
use crate::id::{ClockIdentity, EntityId, StreamId};
use crate::lite::LiteStatus;
use crate::mvu::{MediaClockReference, MilanInfo};
use crate::time::Instant;
use crate::wireless::{Station, WirelessStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EnumerationState {
    /// Not read, as the entity does not support AEM or enumeration is off.
    #[default]
    NotRead,
    Reading,
    Complete,
    Failed(EnumerationFailure),
}

/// Why the ENTITY or CONFIGURATION descriptor could not be read, without
/// which nothing else can be.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnumerationFailure {
    /// No response, after a retry.
    NoResponse,
    /// The entity answered with an error.
    Refused(AemStatus),
    /// The response did not decode.
    Malformed,
    /// The entity runs on the controller's own computer, whose system
    /// never sees the commands the controller sends (macOS).
    OnThisComputer,
}

/// What a listener says one of its stream inputs is bound to, from its
/// GET_RX_STATE, CONNECT_RX (BIND_RX) and DISCONNECT_RX (UNBIND_RX)
/// responses to any controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Binding {
    pub talker: EntityId,
    pub talker_unique_id: u16,
    /// 1 when bound (Milan) or connected (IEEE 1722.1), else 0.
    pub connection_count: u16,
    pub flags: AcmpFlags,
}

impl Binding {
    /// The talker and stream output the input is bound to.
    pub fn talker_stream(&self) -> Option<(EntityId, u16)> {
        (self.connection_count > 0 && self.talker.is_valid())
            .then_some((self.talker, self.talker_unique_id))
    }
}

/// What a talker says of a stream output in a GET_TX_STATE response
/// (8.2.2.6.3): the stream, where it goes and how many listen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxState {
    pub stream_id: StreamId,
    pub destination: MacAddress,
    pub connection_count: u16,
    /// Zero when the SRP domain's VLAN applies.
    pub vlan_id: u16,
}

/// A stream port's dynamic mappings.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct DynamicMap {
    /// As last read in full, then changed by notifications and the
    /// controller's own changes; `None` until read.
    mappings: Option<BTreeSet<AudioMapping>>,
    /// The parts of a read under way.
    reading: BTreeSet<AudioMapping>,
}

/// What an entity reports of a wireless AVB_INTERFACE (AVB Wireless
/// profile, 5.1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wireless {
    pub status: WirelessStatus,
    /// An access point's associated stations.
    pub stations: Vec<Station>,
}

/// What one of an entity's configurations holds, for choosing among them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationSummary<'a> {
    pub index: u16,
    /// Its own name or its localized description; empty when it has
    /// neither.
    pub name: &'a str,
    pub inputs: Vec<StreamDescriptor<'a>>,
    pub outputs: Vec<StreamDescriptor<'a>>,
    /// Whether its descriptor and all its streams are read.
    pub read: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EntityModel {
    pub state: EnumerationState,
    /// From GET_MILAN_INFO; `None` for entities that are not Milan or did
    /// not answer.
    pub milan: Option<MilanInfo>,
    /// The configuration whose descriptors were read.
    pub configuration: u16,
    /// Descriptors that could not be read (not the ENTITY or
    /// CONFIGURATION, whose failure fails enumeration).
    pub failed_reads: u16,
    /// Registered for unsolicited notifications.
    pub registered: bool,
    /// Its descriptors came from a model cached for its entity model, with
    /// what changes on an entity read from it.
    pub from_cache: bool,
    descriptors: BTreeMap<(DescriptorType, u16), Vec<u8>>,
    /// Its other configurations' CONFIGURATION and stream descriptors,
    /// read to tell them apart: by configuration, type and index.
    others: BTreeMap<(u16, DescriptorType, u16), Vec<u8>>,
    bindings: BTreeMap<u16, Binding>,
    stream_info: BTreeMap<(DescriptorType, u16), StreamInfo>,
    avb_info: BTreeMap<u16, AvbInfo>,
    as_paths: BTreeMap<u16, Vec<ClockIdentity>>,
    counters: BTreeMap<(DescriptorType, u16), Counters>,
    audio_maps: BTreeMap<(DescriptorType, u16), DynamicMap>,
    media_clock_references: BTreeMap<u16, MediaClockReference>,
    tx_states: BTreeMap<u16, TxState>,
    max_transit_times: BTreeMap<u16, u64>,
    /// Whether it answers the AVB Lite status query; `None` until asked.
    pub lite_supported: Option<bool>,
    lite_status: BTreeMap<u16, LiteStatus>,
    /// Whether it answers the AVB Wireless status query; `None` until
    /// asked.
    pub wireless_supported: Option<bool>,
    /// Status queries in a row that went unanswered, while it is not
    /// known whether it answers.
    pub(crate) wireless_misses: u8,
    wireless: BTreeMap<u16, Wireless>,
    /// The streams it declares over CVU SRP, by stream ID, with when each
    /// was last heard.
    cvu_talkers: BTreeMap<u64, (TalkerDeclaration, Instant)>,
}

impl EntityModel {
    pub(crate) fn reading() -> Self {
        Self {
            state: EnumerationState::Reading,
            ..Self::default()
        }
    }

    /// Stores a descriptor, returning whether it differs from the one
    /// stored before.
    pub(crate) fn store(
        &mut self,
        descriptor_type: DescriptorType,
        index: u16,
        bytes: &[u8],
    ) -> bool {
        let key = (descriptor_type, index);
        if self
            .descriptors
            .get(&key)
            .is_some_and(|stored| stored == bytes)
        {
            return false;
        }
        self.descriptors.insert(key, bytes.to_vec());
        true
    }

    /// Writes `bytes` into a stored descriptor at `offset`, returning
    /// whether that changed it; false when the descriptor is not stored
    /// or too short.
    fn patch(
        &mut self,
        descriptor_type: DescriptorType,
        index: u16,
        offset: usize,
        bytes: &[u8],
    ) -> bool {
        let Some(target) = self
            .descriptors
            .get_mut(&(descriptor_type, index))
            .and_then(|stored| stored.get_mut(offset..offset + bytes.len()))
        else {
            return false;
        };
        if target == bytes {
            return false;
        }
        target.copy_from_slice(bytes);
        true
    }

    /// Applies what SET_NAME set, returning whether the model changed.
    pub(crate) fn apply_name(&mut self, set: &SetName) -> bool {
        let offset = match (set.descriptor_type, set.name_index) {
            (DescriptorType::ENTITY, 0) => 48,
            (DescriptorType::ENTITY, 1) => 180,
            (descriptor_type, 0) if descriptor_type.has_object_name() => 4,
            _ => return false,
        };
        self.patch(set.descriptor_type, set.index, offset, &set.name)
    }

    /// Applies what SET_STREAM_FORMAT set.
    pub(crate) fn apply_stream_format(&mut self, set: &SetStreamFormat) -> bool {
        matches!(
            set.descriptor_type,
            DescriptorType::STREAM_INPUT | DescriptorType::STREAM_OUTPUT
        ) && self.patch(
            set.descriptor_type,
            set.index,
            74,
            &set.format.0.to_be_bytes(),
        )
    }

    /// Applies what SET_SAMPLING_RATE set to an AUDIO_UNIT.
    pub(crate) fn apply_sampling_rate(&mut self, set: &SetSamplingRate) -> bool {
        set.descriptor_type == DescriptorType::AUDIO_UNIT
            && self.patch(
                set.descriptor_type,
                set.index,
                136,
                &set.rate.0.to_be_bytes(),
            )
    }

    /// Applies what SET_CLOCK_SOURCE set.
    pub(crate) fn apply_clock_source(&mut self, set: &SetClockSource) -> bool {
        self.patch(
            DescriptorType::CLOCK_DOMAIN,
            set.domain,
            70,
            &set.source.to_be_bytes(),
        )
    }

    /// Takes the current values SET_CONTROL or GET_CONTROL carries for a
    /// control, returning whether they changed.
    pub(crate) fn apply_control(&mut self, index: u16, values: &[u8]) -> bool {
        let Some(bytes) = self.descriptors.get_mut(&(DescriptorType::CONTROL, index)) else {
            return false;
        };
        let before = bytes.clone();
        crate::control::write_current(bytes, values) && *bytes != before
    }

    /// Records what the talker says of a stream output, returning whether
    /// it changed.
    pub(crate) fn set_tx_state(&mut self, output: u16, state: TxState) -> bool {
        self.tx_states.insert(output, state) != Some(state)
    }

    /// Records a stream output's max transit time, returning whether it
    /// changed.
    pub(crate) fn set_max_transit_time(&mut self, output: u16, nanoseconds: u64) -> bool {
        self.max_transit_times.insert(output, nanoseconds) != Some(nanoseconds)
    }

    /// Records what the entity reports of AVB Lite on an interface,
    /// returning whether it changed.
    pub(crate) fn set_lite_status(&mut self, status: LiteStatus) -> bool {
        self.lite_status.insert(status.interface, status) != Some(status)
    }

    /// Records what the entity reports of a wireless interface, returning
    /// whether it changed.
    pub(crate) fn set_wireless(&mut self, wireless: Wireless) -> bool {
        let interface = wireless.status.interface;
        if self.wireless.get(&interface) == Some(&wireless) {
            return false;
        }
        self.wireless.insert(interface, wireless);
        true
    }

    /// Records a stream the entity declares over CVU SRP, heard `now`,
    /// returning whether the declaration is new or changed.
    pub(crate) fn set_cvu_talker(&mut self, declaration: TalkerDeclaration, now: Instant) -> bool {
        self.cvu_talkers
            .insert(declaration.stream_id, (declaration, now))
            .is_none_or(|(before, _)| before != declaration)
    }

    /// Forgets a stream the entity withdrew, returning whether it held it.
    pub(crate) fn remove_cvu_talker(&mut self, stream_id: u64) -> bool {
        self.cvu_talkers.remove(&stream_id).is_some()
    }

    /// Forgets the streams last heard at or before `before`, returning
    /// whether any were.
    pub(crate) fn age_cvu_talkers(&mut self, before: Instant) -> bool {
        let held = self.cvu_talkers.len();
        self.cvu_talkers.retain(|_, (_, heard)| *heard > before);
        self.cvu_talkers.len() != held
    }

    /// When the stream heard longest ago was heard.
    pub(crate) fn oldest_cvu_talker(&self) -> Option<Instant> {
        self.cvu_talkers.values().map(|(_, heard)| *heard).min()
    }

    /// Records a stream input's binding, returning whether it changed.
    pub(crate) fn set_binding(&mut self, input: u16, binding: Binding) -> bool {
        self.bindings.insert(input, binding) != Some(binding)
    }

    /// Records a stream's info, returning whether it changed.
    pub(crate) fn set_stream_info(&mut self, info: StreamInfo) -> bool {
        self.stream_info
            .insert((info.descriptor_type, info.index), info)
            != Some(info)
    }

    /// Records an AVB interface's gPTP and SRP state, returning whether it
    /// changed.
    pub(crate) fn set_avb_info(&mut self, info: AvbInfo) -> bool {
        self.avb_info.insert(info.index, info) != Some(info)
    }

    /// Records an AVB interface's gPTP path, returning whether it changed.
    pub(crate) fn set_as_path(&mut self, path: &AsPath<'_>) -> bool {
        let identities: Vec<ClockIdentity> = path.clock_identities().collect();
        if self.as_paths.get(&path.index) == Some(&identities) {
            return false;
        }
        self.as_paths.insert(path.index, identities);
        true
    }

    /// Records a descriptor's counters, returning whether they changed.
    pub(crate) fn set_counters(&mut self, counters: Counters) -> bool {
        self.counters
            .insert((counters.descriptor_type, counters.index), counters)
            != Some(counters)
    }

    /// Records one part of a stream port's dynamic mappings, as
    /// GET_AUDIO_MAP answered; once the last part is in, returns whether
    /// they differ from those known before.
    pub(crate) fn set_audio_map(&mut self, map: &AudioMap<'_>) -> bool {
        let entry = self
            .audio_maps
            .entry((map.descriptor_type, map.index))
            .or_default();
        if map.map_index == 0 {
            entry.reading.clear();
        }
        entry.reading.extend(map.mappings());
        if map.map_index.saturating_add(1) < map.number_of_maps {
            return false;
        }
        let read = core::mem::take(&mut entry.reading);
        let changed = entry.mappings.as_ref() != Some(&read);
        entry.mappings = Some(read);
        changed
    }

    /// Applies mappings an entity added or removed to a stream port read
    /// before, returning whether the model changed.
    pub(crate) fn apply_mappings(
        &mut self,
        change: MappingChange,
        changed: &AudioMappings<'_>,
    ) -> bool {
        let Some(mappings) = self
            .audio_maps
            .get_mut(&(changed.descriptor_type, changed.index))
            .and_then(|map| map.mappings.as_mut())
        else {
            return false;
        };
        changed.mappings().fold(false, |any, mapping| {
            let applied = match change {
                MappingChange::Add => mappings.insert(mapping),
                MappingChange::Remove => mappings.remove(&mapping),
            };
            any | applied
        })
    }

    /// Records a clock domain's media clock reference information,
    /// returning whether it changed.
    pub(crate) fn set_media_clock_reference(&mut self, reference: MediaClockReference) -> bool {
        self.media_clock_references
            .insert(reference.domain, reference)
            != Some(reference)
    }

    /// A clock domain's media clock reference priority and domain name,
    /// once read from a Milan entity.
    pub fn media_clock_reference(&self, domain: u16) -> Option<&MediaClockReference> {
        self.media_clock_references.get(&domain)
    }

    /// The sampling rate of the first audio unit a clock domain clocks.
    pub fn sampling_rate(&self, domain: u16) -> Option<SamplingRate> {
        self.audio_units()
            .find(|unit| unit.clock_domain_index == domain)
            .map(|unit| unit.current_sampling_rate)
    }

    /// What the stream input with `index` is bound to, once known.
    pub fn binding(&self, input: u16) -> Option<&Binding> {
        self.bindings.get(&input)
    }

    /// What the entity reports of AVB Lite on an AVB_INTERFACE.
    pub fn lite_status(&self, interface: u16) -> Option<&LiteStatus> {
        self.lite_status.get(&interface)
    }

    /// What the entity reports of a wireless AVB_INTERFACE.
    pub fn wireless(&self, interface: u16) -> Option<&Wireless> {
        self.wireless.get(&interface)
    }

    /// Each wireless AVB_INTERFACE the entity reports, by index.
    pub fn wireless_interfaces(&self) -> impl Iterator<Item = (u16, &Wireless)> {
        self.wireless
            .iter()
            .map(|(&interface, wireless)| (interface, wireless))
    }

    /// The streams the entity declares over CVU SRP.
    pub fn cvu_talkers(&self) -> impl Iterator<Item = &TalkerDeclaration> {
        self.cvu_talkers
            .values()
            .map(|(declaration, _)| declaration)
    }

    /// What the talker last said of a stream output, when asked.
    pub fn tx_state(&self, output: u16) -> Option<&TxState> {
        self.tx_states.get(&output)
    }

    /// A stream output's max transit time in nanoseconds, once read.
    pub fn max_transit_time(&self, output: u16) -> Option<u64> {
        self.max_transit_times.get(&output).copied()
    }

    /// The dynamic state of a STREAM_INPUT or STREAM_OUTPUT, once read.
    pub fn stream_info(&self, descriptor_type: DescriptorType, index: u16) -> Option<&StreamInfo> {
        self.stream_info.get(&(descriptor_type, index))
    }

    /// An AVB interface's gPTP and SRP state, once read.
    pub fn avb_info(&self, index: u16) -> Option<&AvbInfo> {
        self.avb_info.get(&index)
    }

    /// The gPTP instances from the grandmaster to an AVB interface, once
    /// read.
    pub fn as_path(&self, index: u16) -> Option<&[ClockIdentity]> {
        self.as_paths.get(&index).map(Vec::as_slice)
    }

    /// A descriptor's counters, once read.
    pub fn counters(&self, descriptor_type: DescriptorType, index: u16) -> Option<&Counters> {
        self.counters.get(&(descriptor_type, index))
    }

    /// A stream input's counters, once read.
    pub fn stream_input_counters(&self, index: u16) -> Option<StreamInputCounters> {
        self.counters(DescriptorType::STREAM_INPUT, index)?
            .stream_input()
    }

    /// A stream output's counters, once read, numbered as the entity's
    /// Milan version has them.
    pub fn stream_output_counters(&self, index: u16) -> Option<StreamOutputCounters> {
        let before_milan_1_3 = self
            .milan
            .is_some_and(|milan| milan.specification_version.is_none());
        self.counters(DescriptorType::STREAM_OUTPUT, index)?
            .stream_output(before_milan_1_3)
    }

    /// A clock domain's counters, once read.
    pub fn clock_domain_counters(&self, index: u16) -> Option<ClockDomainCounters> {
        self.counters(DescriptorType::CLOCK_DOMAIN, index)?
            .clock_domain()
    }

    pub(crate) fn has(&self, descriptor_type: DescriptorType, index: u16) -> bool {
        self.descriptors.contains_key(&(descriptor_type, index))
    }

    /// A descriptor's octets, from its descriptor_type field on.
    pub fn descriptor(&self, descriptor_type: DescriptorType, index: u16) -> Option<&[u8]> {
        self.descriptors
            .get(&(descriptor_type, index))
            .map(Vec::as_slice)
    }

    /// The descriptors of one type, by index.
    pub fn descriptors(
        &self,
        descriptor_type: DescriptorType,
    ) -> impl Iterator<Item = (u16, &[u8])> {
        self.descriptors
            .range((descriptor_type, 0)..=(descriptor_type, u16::MAX))
            .map(|(&(_, index), bytes)| (index, bytes.as_slice()))
    }

    /// Every descriptor, by type and index.
    pub fn all_descriptors(&self) -> impl Iterator<Item = (DescriptorType, u16, &[u8])> {
        self.descriptors
            .iter()
            .map(|(&(descriptor_type, index), bytes)| (descriptor_type, index, bytes.as_slice()))
    }

    /// How many descriptors were read.
    pub fn descriptor_count(&self) -> usize {
        self.descriptors.len()
    }

    pub fn entity(&self) -> Option<EntityDescriptor<'_>> {
        EntityDescriptor::decode(self.descriptor(DescriptorType::ENTITY, 0)?).ok()
    }

    /// Stores a descriptor of another configuration than the one read.
    pub(crate) fn store_other(
        &mut self,
        configuration: u16,
        descriptor_type: DescriptorType,
        index: u16,
        bytes: &[u8],
    ) {
        self.others
            .insert((configuration, descriptor_type, index), bytes.to_vec());
    }

    /// What each of its configurations holds, as far as it is read: the
    /// current one from its own descriptors, the others from those read
    /// of them. Empty unless it has more than one.
    pub fn configurations(&self) -> Vec<ConfigurationSummary<'_>> {
        let Some(entity) = self
            .entity()
            .filter(|entity| entity.configurations_count > 1)
        else {
            return Vec::new();
        };
        (0..entity.configurations_count)
            .map(|index| {
                let (descriptor, inputs, outputs) = if index == self.configuration {
                    (
                        self.configuration(),
                        self.streams(true).collect(),
                        self.streams(false).collect(),
                    )
                } else {
                    let streams = |descriptor_type| -> Vec<StreamDescriptor<'_>> {
                        self.others
                            .range((index, descriptor_type, 0)..=(index, descriptor_type, u16::MAX))
                            .filter_map(|(_, bytes)| StreamDescriptor::decode(bytes).ok())
                            .collect()
                    };
                    (
                        self.others
                            .get(&(index, DescriptorType::CONFIGURATION, index))
                            .and_then(|bytes| ConfigurationDescriptor::decode(bytes).ok()),
                        streams(DescriptorType::STREAM_INPUT),
                        streams(DescriptorType::STREAM_OUTPUT),
                    )
                };
                let count = |descriptor_type| {
                    descriptor.map_or(0, |descriptor| {
                        descriptor
                            .descriptor_counts()
                            .find(|(found, _)| *found == descriptor_type)
                            .map_or(0, |(_, count)| usize::from(count))
                    })
                };
                let read = descriptor.is_some()
                    && inputs.len() == count(DescriptorType::STREAM_INPUT)
                    && outputs.len() == count(DescriptorType::STREAM_OUTPUT);
                let name = descriptor.map_or("", |descriptor| {
                    if descriptor.object_name.is_empty() {
                        self.localized(descriptor.localized_description)
                            .unwrap_or_default()
                    } else {
                        descriptor.object_name
                    }
                });
                ConfigurationSummary {
                    index,
                    name,
                    inputs,
                    outputs,
                    read,
                }
            })
            .collect()
    }

    pub fn configuration(&self) -> Option<ConfigurationDescriptor<'_>> {
        ConfigurationDescriptor::decode(
            self.descriptor(DescriptorType::CONFIGURATION, self.configuration)?,
        )
        .ok()
    }

    pub fn audio_units(&self) -> impl Iterator<Item = AudioUnitDescriptor<'_>> {
        self.descriptors(DescriptorType::AUDIO_UNIT)
            .filter_map(|(_, bytes)| AudioUnitDescriptor::decode(bytes).ok())
    }

    /// The STREAM_INPUT descriptors when `input`, else STREAM_OUTPUT.
    pub fn streams(&self, input: bool) -> impl Iterator<Item = StreamDescriptor<'_>> {
        let descriptor_type = if input {
            DescriptorType::STREAM_INPUT
        } else {
            DescriptorType::STREAM_OUTPUT
        };
        self.descriptors(descriptor_type)
            .filter_map(|(_, bytes)| StreamDescriptor::decode(bytes).ok())
    }

    /// The STREAM_PORT_INPUT descriptors when `input`, else
    /// STREAM_PORT_OUTPUT.
    pub fn stream_ports(&self, input: bool) -> impl Iterator<Item = StreamPortDescriptor> + '_ {
        let descriptor_type = if input {
            DescriptorType::STREAM_PORT_INPUT
        } else {
            DescriptorType::STREAM_PORT_OUTPUT
        };
        self.descriptors(descriptor_type)
            .filter_map(|(_, bytes)| StreamPortDescriptor::decode(bytes).ok())
    }

    /// A stream port's audio clusters, by their offset from its
    /// base_cluster, as mappings count them.
    pub fn audio_clusters(
        &self,
        port: &StreamPortDescriptor,
    ) -> impl Iterator<Item = (u16, AudioClusterDescriptor<'_>)> {
        let base = port.base_cluster;
        (0..port.number_of_clusters).filter_map(move |offset| {
            let bytes =
                self.descriptor(DescriptorType::AUDIO_CLUSTER, base.checked_add(offset)?)?;
            Some((offset, AudioClusterDescriptor::decode(bytes).ok()?))
        })
    }

    /// A stream port's dynamic mappings, once read: `None` before, and
    /// for ports whose mappings are fixed.
    pub fn dynamic_mappings(
        &self,
        descriptor_type: DescriptorType,
        index: u16,
    ) -> Option<impl Iterator<Item = AudioMapping> + '_> {
        let mappings = self.audio_maps.get(&(descriptor_type, index))?;
        Some(mappings.mappings.as_ref()?.iter().copied())
    }

    /// The mappings a stream port's AUDIO_MAP descriptors fix.
    pub fn static_mappings(
        &self,
        port: &StreamPortDescriptor,
    ) -> impl Iterator<Item = AudioMapping> + '_ {
        let maps = port.base_map..port.base_map.saturating_add(port.number_of_maps);
        maps.filter_map(|index| self.descriptor(DescriptorType::AUDIO_MAP, index))
            .filter_map(|bytes| AudioMapDescriptor::decode(bytes).ok())
            .flat_map(|map| map.mappings())
    }

    pub fn avb_interfaces(&self) -> impl Iterator<Item = AvbInterfaceDescriptor<'_>> {
        self.descriptors(DescriptorType::AVB_INTERFACE)
            .filter_map(|(_, bytes)| AvbInterfaceDescriptor::decode(bytes).ok())
    }

    pub fn clock_sources(&self) -> impl Iterator<Item = ClockSourceDescriptor<'_>> {
        self.descriptors(DescriptorType::CLOCK_SOURCE)
            .filter_map(|(_, bytes)| ClockSourceDescriptor::decode(bytes).ok())
    }

    pub fn clock_domains(&self) -> impl Iterator<Item = ClockDomainDescriptor<'_>> {
        self.descriptors(DescriptorType::CLOCK_DOMAIN)
            .filter_map(|(_, bytes)| ClockDomainDescriptor::decode(bytes).ok())
    }

    /// The configuration's controls, wherever in it they act.
    pub fn controls(&self) -> impl Iterator<Item = ControlDescriptor<'_>> {
        self.descriptors(DescriptorType::CONTROL)
            .filter_map(|(_, bytes)| ControlDescriptor::decode(bytes).ok())
    }

    pub fn control(&self, index: u16) -> Option<ControlDescriptor<'_>> {
        ControlDescriptor::decode(self.descriptor(DescriptorType::CONTROL, index)?).ok()
    }

    /// A localized string, from the first locale.
    pub fn localized(&self, reference: LocalizedStringRef) -> Option<&str> {
        if reference.is_none() {
            return None;
        }
        let (_, locale) = self.descriptors(DescriptorType::LOCALE).next()?;
        let locale = LocaleDescriptor::decode(locale).ok()?;
        let strings = locale.base_strings.checked_add(reference.offset())?;
        let strings =
            StringsDescriptor::decode(self.descriptor(DescriptorType::STRINGS, strings)?).ok()?;
        strings
            .strings
            .get(usize::from(reference.index()))
            .copied()
            .filter(|text| !text.is_empty())
    }

    /// The name to show for a descriptor: its object_name when someone set
    /// one, else its localized description.
    pub fn name_of(&self, descriptor_type: DescriptorType, index: u16) -> Option<&str> {
        let (name, description) = names(self.descriptor(descriptor_type, index)?)?;
        if name.is_empty() {
            self.localized(description)
        } else {
            Some(name)
        }
    }

    /// The entity's name, when it has one.
    pub fn entity_name(&self) -> Option<&str> {
        self.entity()
            .map(|entity| entity.entity_name)
            .filter(|name| !name.is_empty())
    }
}
