//! The inspector's channel mappings: how each stream port's channels map
//! to and from the streams. A port whose mappings a controller sets gets
//! a picker for each channel; fixed mappings show as text.

use std::fmt;

use atdecc::EntityId;
use atdecc::aem::{AudioMapping, MappingChange};
use atdecc::descriptor::{DescriptorType, StreamPortDescriptor};
use atdecc::model::EntityModel;
use iced::widget::container;
use iced::{Center, Element, Fill, Length};
use scramble_ui::font::{Type, styled};
use scramble_ui::style;
use scramble_ui::{button, component, dropdown};

use crate::app::{Message, Triib};
use crate::fl;
use crate::network::Action;
use crate::view::stacked;

/// The width of a channel's number beside its picker, room for three
/// digits.
const CHANNEL_WIDTH: f32 = 28.0;

/// A stream or a cluster: its index or offset, as mappings count it, its
/// name and how many channels it has.
struct Channels {
    at: u16,
    name: String,
    count: u16,
}

impl Channels {
    /// A channel by the name alone when there is one, else with the
    /// channel counted from 1.
    fn label(&self, channel: u16) -> String {
        if self.count <= 1 {
            self.name.clone()
        } else {
            format!("{} · {}", self.name, channel + 1)
        }
    }

    /// The channels from `first` to `last`.
    fn range(&self, first: u16, last: u16) -> String {
        if self.count <= 1 || first == last {
            self.label(first)
        } else {
            format!("{} · {}–{}", self.name, first + 1, last + 1)
        }
    }

    fn choices(&self) -> impl Iterator<Item = Choice> + '_ {
        (0..self.count).map(|channel| Choice {
            at: Some((self.at, channel)),
            label: self.label(channel),
        })
    }
}

/// What a picker offers: a channel of a stream or cluster, or none.
#[derive(Debug, Clone, PartialEq)]
struct Choice {
    at: Option<(u16, u16)>,
    label: String,
}

impl fmt::Display for Choice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.label)
    }
}

/// The audio streams on one side, with their channel counts.
fn streams(model: &EntityModel, input: bool) -> Vec<Channels> {
    model
        .streams(input)
        .filter(|stream| !stream.is_media_clock())
        .filter_map(|stream| {
            Some(Channels {
                at: stream.index,
                name: model
                    .name_of(stream.descriptor_type, stream.index)
                    .map_or_else(|| source_name(true, stream.index), str::to_owned),
                count: stream.current_format.channels()?,
            })
        })
        .collect()
}

/// A stream port's clusters, by their offset from its first.
fn clusters(model: &EntityModel, port: &StreamPortDescriptor) -> Vec<Channels> {
    model
        .audio_clusters(port)
        .map(|(offset, cluster)| Channels {
            at: offset,
            name: model
                .name_of(
                    DescriptorType::AUDIO_CLUSTER,
                    port.base_cluster.saturating_add(offset),
                )
                .map_or_else(|| source_name(false, offset), str::to_owned),
            count: cluster.channel_count,
        })
        .collect()
}

/// A mapping's two ends as a port's rows see them: on an input the
/// cluster channel, which takes one stream channel, then the stream
/// channel; on an output the stream channel, which takes one cluster
/// channel, then the cluster channel.
fn ends(input: bool, mapping: &AudioMapping) -> ((u16, u16), (u16, u16)) {
    let stream = (mapping.stream_index, mapping.stream_channel);
    let cluster = (mapping.cluster_offset, mapping.cluster_channel);
    if input {
        (cluster, stream)
    } else {
        (stream, cluster)
    }
}

/// The mapping between a row's channel and another's.
fn joining(input: bool, row: (u16, u16), other: (u16, u16)) -> AudioMapping {
    let (stream, cluster) = if input { (other, row) } else { (row, other) };
    AudioMapping {
        stream_index: stream.0,
        stream_channel: stream.1,
        cluster_offset: cluster.0,
        cluster_channel: cluster.1,
    }
}

/// A channel's number, counted from 1, beside its picker.
fn numbered<'a>(channel: u16, picker: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    scramble_ui::row![
        styled((channel + 1).to_string(), Type::BodyMedium)
            .style(style::on_surface_variant)
            .width(Length::Fixed(CHANNEL_WIDTH))
            .align_x(scramble_ui::dir::text_start()),
        container(picker).width(Fill),
    ]
    .spacing(8)
    .align_y(Center)
    .into()
}

fn note<'a>(text: String) -> Element<'a, Message> {
    styled(text, Type::BodySmall)
        .style(style::on_surface_variant)
        .into()
}

/// The channel mappings of each of the entity's stream ports, in a
/// section of their own.
pub fn mapping_details<'a>(
    triib: &'a Triib,
    entity: EntityId,
    model: &'a EntityModel,
) -> Vec<Element<'a, Message>> {
    let ports: Vec<StreamPortDescriptor> = model
        .stream_ports(true)
        .chain(model.stream_ports(false))
        .collect();
    if ports.is_empty() {
        return Vec::new();
    }
    let several = |input: bool| ports.iter().filter(|port| port.is_input() == input).count() > 1;
    let mut items = vec![component::section(fl!("mapping-section"))];
    for port in &ports {
        let mut title = vec![if port.is_input() {
            fl!("mapping-inputs")
        } else {
            fl!("mapping-outputs")
        }];
        if several(port.is_input()) {
            title.push(fl!("mapping-port", number = (port.index + 1)));
        }
        if !port.has_dynamic_mappings() {
            title.push(fl!("mapping-fixed"));
        }
        let title = crate::i18n::list(title);
        items.push(
            styled(title, Type::LabelLarge)
                .style(style::on_surface_variant)
                .into(),
        );
        if port.has_dynamic_mappings() {
            items.extend(dynamic_mappings(triib, entity, model, port));
        } else {
            items.extend(fixed_mappings(model, port));
        }
    }
    items
}

/// A picker for each channel of a port whose mappings a controller sets:
/// on an input, the stream channel each cluster channel takes; on an
/// output, the cluster channel each stream channel sends.
fn dynamic_mappings<'a>(
    triib: &'a Triib,
    entity: EntityId,
    model: &EntityModel,
    port: &StreamPortDescriptor,
) -> Vec<Element<'a, Message>> {
    let Some(mappings) = model.dynamic_mappings(port.descriptor_type, port.index) else {
        return vec![note(fl!("mapping-not-read"))];
    };
    let mappings: Vec<AudioMapping> = mappings.collect();
    let input = port.is_input();
    let (rows, sources) = if input {
        (clusters(model, port), streams(model, true))
    } else {
        (streams(model, false), clusters(model, port))
    };
    if rows.is_empty() {
        return vec![note(if input {
            fl!("mapping-no-clusters")
        } else {
            fl!("mapping-no-streams")
        })];
    }
    let mut choices = vec![Choice {
        at: None,
        label: fl!("mapping-not-mapped"),
    }];
    choices.extend(sources.iter().flat_map(Channels::choices));
    let port_id = (port.descriptor_type, port.index);
    let mut items = Vec::new();
    for row in &rows {
        // A cluster or stream of several channels heads a row for each, so
        // the pickers have room to name the channel they pick.
        if row.count > 1 {
            items.push(styled(row.name.clone(), Type::BodyMedium).into());
        }
        for channel in 0..row.count {
            let here = (row.at, channel);
            let current = mappings
                .iter()
                .find(|mapping| ends(input, mapping).0 == here)
                .copied();
            // A change on its way shows, and holds the picker still.
            let pending = triib.changing(|action| {
                matches!(action, Action::Map { entity: target, port: place, mapping, .. }
                    if *target == entity && *place == port_id && ends(input, mapping).0 == here)
            });
            let shown_at = match pending {
                Some(Action::Map {
                    change: MappingChange::Add,
                    mapping,
                    ..
                }) => Some(ends(input, mapping).1),
                Some(_) => None,
                None => current.map(|mapping| ends(input, &mapping).1),
            };
            let shown = choices
                .iter()
                .find(|choice| choice.at == shown_at)
                .cloned()
                .unwrap_or_else(|| Choice {
                    at: shown_at,
                    label: shown_at.map_or_else(
                        || fl!("mapping-not-mapped"),
                        |(at, channel)| format!("{} · {}", source_name(input, at), channel + 1),
                    ),
                });
            let busy = pending.is_some();
            let row_channel = Row {
                entity,
                port: port_id,
                input,
                here,
                current,
            };
            let picker = dropdown::pick(choices.clone(), Some(shown), move |choice: Choice| {
                if busy {
                    Message::Nothing
                } else {
                    row_channel.pick(choice.at)
                }
            })
            .size(button::Size::ExtraSmall)
            .width(Fill);
            items.push(if row.count > 1 {
                numbered(channel, picker)
            } else {
                stacked(row.name.clone(), picker)
            });
        }
    }
    items
}

/// One channel a port's row is for, and the mapping it has.
#[derive(Debug, Clone, Copy)]
struct Row {
    entity: EntityId,
    port: (DescriptorType, u16),
    input: bool,
    here: (u16, u16),
    current: Option<AudioMapping>,
}

impl Row {
    /// What picking `other` sends: the channel's mapping removed, then the
    /// new one added, as an entity may refuse a second mapping to the same
    /// channel; nothing when it is the one there.
    fn pick(self, other: Option<(u16, u16)>) -> Message {
        if other == self.current.map(|mapping| ends(self.input, &mapping).1) {
            return Message::Nothing;
        }
        let change = |change, mapping| Action::Map {
            entity: self.entity,
            port: self.port,
            change,
            mapping,
        };
        let mut actions = Vec::new();
        if let Some(old) = self.current {
            actions.push(change(MappingChange::Remove, old));
        }
        if let Some(other) = other {
            actions.push(change(
                MappingChange::Add,
                joining(self.input, self.here, other),
            ));
        }
        Message::ActInOrder(actions)
    }
}

/// A stream, or a port's cluster, without a name, counted from 1.
fn source_name(stream: bool, at: u16) -> String {
    if stream {
        fl!("stream-numbered", index = (at + 1))
    } else {
        fl!("mapping-cluster-numbered", index = (at + 1))
    }
}

/// A port's fixed mappings, a run of consecutive channels to a line, in
/// the direction the audio goes.
fn fixed_mappings<'a>(
    model: &EntityModel,
    port: &StreamPortDescriptor,
) -> Vec<Element<'a, Message>> {
    let input = port.is_input();
    let streams = streams(model, input);
    let clusters = clusters(model, port);
    let find = |known: &[Channels], at: u16, stream: bool| {
        known
            .iter()
            .find(|item| item.at == at)
            .map_or_else(|| source_name(stream, at), |item| item.name.clone())
    };
    let counts = |known: &[Channels], at: u16| {
        known
            .iter()
            .find(|item| item.at == at)
            .map_or(2, |item| item.count)
    };
    let mut mappings: Vec<AudioMapping> = model.static_mappings(port).collect();
    if mappings.is_empty() {
        return vec![note(fl!("mapping-none"))];
    }
    mappings.sort_by_key(|mapping| ends(input, mapping));
    // Runs of mappings whose channels both go up by one.
    let mut runs: Vec<(AudioMapping, AudioMapping)> = Vec::new();
    for mapping in mappings {
        if let Some((_, last)) = runs.last_mut()
            && last.stream_index == mapping.stream_index
            && last.cluster_offset == mapping.cluster_offset
            && last.stream_channel.checked_add(1) == Some(mapping.stream_channel)
            && last.cluster_channel.checked_add(1) == Some(mapping.cluster_channel)
        {
            *last = mapping;
        } else {
            runs.push((mapping, mapping));
        }
    }
    runs.into_iter()
        .map(|(first, last)| {
            let stream = Channels {
                at: first.stream_index,
                name: find(&streams, first.stream_index, true),
                count: counts(&streams, first.stream_index),
            }
            .range(first.stream_channel, last.stream_channel);
            let cluster = Channels {
                at: first.cluster_offset,
                name: find(&clusters, first.cluster_offset, false),
                count: counts(&clusters, first.cluster_offset),
            }
            .range(first.cluster_channel, last.cluster_channel);
            let (from, to) = if input {
                (stream, cluster)
            } else {
                (cluster, stream)
            };
            let line = format!("{from} {} {to}", crate::i18n::arrow(&from));
            styled(line, Type::BodyMedium).into()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTITY: EntityId = EntityId(0xd111_e597_f544_8000);
    const INPUT: (DescriptorType, u16) = (DescriptorType::STREAM_PORT_INPUT, 0);

    fn mapping(stream_channel: u16, cluster_channel: u16) -> AudioMapping {
        AudioMapping {
            stream_index: 0,
            stream_channel,
            cluster_offset: 0,
            cluster_channel,
        }
    }

    fn actions(message: Message) -> Vec<(MappingChange, AudioMapping)> {
        match message {
            Message::ActInOrder(actions) => actions
                .into_iter()
                .map(|action| match action {
                    Action::Map {
                        entity: ENTITY,
                        port: INPUT,
                        change,
                        mapping,
                    } => (change, mapping),
                    other => panic!("{other:?}"),
                })
                .collect(),
            Message::Nothing => Vec::new(),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn picking_a_channel_replaces_its_mapping() {
        // Cluster channel 2 of an input, taking stream channel 5.
        let row = Row {
            entity: ENTITY,
            port: INPUT,
            input: true,
            here: (0, 2),
            current: Some(mapping(5, 2)),
        };
        assert_eq!(
            actions(row.pick(Some((0, 1)))),
            [
                (MappingChange::Remove, mapping(5, 2)),
                (MappingChange::Add, mapping(1, 2)),
            ]
        );
        assert_eq!(
            actions(row.pick(None)),
            [(MappingChange::Remove, mapping(5, 2))]
        );
        assert!(actions(row.pick(Some((0, 5)))).is_empty());
        let unmapped = Row {
            current: None,
            ..row
        };
        assert_eq!(
            actions(unmapped.pick(Some((0, 0)))),
            [(MappingChange::Add, mapping(0, 2))]
        );
        assert!(actions(unmapped.pick(None)).is_empty());
    }

    #[test]
    fn output_rows_are_stream_channels() {
        // Stream channel 3 of an output, sending cluster channel 6.
        let row = Row {
            entity: ENTITY,
            port: INPUT,
            input: false,
            here: (0, 3),
            current: None,
        };
        assert_eq!(
            actions(row.pick(Some((0, 6)))),
            [(MappingChange::Add, mapping(3, 6))]
        );
    }

    #[test]
    fn runs_of_channels_read_as_ranges() {
        let stream = Channels {
            at: 0,
            name: "Audio Input Stream 1".to_owned(),
            count: 8,
        };
        assert_eq!(stream.range(0, 7), "Audio Input Stream 1 · 1–8");
        assert_eq!(stream.range(3, 3), "Audio Input Stream 1 · 4");
        let mono = Channels {
            at: 0,
            name: "Mono Audio In".to_owned(),
            count: 1,
        };
        assert_eq!(mono.range(0, 0), "Mono Audio In");
    }
}
