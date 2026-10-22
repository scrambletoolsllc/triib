//! Where things sit in the network, from what entities report: a clock
//! domain for each grandmaster, and what is in none and why. In a gPTP
//! domain the clock passes from the grandmaster down through the bridges
//! to the entities, as their gPTP paths show; a Wi-Fi station hangs under
//! its access point, its link the Wi-Fi hop, as AVB Wireless has it. In a
//! PTP domain, devices running AVB Lite follow their grandmaster end to
//! end: those that fell back to it on finding no AVB bridge hang under the
//! switches that take no part in AVB, one for each PTP grandmaster they
//! follow. A device sharing its domain with no other is a domain of its
//! own. The links a stream between two entities crosses follow from where
//! they sit.

use std::cmp::Reverse;
use std::collections::{BTreeMap, HashMap};

use atdecc::aem::AvbInfo;
use atdecc::lite::{FallbackReason, LiteFlags, LiteStatus};
use atdecc::wireless::{WirelessFlags, WirelessStatus};
use atdecc::{ClockIdentity, EntityId};

pub type NodeId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Bridge,
    Entity {
        entity_id: EntityId,
        interface: u16,
    },
    /// This computer.
    Host,
    /// A grandmaster that is no entity triib reads, known from the devices
    /// following it in AVB Lite.
    Clock,
    /// The switches that take no part in AVB between devices that fell back
    /// to AVB Lite, known from those devices: one for each PTP grandmaster
    /// and domain they follow, as the PTP messages they share mark out.
    Switch {
        grandmaster: ClockIdentity,
        domain: u8,
    },
}

/// The link from a node up to its parent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Link {
    /// Peer delay in nanoseconds, as the node reports it.
    pub delay: Option<u32>,
    /// gPTP runs on the link.
    pub synced: bool,
    /// For this computer, the bridge port it is plugged into.
    pub port: Option<u16>,
    /// In AVB Lite, the node's offset from its grandmaster in nanoseconds,
    /// as it reports it.
    pub offset: Option<i32>,
    /// A Wi-Fi hop up to the station's access point.
    pub wireless: Option<WirelessLink>,
}

/// A Wi-Fi hop, as the station at its lower end reports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WirelessLink {
    /// The station's time is locked to the access point.
    pub locked: bool,
    /// It lost the access point's time and keeps time on its own.
    pub holdover: bool,
    /// The access point's signal at the station, in dBm.
    pub signal: Option<i8>,
}

impl WirelessLink {
    fn of(status: &WirelessStatus) -> Self {
        Self {
            locked: status.flags.contains(WirelessFlags::LOCKED),
            holdover: status.flags.contains(WirelessFlags::HOLDOVER),
            signal: status.signal(),
        }
    }
}

/// Why a node is in no domain known.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Apart {
    /// It follows this grandmaster but did not report its path.
    NoPath(ClockIdentity),
    /// It reported nothing of its gPTP state.
    Unreported,
    /// This computer has heard no bridge.
    NoNeighbor,
    /// This computer cannot listen for gPTP, so where it is is unknown.
    CannotListen,
    /// This computer is behind a switch taking no part in AVB, which no
    /// device sharing it runs AVB Lite behind.
    PlainSwitch,
    /// An entity on this computer, which cannot be read from here.
    OnThisComputer,
}

/// How a device came to run AVB Lite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lite {
    /// It found no AVB bridge and fell back to it.
    FellBack,
    /// It runs AVB Lite only, taking no part in gPTP or SRP.
    Only,
    /// The operator or a controller set it to.
    Set,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Node {
    pub kind: Kind,
    pub clock: Option<ClockIdentity>,
    /// For an entity, its name.
    pub name: String,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub link: Link,
    pub apart: Option<Apart>,
    /// A bridge or entity whose port is a Wi-Fi access point.
    pub access_point: bool,
    /// The gPTP domain an entity reports, for a gPTP grandmaster its
    /// tree's; the PTP domain of one running AVB Lite.
    pub domain: Option<u8>,
    /// How an entity came to run AVB Lite, where it does.
    pub lite: Option<Lite>,
}

impl Node {
    fn new(kind: Kind, clock: Option<ClockIdentity>, name: &str) -> Self {
        Self {
            kind,
            clock,
            name: name.to_owned(),
            parent: None,
            children: Vec::new(),
            link: Link::default(),
            apart: None,
            access_point: false,
            domain: None,
            lite: None,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Hash)]
pub struct Topology {
    pub nodes: Vec<Node>,
    /// The domains, each the root of a tree: gPTP grandmasters, then the
    /// switches taking no part in AVB in front of devices that fell back
    /// to AVB Lite, and the PTP grandmasters followed without one.
    pub roots: Vec<NodeId>,
    /// What is in no domain known.
    pub apart: Vec<NodeId>,
}

/// What an entity reports, for building the topology.
pub struct EntityReport<'a> {
    pub entity_id: EntityId,
    pub name: &'a str,
    pub interfaces: Vec<InterfaceReport<'a>>,
}

pub struct InterfaceReport<'a> {
    pub index: u16,
    pub clock: ClockIdentity,
    /// From GET_AS_PATH, when it answered.
    pub path: Option<&'a [ClockIdentity]>,
    /// From GET_AVB_INFO, when it answered.
    pub info: Option<&'a AvbInfo>,
    /// From GET_WIRELESS_STATUS, for a wireless interface.
    pub wireless: Option<&'a WirelessStatus>,
    /// From GET_LITE_STATUS, when it answered.
    pub lite: Option<&'a LiteStatus>,
    /// The interface supports gPTP, as its descriptor says.
    pub gptp: bool,
}

impl<'a> InterfaceReport<'a> {
    /// What the interface reports as a Wi-Fi station.
    fn station(&self) -> Option<&WirelessStatus> {
        self.wireless.filter(|status| !status.access_point())
    }

    /// Its AVB Lite status, when it runs AVB Lite, taking no part in gPTP.
    fn running_lite(&self) -> Option<&'a LiteStatus> {
        self.lite
            .filter(|status| status.flags.contains(LiteFlags::ACTIVE))
    }
}

/// What this computer heard of what it is plugged into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostReport {
    /// The bridge's clock, its port and whether gPTP runs on the link.
    Heard(ClockIdentity, u16, bool),
    /// A switch taking no part in AVB, shared with the endpoints of these
    /// clocks; and this computer's clock, where its ptp4l runs.
    Shared(Vec<ClockIdentity>, Option<HostClock>),
    /// No bridge heard.
    Unheard,
    /// This computer cannot listen for gPTP on the interface.
    CannotListen,
}

/// This computer's clock, as its ptp4l runs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HostClock {
    pub grandmaster: ClockIdentity,
    /// Its offset from the grandmaster, in nanoseconds.
    pub offset: Option<i32>,
    /// It runs AVB Lite's PTP, end to end, rather than gPTP.
    pub ptp: bool,
}

/// One stream's way through the tree.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct Route {
    /// The links it crosses in order, each named by its lower node, and
    /// whether it goes up that link.
    pub hops: Vec<(NodeId, bool)>,
    /// Where it stops short of the listener, when it does: a bridge where
    /// its reservation failed, or the farthest place it can be traced to.
    pub stops: Option<NodeId>,
}

impl Topology {
    pub fn build(entities: &[EntityReport<'_>], host: Option<HostReport>) -> Self {
        let mut topology = Topology::default();
        // The entity interface each clock identity is, the first where
        // several share one, as this computer's endpoints share its clock.
        let mut owners: HashMap<ClockIdentity, (EntityId, u16, &str)> = HashMap::new();
        for entity in entities {
            for interface in &entity.interfaces {
                if interface.running_lite().is_none() {
                    owners.entry(interface.clock).or_insert((
                        entity.entity_id,
                        interface.index,
                        entity.name,
                    ));
                }
            }
        }
        // A node for each clock a path names, and one for each entity
        // interface, so entities sharing a clock each have their own.
        let mut by_clock: HashMap<ClockIdentity, NodeId> = HashMap::new();
        let mut by_entity: HashMap<(EntityId, u16), NodeId> = HashMap::new();
        let node_for = |topology: &mut Topology,
                        by_clock: &mut HashMap<ClockIdentity, NodeId>,
                        by_entity: &mut HashMap<(EntityId, u16), NodeId>,
                        clock: ClockIdentity|
         -> NodeId {
            if let Some(&node) = by_clock.get(&clock) {
                return node;
            }
            let owner = owners.get(&clock).copied();
            let (kind, name) = match owner {
                Some((entity_id, interface, name)) => (
                    Kind::Entity {
                        entity_id,
                        interface,
                    },
                    name,
                ),
                None => (Kind::Bridge, ""),
            };
            let node = topology.push(Node {
                link: Link {
                    synced: true,
                    ..Link::default()
                },
                ..Node::new(kind, Some(clock), name)
            });
            by_clock.insert(clock, node);
            if let Some((entity_id, interface, _)) = owner {
                by_entity.insert((entity_id, interface), node);
            }
            node
        };
        let entity_kind = |entity: &EntityReport<'_>, interface: u16| Kind::Entity {
            entity_id: entity.entity_id,
            interface,
        };

        // Every reported path, chained from its grandmaster down.
        let mut unplaced = Vec::new();
        let mut running_lite = Vec::new();
        for entity in entities {
            if entity.interfaces.is_empty() {
                // No AVB interface read: nothing reported of its gPTP state.
                let node = topology.push(Node {
                    apart: Some(Apart::Unreported),
                    ..Node::new(entity_kind(entity, 0), None, entity.name)
                });
                topology.apart.push(node);
            }
            for interface in &entity.interfaces {
                if let Some(status) = interface.running_lite() {
                    running_lite.push((entity, interface, status));
                    continue;
                }
                // A station whose path names nothing beyond itself, as
                // before it follows anyone or behind an upstream sending
                // no path, hangs under its access point instead.
                let Some(path) = interface.path.filter(|path| {
                    interface.station().is_none()
                        || path.iter().any(|&clock| clock != interface.clock)
                }) else {
                    unplaced.push((entity, interface));
                    continue;
                };
                let mut path = path.to_vec();
                // The path ends at the neighbor; some entities add themselves.
                if path.last() == Some(&interface.clock) {
                    path.pop();
                }
                let mut parent = None;
                for clock in path {
                    let node = node_for(&mut topology, &mut by_clock, &mut by_entity, clock);
                    if let Some(parent) = parent {
                        topology.attach(node, parent);
                    }
                    parent = Some(node);
                }
                let key = (entity.entity_id, interface.index);
                let node = match by_entity.get(&key) {
                    Some(&node) => node,
                    None => {
                        let node = topology.push(Node {
                            link: Link {
                                synced: true,
                                ..Link::default()
                            },
                            ..Node::new(
                                entity_kind(entity, interface.index),
                                Some(interface.clock),
                                entity.name,
                            )
                        });
                        by_entity.insert(key, node);
                        by_clock.entry(interface.clock).or_insert(node);
                        node
                    }
                };
                if let Some(parent) = parent {
                    topology.attach(node, parent);
                }
                topology.reported(node, interface);
            }
        }
        // Each path starts at a grandmaster; a path of the entity alone
        // makes it the grandmaster of a domain of its own.
        topology.roots = (0..topology.nodes.len())
            .filter(|&node| {
                topology.nodes[node].parent.is_none() && topology.nodes[node].apart.is_none()
            })
            .collect();

        // Entities that did not report a path are placed when another's
        // path names them, and set apart otherwise.
        for (entity, interface) in unplaced {
            let own = by_entity.get(&(entity.entity_id, interface.index)).copied();
            if let Some(node) = own.or_else(|| by_clock.get(&interface.clock).copied()) {
                // Beside another entity on the same clock that is placed.
                let node = match (own, topology.nodes[node].parent) {
                    (None, Some(parent)) => {
                        let beside = topology.push(Node {
                            link: topology.nodes[node].link,
                            ..Node::new(
                                entity_kind(entity, interface.index),
                                Some(interface.clock),
                                entity.name,
                            )
                        });
                        topology.attach(beside, parent);
                        beside
                    }
                    _ => node,
                };
                topology.reported(node, interface);
                continue;
            }
            // A station that reports no gPTP path, as in Mode B, hangs
            // under the access point it names.
            if let Some(&access_point) = interface
                .station()
                .and_then(|status| by_clock.get(&status.ap_clock))
            {
                let node = topology.push(Node::new(
                    entity_kind(entity, interface.index),
                    Some(interface.clock),
                    entity.name,
                ));
                by_clock.insert(interface.clock, node);
                topology.attach(node, access_point);
                topology.reported(node, interface);
                continue;
            }
            // Its own grandmaster, it is a domain of its own.
            let apart = match interface.info {
                Some(info) if info.gptp_grandmaster_id == interface.clock => None,
                Some(info) => Some(Apart::NoPath(info.gptp_grandmaster_id)),
                None => Some(Apart::Unreported),
            };
            let node = topology.push(Node {
                apart,
                ..Node::new(
                    entity_kind(entity, interface.index),
                    Some(interface.clock),
                    entity.name,
                )
            });
            topology.reported(node, interface);
            if apart.is_some() {
                topology.apart.push(node);
            } else {
                by_clock.entry(interface.clock).or_insert(node);
                topology.roots.push(node);
            }
        }

        // Interfaces running AVB Lite, in a PTP domain for each grandmaster
        // they follow. Where any of them fell back on finding no AVB
        // bridge, they all hang under the switches taking no part in AVB
        // in front of them; otherwise under their grandmaster, a node of
        // its own where it is no entity triib reads.
        let mut groups: Vec<((ClockIdentity, u8), Vec<NodeId>)> = Vec::new();
        for (entity, interface, status) in running_lite {
            // An endpoint alone names no grandmaster, or itself.
            let grandmaster = if status.grandmaster.0 == 0 {
                interface.clock
            } else {
                status.grandmaster
            };
            let offset = status.offset();
            let node = topology.push(Node {
                link: Link {
                    offset,
                    synced: grandmaster == interface.clock
                        || offset.is_some_and(|offset| {
                            offset.unsigned_abs() <= crate::lite_view::OFFSET_ALARM.unsigned_abs()
                        }),
                    ..Link::default()
                },
                domain: Some(status.ptp_domain),
                lite: Some(match status.fallback_reason {
                    _ if !interface.gptp => Lite::Only,
                    FallbackReason::NONE => Lite::Only,
                    FallbackReason::CONFIGURED => Lite::Set,
                    _ => Lite::FellBack,
                }),
                ..Node::new(
                    entity_kind(entity, interface.index),
                    Some(interface.clock),
                    entity.name,
                )
            });
            let key = (grandmaster, status.ptp_domain);
            match groups.iter_mut().find(|(other, _)| *other == key) {
                Some((_, nodes)) => nodes.push(node),
                None => groups.push((key, vec![node])),
            }
        }
        for ((grandmaster, domain), nodes) in groups {
            let fell_back = nodes
                .iter()
                .any(|&node| topology.nodes[node].lite == Some(Lite::FellBack));
            let leader = nodes
                .iter()
                .copied()
                .find(|&node| topology.nodes[node].clock == Some(grandmaster));
            let root = match leader {
                _ if fell_back => topology.push(Node::new(
                    Kind::Switch {
                        grandmaster,
                        domain,
                    },
                    None,
                    "",
                )),
                Some(leader) => leader,
                None => topology.push(Node {
                    domain: Some(domain),
                    ..Node::new(Kind::Clock, Some(grandmaster), "")
                }),
            };
            topology.roots.push(root);
            for node in nodes {
                topology.attach(node, root);
            }
        }

        // The access points the wireless interfaces name.
        for interface in entities.iter().flat_map(|entity| &entity.interfaces) {
            if let Some(&node) = interface
                .wireless
                .and_then(|status| by_clock.get(&status.ap_clock))
            {
                topology.nodes[node].access_point = true;
            }
        }

        if let Some(host) = host {
            let mut node = Node {
                apart: Some(Apart::NoNeighbor),
                ..Node::new(Kind::Host, None, "")
            };
            match host {
                HostReport::Heard(clock, port, synced) => {
                    node.apart = None;
                    node.link = Link {
                        synced,
                        port: Some(port),
                        ..Link::default()
                    };
                    let bridge = node_for(&mut topology, &mut by_clock, &mut by_entity, clock);
                    let host = topology.push(node);
                    topology.attach(host, bridge);
                    if topology.nodes[bridge].parent.is_none() && !topology.roots.contains(&bridge)
                    {
                        // Known only from this computer: a tree of its own.
                        topology.nodes[bridge].apart = None;
                        topology.apart.retain(|&apart| apart != bridge);
                        topology.roots.push(bridge);
                    }
                }
                // Under the switches in front of the devices sharing its link
                // where they run AVB Lite behind them, else apart.
                // In sync there while its ptp4l follows the grandmaster the
                // devices behind the switch do, as closely as AVB Lite asks.
                HostReport::Shared(clocks, clock) => {
                    let switch = topology.nodes.iter().find_map(|entry| {
                        entry
                            .parent
                            .filter(|_| entry.clock.is_some_and(|clock| clocks.contains(&clock)))
                            .filter(|&parent| {
                                matches!(topology.nodes[parent].kind, Kind::Switch { .. })
                            })
                    });
                    match switch {
                        Some(switch) => {
                            node.apart = None;
                            let offset = match topology.nodes[switch].kind {
                                Kind::Switch { grandmaster, .. } => clock
                                    .filter(|clock| clock.ptp && clock.grandmaster == grandmaster)
                                    .and_then(|clock| clock.offset),
                                _ => None,
                            };
                            node.link = Link {
                                offset,
                                synced: offset.is_some_and(|offset| {
                                    offset.unsigned_abs()
                                        <= crate::lite_view::OFFSET_ALARM.unsigned_abs()
                                }),
                                ..Link::default()
                            };
                            let host = topology.push(node);
                            topology.attach(host, switch);
                        }
                        None => {
                            node.apart = Some(Apart::PlainSwitch);
                            let host = topology.push(node);
                            topology.apart.push(host);
                        }
                    }
                }
                HostReport::Unheard | HostReport::CannotListen => {
                    if host == HostReport::CannotListen {
                        node.apart = Some(Apart::CannotListen);
                    }
                    let host = topology.push(node);
                    topology.apart.push(host);
                }
            }
        }

        topology.domains();
        topology.sort();
        topology
    }

    /// Takes in what an entity interface's node reports: its link's delay,
    /// its gPTP domain and its Wi-Fi hop.
    fn reported(&mut self, node: NodeId, interface: &InterfaceReport<'_>) {
        self.nodes[node].link.delay = interface.info.map(|info| info.propagation_delay);
        self.nodes[node].domain = interface.info.map(|info| info.gptp_domain_number);
        self.wireless_hop(node, interface);
    }

    /// Gives each gPTP tree the domain most of its entities report.
    fn domains(&mut self) {
        for root in self.roots.clone() {
            if self.is_ptp(root) {
                continue;
            }
            let tree = (0..self.nodes.len()).filter(|&node| self.is_below(node, root));
            if let Some(domain) = most(tree.filter_map(|node| self.nodes[node].domain)) {
                self.nodes[root].domain = Some(domain);
            }
        }
    }

    /// Whether the node heads or is in a PTP domain, its clock AVB Lite's
    /// rather than gPTP's.
    pub fn is_ptp(&self, node: NodeId) -> bool {
        let entry = &self.nodes[node];
        matches!(entry.kind, Kind::Switch { .. } | Kind::Clock) || entry.lite.is_some()
    }

    /// Makes the link up from a station's node its Wi-Fi hop, in sync while
    /// the station's time is locked.
    fn wireless_hop(&mut self, node: NodeId, interface: &InterfaceReport<'_>) {
        if let Some(status) = interface.station() {
            let link = &mut self.nodes[node].link;
            link.wireless = Some(WirelessLink::of(status));
            link.synced = status.flags.contains(WirelessFlags::LOCKED);
            // A Wi-Fi hop's delay of 0 is one not measured, as where FTM
            // feeds the clock rather than the link delay.
            link.delay = link.delay.filter(|&delay| delay > 0);
        }
    }

    fn push(&mut self, node: Node) -> NodeId {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    /// Hangs `child` under `parent`, unless it hangs elsewhere already or
    /// that would make a loop, as conflicting paths could.
    fn attach(&mut self, child: NodeId, parent: NodeId) {
        if child == parent || self.nodes[child].parent.is_some() || self.is_below(parent, child) {
            return;
        }
        self.nodes[child].parent = Some(parent);
        self.nodes[parent].children.push(child);
    }

    /// Whether `node` is `ancestor` or below it.
    fn is_below(&self, node: NodeId, ancestor: NodeId) -> bool {
        let mut at = Some(node);
        while let Some(current) = at {
            if current == ancestor {
                return true;
            }
            at = self.nodes[current].parent;
        }
        false
    }

    /// Orders children: entities by name, then this computer, then bridges;
    /// and the trees by their grandmasters, then the switches that take no
    /// part in AVB.
    fn sort(&mut self) {
        let key = |node: &Node| {
            let rank = match node.kind {
                Kind::Entity { .. } => 0,
                Kind::Host => 1,
                Kind::Bridge | Kind::Clock | Kind::Switch { .. } => 2,
            };
            (rank, node.name.to_lowercase(), node.clock)
        };
        let keys: Vec<_> = self.nodes.iter().map(key).collect();
        for node in &mut self.nodes {
            node.children.sort_by_key(|&child| keys[child].clone());
        }
        // The gPTP domains, then the PTP ones, each the larger first.
        let sizes: Vec<usize> = self
            .roots
            .iter()
            .map(|&root| {
                (0..self.nodes.len())
                    .filter(|&node| self.is_below(node, root))
                    .count()
            })
            .collect();
        let mut roots: Vec<(NodeId, usize)> = self.roots.iter().copied().zip(sizes).collect();
        roots.sort_by_key(|&(root, size)| {
            let entry = &self.nodes[root];
            let grandmaster = match entry.kind {
                Kind::Switch { grandmaster, .. } => Some(grandmaster),
                _ => entry.clock,
            };
            (self.is_ptp(root), Reverse(size), entry.domain, grandmaster)
        });
        self.roots = roots.into_iter().map(|(root, _)| root).collect();
        self.apart.sort_by_key(|&node| keys[node].clone());
    }

    /// Whether the node is an entity running AVB Lite, its clock PTP's.
    pub fn runs_lite(&self, node: NodeId) -> bool {
        let entry = &self.nodes[node];
        matches!(entry.kind, Kind::Entity { .. }) && entry.lite.is_some()
    }

    /// The first of the entity's interfaces that is on a tree.
    pub fn placed(&self, entity_id: EntityId) -> Option<NodeId> {
        self.nodes.iter().position(|node| {
            matches!(node.kind, Kind::Entity { entity_id: id, .. } if id == entity_id)
                && node.apart.is_none()
        })
    }

    /// The node heading the node's domain.
    pub fn root_of(&self, node: NodeId) -> NodeId {
        self.upward(node).last().copied().unwrap_or(node)
    }

    /// The node and its ancestors, up to its root.
    fn upward(&self, node: NodeId) -> Vec<NodeId> {
        let mut path = vec![node];
        while let Some(parent) = self.nodes[*path.last().unwrap_or(&node)].parent {
            path.push(parent);
        }
        path
    }

    /// The links a stream from `talker` to `listener` crosses: up from the
    /// talker to where their paths meet, then down to the listener. When
    /// only one of them is on a tree, the stream is traced to that one's
    /// bridge.
    pub fn route(&self, talker: EntityId, listener: EntityId) -> Route {
        match (self.placed(talker), self.placed(listener)) {
            (Some(talker), Some(listener)) => {
                let up = self.upward(talker);
                let down = self.upward(listener);
                let Some(meet) = up.iter().position(|node| down.contains(node)) else {
                    // On different trees: as far as the talker's root.
                    let root = *up.last().unwrap_or(&talker);
                    return Route {
                        hops: up[..up.len() - 1]
                            .iter()
                            .map(|&node| (node, true))
                            .collect(),
                        stops: Some(root),
                    };
                };
                let shared = up[meet];
                let below = down.iter().position(|&node| node == shared).unwrap_or(0);
                let mut hops: Vec<(NodeId, bool)> =
                    up[..meet].iter().map(|&node| (node, true)).collect();
                hops.extend(down[..below].iter().rev().map(|&node| (node, false)));
                Route { hops, stops: None }
            }
            (Some(talker), None) => Route {
                hops: vec![(talker, true)],
                stops: self.nodes[talker].parent,
            }
            .filter_unrooted(),
            (None, Some(listener)) => Route {
                hops: vec![(listener, false)],
                stops: self.nodes[listener].parent,
            }
            .filter_unrooted(),
            (None, None) => Route::default(),
        }
    }

    /// The route as far as `bridge`, where its reservation failed.
    pub fn cut(&self, route: &Route, bridge: NodeId) -> Route {
        let mut hops = Vec::new();
        for &(node, up) in &route.hops {
            let (from, to) = if up {
                (Some(node), self.nodes[node].parent)
            } else {
                (self.nodes[node].parent, Some(node))
            };
            if from == Some(bridge) {
                break;
            }
            hops.push((node, up));
            if to == Some(bridge) {
                break;
            }
        }
        Route {
            hops,
            stops: Some(bridge),
        }
    }

    /// The bridge with this MAC address, as a bridge ID ends with, matched
    /// against the address its clock identity was made from.
    pub fn bridge_with_mac(&self, mac: [u8; 6]) -> Option<NodeId> {
        self.nodes
            .iter()
            .position(|node| node.kind == Kind::Bridge && node.clock.and_then(mac_of) == Some(mac))
    }
}

impl Route {
    /// A route that would only reach a node with no parent is none.
    fn filter_unrooted(self) -> Self {
        if self.stops.is_some() {
            self
        } else {
            Route::default()
        }
    }
}

/// The value given most often, the least of those tied.
fn most(values: impl Iterator<Item = u8>) -> Option<u8> {
    let mut counts: BTreeMap<u8, usize> = BTreeMap::new();
    for value in values {
        *counts.entry(value).or_default() += 1;
    }
    counts
        .into_iter()
        .max_by_key(|&(value, count)| (count, Reverse(value)))
        .map(|(value, _)| value)
}

/// The MAC address a clock identity was made from, when it was made the
/// usual way: the address with FF-FE in its middle.
pub fn mac_of(clock: ClockIdentity) -> Option<[u8; 6]> {
    let [a, b, c, d, e, f, g, h] = clock.0.to_be_bytes();
    (d == 0xff && e == 0xfe).then_some([a, b, c, f, g, h])
}

/// A station's status for tests: following the access point whose port
/// has `ap_clock`, its time as `flags` say.
#[cfg(test)]
pub(crate) fn test_station(ap_clock: ClockIdentity, flags: WirelessFlags) -> WirelessStatus {
    use atdecc::wireless::{AsCapableReason, Band, TimeMode};
    WirelessStatus {
        interface: 0,
        flags,
        time_mode: TimeMode::MODE_A_FTM,
        band: Band::GHZ_5,
        channel: 36,
        channel_width: 80,
        phy_generation: 6,
        rssi: -52,
        phy_rate: 866,
        ftm_success: 98,
        as_capable_reason: AsCapableReason::NONE,
        ftm_burst_frames: 3,
        listeners_unserved: 0,
        ftm_burst_duration: 7,
        ftm_min_delta: 100,
        ftm_rtt: 42,
        servo_error: -380,
        association_age: 60,
        time_age: 50,
        ap_resets: 0,
        bssid: atdecc::MacAddress([0x30, 0xed, 0xa0, 0x11, 0x22, 0x33]),
        ap_clock,
        ap_port: 2,
        downlink_readdressed: 0,
        downlink_unmapped: 0,
        downlink_dropped: 0,
        uplink_restored: 0,
        frame_budget: 0,
        frames_committed: 0,
        station_count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SWITCH: ClockIdentity = ClockIdentity(0x0001_f2ff_feff_3b14);
    const MAC_CLOCK: ClockIdentity = ClockIdentity(0xeab6_9c6d_c98c_0002);
    const WIRED_CLOCK: ClockIdentity = ClockIdentity(0xe8f6_0aff_fee0_9220);
    const WIFI_CLOCK: ClockIdentity = ClockIdentity(0xfc01_2cff_fefd_fe80);
    const MAC: EntityId = EntityId(0xd111_e597_f544_8000);
    const WIRED: EntityId = EntityId(0xe8f6_0ae0_9220_0000);
    const WIFI: EntityId = EntityId(0xfc01_2cfd_fe80_0000);

    fn info(grandmaster: ClockIdentity, delay: u32) -> AvbInfo {
        let mut payload = [0u8; 20];
        payload[0..2].copy_from_slice(&0x0009u16.to_be_bytes());
        payload[4..12].copy_from_slice(&grandmaster.0.to_be_bytes());
        payload[12..16].copy_from_slice(&delay.to_be_bytes());
        AvbInfo::decode(&payload).unwrap()
    }

    fn kind_of(topology: &Topology, node: NodeId) -> Kind {
        topology.nodes[node].kind
    }

    /// The bench: both wired entities under the switch, which is the
    /// grandmaster, and the Wi-Fi one its own grandmaster.
    fn bench(host: Option<HostReport>) -> Topology {
        let mac_path = [SWITCH, MAC_CLOCK];
        let wired_path = [SWITCH, WIRED_CLOCK];
        let (mac_info, wired_info, wifi_info) =
            (info(SWITCH, 58), info(SWITCH, 432), info(WIFI_CLOCK, 0));
        let entities = [
            EntityReport {
                entity_id: MAC,
                name: "Mac mini",
                interfaces: vec![InterfaceReport {
                    index: 0,
                    clock: MAC_CLOCK,
                    path: Some(&mac_path),
                    info: Some(&mac_info),
                    wireless: None,
                    lite: None,
                    gptp: true,
                }],
            },
            EntityReport {
                entity_id: WIRED,
                name: "AVB Example Entity",
                interfaces: vec![InterfaceReport {
                    index: 0,
                    clock: WIRED_CLOCK,
                    path: Some(&wired_path),
                    info: Some(&wired_info),
                    wireless: None,
                    lite: None,
                    gptp: true,
                }],
            },
            EntityReport {
                entity_id: WIFI,
                name: "AVB Example Entity",
                interfaces: vec![InterfaceReport {
                    index: 0,
                    clock: WIFI_CLOCK,
                    path: None,
                    info: Some(&wifi_info),
                    wireless: None,
                    lite: None,
                    gptp: true,
                }],
            },
        ];
        Topology::build(&entities, host)
    }

    /// The switch heads the bench's tree, and the Wi-Fi entity, its own
    /// grandmaster, is a domain of its own.
    #[test]
    fn the_bench() {
        let topology = bench(Some(HostReport::Heard(SWITCH, 6, false)));
        assert_eq!(topology.roots.len(), 2);
        let root = topology.roots[0];
        assert_eq!(kind_of(&topology, root), Kind::Bridge);
        assert_eq!(topology.nodes[root].clock, Some(SWITCH));
        let children: Vec<Kind> = topology.nodes[root]
            .children
            .iter()
            .map(|&child| kind_of(&topology, child))
            .collect();
        assert_eq!(
            children,
            [
                Kind::Entity {
                    entity_id: WIRED,
                    interface: 0
                },
                Kind::Entity {
                    entity_id: MAC,
                    interface: 0
                },
                Kind::Host,
            ]
        );
        let mac = topology.placed(MAC).unwrap();
        assert_eq!(topology.nodes[mac].link.delay, Some(58));
        assert!(topology.nodes[mac].link.synced);
        let host = topology.nodes[root].children[2];
        assert_eq!(topology.nodes[host].link.port, Some(6));
        assert!(!topology.nodes[host].link.synced);
        assert!(topology.apart.is_empty());
        let wifi = topology.placed(WIFI).unwrap();
        assert_eq!(topology.roots[1], wifi);
        assert!(topology.nodes[wifi].children.is_empty());
        assert!(!topology.is_ptp(wifi));
    }

    /// A station that reports its path hangs under its access point, and
    /// one that reports none, as in Mode B, hangs under the access point it
    /// names; each link is a Wi-Fi hop, in sync while its time is locked.
    #[test]
    fn stations_hang_under_their_access_point() {
        const ACCESS_POINT: ClockIdentity = ClockIdentity(0x30ed_a0ff_fe11_2233);
        const BEACONS: EntityId = EntityId(0xfc01_2cfd_fe80_0001);
        const BEACONS_CLOCK: ClockIdentity = ClockIdentity(0xfc01_2cff_fefd_fe81);
        let path = [SWITCH, ACCESS_POINT, WIFI_CLOCK];
        let locked = test_station(ACCESS_POINT, WirelessFlags::LOCKED);
        let holding = test_station(ACCESS_POINT, WirelessFlags::HOLDOVER);
        let info = info(SWITCH, 12_000);
        let entities = [
            EntityReport {
                entity_id: WIFI,
                name: "Wi-Fi talker",
                interfaces: vec![InterfaceReport {
                    index: 0,
                    clock: WIFI_CLOCK,
                    path: Some(&path),
                    info: Some(&info),
                    wireless: Some(&locked),
                    lite: None,
                    gptp: true,
                }],
            },
            EntityReport {
                entity_id: BEACONS,
                name: "Wi-Fi listener",
                interfaces: vec![InterfaceReport {
                    index: 0,
                    clock: BEACONS_CLOCK,
                    path: None,
                    info: Some(&info),
                    wireless: Some(&holding),
                    lite: None,
                    gptp: true,
                }],
            },
        ];
        let topology = Topology::build(&entities, None);
        let talker = topology.placed(WIFI).unwrap();
        let listener = topology.placed(BEACONS).unwrap();
        let access_point = topology.nodes[talker].parent.unwrap();
        assert_eq!(topology.nodes[listener].parent, Some(access_point));
        assert_eq!(topology.nodes[access_point].clock, Some(ACCESS_POINT));
        assert!(topology.nodes[access_point].access_point);
        assert!(!topology.nodes[talker].access_point);
        let hop = |node: NodeId| topology.nodes[node].link;
        assert!(hop(talker).synced);
        assert_eq!(
            hop(talker).wireless,
            Some(WirelessLink {
                locked: true,
                holdover: false,
                signal: Some(-52),
            })
        );
        assert!(!hop(listener).synced);
        assert!(hop(listener).wireless.is_some_and(|hop| hop.holdover));
        assert_eq!(hop(listener).delay, Some(12_000));
        // A station's path ends with itself, as an ESP reports it; one
        // naming nothing beyond itself, or nothing at all, is no path, and
        // a delay of 0 is one not measured.
        let own = [SWITCH, ACCESS_POINT, WIFI_CLOCK];
        let alone = [BEACONS_CLOCK];
        let unmeasured = super::tests::info(SWITCH, 0);
        for listener_path in [&alone[..], &[][..]] {
            let entities = [
                EntityReport {
                    entity_id: WIFI,
                    name: "Wi-Fi talker",
                    interfaces: vec![InterfaceReport {
                        index: 0,
                        clock: WIFI_CLOCK,
                        path: Some(&own),
                        info: Some(&unmeasured),
                        wireless: Some(&locked),
                        lite: None,
                        gptp: true,
                    }],
                },
                EntityReport {
                    entity_id: BEACONS,
                    name: "Wi-Fi listener",
                    interfaces: vec![InterfaceReport {
                        index: 0,
                        clock: BEACONS_CLOCK,
                        path: Some(listener_path),
                        info: Some(&info),
                        wireless: Some(&holding),
                        lite: None,
                        gptp: true,
                    }],
                },
            ];
            let topology = Topology::build(&entities, None);
            let talker = topology.placed(WIFI).unwrap();
            let listener = topology.placed(BEACONS).unwrap();
            let access_point = topology.nodes[talker].parent.unwrap();
            assert_eq!(topology.nodes[access_point].clock, Some(ACCESS_POINT));
            assert_eq!(topology.nodes[listener].parent, Some(access_point));
            assert_eq!(topology.nodes[talker].link.delay, None);
            assert!(topology.apart.is_empty());
        }
        assert!(topology.apart.is_empty());
        // A stream between them crosses both hops.
        assert_eq!(
            topology.route(WIFI, BEACONS).hops,
            [(talker, true), (listener, false)]
        );
    }

    /// Entities on one gPTP port, as this computer's talker and listener
    /// share its clock, each have their own node, whether or not they
    /// report a path.
    #[test]
    fn entities_sharing_a_clock_each_show() {
        const HOST_CLOCK: ClockIdentity = ClockIdentity(0xf0a7_31ff_fef4_0f14);
        const TALKER: EntityId = EntityId(0xf0a7_31ff_00f4_0f14);
        const LISTENER: EntityId = EntityId(0xf0a7_31ff_01f4_0f14);
        const QUIET: EntityId = EntityId(0xf0a7_31ff_02f4_0f14);
        let path = [SWITCH, HOST_CLOCK];
        let info = info(SWITCH, 327);
        let report = |entity_id, name, path| EntityReport {
            entity_id,
            name,
            interfaces: vec![InterfaceReport {
                index: 0,
                clock: HOST_CLOCK,
                path,
                info: Some(&info),
                wireless: None,
                lite: None,
                gptp: true,
            }],
        };
        let entities = [
            report(TALKER, "Host talker 1", Some(&path[..])),
            report(LISTENER, "Host listener 1", Some(&path[..])),
            report(QUIET, "Host listener 2", None),
        ];
        let topology = Topology::build(&entities, None);
        let nodes: Vec<NodeId> = [TALKER, LISTENER, QUIET]
            .iter()
            .map(|&entity| topology.placed(entity).unwrap())
            .collect();
        assert_eq!(
            nodes.iter().collect::<std::collections::HashSet<_>>().len(),
            3
        );
        let root = topology.roots[0];
        assert_eq!(topology.nodes[root].clock, Some(SWITCH));
        for node in nodes {
            assert_eq!(topology.nodes[node].parent, Some(root));
            assert_eq!(topology.nodes[node].link.delay, Some(327));
        }
        assert!(topology.apart.is_empty());
    }

    #[test]
    fn this_computer_that_cannot_listen_says_so() {
        let topology = bench(Some(HostReport::CannotListen));
        let host = topology
            .nodes
            .iter()
            .find(|node| node.kind == Kind::Host)
            .unwrap();
        assert_eq!(host.apart, Some(Apart::CannotListen));
        assert_eq!(host.parent, None);
    }

    #[test]
    fn this_computer_without_a_bridge_is_apart() {
        let topology = bench(Some(HostReport::Unheard));
        assert!(
            topology
                .apart
                .iter()
                .any(|&node| topology.nodes[node].apart == Some(Apart::NoNeighbor))
        );
        assert_eq!(
            bench(None)
                .nodes
                .iter()
                .filter(|node| node.kind == Kind::Host)
                .count(),
            0
        );
    }

    #[test]
    fn streams_go_up_to_where_paths_meet_and_down() {
        let topology = bench(None);
        let wired = topology.placed(WIRED).unwrap();
        let mac = topology.placed(MAC).unwrap();
        let route = topology.route(WIRED, MAC);
        assert_eq!(route.hops, [(wired, true), (mac, false)]);
        assert_eq!(route.stops, None);
        // A listener on no tree: traced to the talker's bridge.
        let route = topology.route(WIRED, WIFI);
        assert_eq!(route.hops, [(wired, true)]);
        assert_eq!(route.stops, topology.nodes[wired].parent);
        // Neither on a tree: nothing to draw.
        assert_eq!(topology.route(WIFI, WIFI), Route::default());
    }

    #[test]
    fn deeper_trees_and_failed_reservations() {
        // Grandmaster bridge, a stage bridge below it, and an entity under
        // each.
        let main = ClockIdentity(0x00aa_bbff_fe00_0001);
        let stage = ClockIdentity(0x00aa_bbff_fe00_0002);
        let console_clock = ClockIdentity(0x0011_22ff_fe00_0001);
        let stage_box_clock = ClockIdentity(0x0011_22ff_fe00_0002);
        let console_path = [main, console_clock];
        let stage_box_path = [main, stage];
        let entities = [
            EntityReport {
                entity_id: EntityId(1),
                name: "Console",
                interfaces: vec![InterfaceReport {
                    index: 0,
                    clock: console_clock,
                    path: Some(&console_path),
                    info: None,
                    wireless: None,
                    lite: None,
                    gptp: true,
                }],
            },
            EntityReport {
                entity_id: EntityId(2),
                name: "Stage box",
                interfaces: vec![InterfaceReport {
                    index: 0,
                    clock: stage_box_clock,
                    path: Some(&stage_box_path),
                    info: None,
                    wireless: None,
                    lite: None,
                    gptp: true,
                }],
            },
        ];
        let topology = Topology::build(&entities, None);
        assert_eq!(topology.roots.len(), 1);
        let root = topology.roots[0];
        let stage_node = topology
            .nodes
            .iter()
            .position(|node| node.clock == Some(stage))
            .unwrap();
        assert_eq!(topology.nodes[stage_node].parent, Some(root));
        let console = topology.placed(EntityId(1)).unwrap();
        let stage_box = topology.placed(EntityId(2)).unwrap();
        let route = topology.route(EntityId(2), EntityId(1));
        assert_eq!(
            route.hops,
            [(stage_box, true), (stage_node, true), (console, false)]
        );
        // The stage bridge refused the reservation.
        let bridge = topology
            .bridge_with_mac([0x00, 0xaa, 0xbb, 0x00, 0x00, 0x02])
            .unwrap();
        assert_eq!(bridge, stage_node);
        let cut = topology.cut(&route, bridge);
        assert_eq!(cut.hops, [(stage_box, true)]);
        assert_eq!(cut.stops, Some(stage_node));
    }

    #[test]
    fn clock_identities_name_their_mac() {
        assert_eq!(mac_of(SWITCH), Some([0x00, 0x01, 0xf2, 0xff, 0x3b, 0x14]));
        assert_eq!(mac_of(MAC_CLOCK), None);
    }

    fn lite(flags: LiteFlags, grandmaster: ClockIdentity, offset: Option<i32>) -> LiteStatus {
        use atdecc::lite::{FallbackReason, PtpProfile};
        LiteStatus {
            interface: 0,
            flags: if offset.is_some() {
                flags.union(LiteFlags::OFFSET_VALID)
            } else {
                flags
            },
            fallback_reason: FallbackReason::PDELAY_UNANSWERED,
            ptp_profile: PtpProfile::AVB_LITE_PTP,
            ptp_domain: 0,
            media_vlan_id: 2,
            unicast_fanout_limit: 2,
            link_speed: 100,
            committed_egress: 0,
            grandmaster,
            offset_from_grandmaster: offset.unwrap_or(0),
        }
    }

    /// What an entity running AVB Lite reports, with no gPTP path.
    fn lite_report<'a>(
        entity_id: EntityId,
        name: &'a str,
        clock: ClockIdentity,
        status: &'a LiteStatus,
    ) -> EntityReport<'a> {
        EntityReport {
            entity_id,
            name,
            interfaces: vec![InterfaceReport {
                index: 0,
                clock,
                path: None,
                info: None,
                wireless: None,
                lite: Some(status),
                gptp: true,
            }],
        }
    }

    /// Devices set to AVB Lite by hand, the wired ESP their grandmaster,
    /// are a PTP domain with no switch shown, as none of them found one:
    /// the Wi-Fi one follows closely and this computer's entity too far off,
    /// each straight under the grandmaster. The Mac, which runs no AVB
    /// Lite and reports nothing of gPTP, is in no domain known.
    #[test]
    fn avb_lite_flows_end_to_end_from_the_grandmaster() {
        use atdecc::lite::FallbackReason;
        const HOST_CLOCK: ClockIdentity = ClockIdentity(0x9c6b_00ff_fe30_9a2b);
        const HOST_TALKER: EntityId = EntityId(0x9c6b_00ff_0030_9a2b);
        let set = |offset| running(FallbackReason::CONFIGURED, WIRED_CLOCK, offset);
        let (wired, wifi, talker) = (set(None), set(Some(-180)), set(Some(72_000)));
        let mac = lite(LiteFlags::CAPABLE, ClockIdentity(0), None);
        let entities = [
            lite_report(WIRED, "Synthia", WIRED_CLOCK, &wired),
            lite_report(WIFI, "AVB Wireless Entity", WIFI_CLOCK, &wifi),
            lite_report(HOST_TALKER, "Host talker 1", HOST_CLOCK, &talker),
            lite_report(MAC, "Mac mini", MAC_CLOCK, &mac),
        ];
        let topology = Topology::build(&entities, None);
        assert!(
            topology
                .nodes
                .iter()
                .all(|node| !matches!(node.kind, Kind::Bridge | Kind::Switch { .. }))
        );
        assert_eq!(topology.roots.len(), 1);
        let root = topology.roots[0];
        assert_eq!(topology.nodes[root].clock, Some(WIRED_CLOCK));
        assert!(topology.is_ptp(root));
        assert_eq!(topology.nodes[root].domain, Some(0));
        assert_eq!(topology.nodes[root].children.len(), 2);
        let node = |entity_id| &topology.nodes[topology.placed(entity_id).unwrap()];
        let wifi = node(WIFI);
        assert_eq!(wifi.parent, Some(root));
        assert_eq!(wifi.lite, Some(Lite::Set));
        assert!(wifi.link.synced);
        assert_eq!(wifi.link.offset, Some(-180));
        // 72 us is past the 50 us AVB Lite allows. This computer shows as
        // its entity alone.
        let talker = node(HOST_TALKER);
        assert_eq!(talker.parent, Some(root));
        assert!(!talker.link.synced);
        assert!(topology.nodes.iter().all(|node| node.kind != Kind::Host));
        assert_eq!(topology.placed(MAC), None);
        assert_eq!(topology.apart.len(), 1);
        assert_eq!(
            topology.nodes[topology.apart[0]].apart,
            Some(Apart::Unreported)
        );
    }

    /// An AVB Lite status for an interface running it: fallen back for
    /// `reason`, following `grandmaster` `offset` nanoseconds off.
    fn running(
        reason: atdecc::lite::FallbackReason,
        grandmaster: ClockIdentity,
        offset: Option<i32>,
    ) -> LiteStatus {
        LiteStatus {
            fallback_reason: reason,
            ..lite(
                LiteFlags::CAPABLE.union(LiteFlags::ACTIVE),
                grandmaster,
                offset,
            )
        }
    }

    /// Devices that fell back to AVB Lite on finding no AVB bridge hang
    /// under a switch for each PTP grandmaster they follow, beside the gPTP
    /// tree, a stream between two of them crossing it; those running AVB
    /// Lite only, or set to it, following the same grandmaster hang there
    /// too, and one only capable of it stays on the tree.
    #[test]
    fn devices_behind_a_plain_switch_hang_under_it() {
        use atdecc::lite::FallbackReason;
        let clock = |place: u64| ClockIdentity(0x0011_22ff_fe00_0000 | place);
        let entity = |place: u64| EntityId(0x0011_2200_0000_0000 | place);
        let elsewhere = ClockIdentity(0x0066_77ff_fe00_0001);
        let statuses = [
            running(FallbackReason::PDELAY_UNANSWERED, clock(1), None),
            running(FallbackReason::MULTIPLE_RESPONDERS, clock(1), Some(-180)),
            running(FallbackReason::ENDPOINT_TLV, elsewhere, Some(72_000)),
            running(FallbackReason::PDELAY_UNANSWERED, clock(1), Some(40)),
            running(FallbackReason::NONE, clock(1), Some(40)),
            running(FallbackReason::CONFIGURED, clock(1), Some(40)),
            lite(LiteFlags::CAPABLE, ClockIdentity(0), None),
        ];
        let names = [
            "Leader",
            "Follower",
            "Far",
            "No gPTP",
            "Never fell",
            "Set",
            "Capable",
        ];
        let capable_path = [SWITCH, clock(7)];
        let wired_path = [SWITCH, WIRED_CLOCK];
        let wired_info = info(SWITCH, 432);
        let mut entities: Vec<EntityReport<'_>> = (1..=7u64)
            .map(|place| EntityReport {
                entity_id: entity(place),
                name: names[place as usize - 1],
                interfaces: vec![InterfaceReport {
                    index: 0,
                    clock: clock(place),
                    path: (place == 7).then_some(&capable_path[..]),
                    info: None,
                    wireless: None,
                    lite: Some(&statuses[place as usize - 1]),
                    gptp: place != 4,
                }],
            })
            .collect();
        entities.push(EntityReport {
            entity_id: WIRED,
            name: "AVB Example Entity",
            interfaces: vec![InterfaceReport {
                index: 0,
                clock: WIRED_CLOCK,
                path: Some(&wired_path),
                info: Some(&wired_info),
                wireless: None,
                lite: None,
                gptp: true,
            }],
        });
        let topology = Topology::build(&entities, Some(HostReport::Heard(SWITCH, 6, true)));
        let kinds: Vec<Kind> = topology
            .roots
            .iter()
            .map(|&root| kind_of(&topology, root))
            .collect();
        assert_eq!(
            kinds,
            [
                Kind::Bridge,
                Kind::Switch {
                    grandmaster: clock(1),
                    domain: 0
                },
                Kind::Switch {
                    grandmaster: elsewhere,
                    domain: 0
                },
            ]
        );
        let node = |place| topology.placed(entity(place));
        let (leader, follower, far) = (node(1).unwrap(), node(2).unwrap(), node(3).unwrap());
        let switch = topology.roots[1];
        assert_eq!(topology.nodes[leader].parent, Some(switch));
        assert_eq!(topology.nodes[follower].parent, Some(switch));
        assert_eq!(topology.nodes[far].parent, Some(topology.roots[2]));
        assert!(topology.runs_lite(leader) && topology.runs_lite(far));
        // The grandmaster leads, so is in sync; 72 us is past what AVB Lite
        // allows.
        assert!(topology.nodes[leader].link.synced);
        assert!(topology.nodes[follower].link.synced);
        assert_eq!(topology.nodes[follower].link.offset, Some(-180));
        assert!(!topology.nodes[far].link.synced);
        assert_eq!(
            topology.route(entity(1), entity(2)).hops,
            [(leader, true), (follower, false)]
        );
        for (place, how) in [
            (1, Lite::FellBack),
            (4, Lite::Only),
            (5, Lite::Only),
            (6, Lite::Set),
        ] {
            let node = node(place).unwrap();
            assert_eq!(topology.nodes[node].parent, Some(switch));
            assert_eq!(topology.nodes[node].lite, Some(how));
        }
        assert!(topology.apart.is_empty());
        let capable = node(7).unwrap();
        assert_eq!(
            topology.nodes[topology.nodes[capable].parent.unwrap()].clock,
            Some(SWITCH)
        );
        assert!(!topology.runs_lite(capable));
    }

    /// Each gPTP domain is a tree of its own, labelled with its number: one
    /// device alone on another domain heads one, and so does one alone on
    /// the same domain as the switch's tree, the larger tree first.
    #[test]
    fn devices_alone_head_domains_of_their_own() {
        let on_domain = |grandmaster: ClockIdentity, domain: u8| {
            let mut payload = [0u8; 20];
            payload[4..12].copy_from_slice(&grandmaster.0.to_be_bytes());
            payload[16] = domain;
            AvbInfo::decode(&payload).unwrap()
        };
        const LOST: EntityId = EntityId(0x0011_2200_0000_0009);
        const LOST_CLOCK: ClockIdentity = ClockIdentity(0x0011_22ff_fe00_0009);
        let mac_path = [SWITCH, MAC_CLOCK];
        let (mac_info, wifi_info, lost_info) = (
            on_domain(SWITCH, 0),
            on_domain(WIFI_CLOCK, 1),
            on_domain(LOST_CLOCK, 0),
        );
        let report = |entity_id, name, clock, path, info| EntityReport {
            entity_id,
            name,
            interfaces: vec![InterfaceReport {
                index: 0,
                clock,
                path,
                info: Some(info),
                wireless: None,
                lite: None,
                gptp: true,
            }],
        };
        let topology = Topology::build(
            &[
                report(MAC, "Mac mini", MAC_CLOCK, Some(&mac_path[..]), &mac_info),
                report(WIFI, "Domain one", WIFI_CLOCK, None, &wifi_info),
                report(LOST, "Lost", LOST_CLOCK, None, &lost_info),
            ],
            Some(HostReport::Heard(SWITCH, 6, true)),
        );
        assert!(topology.apart.is_empty());
        let heads: Vec<(Option<ClockIdentity>, Option<u8>)> = topology
            .roots
            .iter()
            .map(|&root| (topology.nodes[root].clock, topology.nodes[root].domain))
            .collect();
        assert_eq!(
            heads,
            [
                (Some(SWITCH), Some(0)),
                (Some(LOST_CLOCK), Some(0)),
                (Some(WIFI_CLOCK), Some(1)),
            ]
        );
    }

    /// Behind a switch taking no part in AVB, this computer hangs under the
    /// card for the devices running AVB Lite there, in sync while its own
    /// ptp4l follows their grandmaster within 50 us.
    #[test]
    fn this_computer_behind_a_plain_switch_follows_its_grandmaster() {
        use atdecc::lite::FallbackReason;
        let wired = running(FallbackReason::ENDPOINT_TLV, WIRED_CLOCK, None);
        let entities = [lite_report(WIRED, "Synthia", WIRED_CLOCK, &wired)];
        let host = |clock| {
            let topology = Topology::build(
                &entities,
                Some(HostReport::Shared(vec![WIRED_CLOCK], clock)),
            );
            let host = topology
                .nodes
                .iter()
                .position(|node| node.kind == Kind::Host)
                .unwrap();
            let parent = topology.nodes[host].parent.unwrap();
            assert!(matches!(topology.nodes[parent].kind, Kind::Switch { .. }));
            topology.nodes[host].link
        };
        let following = |grandmaster, offset| {
            Some(HostClock {
                grandmaster,
                offset: Some(offset),
                ptp: true,
            })
        };
        let near = host(following(WIRED_CLOCK, -120));
        assert!(near.synced);
        assert_eq!(near.offset, Some(-120));
        assert!(!host(following(WIRED_CLOCK, 72_000)).synced);
        let elsewhere = host(following(SWITCH, -120));
        assert!(!elsewhere.synced);
        assert_eq!(elsewhere.offset, None);
        assert!(!host(None).synced);
    }

    /// Followers of a grandmaster triib does not read hang under a node of
    /// its own where none of them fell back behind a switch; one with no
    /// offset measured is not in sync.
    #[test]
    fn an_unread_grandmaster_gets_a_node() {
        use atdecc::lite::FallbackReason;
        let wired = running(FallbackReason::CONFIGURED, SWITCH, None);
        let entities = [lite_report(WIRED, "Synthia", WIRED_CLOCK, &wired)];
        let topology = Topology::build(&entities, None);
        let root = topology.roots[0];
        assert_eq!(topology.nodes[root].kind, Kind::Clock);
        assert_eq!(topology.nodes[root].clock, Some(SWITCH));
        assert_eq!(topology.nodes[root].domain, Some(0));
        let follower = topology.nodes[root].children[0];
        assert!(!topology.nodes[follower].link.synced);
    }
}
