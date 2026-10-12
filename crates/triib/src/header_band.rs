//! A table with its heading row shaded: a band behind the headings as
//! tall as the tallest of them and the table's padding around it.

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay};
use iced::border::Radius;
use iced::mouse::{self, Cursor};
use iced::{Background, Border, Element, Event, Length, Rectangle, Size, Theme, Vector};
use scramble_ui::{Scheme, shape};

pub struct HeaderBand<'a, Message> {
    table: Element<'a, Message>,
    /// The table's columns, whose headings come first among its cells.
    columns: usize,
    /// The table's padding above and below each row.
    padding_y: f32,
    /// Whether the band's top left and top right corners are round: not
    /// where it meets another table's band.
    round: (bool, bool),
}

/// `table`, of `columns` columns padded `padding_y` above and below each
/// row, with its heading row shaded.
pub fn header_band<'a, Message>(
    table: impl Into<Element<'a, Message>>,
    columns: usize,
    padding_y: f32,
) -> HeaderBand<'a, Message> {
    HeaderBand {
        table: table.into(),
        columns,
        padding_y,
        round: (true, true),
    }
}

impl<Message> HeaderBand<'_, Message> {
    /// Rounds the band's top left and top right corners, or not.
    pub fn round(mut self, left: bool, right: bool) -> Self {
        self.round = (left, right);
        self
    }
}

impl<'a, Message: 'a> Widget<Message, Theme, iced::Renderer> for HeaderBand<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::stateless()
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.table)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.table));
    }

    fn size(&self) -> Size<Length> {
        self.table.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.table.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let table = self
            .table
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        layout::Node::with_children(table.size(), vec![table])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some(table) = layout.children().next() {
            self.table
                .as_widget_mut()
                .operate(&mut tree.children[0], table, renderer, operation);
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Some(table) = layout.children().next() {
            self.table.as_widget_mut().update(
                &mut tree.children[0],
                event,
                table,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;
        let Some(table) = layout.children().next() else {
            return;
        };
        let bounds = table.bounds();
        let tallest = table
            .children()
            .take(self.columns)
            .map(|heading| heading.bounds().height)
            .fold(0.0, f32::max);
        let top = shape::MEDIUM;
        renderer.fill_quad(
            Quad {
                bounds: Rectangle {
                    height: tallest + 2.0 * self.padding_y,
                    ..bounds
                },
                border: Border {
                    radius: Radius {
                        top_left: if self.round.0 { top } else { 0.0 },
                        top_right: if self.round.1 { top } else { 0.0 },
                        bottom_right: 0.0,
                        bottom_left: 0.0,
                    },
                    ..Border::default()
                },
                ..Quad::default()
            },
            Background::Color(Scheme::of(theme).surface_container),
        );
        self.table.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            table,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        layout
            .children()
            .next()
            .map_or_else(mouse::Interaction::default, |table| {
                self.table.as_widget().mouse_interaction(
                    &tree.children[0],
                    table,
                    cursor,
                    viewport,
                    renderer,
                )
            })
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let table = layout.children().next()?;
        self.table.as_widget_mut().overlay(
            &mut tree.children[0],
            table,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a> From<HeaderBand<'a, Message>> for Element<'a, Message> {
    fn from(band: HeaderBand<'a, Message>) -> Self {
        Element::new(band)
    }
}
