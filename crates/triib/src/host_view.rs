//! This computer's own talkers and listeners in the views: the Entities
//! view's bar that adds them, and their section in the inspector, with
//! their audio, channels and a button that removes them.

use std::fmt;

use atdecc::EntityId;
use iced::widget::{container, space};
use iced::{Center, Element, Fill};
use scramble_ui::button::{self, Kind, Size};
use scramble_ui::font::{Type, styled};
use scramble_ui::icon::Icon;
use scramble_ui::{component, dir, dropdown, style};
use triib_endpointd::config::{Direction, Kind as EndpointKind, VIRTUAL};

use crate::app::{Message, Triib};
use crate::fl;
use crate::host::StreamChange;
use crate::view::{property, stacked};

/// The bar above the Entities view's table: why this computer cannot add
/// talkers and listeners, or the buttons that add them.
pub fn bar(triib: &Triib) -> Option<Element<'_, Message>> {
    if !crate::host::SUPPORTED || triib.interface().is_none() {
        return None;
    }
    let able = triib.hosts_endpoints();
    let foreign = triib
        .endpoints
        .as_ref()
        .filter(|status| status.foreign_mrp)
        .map(|status| status.interface.clone());
    let note = if !able {
        Some(fl!("host-needs-clock"))
    } else if let Some(interface) = &triib.endpoints_elsewhere {
        Some(fl!("host-elsewhere", interface = interface.clone()))
    } else if let Some(interface) = foreign {
        Some(fl!("host-foreign-mrp", interface = interface))
    } else if triib
        .endpoints
        .as_ref()
        .is_some_and(|status| status.gptp.starts_with("ptp4l"))
    {
        Some(fl!("host-no-ptp4l"))
    } else {
        None
    };
    let add = |kind, label: String| {
        button::with_icon(Kind::Tonal, icon(kind), label)
            .size(Size::ExtraSmall)
            .on_press_maybe(able.then_some(Message::HostAdded(kind)))
    };
    let text: Element<'_, Message> = match note {
        Some(note) => styled(note, Type::BodySmall)
            .style(style::on_surface_variant)
            .width(Fill)
            .align_x(dir::text_start())
            .into(),
        None => space().width(Fill).into(),
    };
    Some(
        container(
            scramble_ui::row![
                text,
                component::toggle_tool(
                    Icon::Computer,
                    if triib.settings.host_entities_only {
                        fl!("host-show-all")
                    } else {
                        fl!("host-show-mine")
                    },
                    triib.settings.host_entities_only,
                    Message::HostEntitiesOnlyToggled,
                ),
                add(EndpointKind::Talker, fl!("host-add-talker")),
                add(EndpointKind::Listener, fl!("host-add-listener")),
                add(EndpointKind::Device, fl!("host-add-device")),
            ]
            .spacing(8)
            .align_y(Center),
        )
        .padding([8, 16])
        .into(),
    )
}

/// The icon of this computer's talkers, listeners and devices: output's,
/// input's, and the computer's.
pub fn icon(kind: EndpointKind) -> Icon {
    match kind {
        EndpointKind::Talker => Icon::Output,
        EndpointKind::Listener => Icon::Input,
        EndpointKind::Device => Icon::Computer,
    }
}

/// The channel counts the inspector offers; six samples of each fit in a
/// frame up to 60.
const CHANNEL_COUNTS: [u16; 11] = [1, 2, 4, 6, 8, 12, 16, 24, 32, 48, 56];

/// One choice for an endpoint's audio: what endpoints.toml says, and the
/// words for it.
#[derive(Debug, Clone)]
struct Audio {
    spec: String,
    label: String,
}

impl PartialEq for Audio {
    fn eq(&self, other: &Self) -> bool {
        self.spec == other.spec
    }
}

impl fmt::Display for Audio {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.label)
    }
}

/// The inspector's section for one of this computer's endpoints: its
/// state, and a button that removes it. Its streams are on the Streams
/// tab.
pub fn sections(triib: &Triib, entity_id: EntityId) -> Vec<Element<'_, Message>> {
    let Some(endpoint) = crate::host::status_of(triib.endpoints.as_ref(), entity_id) else {
        return Vec::new();
    };
    vec![
        component::section(fl!("netmap-this-computer")),
        property(fl!("host-state"), state_label(&endpoint.state)),
        button::with_icon(Kind::Outlined, Icon::Delete, fl!("host-remove"))
            .size(Size::ExtraSmall)
            .on_press(Message::HostRemoved(entity_id))
            .into(),
    ]
}

/// Whether the entity is one of this computer's devices, whose streams
/// are added and removed.
pub fn is_device(triib: &Triib, entity_id: EntityId) -> bool {
    crate::host::status_of(triib.endpoints.as_ref(), entity_id)
        .is_some_and(|endpoint| endpoint.kind == EndpointKind::Device)
}

/// A device's buttons that add a stream output or input, each on its own
/// line, as two side by side are wider than the inspector.
pub fn add_buttons(triib: &Triib, entity_id: EntityId) -> Vec<Element<'_, Message>> {
    if !is_device(triib, entity_id) {
        return Vec::new();
    }
    let add = |direction, label: String| {
        button::with_icon(Kind::Tonal, Icon::Add, label)
            .size(Size::ExtraSmall)
            .on_press(Message::HostStream(entity_id, StreamChange::Add(direction)))
            .into()
    };
    vec![
        add(Direction::Out, fl!("host-add-output-stream")),
        add(Direction::In, fl!("host-add-input-stream")),
    ]
}

/// What this computer sets for one of its endpoint's streams, the stream
/// input or output `index`: where its audio comes from or goes, and its
/// channels. Nothing for others' entities.
pub fn stream_rows(
    triib: &Triib,
    entity_id: EntityId,
    input: bool,
    index: u16,
) -> Vec<Element<'_, Message>> {
    let Some(endpoint) = crate::host::status_of(triib.endpoints.as_ref(), entity_id) else {
        return Vec::new();
    };
    let audio_label = if input {
        fl!("host-audio-to")
    } else {
        fl!("host-audio-from")
    };
    if endpoint.kind != EndpointKind::Device {
        return vec![
            stacked(
                audio_label,
                audio_picker(triib, input, false, &endpoint.audio, move |spec| {
                    Message::HostAudio(entity_id, spec)
                }),
            ),
            stacked(
                fl!("host-channels"),
                channel_picker(endpoint.channels, move |count| {
                    Message::HostChannels(entity_id, count)
                }),
            ),
        ];
    }
    // The device's streams one way, in the order the entity numbers them.
    let direction = if input { Direction::In } else { Direction::Out };
    let Some((at, stream)) = endpoint
        .streams
        .iter()
        .enumerate()
        .filter(|(_, stream)| stream.direction == direction)
        .nth(usize::from(index))
    else {
        return Vec::new();
    };
    vec![
        stacked(
            audio_label,
            audio_picker(triib, input, true, &stream.audio, move |spec| {
                Message::HostStream(entity_id, StreamChange::Audio(at, spec))
            }),
        ),
        stacked(
            fl!("host-channels"),
            channel_picker(stream.channels, move |count| {
                Message::HostStream(entity_id, StreamChange::Channels(at, count))
            }),
        ),
    ]
}

/// The button that removes a device's stream input or output `index`,
/// but not its last stream; none for others' entities.
pub fn remove_stream(
    triib: &Triib,
    entity_id: EntityId,
    input: bool,
    index: u16,
) -> Option<Element<'_, Message>> {
    let endpoint = crate::host::status_of(triib.endpoints.as_ref(), entity_id)
        .filter(|endpoint| endpoint.kind == EndpointKind::Device)?;
    let direction = if input { Direction::In } else { Direction::Out };
    let at = endpoint
        .streams
        .iter()
        .enumerate()
        .filter(|(_, stream)| stream.direction == direction)
        .nth(usize::from(index))?
        .0;
    let last = endpoint.streams.len() <= 1;
    Some(
        button::with_icon(Kind::Outlined, Icon::Delete, fl!("host-remove-stream"))
            .size(Size::ExtraSmall)
            .on_press_maybe(
                (!last).then_some(Message::HostStream(entity_id, StreamChange::Remove(at))),
            )
            .into(),
    )
}

/// What an endpoint's state says.
fn state_label(state: &str) -> String {
    match state {
        "streaming" => fl!("host-streaming"),
        "waiting" => fl!("host-waiting"),
        "listening" => fl!("host-listening"),
        "bound" => fl!("host-bound"),
        _ => fl!("host-unbound"),
    }
}

/// A picker of where inputs play, or outputs' audio comes from, with
/// `current` picked, offering a virtual audio device where `virtual_device`.
fn audio_picker<'a>(
    triib: &'a Triib,
    inputs: bool,
    virtual_device: bool,
    current: &str,
    pick: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    let audio = |spec: &str, label: String| Audio {
        spec: spec.to_owned(),
        label,
    };
    let mut choices = Vec::new();
    if virtual_device {
        choices.push(audio(VIRTUAL, fl!("host-virtual-device")));
    }
    let devices = if inputs {
        choices.push(audio("discard", fl!("host-nowhere")));
        &triib.audio_devices.1
    } else {
        choices.push(audio("silence", fl!("host-silence")));
        choices.push(audio("tone", fl!("host-tone")));
        &triib.audio_devices.0
    };
    choices.push(audio("default", fl!("host-default-device")));
    choices.extend(
        devices
            .iter()
            .filter(|name| name.as_str() != "default")
            .map(|name| audio(name, name.clone())),
    );
    let current = choices
        .iter()
        .find(|choice| choice.spec == current)
        .cloned()
        .unwrap_or_else(|| audio(current, current.to_owned()));
    let _reading = dir::reading();
    dropdown::pick(choices, Some(current), move |choice: Audio| {
        pick(choice.spec)
    })
    .size(button::Size::ExtraSmall)
    .width(Fill)
    .into()
}

/// A picker of channel counts, with `current` picked.
fn channel_picker<'a>(current: u16, pick: impl Fn(u16) -> Message + 'a) -> Element<'a, Message> {
    let mut counts = CHANNEL_COUNTS.to_vec();
    if !counts.contains(&current) {
        counts.push(current);
        counts.sort_unstable();
    }
    let _reading = dir::reading();
    dropdown::pick(counts, Some(current), pick)
        .size(button::Size::ExtraSmall)
        .width(Fill)
        .into()
}
