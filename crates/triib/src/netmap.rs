//! The network view: the gPTP tree with each bridge's devices in a column
//! under it, showing the clock, the audio streams or the media clock
//! streams, each stream on a wire colored by its talker that branches
//! where its listeners' paths part, and beside it the details of what is
//! brought forward. Devices that fell back to AVB Lite hang under a card
//! for the switches that take no part in AVB in front of them; devices set
//! apart by design, on another gPTP domain or running AVB Lite only, sit
//! apart from those set apart by a fault.

mod map;

use atdecc::aem::{StreamInfo, StreamInfoFlags};
use atdecc::descriptor::{DescriptorType, InterfaceFlags};
use atdecc::lite::{LiteFlags, LiteStatus, PtpProfile};
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
use crate::network::Plugged;
use crate::settings::NetworkShows;
use crate::topology::{
    Apart, EntityReport, HostClock, HostReport, InterfaceReport, Kind as NodeKind, NodeId, Route,
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

/// What the view brings forward: a stream, or a node with its streams or
/// its clock path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Focus {
    /// A stream to one listener, by the listener's stream input.
    Stream(EntityId, u16),
    /// A talker's stream to all its listeners, by the talker's stream
    /// output.
    Output(EntityId, u16),
    Node(NodeKey),
}

/// A node, by what stays the same while the topology is built again.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeKey {
    Entity(EntityId),
    Clock(ClockIdentity),
    Host,
    /// The switches that take no part in AVB, by the PTP grandmaster and
    /// domain their devices follow.
    Switch(ClockIdentity, u8),
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
    /// The talker's stream output it carries to a multicast address, whose
    /// lines to other listeners share its wires as far as their paths do,
    /// as the stream's frames do; none for a unicast stream, which the
    /// talker sends each listener a copy of, on a wire of its own.
    pub stream: Option<(EntityId, u16)>,
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

/// A stream to a listener bound to it, or one its talker sends with no
/// listener triib knows of bound to it.
#[derive(Debug, Clone)]
struct Stream {
    /// None for a stream with no listener known.
    input: Option<(EntityId, u16)>,
    talker: EntityId,
    output: u16,
    name: String,
    /// The talker's stream output's name.
    output_name: String,
    talker_name: String,
    listener_name: String,
    media_clock: bool,
    /// Sent to a multicast address, not to the listener's own.
    multicast: bool,
    status: Status,
    reason: String,
    route: Route,
}

impl Stream {
    fn focus(&self) -> Focus {
        match self.input {
            Some((listener, input)) => Focus::Stream(listener, input),
            None => Focus::Output(self.talker, self.output),
        }
    }

    fn listener(&self) -> Option<EntityId> {
        self.input.map(|(listener, _)| listener)
    }

    fn key(&self) -> (EntityId, u16) {
        (self.talker, self.output)
    }

    /// What tells its frames apart: a multicast stream's are the same for
    /// every listener, a unicast one's each listener's own.
    fn frames(&self) -> (EntityId, u16, Option<(EntityId, u16)>) {
        (
            self.talker,
            self.output,
            if self.multicast { None } else { self.input },
        )
    }
}

/// How many streams of frames `streams` are: a multicast stream once
/// however many listeners it has, a unicast one once for each.
fn traffic<'a>(streams: impl Iterator<Item = &'a Stream>) -> usize {
    let mut seen = Vec::new();
    for stream in streams {
        if !seen.contains(&stream.frames()) {
            seen.push(stream.frames());
        }
    }
    seen.len()
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
    let topology = topology_of(triib);
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
            .map(|stream| (stream.key(), stream.status))
            .collect::<Vec<_>>(),
    );
    // What is brought forward, as long as it is still there.
    let focus = triib.network_focus.filter(|focus| match focus {
        Focus::Stream(..) => streams.iter().any(|stream| stream.focus() == *focus),
        Focus::Output(talker, output) => streams
            .iter()
            .any(|stream| stream.key() == (*talker, *output)),
        Focus::Node(key) => find(&topology, *key).is_some(),
    });
    let (header, focused) = header(&topology, shows, &streams, focus, width, list_open);
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
    // The same whether this computer's entities run gPTP, AVB Lite or
    // nothing.
    let labels = [fl!("netmap-sync"), fl!("netmap-audio"), "CRF".to_owned()];
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
            Focus::Output(talker, output) => streams
                .iter()
                .find(|stream| stream.key() == (talker, output))
                .map(|stream| stream.output_name.clone())
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
    // The space before what ends the row, and the list's toggle, an icon
    // button.
    used.push(0.0);
    if !wide {
        used.push(40.0);
    }
    let room = width - 2.0 * VIEW_MARGIN;
    // Whether the row has room for one more of `across` pixels, and the gap
    // before it.
    let fits = |used: &[f32], across: f32| {
        used.iter().sum::<f32>() + across + 12.0 * used.len() as f32 <= room
    };
    // The title first, so the row stays put as what is brought forward
    // comes and goes; then what is brought forward, rather than below the
    // map; then the summary, where the title is.
    let title = fl!("toolbar-network");
    let title_width = scramble_ui::font::measure(&title, Type::TitleLarge).ceil();
    let title = fits(&used, title_width).then(|| {
        used.push(title_width);
        title
    });
    let (beside, below) = match focused {
        Some(label) if fits(&used, labelled(&label)) => {
            used.push(labelled(&label));
            (Some(label), None)
        }
        label => (None, label),
    };
    let summary = title.is_some().then(|| summary(topology)).filter(|counts| {
        fits(
            &used,
            scramble_ui::font::measure(counts, Type::BodyMedium).ceil(),
        )
    });
    let mut header = line![].spacing(12).align_y(Center);
    if let Some(title) = title {
        header = header.push(styled(title, Type::TitleLarge));
    }
    header = header.push(choices);
    if let Some(counts) = summary {
        header = header.push(styled(counts, Type::BodyMedium).style(style::on_surface_variant));
    }
    header = header.push(space::horizontal());
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

/// How many devices and bridges the map shows: AVB bridges, and the
/// switches taking no part in AVB that devices fell back to AVB Lite behind.
fn summary(topology: &Topology) -> String {
    let bridges = topology
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, NodeKind::Bridge | NodeKind::Switch { .. }))
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
    crate::i18n::list(vec![
        fl!("netmap-devices", count = devices),
        fl!("netmap-bridges", count = bridges),
    ])
}

/// A bound stream input as the map traces it, for agents.
pub struct Traced {
    pub listener: (EntityId, u16),
    pub talker: (EntityId, u16),
    pub status: Status,
    /// How it is going, as the map's details say it.
    pub reason: String,
    /// The bridges it crosses from the talker, as far as it gets.
    pub via: Vec<String>,
    /// Where it stops short of the listener, when it does.
    pub stops_at: Option<String>,
}

/// Every bound stream input on the network, whatever the search, as the
/// map traces it.
pub fn traced(triib: &Triib) -> Vec<Traced> {
    let topology = topology_of(triib);
    let listeners: Vec<&DiscoveredEntity> = triib.entities.values().collect();
    bound_streams(triib, &topology, &listeners)
        .into_iter()
        .filter_map(|stream| {
            Some(Traced {
                listener: stream.input?,
                talker: (stream.talker, stream.output),
                status: stream.status,
                via: via(&topology, &stream.route)
                    .into_iter()
                    .map(|node| node_name(&topology, node))
                    .collect(),
                stops_at: stream.route.stops.map(|node| node_name(&topology, node)),
                reason: stream.reason,
            })
        })
        .collect()
}

/// A node of the clock tree as agents read it.
pub struct ClockNode {
    pub name: String,
    pub entity_id: Option<EntityId>,
    /// Where it hangs in the tree is known.
    pub placed: bool,
    pub synced: bool,
    /// How it keeps time, or why it does not, as the map's details say it.
    pub state: String,
}

/// The clock tree as agents read it: each domain's grandmaster, and every
/// node with where it hangs, how its link up runs, and whether and how it
/// keeps time; and this computer's ptp4l, where it runs one.
pub fn clock_tree(triib: &Triib) -> (Vec<ClockNode>, serde_json::Value) {
    use serde_json::json;

    let topology = topology_of(triib);
    let mut nodes = Vec::new();
    let mut listed = Vec::new();
    for (node, entry) in topology.nodes.iter().enumerate() {
        let state = clock_details(triib, &topology, node).state;
        let ok = synced(&topology, node);
        let (kind, entity_id, interface) = match entry.kind {
            NodeKind::Bridge => ("bridge", None, None),
            NodeKind::Switch { .. } => ("switch taking no part in AVB", None, None),
            NodeKind::Entity {
                entity_id,
                interface,
            } => ("entity", Some(entity_id), Some(interface)),
            NodeKind::Host => ("this computer", None, None),
            NodeKind::Clock => ("PTP grandmaster", None, None),
        };
        let counters = entity_id.zip(interface).and_then(|(entity_id, interface)| {
            triib
                .models
                .get(&entity_id)?
                .counters(DescriptorType::AVB_INTERFACE, interface)?
                .avb_interface()
        });
        let link = &entry.link;
        listed.push(json!({
            "id": node,
            "name": node_name(&topology, node),
            "kind": kind,
            "entity_id": entity_id.map(|entity_id| entity_id.to_string()),
            "interface": interface,
            "clock_identity": entry.clock.map(|clock| clock.to_string()),
            "parent": entry.parent,
            "domain": entry.domain,
            "synced": ok,
            "state": state,
            "avb_lite": entry.lite.map(|lite| match lite {
                crate::topology::Lite::FellBack => "fell back, finding no AVB bridge",
                crate::topology::Lite::Only => "runs only AVB Lite",
                crate::topology::Lite::Set => "set to AVB Lite",
            }),
            "link_up": (entry.parent.is_some() || link != &Default::default()).then(|| json!({
                "gptp": link.synced,
                "peer_delay_ns": link.delay,
                "bridge_port": link.port,
                "offset_from_grandmaster_ns": link.offset,
                "wifi": link.wireless.map(|hop| json!({
                    "time_locked": hop.locked,
                    "holdover": hop.holdover,
                    "signal_dbm": hop.signal,
                })),
            })),
            "link_drops": counters.and_then(|counters| counters.link_down),
            "grandmaster_changes": counters.and_then(|counters| counters.gptp_gm_changed),
        }));
        nodes.push(ClockNode {
            name: node_name(&topology, node),
            entity_id,
            placed: entry.apart.is_none(),
            synced: ok,
            state,
        });
    }
    let grandmasters: Vec<String> = topology
        .roots
        .iter()
        .map(|&root| match topology.nodes[root].kind {
            NodeKind::Switch { grandmaster, .. } => clock_name(&topology, grandmaster),
            _ => node_name(&topology, root),
        })
        .collect();
    let ptp4l = triib
        .endpoints
        .as_ref()
        .filter(|status| Some(status.interface.as_str()) == triib.settings.interface.as_deref())
        .and_then(|status| status.clock.as_ref())
        .map(|clock| {
            json!({
                "grandmaster": clock.grandmaster().map(|clock| clock.to_string()),
                "offset_ns": clock.offset,
                "end_to_end": clock.end_to_end,
            })
        });
    let tree = json!({
        "grandmasters": grandmasters,
        "nodes": listed,
        "this_computer_ptp4l": ptp4l,
    });
    (nodes, tree)
}

/// The entities the map shows: all but controllers, which take no part
/// in the clock or the streams.
fn mapped_entities(triib: &Triib) -> impl Iterator<Item = &DiscoveredEntity> {
    triib
        .entities
        .values()
        .filter(|entity| !crate::describe::is_only_controller(&entity.adp))
}

/// The clock domains each entity on the map is in, by the node heading
/// each: one with interfaces in several is in each, and one whose domain
/// is not known in none; and those this computer keeps time in.
pub struct Domains {
    entities: std::collections::HashMap<EntityId, Vec<NodeId>>,
    /// Where this computer, or one of its entities standing for it on the
    /// map, is in sync.
    host: Vec<NodeId>,
}

impl Domains {
    pub fn of(triib: &Triib) -> Self {
        let topology = topology_of(triib);
        let mut domains = Self::in_topology(&topology);
        for (node, entry) in topology.nodes.iter().enumerate() {
            if let NodeKind::Entity { entity_id, .. } = entry.kind
                && entry.apart.is_none()
                && entry.link.synced
                && triib.is_this_computers(entity_id)
            {
                domains.host.push(topology.root_of(node));
            }
        }
        domains
    }

    pub(crate) fn in_topology(topology: &Topology) -> Self {
        let mut entities: std::collections::HashMap<EntityId, Vec<NodeId>> =
            std::collections::HashMap::new();
        let mut host = Vec::new();
        for (node, entry) in topology.nodes.iter().enumerate() {
            if entry.apart.is_some() {
                continue;
            }
            match entry.kind {
                NodeKind::Entity { entity_id, .. } => {
                    entities
                        .entry(entity_id)
                        .or_default()
                        .push(topology.root_of(node));
                }
                NodeKind::Host if entry.link.synced => host.push(topology.root_of(node)),
                _ => {}
            }
        }
        Self { entities, host }
    }

    /// Whether the two keep time in different domains, so no stream can
    /// pass between them, as far as their domains are known.
    pub fn apart(&self, one: EntityId, other: EntityId) -> bool {
        match (self.entities.get(&one), self.entities.get(&other)) {
            (Some(one), Some(other)) => !one.iter().any(|domain| other.contains(domain)),
            _ => false,
        }
    }

    /// Whether this computer keeps time in sync in a domain.
    pub fn host_synced(&self) -> bool {
        !self.host.is_empty()
    }

    /// Whether `entity_id` keeps time in none of the domains this computer
    /// is in sync in, as far as its domain is known.
    pub fn outside_host(&self, entity_id: EntityId) -> bool {
        self.host_synced()
            && self
                .entities
                .get(&entity_id)
                .is_some_and(|domains| !domains.iter().any(|domain| self.host.contains(domain)))
    }
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
                            lite: model.lite_status(interface.index),
                            gptp: interface
                                .interface_flags
                                .contains(InterfaceFlags::GPTP_SUPPORTED),
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
    // What it is plugged into, from the ports heard on its link: those
    // with an entity's clock are endpoints, however they declare themselves.
    let entity = |clock: ClockIdentity| {
        reports
            .iter()
            .flat_map(|report| &report.interfaces)
            .any(|interface| interface.clock == clock)
    };
    // Its clock, as its triib-endpointd's ptp4l runs it on the interface.
    let clock = triib
        .endpoints
        .as_ref()
        .filter(|status| Some(status.interface.as_str()) == triib.settings.interface.as_deref())
        .and_then(|status| status.clock.as_ref())
        .and_then(|clock| {
            Some(HostClock {
                grandmaster: clock.grandmaster()?,
                offset: clock.offset.and_then(|offset| i32::try_from(offset).ok()),
                ptp: clock.end_to_end,
            })
        });
    let host = (matches!(triib.network_state, NetworkState::Running { .. }) && !shown_by_entities)
        .then(|| match Plugged::of(&triib.peers, entity) {
            _ if triib.cannot_listen.is_some() => HostReport::CannotListen,
            Plugged::Bridge(bridge) => HostReport::Heard(bridge.clock, bridge.port, bridge.synced),
            Plugged::Shared(endpoints) => {
                HostReport::Shared(endpoints.iter().map(|peer| peer.clock).collect(), clock)
            }
            Plugged::Unheard => HostReport::Unheard,
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

fn key_of(topology: &Topology, node: NodeId) -> NodeKey {
    match topology.nodes[node].kind {
        NodeKind::Entity { entity_id, .. } => NodeKey::Entity(entity_id),
        NodeKind::Host => NodeKey::Host,
        NodeKind::Bridge | NodeKind::Clock => {
            NodeKey::Clock(topology.nodes[node].clock.unwrap_or(ClockIdentity(0)))
        }
        NodeKind::Switch {
            grandmaster,
            domain,
        } => NodeKey::Switch(grandmaster, domain),
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
            .and_then(crate::vendor::name)
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
        NodeKind::Switch { .. } => fl!("netmap-plain-switch"),
    }
}

/// The name of the node a clock identity is, else the identity.
fn clock_name(topology: &Topology, clock: ClockIdentity) -> String {
    topology
        .nodes
        .iter()
        .position(|node| node.clock == Some(clock) && !matches!(node.kind, NodeKind::Switch { .. }))
        .map_or_else(|| clock.to_string(), |node| node_name(topology, node))
}

/// The AVB Lite status an entity's interface reports.
fn lite_status<'a>(triib: &'a Triib, topology: &Topology, node: NodeId) -> Option<&'a LiteStatus> {
    match topology.nodes[node].kind {
        NodeKind::Entity {
            entity_id,
            interface,
        } => triib
            .models
            .get(&entity_id)
            .and_then(|model| model.lite_status(interface)),
        _ => None,
    }
}

/// Whether a device running AVB Lite is the PTP grandmaster it follows.
fn leads_ptp(triib: &Triib, topology: &Topology, node: NodeId) -> bool {
    topology.runs_lite(node)
        && lite_status(triib, topology, node).is_some_and(|status| {
            status.grandmaster.0 == 0 || Some(status.grandmaster) == topology.nodes[node].clock
        })
}

/// Whether the node is the grandmaster of the switches taking no part in
/// AVB that it hangs under, its clock going out through them.
fn leads_switch(topology: &Topology, node: NodeId) -> bool {
    let entry = &topology.nodes[node];
    entry.parent.is_some_and(|parent| {
        matches!(
            topology.nodes[parent].kind,
            NodeKind::Switch { grandmaster, .. } if Some(grandmaster) == entry.clock
        )
    })
}

/// AVB Lite PTP and its domain, as the switches that take no part in AVB
/// carry it.
fn lite_domain(domain: u8) -> String {
    fl!(
        "lite-ptp-domain",
        profile = crate::lite_view::profile_name(PtpProfile::AVB_LITE_PTP),
        domain = domain
    )
}

/// Whether a talker says it sends a stream output: it counts a connection
/// to it, flags it connected, or has started it more often than stopped it.
fn talker_sends(model: &EntityModel, output: u16) -> bool {
    model
        .tx_state(output)
        .is_some_and(|state| state.connection_count > 0)
        || model
            .stream_info(DescriptorType::STREAM_OUTPUT, output)
            .is_some_and(|info| info.flags.contains(StreamInfoFlags::CONNECTED))
        || model
            .stream_output_counters(output)
            .is_some_and(
                |counters| match (counters.stream_start, counters.stream_stop) {
                    (Some(start), Some(stop)) => start > stop,
                    _ => false,
                },
            )
}

/// Whether a stream goes to a multicast address, as its stream info says
/// where it gives the address. Bridges copy such a stream out to each
/// listener from where their paths part; a unicast one, as AVB Lite can
/// send, the talker sends each listener a copy of its own.
fn multicast(info: Option<&StreamInfo>) -> Option<bool> {
    info.filter(|info| info.flags.contains(StreamInfoFlags::STREAM_DEST_MAC_VALID))
        .map(|info| info.stream_dest_mac.0[0] & 1 == 1)
}

/// Every bound stream input, by listener, with how far its stream gets;
/// then each stream output its talker sends that flows to none of them.
fn streams(triib: &Triib, topology: &Topology) -> Vec<Stream> {
    let mut streams = bound_streams(triib, topology, &triib.shown_entities());
    for talker in triib.shown_entities() {
        let talker_id = talker.entity_id();
        let Some(model) = triib.models.get(&talker_id) else {
            continue;
        };
        for output in model.streams(false) {
            let flowing = streams.iter().any(|stream| {
                stream.key() == (talker_id, output.index) && stream.status == Status::Flowing
            });
            if flowing || !talker_sends(model, output.index) {
                continue;
            }
            let name = stream_name(Some(model), DescriptorType::STREAM_OUTPUT, output.index);
            streams.push(Stream {
                input: None,
                talker: talker_id,
                output: output.index,
                name: name.clone(),
                output_name: name,
                talker_name: triib.entity_name(talker),
                listener_name: fl!("netmap-no-listener"),
                media_clock: output.is_media_clock(),
                multicast: multicast(
                    model.stream_info(DescriptorType::STREAM_OUTPUT, output.index),
                )
                .unwrap_or(true),
                status: Status::Flowing,
                reason: fl!("netmap-sending-unheard"),
                route: talker_leg(topology, talker_id),
            });
        }
    }
    streams
}

/// The link up from a talker to its bridge, as far as a stream gets that
/// reaches no listener.
fn talker_leg(topology: &Topology, talker: EntityId) -> Route {
    match topology.placed(talker) {
        Some(node) if topology.nodes[node].parent.is_some() => Route {
            hops: vec![(node, true)],
            stops: topology.nodes[node].parent,
        },
        _ => Route::default(),
    }
}

/// Every bound stream input of `listeners`, by listener, with how far its
/// stream gets.
fn bound_streams(
    triib: &Triib,
    topology: &Topology,
    listeners: &[&DiscoveredEntity],
) -> Vec<Stream> {
    let mut streams = Vec::new();
    for &listener in listeners {
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
            let talker_leg = talker_leg(topology, talker_id);
            let (route, reason) = match status {
                Status::Flowing | Status::Unsynced => (
                    topology.route(talker_id, listener_id),
                    fl!("netmap-connected"),
                ),
                Status::Advertised => (
                    talker_leg,
                    if topology.placed(listener_id).is_none() {
                        fl!(
                            "netmap-advertised-unknown-domain",
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
            let output_name = stream_name(talker_model, DescriptorType::STREAM_OUTPUT, output);
            streams.push(Stream {
                input: Some((listener_id, input.index)),
                talker: talker_id,
                output,
                name: {
                    let input = stream_name(Some(model), DescriptorType::STREAM_INPUT, input.index);
                    format!("{output_name} {} {input}", crate::i18n::arrow(&output_name))
                },
                output_name,
                talker_name: triib.entity_name_of(talker_id),
                listener_name,
                media_clock: input.is_media_clock(),
                // As the listener has it, else as the talker sends it.
                multicast: multicast(info)
                    .or_else(|| {
                        multicast(talker_model.and_then(|model| {
                            model.stream_info(DescriptorType::STREAM_OUTPUT, output)
                        }))
                    })
                    .unwrap_or(true),
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

/// Each stream's color, from its talker's stream output and its status:
/// every talker has a hue and its stream outputs are shades of it, the
/// same for every listener of one; streams that are not flowing lose their
/// hue.
fn paints(streams: &[((EntityId, u16), Status)]) -> Vec<Paint> {
    let mut talkers: Vec<EntityId> = Vec::new();
    let mut outputs: Vec<(EntityId, u16)> = Vec::new();
    for &(output, _) in streams {
        if !talkers.contains(&output.0) {
            talkers.push(output.0);
        }
        if !outputs.contains(&output) {
            outputs.push(output);
        }
    }
    streams
        .iter()
        .map(|&(output, status)| {
            let talker = output.0;
            let own: Vec<&(EntityId, u16)> =
                outputs.iter().filter(|other| other.0 == talker).collect();
            let place = own.iter().position(|&&other| other == output).unwrap_or(0);
            let shade = if own.len() == 1 {
                50
            } else {
                (place * 100 / (own.len() - 1)) as u8
            };
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
    entity.is_some() && (entity == Some(stream.talker) || entity == stream.listener())
        || via(topology, &stream.route).contains(&node)
}

/// Whether a stream is lit under `focus`.
fn lit(topology: &Topology, stream: &Stream, focus: Option<Focus>) -> bool {
    match focus {
        None => true,
        Some(Focus::Stream(..)) => focus == Some(stream.focus()),
        Some(Focus::Output(talker, output)) => stream.key() == (talker, output),
        Some(Focus::Node(key)) => {
            find(topology, key).is_some_and(|node| touches(topology, stream, node))
        }
    }
}

/// Whether a stream is the one brought forward, or one of its talker's
/// stream's listeners when that is.
fn picked(topology: &Topology, stream: &Stream, focus: Option<Focus>) -> bool {
    matches!(focus, Some(Focus::Stream(..) | Focus::Output(..))) && lit(topology, stream, focus)
}

fn lines(
    topology: &Topology,
    shows: NetworkShows,
    streams: &[Stream],
    paints: &[Paint],
    focus: Option<Focus>,
) -> Vec<Line> {
    if shows == NetworkShows::Clock {
        // The link up from every node on a tree, the clock flowing down it,
        // but up it from a grandmaster behind the switches in front of it,
        // which carry its clock to the others there.
        let focused = match focus {
            Some(Focus::Node(key)) => find(topology, key),
            _ => None,
        };
        let parent = |node: NodeId| topology.nodes[node].parent;
        let ahead = |node: NodeId| parent(node).filter(|_| leads_switch(topology, node));
        return (0..topology.nodes.len())
            .filter(|&node| topology.nodes[node].parent.is_some())
            .map(|node| {
                let synced = topology.nodes[node].link.synced;
                Line {
                    hops: vec![(node, ahead(node).is_some())],
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
                        below(topology, node, focused)
                            || below(topology, focused, node)
                            || ahead(node).is_some_and(|switch| below(topology, focused, switch))
                            || ahead(focused).is_some_and(|switch| below(topology, node, switch))
                    }),
                    picked: false,
                    focus: Focus::Node(key_of(topology, node)),
                    stream: None,
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
            picked: picked(topology, stream, focus),
            focus: stream.focus(),
            stream: stream.multicast.then(|| stream.key()),
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
    let chosen: Vec<(&Stream, Paint)> = streams
        .iter()
        .zip(paints)
        .filter(|(stream, _)| picked(topology, stream, focus))
        .map(|(stream, &paint)| (stream, paint))
        .collect();
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
                NodeKind::Switch { .. } => (Icon::Lan, String::new()),
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
                (_, Some(Apart::PlainSwitch), _) => {
                    (fl!("netmap-behind-plain-switch"), Paint::Muted)
                }
                (_, Some(Apart::OnThisComputer), _) => {
                    (fl!("netmap-on-this-computer"), Paint::Muted)
                }
                (_, Some(Apart::NoPath(_)), _) => (fl!("netmap-path-not-reported"), Paint::Failed),
                (_, Some(Apart::Unreported), _) => (fl!("netmap-gptp-not-reported"), Paint::Failed),
                (NetworkShows::Clock, None, NodeKind::Switch { domain, .. }) => {
                    (lite_domain(domain), Paint::Muted)
                }
                (NetworkShows::Clock, None, _)
                    if entry.parent.is_none() || leads_ptp(triib, topology, node) =>
                {
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
                // A bridge, or an entity streams pass through, as an access
                // point; in AVB Lite they pass the switches in front instead.
                (_, None, _)
                    if entity.is_none()
                        || (!entry.children.is_empty() && !topology.runs_lite(node)) =>
                {
                    let through = traffic(
                        streams
                            .iter()
                            .filter(|stream| via(topology, &stream.route).contains(&node)),
                    );
                    (fl!("netmap-through-count", count = through), Paint::Muted)
                }
                (_, None, _) => {
                    let sending = traffic(
                        streams
                            .iter()
                            .filter(|stream| entity == Some(stream.talker)),
                    );
                    let receiving = streams
                        .iter()
                        .filter(|stream| entity.is_some() && entity == stream.listener())
                        .count();
                    let failed = streams
                        .iter()
                        .filter(|stream| {
                            entity.is_some()
                                && entity == stream.listener()
                                && stream.status == Status::Failed
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
            // The stream brought forward that the node listens to, else one
            // it sends.
            let in_picked = chosen
                .iter()
                .find(|(stream, _)| entity.is_some() && entity == stream.listener())
                .or_else(|| {
                    chosen
                        .iter()
                        .find(|(stream, _)| entity == Some(stream.talker))
                });
            let mut strong = false;
            if let Some(&(_, paint)) = in_picked {
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
                Some(_) if focused || in_picked.is_some() => false,
                Some(Focus::Stream(..) | Focus::Output(..)) => {
                    !on_lit_line(node, &|line| line.picked)
                }
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
                    || matches!(entry.kind, NodeKind::Switch { .. })
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
        (_, Some(Focus::Output(talker, output)), _) => {
            let group: Vec<(&Stream, Paint)> = streams
                .iter()
                .zip(paints)
                .filter(|(stream, _)| stream.key() == (talker, output))
                .map(|(stream, &paint)| (stream, paint))
                .collect();
            output_details(shows, &group)
        }
        (_, _, Some(node)) => node_details(topology, streams, paints, node),
        _ => stream_overview(topology, shows, streams, paints),
    }
}

/// A stream's state in a word, and the paint it is written in.
fn state_word(stream: &Stream) -> (String, Paint) {
    match stream.status {
        Status::Flowing | Status::Unsynced if stream.input.is_none() => {
            (fl!("netmap-sending"), Paint::Muted)
        }
        Status::Flowing | Status::Unsynced => (fl!("netmap-connected"), Paint::Muted),
        Status::Advertised => (fl!("netmap-advertised-only"), Paint::Soft),
        Status::Failed => (fl!("netmap-failed-state"), Paint::Failed),
    }
}

fn stream_item(stream: &Stream, paint: Paint) -> Item {
    let (state, detail_paint) = state_word(stream);
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

/// The stream of a talker's stream's listeners that says most of it: one
/// flowing, else the first.
fn lead<'a>(group: &[(&'a Stream, Paint)]) -> (&'a Stream, Paint) {
    *group
        .iter()
        .find(|(stream, _)| stream.status == Status::Flowing)
        .unwrap_or(&group[0])
}

/// A multicast stream to several listeners, listed as its talker's stream,
/// which it brings forward whole.
fn output_item(group: &[(&Stream, Paint)]) -> Item {
    let (stream, paint) = lead(group);
    let (state, detail_paint) = state_word(stream);
    let listeners: Vec<String> = group
        .iter()
        .map(|(stream, _)| stream.listener_name.clone())
        .collect();
    Item {
        name: stream.output_name.clone(),
        detail: fl!(
            "netmap-stream-item",
            talker = stream.talker_name.clone(),
            listener = crate::i18n::list(listeners),
            state = state
        ),
        detail_paint,
        swatch: paint,
        focus: Focus::Output(stream.talker, stream.output),
    }
}

/// The items for the streams `test` lets through, one for each stream of
/// frames: a multicast stream to several listeners as one.
fn grouped(streams: &[Stream], paints: &[Paint], test: &dyn Fn(&Stream) -> bool) -> Vec<Item> {
    let mut groups: Vec<Vec<(&Stream, Paint)>> = Vec::new();
    for (stream, &paint) in streams
        .iter()
        .zip(paints)
        .filter(|(stream, _)| test(stream))
    {
        match groups
            .iter_mut()
            .find(|group| group[0].0.frames() == stream.frames())
        {
            Some(group) => group.push((stream, paint)),
            None => groups.push(vec![(stream, paint)]),
        }
    }
    groups
        .iter()
        .map(|group| match group.as_slice() {
            [(stream, paint)] => stream_item(stream, *paint),
            _ => output_item(group),
        })
        .collect()
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

/// Whether the node is in sync: in a domain, with the clock running up
/// to it from its grandmaster, or following it closely in AVB Lite.
fn synced(topology: &Topology, node: NodeId) -> bool {
    let entry = &topology.nodes[node];
    entry.apart.is_none() && (entry.parent.is_none() || entry.link.synced)
}

/// What the overviews add to their help where the map holds more than one
/// domain or a PTP one.
fn with_domain_help(topology: &Topology, help: String) -> String {
    if topology.roots.len() > 1 || topology.roots.iter().any(|&root| topology.is_ptp(root)) {
        format!("{help}\n\n{}", fl!("netmap-help-domains"))
    } else {
        help
    }
}

fn clock_item(topology: &Topology, node: NodeId) -> Item {
    let ok = synced(topology, node);
    Item {
        name: node_name(topology, node),
        detail: if ok {
            fl!("netmap-synced")
        } else {
            fl!("netmap-not-synced")
        },
        detail_paint: if ok { Paint::Muted } else { Paint::Failed },
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
    let status = lite_status(triib, topology, node).copied();
    let mut facts = Vec::new();
    // The grandmaster it names, else the one it hangs under.
    let grandmaster = match status {
        Some(status) if status.grandmaster.0 != 0 => Some(clock_name(topology, status.grandmaster)),
        _ => entry.parent.map(|parent| node_name(topology, parent)),
    };
    if let Some(grandmaster) = grandmaster {
        facts.push((fl!("avb-interface-grandmaster"), grandmaster));
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
    let leads = leads_ptp(triib, topology, node);
    let state = match (entry.apart, entry.link.offset) {
        (Some(apart), _) => apart_reason(apart),
        (None, _) if leads => fl!("avb-interface-grandmaster"),
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

/// Why a node is in no domain known, in words.
fn apart_reason(apart: Apart) -> String {
    match apart {
        Apart::NoPath(grandmaster) => fl!(
            "netmap-apart-no-path",
            grandmaster = grandmaster.to_string()
        ),
        Apart::Unreported => fl!("netmap-apart-unreported"),
        Apart::NoNeighbor => fl!("netmap-apart-no-neighbor"),
        Apart::CannotListen => fl!("netmap-apart-cannot-listen"),
        Apart::PlainSwitch => fl!("netmap-apart-plain-switch"),
        Apart::OnThisComputer => fl!("netmap-apart-on-this-computer"),
    }
}

fn clock_overview(topology: &Topology) -> Page {
    let unsynced: Vec<Item> = (0..topology.nodes.len())
        .filter(|&node| !synced(topology, node))
        .map(|node| clock_item(topology, node))
        .collect();
    // Each domain's, a PTP domain behind a switch naming its own.
    let grandmasters: Vec<String> = topology
        .roots
        .iter()
        .map(|&root| match topology.nodes[root].kind {
            NodeKind::Switch { grandmaster, .. } => clock_name(topology, grandmaster),
            _ => node_name(topology, root),
        })
        .collect();
    Page {
        kicker: fl!("netmap-sync"),
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
        help: with_domain_help(topology, fl!("netmap-help-clock")),
    }
}

fn clock_details(triib: &Triib, topology: &Topology, node: NodeId) -> Page {
    let entry = &topology.nodes[node];
    let kicker = match entry.kind {
        NodeKind::Bridge => fl!("netmap-bridge"),
        NodeKind::Clock => "PTP".to_owned(),
        _ => fl!("netmap-device"),
    };
    if let NodeKind::Switch {
        grandmaster,
        domain,
    } = entry.kind
    {
        return switch_details(topology, node, grandmaster, domain);
    }
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
    if topology.runs_lite(node) {
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
    if let Some(offset) = entry.link.offset {
        facts.push((fl!("lite-offset"), crate::lite_view::duration(offset)));
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
        // Behind a switch taking no part in AVB, in a PTP domain.
        None if entry.kind == NodeKind::Host && topology.is_ptp(topology.root_of(node)) => {
            match entry.link.offset {
                Some(offset) => fl!(
                    "netmap-ptp-offset-high",
                    offset = crate::lite_view::duration(offset)
                ),
                None => fl!("netmap-host-no-ptp"),
            }
        }
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

/// The switches that take no part in AVB in front of devices that fell back
/// to AVB Lite: the PTP grandmaster those follow and how closely.
fn switch_details(
    topology: &Topology,
    node: NodeId,
    grandmaster: ClockIdentity,
    domain: u8,
) -> Page {
    let children = &topology.nodes[node].children;
    let unsynced = children
        .iter()
        .filter(|&&child| !synced(topology, child))
        .count();
    Page {
        kicker: "AVB Lite".to_owned(),
        title: node_name(topology, node),
        swatch: Some(Paint::Clock),
        state: fl!("netmap-switch-state"),
        state_icon: Icon::Lan,
        state_filled: false,
        state_paint: Paint::Soft,
        facts: vec![
            (
                fl!("avb-interface-grandmaster"),
                clock_name(topology, grandmaster),
            ),
            ("PTP".to_owned(), lite_domain(domain)),
            (
                fl!("netmap-synced"),
                (children.len() - unsynced).to_string(),
            ),
            (fl!("netmap-not-synced"), unsynced.to_string()),
        ],
        sections: vec![(
            fl!("netmap-nodes-below"),
            children
                .iter()
                .map(|&child| clock_item(topology, child))
                .collect(),
        )],
        help: fl!("netmap-help-back"),
    }
}

fn stream_overview(
    topology: &Topology,
    shows: NetworkShows,
    streams: &[Stream],
    paints: &[Paint],
) -> Page {
    let bound = || streams.iter().filter(|stream| stream.input.is_some());
    let count = |status: Status| bound().filter(|stream| stream.status == status).count();
    let attention: Vec<Item> = streams
        .iter()
        .zip(paints)
        .filter(|(stream, _)| stream.status != Status::Flowing)
        .map(|(stream, &paint)| stream_item(stream, paint))
        .collect();
    let unheard: Vec<Item> = streams
        .iter()
        .zip(paints)
        .filter(|(stream, _)| stream.input.is_none())
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
        state: fl!("netmap-bound", count = bound().count()),
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
        sections: [
            (fl!("netmap-needs-attention"), attention),
            (fl!("netmap-no-listener"), unheard),
        ]
        .into_iter()
        .filter(|(_, items)| !items.is_empty())
        .collect(),
        help: with_domain_help(
            topology,
            if media_clock {
                fl!("netmap-help-media-clock")
            } else {
                fl!("netmap-help-audio")
            },
        ),
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
    let (state_icon, state_filled, state_paint) = state_look(stream.status);
    Page {
        kicker: stream_kicker(shows),
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

/// The icon a stream's state shows with, whether filled, and its paint.
fn state_look(status: Status) -> (Icon, bool, Paint) {
    match status {
        Status::Flowing | Status::Unsynced => (Icon::Link, true, Paint::Soft),
        Status::Advertised => (Icon::Pending, false, Paint::Soft),
        Status::Failed => (Icon::Error, true, Paint::Failed),
    }
}

fn stream_kicker(shows: NetworkShows) -> String {
    if shows == NetworkShows::MediaClock {
        fl!("netmap-media-clock-stream")
    } else {
        fl!("netmap-audio-stream")
    }
}

/// A talker's stream with its listeners, each brought forward on its own
/// from the list; or one it sends with no listener known.
fn output_details(shows: NetworkShows, group: &[(&Stream, Paint)]) -> Page {
    let (stream, paint) = lead(group);
    let listeners: Vec<Item> = group
        .iter()
        .filter(|(stream, _)| stream.input.is_some())
        .map(|&(stream, paint)| stream_item(stream, paint))
        .collect();
    let (state_icon, state_filled, state_paint) = state_look(stream.status);
    Page {
        kicker: stream_kicker(shows),
        title: stream.output_name.clone(),
        swatch: Some(paint),
        state: stream.reason.clone(),
        state_icon,
        state_filled,
        state_paint,
        facts: vec![
            (fl!("advert-talker"), stream.talker_name.clone()),
            (fl!("stream-listeners"), listeners.len().to_string()),
        ],
        sections: if listeners.is_empty() {
            Vec::new()
        } else {
            vec![(fl!("stream-listeners"), listeners)]
        },
        help: fl!("netmap-help-back"),
    }
}

fn node_details(topology: &Topology, streams: &[Stream], paints: &[Paint], node: NodeId) -> Page {
    let entry = &topology.nodes[node];
    let entity = entity_of(topology, node);
    let items = |test: &dyn Fn(&Stream) -> bool| grouped(streams, paints, test);
    let bridge = entity.is_none() || !entry.children.is_empty();
    let glyph = match entry.kind {
        NodeKind::Bridge => Icon::Hub,
        NodeKind::Host => Icon::Computer,
        NodeKind::Entity { .. } => Icon::GraphicEq,
        NodeKind::Clock => Icon::Schedule,
        NodeKind::Switch { .. } => Icon::Lan,
    };
    if bridge && entry.kind != NodeKind::Host {
        let through = items(&|stream| via(topology, &stream.route).contains(&node));
        return Page {
            kicker: match entry.kind {
                NodeKind::Switch { .. } => "AVB Lite".to_owned(),
                _ => fl!("netmap-bridge"),
            },
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
    let receiving = items(&|stream| entity.is_some() && entity == stream.listener());
    let problems = streams
        .iter()
        .filter(|stream| {
            entity.is_some()
                && (entity == Some(stream.talker) || entity == stream.listener())
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
                    lite: None,
                    gptp: true,
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
                input: Some((EntityId(listener), input)),
                talker: EntityId(talker),
                output: input,
                name: name.to_owned(),
                output_name: name.split(" → ").next().unwrap_or(name).to_owned(),
                talker_name: names[talker as usize - 1].to_owned(),
                listener_name: names[listener as usize - 1].to_owned(),
                media_clock: false,
                multicast: true,
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
        let theme = crate::view::tests::picture_theme();
        let (topology, streams) = bench();
        let paints = paints(
            &streams
                .iter()
                .map(|stream| (stream.key(), stream.status))
                .collect::<Vec<_>>(),
        );
        let console = find(&topology, NodeKey::Entity(EntityId(2))).unwrap();
        // How to build each page, by the name its picture takes.
        type Build<'a> = Box<dyn Fn() -> Page + 'a>;
        let pages: [(&str, Build<'_>); 4] = [
            (
                "overview",
                Box::new(|| stream_overview(&topology, NetworkShows::Audio, &streams, &paints)),
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
            clock: None,
            setup: None,
            sharers: Vec::new(),
            endpoints: vec![EndpointStatus {
                entity_id: crate::view::tests::WIRED_ESP.to_string(),
                name: "Host entity 1".into(),
                state: "running".into(),
                streams: Vec::new(),
            }],
        });
        assert!(!kinds(&triib).contains(&NodeKind::Host));
    }

    /// A stream output its talker has started more often than stopped is
    /// sent, and shows on the map as far as the talker's bridge while no
    /// listener triib reads takes it.
    #[test]
    fn a_stream_sent_to_no_known_listener_shows() {
        let (entities, mut models) = crate::view::tests::bench();
        let esp = crate::view::tests::WIRED_ESP;
        let read = |counting| {
            crate::view::tests::reread(&entities, &models, esp, |_| {}, &[], &[], counting)
        };
        let (started, idle) = (read(true), read(false));
        assert!(talker_sends(&started, 0));
        assert!(!talker_sends(&idle, 0));
        models.insert(esp, started);
        let interface = avb_net::Interface {
            name: "enp6s0".to_owned(),
            mac: avb_net::MacAddress([0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b]),
            up: true,
            speed: None,
            physical: true,
            wireless: false,
            hardware_clock: None,
        };
        let triib = Triib::sample(
            crate::settings::Settings::default(),
            interface,
            entities,
            models,
        );
        let topology = topology_of(&triib);
        let unheard: Vec<Stream> = streams(&triib, &topology)
            .into_iter()
            .filter(|stream| stream.talker == esp && stream.input.is_none())
            .collect();
        assert!(!unheard.is_empty());
        for stream in &unheard {
            assert_eq!(stream.status, Status::Flowing);
            assert_eq!(stream.focus(), Focus::Output(esp, stream.output));
            assert_eq!(stream.route.hops, talker_leg(&topology, esp).hops);
        }
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

    /// What is brought forward goes beside the header's chips where its row
    /// has room, and below the map where it has not, so the map stays where
    /// it is as it comes and goes.
    #[test]
    fn what_is_brought_forward_goes_below_the_map_without_room() {
        let (topology, streams) = bench();
        let below = |focus: Option<Focus>, width: f32| {
            let shows = NetworkShows::Audio;
            let (_, below) = header(&topology, shows, &streams, focus, width, false);
            below.is_some()
        };
        let focus = Some(streams[2].focus());
        assert!(below(focus, 360.0));
        assert!(!below(focus, 804.0));
        assert!(!below(focus, 1280.0));
        assert!(!below(None, 360.0));
    }

    /// An entity that fell back to AVB Lite hangs under the switch in front
    /// of it, its PTP domain beside any gPTP one: in sync within 50 us and
    /// out of it past that, its details saying whom it follows and why.
    #[test]
    fn avb_lite_shows_the_ptp_clock() {
        let esp = EntityId(0xe8f6_0ae0_9220_0000);
        let triib = lite_triib(-180);
        let topology = topology_of(&triib);
        let node = find(&topology, NodeKey::Entity(esp)).unwrap();
        let switch = topology.nodes[node].parent.unwrap();
        assert!(matches!(
            topology.nodes[switch].kind,
            NodeKind::Switch { domain: 0, .. }
        ));
        assert!(topology.roots.contains(&switch));
        assert!(synced(&topology, node));
        let page = clock_details(&triib, &topology, node);
        assert!(
            page.facts
                .iter()
                .any(|(label, _)| *label == fl!("lite-because"))
        );
        let page = clock_details(&triib, &topology, switch);
        assert_eq!(page.state, fl!("netmap-switch-state"));
        let triib = lite_triib(72_000);
        let topology = topology_of(&triib);
        let node = find(&topology, NodeKey::Entity(esp)).unwrap();
        assert!(!synced(&topology, node));
    }

    /// A grandmaster behind the switches in front of it sends its clock up
    /// to them, and they down to the device following it.
    #[test]
    fn the_clock_flows_out_of_a_grandmaster_behind_a_switch() {
        use atdecc::lite::{FallbackReason, LiteFlags, PtpProfile};
        let leader = ClockIdentity(0x0011_22ff_fe00_0001);
        let follower = ClockIdentity(0x0011_22ff_fe00_0002);
        let status = |offset| LiteStatus {
            interface: 0,
            flags: LiteFlags::CAPABLE
                .union(LiteFlags::ACTIVE)
                .union(LiteFlags::OFFSET_VALID),
            fallback_reason: FallbackReason::PDELAY_UNANSWERED,
            ptp_profile: PtpProfile::AVB_LITE_PTP,
            ptp_domain: 0,
            media_vlan_id: 2,
            unicast_fanout_limit: 2,
            link_speed: 100,
            committed_egress: 0,
            grandmaster: leader,
            offset_from_grandmaster: offset,
        };
        let statuses = [status(0), status(-180)];
        let reports: Vec<EntityReport<'_>> = [(1, leader), (2, follower)]
            .iter()
            .zip(&statuses)
            .map(|(&(id, clock), status)| EntityReport {
                entity_id: EntityId(id),
                name: "",
                interfaces: vec![InterfaceReport {
                    index: 0,
                    clock,
                    path: None,
                    info: None,
                    wireless: None,
                    lite: Some(status),
                    gptp: true,
                }],
            })
            .collect();
        let topology = Topology::build(&reports, None);
        let lines = lines(&topology, NetworkShows::Clock, &[], &[], None);
        let up = |clock| {
            lines
                .iter()
                .find_map(|line| {
                    let (node, up) = line.hops[0];
                    (topology.nodes[node].clock == Some(clock)).then_some(up)
                })
                .unwrap()
        };
        assert!(up(leader));
        assert!(!up(follower));
        // Brought forward, the follower lights the grandmaster's link too.
        let follower = find(&topology, NodeKey::Entity(EntityId(2)));
        let focus = follower.map(|node| Focus::Node(key_of(&topology, node)));
        let lines = super::lines(&topology, NetworkShows::Clock, &[], &[], focus);
        assert!(lines.iter().all(|line| line.related));
    }

    /// The devices sit in the same domains whether or not this computer
    /// hears a bridge running gPTP, as what it hears says nothing of them;
    /// only this computer moves.
    #[test]
    fn devices_sit_the_same_whatever_this_computer_hears() {
        let mut triib = lite_triib(-180);
        let devices = |topology: &Topology| -> Vec<(NodeKey, Option<NodeKey>)> {
            (0..topology.nodes.len())
                .filter(|&node| entity_of(topology, node).is_some())
                .map(|node| {
                    (
                        key_of(topology, node),
                        topology.nodes[node]
                            .parent
                            .map(|parent| key_of(topology, parent)),
                    )
                })
                .collect()
        };
        let alone = devices(&topology_of(&triib));
        triib.peers = vec![crate::network::Peer {
            clock: ClockIdentity(0x0001_f2ff_feff_3b14),
            port: 6,
            endpoint: false,
            synced: true,
        }];
        assert_eq!(devices(&topology_of(&triib)), alone);
    }

    /// With no entity of its own, this computer hangs under the bridge it
    /// alone hears; hearing an endpoint's peer delay requests instead, it
    /// is behind a switch taking no part in AVB, under the card for the
    /// devices running AVB Lite there, or apart where none do.
    #[test]
    fn this_computer_hangs_under_what_it_is_plugged_into() {
        let esp = EntityId(0xe8f6_0ae0_9220_0000);
        let mut triib = lite_triib(-180);
        let host = |triib: &Triib| {
            let topology = topology_of(triib);
            let host = find(&topology, NodeKey::Host).expect("this computer shows");
            let entry = &topology.nodes[host];
            (
                entry.parent.map(|parent| topology.nodes[parent].kind),
                entry.apart,
            )
        };
        assert_eq!(host(&triib), (None, Some(Apart::NoNeighbor)));
        let topology = topology_of(&triib);
        let node = find(&topology, NodeKey::Entity(esp)).unwrap();
        let clock = topology.nodes[node].clock.unwrap();
        let switch = topology.nodes[topology.nodes[node].parent.unwrap()].kind;
        // The ESP's own requests, declared or known by its clock.
        for endpoint in [true, false] {
            triib.peers = vec![crate::network::Peer {
                clock,
                port: 1,
                endpoint,
                synced: false,
            }];
            assert_eq!(host(&triib), (Some(switch), None), "{endpoint}");
        }
        // An endpoint triib does not read, with no AVB Lite device shown.
        triib.peers = vec![crate::network::Peer {
            clock: ClockIdentity(0x0011_22ff_fe00_0001),
            port: 1,
            endpoint: true,
            synced: false,
        }];
        assert_eq!(host(&triib), (None, Some(Apart::PlainSwitch)));
    }

    /// Writes a picture of the network view in AVB Lite to the PNG file the
    /// variable `NETWORK_LITE_PICTURE` names.
    #[test]
    #[ignore = "writes a picture to look at"]
    fn lite_picture() {
        let Ok(path) = std::env::var("NETWORK_LITE_PICTURE") else {
            return;
        };
        let theme = crate::view::tests::picture_theme();
        let mut triib = lite_triib(-180);
        let esp = EntityId(0xe8f6_0ae0_9220_0000);
        let device = Some(Focus::Node(NodeKey::Entity(esp)));
        let switch = Some(Focus::Node(NodeKey::Switch(
            ClockIdentity(0x9c6b_00ff_fe30_9a2b),
            0,
        )));
        for (suffix, shows, focus) in [
            ("overview", NetworkShows::Clock, None),
            ("device", NetworkShows::Clock, device),
            ("switch", NetworkShows::Clock, switch),
            ("audio", NetworkShows::Audio, None),
        ] {
            triib.settings.network_shows = shows;
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
