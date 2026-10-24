//! The entity list: a row for each entity, the name first, then the
//! columns the user picked. Each column's heading is a menu that removes
//! the column or shows another field in it, and a last heading adds one.

use atdecc::descriptor::DescriptorType;
use atdecc::model::EntityModel;
use atdecc::{DiscoveredEntity, EntityId};
use iced::widget::text::Wrapping;
use iced::widget::{container, mouse_area, opaque, scrollable, space, table};
use iced::{Center, Element, Fill, Length};
use scramble_ui::button::{self, Kind};
use scramble_ui::dropdown::{self, Entry};
use scramble_ui::font::{TEXT, Type, styled};
use scramble_ui::icon::{self, Icon};
use scramble_ui::{component, dir, style};

use crate::app::{Message, Triib};
use crate::column_resize::{MIN_WIDTH, resizable};
use crate::describe;
use crate::fl;
use crate::header_band::header_band;
use crate::logos::Logo;
use crate::settings::EntityField;
use crate::text::{fit, measure};
use crate::view::{Rate, VIEW_MARGIN};

/// Around each cell, around the table, and spare room for what measuring
/// its text leaves out.
const TABLE_PADDING: f32 = 12.0;
/// Above and below each row's cells.
const ROW_PADDING: f32 = 4.0;
/// Above and below the headings.
const HEADING_PADDING: f32 = 8.0;
const TABLE_SLACK: f32 = 24.0;
/// What a heading's button adds to its label: padding, a gap and the
/// arrow.
const MENU_ROOM: f32 = 2.0 * 12.0 + 4.0 + 18.0;
/// How far a name is from its cell's edge: the row button's padding, the
/// icon and a gap.
const NAME_INSET: f32 = 16.0 + 20.0 + 8.0;
/// What the name column adds to a name: its inset, and the row button's
/// padding after it.
const NAME_ROOM: f32 = NAME_INSET + 16.0;
/// The width of the heading that adds a column.
const ADD_WIDTH: f32 = 40.0;
/// How far right of a heading its column's divider is: the cell's padding
/// and half the line.
const DIVIDER: f32 = TABLE_PADDING + 0.5;
/// How far a picker's text is from its cell's edge: an extra small
/// drop-down's padding.
const PICKER_INSET: f32 = 12.0;
/// Every row's height: the name's row button's, an extra small button's.
const ROW_HEIGHT: f32 = 32.0;
/// A logo's side in its cell.
const LOGO_SIZE: f32 = 24.0;
/// Every heading's height: the heading menus'.
const HEADING_HEIGHT: f32 = 32.0;
const MEDIUM: iced::Font = iced::Font {
    weight: iced::font::Weight::Medium,
    ..TEXT
};

/// The table, with no margin above it `under_bar`, as the bar over it has
/// its own.
pub fn view(triib: &Triib, under_bar: bool) -> Element<'_, Message> {
    iced::widget::responsive(move |size| view_at(triib, size.width, under_bar)).into()
}

/// What a column's value is for an entity.
fn value(field: EntityField, entity: &DiscoveredEntity, model: Option<&EntityModel>) -> String {
    let descriptor = model.and_then(EntityModel::entity);
    let localized = |reference| {
        model
            .and_then(|model| model.localized(reference))
            .unwrap_or_default()
            .to_owned()
    };
    match field {
        EntityField::Group => descriptor.map_or_else(String::new, |found| found.group_name.into()),
        EntityField::Product => model.map_or_else(String::new, describe::product),
        EntityField::Vendor => {
            descriptor.map_or_else(String::new, |found| localized(found.vendor_name))
        }
        // Drawn, not written.
        EntityField::EntityImage | EntityField::VendorImage => String::new(),
        EntityField::Model => {
            descriptor.map_or_else(String::new, |found| localized(found.model_name))
        }
        EntityField::Firmware => {
            descriptor.map_or_else(String::new, |found| found.firmware_version.into())
        }
        EntityField::SerialNumber => {
            descriptor.map_or_else(String::new, |found| found.serial_number.into())
        }
        EntityField::Milan => model.map_or_else(String::new, describe::milan),
        EntityField::Roles => describe::roles(&entity.adp),
        EntityField::MediaClock => model.map_or_else(String::new, describe::media_clock),
        EntityField::SamplingRate => model
            .and_then(|model| model.audio_units().next())
            .map_or_else(String::new, |unit| {
                Rate(unit.current_sampling_rate).to_string()
            }),
        EntityField::Grandmaster => describe::clock(&entity.adp),
        EntityField::State => describe::read_state(model),
        EntityField::EntityId => entity.entity_id().to_string(),
        EntityField::MacAddress => entity.mac.to_string(),
        EntityField::EntityModelId => entity.adp.entity_model_id.to_string(),
        EntityField::Configuration => model
            .and_then(EntityModel::configuration)
            .map(|configuration| configuration.object_name)
            .filter(|name| !name.is_empty())
            .map_or_else(
                || (entity.adp.current_configuration_index + 1).to_string(),
                str::to_owned,
            ),
        EntityField::TalkerStreams => count(entity.adp.talker_stream_sources),
        EntityField::ListenerStreams => count(entity.adp.listener_stream_sinks),
        EntityField::AvbLite => model.map_or_else(String::new, crate::lite_view::mode),
        EntityField::Wireless => model.map_or_else(String::new, crate::wireless_view::role),
        EntityField::Egress => model
            .and_then(|model| {
                let (interface, _) = model.descriptors(DescriptorType::AVB_INTERFACE).next()?;
                crate::lite_view::egress(model, interface)
            })
            .map_or_else(String::new, |egress| {
                fl!(
                    "common-percent",
                    value = crate::i18n::decimal(format!("{:.1}", egress.share() * 100.0))
                )
            }),
    }
}

/// A cell's text on one line, cut short with an ellipsis to its column.
fn cell<'a>(text: &str, width: f32) -> Element<'a, Message> {
    container(styled(fit(text, TEXT, 14.0, width).name, Type::BodyMedium).wrapping(Wrapping::None))
        .width(Length::Fixed(width))
        .align_x(dir::horizontal_start())
        .clip(true)
        .into()
}

/// One of an entity's logos, once read, at the start of its cell.
fn logo_cell<'a>(
    triib: &'a Triib,
    entity: &DiscoveredEntity,
    logo: Logo,
    width: f32,
) -> Element<'a, Message> {
    let image = triib
        .models
        .get(&entity.entity_id())
        .and_then(|model| triib.logos.of(model, logo));
    let content: Element<'a, Message> = match image {
        Some(image) => iced::widget::image(image)
            .width(LOGO_SIZE)
            .height(LOGO_SIZE)
            .into(),
        None => space().into(),
    };
    container(content)
        .width(Length::Fixed(width))
        .align_x(dir::horizontal_start())
        .into()
}

/// An entity's media clock: a picker of its clock domain's sources, or
/// `shown` when it has only one or is not read.
fn media_clock_cell<'a>(
    triib: &'a Triib,
    entity: &DiscoveredEntity,
    shown: &str,
    width: f32,
) -> Element<'a, Message> {
    let entity = entity.entity_id();
    let picker = triib.models.get(&entity).and_then(|model| {
        let domain = model.clock_domains().next()?;
        crate::view::source_picker(triib, entity, model, &domain).ok()
    });
    match picker {
        // As wide as its column, as the table lays fixed cells out apart.
        Some(picker) => container(picker).width(Length::Fixed(width)).into(),
        // Its text where a picker's starts, so the column's values line
        // up.
        None => {
            let inset = if dir::mirrored() {
                iced::Padding {
                    right: PICKER_INSET,
                    ..iced::Padding::ZERO
                }
            } else {
                iced::Padding {
                    left: PICKER_INSET,
                    ..iced::Padding::ZERO
                }
            };
            container(cell(shown, width - PICKER_INSET))
                .padding(inset)
                .width(Length::Fixed(width))
                .into()
        }
    }
}

/// A count of streams, or nothing for none.
fn count(streams: u16) -> String {
    if streams == 0 {
        String::new()
    } else {
        streams.to_string()
    }
}

/// A column of the entity list: the name, or a field after it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Column {
    Name,
    Field(EntityField),
}

/// The fields no column shows, in the menus' order.
fn unshown(columns: &[EntityField]) -> impl Iterator<Item = EntityField> + '_ {
    EntityField::ALL
        .into_iter()
        .filter(|field| !columns.contains(field))
}

/// An entity's row: its name and the values of the columns shown.
#[derive(Clone)]
struct Row<'a> {
    /// The entity, or `None` for one of this computer's still starting.
    entity: Option<&'a DiscoveredEntity>,
    entity_id: EntityId,
    name: String,
    values: Vec<String>,
}

/// The table for `width` pixels: across the whole width when its columns
/// fit, each widened in proportion to its text, else at their own widths,
/// scrolling sideways.
fn view_at(triib: &Triib, width: f32, under_bar: bool) -> Element<'_, Message> {
    let columns: &[EntityField] = &triib.settings.entity_columns;
    let only_host = triib.settings.host_entities_only && triib.hosts_endpoints();
    let mut rows: Vec<Row<'_>> = triib
        .listed_entities()
        .into_iter()
        .filter(|entity| !only_host || triib.is_host(entity.entity_id()))
        .map(|entity| {
            let model = triib.models.get(&entity.entity_id());
            Row {
                entity: Some(entity),
                entity_id: entity.entity_id(),
                name: triib.entity_name(entity),
                values: columns
                    .iter()
                    .map(|&field| value(field, entity, model))
                    .collect(),
            }
        })
        .collect();
    // This computer's entities still starting, saying what they wait on
    // in the State column, else the first.
    let step_at = columns
        .iter()
        .position(|&field| field == EntityField::State)
        .unwrap_or(0);
    rows.extend(triib.starting_entities().map(|starting| {
        let step = crate::host_view::starting_step(triib, starting.entity_id);
        Row {
            entity: None,
            entity_id: starting.entity_id,
            name: starting.name.clone(),
            values: (0..columns.len())
                .map(|index| {
                    if index == step_at {
                        step.clone()
                    } else {
                        String::new()
                    }
                })
                .collect(),
        }
    }));
    rows.sort_by_key(|row| (row.name.to_lowercase(), row.entity_id));
    let can_add = unshown(columns).next().is_some();
    // Each column's own width: its widest text, its heading with the menu's
    // arrow, and for the name its icon and the row button's padding.
    let widest = |heading: f32, texts: &mut dyn Iterator<Item = &String>| {
        texts
            .map(|text| measure(text, TEXT, 14.0))
            .fold(heading, f32::max)
    };
    let mut natural = vec![
        widest(
            measure(&fl!("entity-name"), MEDIUM, 14.0),
            &mut rows.iter().map(|row| &row.name),
        ) + NAME_ROOM,
    ];
    for (index, field) in columns.iter().enumerate() {
        // A media clock's cell picks the source, with the picker's arrow.
        let picker = if *field == EntityField::MediaClock {
            MENU_ROOM
        } else {
            0.0
        };
        natural.push(
            widest(
                measure(&field.label(), MEDIUM, 14.0) + MENU_ROOM - picker,
                &mut rows.iter().map(|row| &row.values[index]),
            ) + picker,
        );
    }
    // A whole pixel spare, as text exactly a cell's width loses its last
    // letter to the clip.
    for width in &mut natural {
        *width = width.ceil() + 1.0;
    }
    // The widths the user dragged columns to, which they keep.
    let settings = &triib.settings;
    let mut dragged: Vec<Option<f32>> = vec![settings.entity_name_width];
    dragged.extend(
        columns
            .iter()
            .map(|field| settings.entity_column_widths.get(field).copied()),
    );
    if can_add {
        natural.push(ADD_WIDTH);
        dragged.push(Some(ADD_WIDTH));
    }
    // Each column's width: dragged, or its own, the room left in the window
    // shared out among those not dragged in proportion to their own.
    let mut widths: Vec<f32> = natural
        .iter()
        .zip(&dragged)
        .map(|(natural, dragged)| dragged.unwrap_or(*natural).max(MIN_WIDTH))
        .collect();
    let room =
        width - 2.0 * VIEW_MARGIN - TABLE_SLACK - widths.len() as f32 * (2.0 * TABLE_PADDING + 1.0);
    let used: f32 = widths.iter().sum();
    let fits = used <= room;
    let stretchy: f32 = natural
        .iter()
        .zip(&dragged)
        .filter(|(_, dragged)| dragged.is_none())
        .map(|(natural, _)| natural)
        .sum();
    if fits && stretchy > 0.0 {
        for (width, (natural, dragged)) in widths.iter_mut().zip(natural.iter().zip(&dragged)) {
            if dragged.is_none() {
                *width += (room - used) * natural / stretchy;
            }
        }
    }
    let heading = |content: Element<'static, Message>, column: Column, width: f32| {
        resizable(
            content,
            move |width| Message::EntityColumnResized(column, width),
            Message::EntityColumnResizeEnded,
            Message::EntityColumnWidthReset(column),
        )
        .width(Length::Fixed(width))
        .divider(DIVIDER)
        .mirrored(dir::mirrored())
    };
    let name_width = widths[0];
    // Over the names' text, past their icons.
    let name_inset = if dir::mirrored() {
        iced::Padding {
            right: NAME_INSET,
            ..iced::Padding::ZERO
        }
    } else {
        iced::Padding {
            left: NAME_INSET,
            ..iced::Padding::ZERO
        }
    };
    let name_heading = heading(
        container(
            styled(
                fit(&fl!("entity-name"), MEDIUM, 14.0, name_width - NAME_INSET).name,
                Type::LabelLarge,
            )
            .style(style::on_surface_variant)
            .wrapping(Wrapping::None),
        )
        .padding(name_inset)
        .width(Fill)
        // As tall as the other columns' heading menus.
        .height(HEADING_HEIGHT)
        .align_y(Center)
        .align_x(dir::horizontal_start())
        .into(),
        Column::Name,
        name_width,
    );
    let name_cell = move |row: Row<'_>| {
        let entity_id = row.entity_id;
        let name = fit(&row.name, TEXT, 14.0, name_width - NAME_ROOM).name;
        // One starting is this computer's, and nothing to select yet.
        let glyph = match row.entity {
            Some(entity) => triib
                .host_icon(entity_id)
                .unwrap_or_else(|| describe::glyph(&entity.adp)),
            None => Icon::Computer,
        };
        body_cell(
            button::custom(
                Kind::Row,
                scramble_ui::row![
                    icon::icon(glyph, 20),
                    styled(name, Type::BodyMedium).wrapping(Wrapping::None),
                ]
                .spacing(8)
                .align_y(Center),
            )
            .height(ROW_HEIGHT)
            .selected(triib.selected == Some(entity_id))
            .on_press_maybe(row.entity.map(|_| Message::EntitySelected(entity_id))),
            name_width,
        )
    };
    // The other columns' headings, and their cells.
    let mut headings: Vec<(Element<'_, Message>, f32)> = Vec::new();
    let mut cells: Vec<(BodyCell<'_>, f32)> = Vec::new();
    let last = columns.len().saturating_sub(1);
    for (index, &field) in columns.iter().enumerate() {
        let width = widths[index + 1];
        // The column's own actions, then the fields it could show instead.
        let mut entries = vec![
            Entry::item(fl!("column-remove"), Message::EntityColumn(index, None)).icon(Icon::Close),
        ];
        // Earlier columns are on the left, or on the right when the table
        // runs from the right.
        let (earlier, later) = if dir::mirrored() {
            (
                (fl!("column-move-right"), Icon::ArrowForward),
                (fl!("column-move-left"), Icon::ArrowBack),
            )
        } else {
            (
                (fl!("column-move-left"), Icon::ArrowBack),
                (fl!("column-move-right"), Icon::ArrowForward),
            )
        };
        if index > 0 {
            entries.push(
                Entry::item(earlier.0, Message::EntityColumnMoved(index, false)).icon(earlier.1),
            );
        }
        if index < last {
            entries
                .push(Entry::item(later.0, Message::EntityColumnMoved(index, true)).icon(later.1));
        }
        if can_add {
            entries.push(Entry::divider());
            entries.extend(unshown(columns).map(|other| {
                Entry::item(other.label(), Message::EntityColumn(index, Some(other)))
            }));
        }
        let heading_menu = dropdown::menu(field.label(), entries).width(Fill);
        headings.push((
            heading(heading_menu.into(), Column::Field(field), width).into(),
            width,
        ));
        cells.push((
            Box::new(move |row: Row<'_>| {
                let content = match (field, row.entity) {
                    (EntityField::MediaClock, Some(entity)) => {
                        media_clock_cell(triib, entity, &row.values[index], width)
                    }
                    (EntityField::EntityImage, Some(entity)) => {
                        logo_cell(triib, entity, Logo::Entity, width)
                    }
                    (EntityField::VendorImage, Some(entity)) => {
                        logo_cell(triib, entity, Logo::Vendor, width)
                    }
                    _ => cell(&row.values[index], width),
                };
                body_cell(content, width)
            }),
            width,
        ));
    }
    if can_add {
        let add = component::tip(
            dropdown::icon_menu(
                Icon::Add,
                unshown(columns)
                    .map(|field| Entry::item(field.label(), Message::EntityColumnAdded(field)))
                    .collect(),
            ),
            fl!("column-add"),
        );
        let add_width = widths[columns.len() + 1];
        headings.push((add, add_width));
        cells.push((
            Box::new(move |_row: Row<'_>| body_cell(space(), add_width)),
            add_width,
        ));
    }
    // The name on the right and the rest leftwards, in right to left
    // languages.
    let mirrored = dir::mirrored();
    if mirrored {
        headings.reverse();
        cells.reverse();
    }
    // The names' band rounds its outer corner, the rest's its own.
    let name_header = header_table(vec![(name_heading.into(), name_width)], !mirrored, mirrored);
    // The rows' height: each its cells' and the line above it.
    let rows_height = rows.len() as f32 * (ROW_HEIGHT + 2.0 * ROW_PADDING + 1.0);
    let name_body = body_table(vec![(Box::new(name_cell), name_width)], rows.clone());
    let rest_header = header_table(headings, mirrored, !mirrored);
    let rest_body = body_table(cells, rows);
    // The names stay as the rest scrolls sideways, the headings as it
    // scrolls down: the rest's scrolling moves the headings and names with
    // it, and theirs move it.
    let hidden = scrollable::Scrollbar::hidden;
    let rest_header = scrollable(rest_header)
        .id(REST_HEADER)
        .direction(scrollable::Direction::Horizontal(hidden()))
        .on_scroll(|viewport| Message::EntityListScrolled(ListPart::Headings, viewport))
        .width(Fill);
    let name_body = scrollable(name_body)
        .id(NAMES)
        .direction(scrollable::Direction::Vertical(hidden()))
        .on_scroll(|viewport| Message::EntityListScrolled(ListPart::Names, viewport))
        .height(Fill);
    let rest_body = scrollable(rest_body)
        .id(REST)
        .direction(scrollable::Direction::Both {
            vertical: component::thin_scrollbar(),
            horizontal: component::thin_scrollbar(),
        })
        .style(style::scrollbar)
        .on_scroll(|viewport| Message::EntityListScrolled(ListPart::Rest, viewport))
        .width(Fill)
        .height(Fill);
    // A table running from the right starts scrolled to its right.
    let (rest_header, rest_body) = if mirrored {
        (rest_header.anchor_right(), rest_body.anchor_right())
    } else {
        (rest_header, rest_body)
    };
    // The line between the names and the rest: as tall as the headings
    // above, and below as the rows, or the room for them when they scroll,
    // as the table's own lines end with its last row.
    let line = |height: Length| container(space()).width(1).height(height).style(separator);
    let headings_height = Length::Fixed(HEADING_HEIGHT + 2.0 * HEADING_PADDING);
    // The names first, which a row puts on the right in right to left
    // languages.
    let top = scramble_ui::row![name_header, line(headings_height), rest_header];
    let rows_line = container(line(Length::Fixed(rows_height))).height(Fill);
    let bottom = scramble_ui::row![name_body, rows_line, rest_body];
    let margin = iced::Padding {
        top: if under_bar { 0.0 } else { VIEW_MARGIN },
        ..iced::Padding::new(VIEW_MARGIN)
    };
    let view = container(scramble_ui::column![top, bottom].height(Fill)).padding(margin);
    // A press on the table is the table's; one on the space around it
    // clears the selection.
    mouse_area(view).on_press(Message::SelectionCleared).into()
}

/// Headings alone, their band shaded, its top left and right corners round
/// or not.
fn header_table<'a>(
    columns: Vec<(Element<'a, Message>, f32)>,
    left: bool,
    right: bool,
) -> Element<'a, Message> {
    let count = columns.len();
    let columns = columns.into_iter().map(|(heading, width)| {
        table::column(heading, |_row: Row<'a>| space())
            .width(Length::Fixed(width))
            .align_y(Center)
    });
    let table = table(columns, Vec::<Row<'a>>::new())
        .padding_x(TABLE_PADDING)
        .padding_y(HEADING_PADDING);
    opaque(header_band(table, count, HEADING_PADDING).round(left, right))
}

/// Rows alone, each its cells' fixed height, so the names and the rest
/// line up; the line above the first parts them from the headings.
fn body_table<'a>(columns: Vec<(BodyCell<'a>, f32)>, rows: Vec<Row<'a>>) -> Element<'a, Message> {
    let columns = columns.into_iter().map(|(view, width)| {
        table::column(space().width(0).height(0), view)
            .width(Length::Fixed(width))
            .align_y(Center)
    });
    opaque(table(columns, rows).padding_x(TABLE_PADDING).padding_y(0))
}

/// A row's cell of the rows' table: its content at the rows' one height,
/// with the space rows have above and below.
fn body_cell<'a>(content: impl Into<Element<'a, Message>>, width: f32) -> Element<'a, Message> {
    container(content)
        .width(Length::Fixed(width))
        .height(ROW_HEIGHT + 2.0 * ROW_PADDING)
        .align_y(Center)
        .align_x(dir::horizontal_start())
        .clip(true)
        .into()
}

/// The line between the names and the rest, as the table's own lines are.
fn separator(theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(theme.extended_palette().background.strong.color.into()),
        ..container::Style::default()
    }
}

/// A cell's view in the rows' table.
type BodyCell<'a> = Box<dyn Fn(Row<'a>) -> Element<'a, Message> + 'a>;

/// The entity list's scrolled parts: the other columns' headings, the
/// names, and the rest of the rows, which scroll together.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListPart {
    Headings,
    Names,
    Rest,
}

/// Their scrollables, which the others' scrolling moves.
pub const REST_HEADER: &str = "entity-list-headings";
pub const NAMES: &str = "entity-list-names";
pub const REST: &str = "entity-list-rest";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn columns_change_go_and_come() {
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
        let mut triib = Triib::sample(Settings::default(), interface, entities, models);
        let columns = |triib: &Triib| triib.settings.entity_columns.clone();
        // Group becomes Vendor; a field shown elsewhere is not taken twice.
        let _ = triib.update(Message::EntityColumn(0, Some(EntityField::Vendor)));
        assert_eq!(columns(&triib)[0], EntityField::Vendor);
        let _ = triib.update(Message::EntityColumn(0, Some(EntityField::Grandmaster)));
        assert_eq!(columns(&triib)[0], EntityField::Vendor);
        // Product goes; MAC address comes at the right.
        let _ = triib.update(Message::EntityColumn(1, None));
        assert!(!columns(&triib).contains(&EntityField::Product));
        let _ = triib.update(Message::EntityColumnAdded(EntityField::MacAddress));
        assert_eq!(columns(&triib).last(), Some(&EntityField::MacAddress));
        assert_eq!(columns(&triib).len(), EntityField::DEFAULT_COLUMNS.len());
        // A place past the last column changes nothing.
        let before = columns(&triib);
        let _ = triib.update(Message::EntityColumn(99, None));
        assert_eq!(columns(&triib), before);

        // The columns survive saving and loading.
        let file = std::env::temp_dir().join(format!("triib-columns-{}.toml", std::process::id()));
        triib_store::save(&file, &triib.settings).unwrap();
        let saved = std::fs::read_to_string(&file).unwrap();
        let loaded: Settings = triib_store::load_or_create(&file).unwrap();
        let _ = std::fs::remove_file(&file);
        assert!(saved.contains("\"mac-address\""), "{saved}");
        assert_eq!(loaded.entity_columns, before);
    }

    /// This computer's endpoints show what they are from the moment they
    /// advertise, before the daemon's status names them.
    #[test]
    fn own_endpoints_have_their_icons_at_once() {
        use atdecc::EntityId;
        use scramble_ui::icon::Icon;

        let mut triib = sample();
        triib.settings.interface = Some("enp6s0".to_owned());
        triib.endpoints = None;
        let own = avb_net::MacAddress([0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b]);
        let template = *triib.entities.values().next().unwrap();
        let mut advertise = |id: u64, mac, model: u64| {
            let mut entity = template;
            entity.adp.entity_id = EntityId(id);
            entity.adp.entity_model_id = atdecc::EntityModelId(model);
            entity.mac = mac;
            triib.entities.insert(EntityId(id), entity);
        };
        advertise(0x9c6b_00ff_0030_9a2b, own, 0x8c1f_6436_c0f3_59f0);
        // The app's own controller, from the same address.
        advertise(0x9c6b_00ff_fe30_9a2b, own, 0x8c1f_6436_c000_0001);
        // Another computer's device of the same model.
        let other = avb_net::MacAddress([0xf0, 0xa7, 0x31, 0xf4, 0x0f, 0x14]);
        advertise(0xf0a7_31ff_00f4_0f14, other, 0x8c1f_6436_c0f3_59f0);
        assert_eq!(
            triib.host_icon(EntityId(0x9c6b_00ff_0030_9a2b)),
            Some(Icon::Computer)
        );
        assert_eq!(triib.host_icon(EntityId(0x9c6b_00ff_fe30_9a2b)), None);
        assert_eq!(triib.host_icon(EntityId(0xf0a7_31ff_00f4_0f14)), None);
    }

    /// triib's controller, which has no model to read, is named after
    /// the computer it runs on, and controllers have their own icon.
    #[test]
    fn triib_controllers_say_where_they_run() {
        use atdecc::EntityId;
        use scramble_ui::icon::Icon;

        let mut triib = sample();
        triib.settings.interface = Some("enp6s0".to_owned());
        let (&named, template) = triib.entities.iter().next().unwrap();
        let template = *template;
        let host = triib.entity_name_of(named);
        let mut controller = |id: u64, mac| {
            let mut entity = template;
            entity.adp.entity_id = EntityId(id);
            entity.adp.entity_model_id = crate::network::ENTITY_MODEL_ID;
            entity.adp.talker_stream_sources = 0;
            entity.adp.listener_stream_sinks = 0;
            entity.adp.talker_capabilities = atdecc::adp::TalkerCapabilities::empty();
            entity.adp.listener_capabilities = atdecc::adp::ListenerCapabilities::empty();
            entity.adp.controller_capabilities = atdecc::adp::ControllerCapabilities::IMPLEMENTED;
            entity.mac = mac;
            triib.entities.insert(EntityId(id), entity);
            EntityId(id)
        };
        // On the computer of an entity read.
        let there = controller(0x1, template.mac);
        // On this computer.
        let here = controller(
            0x2,
            avb_net::MacAddress([0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b]),
        );
        // On a computer of no other entity.
        let alone = controller(0x3, avb_net::MacAddress([2, 0, 0, 0, 0, 3]));
        assert_eq!(triib.entity_name_of(there), format!("triib on {host}"));
        assert_eq!(triib.entity_name_of(here), "triib on this computer");
        assert_eq!(triib.entity_name_of(alone), "triib controller");
        assert_eq!(
            crate::describe::glyph(&triib.entities[&alone].adp),
            Icon::Traffic
        );
    }

    /// A controller that does not advertise is listed once its command is
    /// heard, named for the computer it runs on, and the inspector says
    /// what is known of it.
    #[test]
    fn controllers_heard_are_listed() {
        use crate::network::{Report, ReportKind};
        use atdecc::EntityId;

        let mut triib = sample();
        let mac_mini = *triib
            .entities
            .values()
            .find(|entity| triib.entity_name(entity) == "Mac mini")
            .expect("the bench's Mac mini");
        let apple = EntityId(0xd111_e597_f544_0000);
        // LOCK_ENTITY, as the Mac's controller sends it.
        let mut bytes = vec![0xfb, 0x00, 0x00, 0x1c];
        bytes.extend(0x9c6b_00ff_0030_9a2bu64.to_be_bytes());
        bytes.extend(apple.0.to_be_bytes());
        bytes.extend([0x00, 0x01, 0x00, 0x01]);
        bytes.resize(40, 0);
        let frame = atdecc::blocking::Frame {
            at: std::time::SystemTime::now(),
            sent: false,
            peer: mac_mini.mac,
            bytes,
        };
        let _ = triib.update(Message::External(crate::External::Network(Report {
            generation: 0,
            kind: ReportKind::Frames(vec![frame]),
        })));
        assert!(
            triib
                .listed_entities()
                .iter()
                .any(|entity| entity.entity_id() == apple)
        );
        assert!(
            !triib
                .shown_entities()
                .iter()
                .any(|entity| entity.entity_id() == apple),
            "the matrix and the map leave it out"
        );
        assert_eq!(triib.entity_name_of(apple), "Controller on Mac mini");
        let _ = triib.update(Message::EntitySelected(apple));
        let mut simulator = crate::view::tests::simulate(&triib, iced::Size::new(1280.0, 800.0));
        assert!(simulator.find("Controller on Mac mini").is_ok());
        assert!(simulator.find(mac_mini.mac.to_string().as_str()).is_ok());
    }

    fn sample() -> Triib {
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
        let settings = crate::settings::Settings {
            view: crate::settings::View::Entities,
            ..crate::settings::Settings::default()
        };
        Triib::sample(settings, interface, entities, models)
    }

    #[test]
    fn columns_move_and_keep_their_widths() {
        let mut triib = sample();
        let columns = |triib: &Triib| triib.settings.entity_columns.clone();
        // Group moves right past Product, then back; the ends stay put.
        let _ = triib.update(Message::EntityColumnMoved(0, true));
        assert_eq!(
            &columns(&triib)[..2],
            [EntityField::Product, EntityField::Group]
        );
        let _ = triib.update(Message::EntityColumnMoved(1, false));
        assert_eq!(
            &columns(&triib)[..2],
            [EntityField::Group, EntityField::Product]
        );
        let before = columns(&triib);
        let _ = triib.update(Message::EntityColumnMoved(0, false));
        let _ = triib.update(Message::EntityColumnMoved(before.len() - 1, true));
        assert_eq!(columns(&triib), before);

        // Dragged widths stay, by field, until reset.
        let _ = triib.update(Message::EntityColumnResized(Column::Name, 240.0));
        let _ = triib.update(Message::EntityColumnResized(
            Column::Field(EntityField::Firmware),
            90.0,
        ));
        let _ = triib.update(Message::EntityColumnResizeEnded);
        assert_eq!(triib.settings.entity_name_width, Some(240.0));
        assert_eq!(
            triib
                .settings
                .entity_column_widths
                .get(&EntityField::Firmware),
            Some(&90.0)
        );

        // Fields, order and widths survive saving and loading.
        let file = std::env::temp_dir().join(format!("triib-widths-{}.toml", std::process::id()));
        triib_store::save(&file, &triib.settings).unwrap();
        let loaded: crate::settings::Settings = triib_store::load_or_create(&file).unwrap();
        let _ = std::fs::remove_file(&file);
        assert_eq!(loaded.entity_columns, triib.settings.entity_columns);
        assert_eq!(
            loaded.entity_column_widths,
            triib.settings.entity_column_widths
        );
        assert_eq!(loaded.entity_name_width, Some(240.0));

        let _ = triib.update(Message::EntityColumnWidthReset(Column::Field(
            EntityField::Firmware,
        )));
        let _ = triib.update(Message::EntityColumnWidthReset(Column::Name));
        assert!(triib.settings.entity_column_widths.is_empty());
        assert_eq!(triib.settings.entity_name_width, None);
    }

    #[test]
    fn dragging_a_headings_edge_resizes_its_column() {
        use iced::mouse::{Button, Event as Mouse};
        use iced::{Event, Point, Size};

        let triib = sample();
        // Presses at `from`, moves `by`, and releases, in a fresh window,
        // returning what the window sent.
        let gesture = |from: Point, by: f32, presses: usize| {
            let mut simulator = iced_test::Simulator::with_size(
                iced::Settings {
                    fonts: scramble_ui::font::files().collect(),
                    default_font: TEXT,
                    ..iced::Settings::default()
                },
                Size::new(2200.0, 420.0),
                crate::view::window(&triib),
            );
            simulator.point_at(from);
            for _ in 0..presses {
                let _ = simulator.simulate([Event::Mouse(Mouse::ButtonPressed(Button::Left))]);
                if by != 0.0 {
                    let to = Point::new(from.x + by, from.y);
                    simulator.point_at(to);
                    let _ = simulator.simulate([Event::Mouse(Mouse::CursorMoved { position: to })]);
                }
                let _ = simulator.simulate([Event::Mouse(Mouse::ButtonReleased(Button::Left))]);
            }
            simulator.into_messages().collect::<Vec<Message>>()
        };
        let group_width = |messages: &[Message]| {
            messages.iter().find_map(|message| match message {
                Message::EntityColumnResized(Column::Field(EntityField::Group), width) => {
                    Some(*width)
                }
                _ => None,
            })
        };
        // Where the Group heading's edge is depends on the fonts measuring
        // the columns, so it is found along the heading row.
        let row = 92.0;
        // Steps narrower than the edge's grab area cannot miss it.
        let (edge, messages) = (200..900)
            .step_by(6)
            .map(|x| Point::new(x as f32, row))
            .find_map(|edge| {
                let messages = gesture(edge, 50.0, 1);
                group_width(&messages).is_some().then_some((edge, messages))
            })
            .expect("the Group heading's edge");
        let width = group_width(&messages).unwrap();
        assert!(width > 50.0 + MIN_WIDTH, "{width}");
        assert!(
            messages
                .iter()
                .any(|message| matches!(message, Message::EntityColumnResizeEnded))
        );
        // Dragging from further in, over the heading's menu, does not.
        assert_eq!(
            group_width(&gesture(Point::new(edge.x - 40.0, row), 50.0, 1)),
            None
        );
        // A double click on the edge asks for the column to fit its text.
        assert!(gesture(edge, 0.0, 2).iter().any(|message| matches!(
            message,
            Message::EntityColumnWidthReset(Column::Field(EntityField::Group))
        )));
    }

    #[test]
    fn values_come_from_the_advertisement_and_the_model() {
        let (entities, models) = crate::view::tests::bench();
        let mac_mini = atdecc::EntityId(0xd111_e597_f544_8000);
        let entity = &entities[&mac_mini];
        let model = models.get(&mac_mini);
        assert_eq!(
            value(EntityField::MacAddress, entity, model),
            entity.mac.to_string()
        );
        assert_eq!(value(EntityField::Vendor, entity, model), "Apple Inc.");
        assert_eq!(
            value(EntityField::MediaClock, entity, model),
            "Mac System Clock"
        );
        assert!(!value(EntityField::SamplingRate, entity, model).is_empty());
    }

    #[test]
    fn menus_offer_only_what_no_column_shows() {
        let columns = [EntityField::Group, EntityField::Grandmaster];
        let offered: Vec<EntityField> = unshown(&columns).collect();
        assert_eq!(offered.len(), EntityField::ALL.len() - 2);
        assert!(!offered.contains(&EntityField::Group));
        assert!(!offered.contains(&EntityField::Grandmaster));
        assert_eq!(offered[0], EntityField::Product);
        assert_eq!(unshown(&EntityField::ALL).count(), 0);
    }
}
