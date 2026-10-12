//! The window: toolbar, the active view, the inspector on the selected
//! entity and the status bar.

use std::fmt;

use atdecc::aem::AvbInfoFlags;
use atdecc::descriptor::DescriptorType;
use atdecc::descriptor::{
    AudioUnitDescriptor, AvbInterfaceDescriptor, ClockDomainDescriptor, ClockSourceDescriptor,
    ConfigurationDescriptor, EntityDescriptor, LocaleDescriptor, SamplingRate, StreamDescriptor,
    StreamPortDescriptor, StringsDescriptor,
};
use atdecc::model::{EntityModel, EnumerationState};
use atdecc::stream_format::StreamFormat;
use atdecc::{DiscoveredEntity, EntityCapabilities, EntityId};
use avb_net::Interface;
use iced::widget::{container, space, text_input};
use iced::{Center, Element, Fill, Length, Theme};
use scramble_ui::button::{self, Kind, Size};
use scramble_ui::component::{self, TOOLBAR_HEIGHT};
use scramble_ui::font::{Type, styled};
use scramble_ui::icon::{self, Icon};
use scramble_ui::{Scheme, dir, dropdown, style};

use crate::app::{Message, NetworkState, Triib};
use crate::describe;
use crate::fl;
use crate::mapping_view;
use crate::network::{Action, Failure, NameTarget, Remedy};
use crate::settings::View;

const STATUS_BAR_HEIGHT: f32 = 28.0;
const SEARCH_WIDTH: f32 = 280.0;
const INTERFACE_PICKER_WIDTH: f32 = 320.0;
/// Between the toolbar's ends and its first and last tools.
const TOOLBAR_PADDING: f32 = 12.0;
/// The narrowest window that has room for the inspector beside the view.
const INSPECTOR_BESIDE: f32 = 720.0;
/// The least height the view keeps above the log panel.
const VIEW_ABOVE_LOG: f32 = 160.0;
/// The narrowest the interface picker gets, and below which its choices
/// leave out all but each interface's name and link.
const PICKER_NARROWEST: f32 = 160.0;
/// Below this width the status bar leaves out the interface.
const STATUS_FULL: f32 = 640.0;
pub fn window(triib: &Triib) -> Element<'_, Message> {
    let window: Element<'_, Message> = iced::widget::column![
        toolbar(triib),
        container(iced::widget::responsive(move |size| {
            panels(triib, size)
        }))
        .height(Fill)
        .style(style::surface),
        status_bar(triib),
    ]
    .into();
    if triib.settings_open {
        crate::settings_view::dialog(triib, window)
    } else if triib.presets_open {
        crate::presets::dialog(triib, window)
    } else {
        window
    }
}

/// The active view with the log under it and the inspector beside both,
/// or on a narrow window the inspector in the view's place until it is
/// closed.
fn panels(triib: &Triib, size: iced::Size) -> Element<'_, Message> {
    let beside = triib.settings.inspector && size.width >= INSPECTOR_BESIDE;
    let main: Element<'_, Message> = if triib.settings.inspector && !beside {
        inspector(triib, Length::Fill)
    } else {
        container(content(triib)).width(Fill).height(Fill).into()
    };
    let main = if triib.settings.log {
        let width = if beside {
            size.width - component::SIDE_SHEET_WIDTH
        } else {
            size.width
        };
        // The view keeps room above the log however tall it was dragged.
        let most = (size.height - VIEW_ABOVE_LOG).max(crate::log_view::LEAST_HEIGHT);
        let height = triib
            .settings
            .log_height
            .clamp(crate::log_view::LEAST_HEIGHT, most);
        iced::widget::column![main, crate::log_view::panel(triib, width, height, most)].into()
    } else {
        main
    };
    if !beside {
        return main;
    }
    // The panels keep their sides in every language.
    iced::widget::row![main, inspector(triib, component::SIDE_SHEET_WIDTH.into())]
        .height(Fill)
        .into()
}

/// An interface as the picker lists it.
#[derive(Debug, Clone, PartialEq)]
struct Choice {
    name: String,
    label: String,
}

impl Choice {
    /// The interface with its link, speed and kind, or only its link when
    /// `brief`.
    fn new(interface: &Interface, brief: bool) -> Self {
        let mut parts = vec![interface.name.clone()];
        if !interface.up {
            parts.push(fl!("interface-link-down"));
        } else if brief {
            return Self {
                name: interface.name.clone(),
                label: crate::i18n::list(parts),
            };
        } else if let Some(speed) = interface.speed {
            parts.push(speed_text(speed));
        }
        if interface.wireless {
            parts.push(fl!("interface-wireless"));
        } else if interface.hardware_clock.is_some() {
            parts.push(fl!("interface-hardware-clock"));
        }
        if !interface.physical {
            parts.push(fl!("interface-virtual"));
        }
        Self {
            name: interface.name.clone(),
            label: crate::i18n::list(parts),
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.label)
    }
}

/// The toolbar, with groups that do not fit the window's width in a
/// "More" menu at its end.
fn toolbar(triib: &Triib) -> Element<'_, Message> {
    component::toolbar(iced::widget::responsive(move |size| {
        toolbar_at(triib, size.width)
    }))
}

fn toolbar_at(triib: &Triib, width: f32) -> Element<'_, Message> {
    // The bar keeps its layout in every language; its text reads in the
    // language's direction.
    let _fixed = dir::fixed();
    use component::{DIVIDER_WIDTH, TOOL_WIDTH, TOOLBAR_GAP};
    let tools = |count: f32| count * TOOL_WIDTH + (count - 1.0) * 4.0;
    // The space between the bar's two ends takes a gap of its own.
    let room = width - 2.0 * TOOLBAR_PADDING - TOOLBAR_GAP;
    // On a narrow window the picker leaves room for the inspector and
    // "More" buttons.
    let picker_width = INTERFACE_PICKER_WIDTH
        .min(room - 2.0 * (TOOL_WIDTH + TOOLBAR_GAP) - TOOLBAR_GAP)
        .max(PICKER_NARROWEST);
    let brief = picker_width < INTERFACE_PICKER_WIDTH;
    let choices: Vec<Choice> = triib
        .listed_interfaces()
        .map(|interface| Choice::new(interface, brief))
        .collect();
    let selected = triib
        .interface()
        .map(|interface| Choice::new(interface, brief));
    // The picker reads in the language's direction, though the bar holding
    // it keeps its layout.
    let picker: Element<'_, Message> = {
        let _reading = dir::reading();
        dropdown::pick(choices, selected, |choice: Choice| {
            Message::InterfacePicked(choice.name)
        })
        .placeholder(fl!("toolbar-choose-interface"))
        .size(button::Size::ExtraSmall)
        .width(picker_width)
        .into()
    };

    let view = triib.settings.view;
    let view_tool = |glyph, label, target| {
        component::toggle_tool(glyph, label, view == target, Message::ViewPicked(target))
    };

    // Each slot: its content, its width, when it moves into "More" (lowest
    // first), and whether a divider goes before it in the bar.
    let mut slots: Vec<(Element<'_, Message>, f32, Option<u8>, bool)> = vec![
        (
            component::tip(picker, fl!("toolbar-interface")),
            picker_width,
            None,
            false,
        ),
        (
            component::toggle_tool(
                Icon::FilterList,
                if triib.settings.hide_virtual_interfaces {
                    fl!("toolbar-show-virtual")
                } else {
                    fl!("toolbar-hide-virtual")
                },
                triib.settings.hide_virtual_interfaces,
                Message::VirtualInterfacesToggled,
            ),
            TOOL_WIDTH,
            Some(0),
            false,
        ),
        (
            component::group([
                view_tool(Icon::GridOn, fl!("toolbar-connections"), View::Matrix),
                view_tool(Icon::Hub, fl!("toolbar-network"), View::Network),
                view_tool(Icon::ViewList, fl!("toolbar-entities"), View::Entities),
            ]),
            DIVIDER_WIDTH + TOOLBAR_GAP + tools(3.0),
            Some(3),
            true,
        ),
        (
            {
                // With Shift held, it clears and rescans instead.
                let running = matches!(triib.network_state, NetworkState::Running { .. });
                if triib.shift {
                    component::tool(
                        Icon::ResetAll,
                        fl!("toolbar-rescan"),
                        running.then_some(Message::Rescan),
                    )
                } else {
                    component::tool(
                        Icon::Refresh,
                        fl!("toolbar-rediscover"),
                        running.then_some(Message::Rediscover),
                    )
                }
            },
            DIVIDER_WIDTH + TOOLBAR_GAP + TOOL_WIDTH,
            Some(1),
            true,
        ),
    ];
    let right = slots.len();
    slots.push((
        component::search_bar(
            text_input(&fl!("toolbar-search"), &triib.search)
                .on_input(Message::SearchChanged)
                // Typed text on the side it starts on: Hebrew from the
                // right, English from the left.
                .align_x(dir::input_align(&triib.search)),
            Vec::new(),
            SEARCH_WIDTH,
        ),
        SEARCH_WIDTH,
        Some(2),
        false,
    ));
    slots.push((
        component::tool(
            Icon::Bookmarks,
            fl!("toolbar-presets"),
            Some(Message::PresetsOpened),
        ),
        TOOL_WIDTH,
        Some(1),
        false,
    ));
    slots.push((
        component::toggle_tool(
            Icon::History,
            fl!("toolbar-log"),
            triib.settings.log,
            Message::LogToggled,
        ),
        TOOL_WIDTH,
        Some(3),
        false,
    ));
    slots.push((
        component::toggle_tool(
            Icon::Info,
            fl!("toolbar-inspector"),
            triib.settings.inspector,
            Message::InspectorToggled,
        ),
        TOOL_WIDTH,
        None,
        false,
    ));
    slots.push((
        component::tool(
            Icon::Settings,
            fl!("toolbar-settings"),
            Some(Message::SettingsOpened),
        ),
        TOOL_WIDTH,
        Some(4),
        false,
    ));

    let widths: Vec<(f32, Option<u8>)> = slots
        .iter()
        .map(|(_, width, order, _)| (*width, *order))
        .collect();
    let shown = component::fitting_slots(room, &widths);
    let mut bar = iced::widget::row![].spacing(TOOLBAR_GAP).align_y(Center);
    let mut hidden = Vec::new();
    for (index, ((element, _, _, divider), shown)) in slots.into_iter().zip(shown).enumerate() {
        if index == right {
            bar = bar.push(space::horizontal());
        }
        if shown {
            if divider {
                bar = bar.push(component::toolbar_divider());
            }
            bar = bar.push(element);
        } else {
            hidden.push(element);
        }
    }
    if !hidden.is_empty() {
        bar = bar.push(component::overflow(
            hidden,
            triib.overflow_open,
            Message::OverflowToggled,
            Message::OverflowClosed,
        ));
    }
    container(bar)
        .padding([0.0, TOOLBAR_PADDING])
        .height(TOOLBAR_HEIGHT)
        .align_y(Center)
        .into()
}

/// What to show instead of entities: no interface, starting, a failure,
/// or listening with none heard yet.
fn network_state_view<'a>(triib: &'a Triib, state: &'a NetworkState) -> Element<'a, Message> {
    let interface = triib.settings.interface.as_deref().unwrap_or_default();
    match state {
        NetworkState::Idle => component::empty_state(
            Icon::Lan,
            fl!("state-no-interface"),
            fl!("state-no-interface-note"),
        ),
        NetworkState::Starting => component::empty_state(
            Icon::Lan,
            fl!("state-starting"),
            fl!("state-starting-note", interface = interface),
        ),
        NetworkState::Running { .. } => component::empty_state(
            Icon::WifiTethering,
            fl!("state-listening"),
            fl!("state-listening-note", interface = interface),
        ),
        NetworkState::Failed(failure) => failure_view(interface, failure),
    }
}

fn failure_view<'a>(interface: &str, failure: &'a Failure) -> Element<'a, Message> {
    let try_again = || {
        button::with_icon(Kind::Tonal, Icon::Refresh, fl!("state-try-again"))
            .on_press(Message::RetryNetwork)
    };
    let (headline, action) = match &failure.remedy {
        Some(Remedy::Command(command)) => (
            fl!("state-permission-needed"),
            button::with_icon(Kind::Tonal, Icon::ContentCopy, fl!("state-copy-command"))
                .on_press(Message::Copy(command.clone())),
        ),
        Some(Remedy::GetNpcap(link)) => (
            fl!("state-npcap-needed"),
            button::with_icon(Kind::Tonal, Icon::Link, fl!("state-get-npcap"))
                .on_press(Message::OpenLink(link.clone())),
        ),
        Some(Remedy::Permission) => (fl!("state-permission-needed"), try_again()),
        None => (fl!("state-cannot-use", interface = interface), try_again()),
    };
    let mut details = iced::widget::column![
        styled(failure.message.as_str(), Type::BodyMedium)
            .style(style::on_surface_variant)
            .center(),
    ]
    .spacing(12)
    .align_x(Center);
    if let Some(Remedy::Command(command)) = &failure.remedy {
        details = details.push(
            styled(command.as_str(), Type::BodySmall)
                .style(style::on_surface_variant)
                .center(),
        );
    }
    container(
        iced::widget::column![
            scramble_ui::icon::icon(Icon::Error, 48).style(style::error_text),
            styled(headline, Type::TitleLarge),
            details,
            action,
        ]
        .spacing(16)
        .align_x(Center)
        .padding(16)
        .max_width(420),
    )
    .center(Fill)
    .into()
}

fn content(triib: &Triib) -> Element<'_, Message> {
    match triib.settings.view {
        View::Matrix => match &triib.network_state {
            NetworkState::Running { .. } | NetworkState::Starting => crate::matrix::view(triib),
            state => network_state_view(triib, state),
        },
        View::Entities => {
            let body = if !triib.entities.is_empty() {
                crate::entity_table::view(triib)
            } else {
                match &triib.network_state {
                    NetworkState::Running { .. } | NetworkState::Starting => {
                        component::empty_state(
                            Icon::ViewList,
                            fl!("entities-none-yet"),
                            fl!("entities-none-yet-note"),
                        )
                    }
                    state => return network_state_view(triib, state),
                }
            };
            // This computer's own talkers and listeners are added here.
            match crate::host_view::bar(triib) {
                Some(bar) => scramble_ui::column![bar, body].into(),
                None => body,
            }
        }
        View::Network => match &triib.network_state {
            NetworkState::Running { .. } | NetworkState::Starting => crate::netmap::view(triib),
            state => network_state_view(triib, state),
        },
    }
}

fn inspector(triib: &Triib, width: Length) -> Element<'_, Message> {
    let open = triib.inspector_tab;
    let body: Element<'_, Message> = match triib.selected {
        None => note(fl!("inspector-select")),
        Some(entity_id) => match triib.entities.get(&entity_id) {
            None => note(fl!("inspector-offline", entity = entity_id.to_string())),
            Some(entity) => entity_details(triib, entity, triib.models.get(&entity_id), open),
        },
    };
    let tab = |label: String, glyph, tab: InspectorTab| component::Tab {
        label: crate::i18n::lasting(label),
        icon: Some(glyph),
        selected: open == tab,
        on_press: Message::InspectorTab(tab),
    };
    // The tabs keep their order in every language.
    let fixed = dir::fixed();
    let tabs = component::tabs(vec![
        tab(fl!("inspector-entity"), Icon::Info, InspectorTab::Entity),
        tab(
            fl!("inspector-streams"),
            Icon::GraphicEq,
            InspectorTab::Streams,
        ),
        tab(
            fl!("inspector-controls"),
            Icon::Tune,
            InspectorTab::Controls,
        ),
        tab(
            fl!("inspector-diagnostics"),
            Icon::MonitorHeart,
            InspectorTab::Diagnostics,
        ),
        tab(
            fl!("inspector-descriptors"),
            Icon::AccountTree,
            InspectorTab::Descriptors,
        ),
    ]);
    drop(fixed);
    component::side_sheet_tabbed(
        fl!("inspector-title"),
        Message::InspectorToggled,
        tabs,
        body,
        width,
    )
}

/// A label and its value, in the inspector.
/// The ID of the field a name is edited in, for focusing it.
pub const NAME_FIELD: &str = "inspector-name";

/// A label and an element beside it, in the inspector.
fn labeled<'a>(
    label: impl Into<String>,
    value: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let label: String = label.into();
    scramble_ui::row![
        styled(label, Type::BodyMedium)
            .style(style::on_surface_variant)
            .width(Length::Fixed(132.0))
            .align_x(dir::text_start()),
        container(value)
            .width(Fill)
            .align_x(dir::horizontal_start()),
    ]
    .spacing(8)
    .align_y(Center)
    .into()
}

/// A label with an element under it, for pickers too wide to sit beside
/// their label.
pub(crate) fn stacked<'a>(
    label: impl Into<String>,
    value: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let label: String = label.into();
    scramble_ui::column![
        styled(label, Type::BodyMedium)
            .style(style::on_surface_variant)
            .align_x(dir::text_start()),
        container(value)
            .width(Fill)
            .align_x(dir::horizontal_start()),
    ]
    .spacing(4)
    .into()
}

/// A name the entity holds with a button to edit it, or while it is
/// edited a field to type it in.
fn name_property<'a>(triib: &'a Triib, label: String, target: NameTarget) -> Element<'a, Message> {
    if let Some((editing, text)) = &triib.editing
        && *editing == target
    {
        return scramble_ui::column![
            styled(label, Type::BodyMedium)
                .style(style::on_surface_variant)
                .align_x(dir::text_start()),
            name_field(text),
        ]
        .spacing(4)
        .into();
    }
    labeled(label, name_line(triib, target, fl!("common-not-set")))
}

/// A name with a button to edit it, `unset` standing in for none, or
/// while it is edited a field to type it in.
fn name_line<'a>(triib: &'a Triib, target: NameTarget, unset: String) -> Element<'a, Message> {
    if let Some((editing, text)) = &triib.editing
        && *editing == target
    {
        return name_field(text);
    }
    let name = triib.current_name(target);
    let shown = if name.is_empty() { unset } else { name };
    scramble_ui::row![
        styled(shown, Type::BodyMedium)
            .width(Fill)
            .align_x(dir::text_start()),
        component::tip(
            button::icon_button(Icon::Edit)
                .size(Size::ExtraSmall)
                .on_press(Message::EditName(target)),
            fl!("inspector-rename"),
        ),
    ]
    .spacing(4)
    .align_y(Center)
    .into()
}

/// The field a name is typed in, with buttons to save and cancel; Enter
/// saves too.
fn name_field<'a>(text: &str) -> Element<'a, Message> {
    let icon = |glyph, label: String, message| {
        component::tip(
            button::icon_button(glyph)
                .size(Size::ExtraSmall)
                .on_press(message),
            label,
        )
    };
    scramble_ui::row![
        text_input(&fl!("inspector-name"), text)
            .id(iced::widget::Id::new(NAME_FIELD))
            .align_x(dir::input_align(text))
            .placeholder_align(dir::horizontal_start())
            .on_input(Message::EditChanged)
            .on_submit(Message::EditSubmitted)
            .padding([6, 10])
            .style(style::outlined_field)
            .width(Fill),
        icon(Icon::Check, fl!("common-save"), Message::EditSubmitted),
        icon(Icon::Close, fl!("common-cancel"), Message::EditCanceled),
    ]
    .spacing(4)
    .align_y(Center)
    .into()
}

/// A picker in the inspector's style, offering `choices` with `shown`
/// picked; a change already on its way holds it still.
fn picker<'a, T>(
    choices: Vec<T>,
    shown: T,
    busy: bool,
    pick: impl Fn(T) -> Action + 'a,
) -> Element<'a, Message>
where
    T: ToString + PartialEq + 'a,
{
    dropdown::pick(choices, Some(shown), move |choice| {
        if busy {
            Message::Nothing
        } else {
            Message::Act(pick(choice))
        }
    })
    .size(button::Size::ExtraSmall)
    .width(Fill)
    .into()
}

/// A stream's format, with a picker of the formats it supports when it
/// has more than one.
fn format_picker<'a>(
    triib: &'a Triib,
    entity: EntityId,
    stream: &StreamDescriptor<'_>,
) -> Element<'a, Message> {
    let (descriptor_type, index) = (stream.descriptor_type, stream.index);
    let changing = triib
        .changing(|action| {
            matches!(action, Action::SetStreamFormat { entity: pending, descriptor_type: kind, index: place, .. }
                if *pending == entity && *kind == descriptor_type && *place == index)
        })
        .and_then(|action| match action {
            Action::SetStreamFormat { format, .. } => Some(*format),
            _ => None,
        });
    let shown = changing.unwrap_or(stream.current_format);
    let mut formats: Vec<StreamFormat> = stream.formats().collect();
    if !formats.contains(&stream.current_format) {
        formats.insert(0, stream.current_format);
    }
    if formats.len() < 2 {
        return styled(shown.to_string(), Type::BodySmall)
            .style(style::on_surface_variant)
            .into();
    }
    picker(formats, shown, changing.is_some(), move |format| {
        Action::SetStreamFormat {
            entity,
            descriptor_type,
            index,
            format,
        }
    })
}

/// A sampling rate in words, such as "48 kHz" or "44.1 kHz".
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Rate(pub(crate) SamplingRate);

impl fmt::Display for Rate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hertz = self.0.base_frequency();
        if hertz.is_multiple_of(1000) {
            write!(formatter, "{} kHz", hertz / 1000)?;
        } else {
            let kilohertz = crate::i18n::decimal((f64::from(hertz) / 1000.0).to_string());
            write!(formatter, "{kilohertz} kHz")?;
        }
        // The pull multiplies the base frequency (7.3.1.2).
        match self.0.pull() {
            0 => Ok(()),
            1 => formatter.write_str(" × 1/1.001"),
            2 => formatter.write_str(" × 1.001"),
            3 => formatter.write_str(" × 24/25"),
            4 => formatter.write_str(" × 25/24"),
            5 => formatter.write_str(" × 1/8"),
            pull => write!(formatter, " {}", fl!("rate-pull", pull = pull)),
        }
    }
}

/// An audio unit's sampling rate, with a picker of the rates it supports
/// when it has more than one.
fn rate_picker<'a>(
    triib: &'a Triib,
    entity: EntityId,
    unit: &AudioUnitDescriptor<'_>,
) -> Element<'a, Message> {
    let index = unit.index;
    let changing = triib
        .changing(|action| {
            matches!(action, Action::SetSamplingRate { entity: pending, unit: place, .. }
                if *pending == entity && *place == index)
        })
        .and_then(|action| match action {
            Action::SetSamplingRate { rate, .. } => Some(Rate(*rate)),
            _ => None,
        });
    let current = Rate(unit.current_sampling_rate);
    let shown = changing.unwrap_or(current);
    let mut rates: Vec<Rate> = unit.sampling_rates().map(Rate).collect();
    if !rates.contains(&current) {
        rates.insert(0, current);
    }
    if rates.len() < 2 {
        return styled(shown.to_string(), Type::BodyMedium).into();
    }
    picker(rates, shown, changing.is_some(), move |rate| {
        Action::SetSamplingRate {
            entity,
            unit: index,
            rate: rate.0,
        }
    })
}

/// A clock source by its name.
#[derive(Debug, Clone, PartialEq)]
struct Source {
    index: u16,
    name: String,
}

impl fmt::Display for Source {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

/// A picker of the clock sources a clock domain can use, or the name of
/// its clock source when it has only that one.
pub(crate) fn source_picker<'a>(
    triib: &'a Triib,
    entity: EntityId,
    model: &EntityModel,
    domain: &ClockDomainDescriptor<'_>,
) -> Result<Element<'a, Message>, String> {
    let domain_index = domain.index;
    let source = |index: u16| Source {
        index,
        name: model
            .name_of(DescriptorType::CLOCK_SOURCE, index)
            .map_or_else(
                || fl!("clock-source-numbered", index = index),
                str::to_owned,
            ),
    };
    let changing = triib
        .changing(|action| {
            matches!(action, Action::SetClockSource { entity: pending, domain: place, .. }
                if *pending == entity && *place == domain_index)
        })
        .and_then(|action| match action {
            Action::SetClockSource { source, .. } => Some(*source),
            _ => None,
        });
    let shown = source(changing.unwrap_or(domain.clock_source_index));
    let mut sources: Vec<Source> = domain.clock_sources().map(source).collect();
    if !sources
        .iter()
        .any(|known| known.index == domain.clock_source_index)
    {
        sources.insert(0, source(domain.clock_source_index));
    }
    if sources.len() < 2 {
        return Err(shown.name);
    }
    Ok(picker(sources, shown, changing.is_some(), move |picked| {
        Action::SetClockSource {
            entity,
            domain: domain_index,
            source: picked.index,
        }
    }))
}

pub(crate) fn property<'a>(label: impl Into<String>, value: String) -> Element<'a, Message> {
    let label: String = label.into();
    scramble_ui::row![
        styled(label, Type::BodyMedium)
            .style(style::on_surface_variant)
            .width(Length::Fixed(132.0))
            .align_x(dir::text_start()),
        styled(value, Type::BodyMedium)
            .width(Fill)
            .align_x(dir::text_start()),
    ]
    .spacing(8)
    .into()
}

/// Everything read from an entity and everything it advertises, in
/// sections.
/// The inspector's tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InspectorTab {
    /// Names, product, media clock and what the entity advertises.
    #[default]
    Entity,
    Streams,
    Controls,
    /// AVB interfaces and counters.
    Diagnostics,
    Descriptors,
}

/// The selected entity's details on the inspector's open tab.
fn entity_details<'a>(
    triib: &'a Triib,
    entity: &'a DiscoveredEntity,
    model: Option<&'a EntityModel>,
    tab: InspectorTab,
) -> Element<'a, Message> {
    let entity_id = entity.entity_id();
    let mut column = scramble_ui::column![].spacing(6);
    if tab == InspectorTab::Entity
        && entity
            .adp
            .entity_capabilities
            .contains(EntityCapabilities::AEM_IDENTIFY_CONTROL_INDEX_VALID)
    {
        let action = Action::Identify(entity_id);
        let busy = triib.pending.contains(&action);
        column = column.push(
            button::with_icon(Kind::Tonal, Icon::Lightbulb, fl!("inspector-identify"))
                .on_press_maybe((!busy).then_some(Message::Act(action))),
        );
    }
    let Some(model) = model else {
        return match tab {
            InspectorTab::Entity => column.extend(advertisement_details(entity)).into(),
            _ => column.push(note(fl!("inspector-model-not-read"))).into(),
        };
    };
    column = column.extend(model_state(model));
    let items = match tab {
        InspectorTab::Entity => {
            let mut items = crate::host_view::sections(triib, entity_id);
            items.extend(entity_sections(triib, entity_id, model));
            items.extend(advertisement_details(entity));
            items
        }
        InspectorTab::Streams => or_note(
            stream_sections(triib, entity_id, model),
            fl!("inspector-no-streams"),
        ),
        InspectorTab::Controls => or_note(
            crate::controls_view::controls(triib, entity_id, model),
            fl!("inspector-no-controls"),
        ),
        InspectorTab::Diagnostics => {
            let mut items = crate::lite_view::entity_alarms(triib, entity_id);
            items.extend(interface_sections(model));
            items.extend(crate::lite_view::bandwidth(model));
            items.extend(crate::diagnostics_view::diagnostics(model));
            or_note(items, fl!("inspector-no-diagnostics"))
        }
        InspectorTab::Descriptors => descriptor_tree(triib, entity_id, model),
    };
    column.extend(items).into()
}

/// `items`, or `text` when there are none.
fn or_note<'a>(items: Vec<Element<'a, Message>>, text: String) -> Vec<Element<'a, Message>> {
    if items.is_empty() {
        vec![note(text)]
    } else {
        items
    }
}

fn note<'a>(text: impl Into<String>) -> Element<'a, Message> {
    styled(text.into(), Type::BodyMedium)
        .style(style::on_surface_variant)
        .into()
}

/// How reading the model goes, while it is read or when it failed.
fn model_state<'a>(model: &EntityModel) -> Vec<Element<'a, Message>> {
    let mut items = Vec::new();
    match model.state {
        EnumerationState::Reading => items.push(
            styled(
                fl!("inspector-reading", count = model.descriptor_count()),
                Type::BodyMedium,
            )
            .style(style::on_surface_variant)
            .into(),
        ),
        EnumerationState::Failed(failure) => items.push(
            styled(
                fl!("inspector-read-failed", reason = describe::failure(failure)),
                Type::BodyMedium,
            )
            .style(style::error_text)
            .into(),
        ),
        EnumerationState::Complete | EnumerationState::NotRead => {}
    }
    items
}

/// The entity's names, product, firmware, configuration and Milan
/// support, then its media clock, with the names, clock source and
/// sampling rate editable.
fn entity_sections<'a>(
    triib: &'a Triib,
    entity_id: EntityId,
    model: &'a EntityModel,
) -> Vec<Element<'a, Message>> {
    let mut items = Vec::new();
    let or_unset = |text: &str| {
        if text.is_empty() {
            fl!("common-not-set")
        } else {
            text.to_owned()
        }
    };
    let entity_name = |name_index| NameTarget {
        entity: entity_id,
        descriptor_type: DescriptorType::ENTITY,
        index: 0,
        name_index,
    };
    if let Some(descriptor) = model.entity() {
        items.push(component::section(fl!("entity-section")));
        items.push(name_property(triib, fl!("entity-name"), entity_name(0)));
        items.push(name_property(triib, fl!("entity-group"), entity_name(1)));
        items.push(property(fl!("entity-product"), describe::product(model)));
        items.push(property(
            fl!("entity-firmware"),
            or_unset(descriptor.firmware_version),
        ));
        items.push(property(
            fl!("entity-serial-number"),
            or_unset(descriptor.serial_number),
        ));
        let configuration = model
            .name_of(
                DescriptorType::CONFIGURATION,
                descriptor.current_configuration,
            )
            .map(|name| {
                fl!(
                    "entity-configuration-of",
                    name = name,
                    number = (descriptor.current_configuration + 1),
                    count = descriptor.configurations_count
                )
            })
            .unwrap_or_else(|| (descriptor.current_configuration + 1).to_string());
        items.push(property(fl!("entity-configuration"), configuration));
        items.push(property(fl!("entity-milan"), describe::milan(model)));
    }
    let domains: Vec<_> = model.clock_domains().collect();
    if !domains.is_empty() {
        items.push(component::section(fl!("entity-media-clock")));
        for domain in domains {
            let name = model
                .name_of(DescriptorType::CLOCK_DOMAIN, domain.index)
                .map_or_else(|| fl!("entity-clock-domain"), str::to_owned);
            items.push(stacked(
                name,
                source_picker(triib, entity_id, model, &domain)
                    .unwrap_or_else(|name| styled(name, Type::BodyMedium).into()),
            ));
        }
        for unit in model.audio_units() {
            items.push(stacked(
                fl!("entity-sampling-rate"),
                rate_picker(triib, entity_id, &unit),
            ));
        }
    }
    items.extend(crate::lite_view::entity_section(model));
    items.extend(crate::wireless_view::entity_section(triib, model));
    items
}

/// The entity's streams with their formats and state, then its channel
/// mappings.
fn stream_sections<'a>(
    triib: &'a Triib,
    entity_id: EntityId,
    model: &'a EntityModel,
) -> Vec<Element<'a, Message>> {
    let mut items = Vec::new();
    for (input, title) in [(true, fl!("stream-inputs")), (false, fl!("stream-outputs"))] {
        let streams: Vec<_> = model.streams(input).collect();
        if streams.is_empty() {
            continue;
        }
        items.push(component::section(title));
        for stream in streams {
            let target = NameTarget {
                entity: entity_id,
                descriptor_type: stream.descriptor_type,
                index: stream.index,
                name_index: 0,
            };
            let mut lines = scramble_ui::column![
                name_line(triib, target, fl!("common-unnamed")),
                format_picker(triib, entity_id, &stream),
            ]
            .spacing(2);
            if let Some(state) = describe::stream_state(triib, model, &stream) {
                lines = lines.push(styled(state, Type::BodySmall).style(style::on_surface_variant));
            }
            if let Some(nanoseconds) = (!input)
                .then(|| model.max_transit_time(stream.index))
                .flatten()
            {
                lines = lines.push(
                    styled(
                        fl!(
                            "stream-max-transit-time",
                            time = describe::milliseconds(nanoseconds)
                        ),
                        Type::BodySmall,
                    )
                    .style(style::on_surface_variant),
                );
            }
            items.push(lines.into());
        }
    }
    items.extend(mapping_view::mapping_details(triib, entity_id, model));
    items
}

/// Each AVB interface with what it reports of gPTP and its counters.
fn interface_sections<'a>(model: &'a EntityModel) -> Vec<Element<'a, Message>> {
    let mut items = Vec::new();
    let interfaces: Vec<_> = model.avb_interfaces().collect();
    if !interfaces.is_empty() {
        items.push(component::section(fl!("avb-interfaces")));
        for interface in interfaces {
            items.extend(interface_details(model, &interface));
        }
    }
    items
}

/// An AVB interface: its address, and what it reports of gPTP, its path
/// to the grandmaster and its link.
fn interface_details<'a>(
    model: &EntityModel,
    interface: &AvbInterfaceDescriptor<'_>,
) -> Vec<Element<'a, Message>> {
    let name = model
        .name_of(DescriptorType::AVB_INTERFACE, interface.index)
        .map_or_else(|| fl!("avb-interface"), str::to_owned);
    let mut items = vec![
        styled(name, Type::TitleSmall).into(),
        property(fl!("common-mac-address"), interface.mac_address.to_string()),
        property(
            fl!("avb-interface-clock-identity"),
            interface.clock_identity.to_string(),
        ),
    ];
    if let Some(info) = model.avb_info(interface.index) {
        items.push(property(
            fl!("avb-interface-grandmaster"),
            fl!(
                "avb-interface-grandmaster-domain",
                grandmaster = info.gptp_grandmaster_id.to_string(),
                domain = info.gptp_domain_number
            ),
        ));
        items.push(property(
            fl!("avb-interface-peer-delay"),
            format!("{} ns", info.propagation_delay),
        ));
        let flags: Vec<String> = [
            (AvbInfoFlags::AS_CAPABLE, "asCapable"),
            (AvbInfoFlags::GPTP_ENABLED, "gPTP"),
            (AvbInfoFlags::SRP_ENABLED, "SRP"),
        ]
        .into_iter()
        .filter(|(flag, _)| info.flags.contains(*flag))
        .map(|(_, name)| name.to_owned())
        .collect();
        items.push(property(
            fl!("avb-interface-running"),
            if flags.is_empty() {
                fl!("avb-interface-none-reported")
            } else {
                crate::i18n::list(flags)
            },
        ));
    }
    if let Some(path) = model.as_path(interface.index) {
        let hops = path.len().saturating_sub(1);
        items.push(property(
            fl!("avb-interface-path"),
            match hops {
                0 => fl!("avb-interface-own-grandmaster"),
                hops => fl!("avb-interface-hops", count = hops),
            },
        ));
    }
    if let Some(counters) = model
        .counters(DescriptorType::AVB_INTERFACE, interface.index)
        .and_then(|counters| counters.avb_interface())
    {
        for (label, value) in [
            (fl!("avb-interface-link-up"), counters.link_up),
            (fl!("avb-interface-link-down"), counters.link_down),
            (
                fl!("avb-interface-grandmaster-changes"),
                counters.gptp_gm_changed,
            ),
            (fl!("avb-interface-frames-sent"), counters.frames_tx),
            (fl!("avb-interface-frames-received"), counters.frames_rx),
            (fl!("avb-interface-crc-errors"), counters.rx_crc_error),
        ] {
            if let Some(value) = value {
                items.push(property(label, value.to_string()));
            }
        }
    }
    items
}

/// The descriptor types the inspector's tree lists, in the standard's
/// order.
const TREE: [DescriptorType; 18] = [
    DescriptorType::ENTITY,
    DescriptorType::CONFIGURATION,
    DescriptorType::AUDIO_UNIT,
    DescriptorType::STREAM_INPUT,
    DescriptorType::STREAM_OUTPUT,
    DescriptorType::JACK_INPUT,
    DescriptorType::JACK_OUTPUT,
    DescriptorType::AVB_INTERFACE,
    DescriptorType::CLOCK_SOURCE,
    DescriptorType::MEMORY_OBJECT,
    DescriptorType::LOCALE,
    DescriptorType::STRINGS,
    DescriptorType::STREAM_PORT_INPUT,
    DescriptorType::STREAM_PORT_OUTPUT,
    DescriptorType::AUDIO_CLUSTER,
    DescriptorType::AUDIO_MAP,
    DescriptorType::CONTROL,
    DescriptorType::CLOCK_DOMAIN,
];

/// Every descriptor read from the entity, by type, each type opening to
/// its descriptors with their names and what matters most about them.
fn descriptor_tree<'a>(
    triib: &'a Triib,
    entity_id: EntityId,
    model: &'a EntityModel,
) -> Vec<Element<'a, Message>> {
    let mut items = Vec::new();
    for descriptor_type in TREE {
        let descriptors: Vec<(u16, &[u8])> = model.descriptors(descriptor_type).collect();
        if descriptors.is_empty() {
            continue;
        }
        let open = triib.tree_open.contains(&(entity_id, descriptor_type));
        let label = descriptor_type
            .name()
            .map_or_else(|| format!("{descriptor_type:?}"), describe::flag_name);
        let heading = scramble_ui::row![
            icon::icon(
                match (open, dir::mirrored()) {
                    (true, _) => Icon::ExpandMore,
                    (false, false) => Icon::ChevronRight,
                    (false, true) => Icon::ChevronLeft,
                },
                20
            ),
            styled(label, Type::BodyMedium)
                .width(Fill)
                .align_x(dir::text_start()),
            styled(descriptors.len().to_string(), Type::BodySmall).style(style::on_surface_variant),
        ]
        .spacing(8)
        .align_y(Center);
        items.push(
            button::custom(Kind::Row, heading)
                .width(Fill)
                .on_press(Message::TreeToggled(entity_id, descriptor_type))
                .into(),
        );
        if !open {
            continue;
        }
        for (index, bytes) in descriptors {
            let name = match descriptor_type {
                DescriptorType::ENTITY => model.entity_name().map(str::to_owned),
                _ => model.name_of(descriptor_type, index).map(str::to_owned),
            };
            let detail = descriptor_detail(model, descriptor_type, bytes);
            // Descriptors that have no name show what they hold instead.
            let (title, detail) = match (name, detail) {
                (Some(name), detail) => (name, detail),
                (None, Some(detail)) if !descriptor_type.has_object_name() => (detail, None),
                (None, detail) => (fl!("common-unnamed"), detail),
            };
            let mut lines = scramble_ui::column![styled(title, Type::BodyMedium)];
            if let Some(detail) = detail {
                lines =
                    lines.push(styled(detail, Type::BodySmall).style(style::on_surface_variant));
            }
            items.push(
                scramble_ui::row![
                    styled(index.to_string(), Type::BodySmall)
                        .style(style::on_surface_variant)
                        .width(Length::Fixed(28.0)),
                    lines,
                ]
                .spacing(8)
                .padding(dir::padding(0.0, 0.0, 0.0, 28.0))
                .into(),
            );
        }
    }
    items
}

/// What matters most about a descriptor, for the inspector's tree.
fn descriptor_detail(
    model: &EntityModel,
    descriptor_type: DescriptorType,
    bytes: &[u8],
) -> Option<String> {
    match descriptor_type {
        DescriptorType::ENTITY => EntityDescriptor::decode(bytes)
            .ok()
            .map(|entity| fl!("tree-firmware", version = entity.firmware_version)),
        DescriptorType::CONFIGURATION => {
            ConfigurationDescriptor::decode(bytes)
                .ok()
                .map(|configuration| {
                    let types = configuration.descriptor_counts().count();
                    fl!("tree-descriptor-types", count = types)
                })
        }
        DescriptorType::AUDIO_UNIT => AudioUnitDescriptor::decode(bytes)
            .ok()
            .map(|unit| Rate(unit.current_sampling_rate).to_string()),
        DescriptorType::STREAM_INPUT | DescriptorType::STREAM_OUTPUT => {
            StreamDescriptor::decode(bytes)
                .ok()
                .map(|stream| stream.current_format.to_string())
        }
        DescriptorType::AVB_INTERFACE => AvbInterfaceDescriptor::decode(bytes)
            .ok()
            .map(|interface| interface.mac_address.to_string()),
        DescriptorType::CLOCK_SOURCE => ClockSourceDescriptor::decode(bytes).ok().map(|source| {
            let kind = source
                .clock_source_type
                .name()
                .map_or_else(|| fl!("tree-clock"), describe::flag_name);
            match source.location_type {
                DescriptorType::ENTITY => kind,
                location => fl!(
                    "tree-clock-source-from",
                    kind = kind,
                    location = location
                        .name()
                        .map_or_else(|| format!("{location:?}"), describe::flag_name),
                    index = source.location_index
                ),
            }
        }),
        DescriptorType::CLOCK_DOMAIN => ClockDomainDescriptor::decode(bytes).ok().map(|domain| {
            let source = model
                .name_of(DescriptorType::CLOCK_SOURCE, domain.clock_source_index)
                .map_or_else(
                    || fl!("clock-source-numbered", index = domain.clock_source_index),
                    str::to_owned,
                );
            fl!("tree-clock-domain-using", source = source)
        }),
        DescriptorType::LOCALE => LocaleDescriptor::decode(bytes)
            .ok()
            .map(|locale| locale.locale_identifier.to_owned()),
        DescriptorType::STRINGS => StringsDescriptor::decode(bytes).ok().map(|strings| {
            let used: Vec<&str> = strings
                .strings
                .iter()
                .copied()
                .filter(|text| !text.is_empty())
                .collect();
            crate::i18n::list(used.into_iter().map(str::to_owned))
        }),
        DescriptorType::STREAM_PORT_INPUT | DescriptorType::STREAM_PORT_OUTPUT => {
            StreamPortDescriptor::decode(bytes).ok().map(|port| {
                crate::i18n::list([
                    fl!("tree-clusters", count = port.number_of_clusters),
                    fl!("tree-maps", count = port.number_of_maps),
                ])
            })
        }
        _ => None,
    }
}

/// What the entity advertises with ADP, in sections.
fn advertisement_details(entity: &DiscoveredEntity) -> Vec<Element<'_, Message>> {
    let adp = &entity.adp;
    let capabilities = adp.entity_capabilities;
    let indexed = |flag: EntityCapabilities, index: u16| {
        if capabilities.contains(flag) {
            index.to_string()
        } else {
            fl!("advert-not-advertised")
        }
    };
    let mut items = vec![
        component::section(fl!("advert-identity")),
        property(fl!("advert-entity-id"), adp.entity_id.to_string()),
        property(fl!("common-mac-address"), entity.mac.to_string()),
        property(fl!("advert-entity-model"), adp.entity_model_id.to_string()),
        component::section(fl!("advert-roles")),
        property(fl!("advert-roles"), describe::roles(adp)),
        property(
            fl!("advert-talker"),
            flags_text(adp.talker_capabilities.names()),
        ),
        property(
            fl!("advert-listener"),
            flags_text(adp.listener_capabilities.names()),
        ),
        component::section(fl!("advert-clock")),
        property(fl!("advert-btc"), describe::clock(adp)),
        property(
            fl!("advert-gptp-domain"),
            adp.gptp_domain_number.to_string(),
        ),
        property(fl!("advert-sr-classes"), describe::classes(adp)),
        component::section(fl!("advert-indexes")),
        property(
            fl!("entity-configuration"),
            indexed(
                EntityCapabilities::AEM_CONFIGURATION_INDEX_VALID,
                adp.current_configuration_index,
            ),
        ),
        property(
            fl!("advert-identify-control"),
            indexed(
                EntityCapabilities::AEM_IDENTIFY_CONTROL_INDEX_VALID,
                adp.identify_control_index,
            ),
        ),
        property(
            fl!("advert-avb-interface"),
            indexed(
                EntityCapabilities::AEM_INTERFACE_INDEX_VALID,
                adp.interface_index,
            ),
        ),
        component::section(fl!("advert-advertising")),
        property(
            fl!("advert-valid-time"),
            format!("{} s", adp.valid_seconds()),
        ),
        property(
            fl!("advert-available-index"),
            adp.available_index.to_string(),
        ),
        property(
            fl!("advert-association"),
            if capabilities.contains(EntityCapabilities::ASSOCIATION_ID_VALID) {
                format!("0x{:016x}", adp.association_id)
            } else {
                fl!("common-none")
            },
        ),
        component::section(fl!("advert-capabilities")),
    ];
    for name in capabilities.names() {
        items.push(styled(describe::flag_name(name), Type::BodyMedium).into());
    }
    items
}

/// A bit field's names in words, or "None".
fn flags_text<'a>(names: impl Iterator<Item = &'a str>) -> String {
    let words: Vec<String> = names
        .filter(|name| *name != "IMPLEMENTED")
        .map(describe::flag_name)
        .collect();
    if words.is_empty() {
        fl!("common-none")
    } else {
        crate::i18n::list(words)
    }
}

fn status_bar(triib: &Triib) -> Element<'_, Message> {
    iced::widget::responsive(move |size| status_bar_at(triib, size.width))
        .height(STATUS_BAR_HEIGHT)
        .into()
}

/// The status bar for `width` pixels, leaving out the interface and the
/// controller's ID when narrow.
fn status_bar_at(triib: &Triib, width: f32) -> Element<'_, Message> {
    let _fixed = dir::fixed();
    let full = width >= STATUS_FULL;
    let link = match triib.interface() {
        Some(interface) => {
            let mut parts = vec![
                interface.name.clone(),
                if interface.up {
                    fl!("interface-up")
                } else {
                    fl!("interface-link-down")
                },
            ];
            if let Some(clock) = interface.hardware_clock {
                parts.push(fl!(
                    "interface-hardware-clock-named",
                    clock = format!("ptp{clock}")
                ));
            }
            crate::i18n::list(parts)
        }
        None => fl!("state-no-interface"),
    };
    let label =
        |content: String| styled(content, Type::LabelMedium).style(style::on_surface_variant);
    let count = triib.entities.len();
    let entities = fl!("status-entities", count = count);
    let notice = triib.notice.clone();
    let state = match &triib.network_state {
        NetworkState::Idle => fl!("status-not-discovering"),
        NetworkState::Starting => fl!("state-starting"),
        NetworkState::Running { controller } if full => {
            fl!("status-discovering-as", controller = controller.to_string())
        }
        NetworkState::Running { .. } => fl!("status-discovering"),
        NetworkState::Failed(Failure {
            remedy: Some(Remedy::GetNpcap(_)),
            ..
        }) => fl!("state-npcap-needed"),
        NetworkState::Failed(failure) if failure.remedy.is_some() => {
            fl!("state-permission-needed")
        }
        NetworkState::Failed(_) => fl!("status-stopped"),
    };
    container(
        iced::widget::row![
            if full {
                Element::from(label(link))
            } else {
                Element::from(space())
            },
            space::horizontal(),
            match notice {
                Some(notice) => iced::widget::row![
                    styled(notice, Type::LabelMedium).style(style::error_text),
                    button::icon_button(Icon::Close).on_press(Message::DismissNotice),
                ]
                .spacing(4)
                .align_y(Center)
                .into(),
                None => Element::from(space()),
            },
            alarm_line(triib, full),
            label(entities),
            label(state),
        ]
        .spacing(if full { 24 } else { 16 })
        .align_y(Center),
    )
    .padding([0, 16])
    .height(STATUS_BAR_HEIGHT)
    .align_y(Center)
    .style(|theme: &Theme| {
        let scheme = Scheme::of(theme);
        container::Style {
            background: Some(scheme.chrome.into()),
            ..Default::default()
        }
    })
    .into()
}

/// The first alarm, with how many more there are, opening its entity's
/// diagnostics when pressed; nothing when there are none.
fn alarm_line(triib: &Triib, full: bool) -> Element<'_, Message> {
    let alarms = crate::lite_view::alarms(triib);
    let Some(first) = alarms.first() else {
        return space().into();
    };
    let mut text = if full {
        fl!(
            "status-alarm-of",
            entity = triib.entity_name_of(first.entity),
            alarm = first.text.clone()
        )
    } else {
        fl!("status-alarm")
    };
    if alarms.len() > 1 {
        text = fl!(
            "status-alarm-more",
            alarm = text,
            count = (alarms.len() - 1)
        );
    }
    iced::widget::mouse_area(
        iced::widget::row![
            icon::icon(Icon::Warning, 16).style(style::error_text),
            styled(text, Type::LabelMedium).style(style::error_text),
        ]
        .spacing(4)
        .align_y(Center),
    )
    .on_press(Message::AlarmOpened(first.entity))
    .interaction(iced::mouse::Interaction::Pointer)
    .into()
}

fn speed_text(megabits: u32) -> String {
    if megabits >= 1000 && megabits.is_multiple_of(1000) {
        format!("{} Gb/s", megabits / 1000)
    } else {
        format!("{megabits} Mb/s")
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    use std::collections::BTreeMap;

    use atdecc::aecp::AemPdu;
    use atdecc::controller::{Config, Controller};
    use atdecc::{EntityId, Instant, MacAddress};
    use iced::Size;

    use crate::settings::Settings;

    /// The entities and models a controller reads from the bench, replaying
    /// the capture of a live run.
    pub(crate) fn bench() -> (
        BTreeMap<EntityId, DiscoveredEntity>,
        BTreeMap<EntityId, EntityModel>,
    ) {
        bench_then(&[])
    }

    /// The bench, then `after`: ATDECC PDUs the controller hears once it
    /// has read the bench.
    pub(crate) fn bench_then(
        after: &[Vec<u8>],
    ) -> (
        BTreeMap<EntityId, DiscoveredEntity>,
        BTreeMap<EntityId, EntityModel>,
    ) {
        const CAPTURE: &[u8] = include_bytes!("../../atdecc/tests/captures/bench-enumeration.pcap");
        const HOST: [u8; 6] = [0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b];
        // Classic pcap, in microseconds or nanoseconds.
        let unit = match u32::from_le_bytes(CAPTURE[..4].try_into().unwrap()) {
            0xa1b2_3c4d => 1,
            _ => 1000,
        };
        let mut records = Vec::new();
        let mut at = 24;
        while at + 16 <= CAPTURE.len() {
            let word = |offset: usize| {
                u32::from_le_bytes(CAPTURE[at + offset..at + offset + 4].try_into().unwrap())
            };
            let time = u64::from(word(0)) * 1_000_000_000 + u64::from(word(4)) * unit;
            let length = word(8) as usize;
            records.push((time, &CAPTURE[at + 16..at + 16 + length]));
            at += 16 + length;
        }
        let ours = |frame: &[u8]| frame[6..12] == HOST;
        // The controller numbers its commands as the one that made the
        // capture did, so the answers match them.
        let first_sequence_id = records
            .iter()
            .filter(|(_, frame)| ours(frame) && frame[14] == 0xfb)
            .find_map(|(_, frame)| AemPdu::decode(&frame[14..]).ok())
            .map_or(0, |aem| aem.header.sequence_id);
        let mut config = Config::new(EntityId(0x9c6b_00ff_fe30_9a2b));
        config.register_unsolicited = false;
        config.network_info = false;
        config.media_clock_info = false;
        config.read_counters = false;
        config.read_transit_times = false;
        config.lite_status = false;
        config.wireless_status = false;
        config.first_sequence_id = first_sequence_id;
        let mut controller = Controller::new(config);
        let mut buffer = [0; 1500];
        controller.discover(None);
        let start = records[0].0;
        for &(time, frame) in &records {
            let now = Instant::from_nanos(time.saturating_sub(start));
            while controller.poll_transmit(&mut buffer).unwrap().is_some() {}
            while let Some(deadline) = controller
                .poll_timeout()
                .filter(|deadline| *deadline <= now)
            {
                controller.handle_timeout(deadline);
            }
            let source = MacAddress(frame[6..12].try_into().unwrap());
            if ours(frame) || u16::from_be_bytes([frame[12], frame[13]]) != 0x22f0 {
                continue;
            }
            let _ = controller.handle_frame(now, source, &frame[14..]);
            controller.handle_timeout(now);
        }
        let end = Instant::from_nanos(records[records.len() - 1].0 - start);
        for pdu in after {
            let _ = controller.handle_frame(end, MacAddress([0; 6]), pdu);
        }
        let entities: BTreeMap<EntityId, DiscoveredEntity> = controller
            .entities()
            .map(|entity| (entity.entity_id(), *entity))
            .collect();
        let models = entities
            .keys()
            .filter_map(|&entity_id| Some((entity_id, controller.model(entity_id)?.clone())))
            .collect();
        (entities, models)
    }

    /// What the bench would report running AVB Lite: the wired ESP on
    /// VLAN 2 with `offset` nanoseconds from the grandmaster and `egress`
    /// kb/s sent of a gigabit, and the Mac mini declaring a stream over
    /// CVU SRP.
    pub(crate) fn lite_frames(offset: i32, egress: u32) -> Vec<Vec<u8>> {
        use atdecc::aecp::{AecpHeader, AecpMessageType, VendorUniquePdu};
        use atdecc::id::ClockIdentity;
        use atdecc::lite::{
            CVU_PROTOCOL_ID, FallbackReason, LiteCommandType, LiteFlags, LiteStatus, PtpProfile,
            STATUS_PROTOCOL_ID,
        };

        let status = LiteStatus {
            interface: 0,
            flags: LiteFlags::CAPABLE
                | LiteFlags::ACTIVE
                | LiteFlags::OFFSET_VALID
                | LiteFlags::EGRESS_VALID,
            fallback_reason: FallbackReason::MULTIPLE_RESPONDERS,
            ptp_profile: PtpProfile::AVB_LITE_PTP,
            ptp_domain: 0,
            media_vlan_id: 2,
            unicast_fanout_limit: 2,
            link_speed: 1000,
            committed_egress: egress,
            grandmaster: ClockIdentity(0x9c6b_00ff_fe30_9a2b),
            offset_from_grandmaster: offset,
        };
        // An unsolicited response, as an entity sends when its status
        // changes.
        let mut payload = (0x8000 | LiteCommandType::GET_LITE_STATUS.0)
            .to_be_bytes()
            .to_vec();
        payload.extend_from_slice(&status.to_bytes());
        let notification = VendorUniquePdu {
            header: AecpHeader {
                message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
                status: 0,
                target_entity_id: WIRED_ESP,
                controller_entity_id: EntityId(0x9c6b_00ff_fe30_9a2b),
                sequence_id: 0,
            },
            protocol_id: STATUS_PROTOCOL_ID,
            payload: &payload,
        };

        // Talker Advertise for the Mac mini's stream output 0.
        let mut message = vec![avb_mrp::msrp::attribute::TALKER_ADVERTISE, 25, 0, 0];
        message.extend_from_slice(&[0x00, 0x01]);
        message.extend_from_slice(&0xd111_e597_f544_8000u64.to_be_bytes());
        message.extend_from_slice(&[0x91, 0xe0, 0xf0, 0x00, 0x6a, 0x20]);
        message.extend_from_slice(&2u16.to_be_bytes());
        message.extend_from_slice(&224u16.to_be_bytes());
        message.extend_from_slice(&1u16.to_be_bytes());
        message.push(0xb0);
        message.extend_from_slice(&500_000u32.to_be_bytes());
        message.push(36); // JoinIn
        message.extend_from_slice(&[0, 0]);
        let list = (message.len() - 4) as u16;
        message[2..4].copy_from_slice(&list.to_be_bytes());
        let mut cvu_payload = vec![avb_mrp::msrp::attribute::TALKER_ADVERTISE];
        cvu_payload.extend_from_slice(&message);
        let declaration = VendorUniquePdu {
            header: AecpHeader {
                message_type: AecpMessageType::VENDOR_UNIQUE_COMMAND,
                status: 0,
                target_entity_id: EntityId(0),
                controller_entity_id: MAC_MINI,
                sequence_id: 0,
            },
            protocol_id: CVU_PROTOCOL_ID,
            payload: &cvu_payload,
        };

        [notification, declaration]
            .iter()
            .map(|pdu| {
                let mut out = [0; 128];
                let length = pdu.encode(&mut out).unwrap();
                out[..length].to_vec()
            })
            .collect()
    }

    /// What the Wi-Fi ESP would report as an AVB Wireless station, its time
    /// as `flags` say, unsolicited.
    pub(crate) fn wireless_frames(flags: atdecc::wireless::WirelessFlags) -> Vec<Vec<u8>> {
        use atdecc::aecp::{AecpHeader, AecpMessageType, VendorUniquePdu};
        use atdecc::id::ClockIdentity;
        use atdecc::wireless::{WIRELESS_PROTOCOL_ID, WirelessCommandType};

        let status = crate::topology::test_station(ClockIdentity(0x30ed_a0ff_fe11_2233), flags);
        let mut payload = (0x8000 | WirelessCommandType::GET_WIRELESS_STATUS.0)
            .to_be_bytes()
            .to_vec();
        payload.extend_from_slice(&status.to_bytes());
        let notification = VendorUniquePdu {
            header: AecpHeader {
                message_type: AecpMessageType::VENDOR_UNIQUE_RESPONSE,
                status: 0,
                target_entity_id: WIFI_ESP,
                controller_entity_id: EntityId(0x9c6b_00ff_fe30_9a2b),
                sequence_id: 0,
            },
            protocol_id: WIRELESS_PROTOCOL_ID,
            payload: &payload,
        };
        let mut out = [0; 160];
        let length = notification.encode(&mut out).unwrap();
        vec![out[..length].to_vec()]
    }

    /// A bench entity as it would read with its descriptors changed by
    /// `patch`: read again by a controller from those descriptors, its
    /// stream input port answering GET_AUDIO_MAP with `mappings` (stream
    /// channel, cluster channel) and its stream inputs bound as `bound`
    /// says (input, talker, output) and flowing.
    fn reread(
        entities: &BTreeMap<EntityId, DiscoveredEntity>,
        models: &BTreeMap<EntityId, EntityModel>,
        entity_id: EntityId,
        patch: impl FnOnce(&mut BTreeMap<(u16, u16), Vec<u8>>),
        mappings: &[(u16, u16)],
        bound: &[(u16, EntityId, u16)],
        counting: bool,
    ) -> EntityModel {
        use atdecc::aecp::{AecpHeader, AecpMessageType, AemStatus};
        use atdecc::aem::StreamInfoFlags;
        use atdecc::{AcmpMessageType, Acmpdu, AemCommandType, EntityCapabilities};

        let mut descriptors: BTreeMap<(u16, u16), Vec<u8>> = models[&entity_id]
            .all_descriptors()
            .map(|(descriptor_type, index, bytes)| ((descriptor_type.0, index), bytes.to_vec()))
            .collect();
        patch(&mut descriptors);
        let bound_to = |input: u16| {
            bound
                .iter()
                .find(|(place, _, _)| *place == input)
                .map(|&(_, talker, output)| (talker, output))
        };

        let mut config = Config::new(EntityId(0x9c6b_00ff_fe30_9a2b));
        config.register_unsolicited = false;
        config.network_info = false;
        let mut controller = Controller::new(config);
        let entity = entities[&entity_id];
        let mut adpdu = entity.adp;
        adpdu.entity_capabilities = EntityCapabilities::AEM_SUPPORTED;
        controller.handle_adpdu(Instant::from_millis(0), entity.mac, &adpdu);
        let mut buffer = [0; 1500];
        let mut out = [0; 1500];
        for step in 1..400 {
            let now = Instant::from_millis(step * 100);
            while let Some(transmit) = controller.poll_transmit(&mut buffer).unwrap() {
                let frame = &buffer[..transmit.length];
                if let Ok(command) = Acmpdu::decode(frame) {
                    if command.message_type != AcmpMessageType::GET_RX_STATE_COMMAND
                        || command.listener_entity_id != entity_id
                    {
                        continue;
                    }
                    let talker = bound_to(command.listener_unique_id);
                    let response = Acmpdu {
                        message_type: command.message_type.response(),
                        talker_entity_id: talker.map_or(EntityId(0), |(talker, _)| talker),
                        talker_unique_id: talker.map_or(0, |(_, output)| output),
                        connection_count: u16::from(talker.is_some()),
                        ..command
                    };
                    let length = response.encode(&mut out).unwrap();
                    controller
                        .handle_frame(now, entity.mac, &out[..length])
                        .unwrap();
                    continue;
                }
                let Ok(command) = AemPdu::decode(frame) else {
                    continue;
                };
                let mut payload = command.payload.to_vec();
                let key = |at: usize, payload: &[u8]| {
                    (
                        u16::from_be_bytes([payload[at], payload[at + 1]]),
                        u16::from_be_bytes([payload[at + 2], payload[at + 3]]),
                    )
                };
                let status = match command.command_type {
                    AemCommandType::READ_DESCRIPTOR => match descriptors.get(&key(4, &payload)) {
                        Some(bytes) => {
                            payload.truncate(4);
                            payload.extend_from_slice(bytes);
                            AemStatus::SUCCESS
                        }
                        None => AemStatus::NO_SUCH_DESCRIPTOR,
                    },
                    AemCommandType::GET_AUDIO_MAP if !mappings.is_empty() => {
                        payload.truncate(6);
                        payload.extend_from_slice(&1u16.to_be_bytes());
                        payload.extend_from_slice(&(mappings.len() as u16).to_be_bytes());
                        payload.extend_from_slice(&[0, 0]);
                        for &(stream_channel, cluster_channel) in mappings {
                            for field in [0, stream_channel, 0, cluster_channel] {
                                payload.extend_from_slice(&u16::to_be_bytes(field));
                            }
                        }
                        AemStatus::SUCCESS
                    }
                    AemCommandType::GET_COUNTERS if counting => {
                        let (descriptor_type, _) = key(0, &payload);
                        // Locks, a lost lock, interruptions, late frames and
                        // frames counted, as an entity a while into a show.
                        let (valid, counts): (u32, &[(usize, u32)]) =
                            match DescriptorType(descriptor_type) {
                                DescriptorType::CLOCK_DOMAIN => (0x3, &[(0, 2), (1, 1)]),
                                DescriptorType::STREAM_INPUT => {
                                    (0xf3f, &[(0, 3), (1, 2), (2, 2), (9, 12), (11, 1_204_331)])
                                }
                                DescriptorType::STREAM_OUTPUT => (0x9b, &[(0, 1), (7, 987_654)]),
                                _ => (0, &[]),
                            };
                        payload.resize(136, 0);
                        payload[4..8].copy_from_slice(&valid.to_be_bytes());
                        for &(position, count) in counts {
                            let at = 8 + 4 * position;
                            payload[at..at + 4].copy_from_slice(&count.to_be_bytes());
                        }
                        AemStatus::SUCCESS
                    }
                    AemCommandType::GET_STREAM_INFO => {
                        let (descriptor_type, index) = key(0, &payload);
                        let mut flags = StreamInfoFlags::STREAM_FORMAT_VALID;
                        if descriptor_type == DescriptorType::STREAM_INPUT.0
                            && bound_to(index).is_some()
                        {
                            flags |= StreamInfoFlags::CONNECTED
                                | StreamInfoFlags::STREAM_ID_VALID
                                | StreamInfoFlags::MSRP_ACC_LAT_VALID;
                        }
                        payload.resize(48, 0);
                        payload[4..8].copy_from_slice(&flags.0.to_be_bytes());
                        AemStatus::SUCCESS
                    }
                    _ => AemStatus::NOT_IMPLEMENTED,
                };
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
                controller
                    .handle_frame(now, entity.mac, &out[..length])
                    .unwrap();
            }
            controller.handle_timeout(now);
            let model = controller.model(entity_id);
            if !controller.busy(entity_id)
                && model.is_some_and(|model| model.state == EnumerationState::Complete)
            {
                break;
            }
        }
        controller.model(entity_id).unwrap().clone()
    }

    const MAC_MINI: EntityId = EntityId(0xd111_e597_f544_8000);
    const WIRED_ESP: EntityId = EntityId(0xe8f6_0ae0_9220_0000);
    const WIFI_ESP: EntityId = EntityId(0xfc01_2cfd_fe80_0000);

    /// The bench's Mac mini as if its stream port input took dynamic
    /// mappings, with its first two stream channels crossed and the fifth
    /// mapped.
    fn dynamic_mac_mini(
        entities: &BTreeMap<EntityId, DiscoveredEntity>,
        models: &BTreeMap<EntityId, EntityModel>,
    ) -> EntityModel {
        let model = reread(
            entities,
            models,
            MAC_MINI,
            |descriptors| {
                let port = descriptors
                    .get_mut(&(DescriptorType::STREAM_PORT_INPUT.0, 0))
                    .unwrap();
                port[16..18].copy_from_slice(&0u16.to_be_bytes());
            },
            &[(0, 1), (1, 0), (4, 4)],
            &[],
            false,
        );
        assert!(
            model
                .dynamic_mappings(DescriptorType::STREAM_PORT_INPUT, 0)
                .is_some_and(|read| read.count() == 3),
            "the mappings read"
        );
        model
    }

    /// The bench with a media clock chain: the wired endpoint clocked from
    /// the Mac mini's CRF output through its CRF input, and the Wi-Fi
    /// endpoint clocked from a CRF input bound to nothing.
    fn clocked_bench(
        entities: &BTreeMap<EntityId, DiscoveredEntity>,
        models: &mut BTreeMap<EntityId, EntityModel>,
    ) {
        // Clock source 1 is the endpoints' CRF input, stream input 1.
        let from_crf_input = |descriptors: &mut BTreeMap<(u16, u16), Vec<u8>>| {
            let domain = descriptors
                .get_mut(&(DescriptorType::CLOCK_DOMAIN.0, 0))
                .unwrap();
            domain[70..72].copy_from_slice(&1u16.to_be_bytes());
        };
        let wired = reread(
            entities,
            models,
            WIRED_ESP,
            from_crf_input,
            &[],
            &[(1, MAC_MINI, 1)],
            false,
        );
        let wifi = reread(entities, models, WIFI_ESP, from_crf_input, &[], &[], false);
        models.insert(WIRED_ESP, wired);
        models.insert(WIFI_ESP, wifi);
    }

    /// Writes pictures of the whole window on the bench, at a phone's width
    /// and a desktop's, to the PNG files the variable `WINDOW_PICTURE`
    /// names, with what each shows added.
    #[test]
    #[ignore = "writes pictures to look at"]
    fn pictures() {
        let Ok(path) = std::env::var("WINDOW_PICTURE") else {
            return;
        };
        // Pictures in another language, such as WINDOW_LANGUAGE=de.
        let language = std::env::var("WINDOW_LANGUAGE").ok();
        crate::i18n::set_language(language.as_deref());
        // Only the pictures whose names hold WINDOW_ONLY, such as "log".
        let only = std::env::var("WINDOW_ONLY").ok();
        let theme = scramble_ui::scheme::theme("triib".to_owned(), crate::app::TRIIB_SEED, true);
        // The inspector in place, not sliding in.
        scramble_ui::motion::set_reduced(true);
        let (entities, models) = bench();
        assert_eq!(models.len(), 3, "the bench's models");
        assert_eq!(entities.len(), 3, "the bench's entities");
        let interface = Interface {
            name: "enp6s0".to_owned(),
            mac: MacAddress(HOST_MAC),
            up: true,
            speed: Some(2500),
            physical: true,
            wireless: false,
            hardware_clock: Some(0),
        };
        let phone = Size::new(360.0, 760.0);
        let desktop = Size::new(1280.0, 800.0);
        for (suffix, view, inspector, size) in [
            ("matrix-phone", View::Matrix, false, phone),
            ("entities-phone", View::Entities, false, phone),
            ("network-phone", View::Network, false, phone),
            ("inspector-phone", View::Matrix, true, phone),
            ("matrix-desktop", View::Matrix, true, desktop),
            ("matrix-hover-own", View::Matrix, false, desktop),
            ("matrix-formats", View::Matrix, false, desktop),
            ("entities-desktop", View::Entities, false, desktop),
            (
                "entities-wide",
                View::Entities,
                false,
                Size::new(2200.0, 420.0),
            ),
            (
                "entities-narrow",
                View::Entities,
                false,
                Size::new(1280.0, 420.0),
            ),
            (
                "entities-menu",
                View::Entities,
                false,
                Size::new(2200.0, 760.0),
            ),
            ("inspector-editing", View::Entities, true, desktop),
            (
                "inspector-tree",
                View::Entities,
                true,
                Size::new(1280.0, 3200.0),
            ),
            ("matrix-filtered", View::Matrix, false, desktop),
            // Text typed in the search field.
            ("matrix-searching", View::Matrix, false, desktop),
            (
                "inspector-diagnostics",
                View::Entities,
                true,
                Size::new(1280.0, 2400.0),
            ),
            (
                "media-clock-bench",
                View::Entities,
                false,
                Size::new(1280.0, 500.0),
            ),
            (
                "inspector-mappings",
                View::Entities,
                true,
                Size::new(1280.0, 2000.0),
            ),
            ("inspector-controls", View::Entities, true, desktop),
            (
                "inspector-lite",
                View::Entities,
                true,
                Size::new(1280.0, 1400.0),
            ),
            (
                "inspector-lite-alarms",
                View::Entities,
                true,
                Size::new(1280.0, 1400.0),
            ),
            (
                "inspector-wireless",
                View::Entities,
                true,
                Size::new(1280.0, 1400.0),
            ),
            ("log-desktop", View::Entities, true, desktop),
            ("log-matrix", View::Matrix, false, desktop),
            // A small window with the inspector and the log open.
            (
                "log-matrix-cramped",
                View::Matrix,
                true,
                Size::new(960.0, 620.0),
            ),
            ("log-phone", View::Network, false, phone),
            ("presets-desktop", View::Matrix, false, desktop),
            (
                "inspector-mappings-phone",
                View::Entities,
                true,
                Size::new(360.0, 2400.0),
            ),
            ("settings-general", View::Matrix, false, desktop),
            ("settings-appearance", View::Matrix, false, desktop),
            ("settings-phone", View::Matrix, false, phone),
            (
                "entities-medium",
                View::Entities,
                false,
                Size::new(760.0, 600.0),
            ),
        ] {
            if only
                .as_ref()
                .is_some_and(|only| !suffix.contains(only.as_str()))
            {
                continue;
            }
            let file = format!("{path}-{suffix}.png");
            for renderer in ["tiny-skia", "wgpu"] {
                let _ = std::fs::remove_file(format!("{path}-{suffix}-{renderer}.png"));
            }
            let settings = Settings {
                interface: Some(interface.name.clone()),
                view,
                inspector,
                ..Settings::default()
            };
            let mut models = if suffix.contains("lite") {
                bench_then(&lite_frames(-72_400, 800_000)).1
            } else if suffix.contains("wireless") {
                bench_then(&wireless_frames(atdecc::wireless::WirelessFlags::HOLDOVER)).1
            } else {
                models.clone()
            };
            if suffix.contains("mappings") {
                let model = dynamic_mac_mini(&entities, &models);
                models.insert(MAC_MINI, model);
            }
            if suffix.contains("media-clock") {
                clocked_bench(&entities, &mut models);
            }
            if suffix.contains("diagnostics") {
                let model = reread(
                    &entities,
                    &models,
                    WIRED_ESP,
                    |_| {},
                    &[],
                    &[(0, MAC_MINI, 0)],
                    true,
                );
                models.insert(WIRED_ESP, model);
            }
            let mut triib = Triib::sample(settings, interface.clone(), entities.clone(), models);
            triib.selected = if ["diagnostics", "controls", "lite"]
                .iter()
                .any(|kind| suffix.contains(kind))
            {
                Some(WIRED_ESP)
            } else if suffix.contains("wireless") {
                Some(WIFI_ESP)
            } else {
                entities.keys().next().copied()
            };
            triib.inspector_tab = if suffix.contains("tree") {
                InspectorTab::Descriptors
            } else if suffix.contains("diagnostics") || suffix.contains("alarms") {
                InspectorTab::Diagnostics
            } else if suffix.contains("mappings") {
                InspectorTab::Streams
            } else if suffix.contains("controls") {
                InspectorTab::Controls
            } else {
                InspectorTab::Entity
            };
            if suffix.contains("filtered") {
                triib.settings.matrix_connectable_only = true;
            }
            if suffix.contains("searching") {
                triib.search = "Mac mini".to_owned();
            }
            if suffix.starts_with("presets") {
                triib.presets = vec![
                    crate::presets::capture(&triib, "Rehearsal"),
                    crate::presets::capture(&triib, "Sunday service"),
                ];
                triib.presets_open = true;
                triib.preset_name = "Matinee".to_owned();
                triib.preset_report = Some("Recalling \"Rehearsal\": 3 changes.".to_owned());
            }
            if suffix.starts_with("log") {
                let frames = crate::log_view::tests::bench_frames(&triib);
                triib.log.add(frames);
                triib.settings.log = true;
            }
            if suffix.contains("narrow") {
                // Columns dragged narrower than their text.
                triib.settings.entity_name_width = Some(130.0);
                triib
                    .settings
                    .entity_column_widths
                    .insert(crate::settings::EntityField::Product, 100.0);
                triib
                    .settings
                    .entity_column_widths
                    .insert(crate::settings::EntityField::Milan, 70.0);
            }
            if suffix.contains("hover-own") {
                // The Mac mini's audio output against its own audio input.
                triib.hover = crate::matrix::Hover {
                    column: Some(crate::matrix::Line::Stream(MAC_MINI, 0)),
                    row: Some(crate::matrix::Line::Stream(MAC_MINI, 0)),
                };
            }
            if suffix.contains("formats") {
                // The menu a cell whose input must change opens.
                triib.format_menu = crate::matrix::first_format_menu(&triib);
                assert!(
                    triib.format_menu.is_some(),
                    "a cell whose input must change"
                );
            }
            if suffix.starts_with("settings") {
                triib.settings_open = true;
                if suffix.contains("appearance") || suffix.contains("phone") {
                    triib.settings_tab = crate::settings_view::SettingsTab::Appearance;
                    triib.settings.system_accent = false;
                }
            }
            if suffix.contains("tree") {
                let entity = entities.keys().next().copied().unwrap();
                for descriptor_type in [
                    DescriptorType::CONFIGURATION,
                    DescriptorType::CLOCK_SOURCE,
                    DescriptorType::CLOCK_DOMAIN,
                    DescriptorType::STRINGS,
                    DescriptorType::STREAM_PORT_INPUT,
                ] {
                    triib.tree_open.insert((entity, descriptor_type));
                }
            }
            if suffix.contains("editing") {
                let target = NameTarget {
                    entity: entities.keys().next().copied().unwrap(),
                    descriptor_type: DescriptorType::ENTITY,
                    index: 0,
                    name_index: 0,
                };
                triib.editing = Some((target, "Studio Mac".to_owned()));
            }
            let iced_settings = iced::Settings {
                fonts: scramble_ui::font::files().collect(),
                default_font: scramble_ui::font::TEXT,
                antialiasing: true,
                ..iced::Settings::default()
            };
            let mut simulator =
                iced_test::Simulator::with_size(iced_settings, size, window(&triib));
            if suffix.contains("menu") {
                // The Group column's menu, open.
                simulator
                    .click(crate::fl!("entity-group").as_str())
                    .expect("the Group heading");
            }
            let snapshot = simulator.snapshot(&theme).expect("draws");
            assert!(snapshot.matches_image(&file).expect("writes"));
        }
    }

    const HOST_MAC: [u8; 6] = [0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b];

    #[test]
    fn rates_read_naturally() {
        assert_eq!(Rate(SamplingRate(48_000)).to_string(), "48 kHz");
        assert_eq!(Rate(SamplingRate(44_100)).to_string(), "44.1 kHz");
        assert_eq!(
            Rate(SamplingRate(1 << 29 | 48_000)).to_string(),
            "48 kHz × 1/1.001"
        );
    }

    /// What pressing and releasing at each point sends from a desktop
    /// window showing `view` on the bench, the Mac mini selected.
    fn presses(view: View, points: &[(f32, f32)]) -> Vec<Vec<Message>> {
        use iced::mouse::{Button, Event as Mouse};
        let (entities, models) = bench();
        let interface = Interface {
            name: "enp6s0".to_owned(),
            mac: MacAddress(HOST_MAC),
            up: true,
            speed: Some(2500),
            physical: true,
            wireless: false,
            hardware_clock: Some(0),
        };
        let settings = Settings {
            interface: Some(interface.name.clone()),
            view,
            inspector: false,
            ..Settings::default()
        };
        let mut triib = Triib::sample(settings, interface, entities, models);
        triib.selected = Some(MAC_MINI);
        points
            .iter()
            .map(|&(x, y)| {
                let mut simulator = iced_test::Simulator::with_size(
                    iced::Settings {
                        fonts: scramble_ui::font::files().collect(),
                        default_font: scramble_ui::font::TEXT,
                        ..iced::Settings::default()
                    },
                    Size::new(1280.0, 800.0),
                    window(&triib),
                );
                simulator.point_at(iced::Point::new(x, y));
                let _ = simulator.simulate([
                    iced::Event::Mouse(Mouse::ButtonPressed(Button::Left)),
                    iced::Event::Mouse(Mouse::ButtonReleased(Button::Left)),
                ]);
                simulator.into_messages().collect()
            })
            .collect()
    }

    fn clears(messages: &[Message]) -> bool {
        messages
            .iter()
            .any(|message| matches!(message, Message::SelectionCleared))
    }

    #[test]
    fn a_press_on_a_views_empty_space_clears_the_selection() {
        // Below the entity list's rows; on a name; on a plain cell. The
        // rows start under the bar that adds this computer's endpoints.
        let list = presses(
            View::Entities,
            &[(640.0, 600.0), (90.0, 197.0), (380.0, 197.0)],
        );
        assert!(clears(&list[0]), "{:?}", list[0]);
        assert!(
            matches!(list[1][..], [Message::EntitySelected(_)]),
            "{:?}",
            list[1]
        );
        assert!(list[2].is_empty(), "{:?}", list[2]);
        // Beside the matrix; on one of its cells.
        let matrix = presses(View::Matrix, &[(1200.0, 400.0), (420.0, 480.0)]);
        assert!(clears(&matrix[0]), "{:?}", matrix[0]);
        assert!(!clears(&matrix[1]), "{:?}", matrix[1]);
        // The network map's empty corner.
        let network = presses(View::Network, &[(60.0, 700.0)]);
        assert!(clears(&network[0]), "{:?}", network[0]);
    }

    /// The bench with the log open under `view` and some frames in it,
    /// and the wired ESP in the inspector when it is open.
    fn logging(view: View, inspector: bool) -> Triib {
        let (entities, models) = bench();
        let interface = Interface {
            name: "enp6s0".to_owned(),
            mac: MacAddress(HOST_MAC),
            up: true,
            speed: Some(2500),
            physical: true,
            wireless: false,
            hardware_clock: Some(0),
        };
        let settings = Settings {
            interface: Some(interface.name.clone()),
            view,
            inspector,
            log: true,
            ..Settings::default()
        };
        let mut triib = Triib::sample(settings, interface, entities, models);
        triib.selected = Some(WIRED_ESP);
        let frames = crate::log_view::tests::bench_frames(&triib);
        triib.log.add(frames);
        triib
    }

    fn simulate(triib: &Triib, size: Size) -> iced_test::Simulator<'_, Message> {
        iced_test::Simulator::with_size(
            iced::Settings {
                fonts: scramble_ui::font::files().collect(),
                default_font: scramble_ui::font::TEXT,
                ..iced::Settings::default()
            },
            size,
            window(triib),
        )
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn this_computers_endpoints_are_added_and_shown_as_its_own() {
        use triib_endpointd::config::Kind as EndpointKind;
        use triib_endpointd::status::{DaemonStatus, EndpointStatus};

        let mut triib = logging(View::Entities, true);
        triib.settings.log = false;
        let size = Size::new(1280.0, 800.0);
        // On an interface with a hardware clock the bar offers both kinds.
        {
            let mut simulator = simulate(&triib, size);
            assert!(simulator.find("Add talker").is_ok());
            assert!(simulator.find("Add listener").is_ok());
        }
        let messages: Vec<Message> = {
            let mut simulator = simulate(&triib, size);
            simulator.click("Add listener").expect("the button");
            simulator.into_messages().collect()
        };
        assert!(
            messages
                .iter()
                .any(|message| matches!(message, Message::HostAdded(EndpointKind::Listener))),
            "{messages:?}"
        );
        // The wired ESP, as if this computer ran it: the inspector shows
        // its state and audio, and a button that removes it.
        triib.endpoints = Some(DaemonStatus {
            pid: 1,
            started: 0,
            interface: "enp6s0".into(),
            gptp: "grandmaster 0x0001f2fffeff3b14, asCapable".into(),
            foreign_mrp: false,
            endpoints: vec![EndpointStatus {
                entity_id: WIRED_ESP.to_string(),
                name: "Host listener 1".into(),
                kind: EndpointKind::Listener,
                state: "listening".into(),
                audio: "discard".into(),
                channels: 8,
                streams: Vec::new(),
            }],
        });
        triib.inspector_tab = InspectorTab::Entity;
        let mut simulator = simulate(&triib, size);
        assert!(simulator.find("Listening").is_ok());
        assert!(simulator.find("Nowhere").is_ok());
        assert!(simulator.find("Channels").is_ok());
        simulator
            .click("Remove from this computer")
            .expect("the button");
        let messages: Vec<Message> = simulator.into_messages().collect();
        assert!(
            messages
                .iter()
                .any(|message| matches!(message, Message::HostRemoved(id) if *id == WIRED_ESP)),
            "{messages:?}"
        );
        // As a device, each of its streams shows its own state, audio and
        // channels, with buttons that add and remove streams.
        if let Some(status) = &mut triib.endpoints {
            use triib_endpointd::config::Direction;
            use triib_endpointd::status::StreamStatus;
            let endpoint = &mut status.endpoints[0];
            endpoint.kind = EndpointKind::Device;
            endpoint.streams = vec![
                StreamStatus {
                    direction: Direction::Out,
                    name: "Out 1".into(),
                    channels: 8,
                    audio: "virtual".into(),
                    state: "streaming".into(),
                },
                StreamStatus {
                    direction: Direction::In,
                    name: "In 1".into(),
                    channels: 2,
                    audio: "discard".into(),
                    state: "unbound".into(),
                },
            ];
        }
        {
            let mut simulator = simulate(&triib, size);
            assert!(simulator.find("Out 1").is_ok());
            assert!(simulator.find("In 1").is_ok());
            assert!(simulator.find("Virtual device").is_ok());
            assert!(simulator.find("Nowhere").is_ok());
            assert!(simulator.find("Add output stream").is_ok());
            assert!(simulator.find("Add device").is_ok());
            simulator.click("Add input stream").expect("the button");
            let messages: Vec<Message> = simulator.into_messages().collect();
            assert!(
                messages.iter().any(|message| matches!(
                    message,
                    Message::HostStream(
                        _,
                        crate::host::StreamChange::Add(triib_endpointd::config::Direction::In)
                    )
                )),
                "{messages:?}"
            );
        }
        // The list can show only this computer's: the wired ESP stays,
        // the Mac mini goes.
        assert!(simulate(&triib, size).find("Mac mini").is_ok());
        triib.settings.host_entities_only = true;
        assert!(simulate(&triib, size).find("Mac mini").is_err());
        let _ = triib.update(Message::HostEntitiesOnlyToggled);
        assert!(!triib.settings.host_entities_only);
        // Without a hardware clock, the bar says why instead.
        triib.interfaces[0].hardware_clock = None;
        let mut simulator = simulate(&triib, size);
        assert!(
            simulator
                .find("This computer's own talkers and listeners need a wired interface with a PTP hardware clock")
                .is_ok()
        );
    }

    #[test]
    fn the_log_shows_under_every_view_beside_the_inspector() {
        let desktop = Size::new(1280.0, 800.0);
        let phone = Size::new(360.0, 760.0);
        for view in [View::Matrix, View::Network, View::Entities] {
            for (size, inspector) in [(desktop, true), (desktop, false), (phone, true)] {
                let triib = logging(view, inspector);
                let mut simulator = simulate(&triib, size);
                let case = format!("{view:?} at {} wide", size.width);
                assert!(simulator.find("Warnings").is_ok(), "{case}");
                assert!(
                    simulator.find("Read descriptor, stream input 1").is_ok(),
                    "{case}"
                );
                assert_eq!(simulator.find("Serial number").is_ok(), inspector, "{case}");
            }
        }
        let mut closed = logging(View::Matrix, true);
        let _ = closed.update(Message::LogToggled);
        assert!(simulate(&closed, desktop).find("Warnings").is_err());
    }

    #[test]
    fn dragging_the_logs_edge_resizes_it_within_the_window() {
        use iced::mouse::{Button, Event as Mouse};
        use scramble_ui::resize::{Drag, HANDLE_WIDTH};

        let mut triib = logging(View::Matrix, true);
        let size = Size::new(1280.0, 800.0);
        let area = size.height - TOOLBAR_HEIGHT - STATUS_BAR_HEIGHT;
        let edge = TOOLBAR_HEIGHT + area - triib.settings.log_height + HANDLE_WIDTH / 2.0;
        let messages: Vec<Message> = {
            let mut simulator = simulate(&triib, size);
            simulator.point_at(iced::Point::new(400.0, edge));
            let _ = simulator.simulate([
                iced::Event::Mouse(Mouse::ButtonPressed(Button::Left)),
                iced::Event::Mouse(Mouse::CursorMoved {
                    position: iced::Point::new(420.0, edge - 100.0),
                }),
                iced::Event::Mouse(Mouse::ButtonReleased(Button::Left)),
            ]);
            simulator.into_messages().collect()
        };
        let most = area - VIEW_ABOVE_LOG;
        let drags: Vec<(Drag, f32, f32)> = messages
            .iter()
            .filter_map(|message| match message {
                Message::LogResized(drag, shown, most) => Some((*drag, *shown, *most)),
                _ => None,
            })
            .collect();
        assert_eq!(
            drags,
            [
                (Drag::Started, 240.0, most),
                (Drag::Moved(-100.0), 240.0, most),
                (Drag::Ended, 240.0, most),
            ],
            "{messages:?}"
        );
        assert_eq!(messages.len(), 3, "the drag alone: {messages:?}");
        for message in messages {
            let _ = triib.update(message);
        }
        assert_eq!(triib.settings.log_height, 340.0);
        // Dragged past the window's top the view keeps its room, and past
        // its bottom the log keeps a few lines.
        for (moved, height) in [(-2000.0, most), (2000.0, crate::log_view::LEAST_HEIGHT)] {
            for drag in [Drag::Started, Drag::Moved(moved), Drag::Ended] {
                let _ = triib.update(Message::LogResized(drag, 340.0, most));
            }
            assert_eq!(triib.settings.log_height, height);
        }
    }

    #[test]
    fn each_inspector_tab_shows_its_part() {
        let (entities, models) = bench();
        let interface = Interface {
            name: "enp6s0".to_owned(),
            mac: MacAddress(HOST_MAC),
            up: true,
            speed: Some(2500),
            physical: true,
            wireless: false,
            hardware_clock: Some(0),
        };
        let settings = Settings {
            interface: Some(interface.name.clone()),
            view: View::Entities,
            inspector: true,
            ..Settings::default()
        };
        let mut triib = Triib::sample(settings, interface, entities, models);
        triib.selected = Some(WIRED_ESP);
        let tabs = [
            (InspectorTab::Entity, "Serial number"),
            (InspectorTab::Streams, "Stream inputs"),
            (InspectorTab::Controls, "Speaker Volume"),
            (InspectorTab::Diagnostics, "AVB interfaces"),
            (InspectorTab::Descriptors, "Clock source"),
        ];
        for (tab, _) in tabs {
            let _ = triib.update(Message::InspectorTab(tab));
            let mut simulator = iced_test::Simulator::with_size(
                iced::Settings {
                    fonts: scramble_ui::font::files().collect(),
                    default_font: scramble_ui::font::TEXT,
                    ..iced::Settings::default()
                },
                Size::new(1280.0, 3000.0),
                window(&triib),
            );
            for (other, part) in tabs {
                assert_eq!(
                    simulator.find(part).is_ok(),
                    other == tab,
                    "{part} on the {tab:?} tab"
                );
            }
        }
    }

    #[test]
    fn edits_send_a_rename_only_when_the_name_changed() {
        let (entities, models) = bench();
        let interface = Interface {
            name: "enp6s0".to_owned(),
            mac: MacAddress(HOST_MAC),
            up: true,
            speed: None,
            physical: true,
            wireless: false,
            hardware_clock: None,
        };
        let mut triib = Triib::sample(Settings::default(), interface, entities, models);
        let mac_mini = EntityId(0xd111_e597_f544_8000);
        let target = NameTarget {
            entity: mac_mini,
            descriptor_type: DescriptorType::ENTITY,
            index: 0,
            name_index: 0,
        };
        let _ = triib.update(Message::EditName(target));
        assert_eq!(triib.editing, Some((target, "Mac mini".to_owned())));
        // Unchanged: nothing to send.
        let _ = triib.update(Message::EditSubmitted);
        assert_eq!(triib.editing, None);
        assert!(triib.pending.is_empty());
        // Longer than an entity holds: not taken.
        let _ = triib.update(Message::EditName(target));
        let _ = triib.update(Message::EditChanged("x".repeat(65)));
        assert_eq!(triib.editing, Some((target, "Mac mini".to_owned())));
        let _ = triib.update(Message::EditChanged("Studio Mac".to_owned()));
        // Selecting another entity drops the edit.
        let _ = triib.update(Message::EntitySelected(EntityId(1)));
        assert_eq!(triib.editing, None);
    }

    #[test]
    fn speeds_read_naturally() {
        assert_eq!(speed_text(100), "100 Mb/s");
        assert_eq!(speed_text(1000), "1 Gb/s");
        assert_eq!(speed_text(2500), "2500 Mb/s");
    }
}
