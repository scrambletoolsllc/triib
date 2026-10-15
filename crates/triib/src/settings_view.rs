//! The Settings dialog: a header with the title, close button and tabs
//! that stays put, the chosen tab's settings under it, which scroll, and
//! triib's version at the foot.

use std::fmt;

use iced::widget::{container, mouse_area, opaque, space, toggler};
use iced::{Center, Color, Element, Fill, Length, Theme};
use scramble_ui::appearance::{Appearance, hex_to_color};
use scramble_ui::button::{self, Kind};
use scramble_ui::component;
use scramble_ui::font::{Type, aligned, styled};
use scramble_ui::icon::Icon;
use scramble_ui::{Scheme, dropdown, scheme, shape, style};

use crate::app::{Message, TRIIB_SEED, Triib};
use crate::fl;
use crate::settings::SYSTEM_LANGUAGE;

/// The tabs Settings is split into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SettingsTab {
    #[default]
    General,
    Appearance,
    Storage,
}

/// The widest and tallest the dialog gets.
const WIDTH: f32 = 520.0;
const HEIGHT: f32 = 640.0;
/// The narrowest the appearance choices show their labels in.
const ROOM_FOR_LABELS: f32 = 330.0;

/// Colors offered for the scheme when the system accent is not used,
/// triib's teal first.
const ACCENT_SWATCHES: [Color; 11] = [
    TRIIB_SEED,
    scheme::DEFAULT_SEED,
    Color::from_rgb8(0x43, 0xa0, 0x47),
    Color::from_rgb8(0xf9, 0xa8, 0x25),
    Color::from_rgb8(0xf5, 0x7c, 0x00),
    Color::from_rgb8(0xe5, 0x39, 0x35),
    Color::from_rgb8(0xd8, 0x1b, 0x60),
    Color::from_rgb8(0x8e, 0x24, 0xaa),
    Color::from_rgb8(0x39, 0x49, 0xab),
    Color::from_rgb8(0x6d, 0x4c, 0x41),
    Color::from_rgb8(0x75, 0x75, 0x75),
];

/// The dialog over `base`, dimming it; a click outside the dialog closes
/// it.
pub fn dialog<'a>(triib: &'a Triib, base: Element<'a, Message>) -> Element<'a, Message> {
    let tab = |label: String, which: SettingsTab| component::Tab {
        label: crate::i18n::lasting(label),
        icon: None,
        selected: triib.settings_tab == which,
        on_press: Message::SettingsTab(which),
    };
    let header = scramble_ui::column![
        scramble_ui::row![
            styled(fl!("settings-title"), Type::HeadlineSmall)
                .width(Fill)
                .align_x(scramble_ui::dir::text_start()),
            component::tip(
                button::icon_button(Icon::Close).on_press(Message::SettingsClosed),
                fl!("common-close"),
            ),
        ]
        .align_y(Center),
        component::tabs(vec![
            tab(fl!("settings-general"), SettingsTab::General),
            tab(fl!("settings-appearance"), SettingsTab::Appearance),
            tab(fl!("settings-storage"), SettingsTab::Storage),
        ]),
    ]
    .spacing(12);
    let mut content = match triib.settings_tab {
        SettingsTab::General => general(triib),
        SettingsTab::Appearance => appearance(triib),
        SettingsTab::Storage => storage(triib),
    };
    if let Some(error) = &triib.settings_error {
        content = content.push(aligned(
            styled(error.as_str(), Type::BodyMedium).style(style::error_text),
        ));
    }
    let footer = container(
        styled(
            format!("triib {}", env!("CARGO_PKG_VERSION")),
            Type::BodySmall,
        )
        .style(style::on_surface_variant),
    )
    .padding([12, 24])
    .center_x(Fill);
    let card = container(scramble_ui::column![
        container(header).padding(iced::Padding {
            top: 24.0,
            right: 24.0,
            bottom: 0.0,
            left: 24.0,
        }),
        container(component::scroll(container(content).padding([16, 24]))).height(Fill),
        footer,
    ])
    .max_width(WIDTH)
    .max_height(HEIGHT)
    .width(Fill)
    .height(Fill)
    .style(style::dialog);
    iced::widget::stack![
        base,
        mouse_area(
            // The card appears at once, even where animations are on.
            container(opaque(card))
                .padding(24)
                .center(Fill)
                .style(style::scrim),
        )
        .on_press(Message::SettingsClosed),
    ]
    .into()
}

/// A language as the picker shows it.
#[derive(Debug, Clone, PartialEq)]
struct Language {
    tag: String,
    label: String,
}

impl fmt::Display for Language {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.label)
    }
}

/// The interface's language.
fn general(triib: &Triib) -> iced::widget::Column<'_, Message> {
    let mut choices = vec![Language {
        tag: SYSTEM_LANGUAGE.to_owned(),
        label: fl!(
            "settings-language-system",
            language = crate::i18n::system_language_name()
        ),
    }];
    choices.extend(crate::i18n::languages().iter().map(|(tag, name)| Language {
        tag: tag.clone(),
        label: name.clone(),
    }));
    let selected = choices
        .iter()
        .find(|choice| choice.tag == triib.settings.language)
        .unwrap_or(&choices[0])
        .clone();
    scramble_ui::column![
        component::section(fl!("settings-language")),
        dropdown::pick(choices, Some(selected), |choice: Language| {
            Message::LanguageSelected(choice.tag)
        })
        .width(Fill),
        aligned(
            styled(fl!("settings-language-note"), Type::BodySmall).style(style::on_surface_variant),
        ),
        component::section(fl!("settings-outside-control")),
        switch_row(
            fl!("settings-allow-outside-control"),
            fl!("settings-allow-outside-control-note"),
            triib.settings.outside_control,
            Message::OutsideControlToggled,
        ),
        allowed_agents(triib),
    ]
    .spacing(12)
}

/// The agents the user allowed to control triib, and forgetting them.
fn allowed_agents(triib: &Triib) -> Element<'_, Message> {
    if triib.settings.allowed_agents.is_empty() {
        return space().into();
    }
    scramble_ui::row![
        container(aligned(
            styled(
                fl!(
                    "settings-allowed-agents",
                    agents = triib.settings.allowed_agents.join(", ")
                ),
                Type::BodyMedium
            )
            .style(style::on_surface_variant)
        ))
        .width(Fill),
        button::button(Kind::Text, fl!("settings-forget-agents")).on_press(Message::ForgetAgents),
    ]
    .spacing(16)
    .align_y(Center)
    .into()
}

/// Where triib keeps its files, as prev's Storage tab has it: each path
/// typed, then applied with Enter or Apply, which shows once it is
/// changed, or chosen with the folder dialog; why one is refused under
/// its row.
fn storage(triib: &Triib) -> iced::widget::Column<'_, Message> {
    use crate::storage::{Storage, shown_path};
    let mut rows = scramble_ui::column![].spacing(12);
    for storage in Storage::shown() {
        let draft = &triib.storage_drafts[storage.index()];
        let changed = *draft != shown_path(storage.path(&triib.settings));
        let mut entry = scramble_ui::row![
            container(component::text_field(
                storage.label(),
                draft,
                component::Backdrop::ContainerHigh,
                move |input| {
                    input
                        .on_input(move |text| Message::StorageDraft(storage, text))
                        .on_submit(Message::StorageApply(storage))
                },
            ))
            .width(Fill),
        ]
        .spacing(8)
        .align_y(Center);
        if changed {
            entry = entry.push(component::beside_field(
                button::button(Kind::Filled, fl!("settings-storage-apply"))
                    .on_press(Message::StorageApply(storage)),
            ));
        }
        entry = entry.push(component::beside_field(
            button::button(Kind::Tonal, fl!("settings-storage-choose"))
                .on_press(Message::StorageChoose(storage)),
        ));
        rows = rows.push(entry);
        if let Some(problem) = &triib.storage_errors[storage.index()] {
            rows = rows.push(aligned(
                styled(problem.as_str(), Type::BodySmall).style(style::error_text),
            ));
        }
    }
    let file = triib_store::paths::config_file()
        .as_deref()
        .map(shown_path)
        .unwrap_or_default();
    rows = rows.push(aligned(
        styled(fl!("settings-storage-note", file = file), Type::BodySmall)
            .style(style::on_surface_variant),
    ));
    if Storage::shown().any(|storage| storage == Storage::Entities) {
        rows = rows.push(aligned(
            styled(fl!("settings-storage-entities-note"), Type::BodySmall)
                .style(style::on_surface_variant),
        ));
    }
    rows
}

/// Light or dark, colors, and motion.
fn appearance(triib: &Triib) -> iced::widget::Column<'_, Message> {
    let settings = &triib.settings;
    let current = settings.appearance;
    // With room, each choice with its label; on a phone, icons with their
    // labels as tooltips.
    let choices = iced::widget::responsive(move |size| {
        let choices = [
            (
                Icon::Settings,
                fl!("settings-appearance-system"),
                Appearance::System,
            ),
            (
                Icon::LightMode,
                fl!("settings-appearance-light"),
                Appearance::Light,
            ),
            (
                Icon::DarkMode,
                fl!("settings-appearance-dark"),
                Appearance::Dark,
            ),
        ];
        let roomy = size.width >= ROOM_FOR_LABELS;
        let group = component::connected_with_tips(
            choices
                .into_iter()
                .map(|(glyph, label, value)| {
                    let choice = if roomy {
                        button::with_icon(Kind::Tonal, glyph, label.clone())
                    } else {
                        button::icon_button(glyph).kind(Kind::Tonal)
                    };
                    let choice = choice
                        .selected(current == value)
                        .on_press(Message::AppearanceSelected(value));
                    (choice, (!roomy).then(|| label.into()))
                })
                .collect(),
        );
        // On the side the settings start on.
        container(group)
            .width(Fill)
            .align_x(scramble_ui::dir::horizontal_start())
            .into()
    })
    .height(Length::Fixed(button::Size::Small.height()));
    let accent_note = match (settings.system_accent, &triib.omarchy, triib.system_accent) {
        (false, _, _) => fl!("settings-accent-picked"),
        (true, Some(palette), _) => fl!("settings-accent-omarchy", theme = palette.name.clone()),
        (true, None, Some(_)) => fl!("settings-accent-desktop"),
        (true, None, None) => fl!("settings-accent-none"),
    };
    // The picked color is in use when the system accent is off or the
    // system has none.
    let picking =
        !settings.system_accent || (triib.omarchy.is_none() && triib.system_accent.is_none());
    let chosen = settings
        .accent_color
        .as_deref()
        .and_then(hex_to_color)
        .unwrap_or(TRIIB_SEED);
    let swatches: Element<'_, Message> = if picking {
        scramble_ui::dir::row(
            ACCENT_SWATCHES
                .iter()
                .map(|&swatch| accent_swatch(swatch, hex(swatch) == hex(chosen))),
        )
        .spacing(4)
        .wrap()
        .into()
    } else {
        space().into()
    };
    let animations_note = if triib.system_animations {
        fl!("settings-animations-note")
    } else {
        fl!("settings-animations-reduced")
    };
    scramble_ui::column![
        choices,
        component::section(fl!("settings-colors")),
        switch_row(
            fl!("settings-system-accent"),
            accent_note,
            settings.system_accent,
            Message::SystemAccentToggled,
        ),
        swatches,
        component::section(fl!("settings-motion")),
        switch_row(
            fl!("settings-animations"),
            animations_note,
            settings.animations,
            Message::AnimationsToggled,
        ),
    ]
    .spacing(12)
}

/// A setting with a title, a note under it and a switch.
fn switch_row<'a>(
    title: String,
    note: String,
    on: bool,
    toggled: fn(bool) -> Message,
) -> Element<'a, Message> {
    scramble_ui::row![
        scramble_ui::column![
            styled(title, Type::BodyLarge),
            aligned(styled(note, Type::BodyMedium).style(style::on_surface_variant)),
        ]
        .spacing(2)
        .width(Fill),
        toggler(on).on_toggle(toggled).size(28).style(style::switch),
    ]
    .spacing(16)
    .align_y(Center)
    .into()
}

fn hex(color: Color) -> String {
    let [red, green, blue, _] = color.into_rgba8();
    format!("#{red:02X}{green:02X}{blue:02X}")
}

/// A round button that picks `color` for the scheme, ringed when chosen.
fn accent_swatch<'a>(color: Color, selected: bool) -> Element<'a, Message> {
    let dot = container(space().width(24).height(24)).style(move |theme: &Theme| {
        let scheme = Scheme::of(theme);
        container::Style {
            background: Some(color.into()),
            border: iced::Border {
                color: if selected {
                    scheme.on_surface
                } else {
                    scheme.outline_variant
                },
                width: if selected { 3.0 } else { 1.0 },
                radius: shape::FULL.into(),
            },
            ..container::Style::default()
        }
    });
    let hex = hex(color);
    component::tip(
        button::custom(Kind::Standard, dot)
            .size(button::Size::ExtraSmall)
            .width(Length::Fixed(36.0))
            .on_press(Message::AccentPicked(hex.clone())),
        hex,
    )
}
