//! The network view: the gPTP tree with each bridge's devices in a column
//! under it, showing the clock, the audio streams or the media clock
//! streams, each stream on a wire of its own colored by its talker, and
//! beside it the details of what is brought forward.

mod map;

use std::collections::HashMap;

use atdecc::descriptor::DescriptorType;
use atdecc::lite::{LiteFlags, LiteStatus};
use atdecc::model::{EntityModel, EnumerationFailure, EnumerationState};
use atdecc::{ClockIdentity, DiscoveredEntity, EntityId};
use iced::widget::{button as plain_button, container, mouse_area, scrollable, space};
use iced::{Center, Element, Fill, Length, Theme};
use scramble_ui::button::{self, Kind, Size};
use scramble_ui::component;
use scramble_ui::font::{Type, styled};
use scramble_ui::icon::{self, Icon};
use scramble_ui::{Scheme, shape, style};
use scramble_ui::{column, dir, line, row};

use crate::app::{Message, NetworkState, Triib};
use crate::describe;
use crate::fl;
use crate::settings::NetworkShows;
use crate::topology::{
    Apart, EntityReport, HostReport, InterfaceReport, Kind as NodeKind, LiteReport, NodeId, Route,
    Topology, WirelessLink,
};
use crate::view::VIEW_MARGIN;

const PANEL_WIDTH: f32 = 360.0;
const NARROW_PANEL_WIDTH: f32 = 300.0;
/// Below this width the side panel gives way to the map, a button in the
/// header switching between the two.
const WIDE: f32 = 880.0;
/// From this width the side panel takes its full width.
const ROOMY: f32 = 1200.0;
/// Below this width the header leaves out its title and summary.
const TITLED: f32 = 1040.0;

/// What the view brings forward: a stream, or a node with its streams or
/// its clock path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Focus {
    /// A stream, by its listener's stream input.
    Stream(EntityId, u16),
    Node(NodeKey),
}

/// A node, by what stays the same while the topology is built again.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKey {
    Entity(EntityId),
    Clock(ClockIdentity),
    Host,
}

/// A color on the map or in the details, the theme deciding the exact
/// color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Paint {
    /// The gPTP clock.
    Clock,
    /// A shade of its talker's hue, `shade` from 0 (darkest) to 100.
    Talker { hue: u16, shade: u8 },
    /// A failed reservation, or a problem.
    Failed,
    /// Advertised with no listener ready.
    Idle,
    /// A link gPTP does not run on.
    Unsynced,
    /// The theme's text color.
    Text,
    /// The theme's softer text color.
    Soft,
    /// The theme's muted color, for secondary text.
    Muted,
    /// The theme's outline color, for borders.
    Plain,
}

/// How far a stream, or the clock on a link, gets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    /// Audio or the clock is flowing.
    Flowing,
    /// Bound and announced, but no listener is ready, so nothing is sent.
    Advertised,
    /// The talker's reservation failed.
    Failed,
    /// gPTP does not run on the link.
    Unsynced,
}

/// A wire on the map: a stream, or in the gPTP view the link up from a
/// node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Line {
    /// The links it crosses in order, each named by its lower node, and
    /// whether it goes up that link.
    pub hops: Vec<(NodeId, bool)>,
    pub status: Status,
    pub paint: Paint,
    /// Lit under what is brought forward.
    pub related: bool,
    /// The stream brought forward.
    pub picked: bool,
    /// What clicking it brings forward.
    pub focus: Focus,
}

/// A node as its card shows it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Card {
    pub icon: char,
    pub name: String,
    /// The end of the node's MAC address, for telling alike names apart.
    pub tag: String,
    pub subtitle: String,
    pub subtitle_paint: Paint,
    pub border: Paint,
    /// A two pixel border, for what is brought forward.
    pub strong: bool,
    pub dashed: bool,
    /// Faded, taking no part in what is brought forward.
    pub faded: bool,
    pub focus: Focus,
}

/// Everything the map draws.
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Map {
    pub topology: Topology,
    /// By node.
    pub cards: Vec<Card>,
    pub lines: Vec<Line>,
    /// What is brought forward, so clicking it again lets it go.
    pub focus: Option<Focus>,
}

/// A bound stream.
#[derive(Debug, Clone)]
struct Stream {
    input: (EntityId, u16),
    talker: EntityId,
    name: String,
    talker_name: String,
    listener_name: String,
    media_clock: bool,
    status: Status,
    reason: String,
    route: Route,
}

impl Stream {
    fn focus(&self) -> Focus {
        Focus::Stream(self.input.0, self.input.1)
    }
}

pub fn view(triib: &Triib) -> Element<'_, Message> {
    iced::widget::responsive(move |size| view_at(triib, size.width)).into()
}

/// The view for `width` pixels: the map with the side panel beside it, or
/// on narrow windows one of the two at a time.
fn view_at(triib: &Triib, width: f32) -> Element<'_, Message> {
    let wide = width >= WIDE;
    let list_open = !wide && triib.network_list;
    let shows = triib.settings.network_shows;
    // Where the network runs AVB Lite, the clock is PTP's, end to end.
    let lite = lite_topology(triib);
    let ptp = lite.is_some();
    let topology = match lite {
        Some(lite) if shows == NetworkShows::Clock => lite,
        _ => topology_of(triib),
    };
    let streams: Vec<Stream> = match shows {
        NetworkShows::Clock => Vec::new(),
        _ => streams(triib, &topology)
            .into_iter()
            .filter(|stream| stream.media_clock == (shows == NetworkShows::MediaClock))
            .collect(),
    };
    let paints = paints(
        &streams
            .iter()
            .map(|stream| (stream.talker, stream.status))
            .collect::<Vec<_>>(),
    );
    // What is brought forward, as long as it is still there.
    let focus = triib.network_focus.filter(|focus| match focus {
        Focus::Stream(..) => streams.iter().any(|stream| stream.focus() == *focus),
        Focus::Node(key) => find(&topology, *key).is_some(),
    });
    let (header, focused) = header(&topology, ptp, shows, &streams, focus, width, list_open);
    // The chips a view's margin from the toolbar and 16 from what is below,
    // as the other views' are. On narrow windows the list's toggle makes
    // their row 8 taller.
    let padding = iced::Padding {
        top: if wide { VIEW_MARGIN } else { VIEW_MARGIN - 4.0 },
        ..iced::Padding::new(VIEW_MARGIN)
    };
    let below = if wide { 16.0 } else { 12.0 };

    if topology.nodes.is_empty() {
        return column![
            header,
            component::empty_state(Icon::Hub, fl!("netmap-empty"), fl!("netmap-empty-note"),)
        ]
        .spacing(below)
        .padding(padding)
        .into();
    }

    let lines = lines(&topology, shows, &streams, &paints, focus);
    let cards = cards(triib, &topology, shows, &streams, &paints, &lines, focus);
    let data = Map {
        topology: topology.clone(),
        cards,
        lines,
        focus,
    };
    // The map fits the room it is given, so it is laid out for that room.
    // A press on its empty space clears the selection.
    let canvas = mouse_area(
        container(iced::widget::responsive(move |area| {
            scrollable(map::map(data.clone(), area))
                .direction(scrollable::Direction::Both {
                    vertical: component::thin_scrollbar(),
                    horizontal: component::thin_scrollbar(),
                })
                .style(style::scrollbar)
                .width(Fill)
                .height(Fill)
                .into()
        }))
        .width(Fill)
        .height(Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Scheme::of(theme).surface_container_low.into()),
            border: iced::border::rounded(shape::LARGE),
            ..container::Style::default()
        }),
    )
    .on_press(Message::SelectionCleared);
    let panel_width = if !wide {
        Length::Fill
    } else if width >= ROOMY {
        Length::Fixed(PANEL_WIDTH)
    } else {
        Length::Fixed(NARROW_PANEL_WIDTH)
    };
    let panel = || details(triib, &topology, shows, &streams, &paints, focus).view(panel_width);
    let mut body: Element<'_, Message> = if list_open { panel() } else { canvas.into() };
    // What is brought forward when the header has no room for it: below
    // the map, which stays where it is as it comes and goes.
    if let Some(focused) = focused {
        let focused = container(focused)
            .width(Fill)
            .align_x(dir::horizontal_start());
        body = column![body, focused].spacing(16).height(Fill).into();
    }
    if wide {
        body = row![body, panel()].spacing(16).height(Fill).into();
    }
    column![header, body]
        .spacing(below)
        .padding(padding)
        .width(Fill)
        .height(Fill)
        .into()
}

/// The title, the choice of what to show, a summary and what is brought
/// forward, with on narrow windows the list's toggle; and what is brought
/// forward on its own instead, for below the map, when the header's row has
/// no room for it.
fn header<'a>(
    topology: &Topology,
    ptp: bool,
    shows: NetworkShows,
    streams: &[Stream],
    focus: Option<Focus>,
    width: f32,
    list_open: bool,
) -> (Element<'a, Message>, Option<Element<'a, Message>>) {
    let wide = width >= WIDE;
    // A labelled button's width: its label, its padding, its icon and the
    // gap between.
    let labelled = |label: &str| {
        (scramble_ui::font::measure(label, Type::LabelLarge) + 2.0 * 12.0 + 20.0 + 8.0).ceil()
    };
    // The widths of what the row holds, to tell whether what is brought
    // forward fits beside it.
    let mut used: Vec<f32> = Vec::new();
    let choice = |glyph: Icon, label: String, value: NetworkShows| {
        button::with_icon(Kind::Filled, glyph, label)
            .size(Size::ExtraSmall)
            .selected(shows == value)
            .on_press(Message::NetworkShows(value))
    };
    let labels = [
        if ptp { "PTP" } else { "gPTP" }.to_owned(),
        fl!("netmap-audio"),
        "CRF".to_owned(),
    ];
    // The group's buttons, and the gaps between them.
    used.push(labels.iter().map(|label| labelled(label)).sum::<f32>() + 2.0 * 2.0);
    let [clock, audio, crf] = labels;
    let choices = component::connected(vec![
        choice(Icon::Schedule, clock, NetworkShows::Clock),
        choice(Icon::GraphicEq, audio, NetworkShows::Audio),
        choice(Icon::Timer, crf, NetworkShows::MediaClock),
    ]);
    let focused = focus.map(|focus| {
        let what = match focus {
            Focus::Stream(..) => streams
                .iter()
                .find(|stream| stream.focus() == focus)
                .map(|stream| stream.name.clone())
                .unwrap_or_default(),
            Focus::Node(key) => {
                let name = find(topology, key)
                    .map(|node| node_name(topology, node))
                    .unwrap_or_default();
                match shows {
                    NetworkShows::Clock => fl!("netmap-focus-clock-path", name = name),
                    _ => fl!("netmap-focus-streams", name = name),
                }
            }
        };
        fl!("netmap-showing", what = what)
    });
    let mut header = line![].spacing(12).align_y(Center);
    if width >= TITLED {
        let title = fl!("toolbar-network");
        used.push(scramble_ui::font::measure(&title, Type::TitleLarge).ceil());
        header = header.push(styled(title, Type::TitleLarge));
    }
    header = header.push(choices);
    if width >= TITLED {
        let bridges = topology
            .nodes
            .iter()
            .filter(|node| node.kind == NodeKind::Bridge)
            .count();
        let mut entities: Vec<EntityId> = topology
            .nodes
            .iter()
            .filter_map(|node| match node.kind {
                NodeKind::Entity { entity_id, .. } => Some(entity_id),
                _ => None,
            })
            .collect();
        entities.sort_by_key(|entity_id| entity_id.0);
        entities.dedup();
        let devices = entities.len();
        // AVB Lite's switches take no part, so none are shown.
        let mut counts = vec![fl!("netmap-devices", count = devices)];
        if !topology.ptp {
            counts.push(fl!("netmap-bridges", count = bridges));
        }
        let counts = crate::i18n::list(counts);
        used.push(scramble_ui::font::measure(&counts, Type::BodyMedium).ceil());
        header = header.push(styled(counts, Type::BodyMedium).style(style::on_surface_variant));
    }
    header = header.push(space::horizontal());
    used.push(0.0);
    // The list's toggle, an icon button.
    if !wide {
        used.push(40.0);
    }
    let room = width - 2.0 * VIEW_MARGIN;
    let fits = |label: &String| {
        used.iter().sum::<f32>() + labelled(label) + 12.0 * used.len() as f32 <= room
    };
    let (beside, below) = match focused {
        Some(label) if fits(&label) => (Some(label), None),
        label => (None, label),
    };
    let focused = |label: String| {
        button::with_icon(Kind::Outlined, Icon::Close, label)
            .size(Size::ExtraSmall)
            .on_press(Message::NetworkFocused(None))
    };
    if let Some(label) = beside {
        header = header.push(focused(label));
    }
    if !wide {
        header = header.push(component::toggle_tool(
            Icon::ViewList,
            if list_open {
                fl!("netmap-show-map")
            } else {
                fl!("netmap-show-details")
            },
            list_open,
            Message::NetworkListToggled,
        ));
    }
    (header.into(), below.map(|label| focused(label).into()))
}

/// The entities the map shows: all but controllers, which take no part
/// in the clock or the streams.
fn mapped_entities(triib: &Triib) -> impl Iterator<Item = &DiscoveredEntity> {
    triib.entities.values().filter(|entity| {
        let adp = &entity.adp;
        !crate::describe::is_controller(adp)
            || crate::describe::is_talker(adp)
            || crate::describe::is_listener(adp)
    })
}

/// The tree the entities' reports make, with this computer when the
/// network runs and none of its entities is in it.
fn topology_of(triib: &Triib) -> Topology {
    let names: Vec<(EntityId, String)> = mapped_entities(triib)
        .map(|entity| (entity.entity_id(), triib.entity_name(entity)))
        .collect();
    let reports: Vec<EntityReport<'_>> = names
        .iter()
        .map(|(entity_id, name)| {
            let model = triib.models.get(entity_id);
            let interfaces = model
                .map(|model| {
                    model
                        .avb_interfaces()
                        .map(|interface| InterfaceReport {
                            index: interface.index,
                            clock: interface.clock_identity,
                            path: model.as_path(interface.index),
                            info: model.avb_info(interface.index),
                            wireless: model
                                .wireless(interface.index)
                                .map(|wireless| &wireless.status),
                        })
                        .collect()
                })
                .unwrap_or_default();
            EntityReport {
                entity_id: *entity_id,
                name,
                interfaces,
            }
        })
        .collect();
    // This computer, unless its own entities show where it is.
    let shown_by_entities = names.iter().any(|(entity_id, _)| {
        (triib.is_host(*entity_id) || triib.is_this_mac(*entity_id))
            && triib.models.get(entity_id).is_some_and(|model| {
                model.state != EnumerationState::Failed(EnumerationFailure::OnThisComputer)
            })
    });
    let host = (matches!(triib.network_state, NetworkState::Running { .. }) && !shown_by_entities)
        .then(|| match triib.neighbor {
            _ if triib.cannot_listen.is_some() => HostReport::CannotListen,
            Some(neighbor) => HostReport::Heard(neighbor.clock, neighbor.port, neighbor.synced),
            None => HostReport::Unheard,
        });
    let mut topology = Topology::build(&reports, host);
    // An entity this computer runs, which triib cannot read from here.
    for node in &mut topology.nodes {
        if let NodeKind::Entity { entity_id, .. } = node.kind
            && node.apart == Some(Apart::Unreported)
            && triib.models.get(&entity_id).is_some_and(|model| {
                model.state == EnumerationState::Failed(EnumerationFailure::OnThisComputer)
            })
        {
            node.apart = Some(Apart::OnThisComputer);
        }
    }
    topology
}

/// The clock as AVB Lite runs it, when it does: this computer hears no
/// AVB bridge with gPTP running to it, and an entity it reads, its own
/// endpoints or another's, operates in AVB Lite.
fn lite_topology(triib: &Triib) -> Option<Topology> {
    if triib.neighbor.is_some_and(|neighbor| neighbor.synced) {
        return None;
    }
    let active = |status: &&LiteStatus| status.flags.contains(LiteFlags::ACTIVE);
    let names: Vec<(EntityId, String)> = mapped_entities(triib)
        .map(|entity| (entity.entity_id(), triib.entity_name(entity)))
        .collect();
    let reports: Vec<LiteReport<'_>> = names
        .iter()
        .map(|(entity_id, name)| LiteReport {
            entity_id: *entity_id,
            name,
            interfaces: triib
                .models
                .get(entity_id)
                .map(|model| {
                    model
                        .avb_interfaces()
                        .map(|interface| {
                            (
                                interface.index,
                                interface.clock_identity,
                                model.lite_status(interface.index),
                            )
                        })
                        .collect()
                })
                .unwrap_or_default(),
        })
        .collect();
    let running = reports
        .iter()
        .flat_map(|report| &report.interfaces)
        .any(|(_, _, status)| status.as_ref().is_some_and(active));
    if !running {
        return None;
    }
    // This computer shows as its own entities, which follow whom it
    // follows, as they share its ptp4l.
    Some(Topology::build_lite(
        &reports,
        crate::lite_view::OFFSET_ALARM,
    ))
}

fn key_of(topology: &Topology, node: NodeId) -> NodeKey {
    match topology.nodes[node].kind {
        NodeKind::Entity { entity_id, .. } => NodeKey::Entity(entity_id),
        NodeKind::Host => NodeKey::Host,
        NodeKind::Bridge | NodeKind::Clock => {
            NodeKey::Clock(topology.nodes[node].clock.unwrap_or(ClockIdentity(0)))
        }
    }
}

fn find(topology: &Topology, key: NodeKey) -> Option<NodeId> {
    (0..topology.nodes.len()).find(|&node| key_of(topology, node) == key)
}

/// The last two octets of the entity's MAC address.
fn tag(entity: &DiscoveredEntity) -> String {
    let [.., fifth, sixth] = entity.mac.0;
    format!("{fifth:02x}:{sixth:02x}")
}

fn stream_name(model: Option<&EntityModel>, descriptor_type: DescriptorType, index: u16) -> String {
    model
        .and_then(|model| model.name_of(descriptor_type, index))
        .map_or_else(|| fl!("stream-numbered", index = index), str::to_owned)
}

/// A node's name on its card and in the details: a bridge's vendor, else
/// "Bridge".
fn node_name(topology: &Topology, node: NodeId) -> String {
    let entry = &topology.nodes[node];
    match entry.kind {
        NodeKind::Bridge => entry
            .clock
            .and_then(crate::topology::mac_of)
            .and_then(|mac| crate::vendor::name([mac[0], mac[1], mac[2]]))
            .unwrap_or_else(|| {
                if entry.access_point {
                    fl!("netmap-access-point")
                } else {
                    fl!("netmap-bridge")
                }
            }),
        NodeKind::Host => fl!("netmap-this-computer"),
        NodeKind::Entity { .. } => entry.name.clone(),
        NodeKind::Clock => entry
            .clock
            .map(|clock| clock.to_string())
            .unwrap_or_default(),
    }
}

/// Every bound stream input, by listener, with how far its stream gets.
fn streams(triib: &Triib, topology: &Topology) -> Vec<Stream> {
    let mut streams = Vec::new();
    for listener in triib.shown_entities() {
        let listener_id = listener.entity_id();
        let Some(model) = triib.models.get(&listener_id) else {
            continue;
        };
        for input in model.streams(true) {
            let Some((talker_id, output)) = model
                .binding(input.index)
                .and_then(|binding| binding.talker_stream())
            else {
                continue;
            };
            let info = model.stream_info(DescriptorType::STREAM_INPUT, input.index);
            let status = match info {
                Some(info) if info.talker_failed() => Status::Failed,
                Some(info) if info.settled() => Status::Flowing,
                _ => Status::Advertised,
            };
            let listener_name = triib.entity_name(listener);
            // A stream that is not flowing reaches its talker's bridge, or
            // a failed one the bridge that refused it.
            let talker_leg = match topology.placed(talker_id) {
                Some(talker) if topology.nodes[talker].parent.is_some() => Route {
                    hops: vec![(talker, true)],
                    stops: topology.nodes[talker].parent,
                },
                _ => Route::default(),
            };
            let (route, reason) = match status {
                Status::Flowing | Status::Unsynced => (
                    topology.route(talker_id, listener_id),
                    fl!("netmap-connected"),
                ),
                Status::Advertised => (
                    talker_leg,
                    if topology.placed(listener_id).is_none() {
                        fl!(
                            "netmap-advertised-off-tree",
                            listener = listener_name.clone()
                        )
                    } else {
                        fl!("netmap-advertised")
                    },
                ),
                Status::Failed => {
                    let code = info.map_or(0, |info| info.msrp_failure_code);
                    let [_, _, mac @ ..] = info
                        .map_or(0, |info| info.msrp_failure_bridge_id)
                        .to_be_bytes();
                    let full = topology.route(talker_id, listener_id);
                    match topology.bridge_with_mac(mac) {
                        Some(bridge) => (
                            if via(topology, &full).contains(&bridge) {
                                topology.cut(&full, bridge)
                            } else {
                                talker_leg
                            },
                            fl!(
                                "netmap-failed-at",
                                bridge = node_name(topology, bridge),
                                reason = describe::msrp_failure(code)
                            ),
                        ),
                        None => (
                            talker_leg,
                            fl!("netmap-failed", reason = describe::msrp_failure(code)),
                        ),
                    }
                }
            };
            let talker_model = triib.models.get(&talker_id);
            streams.push(Stream {
                input: (listener_id, input.index),
                talker: talker_id,
                name: {
                    let output = stream_name(talker_model, DescriptorType::STREAM_OUTPUT, output);
                    let input = stream_name(Some(model), DescriptorType::STREAM_INPUT, input.index);
                    format!("{output} {} {input}", crate::i18n::arrow(&output))
                },
                talker_name: triib.entity_name_of(talker_id),
                listener_name,
                media_clock: input.is_media_clock(),
                status,
                reason,
                route,
            });
        }
    }
    streams
}

/// A talker's hue by its place among the talkers: well apart for the first
/// few, then by the golden angle.
fn hue(place: usize) -> u16 {
    const HUES: [u16; 8] = [172, 205, 262, 318, 28, 108, 140, 235];
    HUES.get(place)
        .copied()
        .unwrap_or_else(|| ((172.0 + place as f32 * 137.5) % 360.0) as u16)
}

/// Each stream's color, from its talker and status: every talker has a
/// hue and its streams are shades of it; streams that are not flowing lose
/// their hue.
fn paints(streams: &[(EntityId, Status)]) -> Vec<Paint> {
    let mut talkers: Vec<EntityId> = Vec::new();
    for &(talker, _) in streams {
        if !talkers.contains(&talker) {
            talkers.push(talker);
        }
    }
    let mut seen: HashMap<EntityId, usize> = HashMap::new();
    streams
        .iter()
        .map(|&(talker, status)| {
            let count = streams.iter().filter(|other| other.0 == talker).count();
            let index = seen.entry(talker).or_default();
            let shade = if count == 1 {
                50
            } else {
                (*index * 100 / (count - 1)) as u8
            };
            *index += 1;
            match status {
                Status::Flowing | Status::Unsynced => Paint::Talker {
                    hue: hue(talkers
                        .iter()
                        .position(|&known| known == talker)
                        .unwrap_or(0)),
                    shade,
                },
                Status::Failed => Paint::Failed,
                Status::Advertised => Paint::Idle,
            }
        })
        .collect()
}

/// The bridges and other nodes a route enters, the upper end of each link
/// it crosses.
fn via(topology: &Topology, route: &Route) -> Vec<NodeId> {
    let mut nodes = Vec::new();
    for &(node, _) in &route.hops {
        if let Some(parent) = topology.nodes[node].parent
            && !nodes.contains(&parent)
        {
            nodes.push(parent);
        }
    }
    nodes
}

/// Whether `node` is `ancestor` or below it.
fn below(topology: &Topology, node: NodeId, ancestor: NodeId) -> bool {
    let mut at = Some(node);
    while let Some(current) = at {
        if current == ancestor {
            return true;
        }
        at = topology.nodes[current].parent;
    }
    false
}

fn entity_of(topology: &Topology, node: NodeId) -> Option<EntityId> {
    match topology.nodes[node].kind {
        NodeKind::Entity { entity_id, .. } => Some(entity_id),
        _ => None,
    }
}

/// Whether a stream sends from, goes to or passes through the node.
fn touches(topology: &Topology, stream: &Stream, node: NodeId) -> bool {
    let entity = entity_of(topology, node);
    entity.is_some() && (entity == Some(stream.talker) || entity == Some(stream.input.0))
        || via(topology, &stream.route).contains(&node)
}

/// Whether a stream is lit under `focus`.
fn lit(topology: &Topology, stream: &Stream, focus: Option<Focus>) -> bool {
    match focus {
        None => true,
        Some(Focus::Stream(..)) => focus == Some(stream.focus()),
        Some(Focus::Node(key)) => {
            find(topology, key).is_some_and(|node| touches(topology, stream, node))
        }
    }
}

fn lines(
    topology: &Topology,
    shows: NetworkShows,
    streams: &[Stream],
    paints: &[Paint],
    focus: Option<Focus>,
) -> Vec<Line> {
    if shows == NetworkShows::Clock {
        // The link up from every node on a tree, the clock flowing down it.
        let focused = match focus {
            Some(Focus::Node(key)) => find(topology, key),
            _ => None,
        };
        return (0..topology.nodes.len())
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
                    related: focused.is_none_or(|focused| {
                        below(topology, node, focused) || below(topology, focused, node)
                    }),
                    picked: false,
                    focus: Focus::Node(key_of(topology, node)),
                }
            })
            .collect();
    }
    streams
        .iter()
        .zip(paints)
        .map(|(stream, &paint)| Line {
            hops: stream.route.hops.clone(),
            status: stream.status,
            paint,
            related: lit(topology, stream, focus),
            picked: focus == Some(stream.focus()),
            focus: stream.focus(),
        })
        .collect()
}

fn cards(
    triib: &Triib,
    topology: &Topology,
    shows: NetworkShows,
    streams: &[Stream],
    paints: &[Paint],
    lines: &[Line],
    focus: Option<Focus>,
) -> Vec<Card> {
    let picked = streams
        .iter()
        .zip(paints)
        .find(|(stream, _)| focus == Some(stream.focus()));
    let interface = triib.settings.interface.as_deref().unwrap_or_default();
    // Whether a lit line enters or leaves the node.
    let on_lit_line = |node: NodeId, lit: &dyn Fn(&Line) -> bool| {
        lines.iter().any(|line| {
            lit(line)
                && line
                    .hops
                    .iter()
                    .any(|&(hop, _)| hop == node || topology.nodes[hop].parent == Some(node))
        })
    };
    (0..topology.nodes.len())
        .map(|node| {
            let entry = &topology.nodes[node];
            let key = key_of(topology, node);
            let entity = entity_of(topology, node);
            let (glyph, tag) = match entry.kind {
                NodeKind::Bridge if entry.access_point => (Icon::WifiTethering, String::new()),
                NodeKind::Bridge => (Icon::Hub, String::new()),
                NodeKind::Host => (Icon::Computer, String::new()),
                NodeKind::Clock => (Icon::Schedule, String::new()),
                NodeKind::Entity { entity_id, .. } => {
                    let found = triib.entities.get(&entity_id);
                    (
                        triib.host_icon(entity_id).unwrap_or_else(|| {
                            found.map_or(Icon::GraphicEq, |entity| {
                                crate::describe::glyph(&entity.adp)
                            })
                        }),
                        found.map(tag).unwrap_or_default(),
                    )
                }
            };
            let mut border = Paint::Plain;
            let (subtitle, subtitle_paint) = match (shows, entry.apart, entry.kind) {
                (_, Some(Apart::NoNeighbor), _) => (
                    fl!("netmap-no-bridge-on", interface = interface),
                    Paint::Muted,
                ),
                (_, Some(Apart::CannotListen), _) => (
                    fl!("netmap-cannot-listen-on", interface = interface),
                    Paint::Muted,
                ),
                (_, Some(Apart::OnThisComputer), _) => {
                    (fl!("netmap-on-this-computer"), Paint::Muted)
                }
                (_, Some(Apart::NotLite), _) => (fl!("netmap-not-lite"), Paint::Muted),
                (_, Some(Apart::LiteUnreported), _) => {
                    (fl!("netmap-lite-not-reported"), Paint::Muted)
                }
                (_, Some(apart), _) => (
                    match apart {
                        Apart::NoPath(_) => fl!("netmap-path-not-reported"),
                        Apart::Unreported => fl!("netmap-gptp-not-reported"),
                        _ => fl!("netmap-off-tree"),
                    },
                    Paint::Failed,
                ),
                (NetworkShows::Clock, None, _) if entry.parent.is_none() => {
                    border = Paint::Clock;
                    (fl!("avb-interface-grandmaster"), Paint::Clock)
                }
                (NetworkShows::Clock, None, NodeKind::Host) => (
                    if entry.link.synced {
                        fl!("netmap-synced")
                    } else {
                        fl!("netmap-not-synced")
                    },
                    Paint::Muted,
                ),
                (NetworkShows::Clock, None, _) if let Some(hop) = entry.link.wireless => (
                    wireless_time(hop),
                    if hop.locked {
                        Paint::Muted
                    } else {
                        Paint::Failed
                    },
                ),
                (NetworkShows::Clock, None, _) if entry.link.synced => {
                    (fl!("netmap-synced"), Paint::Muted)
                }
                (NetworkShows::Clock, None, _) => (fl!("netmap-not-synced"), Paint::Failed),
                (_, None, NodeKind::Host) => {
                    (fl!("netmap-triib-on", interface = interface), Paint::Muted)
                }
                (_, None, _) if entity.is_none() || !entry.children.is_empty() => {
                    let through = streams
                        .iter()
                        .filter(|stream| via(topology, &stream.route).contains(&node))
                        .count();
                    (fl!("netmap-through-count", count = through), Paint::Muted)
                }
                (_, None, _) => {
                    let sending = streams
                        .iter()
                        .filter(|stream| entity == Some(stream.talker))
                        .count();
                    let receiving = streams
                        .iter()
                        .filter(|stream| entity == Some(stream.input.0))
                        .count();
                    let failed = streams
                        .iter()
                        .filter(|stream| {
                            entity == Some(stream.input.0) && stream.status == Status::Failed
                        })
                        .count();
                    let mut counts = Vec::new();
                    if sending > 0 {
                        counts.push(fl!("netmap-out", count = sending));
                    }
                    if receiving > 0 {
                        counts.push(fl!("netmap-in", count = receiving));
                    }
                    if failed > 0 {
                        counts.push(fl!("netmap-failed-count", count = failed));
                    }
                    let text = if counts.is_empty() {
                        fl!("common-none")
                    } else {
                        crate::i18n::list(counts)
                    };
                    if failed > 0 {
                        border = Paint::Failed;
                        (text, Paint::Failed)
                    } else {
                        (text, Paint::Muted)
                    }
                }
            };
            let focused = focus == Some(Focus::Node(key));
            let in_picked = picked.is_some_and(|(stream, _)| {
                entity.is_some()
                    && (entity == Some(stream.talker) || entity == Some(stream.input.0))
            });
            let mut strong = false;
            if let Some((_, &paint)) = picked.filter(|_| in_picked) {
                border = paint;
                strong = true;
            }
            if focused {
                border = Paint::Text;
                strong = true;
            }
            // Under a focus, what takes no part in it fades.
            let faded = match focus {
                None => false,
                Some(_) if focused || in_picked => false,
                Some(Focus::Stream(..)) => !on_lit_line(node, &|line| line.picked),
                Some(Focus::Node(_)) if shows == NetworkShows::Clock => {
                    !on_lit_line(node, &|line| line.related)
                }
                Some(Focus::Node(_)) => !streams
                    .iter()
                    .any(|stream| lit(topology, stream, focus) && touches(topology, stream, node)),
            };
            Card {
                icon: glyph.codepoint(),
                name: node_name(topology, node),
                tag,
                subtitle,
                subtitle_paint,
                border,
                strong,
                dashed: entry.apart.is_some()
                    || (shows == NetworkShows::Clock
                        && entry.parent.is_some()
                        && !entry.link.synced),
                faded,
                focus: Focus::Node(key),
            }
        })
        .collect()
}

/// A stream or node listed in the details.
struct Item {
    name: String,
    detail: String,
    detail_paint: Paint,
    swatch: Paint,
    focus: Focus,
}

/// What the details show.
struct Page {
    kicker: String,
    title: String,
    swatch: Option<Paint>,
    state: String,
    state_icon: Icon,
    state_filled: bool,
    state_paint: Paint,
    facts: Vec<(String, String)>,
    sections: Vec<(String, Vec<Item>)>,
    help: String,
}

/// The panel beside the map: an overview, or the details of the stream or
/// node brought forward.
fn details(
    triib: &Triib,
    topology: &Topology,
    shows: NetworkShows,
    streams: &[Stream],
    paints: &[Paint],
    focus: Option<Focus>,
) -> Page {
    let focused = match focus {
        Some(Focus::Node(key)) => find(topology, key),
        _ => None,
    };
    match (shows, focus, focused) {
        (NetworkShows::Clock, _, Some(node)) => clock_details(triib, topology, node),
        (NetworkShows::Clock, ..) => clock_overview(topology),
        (_, Some(Focus::Stream(..)), _) => {
            let index = streams
                .iter()
                .position(|stream| Some(stream.focus()) == focus)
                .unwrap_or_default();
            stream_details(topology, shows, &streams[index], paints[index])
        }
        (_, _, Some(node)) => node_details(topology, streams, paints, node),
        _ => stream_overview(shows, streams, paints),
    }
}

fn stream_item(stream: &Stream, paint: Paint) -> Item {
    let (state, detail_paint) = match stream.status {
        Status::Flowing | Status::Unsynced => (fl!("netmap-connected"), Paint::Muted),
        Status::Advertised => (fl!("netmap-advertised-only"), Paint::Soft),
        Status::Failed => (fl!("netmap-failed-state"), Paint::Failed),
    };
    Item {
        name: stream.name.clone(),
        detail: fl!(
            "netmap-stream-item",
            talker = stream.talker_name.clone(),
            listener = stream.listener_name.clone(),
            state = state
        ),
        detail_paint,
        swatch: paint,
        focus: stream.focus(),
    }
}

/// How a station's time holds over its Wi-Fi hop.
fn wireless_time(hop: WirelessLink) -> String {
    if hop.locked {
        fl!("wireless-locked")
    } else if hop.holdover {
        fl!("wireless-holdover")
    } else {
        fl!("wireless-not-locked")
    }
}

/// What a station reports of its Wi-Fi hop, for its details.
fn wireless_facts(triib: &Triib, topology: &Topology, node: NodeId) -> Vec<(String, String)> {
    let NodeKind::Entity {
        entity_id,
        interface,
    } = topology.nodes[node].kind
    else {
        return Vec::new();
    };
    let Some(wireless) = triib
        .models
        .get(&entity_id)
        .and_then(|model| model.wireless(interface))
        .filter(|wireless| !wireless.status.access_point())
    else {
        return Vec::new();
    };
    let status = &wireless.status;
    let time = crate::wireless_view::Time::of(status).text();
    let mut link = vec![crate::wireless_view::link_text(status)];
    if let Some(rssi) = status.signal() {
        link.push(crate::wireless_view::signal_text(rssi));
    }
    vec![
        (
            fl!("wireless-mode"),
            crate::wireless_view::mode_text(status.time_mode),
        ),
        (fl!("wireless-time"), time),
        ("Wi-Fi".to_owned(), crate::i18n::list(link)),
    ]
}

/// Whether the node is on a tree with gPTP running up to it.
fn synced(topology: &Topology, node: NodeId) -> bool {
    let entry = &topology.nodes[node];
    entry.apart.is_none() && (entry.parent.is_none() || entry.link.synced)
}

fn clock_item(topology: &Topology, node: NodeId) -> Item {
    let ok = synced(topology, node);
    // In AVB Lite, a device taking no part says why.
    let apart = match topology.nodes[node].apart {
        Some(Apart::NotLite) => Some(fl!("netmap-not-lite")),
        Some(Apart::LiteUnreported) => Some(fl!("netmap-lite-not-reported")),
        _ => None,
    };
    Item {
        name: node_name(topology, node),
        detail: match &apart {
            Some(apart) => apart.clone(),
            None if ok => fl!("netmap-synced"),
            None => fl!("netmap-not-synced"),
        },
        detail_paint: if ok || apart.is_some() {
            Paint::Muted
        } else {
            Paint::Failed
        },
        swatch: if ok { Paint::Clock } else { Paint::Unsynced },
        focus: Focus::Node(key_of(topology, node)),
    }
}

/// A device's clock in AVB Lite: whom it follows and how closely, as it
/// reports it, or why it takes no part.
fn lite_details(
    triib: &Triib,
    topology: &Topology,
    node: NodeId,
    kicker: String,
    title: String,
) -> Page {
    let entry = &topology.nodes[node];
    let status = match entry.kind {
        NodeKind::Entity {
            entity_id,
            interface,
        } => triib
            .models
            .get(&entity_id)
            .and_then(|model| model.lite_status(interface))
            .copied(),
        _ => None,
    };
    let mut facts = Vec::new();
    if let Some(parent) = entry.parent {
        facts.push((
            fl!("avb-interface-grandmaster"),
            node_name(topology, parent),
        ));
    }
    if let Some(offset) = entry.link.offset {
        facts.push((fl!("lite-offset"), crate::lite_view::duration(offset)));
    }
    if let Some(status) = status {
        facts.push((
            "PTP".to_owned(),
            fl!(
                "lite-ptp-domain",
                profile = crate::lite_view::profile_name(status.ptp_profile),
                domain = status.ptp_domain
            ),
        ));
        if status.flags.contains(LiteFlags::ACTIVE) {
            facts.push((
                fl!("lite-because"),
                crate::lite_view::fallback(status.fallback_reason),
            ));
        }
        if status.link_speed > 0 {
            facts.push((
                fl!("lite-link"),
                crate::lite_view::rate(u64::from(status.link_speed) * 1_000_000),
            ));
        }
    }
    if let Some(clock) = entry.clock {
        facts.push((fl!("avb-interface-clock-identity"), clock.to_string()));
    }
    let ok = synced(topology, node);
    let state = match (entry.apart, entry.link.offset) {
        (Some(apart), _) => apart_reason(apart),
        (None, _) if ok => fl!("netmap-synced-to-grandmaster"),
        (None, Some(offset)) => fl!(
            "netmap-ptp-offset-high",
            offset = crate::lite_view::duration(offset)
        ),
        (None, None) => fl!("netmap-ptp-no-offset"),
    };
    Page {
        kicker,
        title,
        swatch: Some(if ok { Paint::Clock } else { Paint::Unsynced }),
        state,
        state_icon: if ok { Icon::CheckCircle } else { Icon::Error },
        state_filled: true,
        state_paint: if ok || entry.apart.is_some() {
            Paint::Soft
        } else {
            Paint::Failed
        },
        facts,
        sections: Vec::new(),
        help: fl!("netmap-help-back"),
    }
}

/// Why a node is not on a tree, in words.
fn apart_reason(apart: Apart) -> String {
    match apart {
        Apart::OwnGrandmaster => fl!("netmap-apart-own-grandmaster"),
        Apart::NoPath(grandmaster) => fl!(
            "netmap-apart-no-path",
            grandmaster = grandmaster.to_string()
        ),
        Apart::Unreported => fl!("netmap-apart-unreported"),
        Apart::NoNeighbor => fl!("netmap-apart-no-neighbor"),
        Apart::CannotListen => fl!("netmap-apart-cannot-listen"),
        Apart::OnThisComputer => fl!("netmap-apart-on-this-computer"),
        Apart::NotLite => fl!("netmap-apart-not-lite"),
        Apart::LiteUnreported => fl!("netmap-apart-lite-unreported"),
    }
}

fn clock_overview(topology: &Topology) -> Page {
    let unsynced: Vec<Item> = (0..topology.nodes.len())
        .filter(|&node| !synced(topology, node))
        .map(|node| clock_item(topology, node))
        .collect();
    let grandmasters: Vec<String> = topology
        .roots
        .iter()
        .map(|&root| node_name(topology, root))
        .collect();
    Page {
        kicker: if topology.ptp { "PTP" } else { "gPTP" }.to_owned(),
        title: fl!("netmap-clock-tree"),
        swatch: Some(Paint::Clock),
        state: if grandmasters.is_empty() {
            fl!("netmap-no-grandmaster")
        } else {
            fl!(
                "netmap-grandmaster-is",
                grandmaster = crate::i18n::list(grandmasters)
            )
        },
        state_icon: Icon::Schedule,
        state_filled: false,
        state_paint: Paint::Soft,
        facts: vec![
            (
                fl!("netmap-synced"),
                (topology.nodes.len() - unsynced.len()).to_string(),
            ),
            (fl!("netmap-not-synced"), unsynced.len().to_string()),
        ],
        sections: if unsynced.is_empty() {
            Vec::new()
        } else {
            vec![(fl!("netmap-needs-attention"), unsynced)]
        },
        help: if topology.ptp {
            fl!("netmap-help-ptp")
        } else {
            fl!("netmap-help-clock")
        },
    }
}

fn clock_details(triib: &Triib, topology: &Topology, node: NodeId) -> Page {
    let entry = &topology.nodes[node];
    let kicker = match entry.kind {
        NodeKind::Bridge => fl!("netmap-bridge"),
        NodeKind::Clock => "PTP".to_owned(),
        _ => fl!("netmap-device"),
    };
    let title = node_name(topology, node);
    let below_here: Vec<NodeId> = (0..topology.nodes.len())
        .filter(|&other| other != node && below(topology, other, node))
        .collect();
    if entry.apart.is_none() && entry.parent.is_none() {
        let unsynced = below_here
            .iter()
            .filter(|&&other| !synced(topology, other))
            .count();
        let bridges: Vec<Item> = below_here
            .iter()
            .filter(|&&other| !topology.nodes[other].children.is_empty())
            .map(|&other| clock_item(topology, other))
            .collect();
        let (heading, items) = if bridges.is_empty() {
            (
                fl!("netmap-nodes-below"),
                entry
                    .children
                    .iter()
                    .map(|&child| clock_item(topology, child))
                    .collect(),
            )
        } else {
            (fl!("netmap-bridges-below"), bridges)
        };
        return Page {
            kicker,
            title,
            swatch: Some(Paint::Clock),
            state: fl!("avb-interface-grandmaster"),
            state_icon: Icon::Schedule,
            state_filled: false,
            state_paint: Paint::Clock,
            facts: vec![
                (
                    fl!("netmap-synced"),
                    (below_here.len() - unsynced).to_string(),
                ),
                (fl!("netmap-not-synced"), unsynced.to_string()),
            ],
            sections: vec![(heading, items)],
            help: fl!("netmap-help-back"),
        };
    }
    if topology.ptp {
        return lite_details(triib, topology, node, kicker, title);
    }
    let mut facts = Vec::new();
    if entry.apart.is_none() {
        let mut path = vec![node_name(topology, node)];
        let mut at = entry.parent;
        while let Some(parent) = at {
            path.push(node_name(topology, parent));
            at = topology.nodes[parent].parent;
        }
        path.reverse();
        let arrow = format!(" {} ", crate::i18n::arrow(&path[0]));
        facts.push((fl!("netmap-clock-path"), path.join(&arrow)));
        facts.push((fl!("netmap-hops"), (path.len() - 1).to_string()));
    }
    facts.extend(wireless_facts(triib, topology, node));
    if let Some(delay) = entry.link.delay {
        facts.push((fl!("netmap-link-delay"), format!("{delay} ns")));
    }
    if let Some(port) = entry.link.port {
        facts.push((fl!("netmap-bridge-port"), port.to_string()));
    }
    if let NodeKind::Entity {
        entity_id,
        interface,
    } = entry.kind
        && let Some(counters) = triib
            .models
            .get(&entity_id)
            .and_then(|model| model.counters(DescriptorType::AVB_INTERFACE, interface))
            .and_then(|counters| counters.avb_interface())
    {
        if let Some(downs) = counters.link_down {
            facts.push((fl!("netmap-link-drops"), downs.to_string()));
        }
        if let Some(changes) = counters.gptp_gm_changed {
            facts.push((
                fl!("avb-interface-grandmaster-changes"),
                changes.to_string(),
            ));
        }
    }
    if let Some(clock) = entry.clock {
        facts.push((fl!("avb-interface-clock-identity"), clock.to_string()));
    }
    let ok = synced(topology, node);
    let state = match entry.apart {
        Some(apart) => apart_reason(apart),
        None if ok => fl!("netmap-synced-to-grandmaster"),
        None if entry.kind == NodeKind::Host => fl!("netmap-host-no-gptp"),
        None if entry.link.wireless.is_some() => fl!("wireless-alarm-not-locked"),
        None => fl!("netmap-link-no-gptp"),
    };
    Page {
        kicker,
        title,
        swatch: Some(if ok { Paint::Clock } else { Paint::Unsynced }),
        state,
        state_icon: if ok { Icon::CheckCircle } else { Icon::Error },
        state_filled: true,
        state_paint: if ok || entry.kind == NodeKind::Host {
            Paint::Soft
        } else {
            Paint::Failed
        },
        facts,
        sections: if below_here.is_empty() {
            Vec::new()
        } else {
            vec![(
                fl!("netmap-nodes-below"),
                below_here
                    .iter()
                    .map(|&other| clock_item(topology, other))
                    .collect(),
            )]
        },
        help: fl!("netmap-help-back"),
    }
}

fn stream_overview(shows: NetworkShows, streams: &[Stream], paints: &[Paint]) -> Page {
    let count = |status: Status| {
        streams
            .iter()
            .filter(|stream| stream.status == status)
            .count()
    };
    let attention: Vec<Item> = streams
        .iter()
        .zip(paints)
        .filter(|(stream, _)| stream.status != Status::Flowing)
        .map(|(stream, &paint)| stream_item(stream, paint))
        .collect();
    let media_clock = shows == NetworkShows::MediaClock;
    Page {
        kicker: if media_clock {
            "CRF".to_owned()
        } else {
            fl!("netmap-audio")
        },
        title: if media_clock {
            fl!("netmap-media-clock-streams")
        } else {
            fl!("netmap-audio-streams")
        },
        swatch: None,
        state: fl!("netmap-bound", count = streams.len()),
        state_icon: if media_clock {
            Icon::Timer
        } else {
            Icon::GraphicEq
        },
        state_filled: false,
        state_paint: Paint::Soft,
        facts: vec![
            (fl!("netmap-flowing"), count(Status::Flowing).to_string()),
            (
                fl!("netmap-advertised-state"),
                count(Status::Advertised).to_string(),
            ),
            (
                fl!("netmap-failed-state"),
                count(Status::Failed).to_string(),
            ),
        ],
        sections: if attention.is_empty() {
            Vec::new()
        } else {
            vec![(fl!("netmap-needs-attention"), attention)]
        },
        help: if media_clock {
            fl!("netmap-help-media-clock")
        } else {
            fl!("netmap-help-audio")
        },
    }
}

fn stream_details(topology: &Topology, shows: NetworkShows, stream: &Stream, paint: Paint) -> Page {
    // The talker, then the far end of each link the stream crosses.
    let mut names = vec![stream.talker_name.clone()];
    for &(node, up) in &stream.route.hops {
        let next = if up {
            topology.nodes[node].parent
        } else {
            Some(node)
        };
        if let Some(next) = next {
            names.push(node_name(topology, next));
        }
    }
    let (state_icon, state_filled, state_paint) = match stream.status {
        Status::Flowing | Status::Unsynced => (Icon::Link, true, Paint::Soft),
        Status::Advertised => (Icon::Pending, false, Paint::Soft),
        Status::Failed => (Icon::Error, true, Paint::Failed),
    };
    Page {
        kicker: if shows == NetworkShows::MediaClock {
            fl!("netmap-media-clock-stream")
        } else {
            fl!("netmap-audio-stream")
        },
        title: stream.name.clone(),
        swatch: Some(paint),
        state: stream.reason.clone(),
        state_icon,
        state_filled,
        state_paint,
        facts: vec![
            (fl!("advert-talker"), stream.talker_name.clone()),
            (fl!("advert-listener"), stream.listener_name.clone()),
            (
                if stream.status == Status::Flowing {
                    fl!("avb-interface-path")
                } else {
                    fl!("netmap-reaches")
                },
                names.join(&format!(" {} ", crate::i18n::arrow(&names[0]))),
            ),
        ],
        sections: Vec::new(),
        help: fl!("netmap-help-back"),
    }
}

fn node_details(topology: &Topology, streams: &[Stream], paints: &[Paint], node: NodeId) -> Page {
    let entry = &topology.nodes[node];
    let entity = entity_of(topology, node);
    let items = |test: &dyn Fn(&Stream) -> bool| -> Vec<Item> {
        streams
            .iter()
            .zip(paints)
            .filter(|(stream, _)| test(stream))
            .map(|(stream, &paint)| stream_item(stream, paint))
            .collect()
    };
    let bridge = entity.is_none() || !entry.children.is_empty();
    let glyph = match entry.kind {
        NodeKind::Bridge => Icon::Hub,
        NodeKind::Host => Icon::Computer,
        NodeKind::Entity { .. } => Icon::GraphicEq,
        NodeKind::Clock => Icon::Schedule,
    };
    if bridge && entry.kind != NodeKind::Host {
        let through = items(&|stream| via(topology, &stream.route).contains(&node));
        return Page {
            kicker: fl!("netmap-bridge"),
            title: node_name(topology, node),
            swatch: None,
            state: fl!("netmap-passing-count", count = through.len()),
            state_icon: glyph,
            state_filled: false,
            state_paint: Paint::Soft,
            facts: vec![(fl!("netmap-through"), through.len().to_string())],
            sections: if through.is_empty() {
                Vec::new()
            } else {
                vec![(fl!("netmap-passing-through"), through)]
            },
            help: fl!("netmap-help-stream"),
        };
    }
    let sending = items(&|stream| entity.is_some() && entity == Some(stream.talker));
    let receiving = items(&|stream| entity.is_some() && entity == Some(stream.input.0));
    let problems = streams
        .iter()
        .filter(|stream| {
            entity.is_some()
                && (entity == Some(stream.talker) || entity == Some(stream.input.0))
                && stream.status != Status::Flowing
        })
        .count();
    let mut counts = Vec::new();
    if !sending.is_empty() {
        counts.push(fl!("netmap-out", count = sending.len()));
    }
    if !receiving.is_empty() {
        counts.push(fl!("netmap-in", count = receiving.len()));
    }
    let (state, state_icon, state_paint) = match entry.apart {
        Some(apart) => (apart_reason(apart), Icon::Error, Paint::Failed),
        None if counts.is_empty() => (fl!("common-none"), glyph, Paint::Soft),
        None => (crate::i18n::list(counts), glyph, Paint::Soft),
    };
    Page {
        kicker: fl!("netmap-device"),
        title: node_name(topology, node),
        swatch: None,
        state,
        state_icon,
        state_filled: false,
        state_paint,
        facts: vec![
            (fl!("netmap-sending"), sending.len().to_string()),
            (fl!("netmap-receiving"), receiving.len().to_string()),
            (fl!("netmap-problems"), problems.to_string()),
        ],
        sections: [
            (fl!("netmap-sending"), sending),
            (fl!("netmap-receiving"), receiving),
        ]
        .into_iter()
        .filter(|(_, items)| !items.is_empty())
        .collect(),
        help: fl!("netmap-help-stream"),
    }
}

impl Page {
    fn view<'a>(self, width: Length) -> Element<'a, Message> {
        let swatch = |paint: Option<Paint>| {
            container(space())
                .width(5)
                .height(Fill)
                .style(move |theme: &Theme| container::Style {
                    background: paint
                        .map(|paint| map::paint_color(&Scheme::of(theme), paint).into()),
                    border: iced::border::rounded(3),
                    ..container::Style::default()
                })
        };
        let state_paint = self.state_paint;
        let state_icon = if self.state_filled {
            icon::filled(self.state_icon, 16)
        } else {
            icon::icon(self.state_icon, 16)
        };
        let tinted = move |theme: &Theme| iced::widget::text::Style {
            color: Some(map::text_color(&Scheme::of(theme), state_paint)),
        };
        let mut body = column![
            styled(crate::i18n::uppercase(&self.kicker), Type::LabelSmall)
                .style(style::on_surface_variant),
            row![
                swatch(self.swatch),
                column![
                    styled(self.title, Type::TitleMedium).align_x(dir::text_start()),
                    row![
                        container(state_icon.style(tinted)).padding(iced::Padding {
                            top: 2.0,
                            ..iced::Padding::ZERO
                        }),
                        styled(self.state, Type::BodyMedium)
                            .style(tinted)
                            .align_x(dir::text_start()),
                    ]
                    .spacing(4),
                ]
                .spacing(2)
                .width(Fill),
            ]
            .spacing(10)
            .height(Length::Shrink),
        ]
        .spacing(14);
        if !self.facts.is_empty() {
            body = body.push(
                dir::column(self.facts.into_iter().map(|(label, value)| {
                    row![
                        styled(label, Type::BodyMedium)
                            .style(style::on_surface_variant)
                            .width(Length::Fixed(112.0))
                            .align_x(dir::text_start()),
                        styled(value, Type::BodyMedium)
                            .width(Fill)
                            .align_x(dir::text_start()),
                    ]
                    .spacing(12)
                    .into()
                }))
                .spacing(8),
            );
        }
        for (title, items) in self.sections {
            let items = items.into_iter().map(|item| {
                let detail_paint = item.detail_paint;
                let content = row![
                    swatch(Some(item.swatch)),
                    column![
                        styled(item.name, Type::LabelLarge).align_x(dir::text_start()),
                        styled(item.detail, Type::BodySmall)
                            .align_x(dir::text_start())
                            .style(move |theme: &Theme| iced::widget::text::Style {
                                color: Some(map::text_color(&Scheme::of(theme), detail_paint)),
                            }),
                    ]
                    .spacing(2)
                    .width(Fill),
                ]
                .spacing(10)
                .height(Length::Shrink);
                plain_button(content)
                    .padding([8, 10])
                    .width(Fill)
                    .style(|theme: &Theme, status| {
                        let scheme = Scheme::of(theme);
                        plain_button::Style {
                            background: Some(
                                match status {
                                    plain_button::Status::Hovered
                                    | plain_button::Status::Pressed => {
                                        scheme.surface_container_high
                                    }
                                    _ => scheme.surface_container,
                                }
                                .into(),
                            ),
                            text_color: scheme.on_surface,
                            border: iced::Border {
                                color: scheme.outline_variant,
                                width: 1.0,
                                radius: 10.0.into(),
                            },
                            ..plain_button::Style::default()
                        }
                    })
                    .on_press(Message::NetworkFocused(Some(item.focus)))
                    .into()
            });
            body = body.push(
                column![styled(title, Type::LabelLarge).style(style::on_surface_variant)]
                    .push(dir::column(items).spacing(6))
                    .spacing(6),
            );
        }
        body = body.push(
            styled(self.help, Type::BodySmall)
                .style(style::on_surface_variant)
                .align_x(dir::text_start()),
        );
        container(
            scrollable(body.padding(16))
                .direction(scrollable::Direction::Vertical(component::thin_scrollbar()))
                .style(style::scrollbar)
                .height(Fill),
        )
        .width(width)
        .height(Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(Scheme::of(theme).surface_container_low.into()),
            border: iced::border::rounded(shape::LARGE),
            ..container::Style::default()
        })
        .into()
    }
}

#[cfg(test)]
mod tests {
    use iced::Size;

    use super::*;

    /// A switch with three entities under it, and four streams between
    /// them in each state.
    fn bench() -> (Topology, Vec<Stream>) {
        let switch = ClockIdentity(0x0001_f2ff_feff_3b14);
        let clocks = [1u64, 2, 3].map(|place| ClockIdentity(0x0011_22ff_fe00_0000 | place));
        let names = ["Stage box", "Console", "Amp"];
        let paths: Vec<[ClockIdentity; 2]> = clocks.iter().map(|&clock| [switch, clock]).collect();
        let reports: Vec<EntityReport<'_>> = (0..3)
            .map(|place| EntityReport {
                entity_id: EntityId(place as u64 + 1),
                name: names[place],
                interfaces: vec![InterfaceReport {
                    index: 0,
                    clock: clocks[place],
                    path: Some(&paths[place]),
                    info: None,
                    wireless: None,
                }],
            })
            .collect();
        let topology = Topology::build(&reports, None);
        let stream = |talker: u64, listener: u64, input: u16, name: &str, status: Status| {
            let route = match status {
                Status::Flowing => topology.route(EntityId(talker), EntityId(listener)),
                _ => {
                    let node = topology.placed(EntityId(talker)).unwrap();
                    Route {
                        hops: vec![(node, true)],
                        stops: topology.nodes[node].parent,
                    }
                }
            };
            Stream {
                input: (EntityId(listener), input),
                talker: EntityId(talker),
                name: name.to_owned(),
                talker_name: names[talker as usize - 1].to_owned(),
                listener_name: names[listener as usize - 1].to_owned(),
                media_clock: false,
                status,
                reason: match status {
                    Status::Failed => "Reservation failed at Bridge: insufficient bandwidth",
                    Status::Advertised => "Advertised, no listener ready",
                    _ => "Connected",
                }
                .to_owned(),
                route,
            }
        };
        let streams = vec![
            stream(1, 2, 0, "Stage 1–8 → Console in 1–8", Status::Flowing),
            stream(1, 2, 1, "Stage 9–16 → Console in 9–16", Status::Flowing),
            stream(2, 3, 0, "Main L/R → Amp in", Status::Failed),
            stream(2, 3, 1, "Fill → Amp in 2", Status::Advertised),
        ];
        (topology, streams)
    }

    /// Writes pictures of the header and the details panel to the PNG files
    /// the variable `NETWORK_PANEL_PICTURE` names, with what each shows added.
    #[test]
    #[ignore = "writes pictures to look at"]
    fn pictures() {
        let Ok(path) = std::env::var("NETWORK_PANEL_PICTURE") else {
            return;
        };
        let theme = scramble_ui::scheme::theme("triib".to_owned(), crate::app::TRIIB_SEED, true);
        let (topology, streams) = bench();
        let paints = paints(
            &streams
                .iter()
                .map(|stream| (stream.talker, stream.status))
                .collect::<Vec<_>>(),
        );
        let console = find(&topology, NodeKey::Entity(EntityId(2))).unwrap();
        // How to build each page, by the name its picture takes.
        type Build<'a> = Box<dyn Fn() -> Page + 'a>;
        let pages: [(&str, Build<'_>); 4] = [
            (
                "overview",
                Box::new(|| stream_overview(NetworkShows::Audio, &streams, &paints)),
            ),
            (
                "stream",
                Box::new(|| stream_details(&topology, NetworkShows::Audio, &streams[2], paints[2])),
            ),
            (
                "device",
                Box::new(|| node_details(&topology, &streams, &paints, console)),
            ),
            ("clock", Box::new(|| clock_overview(&topology))),
        ];
        for (suffix, page) in pages {
            let file = format!("{path}-{suffix}.png");
            for renderer in ["tiny-skia", "wgpu"] {
                let _ = std::fs::remove_file(format!("{path}-{suffix}-{renderer}.png"));
            }
            let settings = iced::Settings {
                fonts: scramble_ui::font::files().collect(),
                default_font: scramble_ui::font::TEXT,
                antialiasing: true,
                ..iced::Settings::default()
            };
            let focus = Some(streams[2].focus());
            let content: Element<'_, Message> = iced::widget::column![
                header(
                    &topology,
                    false,
                    NetworkShows::Audio,
                    &streams,
                    focus,
                    1280.0,
                    false
                )
                .0,
                page().view(Length::Fixed(PANEL_WIDTH)),
            ]
            .spacing(16)
            .padding(16)
            .into();
            let mut simulator =
                iced_test::Simulator::with_size(settings, Size::new(1280.0, 640.0), content);
            let snapshot = simulator.snapshot(&theme).expect("draws");
            assert!(snapshot.matches_image(&file).expect("writes"));
        }
    }

    /// Controllers take no place in the map, and this computer shows as
    /// its own entities once one of them is there.
    #[test]
    fn the_map_leaves_out_controllers_and_a_redundant_host() {
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
        let mut triib = Triib::sample(
            crate::settings::Settings::default(),
            interface,
            entities,
            models,
        );
        triib.network_state = NetworkState::Running {
            controller: EntityId(0x9c6b_00ff_fe30_9a2b),
        };
        let controller = EntityId(1);
        let mut entity = *triib.entities.values().next().unwrap();
        entity.adp.entity_id = controller;
        entity.adp.talker_capabilities = atdecc::adp::TalkerCapabilities::empty();
        entity.adp.listener_capabilities = atdecc::adp::ListenerCapabilities::empty();
        entity.adp.controller_capabilities = atdecc::adp::ControllerCapabilities::IMPLEMENTED;
        triib.entities.insert(controller, entity);
        let kinds = |triib: &Triib| -> Vec<NodeKind> {
            topology_of(triib)
                .nodes
                .iter()
                .map(|node| node.kind)
                .collect()
        };
        assert!(!kinds(&triib).iter().any(|kind| matches!(
            kind,
            NodeKind::Entity { entity_id, .. } if *entity_id == controller
        )));
        assert!(kinds(&triib).contains(&NodeKind::Host));
        triib.endpoints = Some(DaemonStatus {
            pid: 1,
            started: 0,
            interface: "enp6s0".into(),
            gptp: String::new(),
            foreign_mrp: false,
            config: None,
            endpoints: vec![EndpointStatus {
                entity_id: crate::view::tests::WIRED_ESP.to_string(),
                name: "Host entity 1".into(),
                state: "running".into(),
                streams: Vec::new(),
            }],
        });
        assert!(!kinds(&triib).contains(&NodeKind::Host));
    }

    /// The bench on a plain switch, the wired ESP following a grandmaster
    /// in AVB Lite `offset` nanoseconds off, the clock view showing.
    fn lite_triib(offset: i32) -> Triib {
        let (entities, models) =
            crate::view::tests::bench_then(&crate::view::tests::lite_frames(offset, 6_336));
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
            network_shows: NetworkShows::Clock,
            ..crate::settings::Settings::default()
        };
        Triib::sample(settings, interface, entities, models)
    }

    /// With no AVB bridge heard and an entity in AVB Lite, the clock is
    /// PTP's: the ESP hangs under its grandmaster, in sync within 50 us
    /// and out of it past that.
    /// What is brought forward goes beside the header's chips where its row
    /// has room, and below the map where it has not, so the map stays where
    /// it is as it comes and goes.
    #[test]
    fn what_is_brought_forward_goes_below_the_map_without_room() {
        let (topology, streams) = bench();
        let below = |focus: Option<Focus>, width: f32| {
            let shows = NetworkShows::Audio;
            let (_, below) = header(&topology, false, shows, &streams, focus, width, false);
            below.is_some()
        };
        let focus = Some(streams[2].focus());
        assert!(below(focus, 360.0));
        assert!(!below(focus, 804.0));
        assert!(!below(focus, 1280.0));
        assert!(!below(None, 360.0));
    }

    #[test]
    fn avb_lite_shows_the_ptp_clock() {
        let triib = lite_triib(-180);
        let topology = lite_topology(&triib).expect("AVB Lite runs");
        assert!(topology.ptp);
        let esp = find(&topology, NodeKey::Entity(EntityId(0xe8f6_0ae0_9220_0000))).unwrap();
        assert!(synced(&topology, esp));
        assert!(topology.nodes[topology.nodes[esp].parent.unwrap()].kind == NodeKind::Clock);
        let triib = lite_triib(72_000);
        let topology = lite_topology(&triib).unwrap();
        let esp = find(&topology, NodeKey::Entity(EntityId(0xe8f6_0ae0_9220_0000))).unwrap();
        assert!(!synced(&topology, esp));
    }

    /// Writes a picture of the network view in AVB Lite to the PNG file the
    /// variable `NETWORK_LITE_PICTURE` names.
    #[test]
    #[ignore = "writes a picture to look at"]
    fn lite_picture() {
        let Ok(path) = std::env::var("NETWORK_LITE_PICTURE") else {
            return;
        };
        let theme = scramble_ui::scheme::theme("triib".to_owned(), crate::app::TRIIB_SEED, true);
        let mut triib = lite_triib(-180);
        let esp = EntityId(0xe8f6_0ae0_9220_0000);
        for (suffix, focus) in [
            ("overview", None),
            ("device", Some(Focus::Node(NodeKey::Entity(esp)))),
        ] {
            triib.network_focus = focus;
            let settings = iced::Settings {
                fonts: scramble_ui::font::files().collect(),
                default_font: scramble_ui::font::TEXT,
                antialiasing: true,
                ..iced::Settings::default()
            };
            let content = view_at(&triib, 1280.0);
            let mut simulator =
                iced_test::Simulator::with_size(settings, Size::new(1280.0, 720.0), content);
            let snapshot = simulator.snapshot(&theme).expect("draws");
            let file = format!("{path}-{suffix}.png");
            for renderer in ["tiny-skia", "wgpu"] {
                let _ = std::fs::remove_file(format!("{path}-{suffix}-{renderer}.png"));
            }
            assert!(snapshot.matches_image(&file).expect("writes"));
        }
    }
}
