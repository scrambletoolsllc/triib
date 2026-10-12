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
use triib_endpointd::config::Kind as EndpointKind;

use crate::app::{Message, Triib};
use crate::fl;
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
        button::with_icon(Kind::Tonal, Icon::Add, label)
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

/// The channel counts the inspector offers; six samples of each fit in a
/// frame up to 60.
const CHANNEL_COUNTS: [u16; 11] = [1, 2, 4, 6, 8, 12, 16, 24, 32, 48, 56];

/// The stream counts each way the inspector offers a device.
const STREAM_COUNTS: [u16; 8] = [1, 2, 3, 4, 6, 8, 12, 16];

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

/// The inspector's section for one of this computer's endpoints.
pub fn sections(triib: &Triib, entity_id: EntityId) -> Vec<Element<'_, Message>> {
    let Some(endpoint) = crate::host::status_of(triib.endpoints.as_ref(), entity_id) else {
        return Vec::new();
    };
    let state = match endpoint.state.as_str() {
        "streaming" => fl!("host-streaming"),
        "waiting" => fl!("host-waiting"),
        "listening" => fl!("host-listening"),
        "bound" => fl!("host-bound"),
        _ => fl!("host-unbound"),
    };
    let device = endpoint.kind == EndpointKind::Device;
    let mut rows = vec![
        component::section(fl!("netmap-this-computer")),
        property(fl!("host-state"), state),
    ];
    if endpoint.kind != EndpointKind::Listener {
        rows.push(stacked(
            fl!("host-audio-from"),
            audio_picker(triib, entity_id, false, &endpoint.audio),
        ));
    }
    match endpoint.kind {
        EndpointKind::Listener => rows.push(stacked(
            fl!("host-audio-to"),
            audio_picker(triib, entity_id, true, &endpoint.audio),
        )),
        EndpointKind::Device => {
            rows.push(stacked(
                fl!("host-audio-to"),
                audio_picker(triib, entity_id, true, &endpoint.sink),
            ));
            rows.push(stacked(
                fl!("stream-outputs"),
                stream_picker(entity_id, false, endpoint.outputs),
            ));
            rows.push(stacked(
                fl!("stream-inputs"),
                stream_picker(entity_id, true, endpoint.inputs),
            ));
        }
        EndpointKind::Talker => {}
    }
    let channels: Element<'_, Message> = {
        let mut counts = CHANNEL_COUNTS.to_vec();
        if !counts.contains(&endpoint.channels) {
            counts.push(endpoint.channels);
            counts.sort_unstable();
        }
        let _reading = dir::reading();
        dropdown::pick(counts, Some(endpoint.channels), move |count| {
            Message::HostChannels(entity_id, count)
        })
        .size(button::Size::ExtraSmall)
        .width(Fill)
        .into()
    };
    rows.push(stacked(
        if device {
            fl!("host-channels-each")
        } else {
            fl!("host-channels")
        },
        channels,
    ));
    rows.push(
        button::with_icon(Kind::Outlined, Icon::Delete, fl!("host-remove"))
            .size(Size::ExtraSmall)
            .on_press(Message::HostRemoved(entity_id))
            .into(),
    );
    rows
}

/// A picker of where an endpoint's inputs play, or its outputs' audio
/// comes from, with `current` picked.
fn audio_picker<'a>(
    triib: &'a Triib,
    entity_id: EntityId,
    inputs: bool,
    current: &str,
) -> Element<'a, Message> {
    let audio = |spec: &str, label: String| Audio {
        spec: spec.to_owned(),
        label,
    };
    let (mut choices, devices) = if inputs {
        (
            vec![audio("discard", fl!("host-nowhere"))],
            &triib.audio_devices.1,
        )
    } else {
        (
            vec![
                audio("silence", fl!("host-silence")),
                audio("tone", fl!("host-tone")),
            ],
            &triib.audio_devices.0,
        )
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
        Message::HostAudio(entity_id, inputs, choice.spec)
    })
    .size(button::Size::ExtraSmall)
    .width(Fill)
    .into()
}

/// A picker of how many stream inputs, else outputs, a device has.
fn stream_picker<'a>(entity_id: EntityId, inputs: bool, current: u16) -> Element<'a, Message> {
    let mut counts = STREAM_COUNTS.to_vec();
    if !counts.contains(&current) {
        counts.push(current);
        counts.sort_unstable();
    }
    let _reading = dir::reading();
    dropdown::pick(counts, Some(current), move |count| {
        Message::HostStreams(entity_id, inputs, count)
    })
    .size(button::Size::ExtraSmall)
    .width(Fill)
    .into()
}
