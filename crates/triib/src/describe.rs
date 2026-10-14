//! Words and icons for what an entity advertises, shared by the entity
//! list, the table and the inspector.

use atdecc::aecp::AemStatus;
use atdecc::aem::MappingChange;
use atdecc::controller::{Outcome, Refusal};
use atdecc::model::{EntityModel, EnumerationFailure, EnumerationState};
use atdecc::{DescriptorType, EntityId};

use crate::app::Triib;
use crate::fl;
use crate::network::Action;
use atdecc::{
    Adpdu, ControllerCapabilities, EntityCapabilities, ListenerCapabilities, TalkerCapabilities,
};
use scramble_ui::icon::Icon;

pub fn is_talker(adp: &Adpdu) -> bool {
    adp.talker_capabilities
        .contains(TalkerCapabilities::IMPLEMENTED)
}

pub fn is_listener(adp: &Adpdu) -> bool {
    adp.listener_capabilities
        .contains(ListenerCapabilities::IMPLEMENTED)
}

pub fn is_controller(adp: &Adpdu) -> bool {
    adp.controller_capabilities
        .contains(ControllerCapabilities::IMPLEMENTED)
}

/// An icon for the entity's roles.
pub fn glyph(adp: &Adpdu) -> Icon {
    match (is_talker(adp), is_listener(adp)) {
        (true, true) => Icon::GraphicEq,
        (true, false) => Icon::Mic,
        (false, true) => Icon::Speaker,
        (false, false) if is_controller(adp) => Icon::Traffic,
        (false, false) => Icon::Lan,
    }
}

/// "Talker 2, listener 2", "Controller" and the like.
pub fn roles(adp: &Adpdu) -> String {
    let mut roles = Vec::new();
    if is_talker(adp) {
        roles.push(fl!("role-talker", count = adp.talker_stream_sources));
    }
    if is_listener(adp) {
        roles.push(fl!("role-listener", count = adp.listener_stream_sinks));
    }
    if is_controller(adp) {
        roles.push(fl!("role-controller"));
    }
    capitalized(&if roles.is_empty() {
        fl!("role-none")
    } else {
        crate::i18n::list(roles)
    })
}

/// The SR classes it supports.
pub fn classes(adp: &Adpdu) -> String {
    let capabilities = adp.entity_capabilities;
    match (
        capabilities.contains(EntityCapabilities::CLASS_A_SUPPORTED),
        capabilities.contains(EntityCapabilities::CLASS_B_SUPPORTED),
    ) {
        (true, true) => fl!("classes-a-and-b"),
        (true, false) => "A".to_owned(),
        (false, true) => "B".to_owned(),
        (false, false) => fl!("common-none"),
    }
}

/// Its gPTP BTC, or that it has none.
pub fn clock(adp: &Adpdu) -> String {
    if adp
        .entity_capabilities
        .contains(EntityCapabilities::GPTP_SUPPORTED)
    {
        adp.gptp_grandmaster_id.to_string()
    } else {
        fl!("clock-no-gptp")
    }
}

/// The media clock the entity follows: its first clock domain's current
/// clock source, by name.
pub fn media_clock(model: &EntityModel) -> String {
    model
        .clock_domains()
        .next()
        .map(|domain| {
            model
                .name_of(DescriptorType::CLOCK_SOURCE, domain.clock_source_index)
                .map_or_else(
                    || fl!("clock-source-numbered", index = domain.clock_source_index),
                    str::to_owned,
                )
        })
        .unwrap_or_default()
}

/// Nanoseconds as milliseconds, with no more decimals than they need:
/// "12 ms", "2.5 ms".
pub fn milliseconds(nanoseconds: u64) -> String {
    let text = format!("{:.3}", nanoseconds as f64 / 1e6);
    let text = text.trim_end_matches('0').trim_end_matches('.');
    format!("{} ms", crate::i18n::decimal(text.to_owned()))
}

/// How far reading an entity's model got, in a few words.
pub fn read_state(model: Option<&EntityModel>) -> String {
    let Some(model) = model else {
        return fl!("read-not-read");
    };
    match model.state {
        EnumerationState::NotRead => fl!("read-not-read"),
        EnumerationState::Reading => fl!("read-reading", count = model.descriptor_count()),
        EnumerationState::Complete if model.failed_reads > 0 => {
            fl!("read-ready-unreadable", count = model.failed_reads)
        }
        EnumerationState::Complete if model.from_cache => fl!("read-ready-cached"),
        EnumerationState::Complete => fl!("read-ready"),
        EnumerationState::Failed(why) => fl!("read-failed", reason = failure(why)),
    }
}

/// The vendor and model names from the entity's localized strings.
pub fn product(model: &EntityModel) -> String {
    let Some(entity) = model.entity() else {
        return String::new();
    };
    let names: Vec<&str> = [entity.vendor_name, entity.model_name]
        .into_iter()
        .filter_map(|reference| model.localized(reference))
        .collect();
    // Some entities point both at the same string.
    match names.as_slice() {
        [vendor, model] if vendor == model => (*vendor).to_owned(),
        names => names.join(" "),
    }
}

/// Milan support: the specification version and certification, or "No".
pub fn milan(model: &EntityModel) -> String {
    let Some(milan) = model.milan else {
        // Unknown, rather than no, for an entity that could not be read.
        return match model.state {
            EnumerationState::Failed(_) => String::new(),
            _ => fl!("milan-no"),
        };
    };
    let dotted = |version: [u8; 4]| {
        let mut parts: Vec<String> = version.iter().map(u8::to_string).collect();
        while parts.len() > 2 && parts.last().is_some_and(|part| part == "0") {
            parts.pop();
        }
        parts.join(".")
    };
    let version = milan
        .specification_version
        .map_or_else(|| fl!("milan-before-1-3"), dotted);
    if milan.is_certified() {
        fl!(
            "milan-certified",
            version = version,
            certification = dotted(milan.certification_version)
        )
    } else {
        fl!("milan-not-certified", version = version)
    }
}

/// Each of an entity's configurations, labelled to choose among them: by
/// its name where each has a name of its own, else by what it holds, as
/// "16 in, 16 out, 8 ch streams" for one the Mac names "8 in, 8 out, 8 ch
/// streams" as it does all ten of its own.
pub fn configurations(model: &EntityModel) -> Vec<(u16, String)> {
    let summaries = model.configurations();
    let names: std::collections::HashSet<&str> =
        summaries.iter().map(|summary| summary.name).collect();
    let named = names.len() == summaries.len() && !names.contains("");
    let labels: Vec<String> = summaries
        .iter()
        .map(|summary| {
            if named {
                summary.name.to_owned()
            } else if summary.read {
                holds(summary)
            } else if !summary.name.is_empty() {
                summary.name.to_owned()
            } else {
                let number = u32::from(summary.index) + 1;
                fl!("entity-configuration-numbered", number = number)
            }
        })
        .collect();
    // Two that would read the same are told apart by their numbers.
    summaries
        .iter()
        .zip(&labels)
        .map(|(summary, label)| {
            let label = if labels.iter().filter(|other| *other == label).count() > 1 {
                let number = u32::from(summary.index) + 1;
                fl!(
                    "entity-configuration-of",
                    name = label.as_str(),
                    number = number,
                    count = summaries.len()
                )
            } else {
                label.clone()
            };
            (summary.index, label)
        })
        .collect()
}

/// What a configuration holds: its audio channels each way, and its audio
/// streams' channels where they all have as many.
fn holds(summary: &atdecc::model::ConfigurationSummary<'_>) -> String {
    let audio = |streams: &[atdecc::descriptor::StreamDescriptor<'_>]| -> Vec<u16> {
        streams
            .iter()
            .filter(|stream| !stream.is_media_clock())
            .filter_map(|stream| stream.current_format.channels())
            .collect()
    };
    holding(&audio(&summary.inputs), &audio(&summary.outputs))
}

/// What audio streams of `inputs` and `outputs` channels hold, in words.
fn holding(inputs: &[u16], outputs: &[u16]) -> String {
    let total = |channels: &[u16]| channels.iter().map(|&count| u32::from(count)).sum::<u32>();
    let mut each = inputs.iter().chain(outputs).copied();
    match each.next() {
        Some(first) if each.all(|channels| channels == first) => fl!(
            "entity-configuration-holds",
            inputs = total(inputs),
            outputs = total(outputs),
            channels = first
        ),
        _ => fl!(
            "entity-configuration-holds-mixed",
            inputs = total(inputs),
            outputs = total(outputs)
        ),
    }
}

/// What went wrong with an action, in words, or `None` when it worked.
pub fn outcome(triib: &Triib, action: Action, outcome: Outcome) -> Option<String> {
    let reason = match outcome {
        Outcome::Done => return None,
        Outcome::Refused(Refusal::Acmp(status)) => status
            .name()
            .map_or_else(|| fl!("outcome-status", status = status.0), flag_name),
        Outcome::Refused(Refusal::Aem(status)) => status
            .name()
            .map_or_else(|| fl!("outcome-status", status = status.0), flag_name),
        Outcome::NoResponse => fl!("outcome-no-response"),
        Outcome::NotPossible => fl!("outcome-not-possible"),
    };
    Some(match action {
        Action::Connect { talker, listener } => fl!(
            "outcome-connect",
            talker = triib.stream_name(talker.0, DescriptorType::STREAM_OUTPUT, talker.1),
            listener = triib.stream_name(listener.0, DescriptorType::STREAM_INPUT, listener.1),
            reason = reason
        ),
        Action::Disconnect { listener } => fl!(
            "outcome-disconnect",
            listener = triib.stream_name(listener.0, DescriptorType::STREAM_INPUT, listener.1),
            reason = reason
        ),
        Action::Identify(entity_id) => fl!(
            "outcome-identify",
            entity = triib.entity_name_of(entity_id),
            reason = reason
        ),
        Action::Rename { target, name } => match (target.descriptor_type, target.name_index) {
            (DescriptorType::ENTITY, 1) => fl!(
                "outcome-rename-group",
                entity = triib.entity_name_of(target.entity),
                name = name.as_str(),
                reason = reason
            ),
            (descriptor_type, _) => fl!(
                "outcome-rename",
                what = match descriptor_type {
                    DescriptorType::ENTITY => triib.entity_name_of(target.entity),
                    _ => triib.stream_name(target.entity, descriptor_type, target.index),
                },
                name = name.as_str(),
                reason = reason
            ),
        },
        Action::SetStreamFormat {
            entity,
            descriptor_type,
            index,
            ..
        } if outcome == Outcome::Refused(Refusal::Aem(AemStatus::STREAM_IS_RUNNING)) => fl!(
            "outcome-format-streaming",
            stream = triib.stream_name(entity, descriptor_type, index)
        ),
        Action::SetStreamFormat {
            entity,
            descriptor_type,
            index,
            ..
        } => fl!(
            "outcome-format",
            stream = triib.stream_name(entity, descriptor_type, index),
            reason = reason
        ),
        Action::SetConfiguration { entity, .. }
            if outcome == Outcome::Refused(Refusal::Aem(AemStatus::STREAM_IS_RUNNING)) =>
        {
            fl!(
                "outcome-configuration-streaming",
                entity = triib.entity_name_of(entity)
            )
        }
        Action::SetConfiguration { entity, .. } => fl!(
            "outcome-configuration",
            entity = triib.entity_name_of(entity),
            reason = reason
        ),
        Action::SetSamplingRate { entity, .. } => fl!(
            "outcome-sampling-rate",
            entity = triib.entity_name_of(entity),
            reason = reason
        ),
        Action::SetClockSource { entity, .. } => fl!(
            "outcome-clock-source",
            entity = triib.entity_name_of(entity),
            reason = reason
        ),
        Action::Map {
            entity,
            change: MappingChange::Add,
            ..
        } => fl!(
            "outcome-map",
            entity = triib.entity_name_of(entity),
            reason = reason
        ),
        Action::Map { entity, .. } => fl!(
            "outcome-unmap",
            entity = triib.entity_name_of(entity),
            reason = reason
        ),
        Action::SetMaxTransitTime { entity, output, .. }
            if outcome == Outcome::Refused(Refusal::Aem(AemStatus::STREAM_IS_RUNNING)) =>
        {
            fl!(
                "outcome-max-transit-time-streaming",
                stream = triib.stream_name(entity, DescriptorType::STREAM_OUTPUT, output)
            )
        }
        Action::SetMaxTransitTime { entity, output, .. } => fl!(
            "outcome-max-transit-time",
            stream = triib.stream_name(entity, DescriptorType::STREAM_OUTPUT, output),
            reason = reason
        ),
        Action::SetControl { entity, index, .. } => {
            let entity_name = triib.entity_name_of(entity);
            match triib
                .models
                .get(&entity)
                .and_then(|model| model.name_of(DescriptorType::CONTROL, index))
            {
                Some(control) => fl!(
                    "outcome-control",
                    control = control,
                    entity = entity_name,
                    reason = reason
                ),
                None => fl!(
                    "outcome-control-numbered",
                    index = index,
                    entity = entity_name,
                    reason = reason
                ),
            }
        }
    })
}

/// A stream's state in words: what an input is bound to and how that is
/// going, or where an output sends, while some listener takes it.
pub fn stream_state(
    triib: &Triib,
    entity: EntityId,
    model: &EntityModel,
    stream: &atdecc::descriptor::StreamDescriptor<'_>,
) -> Option<String> {
    let info = model.stream_info(stream.descriptor_type, stream.index);
    if stream.is_input() {
        let binding = model.binding(stream.index)?;
        let Some((talker, output)) = binding.talker_stream() else {
            return Some(fl!("stream-not-connected"));
        };
        let stream = triib.stream_name(talker, DescriptorType::STREAM_OUTPUT, output);
        Some(match info {
            Some(info) if info.talker_failed() => fl!(
                "stream-from-failed",
                stream = stream,
                reason = reservation_failure(info)
            ),
            Some(info) if info.settled() => fl!("stream-from-receiving", stream = stream),
            Some(_) => fl!("stream-from-waiting", stream = stream),
            None => fl!("stream-from", stream = stream),
        })
    } else {
        // A Milan talker has its stream's ID and address as soon as it has
        // an address: it sends only to listeners bound to it, as triib
        // knows them or the talker counts them, as the matrix and the
        // network view show.
        let counted = model
            .tx_state(stream.index)
            .map_or(0, |state| usize::from(state.connection_count));
        if triib.listeners_of((entity, stream.index)).max(counted) == 0 {
            return Some(fl!("stream-not-connected"));
        }
        let info = info?;
        info.settled().then(|| {
            fl!(
                "stream-sending-to",
                destination = info.stream_dest_mac.to_string()
            )
        })
    }
}

/// Why reading an entity model failed, in words.
pub fn failure(failure: EnumerationFailure) -> String {
    match failure {
        EnumerationFailure::NoResponse => fl!("failure-no-response"),
        EnumerationFailure::Refused(status) => fl!(
            "failure-refused",
            status = status
                .name()
                .map_or_else(|| status.0.to_string(), flag_name)
        ),
        EnumerationFailure::Malformed => fl!("failure-malformed"),
        EnumerationFailure::OnThisComputer => fl!("failure-on-this-computer"),
    }
}

/// A capability's name from the standard in words: `AEM_SUPPORTED`
/// becomes "AEM supported".
pub fn flag_name(name: &str) -> String {
    name_words(name, true)
}

/// A standard's name in words, capitalized when it starts a sentence:
/// "READ_DESCRIPTOR" as "Read descriptor", "STREAM_INPUT" in a sentence
/// as "stream input", keeping acronyms such as AVB and gPTP.
pub fn name_words(name: &str, capital: bool) -> String {
    const ACRONYMS: [&str; 19] = [
        "AEM", "ACMP", "ATDECC", "AVB", "AVC", "EFU", "ID", "MIDI", "PTP", "SMPTE", "SRP", "MSRP",
        "MVU", "CRF", "RX", "TX", "AS", "A", "B",
    ];
    let words: Vec<String> = name
        .split('_')
        .map(|word| match word {
            "GPTP" => "gPTP".to_owned(),
            "UDPV4" => "UDPv4".to_owned(),
            "UDPV6" => "UDPv6".to_owned(),
            word if ACRONYMS.contains(&word) => word.to_owned(),
            word => word.to_lowercase(),
        })
        .collect();
    let text = words.join(" ");
    // Only a lowercased first word is capitalized; acronyms keep their
    // own case, as in "gPTP".
    match words.first() {
        Some(first) if capital && first.chars().all(|character| character.is_lowercase()) => {
            capitalized(&text)
        }
        _ => text,
    }
}

fn capitalized(text: &str) -> String {
    crate::i18n::capitalized(text)
}

/// What an MSRP failure code means (IEEE 802.1Q, 35.2.2.8.7).
pub fn msrp_failure(code: u8) -> String {
    match code {
        1 => fl!("msrp-failure-1"),
        2 => fl!("msrp-failure-2"),
        3 => fl!("msrp-failure-3"),
        4 => fl!("msrp-failure-4"),
        5 => fl!("msrp-failure-5"),
        6 => fl!("msrp-failure-6"),
        7 => fl!("msrp-failure-7"),
        8 => fl!("msrp-failure-8"),
        9 => fl!("msrp-failure-9"),
        10 => fl!("msrp-failure-10"),
        11 => fl!("msrp-failure-11"),
        12 => fl!("msrp-failure-12"),
        13 => fl!("msrp-failure-13"),
        14 => fl!("msrp-failure-14"),
        15 => fl!("msrp-failure-15"),
        16 => fl!("msrp-failure-16"),
        17 => fl!("msrp-failure-17"),
        18 => fl!("msrp-failure-18"),
        19 => fl!("msrp-failure-19"),
        _ => fl!("msrp-failure-unknown"),
    }
}

/// Where a talker's reservation failed, in words: the reason, and the
/// bridge reporting it by the MAC address in its bridge ID.
pub fn reservation_failure(info: &atdecc::aem::StreamInfo) -> String {
    let reason = msrp_failure(info.msrp_failure_code);
    let [_, _, mac @ ..] = info.msrp_failure_bridge_id.to_be_bytes();
    if mac == [0; 6] {
        reason
    } else {
        fl!(
            "msrp-failure-at",
            reason = reason,
            bridge = avb_net::MacAddress(mac).to_string()
        )
    }
}

#[cfg(test)]
mod tests {

    /// A configuration reads as the audio it holds each way, with its
    /// streams' channels where they agree, as the Mac's own should be
    /// named.
    #[test]
    fn configurations_read_as_what_they_hold() {
        assert_eq!(holding(&[8], &[8]), "8 in, 8 out, 8 ch streams");
        assert_eq!(
            holding(&[16, 16], &[16, 16]),
            "32 in, 32 out, 16 ch streams"
        );
        assert_eq!(
            holding(&[8, 8, 8], &[8, 8, 8]),
            "24 in, 24 out, 8 ch streams"
        );
        assert_eq!(holding(&[16], &[8]), "16 in, 8 out");
        assert_eq!(holding(&[], &[2]), "0 in, 2 out, 2 ch streams");
    }

    use super::*;

    /// An output says it sends only while a listener takes it, as the
    /// matrix and the network view show; else it is not connected.
    #[test]
    fn outputs_send_only_to_listeners() {
        use crate::settings::Settings;
        let (entities, models) = crate::view::tests::bench();
        let interface = avb_net::Interface {
            name: "enp6s0".to_owned(),
            mac: avb_net::MacAddress([0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b]),
            up: true,
            speed: None,
            physical: true,
            wireless: false,
            hardware_clock: None,
        };
        let triib = crate::app::Triib::sample(Settings::default(), interface, entities, models);
        let mut outputs = 0;
        for (entity, model) in &triib.models {
            for output in model.streams(false) {
                // The bench's outputs have their stream IDs and addresses,
                // but none bound to them is not connected all the same.
                if triib.listeners_of((*entity, output.index)) > 0
                    || model
                        .tx_state(output.index)
                        .is_some_and(|state| state.connection_count > 0)
                {
                    continue;
                }
                assert_eq!(
                    stream_state(&triib, *entity, model, &output).as_deref(),
                    Some("Not connected"),
                    "{entity} output {}",
                    output.index
                );
                outputs += 1;
            }
        }
        assert!(outputs > 0);
    }

    /// A max transit time a running stream refuses says what to do; any
    /// other refusal says why.
    #[test]
    fn a_refused_transit_time_says_why() {
        use crate::settings::Settings;
        let (entities, models) = crate::view::tests::bench();
        let interface = avb_net::Interface {
            name: "enp6s0".to_owned(),
            mac: avb_net::MacAddress([0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b]),
            up: true,
            speed: None,
            physical: true,
            wireless: false,
            hardware_clock: None,
        };
        let triib = crate::app::Triib::sample(Settings::default(), interface, entities, models);
        let (&entity, _) = triib
            .models
            .iter()
            .find(|(_, model)| model.streams(false).next().is_some())
            .expect("an entity with an output");
        let action = Action::SetMaxTransitTime {
            entity,
            output: 0,
            nanoseconds: 5_000_000,
        };
        let running = outcome(
            &triib,
            action,
            Outcome::Refused(Refusal::Aem(AemStatus::STREAM_IS_RUNNING)),
        )
        .unwrap();
        assert!(
            running.ends_with("it is streaming. Disconnect it first."),
            "{running}"
        );
        let other = outcome(
            &triib,
            action,
            Outcome::Refused(Refusal::Aem(AemStatus::NOT_IMPLEMENTED)),
        )
        .unwrap();
        assert!(
            other.starts_with("Could not change the max transit time"),
            "{other}"
        );
        assert!(!other.contains("Disconnect"), "{other}");
    }

    #[test]
    fn flag_names_read_as_words() {
        assert_eq!(flag_name("AEM_SUPPORTED"), "AEM supported");
        assert_eq!(flag_name("CLASS_A_SUPPORTED"), "Class A supported");
        assert_eq!(flag_name("SUPPORTS_UDPV4_ATDECC"), "Supports UDPv4 ATDECC");
        assert_eq!(flag_name("GPTP_SUPPORTED"), "gPTP supported");
        assert_eq!(flag_name("ENTITY_NOT_READY"), "Entity not ready");
        assert_eq!(flag_name("AVB_INTERFACE"), "AVB interface");
        assert_eq!(
            name_words("GET_RX_STATE_RESPONSE", true),
            "Get RX state response"
        );
        assert_eq!(name_words("STREAM_INPUT", false), "stream input");
        assert_eq!(milliseconds(12_000_000), "12 ms");
        assert_eq!(milliseconds(2_500_000), "2.5 ms");
    }

    #[test]
    fn roles_and_classes() {
        let adp = Adpdu {
            talker_stream_sources: 2,
            talker_capabilities: TalkerCapabilities::IMPLEMENTED,
            listener_stream_sinks: 3,
            listener_capabilities: ListenerCapabilities::IMPLEMENTED,
            entity_capabilities: EntityCapabilities::CLASS_B_SUPPORTED,
            ..Adpdu::default()
        };
        assert_eq!(roles(&adp), "Talker 2, listener 3");
        assert_eq!(classes(&adp), "B");
        assert_eq!(glyph(&adp), Icon::GraphicEq);
        assert_eq!(roles(&Adpdu::default()), "No roles");
        assert_eq!(clock(&adp), "No gPTP");
    }
}
