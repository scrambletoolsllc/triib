//! The matrix as one widget, so its headings stay in place while the
//! cells scroll under them: column headings slanted at 45° along the top,
//! row headings on the right.

use std::cell::{Cell as Memo, RefCell};
use std::collections::HashMap;
use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_4};
use std::hash::{DefaultHasher, Hash, Hasher};

use iced::advanced::graphics::geometry::Renderer as _;
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::text::{Alignment, Shaping};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell};
use iced::alignment::Vertical;
use iced::font::Weight;
use iced::mouse::{self, Cursor};
use iced::widget::canvas::{Cache, Frame, Path, Stroke, Text};
use iced::widget::text::LineHeight;
use iced::{
    Background, Color, Element, Event, Font, Length, Pixels, Point, Rectangle, Size, Theme, Vector,
    border, keyboard,
};
use scramble_ui::font::{ICONS, ICONS_FILLED, TEXT};
use scramble_ui::icon::Icon;
use scramble_ui::{Scheme, blend, faded};

use super::{Cell, FormatMenu, Grid, Hover, Line, Side, State};
use crate::app::Message;
use crate::network::Action;
use crate::text::{Fitted, fit, measure};

/// A cell's width and height.
pub(super) const CELL: f32 = 40.0;
/// Height of the slanted column headings.
pub(super) const HEADER: f32 = 176.0;
/// The least height the grid takes: its headings and two rows.
pub const LEAST_HEIGHT: f32 = HEADER + 2.0 * CELL;
/// Width of the row headings.
const ROW_HEADING: f32 = 300.0;
/// The narrowest the row headings get, on a narrow window, which gives
/// them up to half its width.
const NARROWEST_HEADING: f32 = 150.0;
/// The longest a slanted heading runs.
const SLANT_MAX: f32 = 220.0;
/// How far along a talker's or listener's heading its chevron reaches.
const CHEVRON_REACH: f32 = 24.0;
/// How far back along its slant, toward the grid, a talker's heading is
/// drawn, so it starts about where a listener's does beside its row.
const TALKER_PULL: f32 = 5.0;
const RADIUS: f32 = 16.0;
const SCROLLBAR: f32 = 6.0;
const SCROLLBAR_MARGIN: f32 = 2.0;
/// How far past a scrollbar a press still grabs it.
const SCROLLBAR_REACH: f32 = 4.0;
const THUMB_MIN: f32 = 24.0;
/// Pixels a wheel line scrolls, as in iced's scrollables.
const LINE_PIXELS: f32 = 60.0;
const MONOSPACE: Font = Font::MONOSPACE;
const BOLD: Font = Font {
    weight: Weight::Bold,
    ..TEXT
};

pub struct Matrix {
    grid: Grid,
    hover: Hover,
    /// A hash of the grid, telling when to draw it again.
    signature: u64,
}

pub fn matrix(grid: Grid, hover: Hover) -> Matrix {
    let mut hasher = DefaultHasher::new();
    grid.hash(&mut hasher);
    Matrix {
        signature: hasher.finish(),
        grid,
        hover,
    }
}

#[derive(Default)]
struct Memory {
    /// How far the cells are scrolled, before rounding to whole pixels.
    offset: Vector,
    drag: Option<Drag>,
    modifiers: keyboard::Modifiers,
    header: Cache,
    cells: Cache,
    headings: Cache,
    /// The hashes the caches were drawn for: the grid and colors, and
    /// the header's place.
    drawn: Memo<(u64, u64)>,
    /// Headings cut to fit, by column or row.
    fitted: RefCell<HashMap<(Side, usize), Fitted>>,
}

/// A scrollbar thumb being dragged.
#[derive(Debug, Clone, Copy)]
struct Drag {
    vertical: bool,
    /// Where along the thumb it was grabbed.
    grab: f32,
}

/// Where the parts of the matrix are, from its top left corner.
#[derive(Debug, Clone, Copy)]
struct Parts {
    /// All the cells.
    content: Size,
    /// The cells in view.
    view: Size,
    /// The farthest the cells scroll.
    max: Vector,
    /// The width of the row headings.
    heading: f32,
}

impl Parts {
    fn new(grid: &Grid, bounds: Size) -> Self {
        let content = Size::new(
            grid.columns.len() as f32 * CELL,
            grid.rows.len() as f32 * CELL,
        );
        // Full width when everything fits, else up to half the room.
        let heading = if bounds.width >= content.width + ROW_HEADING {
            ROW_HEADING
        } else {
            ROW_HEADING.min((bounds.width / 2.0).max(NARROWEST_HEADING))
        };
        let view = Size::new(
            content.width.min((bounds.width - heading).max(0.0)),
            content.height.min((bounds.height - HEADER).max(0.0)),
        );
        Self {
            content,
            view,
            heading,
            max: Vector::new(
                (content.width - view.width).max(0.0),
                (content.height - view.height).max(0.0),
            ),
        }
    }

    fn clamp(&self, offset: Vector) -> Vector {
        Vector::new(
            offset.x.clamp(0.0, self.max.x),
            offset.y.clamp(0.0, self.max.y),
        )
    }

    /// The offset to draw at, in whole pixels so lines stay sharp.
    fn drawn(&self, offset: Vector) -> Vector {
        let offset = self.clamp(offset);
        Vector::new(offset.x.round(), offset.y.round())
    }

    /// Where the row headings start.
    fn headings_x(&self) -> f32 {
        self.view.width
    }

    /// A scrollbar's track and thumb, when the cells overflow that way.
    fn scrollbar(&self, vertical: bool, offset: Vector) -> Option<(Rectangle, Rectangle)> {
        if vertical {
            (self.max.y > 0.0).then(|| {
                let track = Rectangle {
                    x: self.headings_x() + self.heading - SCROLLBAR - SCROLLBAR_MARGIN,
                    y: HEADER + SCROLLBAR_MARGIN,
                    width: SCROLLBAR,
                    height: self.view.height - 2.0 * SCROLLBAR_MARGIN,
                };
                let length = thumb_length(track.height, self.view.height, self.content.height);
                let y = track.y + (track.height - length) * offset.y / self.max.y;
                (
                    track,
                    Rectangle {
                        y,
                        height: length,
                        ..track
                    },
                )
            })
        } else {
            (self.max.x > 0.0).then(|| {
                let track = Rectangle {
                    x: SCROLLBAR_MARGIN,
                    y: HEADER + self.view.height - SCROLLBAR - SCROLLBAR_MARGIN,
                    width: self.view.width - 2.0 * SCROLLBAR_MARGIN,
                    height: SCROLLBAR,
                };
                let length = thumb_length(track.width, self.view.width, self.content.width);
                let x = track.x + (track.width - length) * offset.x / self.max.x;
                (
                    track,
                    Rectangle {
                        x,
                        width: length,
                        ..track
                    },
                )
            })
        }
    }

    /// Whether a column's cells are at least partly in view.
    fn column_in_view(&self, column: usize, offset: Vector) -> bool {
        let x = column as f32 * CELL - offset.x;
        x < self.view.width && x + CELL > 0.0
    }

    /// The rows at least partly in view.
    fn rows_in_view(&self, rows: usize, offset: Vector) -> std::ops::Range<usize> {
        let first = (offset.y / CELL).floor() as usize;
        let last = ((offset.y + self.view.height) / CELL).ceil() as usize;
        first.min(rows)..last.min(rows)
    }
}

fn thumb_length(track: f32, view: f32, content: f32) -> f32 {
    (track * view / content).max(THUMB_MIN).min(track)
}

/// What a point of the matrix is over.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Target {
    Nothing,
    Cell(usize, usize),
    Column(usize),
    Row(usize),
    /// The chevron of a talker's column or a listener's row.
    Chevron(Side, usize),
    Scrollbar {
        vertical: bool,
    },
}

impl Matrix {
    fn target(&self, parts: &Parts, offset: Vector, point: Point) -> Target {
        for vertical in [true, false] {
            if let Some((track, _)) = parts.scrollbar(vertical, offset) {
                let reach = Rectangle {
                    x: track.x - SCROLLBAR_REACH,
                    y: track.y - SCROLLBAR_REACH,
                    width: track.width + 2.0 * SCROLLBAR_REACH,
                    height: track.height + 2.0 * SCROLLBAR_REACH,
                };
                if reach.contains(point) {
                    return Target::Scrollbar { vertical };
                }
            }
        }
        let columns = self.grid.columns.len();
        let rows = self.grid.rows.len();
        if point.y < HEADER {
            // Each column's heading leans right as it rises.
            if point.x >= parts.headings_x() + parts.heading {
                return Target::Nothing;
            }
            let x = point.x + offset.x - (HEADER - point.y);
            if x < 0.0 {
                return Target::Nothing;
            }
            let column = (x / CELL) as usize;
            if column >= columns || !parts.column_in_view(column, offset) {
                return Target::Nothing;
            }
            if self.grid.columns[column].collapsed.is_some() {
                // How far up the slanted name the point is, from where
                // the name starts.
                let start = Point::new(column as f32 * CELL - offset.x + CELL, HEADER - 6.0);
                let along = ((point.x - start.x) - (point.y - start.y)) * FRAC_1_SQRT_2;
                if along < CHEVRON_REACH - TALKER_PULL {
                    return Target::Chevron(Side::Talker, column);
                }
            }
            return Target::Column(column);
        }
        let y = point.y - HEADER;
        if y >= parts.view.height {
            return Target::Nothing;
        }
        let row = ((y + offset.y) / CELL) as usize;
        if row >= rows {
            return Target::Nothing;
        }
        if point.x < parts.headings_x() {
            let column = ((point.x + offset.x) / CELL) as usize;
            if column < columns {
                return Target::Cell(row, column);
            }
        } else if point.x < parts.headings_x() + parts.heading {
            if self.grid.rows[row].collapsed.is_some()
                && point.x - parts.headings_x() < 8.0 + CHEVRON_REACH
            {
                return Target::Chevron(Side::Listener, row);
            }
            return Target::Row(row);
        }
        Target::Nothing
    }

    fn hover_of(&self, target: Target) -> Hover {
        let column = |index: usize| self.grid.columns.get(index).map(|heading| heading.line);
        let row = |index: usize| self.grid.rows.get(index).map(|heading| heading.line);
        match target {
            Target::Cell(row_index, column_index) => Hover {
                column: column(column_index),
                row: row(row_index),
            },
            Target::Column(index) | Target::Chevron(Side::Talker, index) => Hover {
                column: column(index),
                row: None,
            },
            Target::Row(index) | Target::Chevron(Side::Listener, index) => Hover {
                column: None,
                row: row(index),
            },
            Target::Nothing | Target::Scrollbar { .. } => Hover::default(),
        }
    }

    /// What pressing on `target` does, with the cells scrolled by `offset`.
    fn press(&self, target: Target, offset: Vector) -> Option<Message> {
        match target {
            Target::Cell(row, column) => match self.grid.cell(row, column) {
                // Formats to choose from before connecting.
                Cell::Stream {
                    state: State::Change,
                    action: Some(Action::Connect { talker, listener }),
                } => Some(Message::MatrixFormatsOpened(FormatMenu {
                    talker,
                    listener,
                    cell: Rectangle {
                        x: column as f32 * CELL - offset.x,
                        y: HEADER + row as f32 * CELL - offset.y,
                        width: CELL,
                        height: CELL,
                    },
                })),
                Cell::Stream {
                    action: Some(action),
                    ..
                } => Some(Message::Act(action)),
                _ => None,
            },
            Target::Chevron(side, index) => {
                let heading = match side {
                    Side::Talker => self.grid.columns.get(index)?,
                    Side::Listener => self.grid.rows.get(index)?,
                };
                Some(Message::MatrixCollapseToggled(side, heading.line.entity()))
            }
            Target::Column(column) => Some(Message::EntitySelected(
                self.grid.columns.get(column)?.line.entity(),
            )),
            Target::Row(row) => Some(Message::EntitySelected(
                self.grid.rows.get(row)?.line.entity(),
            )),
            Target::Nothing | Target::Scrollbar { .. } => None,
        }
    }
}

/// Moves the dragged thumb's grab point to `point`.
fn drag_to(parts: &Parts, state: &mut Memory, drag: Drag, point: Point) {
    let offset = parts.clamp(state.offset);
    let Some((track, thumb)) = parts.scrollbar(drag.vertical, offset) else {
        return;
    };
    if drag.vertical {
        let room = track.height - thumb.height;
        if room > 0.0 {
            let start = (point.y - drag.grab - track.y).clamp(0.0, room);
            state.offset.y = start / room * parts.max.y;
        }
    } else {
        let room = track.width - thumb.width;
        if room > 0.0 {
            let start = (point.x - drag.grab - track.x).clamp(0.0, room);
            state.offset.x = start / room * parts.max.x;
        }
    }
}

impl Widget<Message, Theme, iced::Renderer> for Matrix {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Memory>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Memory::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Fill)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // As wide as the cells and headings need, or the room there is,
        // and as tall as the rows need.
        let max = limits.max();
        let content = Size::new(
            self.grid.columns.len() as f32 * CELL + ROW_HEADING,
            self.grid.rows.len() as f32 * CELL + HEADER,
        );
        let width = content.width.min(max.width);
        // Never shorter than the headings and two rows, which the headings
        // would otherwise spill out of.
        let height = content
            .height
            .min(max.height)
            .max(LEAST_HEIGHT.min(content.height));
        layout::Node::new(Size::new(width, height))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<Memory>();
        let bounds = layout.bounds();
        let parts = Parts::new(&self.grid, bounds.size());
        state.offset = parts.clamp(state.offset);
        let point = cursor.position_in(bounds);
        match event {
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.modifiers = *modifiers;
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) if point.is_some() => {
                let (mut x, mut y) = match *delta {
                    mouse::ScrollDelta::Lines { x, y } => (x * LINE_PIXELS, y * LINE_PIXELS),
                    mouse::ScrollDelta::Pixels { x, y } => (x, y),
                };
                if state.modifiers.shift() && x == 0.0 {
                    (x, y) = (y, 0.0);
                }
                let before = state.offset;
                state.offset = parts.clamp(state.offset - Vector::new(x, y));
                if state.offset != before {
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(point) = point {
                    let offset = parts.drawn(state.offset);
                    let target = self.target(&parts, offset, point);
                    if let Target::Scrollbar { vertical } = target {
                        if let Some((_, thumb)) = parts.scrollbar(vertical, offset) {
                            let (along, start, length) = if vertical {
                                (point.y, thumb.y, thumb.height)
                            } else {
                                (point.x, thumb.x, thumb.width)
                            };
                            // A press off the thumb brings the thumb's
                            // middle to it.
                            let grab = if (start..start + length).contains(&along) {
                                along - start
                            } else {
                                length / 2.0
                            };
                            let drag = Drag { vertical, grab };
                            state.drag = Some(drag);
                            drag_to(&parts, state, drag, point);
                            shell.request_redraw();
                        }
                        shell.capture_event();
                    } else if target != Target::Nothing {
                        // Every press on the matrix's lines is its own, done
                        // or not; one on its empty corners passes through.
                        if let Some(message) = self.press(target, offset) {
                            shell.publish(message);
                        }
                        shell.capture_event();
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if state.drag.take().is_some() {
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if let (Some(drag), Some(position)) = (state.drag, cursor.position()) {
                    let point = Point::new(position.x - bounds.x, position.y - bounds.y);
                    drag_to(&parts, state, drag, point);
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            _ => {}
        }
        if let Event::Mouse(_) = event {
            // What the pointer is over, after any scrolling.
            let hover = match (point, state.drag) {
                (Some(point), None) => {
                    self.hover_of(self.target(&parts, parts.drawn(state.offset), point))
                }
                _ => Hover::default(),
            };
            if hover != self.hover {
                self.hover = hover;
                shell.publish(Message::MatrixHovered(hover));
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<Memory>();
        if state.drag.is_some() {
            return mouse::Interaction::Grabbing;
        }
        let bounds = layout.bounds();
        let Some(point) = cursor.position_in(bounds) else {
            return mouse::Interaction::default();
        };
        let parts = Parts::new(&self.grid, bounds.size());
        match self.target(&parts, parts.drawn(state.offset), point) {
            Target::Scrollbar { .. } => mouse::Interaction::Grab,
            target if self.press(target, Vector::ZERO).is_some() => mouse::Interaction::Pointer,
            _ => mouse::Interaction::default(),
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<Memory>();
        let bounds = layout.bounds();
        let parts = Parts::new(&self.grid, bounds.size());
        let offset = parts.drawn(state.offset);
        let colors = Colors::of(theme);
        // Each part's own layer kept to what is in view, as a scrolled
        // view would otherwise show the grid past its edge.
        let visible = |area: Rectangle| area.intersection(viewport).unwrap_or_default();

        // Draw the caches again for a new grid or theme, and the header
        // when it scrolls sideways.
        let content = {
            let mut hasher = DefaultHasher::new();
            self.signature.hash(&mut hasher);
            colors.hash(&mut hasher);
            parts.heading.to_bits().hash(&mut hasher);
            hasher.finish()
        };
        let header = {
            let mut hasher = DefaultHasher::new();
            content.hash(&mut hasher);
            offset.x.to_bits().hash(&mut hasher);
            parts.view.width.to_bits().hash(&mut hasher);
            hasher.finish()
        };
        let (drawn_content, drawn_header) = state.drawn.get();
        if drawn_content != content {
            state.cells.clear();
            state.headings.clear();
            state.fitted.borrow_mut().clear();
        }
        if drawn_header != header {
            state.header.clear();
        }
        state.drawn.set((content, header));

        let origin = Vector::new(bounds.x, bounds.y);
        renderer.fill_quad(
            Quad {
                bounds,
                border: border::rounded(RADIUS),
                ..Quad::default()
            },
            colors.container,
        );
        let hovered_row = self.hover.row.and_then(|line| self.grid.row_of(line));
        let hovered_column = self
            .hover
            .column
            .and_then(|line| self.grid.column_of(line))
            .filter(|column| parts.column_in_view(*column, offset));
        let quad = |renderer: &mut iced::Renderer, area: Rectangle, color: Color| {
            renderer.fill_quad(
                Quad {
                    bounds: area,
                    ..Quad::default()
                },
                Background::Color(color),
            );
        };

        // The column headings: their bands, then the cached lines and
        // names, then the hovered column's name brighter.
        let header_area = Rectangle {
            x: bounds.x,
            y: bounds.y,
            width: parts.headings_x() + parts.heading,
            height: HEADER,
        };
        renderer.with_layer(visible(header_area), |renderer| {
            let mut bands = Frame::new(renderer, header_area.size());
            for (index, heading) in self.grid.columns.iter().enumerate() {
                if !parts.column_in_view(index, offset) {
                    continue;
                }
                let x = index as f32 * CELL - offset.x;
                if heading.collapsed.is_some() {
                    bands.fill(&band(x), colors.band);
                }
                if hovered_column == Some(index) {
                    bands.fill(&band(x), colors.hover_heading);
                }
            }
            let cached = state.header.draw(renderer, header_area.size(), |frame| {
                self.draw_header(frame, state, &parts, offset, &colors);
            });
            let mut bright = Frame::new(renderer, header_area.size());
            if let Some(index) = hovered_column {
                let x = index as f32 * CELL - offset.x;
                self.column_name(&mut bright, state, index, x, colors.text, &colors);
            }
            let (bands, bright) = (bands.into_geometry(), bright.into_geometry());
            renderer.with_translation(origin, |renderer| {
                renderer.draw_geometry(bands);
                renderer.draw_geometry(cached);
                renderer.draw_geometry(bright);
            });
        });

        // The cells: the pointer's row and column under the cached lines
        // and icons.
        let cells_area = Rectangle {
            x: bounds.x,
            y: bounds.y + HEADER,
            width: parts.view.width,
            height: parts.view.height,
        };
        renderer.with_layer(visible(cells_area), |renderer| {
            let row_area = |row: usize| Rectangle {
                y: cells_area.y + row as f32 * CELL - offset.y,
                height: CELL,
                ..cells_area
            };
            let column_area = |column: usize| Rectangle {
                x: cells_area.x + column as f32 * CELL - offset.x,
                width: CELL,
                ..cells_area
            };
            match (hovered_row, hovered_column) {
                // An L joining the cell to both headings: its column from
                // the top down to it, its row from it across to the right.
                (Some(row), Some(column)) => {
                    let (row_area, column_area) = (row_area(row), column_area(column));
                    quad(
                        renderer,
                        Rectangle {
                            height: row_area.y + CELL - cells_area.y,
                            ..column_area
                        },
                        colors.hover_line,
                    );
                    quad(
                        renderer,
                        Rectangle {
                            x: column_area.x,
                            width: cells_area.x + cells_area.width - column_area.x,
                            ..row_area
                        },
                        colors.hover_line,
                    );
                }
                (Some(row), None) => quad(renderer, row_area(row), colors.hover_line),
                (None, Some(column)) => quad(renderer, column_area(column), colors.hover_line),
                (None, None) => {}
            }
            if let (Some(row), Some(column)) = (hovered_row, hovered_column) {
                let cell = row_area(row).intersection(&column_area(column));
                if let Some(cell) = cell {
                    quad(renderer, cell, colors.cross);
                    if let Cell::Stream {
                        action: Some(_), ..
                    } = self.grid.cell(row, column)
                    {
                        let center = cell.center();
                        renderer.fill_quad(
                            Quad {
                                bounds: Rectangle {
                                    x: center.x - 16.0,
                                    y: center.y - 16.0,
                                    width: 32.0,
                                    height: 32.0,
                                },
                                border: border::rounded(16.0),
                                ..Quad::default()
                            },
                            colors.hit,
                        );
                    }
                }
            }
            let cached = state.cells.draw(renderer, parts.content, |frame| {
                self.draw_cells(frame, &parts, &colors);
            });
            renderer.with_translation(
                Vector::new(cells_area.x - offset.x, cells_area.y - offset.y),
                |renderer| renderer.draw_geometry(cached),
            );
        });

        // The row headings.
        let headings_area = Rectangle {
            x: bounds.x + parts.headings_x(),
            y: bounds.y + HEADER,
            width: parts.heading,
            height: parts.view.height,
        };
        renderer.with_layer(visible(headings_area), |renderer| {
            for index in parts.rows_in_view(self.grid.rows.len(), offset) {
                let area = Rectangle {
                    y: headings_area.y + index as f32 * CELL - offset.y,
                    height: CELL,
                    ..headings_area
                };
                if self.grid.rows[index].collapsed.is_some() {
                    quad(renderer, area, colors.band);
                }
                if hovered_row == Some(index) {
                    quad(renderer, area, colors.hover_heading);
                }
            }
            let size = Size::new(parts.heading, parts.content.height);
            let cached = state.headings.draw(renderer, size, |frame| {
                self.draw_row_headings(frame, state, &colors, parts.heading);
            });
            renderer.with_translation(
                Vector::new(headings_area.x, headings_area.y - offset.y),
                |renderer| renderer.draw_geometry(cached),
            );
        });

        // The scrollbars, over everything.
        renderer.with_layer(visible(bounds), |renderer| {
            for vertical in [true, false] {
                let Some((track, thumb)) = parts.scrollbar(vertical, offset) else {
                    continue;
                };
                let dragged = state.drag.is_some_and(|drag| drag.vertical == vertical);
                let hovered = cursor
                    .position_in(bounds)
                    .is_some_and(|point| track.contains(point));
                let alpha = if dragged {
                    0.7
                } else if hovered {
                    0.55
                } else {
                    0.35
                };
                renderer.fill_quad(
                    Quad {
                        bounds: thumb + origin,
                        border: border::rounded(SCROLLBAR / 2.0),
                        ..Quad::default()
                    },
                    faded(colors.scrollbar, alpha),
                );
            }
        });
    }
}

impl Matrix {
    fn draw_header(
        &self,
        frame: &mut Frame,
        state: &Memory,
        parts: &Parts,
        offset: Vector,
        colors: &Colors,
    ) {
        let stroke = Stroke::default().with_color(colors.line).with_width(1.0);
        let divider = |frame: &mut Frame, x: f32| {
            let x = x + 0.5;
            frame.stroke(
                &Path::line(Point::new(x, HEADER), Point::new(x + HEADER, 0.0)),
                stroke,
            );
        };
        let mut last = None;
        for index in 0..self.grid.columns.len() {
            if !parts.column_in_view(index, offset) {
                continue;
            }
            let x = index as f32 * CELL - offset.x;
            divider(frame, x);
            self.column_name(frame, state, index, x, colors.label, colors);
            last = Some(x);
        }
        if let Some(x) = last {
            divider(frame, x + CELL);
        }
        frame.fill_rectangle(
            Point::new(0.0, HEADER - 1.0),
            Size::new(parts.headings_x() + parts.heading, 1.0),
            colors.line,
        );

        // Which way each side runs, in the corner over the row headings,
        // unless narrow headings leave the slanted names no room for it.
        if parts.heading < ROW_HEADING {
            return;
        }
        let right = parts.headings_x() + parts.heading - 12.0;
        for (center, glyph, label) in [
            (
                HEADER - 42.0,
                Icon::West,
                crate::fl!("matrix-talker-outputs"),
            ),
            (
                HEADER - 20.0,
                Icon::South,
                crate::fl!("matrix-listener-inputs"),
            ),
        ] {
            let width = measure(&label, TEXT, 12.0);
            frame.fill_text(Text {
                content: label,
                position: Point::new(right, center),
                color: colors.detail,
                size: Pixels(12.0),
                line_height: LineHeight::Absolute(Pixels(16.0)),
                font: TEXT,
                align_x: Alignment::Right,
                align_y: Vertical::Center,
                shaping: Shaping::Advanced,
                ..Text::default()
            });
            icon(
                frame,
                glyph,
                false,
                Point::new(right - width - 6.0 - 8.0, center),
                16.0,
                colors.detail,
            );
        }
    }

    /// A column's name, slanted up its band from the cell row.
    fn column_name(
        &self,
        frame: &mut Frame,
        state: &Memory,
        index: usize,
        x: f32,
        color: Color,
        colors: &Colors,
    ) {
        let heading = &self.grid.columns[index];
        match heading.collapsed {
            None => {
                // The stream's format at the far end of its heading, so the
                // formats line up, as the rows have them at their right.
                let detail_width = measure(&heading.detail, TEXT, 11.0);
                let fitted = fitted(state, Side::Talker, index, || {
                    fit(&heading.name, TEXT, 12.0, SLANT_MAX - 8.0 - detail_width)
                });
                slanted(frame, Point::new(x + 37.0, HEADER - 6.0), |frame| {
                    frame.fill_text(Text {
                        content: fitted.name,
                        position: Point::ORIGIN,
                        color,
                        size: Pixels(12.0),
                        line_height: LineHeight::Absolute(Pixels(16.0)),
                        font: TEXT,
                        align_y: Vertical::Bottom,
                        shaping: Shaping::Advanced,
                        ..Text::default()
                    });
                    frame.fill_text(Text {
                        content: heading.detail.clone(),
                        position: Point::new(SLANT_MAX, 0.0),
                        color: colors.detail,
                        size: Pixels(11.0),
                        line_height: LineHeight::Absolute(Pixels(16.0)),
                        font: TEXT,
                        align_x: Alignment::Right,
                        align_y: Vertical::Bottom,
                        shaping: Shaping::Advanced,
                        ..Text::default()
                    });
                });
            }
            Some(collapsed) => {
                let tag_width = measure(&heading.detail, MONOSPACE, 11.0);
                let fitted = fitted(state, Side::Talker, index, || {
                    fit(&heading.name, BOLD, 13.0, SLANT_MAX - 24.0 - tag_width)
                });
                let name_color = self.name_color(heading.line, colors);
                slanted(frame, Point::new(x + CELL, HEADER - 6.0), |frame| {
                    let chevron = if collapsed {
                        Icon::ChevronRight
                    } else {
                        Icon::ExpandMore
                    };
                    icon(
                        frame,
                        chevron,
                        false,
                        Point::new(8.0 - TALKER_PULL, -10.0),
                        16.0,
                        colors.text,
                    );
                    label(
                        frame,
                        fitted.name,
                        Point::new(20.0 - TALKER_PULL, -10.0),
                        BOLD,
                        13.0,
                        name_color,
                    );
                    label(
                        frame,
                        heading.detail.clone(),
                        Point::new(24.0 - TALKER_PULL + fitted.name_width, -10.0),
                        MONOSPACE,
                        11.0,
                        colors.detail,
                    );
                });
            }
        }
    }

    /// A talker's or listener's name, in the accent when the inspector
    /// shows it.
    fn name_color(&self, line: Line, colors: &Colors) -> Color {
        if self.grid.selected == Some(line.entity()) {
            colors.primary
        } else {
            colors.text
        }
    }

    fn draw_cells(&self, frame: &mut Frame, parts: &Parts, colors: &Colors) {
        let columns = self.grid.columns.len();
        for column in 1..columns {
            frame.fill_rectangle(
                Point::new(column as f32 * CELL, 0.0),
                Size::new(1.0, parts.content.height),
                colors.line,
            );
        }
        for row in 1..self.grid.rows.len() {
            frame.fill_rectangle(
                Point::new(0.0, row as f32 * CELL),
                Size::new(parts.content.width, 1.0),
                colors.line,
            );
        }
        for (index, cell) in self.grid.cells.iter().enumerate() {
            let center = Point::new(
                (index % columns) as f32 * CELL + CELL / 2.0,
                (index / columns) as f32 * CELL + CELL / 2.0,
            );
            match *cell {
                // An entity's outputs against its own inputs stay empty, so
                // the pointer's row and column show through.
                Cell::Blank | Cell::Own | Cell::Count(0) => {}
                Cell::Count(count) => {
                    let count = count.to_string();
                    let width = (measure(&count, BOLD, 11.0) + 10.0).max(20.0);
                    frame.fill(
                        &Path::rounded_rectangle(
                            Point::new(center.x - width / 2.0, center.y - 10.0),
                            Size::new(width, 20.0),
                            border::Radius::from(10.0),
                        ),
                        colors.primary,
                    );
                    frame.fill_text(Text {
                        content: count,
                        position: center,
                        color: colors.on_primary,
                        size: Pixels(11.0),
                        line_height: LineHeight::Absolute(Pixels(14.0)),
                        font: BOLD,
                        align_x: Alignment::Center,
                        align_y: Vertical::Center,
                        ..Text::default()
                    });
                }
                Cell::Stream { state, .. } => {
                    let (glyph, filled) = state.icon();
                    icon(
                        frame,
                        glyph,
                        filled,
                        center,
                        20.0,
                        state.color(&colors.scheme),
                    );
                }
            }
        }
    }

    fn draw_row_headings(&self, frame: &mut Frame, state: &Memory, colors: &Colors, width: f32) {
        let height = self.grid.rows.len() as f32 * CELL;
        frame.fill_rectangle(Point::ORIGIN, Size::new(1.0, height), colors.line);
        for (index, heading) in self.grid.rows.iter().enumerate() {
            let top = index as f32 * CELL;
            if index > 0 {
                frame.fill_rectangle(Point::new(0.0, top), Size::new(width, 1.0), colors.line);
            }
            let center = top + CELL / 2.0;
            match heading.collapsed {
                Some(collapsed) => {
                    let tag_width = measure(&heading.detail, MONOSPACE, 11.0);
                    let fitted = fitted(state, Side::Listener, index, || {
                        fit(&heading.name, BOLD, 13.0, width - 46.0 - tag_width)
                    });
                    let chevron = if collapsed {
                        Icon::ChevronRight
                    } else {
                        Icon::ExpandMore
                    };
                    icon(
                        frame,
                        chevron,
                        false,
                        Point::new(17.0, center),
                        18.0,
                        colors.text,
                    );
                    label(
                        frame,
                        heading.detail.clone(),
                        Point::new(42.0 + fitted.name_width, center),
                        MONOSPACE,
                        11.0,
                        colors.detail,
                    );
                    label(
                        frame,
                        fitted.name,
                        Point::new(34.0, center),
                        BOLD,
                        13.0,
                        self.name_color(heading.line, colors),
                    );
                }
                None => {
                    // Close to the grid, as the columns' stream names are,
                    // with the talker's or listener's name further out.
                    let detail_width = measure(&heading.detail, TEXT, 11.0);
                    let fitted = fitted(state, Side::Listener, index, || {
                        fit(&heading.name, TEXT, 14.0, width - 32.0 - detail_width)
                    });
                    label(
                        frame,
                        fitted.name,
                        Point::new(12.0, center),
                        TEXT,
                        14.0,
                        colors.text,
                    );
                    frame.fill_text(Text {
                        content: heading.detail.clone(),
                        position: Point::new(width - 12.0, center),
                        color: colors.detail,
                        size: Pixels(11.0),
                        line_height: LineHeight::Absolute(Pixels(16.0)),
                        font: TEXT,
                        align_x: Alignment::Right,
                        align_y: Vertical::Center,
                        shaping: Shaping::Advanced,
                        ..Text::default()
                    });
                }
            }
        }
    }
}

/// A column's heading band: the column leaning right as it rises.
fn band(x: f32) -> Path {
    Path::new(|builder| {
        builder.move_to(Point::new(x, HEADER));
        builder.line_to(Point::new(x + CELL, HEADER));
        builder.line_to(Point::new(x + CELL + HEADER, 0.0));
        builder.line_to(Point::new(x + HEADER, 0.0));
        builder.close();
    })
}

/// Draws with `draw` turned 45° up from `anchor`.
fn slanted(frame: &mut Frame, anchor: Point, draw: impl FnOnce(&mut Frame)) {
    frame.with_save(|frame| {
        frame.translate(Vector::new(anchor.x, anchor.y));
        frame.rotate(-FRAC_PI_4);
        draw(frame);
    });
}

/// One line of text, its left end's middle at `start`.
fn label(frame: &mut Frame, content: String, start: Point, font: Font, size: f32, color: Color) {
    frame.fill_text(Text {
        content,
        position: start,
        color,
        size: Pixels(size),
        line_height: LineHeight::Absolute(Pixels(size + 4.0)),
        font,
        align_y: Vertical::Center,
        shaping: Shaping::Advanced,
        ..Text::default()
    });
}

fn icon(frame: &mut Frame, glyph: Icon, filled: bool, center: Point, size: f32, color: Color) {
    frame.fill_text(Text {
        content: glyph.codepoint().to_string(),
        position: center,
        color,
        size: Pixels(size),
        line_height: LineHeight::Absolute(Pixels(size)),
        font: if filled { ICONS_FILLED } else { ICONS },
        align_x: Alignment::Center,
        align_y: Vertical::Center,
        shaping: Shaping::Basic,
        ..Text::default()
    });
}

/// A heading cut to fit, remembered until the grid changes.
fn fitted(state: &Memory, side: Side, index: usize, fit: impl FnOnce() -> Fitted) -> Fitted {
    state
        .fitted
        .borrow_mut()
        .entry((side, index))
        .or_insert_with(fit)
        .clone()
}

/// The colors the matrix draws with, from the theme's scheme.
#[derive(Debug, Clone, Copy)]
struct Colors {
    scheme: Scheme,
    container: Color,
    /// A talker's or listener's own column or row heading.
    band: Color,
    line: Color,
    hover_line: Color,
    hover_heading: Color,
    /// The cell under the pointer.
    cross: Color,
    /// The circle behind a cell's icon that clicking acts on.
    hit: Color,
    label: Color,
    text: Color,
    detail: Color,
    primary: Color,
    on_primary: Color,
    scrollbar: Color,
}

impl Colors {
    fn of(theme: &Theme) -> Self {
        let scheme = Scheme::of(theme);
        let container = scheme.surface_container_low;
        Self {
            scheme,
            container,
            band: scheme.surface_container,
            line: blend(container, scheme.outline_variant, 0.55),
            hover_line: blend(container, scheme.primary, 0.05),
            hover_heading: blend(container, scheme.primary, 0.08),
            cross: blend(container, scheme.primary, 0.11),
            hit: blend(container, scheme.primary, 0.2),
            label: scheme.on_surface_variant,
            text: scheme.on_surface,
            detail: scheme.outline,
            primary: scheme.primary,
            on_primary: scheme.on_primary,
            scrollbar: scheme.on_surface_variant,
        }
    }
}

impl Hash for Colors {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        for color in [
            self.container,
            self.band,
            self.line,
            self.label,
            self.text,
            self.detail,
            self.primary,
            self.on_primary,
            self.scheme.tertiary,
            self.scheme.error,
            self.scheme.on_surface_variant,
        ] {
            color.into_rgba8().hash(hasher);
        }
    }
}

impl From<Matrix> for Element<'_, Message> {
    fn from(matrix: Matrix) -> Self {
        Element::new(matrix)
    }
}

#[cfg(test)]
mod tests {
    use atdecc::EntityId;

    use super::super::{Heading, State};
    use super::*;
    use crate::network::Action;

    fn entity(id: u64) -> EntityId {
        EntityId(id)
    }

    fn group(id: u64, name: &str, tag: &str, collapsed: bool) -> Heading {
        Heading {
            line: Line::Entity(entity(id)),
            name: name.to_owned(),
            detail: tag.to_owned(),
            collapsed: Some(collapsed),
        }
    }

    fn stream(id: u64, index: u16, name: &str, detail: &str) -> Heading {
        Heading {
            line: Line::Stream(entity(id), index),
            name: name.to_owned(),
            detail: detail.to_owned(),
            collapsed: None,
        }
    }

    /// Three entities as on a small bench.
    fn bench() -> Grid {
        let columns = vec![
            group(1, "Mac mini", "f5:44", false),
            stream(1, 0, "Audio Output Stream 1", ""),
            stream(1, 1, "Media Clock Output Stream 1", ""),
            group(2, "AVB Example Entity", "92:20", false),
            stream(2, 0, "Audio Stream Out", ""),
            stream(2, 1, "CRF Media Clock Out", ""),
            group(3, "AVB Example Entity", "fe:80", true),
        ];
        let rows = vec![
            group(1, "Mac mini", "f5:44", false),
            stream(1, 0, "Audio Input Stream 1", "192k 8ch"),
            stream(1, 1, "Media Clock Input Stream", "48k CRF"),
            group(2, "AVB Example Entity", "92:20", false),
            stream(2, 0, "Audio Stream In", "48k 8ch"),
            stream(2, 1, "CRF Media Clock In", "48k CRF"),
            group(3, "AVB Example Entity", "fe:80", true),
        ];
        let action = Some(Action::Identify(entity(1)));
        let state = |state: State| Cell::Stream { state, action };
        let none = Cell::Stream {
            state: State::Incompatible,
            action: None,
        };
        let blank = Cell::Blank;
        let cells = [
            [blank; 7],
            [
                blank,
                state(State::Open),
                none,
                blank,
                none,
                none,
                Cell::Count(0),
            ],
            [
                blank,
                none,
                state(State::Open),
                blank,
                none,
                state(State::Connected),
                Cell::Count(0),
            ],
            [blank; 7],
            [
                blank,
                none,
                none,
                blank,
                state(State::Change),
                none,
                Cell::Count(0),
            ],
            [
                blank,
                none,
                state(State::Open),
                blank,
                none,
                state(State::Waiting),
                Cell::Count(1),
            ],
            [
                blank,
                none,
                Cell::Count(0),
                blank,
                Cell::Count(1),
                Cell::Count(0),
                Cell::Count(1),
            ],
        ]
        .concat();
        Grid {
            columns,
            rows,
            cells,
            selected: Some(entity(2)),
        }
    }

    /// Many talkers and listeners, each with four streams.
    fn large() -> Grid {
        let headings = |kind: &str| {
            (1..=8u64)
                .flat_map(|id| {
                    std::iter::once(group(id, &format!("Stage box {id}"), "0a:0b", false)).chain(
                        (0..4).map(move |index| {
                            stream(id, index, &format!("{kind} {}", index + 1), "48k 8ch")
                        }),
                    )
                })
                .collect::<Vec<_>>()
        };
        let columns = headings("Output");
        let rows = headings("Input");
        let cells = rows
            .iter()
            .flat_map(|row| {
                columns
                    .iter()
                    .map(move |column| match (row.line, column.line) {
                        (Line::Stream(..), Line::Stream(..)) => Cell::Stream {
                            state: State::Open,
                            action: None,
                        },
                        _ => Cell::Blank,
                    })
            })
            .collect();
        Grid {
            columns,
            rows,
            cells,
            selected: Some(entity(2)),
        }
    }

    /// Draws `grid` to the PNG file the variable `name` names, after the
    /// given events, to look at.
    fn picture(name: &str, grid: Grid, hover: Hover, size: (f32, f32), events: &[Event]) {
        let Ok(path) = std::env::var(name) else {
            return;
        };
        // The simulator adds its renderer's name to the file name.
        if let Some(stem) = path.strip_suffix(".png") {
            for renderer in ["tiny-skia", "wgpu"] {
                let _ = std::fs::remove_file(format!("{stem}-{renderer}.png"));
            }
        }
        let theme = crate::view::tests::picture_theme();
        let settings = iced::Settings {
            fonts: scramble_ui::font::files().collect(),
            default_font: TEXT,
            antialiasing: true,
            ..iced::Settings::default()
        };
        let mut simulator =
            iced_test::Simulator::with_size(settings, size, Element::from(matrix(grid, hover)));
        simulator.point_at(Point::new(100.0, 300.0));
        let _ = simulator.simulate(events.iter().cloned());
        let snapshot = simulator.snapshot(&theme).expect("draws");
        assert!(snapshot.matches_image(&path).expect("writes"));
    }

    /// Writes pictures of the matrix to the PNG files `MATRIX_PICTURE`
    /// and `MATRIX_SCROLLED` name, to look at.
    #[test]
    #[ignore = "writes pictures to look at"]
    fn pictures() {
        let hover = Hover {
            column: Some(Line::Stream(entity(2), 1)),
            row: Some(Line::Stream(entity(1), 1)),
        };
        picture("MATRIX_PICTURE", bench(), hover, (900.0, 520.0), &[]);
        let scroll = Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Pixels {
                x: -230.0,
                y: -150.0,
            },
        });
        picture(
            "MATRIX_SCROLLED",
            large(),
            Hover::default(),
            (900.0, 520.0),
            &[scroll],
        );
    }
}
