//! The network map widget: each gPTP tree from its grandmaster down, the
//! bridges under it on a row with their devices in columns beneath and the
//! grandmaster's own devices in the middle. Every stream has a wire that
//! enters and leaves each bridge it crosses, glowing and carrying moving
//! dots while something flows: a wire for the frames on each link, so a
//! multicast stream's listeners share one as far as their paths do and it
//! branches where they part, while a unicast stream's each have their own.
//! Each tree is a clock domain, labelled gPTP or PTP with its number, and
//! what is in no domain known sits in a band of its own.

use std::cell::Cell as Memo;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::f32::consts::TAU;
use std::hash::{DefaultHasher, Hash, Hasher};

use iced::advanced::graphics::geometry::Renderer as _;
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::text::{Alignment, Shaping};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell};
use iced::alignment::Vertical;
use iced::font::Weight;
use iced::mouse::{self, Cursor};
use iced::time::Instant;
use iced::widget::canvas::{Cache, Frame, LineCap, LineDash, Path, Stroke, Text};
use iced::widget::text::LineHeight;
use iced::{
    Color, Element, Event, Font, Length, Pixels, Point, Rectangle, Size, Theme, Vector, border,
    window,
};
use scramble_ui::font::{ICONS, TEXT};
use scramble_ui::{Scheme, blend, faded, motion};

use super::{Card, Focus, Map, Paint, Status};
use crate::app::Message;
use crate::text::{fit, measure};
use crate::topology::{Kind, NodeId, Topology};

/// A device card at its smallest, in a bridge's column.
const DEVICE: Size = Size::new(200.0, 46.0);
/// The narrowest the grandmaster's own column is.
const CENTER_WIDTH: f32 = 150.0;
const BRIDGE_HEIGHT: f32 = 64.0;
const ROOT: Size = Size::new(200.0, 56.0);
/// The widest a card grows for its text before cutting it short.
const WIDEST: f32 = 280.0;
/// Between cards stacked in a column.
const STACK_GAP: f32 = 8.0;
/// From the row of bridges to the first card under them.
const UNDER_ROW: f32 = 26.0;
/// From the grandmaster down to the row of bridges, at least.
const BAND: f32 = 74.0;
/// Between a column's gutter and the next column out.
const COLUMN_GAP: f32 = 24.0;
const MARGIN: f32 = 32.0;
/// Between trees of different grandmasters.
const TREE_GAP: f32 = 64.0;
/// Of a device's corners and a bridge's.
const DEVICE_RADIUS: f32 = 10.0;
const HUB_RADIUS: f32 = 14.0;
/// Of a wire's rounded elbows: twice a device's corners.
const ELBOW: f32 = 2.0 * DEVICE_RADIUS;
/// Between wires side by side in a column's gutter, in the grandmaster's
/// column, rising from a bridge, and entering a bridge's side.
const LANE: f32 = 7.0;
const CENTER_LANE: f32 = 5.0;
const TRUNK: f32 = 8.0;
const ENTRY: f32 = 4.0;
/// In the compact layout: how far each level steps in at least, the gap
/// between cards, and the margin.
const INDENT: f32 = 40.0;
const COMPACT_GAP: f32 = 14.0;
const COMPACT_MARGIN: f32 = 16.0;
/// The smallest the map is drawn before it takes the compact layout or
/// scrolls; smaller, its text would be hard to read.
const MIN_SCALE: f32 = 0.6;
/// Around what is on no tree, the room for each band's label, and between
/// bands.
const BAND_PADDING: f32 = 16.0;
const BAND_LABEL: f32 = 36.0;
const BAND_GAP: f32 = 16.0;
/// In the compact layout, the room for a tree's label above it.
const TREE_LABEL: f32 = 22.0;
/// Between the dots moving along a wire.
const STEP: f32 = 14.0;
/// The time a dot takes to move one step, and the glow to pulse once.
const BEAT: f32 = 0.4;
/// How close a click must come to a wire to pick it.
const REACH: f32 = 6.0;
const ICON: f32 = 18.0;
const ICON_GAP: f32 = 8.0;
const SEMIBOLD: Font = Font {
    weight: Weight::Semibold,
    ..TEXT
};

pub struct NetMap {
    map: Map,
    placement: Placement,
    /// How much smaller than its natural size the map is drawn.
    scale: f32,
    /// Where the drawing starts, centring it in a wider area.
    origin: Vector,
    /// The room it takes.
    size: Size,
    /// The lines in the order they are drawn, what is brought forward last.
    order: Vec<usize>,
    /// By line, which of its wires it draws: a wire lines share is drawn
    /// once, by the last of them drawn, lit when any of them is.
    draws: Vec<Vec<bool>>,
    /// A hash of the map, telling when to draw it again.
    signature: u64,
}

/// The map for an `area` of the given size: the trees scaled down to fit
/// its width, or, when that would make them too small, stacked in the
/// compact layout; centered when there is room to spare.
pub fn map(map: Map, area: Size) -> NetMap {
    let width = if area.width.is_finite() && area.width > 0.0 {
        area.width
    } else {
        f32::INFINITY
    };
    let mut placement = fan(&map, width);
    if placement.size.width * MIN_SCALE > width {
        placement = compact(&map);
    }
    let scale = (width / placement.size.width).clamp(MIN_SCALE, 1.0);
    let drawn = Size::new(placement.size.width * scale, placement.size.height * scale);
    let size = Size::new(
        if width.is_finite() {
            drawn.width.max(width)
        } else {
            drawn.width
        },
        drawn.height,
    );
    let origin = Vector::new((size.width - drawn.width) / 2.0, 0.0);
    let mut order: Vec<usize> = (0..map.lines.len()).collect();
    order.sort_by_key(|&line| (map.lines[line].related, map.lines[line].picked));
    let mut drawer: HashMap<(usize, usize), usize> = HashMap::new();
    for &line in &order {
        for wire in &placement.wires[line] {
            drawer.insert(wire.key, line);
        }
    }
    let draws = placement
        .wires
        .iter()
        .enumerate()
        .map(|(line, wires)| wires.iter().map(|wire| drawer[&wire.key] == line).collect())
        .collect();
    let mut hasher = DefaultHasher::new();
    map.hash(&mut hasher);
    placement.compact.hash(&mut hasher);
    scale.to_bits().hash(&mut hasher);
    origin.x.to_bits().hash(&mut hasher);
    placement.size.width.to_bits().hash(&mut hasher);
    NetMap {
        signature: hasher.finish(),
        map,
        placement,
        scale,
        origin,
        size,
        order,
        draws,
    }
}

/// The color of a wire, a swatch or a border.
pub fn paint_color(scheme: &Scheme, paint: Paint) -> Color {
    match paint {
        Paint::Clock if scheme.dark => Color::from_rgb8(0xf2, 0xc4, 0x6d),
        Paint::Clock => Color::from_rgb8(0x8a, 0x5a, 0x00),
        Paint::Talker { hue, shade } => {
            let shade = f32::from(shade) / 100.0;
            if scheme.dark {
                hsl(f32::from(hue), 0.65, 0.60 + 0.24 * shade)
            } else {
                hsl(f32::from(hue), 0.70, 0.28 + 0.16 * shade)
            }
        }
        Paint::Failed => scheme.error,
        Paint::Idle => blend(scheme.outline, scheme.on_surface_variant, 0.4),
        Paint::Unsynced => blend(scheme.outline_variant, scheme.outline, 0.4),
        Paint::Text => scheme.on_surface,
        Paint::Soft => scheme.on_surface_variant,
        Paint::Muted => scheme.outline,
        Paint::Plain => scheme.outline_variant,
    }
}

/// The color of text in a paint: secondary text muted, borders' paint
/// readable.
pub fn text_color(scheme: &Scheme, paint: Paint) -> Color {
    match paint {
        Paint::Plain => scheme.outline,
        paint => paint_color(scheme, paint),
    }
}

/// A color from hue in degrees, saturation and lightness.
fn hsl(hue: f32, saturation: f32, lightness: f32) -> Color {
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let part = hue / 60.0;
    let second = chroma * (1.0 - (part % 2.0 - 1.0).abs());
    let (red, green, blue) = match part as u32 {
        0 => (chroma, second, 0.0),
        1 => (second, chroma, 0.0),
        2 => (0.0, chroma, second),
        3 => (0.0, second, chroma),
        4 => (second, 0.0, chroma),
        _ => (chroma, 0.0, second),
    };
    let lift = lightness - chroma / 2.0;
    Color::from_rgb(red + lift, green + lift, blue + lift)
}

/// Where every card and wire goes.
#[derive(Debug, Clone, Default)]
struct Placement {
    /// By node; none for a node not drawn.
    cards: Vec<Option<Rectangle>>,
    /// Drawn as a bridge: rounder, with a larger name.
    hubs: Vec<bool>,
    /// Around what is on no tree, by why, each with its label.
    bands: Vec<(Rectangle, String)>,
    /// Each tree's label, by the middle of its left end.
    labels: Vec<(Point, String)>,
    size: Size,
    /// Stacked under each other rather than in columns.
    compact: bool,
    /// Each line's wires, in the order it travels them.
    wires: Vec<Vec<Wire>>,
}

/// A link a line crosses: the line, the link's place on it, the link's
/// lower node, and whether the line goes up it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Leg {
    line: usize,
    hop: usize,
    node: NodeId,
    up: bool,
}

/// The links the lines cross, each drawn once for the frames on it: where
/// the lines of a multicast stream cross a link alike, the first of them
/// has the leg drawn and the others are paired with it, to take its wire.
fn legs(map: &Map) -> (Vec<Leg>, Vec<(Leg, Leg)>) {
    let topology = &map.topology;
    let mut drawn = Vec::new();
    let mut copies = Vec::new();
    let mut first = HashMap::new();
    for (line, entry) in map.lines.iter().enumerate() {
        for (hop, &(node, up)) in entry.hops.iter().enumerate() {
            if topology.nodes[node].parent.is_none() {
                continue;
            }
            let leg = Leg {
                line,
                hop,
                node,
                up,
            };
            let Some(stream) = entry.stream else {
                drawn.push(leg);
                continue;
            };
            match first.entry((stream, entry.status, node, up)) {
                Entry::Occupied(found) => copies.push((leg, *found.get())),
                Entry::Vacant(slot) => {
                    slot.insert(leg);
                    drawn.push(leg);
                }
            }
        }
    }
    (drawn, copies)
}

/// Gives each leg paired with a drawn one that leg's wire, marking both
/// shared.
fn share(wires: &mut [Vec<Option<Wire>>], copies: &[(Leg, Leg)]) {
    for (copy, drawn) in copies {
        let Some(wire) = wires[drawn.line][drawn.hop].as_mut() else {
            continue;
        };
        wire.shared = true;
        let wire = wire.clone();
        wires[copy.line][copy.hop] = Some(wire);
    }
}

/// Whether a node heads a column of its own: a bridge, or anything with
/// nodes below it.
fn is_head(topology: &Topology, node: NodeId) -> bool {
    topology.nodes[node].kind == Kind::Bridge || !topology.nodes[node].children.is_empty()
}

/// What sits in a head's column, or the grandmaster's.
fn column_of(topology: &Topology, node: NodeId) -> Vec<NodeId> {
    topology.nodes[node]
        .children
        .iter()
        .copied()
        .filter(|&child| !is_head(topology, child))
        .collect()
}

/// How wide a card needs to be for its name and second line.
fn natural_width(card: &Card, hub: bool) -> f32 {
    let (padding, size) = if hub { (14.0, 14.0) } else { (10.0, 13.0) };
    let name = measure(&card.name, SEMIBOLD, size);
    let tag = if card.tag.is_empty() {
        0.0
    } else {
        measure(&card.tag, Font::MONOSPACE, 11.0) + 6.0
    };
    let second = tag + measure(&card.subtitle, TEXT, 11.0);
    2.0 * padding + ICON + ICON_GAP + name.max(second) + 2.0
}

/// Where a wire enters a card when `count` wires do: spread down its
/// side, or in its middle for one.
fn entry_y(card: Rectangle, place: usize, count: usize) -> f32 {
    if count <= 1 {
        card.center_y()
    } else {
        card.y + 6.0 + place as f32 * (card.height - 12.0) / (count - 1) as f32
    }
}

/// The tallest a card needs to be for `count` wires entering its side.
fn height_for(base: f32, count: usize) -> f32 {
    base.max(12.0 + ENTRY * count.saturating_sub(1) as f32)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    Root,
    /// Heads a column on the left (0) or right (1).
    Head(usize),
    /// In a column.
    Device,
    Apart,
}

/// A tree: its grandmaster, the devices straight under it, and each side's
/// columns from the middle out.
struct Grove {
    root: NodeId,
    center: Vec<NodeId>,
    sides: [Vec<NodeId>; 2],
}

/// Puts `head` and the heads below it on a side, from the middle out.
fn visit(topology: &Topology, head: NodeId, side: usize, out: &mut Vec<NodeId>, role: &mut [Role]) {
    role[head] = Role::Head(side);
    out.push(head);
    for &child in &topology.nodes[head].children {
        if is_head(topology, child) {
            visit(topology, child, side, out, role);
        } else {
            role[child] = Role::Device;
        }
    }
}

/// From the end of a head's top edge away from the middle.
fn from_outer(card: Rectangle, side: usize, distance: f32) -> f32 {
    if side == 0 {
        card.x + distance
    } else {
        card.x + card.width - distance
    }
}

/// The edge of a card away from the middle.
fn outer_edge(card: Rectangle, side: usize) -> f32 {
    if side == 0 {
        card.x
    } else {
        card.x + card.width
    }
}

/// The room a column's gutter takes for `lanes` wires.
fn gutter(lanes: usize) -> f32 {
    if lanes == 0 {
        0.0
    } else {
        ELBOW + (lanes - 1) as f32 * LANE + 10.0
    }
}

/// What is in no domain known, in its band with its label.
fn bands(topology: &Topology) -> Vec<(String, Vec<NodeId>)> {
    if topology.apart.is_empty() {
        return Vec::new();
    }
    vec![(crate::fl!("netmap-domain-unknown"), topology.apart.clone())]
}

/// A tree's label: its gPTP domain, or its PTP domain where it runs AVB
/// Lite, where known.
fn tree_label(topology: &Topology, root: NodeId) -> Option<String> {
    let domain = match topology.nodes[root].kind {
        Kind::Switch { domain, .. } => Some(domain),
        _ => topology.nodes[root].domain,
    }?;
    Some(if topology.is_ptp(root) {
        crate::fl!("netmap-ptp-domain", domain = domain)
    } else {
        crate::fl!("netmap-domain", domain = domain)
    })
}

/// A label as the map writes it: in capitals where the script has them,
/// gPTP kept as written.
fn label_text(label: &str) -> String {
    crate::i18n::uppercase(label).replace("GPTP", "gPTP")
}

/// The wide layout: each tree with its grandmaster at the top, the bridges
/// under it on a row, split between the left and right, each heading a
/// column of its devices with their wires in the gutter on its outer side,
/// and the bridges below those further out. The grandmaster's own devices
/// sit in the middle. What is on no tree goes to the right, or below when
/// `width` has no room for it there.
fn fan(map: &Map, width: f32) -> Placement {
    let topology = &map.topology;
    let count = topology.nodes.len();
    let (legs, copies) = legs(map);
    let mut role = vec![Role::Apart; count];
    let mut groves = Vec::new();
    for &root in &topology.roots {
        role[root] = Role::Root;
        let mut grove = Grove {
            root,
            center: Vec::new(),
            sides: [Vec::new(), Vec::new()],
        };
        let mut heads = 0;
        for &child in &topology.nodes[root].children {
            if is_head(topology, child) {
                let side = heads % 2;
                heads += 1;
                visit(topology, child, side, &mut grove.sides[side], &mut role);
            } else {
                role[child] = Role::Device;
                grove.center.push(child);
            }
        }
        groves.push(grove);
    }
    let hubs: Vec<bool> = role
        .iter()
        .map(|role| matches!(role, Role::Root | Role::Head(_)))
        .collect();
    // How far out each head sits on its side, and each device's place in
    // its column.
    let mut rank = vec![0; count];
    let mut stack = vec![0; count];
    for grove in &groves {
        for side in &grove.sides {
            for (place, &head) in side.iter().enumerate() {
                rank[head] = place;
            }
        }
        for (place, &device) in grove.center.iter().enumerate() {
            stack[device] = place;
        }
    }
    for (node, _) in role
        .iter()
        .enumerate()
        .filter(|(_, role)| matches!(role, Role::Head(_)))
    {
        for (place, device) in column_of(topology, node).into_iter().enumerate() {
            stack[device] = place;
        }
    }

    // Wires down to devices, by the head or grandmaster they hang from;
    // wires rising from a head; and those entering a head from above or a
    // grandmaster from the side.
    let mut lanes: HashMap<NodeId, Vec<Leg>> = HashMap::new();
    let mut rising: HashMap<NodeId, Vec<Leg>> = HashMap::new();
    let mut arches: HashMap<NodeId, Vec<Leg>> = HashMap::new();
    let mut trunks: HashMap<(NodeId, usize), Vec<Leg>> = HashMap::new();
    for &leg in &legs {
        let Some(upper) = topology.nodes[leg.node].parent else {
            continue;
        };
        match (role[leg.node], role[upper]) {
            (Role::Device, Role::Root | Role::Head(_)) => lanes.entry(upper).or_default().push(leg),
            (Role::Head(side), Role::Root) => {
                rising.entry(leg.node).or_default().push(leg);
                trunks.entry((upper, side)).or_default().push(leg);
            }
            (Role::Head(_), Role::Head(_)) => {
                rising.entry(leg.node).or_default().push(leg);
                arches.entry(upper).or_default().push(leg);
            }
            _ => {}
        }
    }
    // Shallow devices take the lanes nearest their column, so no wire
    // crosses another; a device's wires enter it in lane order.
    for group in lanes.values_mut() {
        group.sort_by_key(|leg| (stack[leg.node], leg.line, !leg.up));
    }
    for group in rising.values_mut() {
        group.sort_by_key(|leg| (leg.line, !leg.up));
    }
    let rise_place = |leg: &Leg| {
        rising
            .get(&leg.node)
            .and_then(|group| group.iter().position(|other| other == leg))
            .unwrap_or(0)
    };
    // Wires from farther out run higher, so none crosses another.
    for group in arches.values_mut().chain(trunks.values_mut()) {
        group.sort_by_key(|leg| (Reverse(rank[leg.node]), rise_place(leg)));
    }
    let mut entries: HashMap<NodeId, Vec<Leg>> = HashMap::new();
    for group in lanes.values() {
        for &leg in group {
            entries.entry(leg.node).or_default().push(leg);
        }
    }
    let count_of =
        |map: &HashMap<NodeId, Vec<Leg>>, node: NodeId| map.get(&node).map_or(0, Vec::len);

    let mut heights = vec![DEVICE.height; count];
    for node in 0..count {
        heights[node] = match role[node] {
            Role::Device => height_for(DEVICE.height, count_of(&entries, node)),
            Role::Head(_) => BRIDGE_HEIGHT.max(8.0 + ENTRY * count_of(&lanes, node) as f32),
            Role::Root => {
                let most = (0..2)
                    .map(|side| trunks.get(&(node, side)).map_or(0, Vec::len))
                    .max()
                    .unwrap_or(0);
                ROOT.height.max(8.0 + TRUNK * most as f32 + 8.0)
            }
            Role::Apart => DEVICE.height,
        };
    }
    let natural: Vec<f32> = (0..count)
        .map(|node| natural_width(&map.cards[node], hubs[node]))
        .collect();

    let mut cards = vec![None; count];
    let mut labels = Vec::new();
    // The trees start lower where any has a label, which sits level with
    // the bands' labels.
    let top = if groves
        .iter()
        .any(|grove| tree_label(topology, grove.root).is_some())
    {
        MARGIN + BAND_LABEL
    } else {
        MARGIN
    };
    // Where each grandmaster's own column starts.
    let mut middle: HashMap<NodeId, f32> = HashMap::new();
    let mut left = MARGIN;
    for grove in &groves {
        let root = grove.root;
        let center_width = grove
            .center
            .iter()
            .map(|&device| natural[device])
            .fold(CENTER_WIDTH, f32::max)
            .min(WIDEST);
        let center_lanes = count_of(&lanes, root);
        let lane_low = -ELBOW - center_lanes.saturating_sub(1) as f32 * CENTER_LANE;
        let root_width = natural[root].clamp(ROOT.width, WIDEST);
        let (root_left, root_right) = if grove.center.is_empty() {
            (-root_width / 2.0, root_width / 2.0)
        } else {
            (
                (center_width - root_width).min(-50.0).min(lane_low - 12.0),
                center_width,
            )
        };
        // Each column's width and place, from the middle out.
        let mut columns: Vec<(NodeId, f32, f32)> = Vec::new();
        let column_width = |head: NodeId| {
            let text = column_of(topology, head)
                .into_iter()
                .map(|device| natural[device])
                .fold(natural[head].max(DEVICE.width), f32::max)
                .min(WIDEST);
            let rises = count_of(&arches, head) + count_of(&rising, head);
            text.max(32.0 + rises.saturating_sub(1) as f32 * TRUNK)
        };
        let mut low = root_left.min(if center_lanes > 0 {
            lane_low - ELBOW
        } else {
            root_left
        });
        let mut edge = root_left - 30.0;
        for &head in &grove.sides[0] {
            let width = column_width(head);
            let x = edge - width;
            columns.push((head, x, width));
            let room = gutter(count_of(&lanes, head));
            low = low.min(x - room);
            edge = x - room - COLUMN_GAP;
        }
        let mut high = root_right;
        let mut edge = root_right + 24.0;
        for &head in &grove.sides[1] {
            let width = column_width(head);
            columns.push((head, edge, width));
            let room = gutter(count_of(&lanes, head));
            high = high.max(edge + width + room);
            edge += width + room + COLUMN_GAP;
        }
        let shift = left - low;

        let root_card = Rectangle::new(
            Point::new(root_left + shift, top),
            Size::new(root_right - root_left, heights[root]),
        );
        cards[root] = Some(root_card);
        if let Some(label) = tree_label(topology, root) {
            labels.push((Point::new(root_card.x, MARGIN + BAND_LABEL / 2.0), label));
        }
        let most_arches = columns
            .iter()
            .map(|&(head, ..)| count_of(&arches, head))
            .max()
            .unwrap_or(0);
        let row_y = root_card.y + root_card.height + BAND.max(30.0 + 6.0 * most_arches as f32);
        let row_height = columns
            .iter()
            .map(|&(head, ..)| heights[head])
            .fold(0.0, f32::max);
        let first = if columns.is_empty() {
            root_card.y + root_card.height + 40.0
        } else {
            row_y + row_height + UNDER_ROW
        };
        for &(head, x, width) in &columns {
            cards[head] = Some(Rectangle::new(
                Point::new(x + shift, row_y),
                Size::new(width, heights[head]),
            ));
            let mut y = first;
            for device in column_of(topology, head) {
                cards[device] = Some(Rectangle::new(
                    Point::new(x + shift, y),
                    Size::new(width, heights[device]),
                ));
                y += heights[device] + STACK_GAP;
            }
        }
        let mut y = first;
        for &device in &grove.center {
            cards[device] = Some(Rectangle::new(
                Point::new(shift, y),
                Size::new(center_width, heights[device]),
            ));
            y += heights[device] + STACK_GAP;
        }
        middle.insert(root, shift);
        left = high + shift + TREE_GAP;
    }
    let trees_right = if groves.is_empty() {
        MARGIN
    } else {
        left - TREE_GAP
    };
    let trees_bottom = cards
        .iter()
        .flatten()
        .map(|card| card.y + card.height)
        .fold(MARGIN, f32::max);

    // What is on no tree, in its bands: to the right in a column, or below
    // in rows, the bands one under another.
    let apart_width = topology
        .apart
        .iter()
        .map(|&node| natural[node])
        .fold(DEVICE.width, f32::max)
        .min(WIDEST);
    let mut placed_bands = Vec::new();
    if !topology.apart.is_empty() {
        let band_width = apart_width + 2.0 * BAND_PADDING;
        let beside = if groves.is_empty() {
            MARGIN
        } else {
            trees_right + TREE_GAP
        };
        let (mut origin, per_row) = if beside + band_width + MARGIN <= width {
            (Point::new(beside, MARGIN), 1)
        } else {
            let room = (trees_right - MARGIN).max(band_width);
            let per_row = (((room - 2.0 * BAND_PADDING + COLUMN_GAP) / (apart_width + COLUMN_GAP))
                as usize)
                .max(1);
            let top = if groves.is_empty() {
                MARGIN
            } else {
                trees_bottom + 48.0
            };
            (Point::new(MARGIN, top), per_row)
        };
        for (label, nodes) in bands(topology) {
            for (place, &node) in nodes.iter().enumerate() {
                let (row, column) = (place / per_row, place % per_row);
                cards[node] = Some(Rectangle::new(
                    Point::new(
                        origin.x + BAND_PADDING + column as f32 * (apart_width + COLUMN_GAP),
                        origin.y + BAND_LABEL + row as f32 * (DEVICE.height + STACK_GAP),
                    ),
                    Size::new(apart_width, DEVICE.height),
                ));
            }
            let rows = nodes.len().div_ceil(per_row);
            let columns = nodes.len().min(per_row);
            let band = Rectangle {
                x: origin.x,
                y: origin.y,
                width: 2.0 * BAND_PADDING
                    + columns as f32 * apart_width
                    + columns.saturating_sub(1) as f32 * COLUMN_GAP,
                height: BAND_LABEL
                    + rows as f32 * DEVICE.height
                    + rows.saturating_sub(1) as f32 * STACK_GAP
                    + BAND_PADDING,
            };
            origin.y += band.height + BAND_GAP;
            placed_bands.push((band, label));
        }
    }

    let mut wires: Vec<Vec<Option<Wire>>> = map
        .lines
        .iter()
        .map(|line| vec![None; line.hops.len()])
        .collect();
    // Corners run down the link; a wire going up runs them backwards.
    let mut put = |leg: &Leg, corners: Vec<Point>| {
        wires[leg.line][leg.hop] = Some(Wire::new(corners).for_leg(map, leg));
    };
    let entry = |leg: &Leg, card: Rectangle| {
        let own = &entries[&leg.node];
        entry_y(
            card,
            own.iter().position(|other| other == leg).unwrap_or(0),
            own.len(),
        )
    };
    // Down to devices: in the gutter outside a column, entering its head's
    // side, or straight down out of the grandmaster.
    for (&upper, group) in &lanes {
        let Some(above) = cards[upper] else {
            continue;
        };
        for (lane, leg) in group.iter().enumerate() {
            let Some(card) = cards[leg.node] else {
                continue;
            };
            let y = entry(leg, card);
            let corners = match role[upper] {
                Role::Head(side) => {
                    let sign = if side == 0 { -1.0 } else { 1.0 };
                    let x = outer_edge(above, side) + sign * (ELBOW + lane as f32 * LANE);
                    // Centered down the head's side, however many leave it;
                    // the outer lanes leave higher, so none crosses another.
                    let middle = (group.len() - 1) as f32 / 2.0;
                    let top = above.center_y() + (middle - lane as f32) * ENTRY;
                    vec![
                        Point::new(outer_edge(above, side), top),
                        Point::new(x, top),
                        Point::new(x, y),
                        Point::new(outer_edge(card, side), y),
                    ]
                }
                _ => {
                    let x = middle.get(&upper).copied().unwrap_or(above.x)
                        - ELBOW
                        - lane as f32 * CENTER_LANE;
                    vec![
                        Point::new(x, above.y + above.height),
                        Point::new(x, y),
                        Point::new(card.x, y),
                    ]
                }
            };
            put(leg, corners);
        }
    }
    // Up from a head into its grandmaster's side.
    for (&(root, side), group) in &trunks {
        let Some(above) = cards[root] else {
            continue;
        };
        for (place, leg) in group.iter().enumerate() {
            let Some(card) = cards[leg.node] else {
                continue;
            };
            let x = from_outer(
                card,
                side,
                20.0 + (count_of(&arches, leg.node) + rise_place(leg)) as f32 * TRUNK,
            );
            // Centered down the grandmaster's side, the farther out higher.
            let middle = (group.len() - 1) as f32 / 2.0;
            let y = above.center_y() + (place as f32 - middle) * TRUNK;
            let into = if side == 0 {
                above.x
            } else {
                above.x + above.width
            };
            put(
                leg,
                vec![Point::new(into, y), Point::new(x, y), Point::new(x, card.y)],
            );
        }
    }
    // Up from a head over to the top of the head it hangs from.
    for (&upper, group) in &arches {
        let (Some(above), Role::Head(side)) = (cards[upper], role[upper]) else {
            continue;
        };
        let total = group.len();
        for (place, leg) in group.iter().enumerate() {
            let Some(card) = cards[leg.node] else {
                continue;
            };
            let x = from_outer(
                card,
                side,
                20.0 + (count_of(&arches, leg.node) + rise_place(leg)) as f32 * TRUNK,
            );
            let inward = (total - 1 - place) as f32;
            let into = from_outer(above, side, 8.0 + inward * TRUNK);
            let y = above.y - ELBOW - inward * 6.0;
            put(
                leg,
                vec![
                    Point::new(into, above.y),
                    Point::new(into, y),
                    Point::new(x, y),
                    Point::new(x, card.y),
                ],
            );
        }
    }

    share(&mut wires, &copies);
    let right = cards
        .iter()
        .flatten()
        .map(|card| card.x + card.width)
        .chain(placed_bands.iter().map(|(band, _)| band.x + band.width))
        .fold(trees_right, f32::max);
    let bottom = cards
        .iter()
        .flatten()
        .map(|card| card.y + card.height)
        .chain(placed_bands.iter().map(|(band, _)| band.y + band.height))
        .fold(MARGIN, f32::max);
    Placement {
        cards,
        hubs,
        bands: placed_bands,
        labels,
        size: Size::new(right + MARGIN, bottom + MARGIN),
        compact: false,
        wires: wires
            .into_iter()
            .map(|line| line.into_iter().flatten().collect())
            .collect(),
    }
}

/// The narrow layout: every tree stacked, each node's children under it a
/// step in, their wires in the gutter that step leaves, and what is on no
/// tree below.
fn compact(map: &Map) -> Placement {
    let topology = &map.topology;
    let count = topology.nodes.len();
    let (legs, copies) = legs(map);
    let hubs: Vec<bool> = (0..count)
        .map(|node| {
            topology.nodes[node].apart.is_none()
                && (topology.nodes[node].parent.is_none() || is_head(topology, node))
        })
        .collect();
    // Devices first, then the heads with what hangs from them.
    let ordered = |node: NodeId| -> Vec<NodeId> {
        let children = &topology.nodes[node].children;
        children
            .iter()
            .copied()
            .filter(|&child| !is_head(topology, child))
            .chain(
                children
                    .iter()
                    .copied()
                    .filter(|&child| is_head(topology, child)),
            )
            .collect()
    };
    let mut stack = vec![0; count];
    for node in 0..count {
        for (place, child) in ordered(node).into_iter().enumerate() {
            stack[child] = place;
        }
    }
    let mut lanes: HashMap<NodeId, Vec<Leg>> = HashMap::new();
    for &leg in &legs {
        if let Some(upper) = topology.nodes[leg.node].parent {
            lanes.entry(upper).or_default().push(leg);
        }
    }
    for group in lanes.values_mut() {
        group.sort_by_key(|leg| (stack[leg.node], leg.line, !leg.up));
    }
    let mut entries: HashMap<NodeId, Vec<Leg>> = HashMap::new();
    for group in lanes.values() {
        for &leg in group {
            entries.entry(leg.node).or_default().push(leg);
        }
    }
    let indent = |node: NodeId| {
        INDENT.max(ELBOW + 12.0 + CENTER_LANE * lanes.get(&node).map_or(0, Vec::len) as f32)
    };
    let sizes: Vec<Size> = (0..count)
        .map(|node| {
            let base = if hubs[node] {
                ROOT.height
            } else {
                DEVICE.height
            };
            Size::new(
                natural_width(&map.cards[node], hubs[node]).clamp(DEVICE.width, WIDEST),
                height_for(base, entries.get(&node).map_or(0, Vec::len)),
            )
        })
        .collect();

    let mut cards = vec![None; count];
    fn put(
        node: NodeId,
        x: f32,
        y: &mut f32,
        sizes: &[Size],
        cards: &mut [Option<Rectangle>],
        children: &dyn Fn(NodeId) -> Vec<NodeId>,
        indent: &dyn Fn(NodeId) -> f32,
    ) {
        cards[node] = Some(Rectangle::new(Point::new(x, *y), sizes[node]));
        *y += sizes[node].height + COMPACT_GAP;
        for child in children(node) {
            put(child, x + indent(node), y, sizes, cards, children, indent);
        }
    }
    let mut y = COMPACT_MARGIN;
    let mut labels = Vec::new();
    for &root in &topology.roots {
        if let Some(label) = tree_label(topology, root) {
            labels.push((Point::new(COMPACT_MARGIN, y + 8.0), label));
            y += TREE_LABEL;
        }
        put(
            root,
            COMPACT_MARGIN,
            &mut y,
            &sizes,
            &mut cards,
            &ordered,
            &indent,
        );
        y += 2.0 * COMPACT_GAP;
    }
    let mut placed_bands = Vec::new();
    if !topology.apart.is_empty() {
        let width = topology
            .apart
            .iter()
            .map(|&node| sizes[node].width)
            .fold(DEVICE.width, f32::max);
        for (label, nodes) in bands(topology) {
            let top = y;
            y += BAND_LABEL;
            for &node in &nodes {
                cards[node] = Some(Rectangle::new(
                    Point::new(COMPACT_MARGIN + BAND_PADDING, y),
                    Size::new(width, DEVICE.height),
                ));
                y += DEVICE.height + STACK_GAP;
            }
            let band = Rectangle {
                x: COMPACT_MARGIN,
                y: top,
                width: width + 2.0 * BAND_PADDING,
                height: y - STACK_GAP - top + BAND_PADDING,
            };
            y = band.y + band.height + BAND_GAP;
            placed_bands.push((band, label));
        }
    }

    let mut wires: Vec<Vec<Option<Wire>>> = map
        .lines
        .iter()
        .map(|line| vec![None; line.hops.len()])
        .collect();
    for (&upper, group) in &lanes {
        let Some(above) = cards[upper] else {
            continue;
        };
        for (lane, leg) in group.iter().enumerate() {
            let Some(card) = cards[leg.node] else {
                continue;
            };
            let own = &entries[&leg.node];
            let y = entry_y(
                card,
                own.iter().position(|other| other == leg).unwrap_or(0),
                own.len(),
            );
            let x = above.x + indent(upper) - ELBOW - lane as f32 * CENTER_LANE;
            let wire = Wire::new(vec![
                Point::new(x, above.y + above.height),
                Point::new(x, y),
                Point::new(card.x, y),
            ]);
            wires[leg.line][leg.hop] = Some(wire.for_leg(map, leg));
        }
    }
    share(&mut wires, &copies);
    let right = cards
        .iter()
        .flatten()
        .map(|card| card.x + card.width)
        .chain(placed_bands.iter().map(|(band, _)| band.x + band.width))
        .fold(COMPACT_MARGIN, f32::max);
    let bottom = cards
        .iter()
        .flatten()
        .map(|card| card.y + card.height)
        .chain(placed_bands.iter().map(|(band, _)| band.y + band.height))
        .fold(COMPACT_MARGIN, f32::max);
    Placement {
        cards,
        hubs,
        bands: placed_bands,
        labels,
        size: Size::new(right + COMPACT_MARGIN, bottom + COMPACT_MARGIN),
        compact: true,
        wires: wires
            .into_iter()
            .map(|line| line.into_iter().flatten().collect())
            .collect(),
    }
}

/// A wire's corners, drawn with rounded elbows, and points along it for
/// placing the moving dots.
#[derive(Debug, Clone)]
struct Wire {
    corners: Vec<Point>,
    /// Along a Wi-Fi hop, drawn dotted.
    wireless: bool,
    /// Points along the drawn wire with the distance to each from its start.
    samples: Vec<(Point, f32)>,
    /// The line and hop it was laid out for, the same for every line that
    /// shares it.
    key: (usize, usize),
    /// Carrying a multicast stream to more than one listener.
    shared: bool,
}

enum Piece {
    Straight(Point, Point),
    Elbow(Point, Point, Point),
}

impl Wire {
    fn new(corners: Vec<Point>) -> Self {
        let mut samples = Vec::new();
        let mut traveled = 0.0;
        let mut add = |point: Point, samples: &mut Vec<(Point, f32)>| {
            if let Some(&(last, _)) = samples.last() {
                traveled += last.distance(point);
            }
            samples.push((point, traveled));
        };
        for piece in pieces(&corners) {
            match piece {
                Piece::Straight(from, to) => {
                    if samples.is_empty() {
                        add(from, &mut samples);
                    }
                    add(to, &mut samples);
                }
                Piece::Elbow(from, corner, to) => {
                    if samples.is_empty() {
                        add(from, &mut samples);
                    }
                    for step in 1..=8 {
                        let t = step as f32 / 8.0;
                        let inverse = 1.0 - t;
                        add(
                            Point::new(
                                inverse * inverse * from.x
                                    + 2.0 * inverse * t * corner.x
                                    + t * t * to.x,
                                inverse * inverse * from.y
                                    + 2.0 * inverse * t * corner.y
                                    + t * t * to.y,
                            ),
                            &mut samples,
                        );
                    }
                }
            }
        }
        Self {
            corners,
            samples,
            wireless: false,
            key: (0, 0),
            shared: false,
        }
    }

    fn reversed(&self) -> Self {
        Self {
            wireless: self.wireless,
            key: self.key,
            shared: self.shared,
            ..Self::new(self.corners.iter().rev().copied().collect())
        }
    }

    /// The wire for a leg: reversed when it goes up, dotted along a Wi-Fi
    /// hop.
    fn for_leg(self, map: &Map, leg: &Leg) -> Self {
        let mut wire = if leg.up { self.reversed() } else { self };
        let (node, _) = map.lines[leg.line].hops[leg.hop];
        wire.wireless = map.topology.nodes[node].link.wireless.is_some();
        wire.key = (leg.line, leg.hop);
        wire
    }

    fn length(&self) -> f32 {
        self.samples.last().map_or(0.0, |&(_, length)| length)
    }

    fn point_at(&self, distance: f32) -> Point {
        let index = self
            .samples
            .partition_point(|&(_, traveled)| traveled < distance)
            .clamp(1, self.samples.len().max(2) - 1);
        let ((from, at_from), (to, at_to)) = (self.samples[index - 1], self.samples[index]);
        let t = if at_to > at_from {
            (distance - at_from) / (at_to - at_from)
        } else {
            0.0
        };
        Point::new(from.x + (to.x - from.x) * t, from.y + (to.y - from.y) * t)
    }

    /// How far `point` is from the wire.
    fn distance(&self, point: Point) -> f32 {
        self.samples
            .windows(2)
            .map(|pair| {
                let (from, to) = (pair[0].0, pair[1].0);
                let along = to - from;
                let length = along.x * along.x + along.y * along.y;
                let t = if length > 0.0 {
                    (((point.x - from.x) * along.x + (point.y - from.y) * along.y) / length)
                        .clamp(0.0, 1.0)
                } else {
                    0.0
                };
                point.distance(Point::new(from.x + along.x * t, from.y + along.y * t))
            })
            .fold(f32::INFINITY, f32::min)
    }

    fn path(&self) -> Path {
        Path::new(|builder| {
            let mut started = false;
            for piece in pieces(&self.corners) {
                match piece {
                    Piece::Straight(from, to) => {
                        if !started {
                            builder.move_to(from);
                            started = true;
                        }
                        builder.line_to(to);
                    }
                    Piece::Elbow(from, corner, to) => {
                        if !started {
                            builder.move_to(from);
                            started = true;
                        }
                        builder.quadratic_curve_to(corner, to);
                    }
                }
            }
        })
    }

    /// The end, and the way the wire runs into it.
    fn end(&self) -> (Point, Vector) {
        let count = self.samples.len();
        let end = self.samples[count - 1].0;
        let before = self.samples[count.saturating_sub(2)].0;
        (end, unit(end - before))
    }
}

fn unit(vector: Vector) -> Vector {
    let length = (vector.x * vector.x + vector.y * vector.y).sqrt();
    if length > 0.0 {
        Vector::new(vector.x / length, vector.y / length)
    } else {
        Vector::new(0.0, 1.0)
    }
}

/// Straight runs between the corners, and an elbow at each. A run between
/// two elbows gives each half of it; a wire's first and last runs give
/// their one elbow all of theirs.
fn pieces(corners: &[Point]) -> Vec<Piece> {
    let mut pieces = Vec::new();
    let Some((&first, rest)) = corners.split_first() else {
        return pieces;
    };
    let mut from = first;
    let elbows = corners.len().saturating_sub(2);
    for (index, window) in corners.windows(3).enumerate() {
        let (before, corner, after) = (window[0], window[1], window[2]);
        let share = |end: bool| if end { 1.0 } else { 0.5 };
        let radius = ELBOW
            .min(before.distance(corner) * share(index == 0))
            .min(corner.distance(after) * share(index + 1 == elbows));
        let toward = |target: Point| {
            let direction = unit(target - corner);
            Point::new(
                corner.x + direction.x * radius,
                corner.y + direction.y * radius,
            )
        };
        let (enter, leave) = (toward(before), toward(after));
        pieces.push(Piece::Straight(from, enter));
        pieces.push(Piece::Elbow(enter, corner, leave));
        from = leave;
    }
    if let Some(&last) = rest.last() {
        pieces.push(Piece::Straight(from, last));
    }
    pieces
}

struct Memory {
    cache: Cache,
    drawn: Memo<u64>,
    started: Instant,
    /// The time of the frame being drawn.
    now: Instant,
}

impl Default for Memory {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            cache: Cache::default(),
            drawn: Memo::new(0),
            started: now,
            now,
        }
    }
}

impl NetMap {
    /// Whether anything flows, so the map keeps drawing frames.
    fn animates(&self) -> bool {
        self.map
            .lines
            .iter()
            .zip(&self.placement.wires)
            .any(|(line, wires)| line.status == Status::Flowing && !wires.is_empty())
    }

    /// The point in the map's own measure.
    fn unscaled(&self, point: Point) -> Point {
        Point::new(
            (point.x - self.origin.x) / self.scale,
            (point.y - self.origin.y) / self.scale,
        )
    }

    fn card_at(&self, point: Point) -> Option<usize> {
        let point = self.unscaled(point);
        self.placement
            .cards
            .iter()
            .position(|card| card.is_some_and(|card| card.contains(point)))
    }

    /// The topmost wire near the point, with its line.
    fn wire_at(&self, point: Point) -> Option<(usize, &Wire)> {
        let point = self.unscaled(point);
        self.order.iter().rev().find_map(|&line| {
            self.placement.wires[line]
                .iter()
                .find(|wire| wire.distance(point) <= REACH)
                .map(|wire| (line, wire))
        })
    }

    /// What a click at the point brings forward: what it lands on, or
    /// nothing when that is brought forward already or it lands on neither.
    /// A wire a multicast stream's listeners share brings the whole stream.
    fn clicked(&self, point: Point) -> Option<Focus> {
        let target = self
            .card_at(point)
            .map(|node| self.map.cards[node].focus)
            .or_else(|| {
                self.wire_at(point).map(|(line, wire)| {
                    let line = &self.map.lines[line];
                    match line.stream {
                        Some((talker, output)) if wire.shared => Focus::Output(talker, output),
                        _ => line.focus,
                    }
                })
            });
        target.filter(|&focus| self.map.focus != Some(focus))
    }

    fn draw_still(&self, frame: &mut Frame, colors: &Colors) {
        let label = |frame: &mut Frame, position: Point, text: &str| {
            frame.fill_text(Text {
                content: label_text(text),
                position,
                color: colors.muted,
                size: Pixels(11.0),
                line_height: LineHeight::Absolute(Pixels(16.0)),
                font: SEMIBOLD,
                align_y: Vertical::Center,
                ..Text::default()
            });
        };
        for (band, text) in &self.placement.bands {
            frame.stroke(
                &Path::rounded_rectangle(band.position(), band.size(), border::Radius::from(16.0)),
                Stroke::default()
                    .with_color(colors.border)
                    .with_width(1.0)
                    .dashed(&[5.0, 4.0]),
            );
            label(
                frame,
                Point::new(band.x + BAND_PADDING, band.y + BAND_LABEL / 2.0),
                text,
            );
        }
        for (position, text) in &self.placement.labels {
            label(frame, *position, text);
        }

        for &index in &self.order {
            let line = &self.map.lines[index];
            let wires = &self.placement.wires[index];
            let color = paint_color(&colors.scheme, line.paint);
            let dim = if line.related { 1.0 } else { 0.12 };
            let alpha = match line.status {
                Status::Flowing => 0.6,
                _ if line.picked => 0.7,
                _ => 0.45,
            };
            let mut stroke = Stroke::default()
                .with_color(faded(color, alpha * dim))
                .with_width(2.0);
            if line.status == Status::Unsynced {
                stroke = stroke.dashed(&[6.0, 5.0]);
            }
            let drawn = || {
                wires
                    .iter()
                    .zip(&self.draws[index])
                    .filter(|(_, draws)| **draws)
                    .map(|(wire, _)| wire)
            };
            for wire in drawn() {
                let stroke = if wire.wireless {
                    Stroke {
                        line_cap: LineCap::Round,
                        ..stroke.dashed(&[0.5, 5.0])
                    }
                } else {
                    stroke
                };
                frame.stroke(&wire.path(), stroke);
            }
            // Where a stream that is not flowing stops: a tick when it is
            // advertised, a cross when its reservation failed.
            let mark = match line.status {
                Status::Advertised | Status::Failed if self.draws[index].last() == Some(&true) => {
                    wires.last()
                }
                _ => None,
            };
            if let Some(wire) = mark {
                let (end, direction) = wire.end();
                let center = Point::new(end.x - direction.x * 6.0, end.y - direction.y * 6.0);
                let mark_stroke = Stroke::default()
                    .with_color(faded(color, dim))
                    .with_width(2.5);
                if line.status == Status::Failed {
                    for (dx, dy) in [(4.0, 4.0), (4.0, -4.0)] {
                        frame.stroke(
                            &Path::line(
                                Point::new(center.x - dx, center.y - dy),
                                Point::new(center.x + dx, center.y + dy),
                            ),
                            mark_stroke,
                        );
                    }
                } else {
                    let across = Vector::new(-direction.y * 5.0, direction.x * 5.0);
                    frame.stroke(&Path::line(center - across, center + across), mark_stroke);
                }
            }
        }

        for (node, card) in self.map.cards.iter().enumerate() {
            let Some(area) = self.placement.cards[node] else {
                continue;
            };
            self.draw_card(frame, colors, card, area, self.placement.hubs[node]);
        }
    }

    fn draw_card(
        &self,
        frame: &mut Frame,
        colors: &Colors,
        card: &Card,
        area: Rectangle,
        hub: bool,
    ) {
        let (radius, padding, name_size) = if hub {
            (HUB_RADIUS, 14.0, 14.0)
        } else {
            (DEVICE_RADIUS, 10.0, 13.0)
        };
        let opacity = if card.faded { 0.4 } else { 1.0 };
        let shape =
            Path::rounded_rectangle(area.position(), area.size(), border::Radius::from(radius));
        frame.fill(&shape, blend(colors.background, colors.card, opacity));
        let mut stroke = Stroke::default()
            .with_color(faded(paint_color(&colors.scheme, card.border), opacity))
            .with_width(if card.strong { 2.0 } else { 1.0 });
        if card.dashed {
            stroke = stroke.dashed(&[5.0, 4.0]);
        }
        frame.stroke(&shape, stroke);
        let middle = area.center_y();
        frame.fill_text(Text {
            content: card.icon.to_string(),
            position: Point::new(area.x + padding + ICON / 2.0, middle),
            color: faded(colors.icon, opacity),
            size: Pixels(ICON),
            line_height: LineHeight::Absolute(Pixels(ICON)),
            font: ICONS,
            align_x: Alignment::Center,
            align_y: Vertical::Center,
            shaping: Shaping::Basic,
            ..Text::default()
        });
        let left = area.x + padding + ICON + ICON_GAP;
        let room = area.width - (padding + ICON + ICON_GAP) - padding;
        frame.fill_text(Text {
            content: fit(&card.name, SEMIBOLD, name_size, room).name,
            position: Point::new(left, middle - 7.0),
            color: faded(colors.text, opacity),
            size: Pixels(name_size),
            line_height: LineHeight::Absolute(Pixels(16.0)),
            font: SEMIBOLD,
            align_y: Vertical::Center,
            shaping: Shaping::Advanced,
            ..Text::default()
        });
        let tag_width = if card.tag.is_empty() {
            0.0
        } else {
            frame.fill_text(Text {
                content: card.tag.clone(),
                position: Point::new(left, middle + 8.0),
                color: faded(colors.muted, opacity),
                size: Pixels(11.0),
                line_height: LineHeight::Absolute(Pixels(14.0)),
                font: Font::MONOSPACE,
                align_y: Vertical::Center,
                ..Text::default()
            });
            measure(&card.tag, Font::MONOSPACE, 11.0) + 6.0
        };
        frame.fill_text(Text {
            content: fit(&card.subtitle, TEXT, 11.0, room - tag_width).name,
            position: Point::new(left + tag_width, middle + 8.0),
            color: faded(text_color(&colors.scheme, card.subtitle_paint), opacity),
            size: Pixels(11.0),
            line_height: LineHeight::Absolute(Pixels(14.0)),
            font: TEXT,
            align_y: Vertical::Center,
            shaping: Shaping::Advanced,
            ..Text::default()
        });
    }
}

impl Widget<Message, Theme, iced::Renderer> for NetMap {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<Memory>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(Memory::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(
            Length::Fixed(self.size.width),
            Length::Fixed(self.size.height),
        )
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(self.size)
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
        match event {
            Event::Window(window::Event::RedrawRequested(now)) => {
                state.now = *now;
                if !motion::reduced() && self.animates() {
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(point) = cursor.position_in(layout.bounds()) {
                    // A press on the empty map passes through, to clear the
                    // selection and what the map shows.
                    if let Some(focus) = self.clicked(point) {
                        shell.publish(Message::NetworkFocused(Some(focus)));
                        shell.capture_event();
                    }
                }
            }
            _ => {}
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let over = cursor
            .position_in(layout.bounds())
            .is_some_and(|point| self.card_at(point).is_some() || self.wire_at(point).is_some());
        if over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: Cursor,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<Memory>();
        let bounds = layout.bounds();
        let colors = Colors::of(theme);
        let signature = {
            let mut hasher = DefaultHasher::new();
            self.signature.hash(&mut hasher);
            colors.hash(&mut hasher);
            hasher.finish()
        };
        if state.drawn.get() != signature {
            state.cache.clear();
            state.drawn.set(signature);
        }

        // One beat moves the dots a step and pulses the glow once; with
        // reduced motion both hold still.
        let (phase, pulse) = if motion::reduced() {
            (0.0, 0.5)
        } else {
            let seconds = state
                .now
                .saturating_duration_since(state.started)
                .as_secs_f32();
            let phase = (seconds / BEAT).fract();
            (phase, 0.5 - 0.5 * (TAU * phase).cos())
        };
        let mut glow = Frame::new(renderer, bounds.size());
        let mut dots = Frame::new(renderer, bounds.size());
        for frame in [&mut glow, &mut dots] {
            frame.translate(self.origin);
            frame.scale(self.scale);
        }
        for &index in &self.order {
            let line = &self.map.lines[index];
            if line.status != Status::Flowing {
                continue;
            }
            let color = paint_color(&colors.scheme, line.paint);
            // A soft glow on everything lit, a strong one on the stream
            // brought forward; none on what is dimmed.
            let strength = if line.picked {
                0.08 + 0.24 * pulse
            } else {
                0.03 + 0.09 * pulse
            };
            let wires = self.placement.wires[index]
                .iter()
                .zip(&self.draws[index])
                .filter(|(_, draws)| **draws);
            for (wire, _) in wires {
                if line.related {
                    let path = wire.path();
                    let wide = if line.picked { 12.0 } else { 10.0 };
                    glow.stroke(
                        &path,
                        Stroke::default()
                            .with_color(faded(color, strength * 0.6))
                            .with_width(wide),
                    );
                    glow.stroke(
                        &path,
                        Stroke::default()
                            .with_color(faded(color, strength))
                            .with_width(wide / 2.0),
                    );
                }
                let length = wire.length();
                let mut distance = phase * STEP;
                let alpha = if line.related { 1.0 } else { 0.12 };
                while distance < length {
                    dots.fill(
                        &Path::circle(wire.point_at(distance), 1.4),
                        faded(color, alpha),
                    );
                    distance += STEP;
                }
            }
        }
        let still = state.cache.draw(renderer, bounds.size(), |frame| {
            frame.translate(self.origin);
            frame.scale(self.scale);
            self.draw_still(frame, &colors);
        });
        let (glow, dots) = (glow.into_geometry(), dots.into_geometry());
        // A layer is not clipped by the scrollable holding the map, so it
        // takes only the part in view.
        let visible = bounds.intersection(viewport).unwrap_or_default();
        renderer.with_layer(visible, |renderer| {
            renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
                renderer.draw_geometry(glow);
                renderer.draw_geometry(still);
                renderer.draw_geometry(dots);
            });
        });
    }
}

/// A stroke drawn in dashes of the given lengths.
trait Dashed {
    fn dashed(self, segments: &'static [f32]) -> Self;
}

impl Dashed for Stroke<'static> {
    fn dashed(self, segments: &'static [f32]) -> Self {
        Stroke {
            line_dash: LineDash {
                segments,
                offset: 0,
            },
            ..self
        }
    }
}

/// The colors the map draws with, from the theme's scheme.
#[derive(Debug, Clone, Copy)]
struct Colors {
    scheme: Scheme,
    background: Color,
    card: Color,
    border: Color,
    icon: Color,
    text: Color,
    muted: Color,
}

impl Colors {
    fn of(theme: &Theme) -> Self {
        let scheme = Scheme::of(theme);
        Self {
            scheme,
            background: scheme.surface_container_low,
            card: scheme.surface_container,
            border: scheme.outline_variant,
            icon: scheme.on_surface_variant,
            text: scheme.on_surface,
            muted: scheme.outline,
        }
    }
}

impl Hash for Colors {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        for color in [
            self.background,
            self.card,
            self.border,
            self.icon,
            self.text,
            self.muted,
            self.scheme.error,
            paint_color(&self.scheme, Paint::Clock),
        ] {
            color.into_rgba8().hash(hasher);
        }
    }
}

impl From<NetMap> for Element<'_, Message> {
    fn from(map: NetMap) -> Self {
        Element::new(map)
    }
}

#[cfg(test)]
mod tests {
    use atdecc::aem::AvbInfo;
    use atdecc::{ClockIdentity, EntityId};
    use scramble_ui::icon::Icon;

    use super::super::{Line, NodeKey, paints};
    use super::*;
    use crate::topology::{EntityReport, HostReport, InterfaceReport, Route};

    fn info(grandmaster: ClockIdentity, delay: u32) -> AvbInfo {
        let mut payload = [0u8; 20];
        payload[4..12].copy_from_slice(&grandmaster.0.to_be_bytes());
        payload[12..16].copy_from_slice(&delay.to_be_bytes());
        AvbInfo::decode(&payload).unwrap()
    }

    /// A stream as the tests bind it: talker, listener, name and status.
    type Bound = (EntityId, EntityId, Status);

    /// The map of a topology with these streams, as the view builds it,
    /// each talker sending one multicast stream.
    fn draw(topology: Topology, streams: &[Bound], clock: bool, focus: Option<Focus>) -> Map {
        draw_as(topology, streams, clock, focus, true)
    }

    /// The map of a topology with these streams, each talker sending one
    /// stream, to a multicast address or to each listener's own.
    fn draw_as(
        topology: Topology,
        streams: &[Bound],
        clock: bool,
        focus: Option<Focus>,
        multicast: bool,
    ) -> Map {
        let paints = paints(
            &streams
                .iter()
                .map(|&(talker, _, status)| ((talker, 0), status))
                .collect::<Vec<_>>(),
        );
        let lines: Vec<Line> = if clock {
            (0..topology.nodes.len())
                .filter(|&node| topology.nodes[node].parent.is_some())
                .map(|node| {
                    let synced = topology.nodes[node].link.synced;
                    Line {
                        hops: vec![(node, false)],
                        status: if synced {
                            Status::Flowing
                        } else {
                            Status::Unsynced
                        },
                        paint: if synced {
                            Paint::Clock
                        } else {
                            Paint::Unsynced
                        },
                        related: true,
                        picked: false,
                        focus: Focus::Node(NodeKey::Host),
                        stream: None,
                    }
                })
                .collect()
        } else {
            streams
                .iter()
                .zip(&paints)
                .enumerate()
                .map(|(index, (&(talker, listener, status), &paint))| {
                    let route = match status {
                        Status::Flowing => topology.route(talker, listener),
                        _ => topology
                            .placed(talker)
                            .map(|node| Route {
                                hops: vec![(node, true)],
                                stops: topology.nodes[node].parent,
                            })
                            .unwrap_or_default(),
                    };
                    let picked = focus == Some(Focus::Stream(listener, index as u16))
                        || focus == Some(Focus::Output(talker, 0));
                    Line {
                        hops: route.hops,
                        status,
                        paint,
                        related: focus.is_none() || picked,
                        picked,
                        focus: Focus::Stream(listener, index as u16),
                        stream: multicast.then_some((talker, 0)),
                    }
                })
                .collect()
        };
        let cards = topology
            .nodes
            .iter()
            .map(|node| {
                let entity = match node.kind {
                    Kind::Entity { entity_id, .. } => Some(entity_id),
                    _ => None,
                };
                let (icon, name) = match node.kind {
                    Kind::Bridge if node.access_point => {
                        (Icon::WifiTethering, crate::fl!("netmap-access-point"))
                    }
                    Kind::Bridge => (Icon::Hub, node.name.clone()),
                    Kind::Host => (Icon::Computer, crate::fl!("netmap-this-computer")),
                    Kind::Entity { .. } => (Icon::GraphicEq, node.name.clone()),
                    Kind::Clock => (Icon::Schedule, node.name.clone()),
                    Kind::Switch { .. } => (Icon::Lan, crate::fl!("netmap-plain-switch")),
                };
                let mut sending = streams
                    .iter()
                    .filter(|bound| Some(bound.0) == entity)
                    .count();
                if multicast {
                    sending = sending.min(1);
                }
                let receiving = streams
                    .iter()
                    .filter(|bound| Some(bound.1) == entity)
                    .count();
                let failed = streams
                    .iter()
                    .any(|bound| Some(bound.1) == entity && bound.2 == Status::Failed);
                let (subtitle, paint) = match (clock, node.kind, node.apart) {
                    (_, _, Some(_)) => (crate::fl!("netmap-domain-unknown"), Paint::Failed),
                    (true, Kind::Switch { .. }, _) => {
                        ("AVB Lite PTP, domain 0".to_owned(), Paint::Muted)
                    }
                    (true, _, _) if node.parent.is_none() => {
                        (crate::fl!("avb-interface-grandmaster"), Paint::Clock)
                    }
                    (true, _, _) if node.link.wireless.is_some_and(|hop| hop.holdover) => {
                        (crate::fl!("wireless-holdover"), Paint::Failed)
                    }
                    (true, _, _) if node.link.wireless.is_some_and(|hop| hop.locked) => {
                        (crate::fl!("wireless-locked"), Paint::Muted)
                    }
                    (true, _, _) if node.link.synced => (crate::fl!("netmap-synced"), Paint::Muted),
                    (true, _, _) => (crate::fl!("netmap-not-synced"), Paint::Failed),
                    (false, Kind::Entity { .. }, _) if node.children.is_empty() => {
                        let mut text = [(sending, "out"), (receiving, "in")]
                            .iter()
                            .filter(|(count, _)| *count > 0)
                            .map(|(count, what)| format!("{count} {what}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        if text.is_empty() {
                            text = "None".to_owned();
                        }
                        if failed {
                            (format!("{text}, 1 failed"), Paint::Failed)
                        } else {
                            (text, Paint::Muted)
                        }
                    }
                    _ => ("Bridge".to_owned(), Paint::Muted),
                };
                Card {
                    icon: icon.codepoint(),
                    name,
                    tag: String::new(),
                    subtitle,
                    subtitle_paint: paint,
                    border: if clock
                        && node.parent.is_none()
                        && node.apart.is_none()
                        && !matches!(node.kind, Kind::Switch { .. })
                    {
                        Paint::Clock
                    } else if failed && !clock {
                        Paint::Failed
                    } else {
                        Paint::Plain
                    },
                    strong: false,
                    dashed: node.apart.is_some()
                        || matches!(node.kind, Kind::Switch { .. })
                        || (clock && node.parent.is_some() && !node.link.synced),
                    faded: false,
                    focus: Focus::Node(NodeKey::Host),
                }
            })
            .collect();
        Map {
            topology,
            cards,
            lines,
            focus,
        }
    }

    const SWITCH: ClockIdentity = ClockIdentity(0x0001_f2ff_feff_3b14);
    const MAC_CLOCK: ClockIdentity = ClockIdentity(0xeab6_9c6d_c98c_0002);
    const WIRED_CLOCK: ClockIdentity = ClockIdentity(0xe8f6_0aff_fee0_9220);
    const WIFI_CLOCK: ClockIdentity = ClockIdentity(0xfc01_2cff_fefd_fe80);
    const MAC: EntityId = EntityId(0xd111_e597_f544_8000);
    const WIRED: EntityId = EntityId(0xe8f6_0ae0_9220_0000);
    const WIFI: EntityId = EntityId(0xfc01_2cfd_fe80_0000);

    /// The bench: two entities under the switch, which is the grandmaster,
    /// this computer on it, and one entity on no tree.
    fn bench(clock: bool) -> Map {
        let mac_path = [SWITCH, MAC_CLOCK];
        let wired_path = [SWITCH, WIRED_CLOCK];
        let (mac_info, wired_info, wifi_info) =
            (info(SWITCH, 58), info(SWITCH, 432), info(WIFI_CLOCK, 0));
        let report = |entity_id, name, clock, path, info| EntityReport {
            entity_id,
            name,
            interfaces: vec![InterfaceReport {
                index: 0,
                clock,
                path,
                info,
                wireless: None,
                lite: None,
                gptp: true,
            }],
        };
        let mut topology = Topology::build(
            &[
                report(
                    MAC,
                    "Mac mini",
                    MAC_CLOCK,
                    Some(&mac_path[..]),
                    Some(&mac_info),
                ),
                report(
                    WIRED,
                    "AVB Example Entity",
                    WIRED_CLOCK,
                    Some(&wired_path[..]),
                    Some(&wired_info),
                ),
                report(
                    WIFI,
                    "AVB Example Entity",
                    WIFI_CLOCK,
                    None,
                    Some(&wifi_info),
                ),
            ],
            Some(HostReport::Heard(SWITCH, 6, false)),
        );
        for node in &mut topology.nodes {
            if node.kind == Kind::Bridge {
                node.name = "Mark of the Unicorn".to_owned();
            }
        }
        draw(
            topology,
            &[
                (WIRED, MAC, Status::Flowing),
                (MAC, WIRED, Status::Flowing),
                (WIRED, WIFI, Status::Advertised),
            ],
            clock,
            None,
        )
    }

    /// The bench with an AVB Wireless access point under the switch: the
    /// Wi-Fi entity a station reporting its path, and another station
    /// following beacons, holding over.
    fn wireless(clock: bool) -> Map {
        use atdecc::wireless::WirelessFlags;
        const ACCESS_POINT: ClockIdentity = ClockIdentity(0x30ed_a0ff_fe11_2233);
        const BEACONS: EntityId = EntityId(0xfc01_2cfd_fe80_0001);
        const BEACONS_CLOCK: ClockIdentity = ClockIdentity(0xfc01_2cff_fefd_fe81);
        let mac_path = [SWITCH, MAC_CLOCK];
        let wired_path = [SWITCH, WIRED_CLOCK];
        let wifi_path = [SWITCH, ACCESS_POINT, WIFI_CLOCK];
        let (mac_info, wired_info, wifi_info) =
            (info(SWITCH, 58), info(SWITCH, 432), info(SWITCH, 12_000));
        let locked = crate::topology::test_station(ACCESS_POINT, WirelessFlags::LOCKED);
        let holding = crate::topology::test_station(ACCESS_POINT, WirelessFlags::HOLDOVER);
        let report = |entity_id, name, clock, path, info, wireless| EntityReport {
            entity_id,
            name,
            interfaces: vec![InterfaceReport {
                index: 0,
                clock,
                path,
                info,
                wireless,
                lite: None,
                gptp: true,
            }],
        };
        let mut topology = Topology::build(
            &[
                report(
                    MAC,
                    "Mac mini",
                    MAC_CLOCK,
                    Some(&mac_path[..]),
                    Some(&mac_info),
                    None,
                ),
                report(
                    WIRED,
                    "AVB Example Entity",
                    WIRED_CLOCK,
                    Some(&wired_path[..]),
                    Some(&wired_info),
                    None,
                ),
                report(
                    WIFI,
                    "Wi-Fi talker",
                    WIFI_CLOCK,
                    Some(&wifi_path[..]),
                    Some(&wifi_info),
                    Some(&locked),
                ),
                report(
                    BEACONS,
                    "Wi-Fi listener",
                    BEACONS_CLOCK,
                    None,
                    Some(&wifi_info),
                    Some(&holding),
                ),
            ],
            Some(HostReport::Heard(SWITCH, 6, true)),
        );
        for node in &mut topology.nodes {
            if node.clock == Some(SWITCH) {
                node.name = "Mark of the Unicorn".to_owned();
            }
        }
        draw(
            topology,
            &[
                (WIFI, MAC, Status::Flowing),
                (WIRED, BEACONS, Status::Flowing),
            ],
            clock,
            None,
        )
    }

    /// The bench on a gPTP network with devices that fell back to AVB Lite
    /// behind plain switches, following two grandmasters, and one in each
    /// band: running AVB Lite only, set to it, on another gPTP domain, and
    /// on no tree for a fault.
    fn mixed(clock: bool) -> Map {
        use atdecc::lite::{FallbackReason, LiteFlags, LiteStatus, PtpProfile};
        let own = |place: u64| ClockIdentity(0x0011_22ff_fe00_0000 | place);
        let entity = |place: u64| EntityId(0x0011_2200_0000_0000 | place);
        let elsewhere = ClockIdentity(0x0066_77ff_fe00_0001);
        let running = |reason, grandmaster, offset: i32| LiteStatus {
            interface: 0,
            flags: LiteFlags::CAPABLE | LiteFlags::ACTIVE | LiteFlags::OFFSET_VALID,
            fallback_reason: reason,
            ptp_profile: PtpProfile::AVB_LITE_PTP,
            ptp_domain: 0,
            media_vlan_id: 2,
            unicast_fanout_limit: 2,
            link_speed: 100,
            committed_egress: 0,
            grandmaster,
            offset_from_grandmaster: offset,
        };
        let statuses = [
            running(FallbackReason::PDELAY_UNANSWERED, own(1), 0),
            running(FallbackReason::MULTIPLE_RESPONDERS, own(1), -180),
            running(FallbackReason::MULTIPLE_RESPONDERS, own(1), 900),
            running(FallbackReason::ENDPOINT_TLV, elsewhere, 72_000),
            running(FallbackReason::PDELAY_UNANSWERED, own(1), 40),
            running(FallbackReason::CONFIGURED, own(1), 40),
        ];
        let names = [
            "Synthia",
            "Power Amp",
            "Condenser Mic",
            "Wireless ESP3",
            "Lite gadget",
            "Test rig",
        ];
        let mac_path = [SWITCH, MAC_CLOCK];
        let wired_path = [SWITCH, WIRED_CLOCK];
        let on_domain = |grandmaster: ClockIdentity, domain: u8| {
            let mut payload = [0u8; 20];
            payload[4..12].copy_from_slice(&grandmaster.0.to_be_bytes());
            payload[16] = domain;
            AvbInfo::decode(&payload).unwrap()
        };
        let (mac_info, wired_info) = (info(SWITCH, 58), info(SWITCH, 432));
        let (other_info, lost_info) = (on_domain(own(8), 1), on_domain(elsewhere, 0));
        let report = |entity_id, name, clock, path, info, lite, gptp| EntityReport {
            entity_id,
            name,
            interfaces: vec![InterfaceReport {
                index: 0,
                clock,
                path,
                info,
                wireless: None,
                lite,
                gptp,
            }],
        };
        let mut reports: Vec<EntityReport<'_>> = (1..=6u64)
            .map(|place| {
                let index = place as usize - 1;
                report(
                    entity(place),
                    names[index],
                    own(place),
                    None,
                    None,
                    Some(&statuses[index]),
                    place != 5,
                )
            })
            .collect();
        reports.extend([
            report(
                MAC,
                "Mac mini",
                MAC_CLOCK,
                Some(&mac_path[..]),
                Some(&mac_info),
                None,
                true,
            ),
            report(
                WIRED,
                "AVB Example Entity",
                WIRED_CLOCK,
                Some(&wired_path[..]),
                Some(&wired_info),
                None,
                true,
            ),
            report(
                entity(8),
                "Domain one box",
                own(8),
                None,
                Some(&other_info),
                None,
                true,
            ),
            report(
                entity(9),
                "Lost box",
                own(9),
                None,
                Some(&lost_info),
                None,
                true,
            ),
        ]);
        let mut topology = Topology::build(&reports, Some(HostReport::Heard(SWITCH, 6, true)));
        for node in &mut topology.nodes {
            if node.kind == Kind::Bridge {
                node.name = "Mark of the Unicorn".to_owned();
            }
        }
        draw_as(
            topology,
            &[
                (WIRED, MAC, Status::Flowing),
                (entity(1), entity(2), Status::Flowing),
                (entity(3), entity(1), Status::Flowing),
                (entity(4), entity(2), Status::Advertised),
            ],
            clock,
            None,
            false,
        )
    }

    /// A larger show: a core bridge as grandmaster with a stage bridge and
    /// a front of house bridge under it, devices under each, and some
    /// straight under the core.
    fn show(clock: bool, focus: Option<Focus>) -> Map {
        show_as(clock, focus, true)
    }

    /// The show, each talker's stream sent to a multicast address or to
    /// each listener's own.
    fn show_as(clock: bool, focus: Option<Focus>, multicast: bool) -> Map {
        let bridge = |id: u64| ClockIdentity(0x00aa_bbff_fe00_0000 | id);
        let (core, stage, front) = (bridge(1), bridge(2), bridge(3));
        let devices: [(&str, ClockIdentity); 18] = [
            ("Stage box A", stage),
            ("Stage box B", stage),
            ("Wireless rack", stage),
            ("Monitor console", stage),
            ("Monitor amp 1", stage),
            ("Monitor amp 2", stage),
            ("IEM rack", stage),
            ("FOH console", front),
            ("Playback Mac", front),
            ("Main amp L", front),
            ("Main amp R", front),
            ("Sub amp", front),
            ("Delay amp", front),
            ("Recorder", core),
            ("Broadcast", core),
            ("Video server", core),
            ("Comms base", core),
            ("Press box", core),
        ];
        let paths: Vec<(ClockIdentity, Vec<ClockIdentity>)> = devices
            .iter()
            .enumerate()
            .map(|(place, &(_, above))| {
                let own = ClockIdentity(0x0011_22ff_fe00_0000 | place as u64);
                let path = if above == core {
                    vec![core, own]
                } else {
                    vec![core, above, own]
                };
                (own, path)
            })
            .collect();
        let reports: Vec<EntityReport<'_>> = devices
            .iter()
            .zip(&paths)
            .enumerate()
            .map(|(place, (&(name, _), (own, path)))| EntityReport {
                entity_id: EntityId(place as u64 + 1),
                name,
                interfaces: vec![InterfaceReport {
                    index: 0,
                    clock: *own,
                    path: Some(path),
                    info: None,
                    wireless: None,
                    lite: None,
                    gptp: true,
                }],
            })
            .collect();
        let mut topology = Topology::build(&reports, None);
        for node in &mut topology.nodes {
            node.name = match node.clock {
                Some(clock) if clock == core => "Core bridge".to_owned(),
                Some(clock) if clock == stage => "Stage bridge".to_owned(),
                Some(clock) if clock == front => "FOH bridge".to_owned(),
                _ => node.name.clone(),
            };
            if node.name == "Monitor amp 2" {
                node.link.synced = false;
            }
        }
        let id = |name: &str| {
            EntityId(devices.iter().position(|device| device.0 == name).unwrap() as u64 + 1)
        };
        let streams: Vec<Bound> = [
            ("Stage box A", "FOH console", Status::Flowing),
            ("Stage box B", "FOH console", Status::Flowing),
            ("Wireless rack", "FOH console", Status::Flowing),
            ("Stage box A", "Recorder", Status::Flowing),
            ("Stage box B", "Recorder", Status::Flowing),
            ("Monitor console", "IEM rack", Status::Flowing),
            ("Monitor console", "Monitor amp 1", Status::Flowing),
            ("Monitor console", "Monitor amp 2", Status::Advertised),
            ("FOH console", "Main amp L", Status::Flowing),
            ("FOH console", "Main amp R", Status::Flowing),
            ("FOH console", "Sub amp", Status::Flowing),
            ("FOH console", "Delay amp", Status::Failed),
            ("FOH console", "Broadcast", Status::Flowing),
            ("Playback Mac", "FOH console", Status::Flowing),
            ("Video server", "Broadcast", Status::Flowing),
            ("Video server", "Press box", Status::Failed),
            ("Comms base", "Press box", Status::Flowing),
            ("Comms base", "FOH console", Status::Flowing),
        ]
        .iter()
        .map(|&(talker, listener, status)| (id(talker), id(listener), status))
        .collect();
        draw_as(topology, &streams, clock, focus, multicast)
    }

    /// A multicast stream takes one wire on each link however many of its
    /// listeners lie beyond it, and branches where their paths part; a
    /// unicast one takes a wire for each listener all the way.
    #[test]
    fn multicast_listeners_share_wires_and_unicast_ones_do_not() {
        let named = |map: &Map, name: &str| {
            map.topology
                .nodes
                .iter()
                .position(|node| node.name == name)
                .unwrap()
        };
        // The wires drawn for a talker's streams on the link up from a node.
        let on_link = |map: &Map, talker: &str, link: NodeId| {
            let talker = named(map, talker);
            let placement = fan(map, f32::INFINITY);
            let mut keys: Vec<(usize, usize)> = map
                .lines
                .iter()
                .zip(&placement.wires)
                .filter(|(line, _)| line.hops.first().is_some_and(|&(hop, _)| hop == talker))
                .flat_map(|(line, wires)| {
                    line.hops
                        .iter()
                        .zip(wires)
                        .filter(|((hop, _), _)| *hop == link)
                        .map(|(_, wire)| wire.key)
                })
                .collect();
            keys.sort_unstable();
            keys.dedup();
            keys.len()
        };
        let up_from = |map: &Map, name: &str| map.topology.nodes[named(map, name)].parent.unwrap();
        // Stage box A sends to the FOH console and the recorder, beyond the
        // stage bridge; the FOH console to three amps and the broadcast
        // feed flowing, and to a fourth amp whose reservation failed.
        let multicast = show(false, None);
        let unicast = show_as(false, None, false);
        for (map, own, stage, console) in [(&multicast, 1, 1, 2), (&unicast, 2, 2, 5)] {
            let stage_box = named(map, "Stage box A");
            let foh = named(map, "FOH console");
            assert_eq!(on_link(map, "Stage box A", stage_box), own);
            assert_eq!(
                on_link(map, "Stage box A", up_from(map, "Stage box A")),
                stage
            );
            assert_eq!(on_link(map, "FOH console", foh), console);
            assert_eq!(on_link(map, "FOH console", up_from(map, "FOH console")), 1);
        }
        // Every line still has a wire for each link it crosses.
        let placement = fan(&multicast, f32::INFINITY);
        for (line, wires) in multicast.lines.iter().zip(&placement.wires) {
            assert_eq!(wires.len(), line.hops.len());
        }
    }

    /// A click on a wire a multicast stream's listeners share brings the
    /// whole stream forward; on a branch, the stream to that listener.
    #[test]
    fn a_shared_wire_brings_the_whole_stream_forward() {
        let map = show(false, None);
        let talker = map
            .topology
            .nodes
            .iter()
            .position(|node| node.name == "Stage box A")
            .unwrap();
        let drawn = super::map(map, Size::new(f32::INFINITY, f32::INFINITY));
        let middle = |wire: &Wire| {
            let point = wire.point_at(wire.length() / 2.0);
            Point::new(
                point.x * drawn.scale + drawn.origin.x,
                point.y * drawn.scale + drawn.origin.y,
            )
        };
        let mut shared = None;
        let mut branch = None;
        for (line, wires) in drawn.map.lines.iter().zip(&drawn.placement.wires) {
            if line.stream.map(|(entity, _)| entity) != talker_entity(&drawn.map, talker) {
                continue;
            }
            for (&(node, _), wire) in line.hops.iter().zip(wires) {
                if node == talker {
                    shared = Some(middle(wire));
                } else if !wire.shared && branch.is_none() {
                    branch = Some((middle(wire), line.focus));
                }
            }
        }
        let entity = talker_entity(&drawn.map, talker).unwrap();
        assert_eq!(
            drawn.clicked(shared.unwrap()),
            Some(Focus::Output(entity, 0))
        );
        let (point, focus) = branch.unwrap();
        assert_eq!(drawn.clicked(point), Some(focus));
    }

    fn talker_entity(map: &Map, node: NodeId) -> Option<EntityId> {
        match map.topology.nodes[node].kind {
            Kind::Entity { entity_id, .. } => Some(entity_id),
            _ => None,
        }
    }

    #[test]
    fn wires_end_on_the_cards_they_join() {
        let map = show(false, None);
        let placement = fan(&map, f32::INFINITY);
        let near = |point: Point, card: Rectangle| {
            let inflated = Rectangle {
                x: card.x - 0.5,
                y: card.y - 0.5,
                width: card.width + 1.0,
                height: card.height + 1.0,
            };
            inflated.contains(point)
        };
        for (line, wires) in map.lines.iter().zip(&placement.wires) {
            assert_eq!(wires.len(), line.hops.len());
            for (&(node, up), wire) in line.hops.iter().zip(wires) {
                let parent = map.topology.nodes[node].parent.unwrap();
                let (from, to) = if up { (node, parent) } else { (parent, node) };
                let start = wire.samples[0].0;
                let (end, _) = wire.end();
                assert!(near(start, placement.cards[from].unwrap()), "{start:?}");
                assert!(near(end, placement.cards[to].unwrap()), "{end:?}");
            }
        }
        // No card overlaps another.
        let cards: Vec<Rectangle> = placement.cards.iter().flatten().copied().collect();
        for (place, card) in cards.iter().enumerate() {
            for other in &cards[place + 1..] {
                assert!(card.intersection(other).is_none(), "{card:?} {other:?}");
            }
        }
    }

    /// Writes pictures of the bench and of a larger show to the PNG files
    /// the variable `NETWORK_PICTURE` names, with what each shows added.
    #[test]
    #[ignore = "writes pictures to look at"]
    fn pictures() {
        let Ok(path) = std::env::var("NETWORK_PICTURE") else {
            return;
        };
        let theme = scramble_ui::scheme::theme("triib".to_owned(), crate::app::TRIIB_SEED, true);
        let natural = Size::new(f32::INFINITY, f32::INFINITY);
        let phone = Size::new(390.0, 760.0);
        let picked = Some(Focus::Stream(EntityId(14), 3));
        for (suffix, map, area) in [
            ("bench", bench(false), natural),
            ("bench-clock", bench(true), natural),
            ("bench-phone", bench(false), phone),
            ("wireless", wireless(false), natural),
            ("wireless-clock", wireless(true), natural),
            ("show", show(false, None), natural),
            ("show-clock", show(true, None), natural),
            ("show-picked", show(false, picked), natural),
            (
                "show-stream-picked",
                show(false, Some(Focus::Output(EntityId(8), 0))),
                natural,
            ),
            ("show-unicast", show_as(false, None, false), natural),
            ("show-medium", show(false, None), Size::new(700.0, 600.0)),
            ("show-phone", show(false, None), phone),
            ("mixed", mixed(false), natural),
            ("mixed-clock", mixed(true), natural),
            ("mixed-medium", mixed(false), Size::new(700.0, 760.0)),
            ("mixed-phone", mixed(true), phone),
        ] {
            let file = format!("{path}-{suffix}.png");
            // The simulator adds its renderer's name to the file name.
            for renderer in ["tiny-skia", "wgpu"] {
                let _ = std::fs::remove_file(format!("{path}-{suffix}-{renderer}.png"));
            }
            let settings = iced::Settings {
                fonts: scramble_ui::font::files().collect(),
                default_font: TEXT,
                antialiasing: true,
                ..iced::Settings::default()
            };
            let drawn = super::map(map, area);
            let window = if area.width.is_finite() {
                area
            } else {
                drawn.size
            };
            let mut simulator =
                iced_test::Simulator::with_size(settings, window, Element::from(drawn));
            let snapshot = simulator.snapshot(&theme).expect("draws");
            assert!(snapshot.matches_image(&file).expect("writes"));
        }
    }
}
