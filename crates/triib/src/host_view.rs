//! This computer's own entities in the views: the Entities view's bar
//! that adds them, their section in the inspector with a button that
//! removes them, and their streams' audio and channels; on a Mac, the
//! Mac's own entity, which macOS runs.

use std::fmt;

use atdecc::EntityId;
use iced::widget::{container, space};
use iced::{Center, Element, Fill};
use scramble_ui::button::{self, Kind, Size};
use scramble_ui::dropdown::Entry;
use scramble_ui::font::{Type, styled};
use scramble_ui::icon::Icon;
use scramble_ui::popover::popover;
use scramble_ui::{component, dir, dropdown, style};
use triib_endpointd::config::{Direction, VIRTUAL};
use triib_endpointd::status::{DaemonStatus, Setup};

use crate::app::{Message, Triib};
use crate::fl;
use crate::host::StreamChange;
use crate::view::{VIEW_MARGIN, Value, field, property};

/// The daemon's status, where it runs on the interface shown.
fn status_here(triib: &Triib) -> Option<&DaemonStatus> {
    triib
        .endpoints
        .as_ref()
        .filter(|status| Some(status.interface.as_str()) == triib.settings.interface.as_deref())
}

/// What the daemon is setting up, or misses, while it does; or, before it
/// runs, the entity it starts for.
fn setup_note(triib: &Triib) -> Option<String> {
    let Some(status) = status_here(triib) else {
        return triib
            .starting_entities()
            .next()
            .map(|starting| fl!("host-starting-entity", name = starting.name.clone()));
    };
    let interface = status.interface.clone();
    match status.setup? {
        Setup::Link => Some(fl!("host-setting-up", interface = interface)),
        Setup::NoLink => Some(fl!("host-no-link", interface = interface)),
        Setup::Ptp4l => Some(fl!("host-starting-ptp4l")),
        Setup::NoLinuxptp => Some(fl!("host-no-linuxptp")),
        Setup::Other => None,
    }
}

/// What one of this computer's entities, added but not heard yet, waits on
/// before triib hears it: the daemon to start and run it, the link, then
/// its next advertisement.
pub fn starting_step(triib: &Triib, entity_id: EntityId) -> String {
    let Some(status) = status_here(triib)
        .filter(|status| crate::host::status_of(Some(status), entity_id).is_some())
    else {
        return fl!("host-starting");
    };
    let interface = status.interface.clone();
    match status.setup {
        Some(Setup::Link) => fl!("host-setting-up", interface = interface),
        Some(Setup::NoLink) => fl!("host-no-link", interface = interface),
        _ => fl!("host-waiting-network"),
    }
}

/// The bar above the Entities view's table, after the view's `title`
/// where there is one: why this computer cannot add entities of its own,
/// or the button that adds one.
pub fn bar(triib: &Triib, title: Option<String>) -> Option<Element<'_, Message>> {
    if triib.avb_enabled == Some(false) {
        return mac_avb_off(triib, title);
    }
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
    } else if let Some(note) = setup_note(triib) {
        Some(note)
    } else if triib
        .endpoints
        .as_ref()
        .is_some_and(|status| status.gptp.starts_with("ptp4l"))
    {
        Some(fl!("host-no-ptp4l"))
    } else {
        None
    };
    let add = button::with_icon(Kind::Tonal, Icon::Add, fl!("host-add-entity"))
        .size(Size::ExtraSmall)
        .on_press_maybe(able.then_some(Message::HostAdded));
    let text: Element<'_, Message> = match note {
        Some(note) => styled(note, Type::BodySmall)
            .style(style::on_surface_variant)
            .width(Fill)
            .align_x(dir::text_start())
            .into(),
        None => space().width(Fill).into(),
    };
    Some(
        container(crate::view::titled(
            title,
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
                add,
            ]
            .spacing(8)
            .align_y(Center),
        ))
        // The button a view's margin from the toolbar and the table, as the
        // other views' filters are, in a row the toggle makes 8 taller.
        .padding([VIEW_MARGIN - 4.0, VIEW_MARGIN])
        .into(),
    )
}

/// On a Mac where AVB is off on the interface, so the Mac has no entity
/// of its own: where to turn it on, after the view's `title`.
fn mac_avb_off(triib: &Triib, title: Option<String>) -> Option<Element<'_, Message>> {
    let interface = triib.settings.interface.clone()?;
    Some(
        container(crate::view::titled(
            title,
            scramble_ui::row![
                styled(fl!("mac-avb-off", interface = interface), Type::BodySmall)
                    .style(style::on_surface_variant)
                    .width(Fill)
                    .align_x(dir::text_start()),
                open_audio_midi_setup(),
            ]
            .spacing(8)
            .align_y(Center),
        ))
        // As far from the toolbar and the table as the other views' filters.
        .padding(VIEW_MARGIN)
        .into(),
    )
}

fn open_audio_midi_setup<'a>() -> Element<'a, Message> {
    button::button(Kind::Tonal, fl!("mac-open-audio-midi-setup"))
        .size(Size::ExtraSmall)
        .on_press(Message::OpenAudioMidiSetup)
        .into()
}

/// The inspector's section for the Mac's own entity, which macOS runs:
/// the Core Audio device apps use for its streams, and Audio MIDI Setup,
/// where macOS sets AVB up.
pub fn mac_sections(triib: &Triib, entity_id: EntityId) -> Vec<Element<'_, Message>> {
    if !triib.is_this_mac(entity_id) {
        return Vec::new();
    }
    vec![
        component::section(fl!("netmap-this-computer")),
        property(
            fl!("mac-audio-device"),
            triib
                .mac_audio_device
                .clone()
                .unwrap_or_else(|| fl!("mac-no-audio-device")),
        ),
        styled(fl!("mac-entity-note"), Type::BodySmall)
            .style(style::on_surface_variant)
            .into(),
        open_audio_midi_setup(),
    ]
}

/// The inspector's section that listens to the Mac's own entity on this
/// computer: a button that starts and stops, the channels and the output,
/// the gain, and what the listening helper says.
pub fn listen_sections(triib: &Triib, entity_id: EntityId) -> Vec<Element<'_, Message>> {
    use crate::monitor::{Listen, Side};

    if !triib.is_this_mac(entity_id) {
        return Vec::new();
    }
    let listen = triib.settings.listen.clone();
    // The Mac's own device, until another input is named.
    let input = listen
        .input
        .clone()
        .or_else(|| triib.mac_audio_device.clone());
    let with = {
        let listen = listen.clone();
        let input = input.clone();
        move |change: &dyn Fn(&mut Listen)| {
            let mut changed = Listen {
                input: input.clone(),
                ..listen.clone()
            };
            change(&mut changed);
            Message::Listen(changed)
        }
    };
    let mut items = vec![component::section(fl!("listen-section"))];
    let toggle = if listen.enabled {
        button::button(Kind::Filled, fl!("listen-stop"))
    } else {
        button::button(Kind::Tonal, fl!("listen-start"))
    };
    let toggled = with(&|listen| listen.enabled = !listen.enabled);
    items.push(
        toggle
            .size(Size::ExtraSmall)
            .on_press_maybe(input.is_some().then_some(toggled))
            .into(),
    );
    if let Some(status) = listen_status(triib) {
        items.push(
            styled(status, Type::BodySmall)
                .style(style::on_surface_variant)
                .into(),
        );
    }
    // What it receives, or what it sends on one of its output streams,
    // which takes a tap only macOS 14.2 and later have.
    let hears_sent = crate::monitor::hears_sent();
    let mut sides = vec![Choice {
        value: (Side::Received, 0),
        label: fl!("listen-received"),
    }];
    if hears_sent {
        sides.extend(
            triib
                .mac_output_streams
                .iter()
                .enumerate()
                .map(|(stream, &channels)| Choice {
                    value: (Side::Sent, stream as u16),
                    label: fl!("listen-sent", number = (stream + 1), channels = channels),
                }),
        );
    }
    let current_side = match listen.side {
        Side::Received => (Side::Received, 0),
        Side::Sent => (Side::Sent, listen.stream),
    };
    let pick_side = with.clone();
    items.push(field(
        fl!("listen-source"),
        choose(sides, current_side, move |(side, stream)| {
            pick_side(&|listen| {
                listen.side = side;
                listen.stream = stream;
                listen.first_channel = 0;
            })
        }),
    ));
    if !hears_sent && !triib.mac_output_streams.is_empty() {
        items.push(
            styled(fl!("listen-sent-needs-macos"), Type::BodySmall)
                .style(style::on_surface_variant)
                .into(),
        );
    }
    // The channels to pick from: its audio inputs', as its configuration
    // has them, or the output stream's.
    let count: u16 = match listen.side {
        Side::Sent => triib
            .mac_output_streams
            .get(usize::from(listen.stream))
            .copied(),
        Side::Received => triib.models.get(&entity_id).map(|model| {
            model
                .streams(true)
                .filter(|stream| !stream.is_media_clock())
                .filter_map(|stream| stream.current_format.channels())
                .sum()
        }),
    }
    .filter(|&count| count > 0)
    .unwrap_or(8);
    let mut channels: Vec<Choice<(u16, u16)>> = (0..count)
        .map(|first| Choice {
            value: (first, 1),
            label: fl!("listen-channel", number = (first + 1)),
        })
        .collect();
    channels.extend((0..count.saturating_sub(1)).step_by(2).map(|first| Choice {
        value: (first, 2),
        label: fl!(
            "listen-channel-pair",
            first = (first + 1),
            last = (first + 2)
        ),
    }));
    let pick_channels = with.clone();
    items.push(field(
        fl!("listen-channels"),
        choose(
            channels,
            (listen.first_channel, listen.channels),
            move |(first, count)| {
                pick_channels(&|listen| {
                    listen.first_channel = first;
                    listen.channels = count;
                })
            },
        ),
    ));
    let mut outputs = vec![Choice {
        value: None,
        label: fl!("listen-default-output"),
    }];
    outputs.extend(
        triib
            .audio_devices
            .1
            .iter()
            .filter(|name| Some(*name) != input.as_ref() && name.as_str() != "default")
            .map(|name| Choice {
                value: Some(name.clone()),
                label: name.clone(),
            }),
    );
    let pick_output = with.clone();
    items.push(field(
        fl!("listen-output"),
        choose(outputs, listen.output.clone(), move |output| {
            pick_output(&|listen| listen.output = output.clone())
        }),
    ));
    let mut gains: Vec<f32> = vec![-24.0, -18.0, -12.0, -6.0, 0.0, 6.0, 12.0];
    if !gains.contains(&listen.gain_db) {
        gains.push(listen.gain_db);
        gains.sort_by(f32::total_cmp);
    }
    let gains = gains
        .into_iter()
        .map(|gain| Choice {
            value: gain,
            label: fl!("listen-gain-db", db = decibels(gain)),
        })
        .collect();
    items.push(field(
        fl!("listen-gain"),
        choose(gains, listen.gain_db, move |gain| {
            with(&|listen| listen.gain_db = gain)
        }),
    ));
    items
}

/// What the listening helper says, in words; nothing while not listening.
fn listen_status(triib: &Triib) -> Option<String> {
    use crate::monitor::State;

    let listen = &triib.settings.listen;
    let status = &triib.monitor_status;
    if status.state != State::Off
        && listen.side == crate::monitor::Side::Sent
        && !crate::monitor::hears_sent()
    {
        return Some(fl!("listen-sent-needs-macos"));
    }
    Some(match &status.state {
        State::Off => return None,
        State::Starting => fl!("listen-starting"),
        State::WaitingForInput(_) => fl!(
            "listen-waiting-input",
            device = listen.input.clone().unwrap_or_default()
        ),
        State::WaitingForOutput(_) => fl!(
            "listen-waiting-output",
            device = listen
                .output
                .clone()
                .unwrap_or_else(|| fl!("listen-default-output"))
        ),
        State::Playing if status.peak <= 1e-5 => fl!("listen-playing-silent"),
        State::Playing => fl!(
            "listen-playing",
            db = decibels((20.0 * status.peak.log10()).round())
        ),
    })
}

/// Decibels as said, with their sign: "+6", "0", "-12".
fn decibels(db: f32) -> String {
    if db > 0.0 {
        format!("+{db}")
    } else {
        format!("{db}")
    }
}

/// One choice of a picker: its value and its words.
#[derive(Debug, Clone)]
struct Choice<T> {
    value: T,
    label: String,
}

impl<T: PartialEq> PartialEq for Choice<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> fmt::Display for Choice<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.label)
    }
}

/// A picker of `choices`, `current` picked.
fn choose<'a, T: Clone + PartialEq + fmt::Debug + 'a>(
    choices: Vec<Choice<T>>,
    current: T,
    pick: impl Fn(T) -> Message + 'a,
) -> Value<'a> {
    let shown = choices
        .iter()
        .find(|choice| choice.value == current)
        .cloned();
    let labels: Vec<String> = choices.iter().map(|choice| choice.label.clone()).collect();
    let view = dropdown::pick(choices, shown, move |choice: Choice<T>| pick(choice.value))
        .size(button::Size::ExtraSmall)
        .width(Fill);
    Value::picker(view, labels)
}

/// The channel counts the inspector offers; six samples of each fit in a
/// frame up to 60.
const CHANNEL_COUNTS: [u16; 11] = [1, 2, 4, 6, 8, 12, 16, 24, 32, 48, 56];

/// One choice for a stream's audio: what entities.toml says, and the
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

/// The inspector's section for one of this computer's entities: its
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

/// An entity's button that adds a stream, with a menu of the kinds it
/// adds: audio or a media clock, either way.
pub fn add_buttons(triib: &Triib, entity_id: EntityId) -> Vec<Element<'_, Message>> {
    if !triib.is_host(entity_id) {
        return Vec::new();
    }
    let open = triib.add_stream_open;
    let entry = |label: String, direction, clock| {
        Entry::item(
            label,
            Message::HostStream(entity_id, StreamChange::Add { direction, clock }),
        )
    };
    let menu = open.then(|| {
        dropdown::surface(
            vec![
                entry(fl!("host-add-audio-input"), Direction::In, false),
                entry(fl!("host-add-audio-output"), Direction::Out, false),
                entry(fl!("host-add-clock-input"), Direction::In, true),
                entry(fl!("host-add-clock-output"), Direction::Out, true),
            ],
            0.0,
        )
    });
    let add = button::with_icon(Kind::Tonal, Icon::Add, fl!("host-add-stream"))
        .size(Size::ExtraSmall)
        .on_press(Message::AddStreamMenu(!open));
    vec![
        popover(add, menu, Message::AddStreamMenu(false))
            .close_on_choice()
            .into(),
    ]
}

/// What this computer sets for one of its entities' streams, the stream
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
    // The entity's streams one way, in the order it numbers them.
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
    // A media clock has no audio and no channels.
    if stream.clock {
        return Vec::new();
    }
    vec![
        field(
            audio_label,
            audio_picker(triib, input, &stream.audio, move |spec| {
                Message::HostStream(entity_id, StreamChange::Audio(at, spec))
            }),
        ),
        field(
            fl!("host-channels"),
            channel_picker(stream.channels, move |count| {
                Message::HostStream(entity_id, StreamChange::Channels(at, count))
            }),
        ),
    ]
}

/// The button that removes an entity's stream input or output `index`,
/// but not its last stream; none for others' entities.
pub fn remove_stream(
    triib: &Triib,
    entity_id: EntityId,
    input: bool,
    index: u16,
) -> Option<Element<'_, Message>> {
    let endpoint = crate::host::status_of(triib.endpoints.as_ref(), entity_id)?;
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
/// `current` picked: the stream's own virtual audio device, the
/// computer's devices, or a tone, silence or nowhere.
fn audio_picker<'a>(
    triib: &'a Triib,
    inputs: bool,
    current: &str,
    pick: impl Fn(String) -> Message + 'a,
) -> Value<'a> {
    let audio = |spec: &str, label: String| Audio {
        spec: spec.to_owned(),
        label,
    };
    let mut choices = vec![audio(VIRTUAL, fl!("host-virtual-device"))];
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
    let labels: Vec<String> = choices.iter().map(|choice| choice.label.clone()).collect();
    let _reading = dir::reading();
    let view = dropdown::pick(choices, Some(current), move |choice: Audio| {
        pick(choice.spec)
    })
    .size(button::Size::ExtraSmall)
    .width(Fill);
    Value::picker(view, labels)
}

/// A picker of channel counts, with `current` picked.
fn channel_picker<'a>(current: u16, pick: impl Fn(u16) -> Message + 'a) -> Value<'a> {
    let mut counts = CHANNEL_COUNTS.to_vec();
    if !counts.contains(&current) {
        counts.push(current);
        counts.sort_unstable();
    }
    let labels: Vec<String> = counts.iter().map(ToString::to_string).collect();
    let _reading = dir::reading();
    let view = dropdown::pick(counts, Some(current), pick)
        .size(button::Size::ExtraSmall)
        .width(Fill);
    Value::picker(view, labels)
}
