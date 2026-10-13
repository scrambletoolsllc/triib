//! The connection matrix: talkers' stream outputs along the top,
//! listeners' stream inputs down the right, each talker and listener
//! collapsible to one column or row, and in each cell whether the input
//! is bound to the output and how that is going.

mod grid;

use atdecc::model::EntityModel;
use atdecc::stream_format::{Fit, fit};
use atdecc::{DescriptorType, DiscoveredEntity, EntityId, StreamFormat};
use iced::widget::text::Wrapping;
use iced::widget::{container, mouse_area, pin, space, stack};
use iced::{Center, Color, Element, Fill, Length, Rectangle, Theme};
use scramble_ui::button::{self, Kind, Size};
use scramble_ui::component;
use scramble_ui::dropdown::{self, Entry};
use scramble_ui::font::{Type, styled};
use scramble_ui::icon::{self, Icon};
use scramble_ui::popover::popover;
use scramble_ui::{Scheme, faded, shape, style};
use scramble_ui::{column, dir, row};

use crate::app::{self, Message, Triib};
use crate::fl;
use crate::network::Action;
use crate::settings::Streams;

const STATUS_HEIGHT: f32 = 48.0;

/// Which end of a connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    Talker,
    Listener,
}

/// A column or row: a talker or listener, or one of its streams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Line {
    Entity(EntityId),
    Stream(EntityId, u16),
}

impl Line {
    pub fn entity(self) -> EntityId {
        match self {
            Line::Entity(entity_id) | Line::Stream(entity_id, _) => entity_id,
        }
    }
}

/// What the pointer is over: a cell, or a column's or row's heading.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Hover {
    pub column: Option<Line>,
    pub row: Option<Line>,
}

/// A column's or row's heading.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Heading {
    pub line: Line,
    pub name: String,
    /// For an entity, the end of its MAC address, telling apart entities
    /// of the same name; for a stream, its format in short.
    pub detail: String,
    /// For an entity, whether it is collapsed to this line.
    pub collapsed: Option<bool>,
}

/// What a cell for one stream output and one stream input shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    Connected,
    /// Bound, the listener has not found the talker's stream yet.
    Waiting,
    /// Bound, but the talker's reservation failed or the formats differ.
    Trouble,
    /// A bind or unbind is on its way.
    Working,
    /// Not bound to this output; the formats match.
    Open,
    /// The input takes the talker's format but is set to another.
    Change,
    /// The input does not take the talker's format.
    Incompatible,
}

impl State {
    /// The icon, and whether it is filled.
    pub fn icon(self) -> (Icon, bool) {
        match self {
            State::Connected => (Icon::Link, true),
            State::Waiting => (Icon::Pending, false),
            State::Trouble => (Icon::Error, true),
            State::Working => (Icon::ProgressActivity, false),
            State::Open => (Icon::RadioButtonUnchecked, false),
            State::Change => (Icon::ChangeCircle, false),
            State::Incompatible => (Icon::Remove, false),
        }
    }

    pub fn color(self, scheme: &Scheme) -> Color {
        match self {
            State::Connected => scheme.primary,
            State::Waiting => scheme.tertiary,
            State::Trouble => scheme.error,
            State::Working => scheme.on_surface_variant,
            State::Open => faded(scheme.on_surface_variant, 0.45),
            State::Change => faded(scheme.on_surface_variant, 0.6),
            State::Incompatible => faded(scheme.on_surface_variant, 0.22),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    /// Under an expanded talker or beside an expanded listener.
    Blank,
    /// An entity's outputs against its own inputs, which triib does not
    /// connect, as Hive does not; drawn empty.
    Own,
    /// How many bindings join a collapsed talker's or listener's streams
    /// to the other line.
    Count(usize),
    Stream {
        state: State,
        /// What clicking does.
        action: Option<Action>,
    },
}

/// A matrix cell's menu of the formats both its ends take.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FormatMenu {
    pub talker: (EntityId, u16),
    pub listener: (EntityId, u16),
    /// The cell, from the matrix's top left corner.
    pub cell: Rectangle,
}

/// Everything the matrix widget draws.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Grid {
    pub columns: Vec<Heading>,
    pub rows: Vec<Heading>,
    /// Row by row.
    pub cells: Vec<Cell>,
    /// The entity the inspector shows.
    pub selected: Option<EntityId>,
}

impl Grid {
    pub fn cell(&self, row: usize, column: usize) -> Cell {
        self.cells
            .get(row * self.columns.len() + column)
            .copied()
            .unwrap_or(Cell::Blank)
    }

    fn row_of(&self, line: Line) -> Option<usize> {
        self.rows.iter().position(|heading| heading.line == line)
    }

    fn column_of(&self, line: Line) -> Option<usize> {
        self.columns.iter().position(|heading| heading.line == line)
    }
}

/// A stream of an entity, as the matrix lists it.
struct Stream<'a> {
    entity_id: EntityId,
    index: u16,
    name: String,
    format: StreamFormat,
    /// For an input, the formats it takes.
    supported: Vec<StreamFormat>,
    model: &'a EntityModel,
    /// One of this computer's entities, whose inputs bind to its own
    /// outputs.
    host: bool,
}

impl Stream<'_> {
    fn key(&self) -> (EntityId, u16) {
        (self.entity_id, self.index)
    }

    /// For an input, whether `output` is its own entity's and not bound to
    /// it: a connection triib does not offer, as most entities refuse it,
    /// but on this computer's own, which take it. One made elsewhere still
    /// shows, so it can be undone.
    fn own(&self, output: &Stream<'_>) -> bool {
        self.entity_id == output.entity_id && !self.host && self.bound_to() != Some(output.key())
    }

    /// For an input, the talker's stream output it is bound to.
    fn bound_to(&self) -> Option<(EntityId, u16)> {
        self.model
            .binding(self.index)
            .and_then(|binding| binding.talker_stream())
    }
}

/// A talker or listener with the streams the matrix shows.
struct Group<'a> {
    entity_id: EntityId,
    name: String,
    /// The end of its MAC address.
    tag: String,
    streams: Vec<Stream<'a>>,
}

impl Group<'_> {
    fn label(&self) -> String {
        format!("{} {}", self.name, self.tag)
    }
}

/// The room the matrix's filters, hint, legend and padding take around the
/// grid, with the legend on two lines.
const AROUND_GRID: f32 = 2.0 * 16.0 + 40.0 + 16.0 + 12.0 + STATUS_HEIGHT + 12.0 + 48.0;

pub fn view(triib: &Triib) -> Element<'_, Message> {
    iced::widget::responsive(move |size| view_at(triib, size.height)).into()
}

/// The matrix in `height` pixels: the grid takes what the rest leaves, or
/// when that is too little for its headings and two rows, the whole view
/// scrolls with the grid at that least height.
fn view_at(triib: &Triib, height: f32) -> Element<'_, Message> {
    let cramped = height < grid::LEAST_HEIGHT + AROUND_GRID;
    let mut talkers = groups(triib, Side::Talker);
    let mut listeners = groups(triib, Side::Listener);
    let count = |groups: &[Group<'_>]| {
        groups
            .iter()
            .map(|group| group.streams.len())
            .sum::<usize>()
    };
    // How many streams leaving out what cannot connect hides, when it is on.
    let hidden = triib.settings.matrix_connectable_only.then(|| {
        let before = count(&talkers) + count(&listeners);
        connectable_only(&mut talkers, &mut listeners);
        before - count(&talkers) - count(&listeners)
    });
    let body: Element<'_, Message> = if talkers.is_empty() || listeners.is_empty() {
        let filtered = !triib.search.trim().is_empty()
            || triib.settings.matrix_streams != Streams::All
            || triib.settings.matrix_connectable_only;
        if filtered {
            component::empty_state(
                Icon::FilterAlt,
                fl!("matrix-nothing-shown"),
                fl!("matrix-nothing-shown-note"),
            )
        } else {
            component::empty_state(Icon::GridOn, fl!("matrix-empty"), fl!("matrix-empty-note"))
        }
    } else {
        let grid = grid(triib, &talkers, &listeners);
        let status = status(triib, &grid, &talkers, &listeners);
        column![
            // A press on the matrix's lines is the matrix's; one on the
            // space around them clears the selection.
            mouse_area(
                container(with_format_menu(triib, grid::matrix(grid, triib.hover)))
                    .center_x(Fill)
                    .height(if cramped {
                        Length::Fixed(grid::LEAST_HEIGHT)
                    } else {
                        Length::Fill
                    })
            )
            .on_press(Message::SelectionCleared),
            status,
            legend()
        ]
        .spacing(12)
        .into()
    };
    let view = column![filters(triib, hidden), body]
        .spacing(16)
        .padding([16, 24])
        .width(Fill);
    if cramped {
        component::scroll(view).height(Fill).into()
    } else {
        view.height(Fill).into()
    }
}

/// The streams shown, and whether only connectable ones are, with how
/// many that hides.
fn filters(triib: &Triib, hidden: Option<usize>) -> Element<'_, Message> {
    let streams = triib.settings.matrix_streams;
    let choice = |label: String, value: Streams| {
        button::button(Kind::Filled, label)
            .size(Size::ExtraSmall)
            .selected(streams == value)
            .on_press(Message::MatrixStreams(value))
    };
    let mut filters: Vec<Element<'_, Message>> = vec![
        component::connected(vec![
            choice(fl!("matrix-all-streams"), Streams::All),
            choice(fl!("netmap-audio"), Streams::Audio),
            choice(fl!("entity-media-clock"), Streams::Clock),
        ]),
        // A secondary option, so a quiet tonal fill when on rather than
        // the stream choices' strong one.
        if hidden.is_some() {
            button::with_icon(Kind::Tonal, Icon::FilterAlt, fl!("matrix-connectable-only"))
        } else {
            button::with_icon(
                Kind::Filled,
                Icon::FilterAlt,
                fl!("matrix-connectable-only"),
            )
            .selected(false)
        }
        .size(Size::ExtraSmall)
        .on_press(Message::MatrixConnectableOnlyToggled)
        .into(),
    ];
    if let Some(hidden) = hidden {
        filters.push(
            styled(
                match hidden {
                    0 => fl!("matrix-none-hidden"),
                    hidden => fl!("matrix-hidden", count = hidden),
                },
                Type::BodyMedium,
            )
            .style(style::on_surface_variant)
            .into(),
        );
    }
    dir::row(filters)
        .spacing(12)
        .align_y(Center)
        .wrap()
        .vertical_spacing(8)
        .into()
}

/// The talkers or listeners with streams the search and the stream filter
/// let through, by name.
fn groups(triib: &Triib, side: Side) -> Vec<Group<'_>> {
    let input = side == Side::Listener;
    let search = triib.search_text();
    let streams = triib.settings.matrix_streams;
    triib
        .shown_entities()
        .into_iter()
        .filter_map(|entity| {
            let entity_id = entity.entity_id();
            let model = triib.models.get(&entity_id)?;
            // An entity found by its own name shows all its streams.
            let whole = search.is_empty() || triib.entity_matches(entity, &search);
            let shown: Vec<Stream<'_>> = model
                .streams(input)
                .filter(|stream| whole || app::stream_matches(model, stream, &search))
                .filter(|stream| match streams {
                    Streams::All => true,
                    Streams::Audio => !stream.is_media_clock(),
                    Streams::Clock => stream.is_media_clock(),
                })
                .map(|stream| Stream {
                    entity_id,
                    index: stream.index,
                    name: model
                        .name_of(stream.descriptor_type, stream.index)
                        .map_or_else(
                            || fl!("stream-numbered", index = stream.index),
                            str::to_owned,
                        ),
                    format: stream.current_format,
                    supported: if input {
                        stream.formats().collect()
                    } else {
                        Vec::new()
                    },
                    model,
                    host: triib.is_host(entity_id),
                })
                .collect();
            (!shown.is_empty()).then(|| Group {
                entity_id,
                name: triib.entity_name(entity),
                tag: tag(entity),
                streams: shown,
            })
        })
        .collect()
}

/// The last two octets of the entity's MAC address.
pub(crate) fn tag(entity: &DiscoveredEntity) -> String {
    let [.., fifth, sixth] = entity.mac.0;
    format!("{fifth:02x}:{sixth:02x}")
}

/// Leaves out the streams that nothing on the other side could connect
/// to, keeping bound ones.
fn connectable_only<'a>(talkers: &mut Vec<Group<'a>>, listeners: &mut Vec<Group<'a>>) {
    let reaches = |output: &Stream<'_>, input: &Stream<'_>| {
        input.bound_to() == Some(output.key())
            || (!input.own(output)
                && fit(output.format, input.format, input.supported.iter().copied())
                    != Fit::Incompatible)
    };
    let outputs: Vec<(EntityId, u16)> = talkers
        .iter()
        .flat_map(|group| &group.streams)
        .filter(|output| {
            listeners
                .iter()
                .flat_map(|group| &group.streams)
                .any(|input| reaches(output, input))
        })
        .map(Stream::key)
        .collect();
    let inputs: Vec<(EntityId, u16)> = listeners
        .iter()
        .flat_map(|group| &group.streams)
        .filter(|input| {
            talkers
                .iter()
                .flat_map(|group| &group.streams)
                .any(|output| reaches(output, input))
        })
        .map(Stream::key)
        .collect();
    for (groups, kept) in [(talkers, outputs), (listeners, inputs)] {
        for group in groups.iter_mut() {
            group.streams.retain(|stream| kept.contains(&stream.key()));
        }
        groups.retain(|group| !group.streams.is_empty());
    }
}

/// A stream format in a few characters, such as "48k 8ch" or "48k CRF".
pub(crate) fn short(format: StreamFormat) -> String {
    let rate = format.sample_rate().map(|hertz| {
        if hertz.is_multiple_of(1000) {
            format!("{}k", hertz / 1000)
        } else {
            format!("{:.1}k", hertz as f32 / 1000.0)
        }
    });
    if format.is_clock() {
        return rate.map_or_else(|| "CRF".to_owned(), |rate| format!("{rate} CRF"));
    }
    match (rate, format.channels()) {
        (Some(rate), Some(channels)) => format!("{rate} {channels}ch"),
        (Some(rate), None) => rate,
        (None, Some(channels)) => format!("{channels}ch"),
        (None, None) => String::new(),
    }
}

/// The streams one column or row stands for: one stream, or every stream
/// of a collapsed talker or listener. `None` for an expanded one.
type Members<'s, 'a> = Option<Vec<&'s Stream<'a>>>;

fn lines<'s, 'a>(
    triib: &Triib,
    groups: &'s [Group<'a>],
    side: Side,
) -> Vec<(Heading, Members<'s, 'a>)> {
    let mut lines = Vec::new();
    for group in groups {
        let collapsed = triib
            .settings
            .matrix_collapsed
            .contains(side, group.entity_id);
        lines.push((
            Heading {
                line: Line::Entity(group.entity_id),
                name: group.name.clone(),
                detail: group.tag.clone(),
                collapsed: Some(collapsed),
            },
            collapsed.then(|| group.streams.iter().collect()),
        ));
        if collapsed {
            continue;
        }
        for stream in &group.streams {
            lines.push((
                Heading {
                    line: Line::Stream(stream.entity_id, stream.index),
                    name: stream.name.clone(),
                    detail: short(stream.format),
                    collapsed: None,
                },
                Some(vec![stream]),
            ));
        }
    }
    lines
}

fn grid(triib: &Triib, talkers: &[Group<'_>], listeners: &[Group<'_>]) -> Grid {
    let (columns, outputs): (Vec<_>, Vec<_>) =
        lines(triib, talkers, Side::Talker).into_iter().unzip();
    let (rows, inputs): (Vec<_>, Vec<_>) =
        lines(triib, listeners, Side::Listener).into_iter().unzip();
    let mut cells = Vec::with_capacity(rows.len() * columns.len());
    for inputs in &inputs {
        for outputs in &outputs {
            cells.push(match (inputs, outputs) {
                (Some(inputs), Some(outputs)) => match (inputs.as_slice(), outputs.as_slice()) {
                    ([input], [output]) if input.own(output) => Cell::Own,
                    ([input], [output]) => {
                        let (state, action) = assess(triib, output, input);
                        Cell::Stream { state, action }
                    }
                    _ => {
                        let count = inputs
                            .iter()
                            .filter(|input| {
                                input.bound_to().is_some_and(|bound| {
                                    outputs.iter().any(|output| output.key() == bound)
                                })
                            })
                            .count();
                        let same = inputs.iter().all(|input| {
                            !input.host
                                && outputs
                                    .iter()
                                    .all(|output| output.entity_id == input.entity_id)
                        });
                        if same && count == 0 {
                            Cell::Own
                        } else {
                            Cell::Count(count)
                        }
                    }
                },
                _ => Cell::Blank,
            });
        }
    }
    Grid {
        columns,
        rows,
        cells,
        selected: triib.selected,
    }
}

/// How `output` and `input` stand, and what clicking their cell does.
fn assess(triib: &Triib, output: &Stream<'_>, input: &Stream<'_>) -> (State, Option<Action>) {
    let talker = output.key();
    let listener = input.key();
    let bound_here = input.bound_to() == Some(talker);
    let fits = fit(output.format, input.format, input.supported.iter().copied());
    let state = if bound_here {
        let info = input
            .model
            .stream_info(DescriptorType::STREAM_INPUT, input.index);
        match (fits, info) {
            (Fit::Matches, Some(info)) if info.talker_failed() => State::Trouble,
            (Fit::Matches, Some(info)) if !info.settled() => State::Waiting,
            (Fit::Matches, _) => State::Connected,
            _ => State::Trouble,
        }
    } else {
        match fits {
            Fit::Matches => State::Open,
            Fit::InputMustChange => State::Change,
            Fit::Incompatible => State::Incompatible,
        }
    };
    let pending = triib.pending.iter().find_map(|action| match *action {
        Action::Connect {
            talker: target,
            listener: pending,
        } if pending == listener => Some(target == talker),
        Action::Disconnect { listener: pending } if pending == listener => Some(bound_here),
        _ => None,
    });
    let chained = triib.waiting(|action| *action == Action::Connect { talker, listener });
    match pending {
        Some(true) => (State::Working, None),
        // Its formats are changing before it connects.
        None if chained => (State::Working, None),
        // Another change to this input is on its way.
        Some(false) => (state, None),
        None if bound_here => (state, Some(Action::Disconnect { listener })),
        None if fits == Fit::Incompatible => (state, None),
        None => (state, Some(Action::Connect { talker, listener })),
    }
}

/// The matrix with the format menu over the cell it was opened from,
/// while it is open.
fn with_format_menu<'a>(
    triib: &'a Triib,
    matrix: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let matrix = matrix.into();
    let Some(menu) = triib.format_menu else {
        return matrix;
    };
    let entries = format_entries(triib, menu);
    if entries.is_empty() {
        return matrix;
    }
    let anchor = space().width(menu.cell.width).height(menu.cell.height);
    let surface = dropdown::surface(entries, 0.0);
    let opened = popover(anchor, Some(surface), Message::MatrixFormatsClosed).close_on_choice();
    stack![matrix, pin(opened).x(menu.cell.x).y(menu.cell.y)].into()
}

/// The format menu's entries: the talker's format, which only the input
/// changes to, then the other formats both take, which both change to,
/// each connecting once the formats are set; then connecting as they are.
fn format_entries(triib: &Triib, menu: FormatMenu) -> Vec<Entry<Message>> {
    let FormatMenu {
        talker, listener, ..
    } = menu;
    let stream = |(entity, index): (EntityId, u16), input: bool| {
        triib
            .models
            .get(&entity)?
            .streams(input)
            .find(|stream| stream.index == index)
            .map(|stream| (stream.current_format, stream.formats().collect::<Vec<_>>()))
    };
    let (Some((sent, mut sendable)), Some((set, takes))) =
        (stream(talker, false), stream(listener, true))
    else {
        return Vec::new();
    };
    // The talker's own format first, then its others at the same rate:
    // another rate is a change of the entities' sampling rate, which can
    // be refused or move every stream they have.
    sendable.retain(|format| *format != sent && format.sample_rate() == sent.sample_rate());
    sendable.insert(0, sent);
    // Each format the output can send, with what the input is set to for
    // it: as it is when it takes it already, else the same format where
    // it lists it, else the one it lists that takes it.
    let shared = sendable.into_iter().filter_map(|format| {
        if format.suits(set) {
            return Some((format, set));
        }
        takes
            .iter()
            .copied()
            .find(|taken| *taken == format)
            .or_else(|| takes.iter().copied().find(|taken| format.suits(*taken)))
            .map(|taken| (format, taken))
    });
    // A talker already streaming to another listener keeps its format.
    let busy = streams_elsewhere(triib, talker, listener);
    let change = |entity: EntityId, descriptor_type: DescriptorType, index: u16, format| {
        Action::SetStreamFormat {
            entity,
            descriptor_type,
            index,
            format,
        }
    };
    let connect = Action::Connect { talker, listener };
    let mut entries = Vec::new();
    let mut both = false;
    let mut held = false;
    for (format, taken) in shared {
        let mut steps = Vec::new();
        if format == sent {
            entries.push(Entry::heading(fl!("matrix-format-input")));
        } else if busy {
            held = true;
            continue;
        } else {
            if !both {
                entries.push(Entry::heading(fl!("matrix-format-both")));
                both = true;
            }
            steps.push(change(
                talker.0,
                DescriptorType::STREAM_OUTPUT,
                talker.1,
                format,
            ));
        }
        if taken != set {
            steps.push(change(
                listener.0,
                DescriptorType::STREAM_INPUT,
                listener.1,
                taken,
            ));
        }
        steps.push(connect);
        entries.push(Entry::item(format.to_string(), Message::ActInTurn(steps)));
    }
    if held {
        entries.push(Entry::note(fl!("matrix-format-talker-busy")));
    }
    entries.push(Entry::divider());
    entries.push(Entry::item(
        fl!("matrix-format-anyway"),
        Message::Act(connect),
    ));
    entries
}

/// The format menu of the first cell whose input must change, as a click
/// on it opens it, for pictures.
#[cfg(test)]
pub(crate) fn first_format_menu(triib: &Triib) -> Option<FormatMenu> {
    let talkers = groups(triib, Side::Talker);
    let listeners = groups(triib, Side::Listener);
    let built = grid(triib, &talkers, &listeners);
    let columns = built.columns.len();
    built
        .cells
        .iter()
        .enumerate()
        .find_map(|(at, cell)| match *cell {
            Cell::Stream {
                state: State::Change,
                action: Some(Action::Connect { talker, listener }),
            } => Some(FormatMenu {
                talker,
                listener,
                cell: Rectangle {
                    x: (at % columns) as f32 * grid::CELL,
                    y: grid::HEADER + (at / columns) as f32 * grid::CELL,
                    width: grid::CELL,
                    height: grid::CELL,
                },
            }),
            _ => None,
        })
}

/// Whether the talker's output streams to a listener other than
/// `listener`, as its bindings or its own count tell.
fn streams_elsewhere(triib: &Triib, talker: (EntityId, u16), listener: (EntityId, u16)) -> bool {
    let bound = triib.models.iter().any(|(entity, model)| {
        model.streams(true).any(|input| {
            (*entity, input.index) != listener
                && model
                    .binding(input.index)
                    .and_then(|binding| binding.talker_stream())
                    == Some(talker)
        })
    });
    let counted = triib
        .models
        .get(&talker.0)
        .and_then(|model| model.tx_state(talker.1))
        .is_some_and(|state| state.connection_count > 0);
    bound || counted
}

/// A column's or row's line found among the talkers or listeners.
enum Found<'s, 'a> {
    Group(&'s Group<'a>),
    Stream(&'s Group<'a>, &'s Stream<'a>),
}

fn find<'s, 'a>(groups: &'s [Group<'a>], line: Line) -> Option<Found<'s, 'a>> {
    match line {
        Line::Entity(entity_id) => groups
            .iter()
            .find(|group| group.entity_id == entity_id)
            .map(Found::Group),
        Line::Stream(entity_id, index) => groups
            .iter()
            .find(|group| group.entity_id == entity_id)
            .and_then(|group| {
                group
                    .streams
                    .iter()
                    .find(|stream| stream.index == index)
                    .map(|stream| Found::Stream(group, stream))
            }),
    }
}

impl Found<'_, '_> {
    fn label(&self) -> String {
        match self {
            Found::Group(group) => group.label(),
            Found::Stream(group, stream) => format!("{}, {}", group.label(), stream.name),
        }
    }
}

/// What the status line says about the pointer's place.
struct Said {
    glyph: (Icon, bool),
    color: Color,
    title: String,
    detail: String,
}

/// What the pointer is over, in words: a cell's talker and listener and
/// what clicking does, or a heading's stream.
fn status<'a>(
    triib: &Triib,
    grid: &Grid,
    talkers: &[Group<'_>],
    listeners: &[Group<'_>],
) -> Element<'a, Message> {
    let said = say(triib, grid, talkers, listeners);
    let color = said.color;
    let glyph = if said.glyph.1 {
        icon::filled(said.glyph.0, 20)
    } else {
        icon::icon(said.glyph.0, 20)
    };
    container(
        row![
            glyph.color(color),
            // On a narrow window what does not fit is cut off.
            styled(said.title, Type::BodyMedium).wrapping(Wrapping::None),
            styled(said.detail, Type::BodyMedium)
                .style(style::on_surface_variant)
                .wrapping(Wrapping::None),
        ]
        .spacing(10)
        .align_y(Center),
    )
    .padding([0, 16])
    .height(STATUS_HEIGHT)
    .width(Fill)
    .clip(true)
    .align_x(dir::horizontal_start())
    .align_y(Center)
    .style(|theme: &Theme| container::Style {
        background: Some(Scheme::of(theme).surface_container.into()),
        border: iced::border::rounded(shape::MEDIUM),
        ..container::Style::default()
    })
    .into()
}

fn say(triib: &Triib, grid: &Grid, talkers: &[Group<'_>], listeners: &[Group<'_>]) -> Said {
    let scheme = Scheme::of(&triib.theme());
    let info = |title: String, detail: String| Said {
        glyph: (Icon::Info, false),
        color: scheme.on_surface_variant,
        title,
        detail,
    };
    let hover = triib.hover;
    let column = hover.column.and_then(|line| find(talkers, line));
    let row = hover.row.and_then(|line| find(listeners, line));
    let cell = hover
        .row
        .and_then(|line| grid.row_of(line))
        .zip(hover.column.and_then(|line| grid.column_of(line)))
        .map(|(row, column)| grid.cell(row, column));
    match (column, row, cell) {
        (Some(Found::Stream(talker, output)), Some(Found::Stream(listener, input)), _) => {
            let route = format!(
                "{}, {}  {}  {}, {}",
                talker.label(),
                output.name,
                crate::i18n::arrow(&talker.label()),
                listener.label(),
                input.name
            );
            if input.own(output) {
                return info(route, fl!("matrix-own"));
            }
            let (state, action) = assess(triib, output, input);
            let (sent, set) = (output.format.to_string(), input.format.to_string());
            let detail = match state {
                State::Working => fl!("matrix-working"),
                _ if action.is_none() && state != State::Incompatible => {
                    fl!("matrix-waiting-change")
                }
                State::Connected => fl!("matrix-connected"),
                State::Waiting => fl!("matrix-bound-waiting"),
                State::Trouble => match input
                    .model
                    .stream_info(DescriptorType::STREAM_INPUT, input.index)
                {
                    Some(info) if info.talker_failed() => fl!(
                        "matrix-bound-failed",
                        reason = crate::describe::reservation_failure(info)
                    ),
                    _ => fl!("matrix-bound-formats-differ", sent = sent, set = set),
                },
                State::Open => fl!("matrix-formats-match", format = sent),
                State::Change => fl!("matrix-format-must-change", sent = sent, set = set),
                State::Incompatible => fl!("matrix-incompatible", sent = sent, set = set),
            };
            Said {
                glyph: state.icon(),
                color: state.color(&scheme),
                title: route,
                detail,
            }
        }
        (Some(column), Some(row), Some(Cell::Own)) => info(
            format!(
                "{}  {}  {}",
                column.label(),
                crate::i18n::arrow(&column.label()),
                row.label()
            ),
            fl!("matrix-own"),
        ),
        (Some(column), Some(row), Some(Cell::Count(count))) => info(
            format!(
                "{}  {}  {}",
                column.label(),
                crate::i18n::arrow(&column.label()),
                row.label()
            ),
            if count == 0 {
                fl!("matrix-group-none")
            } else {
                fl!("matrix-group-connected", count = count)
            },
        ),
        (column, row, _) => {
            // A heading, or a blank cell, said as its stream's heading.
            let heading = match (row, column) {
                (Some(row @ Found::Stream(..)), _) | (Some(row), None) => {
                    Some((row, Side::Listener))
                }
                (_, Some(column)) => Some((column, Side::Talker)),
                (None, None) => None,
            };
            match heading {
                Some((Found::Group(group), side)) => {
                    let count = group.streams.len();
                    let collapsed = triib
                        .settings
                        .matrix_collapsed
                        .contains(side, group.entity_id);
                    info(
                        group.label(),
                        match (side, collapsed) {
                            (Side::Talker, true) => fl!("matrix-outputs-expand", count = count),
                            (Side::Talker, false) => {
                                fl!("matrix-outputs-collapse", count = count)
                            }
                            (Side::Listener, true) => fl!("matrix-inputs-expand", count = count),
                            (Side::Listener, false) => {
                                fl!("matrix-inputs-collapse", count = count)
                            }
                        },
                    )
                }
                Some((Found::Stream(group, stream), side)) => {
                    let format = stream.format.to_string();
                    let mut detail = fl!("matrix-stream-format", format = format.as_str());
                    if side == Side::Listener
                        && let Some(state) = stream
                            .model
                            .streams(true)
                            .find(|descriptor| descriptor.index == stream.index)
                            .and_then(|descriptor| {
                                crate::describe::stream_state(
                                    triib,
                                    stream.entity_id,
                                    stream.model,
                                    &descriptor,
                                )
                            })
                    {
                        detail = fl!(
                            "matrix-stream-format-state",
                            format = format.as_str(),
                            state = state
                        );
                    }
                    info(
                        format!("{}, {}", group.label(), stream.name),
                        fl!(
                            "matrix-stream-inspect",
                            detail = detail,
                            entity = group.name.clone()
                        ),
                    )
                }
                None => info(fl!("matrix-point"), fl!("matrix-point-note")),
            }
        }
    }
}

/// What each cell icon means.
fn legend<'a>() -> Element<'a, Message> {
    let entry = |state: State, label: String| -> Element<'a, Message> {
        let (glyph, filled) = state.icon();
        let glyph = if filled {
            icon::filled(glyph, 20)
        } else {
            icon::icon(glyph, 20)
        };
        row![
            glyph.style(move |theme: &Theme| iced::widget::text::Style {
                color: Some(state.color(&Scheme::of(theme))),
            }),
            styled(label, Type::BodySmall).style(style::on_surface_variant),
        ]
        .spacing(6)
        .align_y(Center)
        .into()
    };
    row![
        entry(State::Connected, fl!("netmap-connected")),
        entry(State::Waiting, fl!("matrix-legend-waiting")),
        entry(State::Trouble, fl!("matrix-legend-trouble")),
        entry(State::Open, fl!("matrix-legend-open")),
        entry(State::Change, fl!("matrix-legend-change")),
        entry(State::Incompatible, fl!("matrix-legend-incompatible")),
    ]
    .spacing(20)
    .wrap()
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::Settings;

    /// One of this computer's entities connects to itself, as it takes
    /// the binding; any other's own cells stay blank.
    #[test]
    fn this_computers_entities_connect_to_themselves() {
        use triib_endpointd::status::{DaemonStatus, EndpointStatus};

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
        let host = crate::view::tests::WIRED_ESP;
        triib.endpoints = Some(DaemonStatus {
            pid: 1,
            started: 0,
            interface: "enp6s0".into(),
            gptp: String::new(),
            foreign_mrp: false,
            config: None,
            endpoints: vec![EndpointStatus {
                entity_id: host.to_string(),
                name: "Host entity 1".into(),
                state: "waiting".into(),
                streams: Vec::new(),
            }],
        });
        let talkers = groups(&triib, Side::Talker);
        let listeners = groups(&triib, Side::Listener);
        let built = grid(&triib, &talkers, &listeners);
        let (mut own, mut offered) = (0, 0);
        for (row, row_heading) in built.rows.iter().enumerate() {
            for (column, column_heading) in built.columns.iter().enumerate() {
                if let (Line::Stream(listener, _), Line::Stream(talker, _)) =
                    (row_heading.line, column_heading.line)
                    && listener == talker
                {
                    let cell = built.cell(row, column);
                    if listener == host {
                        assert_ne!(cell, Cell::Own, "{row}, {column}");
                        offered += 1;
                    } else {
                        own += usize::from(cell == Cell::Own);
                    }
                }
            }
        }
        assert!(offered > 0 && own > 0, "{offered} {own}");
    }

    /// A media clock output sending its clock as audio, as the Mac's
    /// Media Clock Output Streams 2 and 3 send one channel of AAF, is a
    /// media clock stream: the matrix's media clock streams show it, its
    /// audio streams do not.
    #[test]
    fn a_media_clock_output_sending_audio_is_a_media_clock_stream() {
        use crate::settings::Streams;

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
        let shows = |triib: &Triib| {
            groups(triib, Side::Talker)
                .iter()
                .flat_map(|group| &group.streams)
                .find(|stream| stream.name == "Media Clock Output Stream 2")
                .map(|stream| stream.format)
        };
        let format = shows(&triib).expect("the Mac's second media clock output");
        assert!(!format.is_clock(), "sending AAF");
        triib.settings.matrix_streams = Streams::Clock;
        assert!(shows(&triib).is_some());
        triib.settings.matrix_streams = Streams::Audio;
        assert!(shows(&triib).is_none());
    }

    #[test]
    fn entities_do_not_connect_to_themselves() {
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
        let triib = Triib::sample(Settings::default(), interface, entities, models);
        let talkers = groups(&triib, Side::Talker);
        let listeners = groups(&triib, Side::Listener);
        let built = grid(&triib, &talkers, &listeners);
        let mut own = 0;
        for (row, row_heading) in built.rows.iter().enumerate() {
            for (column, column_heading) in built.columns.iter().enumerate() {
                let cell = built.cell(row, column);
                match (row_heading.line, column_heading.line) {
                    (Line::Stream(listener, _), Line::Stream(talker, _)) => {
                        // Nothing on the bench is bound to itself.
                        assert_eq!(cell == Cell::Own, listener == talker, "{row}, {column}");
                        own += usize::from(cell == Cell::Own);
                    }
                    _ => assert_ne!(cell, Cell::Own),
                }
            }
        }
        assert!(own > 0, "each entity meets its own streams");
    }

    /// A cell whose input must change first offers the talker's format,
    /// which only the input changes to, other formats both take, then
    /// connecting as they are; a choice changes formats before connecting.
    #[test]
    fn a_cell_whose_input_must_change_offers_formats() {
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
        let triib = Triib::sample(Settings::default(), interface, entities, models);
        let talkers = groups(&triib, Side::Talker);
        let listeners = groups(&triib, Side::Listener);
        let built = grid(&triib, &talkers, &listeners);
        let (talker, listener) = built
            .cells
            .iter()
            .find_map(|cell| match *cell {
                Cell::Stream {
                    state: State::Change,
                    action: Some(Action::Connect { talker, listener }),
                } => Some((talker, listener)),
                _ => None,
            })
            .expect("the bench has a cell whose input must change");
        let entries = format_entries(
            &triib,
            FormatMenu {
                talker,
                listener,
                cell: Rectangle::default(),
            },
        );
        let connect = Action::Connect { talker, listener };
        assert!(matches!(&entries[0], Entry::Heading(_)));
        let Entry::Item {
            message: Message::ActInTurn(steps),
            ..
        } = &entries[1]
        else {
            panic!("the talker's format comes first");
        };
        assert!(matches!(
            steps[0],
            Action::SetStreamFormat { entity, descriptor_type: DescriptorType::STREAM_INPUT, index, .. }
                if (entity, index) == listener
        ));
        assert_eq!(steps.last(), Some(&connect));
        assert!(matches!(
            entries.last(),
            Some(Entry::Item { message: Message::Act(action), .. }) if *action == connect
        ));
        // Any other format changes the talker first.
        for entry in &entries[2..] {
            if let Entry::Item {
                message: Message::ActInTurn(steps),
                ..
            } = entry
            {
                assert!(matches!(
                    steps[0],
                    Action::SetStreamFormat { entity, descriptor_type: DescriptorType::STREAM_OUTPUT, index, .. }
                        if (entity, index) == talker
                ));
                assert_eq!(steps.last(), Some(&connect));
            }
        }
    }

    #[test]
    fn formats_shorten() {
        // AAF, 32-bit integer, 48 kHz, 8 channels of 32 bits, 6 per frame.
        let aaf = StreamFormat(0x0205_0220_0200_6000);
        assert_eq!(short(aaf), "48k 8ch");
        // CRF audio sample, 48 kHz.
        let crf = StreamFormat(0x0410_6001_0000_bb80);
        assert_eq!(short(crf), "48k CRF");
    }
}
