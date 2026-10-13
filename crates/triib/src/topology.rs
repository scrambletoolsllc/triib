//! Where things sit in the network, from what entities report of their
//! gPTP paths: a tree from each grandmaster down through the bridges to the
//! entities, what is not on any tree and why, and the links a stream
//! between two entities crosses. Where the network runs AVB Lite, the
//! clock instead flows end to end from each grandmaster to the devices
//! following it, as they report it. A Wi-Fi station hangs under its access
//! point, its link the Wi-Fi hop, as AVB Wireless has it.

use std::collections::HashMap;

use atdecc::aem::AvbInfo;
use atdecc::lite::{LiteFlags, LiteStatus};
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

/// Why a node is not on any tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Apart {
    /// It is its own grandmaster: gPTP does not run on its link.
    OwnGrandmaster,
    /// It follows this grandmaster but did not report its path.
    NoPath(ClockIdentity),
    /// It reported nothing of its gPTP state.
    Unreported,
    /// This computer has heard no bridge.
    NoNeighbor,
    /// This computer cannot listen for gPTP, so where it is is unknown.
    CannotListen,
    /// An entity on this computer, which cannot be read from here.
    OnThisComputer,
    /// An entity on a network running AVB Lite that does not run it.
    NotLite,
    /// An entity on a network running AVB Lite that reports nothing of it.
    LiteUnreported,
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
}

#[derive(Debug, Clone, Default, PartialEq, Hash)]
pub struct Topology {
    pub nodes: Vec<Node>,
    /// The grandmasters, each the root of a tree.
    pub roots: Vec<NodeId>,
    /// What is on no tree.
    pub apart: Vec<NodeId>,
    /// Built from AVB Lite's PTP rather than from gPTP paths.
    pub ptp: bool,
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
}

impl InterfaceReport<'_> {
    /// What the interface reports as a Wi-Fi station.
    fn station(&self) -> Option<&WirelessStatus> {
        self.wireless.filter(|status| !status.access_point())
    }
}

/// What an entity reports of AVB Lite, for the PTP topology.
pub struct LiteReport<'a> {
    pub entity_id: EntityId,
    pub name: &'a str,
    /// Each AVB interface: its index, its clock identity, and its AVB Lite
    /// status when it reports one.
    pub interfaces: Vec<(u16, ClockIdentity, Option<&'a LiteStatus>)>,
}

/// What this computer heard of its neighbor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostReport {
    /// The bridge's clock, its port and whether gPTP runs on the link.
    Heard(ClockIdentity, u16, bool),
    /// No bridge heard.
    Unheard,
    /// This computer cannot listen for gPTP on the interface.
    CannotListen,
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
                owners.entry(interface.clock).or_insert((
                    entity.entity_id,
                    interface.index,
                    entity.name,
                ));
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
                    name.to_owned(),
                ),
                None => (Kind::Bridge, String::new()),
            };
            let node = topology.push(Node {
                kind,
                clock: Some(clock),
                name,
                parent: None,
                children: Vec::new(),
                link: Link {
                    synced: true,
                    ..Link::default()
                },
                apart: None,
                access_point: false,
            });
            by_clock.insert(clock, node);
            if let Some((entity_id, interface, _)) = owner {
                by_entity.insert((entity_id, interface), node);
            }
            node
        };

        // Every reported path, chained from its grandmaster down.
        let mut unplaced = Vec::new();
        for entity in entities {
            if entity.interfaces.is_empty() {
                // No AVB interface read: nothing reported of its gPTP state.
                let node = topology.push(Node {
                    kind: Kind::Entity {
                        entity_id: entity.entity_id,
                        interface: 0,
                    },
                    clock: None,
                    name: entity.name.to_owned(),
                    parent: None,
                    children: Vec::new(),
                    link: Link::default(),
                    apart: Some(Apart::Unreported),
                    access_point: false,
                });
                topology.apart.push(node);
            }
            for interface in &entity.interfaces {
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
                            kind: Kind::Entity {
                                entity_id: entity.entity_id,
                                interface: interface.index,
                            },
                            clock: Some(interface.clock),
                            name: entity.name.to_owned(),
                            parent: None,
                            children: Vec::new(),
                            link: Link {
                                synced: true,
                                ..Link::default()
                            },
                            apart: None,
                            access_point: false,
                        });
                        by_entity.insert(key, node);
                        by_clock.entry(interface.clock).or_insert(node);
                        node
                    }
                };
                if let Some(parent) = parent {
                    topology.attach(node, parent);
                }
                topology.nodes[node].link.delay = interface.info.map(|info| info.propagation_delay);
                topology.wireless_hop(node, interface);
            }
        }
        // A path of the entity alone makes it a grandmaster, a root only
        // when another entity's path starts at it.
        topology.roots = (0..topology.nodes.len())
            .filter(|&node| {
                topology.nodes[node].parent.is_none() && !topology.nodes[node].children.is_empty()
            })
            .collect();
        for node in 0..topology.nodes.len() {
            let entry = &topology.nodes[node];
            if entry.parent.is_none() && entry.children.is_empty() && entry.apart.is_none() {
                topology.nodes[node].apart = Some(Apart::OwnGrandmaster);
                topology.apart.push(node);
            }
        }

        // Entities that did not report a path are placed when another's
        // path names them, and set apart otherwise.
        for (entity, interface) in unplaced {
            let own = by_entity.get(&(entity.entity_id, interface.index)).copied();
            if let Some(node) = own.or_else(|| by_clock.get(&interface.clock).copied()) {
                // Beside another entity on the same clock that is placed.
                let node = match (own, topology.nodes[node].parent) {
                    (None, Some(parent)) => {
                        let beside = topology.push(Node {
                            kind: Kind::Entity {
                                entity_id: entity.entity_id,
                                interface: interface.index,
                            },
                            clock: Some(interface.clock),
                            name: entity.name.to_owned(),
                            parent: None,
                            children: Vec::new(),
                            link: topology.nodes[node].link,
                            apart: None,
                            access_point: false,
                        });
                        topology.attach(beside, parent);
                        beside
                    }
                    _ => node,
                };
                topology.nodes[node].link.delay = interface.info.map(|info| info.propagation_delay);
                topology.wireless_hop(node, interface);
                continue;
            }
            // A station that reports no gPTP path, as in Mode B, hangs
            // under the access point it names.
            if let Some(&access_point) = interface
                .station()
                .and_then(|status| by_clock.get(&status.ap_clock))
            {
                let node = topology.push(Node {
                    kind: Kind::Entity {
                        entity_id: entity.entity_id,
                        interface: interface.index,
                    },
                    clock: Some(interface.clock),
                    name: entity.name.to_owned(),
                    parent: None,
                    children: Vec::new(),
                    link: Link {
                        delay: interface.info.map(|info| info.propagation_delay),
                        ..Link::default()
                    },
                    apart: None,
                    access_point: false,
                });
                by_clock.insert(interface.clock, node);
                topology.attach(node, access_point);
                topology.wireless_hop(node, interface);
                continue;
            }
            let apart = match interface.info {
                Some(info) if info.gptp_grandmaster_id == interface.clock => Apart::OwnGrandmaster,
                Some(info) => Apart::NoPath(info.gptp_grandmaster_id),
                None => Apart::Unreported,
            };
            let node = topology.push(Node {
                kind: Kind::Entity {
                    entity_id: entity.entity_id,
                    interface: interface.index,
                },
                clock: Some(interface.clock),
                name: entity.name.to_owned(),
                parent: None,
                children: Vec::new(),
                link: Link {
                    delay: interface.info.map(|info| info.propagation_delay),
                    ..Link::default()
                },
                apart: Some(apart),
                access_point: false,
            });
            topology.apart.push(node);
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
                kind: Kind::Host,
                clock: None,
                name: String::new(),
                parent: None,
                children: Vec::new(),
                link: Link::default(),
                apart: Some(Apart::NoNeighbor),
                access_point: false,
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
                HostReport::Unheard | HostReport::CannotListen => {
                    if host == HostReport::CannotListen {
                        node.apart = Some(Apart::CannotListen);
                    }
                    let host = topology.push(node);
                    topology.apart.push(host);
                }
            }
        }

        topology.sort();
        topology
    }

    /// The clock as AVB Lite runs it (AVB Lite profile, 5): PTP end to end
    /// from each grandmaster to every device following it, through
    /// switches that take no part, so no bridges. A follower is in sync
    /// while its offset is measured and within `alarm` nanoseconds; an
    /// entity that does not run AVB Lite is set apart.
    pub fn build_lite(entities: &[LiteReport<'_>], alarm: i32) -> Self {
        let mut topology = Topology {
            ptp: true,
            ..Topology::default()
        };
        let mut by_clock: HashMap<ClockIdentity, NodeId> = HashMap::new();
        // Each node running AVB Lite, its grandmaster and its offset.
        let mut follows: Vec<(NodeId, ClockIdentity, Option<i32>)> = Vec::new();
        for entity in entities {
            let mut running = false;
            let reported = entity
                .interfaces
                .iter()
                .any(|(_, _, status)| status.is_some());
            for &(interface, clock, status) in &entity.interfaces {
                let Some(status) = status.filter(|status| status.flags.contains(LiteFlags::ACTIVE))
                else {
                    continue;
                };
                running = true;
                // Its own node, even where it shares a clock with another,
                // as this computer's endpoints share its clock.
                let node = topology.push(Node {
                    kind: Kind::Entity {
                        entity_id: entity.entity_id,
                        interface,
                    },
                    clock: Some(clock),
                    name: entity.name.to_owned(),
                    parent: None,
                    children: Vec::new(),
                    link: Link::default(),
                    apart: None,
                    access_point: false,
                });
                by_clock.entry(clock).or_insert(node);
                // An endpoint alone names no grandmaster, or itself.
                let grandmaster = if status.grandmaster.0 == 0 {
                    clock
                } else {
                    status.grandmaster
                };
                follows.push((node, grandmaster, status.offset()));
            }
            if !running {
                let first = entity.interfaces.first();
                let node = topology.push(Node {
                    kind: Kind::Entity {
                        entity_id: entity.entity_id,
                        interface: first.map_or(0, |(index, _, _)| *index),
                    },
                    clock: first.map(|(_, clock, _)| *clock),
                    name: entity.name.to_owned(),
                    parent: None,
                    children: Vec::new(),
                    link: Link::default(),
                    apart: Some(if reported {
                        Apart::NotLite
                    } else {
                        Apart::LiteUnreported
                    }),
                    access_point: false,
                });
                topology.apart.push(node);
            }
        }
        for (node, grandmaster, offset) in follows {
            if topology.nodes[node].clock == Some(grandmaster) {
                continue;
            }
            let root = match by_clock.get(&grandmaster) {
                Some(&root) => root,
                None => {
                    let root = topology.push(Node {
                        kind: Kind::Clock,
                        clock: Some(grandmaster),
                        name: String::new(),
                        parent: None,
                        children: Vec::new(),
                        link: Link::default(),
                        apart: None,
                        access_point: false,
                    });
                    by_clock.insert(grandmaster, root);
                    root
                }
            };
            topology.attach(node, root);
            topology.nodes[node].link = Link {
                offset,
                synced: offset.is_some_and(|offset| offset.unsigned_abs() <= alarm.unsigned_abs()),
                ..Link::default()
            };
        }
        topology.roots = (0..topology.nodes.len())
            .filter(|&node| {
                topology.nodes[node].parent.is_none() && topology.nodes[node].apart.is_none()
            })
            .collect();
        topology.sort();
        topology
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

    /// Orders children: entities by name, then this computer, then bridges.
    fn sort(&mut self) {
        let key = |node: &Node| {
            let rank = match node.kind {
                Kind::Entity { .. } => 0,
                Kind::Host => 1,
                Kind::Bridge | Kind::Clock => 2,
            };
            (rank, node.name.to_lowercase(), node.clock)
        };
        let keys: Vec<_> = self.nodes.iter().map(key).collect();
        for node in &mut self.nodes {
            node.children.sort_by_key(|&child| keys[child].clone());
        }
        self.roots.sort_by_key(|&root| self.nodes[root].clock);
        self.apart.sort_by_key(|&node| keys[node].clone());
    }

    /// The first of the entity's interfaces that is on a tree.
    pub fn placed(&self, entity_id: EntityId) -> Option<NodeId> {
        self.nodes.iter().position(|node| {
            matches!(node.kind, Kind::Entity { entity_id: id, .. } if id == entity_id)
                && node.apart.is_none()
        })
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
                }],
            },
        ];
        Topology::build(&entities, host)
    }

    #[test]
    fn the_bench() {
        let topology = bench(Some(HostReport::Heard(SWITCH, 6, false)));
        assert_eq!(topology.roots.len(), 1);
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
        assert_eq!(topology.apart.len(), 1);
        let wifi = topology.apart[0];
        assert_eq!(topology.nodes[wifi].apart, Some(Apart::OwnGrandmaster));
        assert_eq!(topology.placed(WIFI), None);
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

    /// A plain switch with the wired ESP as grandmaster: the Wi-Fi one
    /// follows it closely, and this computer's endpoint too far off; the
    /// Mac, which runs no AVB Lite, is set apart. No switch is shown, and
    /// each follower hangs straight under the grandmaster.
    #[test]
    fn avb_lite_flows_end_to_end_from_the_grandmaster() {
        const HOST_CLOCK: ClockIdentity = ClockIdentity(0x9c6b_00ff_fe30_9a2b);
        const HOST_TALKER: EntityId = EntityId(0x9c6b_00ff_0030_9a2b);
        let active = LiteFlags::CAPABLE.union(LiteFlags::ACTIVE);
        let wired = lite(active, WIRED_CLOCK, None);
        let wifi = lite(active, WIRED_CLOCK, Some(-180));
        let talker = lite(active, WIRED_CLOCK, Some(72_000));
        let mac = lite(LiteFlags::CAPABLE, ClockIdentity(0), None);
        let entities = [
            LiteReport {
                entity_id: WIRED,
                name: "Synthia",
                interfaces: vec![(0, WIRED_CLOCK, Some(&wired))],
            },
            LiteReport {
                entity_id: WIFI,
                name: "AVB Wireless Entity",
                interfaces: vec![(0, WIFI_CLOCK, Some(&wifi))],
            },
            LiteReport {
                entity_id: HOST_TALKER,
                name: "Host talker 1",
                interfaces: vec![(0, HOST_CLOCK, Some(&talker))],
            },
            LiteReport {
                entity_id: MAC,
                name: "Mac mini",
                interfaces: vec![(0, MAC_CLOCK, Some(&mac))],
            },
        ];
        let topology = Topology::build_lite(&entities, 50_000);
        assert!(topology.ptp);
        assert!(topology.nodes.iter().all(|node| node.kind != Kind::Bridge));
        assert_eq!(topology.roots.len(), 1);
        let root = topology.roots[0];
        assert_eq!(topology.nodes[root].clock, Some(WIRED_CLOCK));
        assert_eq!(topology.nodes[root].children.len(), 2);
        let node = |kind: Kind| {
            topology
                .nodes
                .iter()
                .find(|node| node.kind == kind)
                .unwrap()
        };
        let wifi = node(Kind::Entity {
            entity_id: WIFI,
            interface: 0,
        });
        assert_eq!(wifi.parent, Some(root));
        assert!(wifi.link.synced);
        assert_eq!(wifi.link.offset, Some(-180));
        // 72 us is past the 50 us AVB Lite allows. This computer shows as
        // its entity alone.
        let talker = node(Kind::Entity {
            entity_id: HOST_TALKER,
            interface: 0,
        });
        assert_eq!(talker.parent, Some(root));
        assert!(!talker.link.synced);
        assert!(topology.nodes.iter().all(|node| node.kind != Kind::Host));
        let mac = node(Kind::Entity {
            entity_id: MAC,
            interface: 0,
        });
        assert_eq!(mac.apart, Some(Apart::NotLite));
        assert_eq!(topology.apart.len(), 1);
        // One that reports no AVB Lite status at all is set apart for that.
        let quiet = [LiteReport {
            entity_id: MAC,
            name: "Mac mini",
            interfaces: vec![(0, MAC_CLOCK, None)],
        }];
        let topology = Topology::build_lite(&quiet, 50_000);
        assert_eq!(topology.nodes[0].apart, Some(Apart::LiteUnreported));
    }

    /// Followers of a grandmaster triib does not read hang under a node of
    /// its own; one with no offset measured is not in sync.
    #[test]
    fn an_unread_grandmaster_gets_a_node() {
        let active = LiteFlags::CAPABLE.union(LiteFlags::ACTIVE);
        let wired = lite(active, SWITCH, None);
        let entities = [LiteReport {
            entity_id: WIRED,
            name: "Synthia",
            interfaces: vec![(0, WIRED_CLOCK, Some(&wired))],
        }];
        let topology = Topology::build_lite(&entities, 50_000);
        let root = topology.roots[0];
        assert_eq!(topology.nodes[root].kind, Kind::Clock);
        assert_eq!(topology.nodes[root].clock, Some(SWITCH));
        let follower = topology.nodes[root].children[0];
        assert!(!topology.nodes[follower].link.synced);
    }
}
