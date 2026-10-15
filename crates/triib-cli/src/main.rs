//! Headless ATDECC controller. It lists interfaces, discovers entities,
//! reads their descriptors, maps the network from their gPTP paths,
//! renames entities, changes stream formats, sampling rates and clock
//! sources, shows the media clock each entity follows, shows and changes
//! how channels map to streams, shows and sets controls, connects and
//! disconnects streams, reads what streams and descriptors hold, and
//! shows how entities run AVB Lite.

use std::collections::{BTreeMap, BTreeSet};
use std::process::ExitCode;
use std::time::{Duration, Instant};

use atdecc::aem::{AudioMapping, AvbInfoFlags, MappingChange};
use atdecc::blocking::Driver;
use atdecc::control::{ControlDescriptor, Linear, Number, Selector, Shape, Unit, encode_values};
use atdecc::controller::{Controller, Outcome, Refusal};
use atdecc::descriptor::{ClockSourceType, DescriptorType, LocalizedStringRef, SamplingRate};
use atdecc::lite::LiteFlags;
use atdecc::media_clock::{Broken, ClockFrom, DomainClock, DomainId, media_clocks};
use atdecc::model::{EntityModel, EnumerationState};
use atdecc::neighbor::{Neighbor, NeighborListener};
use atdecc::stream_format::StreamFormat;
use atdecc::wireless::{AsCapableReason, Band, StationFlags, TimeMode, WirelessFlags};
use atdecc::{
    ClockIdentity, ControllerCapabilities, DiscoveredEntity, EntityCapabilities, EntityId, Event,
    ListenerCapabilities, OfflineReason, StreamId, TalkerCapabilities,
};

const USAGE: &str = "\
usage: triib-cli <command>

commands:
  interfaces                         list Ethernet interfaces and what they can do
  discover <interface> [seconds]     list entities as they come, go and are read,
                                     for 10 seconds by default, 0 for until stopped
  describe <interface> [entity-id]   read entities' descriptors and print what
                                     they say, every entity when no ID is given
  network <interface>                read every entity and print the network as
                                     their gPTP paths show it
  name <interface> <entity-id> <what> <name>
                                     rename what: entity, group, or a descriptor
                                     as type:index, such as stream-input:1
  format <interface> <entity-id> <stream> <format>
                                     set a stream's format, the stream as
                                     stream-input:0 or stream-output:0 and the
                                     format in hex, as describe prints them
  rate <interface> <entity-id> <audio-unit> <hertz>
                                     set an audio unit's sampling rate
  clock <interface> <entity-id> <clock-domain> <clock-source>
                                     pick a clock domain's clock source
  clocks <interface>                 read every entity and print each media clock
                                     reference with the entities following it
  maps <interface> [entity-id]       print how each stream port's channels map
                                     to and from the streams
  map <interface> <entity-id> <add|remove> <port> <mapping>...
                                     change a stream port's dynamic mappings,
                                     the port as stream-port-input:0 and each
                                     mapping as stream:channel=cluster:channel,
                                     the cluster counted from the port's first
  connect <interface> <talker-id>:<output> <listener-id>:<input>
                                     bind a listener's stream input to a
                                     talker's stream output
  disconnect <interface> <listener-id>:<input>
                                     unbind a listener's stream input
  disconnect-talker <interface> <talker-id>:<output> <listener-id>:<input>
                                     tell a talker to stop sending to a
                                     listener, as when the listener is gone
  configuration <interface> <entity-id> <configuration>
                                     make another of the entity's
                                     configurations its current one, then
                                     read it again
  identify <interface> <entity-id> [seconds]
                                     make an entity identify itself, for 5
                                     seconds by default
  streams <interface> <entity-id>    print each stream's state: binding, stream
                                     ID, destination, VLAN, latency, inputs'
                                     counters, and for outputs what the
                                     talker sends and their max transit time
  lite-config <interface> <entity-id> <avb-interface> <on|off>
                                     allow or disallow an interface's talkers
                                     escalating streams to multicast in AVB
                                     Lite (SET_LITE_CONFIG)
  lite <interface> <entity-id> [seconds]
                                     print how each of an entity's interfaces
                                     runs AVB Lite, as its status query
                                     answers, with the octets of the answer,
                                     then the streams it declares over CVU
                                     SRP, heard for the seconds given
  wireless-config <interface> <entity-id> <avb-interface> <on|off>
                                     allow or keep off Class A toward an
                                     access point's wireless port, which
                                     it admits only where it delivers at
                                     least 12000 frames a second
                                     (SET_WIRELESS_CONFIG)
  wireless <interface> <entity-id>   print what each of an entity's
                                     interfaces reports of AVB Wireless: its
                                     link, its time and, on an access point,
                                     its stations, with the octets of the
                                     answer
  transit <interface> <entity-id> <output> [nanoseconds]
                                     print or set a stream output's max transit
                                     time, 0 for the entity's default
  descriptor <interface> <entity-id> <type:index>
                                     read a descriptor again and print its
                                     octets, the type as in name
  harvest <interface> <entity-id> [repeat]
                                     read index 0 of each descriptor type the
                                     entity has and print how long each took
  controls <interface> [entity-id]   print each control's values and ranges
  control <interface> <entity-id> <control> <value>...
                                     set a control by its index, each value in
                                     its unit, such as -12 for -12 dB, a
                                     selector's as one of its options
  help                               show this text";

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let argument = |position: usize| arguments.get(position).map(String::as_str);
    let result = match argument(0) {
        Some("interfaces") => {
            print_interfaces();
            Ok(())
        }
        Some("discover") => {
            let Some(interface) = argument(1) else {
                return usage();
            };
            let seconds = match argument(2).map(str::parse::<u64>) {
                None => 10,
                Some(Ok(seconds)) => seconds,
                Some(Err(_)) => return usage(),
            };
            discover(interface, seconds).map_err(|error| format!("{interface}: {error}"))
        }
        Some("describe") => {
            let Some(interface) = argument(1) else {
                return usage();
            };
            let entity = match argument(2).map(str::parse::<EntityId>) {
                None => None,
                Some(Ok(entity)) => Some(entity),
                Some(Err(_)) => return usage(),
            };
            describe(interface, entity).map_err(|error| format!("{interface}: {error}"))
        }
        Some("network") => {
            let Some(interface) = argument(1) else {
                return usage();
            };
            network(interface).map_err(|error| format!("{interface}: {error}"))
        }
        Some("clocks") => {
            let Some(interface) = argument(1) else {
                return usage();
            };
            clocks(interface).map_err(|error| format!("{interface}: {error}"))
        }
        Some("maps") => {
            let Some(interface) = argument(1) else {
                return usage();
            };
            let entity = match argument(2).map(str::parse::<EntityId>) {
                None => None,
                Some(Ok(entity)) => Some(entity),
                Some(Err(_)) => return usage(),
            };
            maps(interface, entity).map_err(|error| format!("{interface}: {error}"))
        }
        Some("map") => {
            let (Some(interface), Some(Ok(entity)), Some(change), Some(port)) = (
                argument(1),
                argument(2).map(str::parse::<EntityId>),
                argument(3),
                argument(4),
            ) else {
                return usage();
            };
            let Some(change) = Change::parse_mappings(change, port, &arguments[5..]) else {
                return usage();
            };
            set(interface, entity, change).map_err(|error| format!("{interface}: {error}"))
        }
        Some("connect") => {
            let (Some(interface), Some(Some(talker)), Some(Some(listener))) = (
                argument(1),
                argument(2).map(stream_reference),
                argument(3).map(stream_reference),
            ) else {
                return usage();
            };
            let change = Change::Connect {
                talker,
                input: listener.1,
            };
            set(interface, listener.0, change).map_err(|error| format!("{interface}: {error}"))
        }
        Some("disconnect") => {
            let (Some(interface), Some(Some(listener))) =
                (argument(1), argument(2).map(stream_reference))
            else {
                return usage();
            };
            let change = Change::Disconnect { input: listener.1 };
            set(interface, listener.0, change).map_err(|error| format!("{interface}: {error}"))
        }
        Some("disconnect-talker") => {
            let (Some(interface), Some(Some(talker)), Some(Some(listener))) = (
                argument(1),
                argument(2).map(stream_reference),
                argument(3).map(stream_reference),
            ) else {
                return usage();
            };
            let change = Change::DisconnectTalker {
                output: talker.1,
                listener,
            };
            set(interface, talker.0, change).map_err(|error| format!("{interface}: {error}"))
        }
        Some("configuration") => {
            let (Some(interface), Some(Ok(entity)), Some(Ok(index))) = (
                argument(1),
                argument(2).map(str::parse::<EntityId>),
                argument(3).map(str::parse::<u16>),
            ) else {
                return usage();
            };
            set(interface, entity, Change::Configuration { index })
                .map_err(|error| format!("{interface}: {error}"))
        }
        Some("identify") => {
            let (Some(interface), Some(Ok(entity))) =
                (argument(1), argument(2).map(str::parse::<EntityId>))
            else {
                return usage();
            };
            let seconds = match argument(3).map(str::parse::<u64>) {
                None => 5,
                Some(Ok(seconds)) => seconds,
                Some(Err(_)) => return usage(),
            };
            set(interface, entity, Change::Identify { seconds })
                .map_err(|error| format!("{interface}: {error}"))
        }
        Some("streams") => {
            let (Some(interface), Some(Ok(entity))) =
                (argument(1), argument(2).map(str::parse::<EntityId>))
            else {
                return usage();
            };
            streams(interface, entity).map_err(|error| format!("{interface}: {error}"))
        }
        Some("lite") => {
            let (Some(interface), Some(Ok(entity))) =
                (argument(1), argument(2).map(str::parse::<EntityId>))
            else {
                return usage();
            };
            let seconds = match argument(3).map(str::parse::<u64>) {
                None => 0,
                Some(Ok(seconds)) => seconds,
                Some(Err(_)) => return usage(),
            };
            lite(interface, entity, Duration::from_secs(seconds))
                .map_err(|error| format!("{interface}: {error}"))
        }
        Some("wireless") => {
            let (Some(interface), Some(Ok(entity))) =
                (argument(1), argument(2).map(str::parse::<EntityId>))
            else {
                return usage();
            };
            wireless(interface, entity).map_err(|error| format!("{interface}: {error}"))
        }
        Some("transit") => {
            let (Some(interface), Some(Ok(entity)), Some(Ok(output))) = (
                argument(1),
                argument(2).map(str::parse::<EntityId>),
                argument(3).map(str::parse::<u16>),
            ) else {
                return usage();
            };
            let nanoseconds = match argument(4).map(str::parse::<u64>) {
                None => None,
                Some(Ok(nanoseconds)) => Some(nanoseconds),
                Some(Err(_)) => return usage(),
            };
            let change = Change::Transit {
                output,
                nanoseconds,
            };
            set(interface, entity, change).map_err(|error| format!("{interface}: {error}"))
        }
        Some("descriptor") => {
            let (Some(interface), Some(Ok(entity)), Some(Some((descriptor_type, index)))) = (
                argument(1),
                argument(2).map(str::parse::<EntityId>),
                argument(3).map(descriptor),
            ) else {
                return usage();
            };
            let change = Change::Read {
                descriptor_type,
                index,
            };
            set(interface, entity, change).map_err(|error| format!("{interface}: {error}"))
        }
        Some("harvest") => {
            let (Some(interface), Some(Ok(entity))) =
                (argument(1), argument(2).map(str::parse::<EntityId>))
            else {
                return usage();
            };
            let repeat = match argument(3).map(str::parse::<u32>) {
                None => 1,
                Some(Ok(repeat)) if repeat > 0 => repeat,
                Some(_) => return usage(),
            };
            harvest(interface, entity, repeat).map_err(|error| format!("{interface}: {error}"))
        }
        Some("controls") => {
            let Some(interface) = argument(1) else {
                return usage();
            };
            let entity = match argument(2).map(str::parse::<EntityId>) {
                None => None,
                Some(Ok(entity)) => Some(entity),
                Some(Err(_)) => return usage(),
            };
            controls(interface, entity).map_err(|error| format!("{interface}: {error}"))
        }
        Some("control") => {
            let (Some(interface), Some(Ok(entity)), Some(Ok(index))) = (
                argument(1),
                argument(2).map(str::parse::<EntityId>),
                argument(3).map(str::parse::<u16>),
            ) else {
                return usage();
            };
            if arguments.len() < 5 {
                return usage();
            }
            let change = Change::Control {
                index,
                values: arguments[4..].to_vec(),
            };
            set(interface, entity, change).map_err(|error| format!("{interface}: {error}"))
        }
        Some(
            command @ ("name" | "format" | "rate" | "clock" | "lite-config" | "wireless-config"),
        ) => {
            let (Some(interface), Some(Ok(entity)), Some(target), Some(value)) = (
                argument(1),
                argument(2).map(str::parse::<EntityId>),
                argument(3),
                argument(4),
            ) else {
                return usage();
            };
            let Some(change) = Change::parse(command, target, value) else {
                return usage();
            };
            set(interface, entity, change).map_err(|error| format!("{interface}: {error}"))
        }
        Some("help" | "--help" | "-h") => {
            println!("{USAGE}");
            Ok(())
        }
        Some("--version" | "-V") => {
            println!("triib-cli {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        _ => return usage(),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("triib-cli: {error}");
            ExitCode::FAILURE
        }
    }
}

fn usage() -> ExitCode {
    eprintln!("{USAGE}");
    ExitCode::FAILURE
}

fn print_interfaces() {
    let interfaces = avb_net::interfaces();
    match avb_net::check_access() {
        Ok(()) => println!("raw Ethernet: ready"),
        Err(error) => println!("raw Ethernet: {error}"),
    }
    if interfaces.is_empty() {
        println!("no Ethernet interfaces found");
        return;
    }
    let width = interfaces
        .iter()
        .map(|interface| interface.name.len())
        .max()
        .unwrap_or(0);
    for interface in interfaces {
        let speed = interface
            .speed
            .map(|speed| format!("{speed} Mb/s"))
            .unwrap_or_default();
        let mut notes = Vec::new();
        if !interface.physical {
            notes.push("virtual".to_owned());
        }
        if interface.wireless {
            notes.push("wireless".to_owned());
        }
        if let Some(clock) = interface.hardware_clock {
            notes.push(format!("hardware clock ptp{clock}"));
        }
        println!(
            "{:<width$}  {}  {:<4}  {:>11}  {}",
            interface.name,
            interface.mac,
            if interface.up { "up" } else { "down" },
            speed,
            notes.join(", "),
        );
    }
}

/// Opens `interface` for a short-lived controller: it reads descriptors
/// but neither advertises nor registers for notifications, as it will be
/// gone before they matter. Its entity ID differs from the app's on the
/// same interface, FF-FD in its middle where the app's has FF-FE, so
/// entities tell the two apart and tell the app what the CLI changes.
fn open(interface: &str) -> std::io::Result<Driver> {
    Driver::open_with(interface, |config| {
        config.register_unsolicited = false;
        let mut octets = config.entity_id.0.to_be_bytes();
        octets[4] = 0xfd;
        config.entity_id = EntityId(u64::from_be_bytes(octets));
    })
}

/// Asks every entity to advertise, then prints them as they come, change,
/// are read and go, and a summary at the end.
fn discover(interface: &str, seconds: u64) -> std::io::Result<()> {
    let mut driver = open(interface)?;
    println!(
        "discovering on {interface} as controller {}",
        driver.controller().entity_id()
    );
    driver.controller_mut().discover(None);
    let started = Instant::now();
    let until = (seconds > 0).then(|| started + Duration::from_secs(seconds));
    loop {
        let wait = until
            .map(|until| until.saturating_duration_since(Instant::now()))
            .unwrap_or(Duration::from_secs(1))
            .min(Duration::from_millis(250));
        driver.turn(wait)?;
        let elapsed = started.elapsed().as_secs_f32();
        while let Some(event) = driver.controller_mut().poll_event() {
            let controller = driver.controller();
            let line = |mark: &str, entity_id: EntityId| match controller.entity(entity_id) {
                Some(entity) => format!("{elapsed:6.2}s  {mark} {}", summary(entity)),
                None => format!("{elapsed:6.2}s  {mark} {entity_id}"),
            };
            match event {
                Event::EntityOnline(entity_id) => println!("{}", line("+", entity_id)),
                Event::EntityChanged(entity_id) => println!("{}", line("~", entity_id)),
                Event::EntityRestarted(entity_id) => {
                    println!("{elapsed:6.2}s  ! {entity_id} restarted")
                }
                Event::EntityOffline(entity_id, reason) => {
                    let why = match reason {
                        OfflineReason::Departed => "departed",
                        OfflineReason::TimedOut => "timed out",
                    };
                    println!("{elapsed:6.2}s  - {entity_id} {why}");
                }
                Event::EntityEnumerated(entity_id) => {
                    let model = controller.model(entity_id);
                    let name = model
                        .and_then(EntityModel::entity_name)
                        .unwrap_or("unnamed");
                    let count = model.map_or(0, EntityModel::descriptor_count);
                    println!("{elapsed:6.2}s  = {entity_id} \"{name}\", {count} descriptors read");
                }
                Event::EnumerationFailed(entity_id, failure) => {
                    println!("{elapsed:6.2}s  x {entity_id} could not be read: {failure:?}");
                }
                Event::EnumerationStarted(_)
                | Event::EntityModelChanged(_)
                | Event::CommandFinished(_, _) => {}
            }
        }
        if until.is_some_and(|until| Instant::now() >= until) {
            break;
        }
    }
    let controller = driver.controller();
    let count = controller.entities().count();
    println!(
        "\n{count} {} online:",
        if count == 1 { "entity" } else { "entities" }
    );
    for entity in controller.entities() {
        let name = controller
            .model(entity.entity_id())
            .and_then(EntityModel::entity_name)
            .map(|name| format!("\"{name}\"  "))
            .unwrap_or_default();
        println!("  {name}{}", summary(entity));
    }
    if controller.malformed_frames() > 0 {
        println!("{} frames did not decode", controller.malformed_frames());
    }
    driver.close()
}

/// Discovers entities and reads `entity`, or every entity seen in the first
/// few seconds, until it has answered everything the controller asks.
fn read(interface: &str, entity: Option<EntityId>) -> std::io::Result<Driver> {
    const LIMIT: Duration = Duration::from_secs(30);
    let mut driver = open(interface)?;
    driver.controller_mut().discover(entity);
    settle(&mut driver, entity, LIMIT)?;
    Ok(driver)
}

/// Runs the driver until `entity` is found and read, or every entity
/// found is read when none is given, or `limit` has passed.
fn settle(driver: &mut Driver, entity: Option<EntityId>, limit: Duration) -> std::io::Result<()> {
    const DISCOVERY: Duration = Duration::from_secs(6);
    let started = Instant::now();
    loop {
        driver.turn(Duration::from_millis(100))?;
        while driver.controller_mut().poll_event().is_some() {}
        let controller = driver.controller();
        let settled = |entity_id: EntityId| {
            !controller.busy(entity_id)
                && controller
                    .model(entity_id)
                    .is_none_or(|model| model.state != EnumerationState::Reading)
        };
        let done = match entity {
            Some(entity_id) => controller.entity(entity_id).is_some() && settled(entity_id),
            None => {
                started.elapsed() >= DISCOVERY
                    && controller
                        .entities()
                        .all(|found| settled(found.entity_id()))
            }
        };
        if done || started.elapsed() >= limit {
            return Ok(());
        }
    }
}

/// A change to make to an entity's model.
#[derive(Debug, Clone)]
enum Change {
    Name {
        descriptor_type: DescriptorType,
        index: u16,
        name_index: u16,
        name: String,
    },
    Format {
        descriptor_type: DescriptorType,
        index: u16,
        format: StreamFormat,
    },
    Rate {
        unit: u16,
        hertz: u32,
    },
    Clock {
        domain: u16,
        source: u16,
    },
    Configuration {
        index: u16,
    },
    Mappings {
        change: MappingChange,
        port: (DescriptorType, u16),
        mappings: Vec<AudioMapping>,
    },
    /// A control's new values as typed, read against the control once its
    /// entity is read.
    Control {
        index: u16,
        values: Vec<String>,
    },
    /// Bind the entity's stream input to a talker's stream output.
    Connect {
        talker: (EntityId, u16),
        input: u16,
    },
    Disconnect {
        input: u16,
    },
    /// Tell the entity, a talker, to stop sending a stream output to a
    /// listener's stream input.
    DisconnectTalker {
        output: u16,
        listener: (EntityId, u16),
    },
    Identify {
        seconds: u64,
    },
    /// Read a stream output's max transit time, or set it.
    Transit {
        output: u16,
        nanoseconds: Option<u64>,
    },
    /// Read a descriptor again.
    Read {
        descriptor_type: DescriptorType,
        index: u16,
    },
    /// Allow or disallow escalating streams to multicast in AVB Lite.
    LiteConfig {
        interface: u16,
        escalation: bool,
    },
    /// Allow or disallow Class A toward an access point's wireless port.
    WirelessConfig {
        interface: u16,
        class_a: bool,
    },
}

impl Change {
    fn parse(command: &str, target: &str, value: &str) -> Option<Self> {
        let number = |text: &str| text.parse::<u16>().ok();
        match command {
            "name" => {
                let (descriptor_type, index, name_index) = match target {
                    "entity" => (DescriptorType::ENTITY, 0, 0),
                    "group" => (DescriptorType::ENTITY, 0, 1),
                    _ => {
                        let (descriptor_type, index) = descriptor(target)?;
                        (descriptor_type, index, 0)
                    }
                };
                Some(Change::Name {
                    descriptor_type,
                    index,
                    name_index,
                    name: value.to_owned(),
                })
            }
            "format" => {
                let (descriptor_type, index) = descriptor(target)?;
                let hex = value.trim_start_matches("0x");
                Some(Change::Format {
                    descriptor_type,
                    index,
                    format: StreamFormat(u64::from_str_radix(hex, 16).ok()?),
                })
            }
            "lite-config" => Some(Change::LiteConfig {
                interface: number(target)?,
                escalation: match value {
                    "on" => true,
                    "off" => false,
                    _ => return None,
                },
            }),
            "wireless-config" => Some(Change::WirelessConfig {
                interface: number(target)?,
                class_a: match value {
                    "on" => true,
                    "off" => false,
                    _ => return None,
                },
            }),
            "rate" => Some(Change::Rate {
                unit: number(target)?,
                hertz: value.parse().ok()?,
            }),
            "clock" => Some(Change::Clock {
                domain: number(target)?,
                source: number(value)?,
            }),
            _ => None,
        }
    }

    /// `map`'s arguments: add or remove, the port, and mappings written as
    /// stream:channel=cluster:channel.
    fn parse_mappings(change: &str, port: &str, mappings: &[String]) -> Option<Self> {
        let change = match change {
            "add" => MappingChange::Add,
            "remove" => MappingChange::Remove,
            _ => return None,
        };
        let port = descriptor(port)?;
        if !matches!(
            port.0,
            DescriptorType::STREAM_PORT_INPUT | DescriptorType::STREAM_PORT_OUTPUT
        ) || mappings.is_empty()
        {
            return None;
        }
        let pair = |text: &str| {
            let (first, second) = text.split_once(':')?;
            Some((first.parse::<u16>().ok()?, second.parse::<u16>().ok()?))
        };
        let mappings = mappings
            .iter()
            .map(|text| {
                let (stream, cluster) = text.split_once('=')?;
                let (stream_index, stream_channel) = pair(stream)?;
                let (cluster_offset, cluster_channel) = pair(cluster)?;
                Some(AudioMapping {
                    stream_index,
                    stream_channel,
                    cluster_offset,
                    cluster_channel,
                })
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Change::Mappings {
            change,
            port,
            mappings,
        })
    }
}

/// A descriptor named as type:index, the type as the standard names it in
/// lower case with dashes, such as stream-input:1.
/// An entity and one of its streams, as `0xe8f60ae092200000:1`.
fn stream_reference(text: &str) -> Option<(EntityId, u16)> {
    let (entity, index) = text.rsplit_once(':')?;
    Some((entity.parse().ok()?, index.parse().ok()?))
}

/// Octets as rows of 16 in hex, each with its offset.
fn print_octets(bytes: &[u8]) {
    for (row, chunk) in bytes.chunks(16).enumerate() {
        let octets: Vec<String> = chunk.iter().map(|octet| format!("{octet:02x}")).collect();
        println!("  {:04x}  {}", row * 16, octets.join(" "));
    }
}

/// Reads an entity and prints each stream's state, asking the talker
/// what it sends of each output first.
fn streams(interface: &str, entity: EntityId) -> std::io::Result<()> {
    const LIMIT: Duration = Duration::from_secs(10);
    let mut driver = read(interface, Some(entity))?;
    let Some(found) = driver.controller().entity(entity).copied() else {
        println!("{entity} not found");
        return driver.close();
    };
    let outputs: Vec<u16> = driver
        .controller()
        .model(entity)
        .map(|model| model.streams(false).map(|stream| stream.index).collect())
        .unwrap_or_default();
    let now = driver.now();
    let mut waiting: BTreeSet<_> = outputs
        .iter()
        .map(|&output| driver.controller_mut().tx_state(now, (entity, output)))
        .collect();
    let started = Instant::now();
    while !waiting.is_empty() && started.elapsed() < LIMIT {
        driver.turn(Duration::from_millis(100))?;
        while let Some(event) = driver.controller_mut().poll_event() {
            if let Event::CommandFinished(command, _) = event {
                waiting.remove(&command);
            }
        }
    }
    // Each output's listeners, one connection at a time.
    let counts: Vec<(u16, u16)> = outputs
        .iter()
        .filter_map(|&output| {
            let model = driver.controller().model(entity)?;
            Some((output, model.tx_state(output)?.connection_count))
        })
        .collect();
    let now = driver.now();
    let mut waiting: BTreeSet<_> = counts
        .iter()
        .flat_map(|&(output, count)| (0..count).map(move |index| (output, index)))
        .map(|(output, index)| {
            driver
                .controller_mut()
                .tx_connection(now, (entity, output), index)
        })
        .collect();
    let started = Instant::now();
    while !waiting.is_empty() && started.elapsed() < LIMIT {
        driver.turn(Duration::from_millis(100))?;
        while let Some(event) = driver.controller_mut().poll_event() {
            if let Event::CommandFinished(command, _) = event {
                waiting.remove(&command);
            }
        }
    }
    // Whether each listener is there, asked of it, as only the talker was
    // looked for.
    let listeners: BTreeSet<EntityId> = driver
        .controller()
        .model(entity)
        .map(|model| {
            outputs
                .iter()
                .flat_map(|&output| model.tx_connections(output).map(|(listener, _)| listener))
                .collect()
        })
        .unwrap_or_default();
    if !listeners.is_empty() {
        for &listener in &listeners {
            driver.controller_mut().discover(Some(listener));
        }
        let asked = Instant::now();
        while asked.elapsed() < Duration::from_secs(2) {
            driver.turn(Duration::from_millis(100))?;
            while driver.controller_mut().poll_event().is_some() {}
        }
    }
    println!("{}", summary(&found));
    let controller = driver.controller();
    let Some(model) = controller.model(entity) else {
        println!("  no AEM model");
        return driver.close();
    };
    for input in [true, false] {
        for stream in model.streams(input) {
            let side = if input { "input " } else { "output" };
            let name = model
                .name_of(stream.descriptor_type, stream.index)
                .unwrap_or_default();
            println!(
                "  {side} {:<3} \"{name}\"  {}",
                stream.index, stream.current_format
            );
            if input {
                match model
                    .binding(stream.index)
                    .and_then(|binding| binding.talker_stream())
                {
                    Some((talker, output)) => println!("      bound to {talker} output {output}"),
                    None => println!("      not bound"),
                }
            }
            if let Some(info) = model.stream_info(stream.descriptor_type, stream.index) {
                println!(
                    "      stream {}  to {}  VLAN {}  flags {}",
                    info.stream_id,
                    info.stream_dest_mac,
                    info.stream_vlan_id,
                    info.flags.names().collect::<Vec<_>>().join(" ")
                );
                if input && info.registering() {
                    println!(
                        "      {} us accumulated latency",
                        info.msrp_accumulated_latency / 1000
                    );
                }
                if input && let Some((state, status)) = info.probing {
                    let state = match state {
                        0 => "disabled",
                        1 => "passive",
                        2 => "active",
                        3 => "completed",
                        _ => "unknown",
                    };
                    println!("      probing {state}, last answer {status:?}");
                }
            }
            if input && let Some(counters) = model.stream_input_counters(stream.index) {
                let kept: Vec<String> = [
                    ("frames", counters.frames_rx),
                    ("locked", counters.media_locked),
                    ("unlocked", counters.media_unlocked),
                    ("interrupted", counters.stream_interrupted),
                    ("out of sequence", counters.seq_num_mismatch),
                    ("late", counters.late_timestamp),
                    ("early", counters.early_timestamp),
                    ("unsupported", counters.unsupported_format),
                ]
                .into_iter()
                .filter_map(|(name, count)| count.map(|count| format!("{count} {name}")))
                .collect();
                if !kept.is_empty() {
                    println!("      counters: {}", kept.join(", "));
                }
            }
            if !input {
                if let Some(state) = model.tx_state(stream.index) {
                    let plural = if state.connection_count == 1 { "" } else { "s" };
                    println!(
                        "      sends {} to {}, VLAN {}, {} listener{plural}",
                        state.stream_id, state.destination, state.vlan_id, state.connection_count
                    );
                }
                for (listener, input) in model.tx_connections(stream.index) {
                    let known = controller
                        .model(listener)
                        .and_then(|model| model.entity_name().map(str::to_owned))
                        .or_else(|| controller.entity(listener).map(|_| String::new()));
                    match known {
                        Some(name) if !name.is_empty() => {
                            println!("      to {listener} input {input}, \"{name}\"")
                        }
                        Some(_) => println!("      to {listener} input {input}"),
                        None => println!("      to {listener} input {input}, not online"),
                    }
                }
                if let Some(nanoseconds) = model.max_transit_time(stream.index) {
                    println!(
                        "      max transit time {nanoseconds} ns ({:.3} ms)",
                        nanoseconds as f64 / 1e6
                    );
                }
            }
        }
    }
    driver.close()
}

/// Reads an entity, which asks it for its AVB Wireless status, then prints
/// what each interface reports.
fn wireless(interface: &str, entity: EntityId) -> std::io::Result<()> {
    let driver = read(interface, Some(entity))?;
    let controller = driver.controller();
    let (Some(found), Some(model)) = (controller.entity(entity), controller.model(entity)) else {
        println!("{entity} not found or without an AEM model");
        return driver.close();
    };
    println!("{}", summary(found));
    for (index, _) in model.descriptors(DescriptorType::AVB_INTERFACE) {
        let name = model
            .name_of(DescriptorType::AVB_INTERFACE, index)
            .unwrap_or_default();
        let Some(wireless) = model.wireless(index) else {
            let why = match model.wireless_supported {
                Some(false) => "does not implement the AVB Wireless status query",
                Some(true) => "wired",
                None => "did not answer the AVB Wireless status query",
            };
            println!("  interface {index} \"{name}\": {why}");
            continue;
        };
        let status = &wireless.status;
        let flags = status.flags;
        if status.access_point() {
            println!(
                "  interface {index} \"{name}\": access point, BSS up {} s, Class A {}",
                status.association_age,
                if flags.contains(WirelessFlags::CLASS_A_ALLOWED) {
                    "allowed"
                } else {
                    "not allowed"
                }
            );
        } else {
            let time = if flags.contains(WirelessFlags::LOCKED) {
                "time locked"
            } else if flags.contains(WirelessFlags::HOLDOVER) {
                "holding over"
            } else {
                "time not locked"
            };
            println!(
                "  interface {index} \"{name}\": station, {}, {time}",
                time_mode(status.time_mode)
            );
        }
        println!("      {}", wireless_link(status));
        println!(
            "      BSSID {}, access point port {} port {}",
            status.bssid, status.ap_clock, status.ap_port
        );
        if status.access_point() {
            let unserved = status
                .unserved()
                .map_or("not known".to_owned(), |count| count.to_string());
            println!(
                "      stream frames readdressed {}, unmapped {}, dropped {}, restored {}; \
                 listeners unserved {unserved}",
                status.downlink_readdressed,
                status.downlink_unmapped,
                status.downlink_dropped,
                status.uplink_restored
            );
            if status.frame_budget > 0 {
                println!(
                    "      frames a second {} of a budget of {}",
                    status.frames_committed, status.frame_budget
                );
            }
            for station in &wireless.stations {
                let rssi = station
                    .signal()
                    .map_or("RSSI not known".to_owned(), |rssi| {
                        format!("RSSI {rssi} dBm")
                    });
                let ftm = if !station.flags.contains(StationFlags::FTM_KNOWN) {
                    "FTM not known"
                } else if station.flags.contains(StationFlags::FTM_INITIATOR) {
                    "FTM initiator"
                } else {
                    "no FTM"
                };
                println!("      station {}, {rssi}, {ftm}", station.mac);
            }
        } else {
            let mut ftm = Vec::new();
            if status.ftm_success != 0xff {
                ftm.push(format!("{}% valid", status.ftm_success));
            }
            if let Some(rtt) = status.rtt() {
                ftm.push(format!("RTT {rtt} ns"));
            }
            match status.ftm_burst_frames {
                0 => ftm.push("none".to_owned()),
                0xff => {}
                frames => ftm.push(format!("bursts of {frames} frames")),
            }
            if !matches!(status.ftm_burst_duration, 0 | 0xff) {
                ftm.push(format!("duration code {}", status.ftm_burst_duration));
            }
            if !matches!(status.ftm_min_delta, 0 | 0xff) {
                ftm.push(format!(
                    "min delta {:.1} ms",
                    f64::from(status.ftm_min_delta) / 10.0
                ));
            }
            if !ftm.is_empty() {
                println!("      FTM {}", ftm.join(", "));
            }
            if status.as_capable_reason != AsCapableReason::NONE {
                println!(
                    "      asCapable FALSE: {}",
                    match status.as_capable_reason {
                        AsCapableReason::BURST_FRAMES =>
                            "FTM bursts granted of other than three or two frames",
                        AsCapableReason::NO_MEASUREMENT => "neither FTM nor TM with the other end",
                        AsCapableReason::NO_SIGNALING => "no gPTP-capable Signaling from it",
                        _ => "a reason this version does not know",
                    }
                );
            }
            let servo = status
                .servo()
                .map_or("not known".to_owned(), |servo| format!("{servo} ns"));
            let age = match status.time_age {
                0xffff => "none applied".to_owned(),
                millis => format!("applied {millis} ms ago"),
            };
            println!(
                "      servo error {servo}, time element {age}, associated {} s, \
                 {} access point resets",
                status.association_age, status.ap_resets
            );
        }
        let mut octets = status.to_bytes().to_vec();
        for station in &wireless.stations {
            octets.extend_from_slice(&station.to_bytes());
        }
        print_octets(&octets);
    }
    driver.close()
}

fn time_mode(mode: TimeMode) -> String {
    match mode {
        TimeMode::NONE => "no time".to_owned(),
        TimeMode::MODE_A_FTM => "Mode A over FTM".to_owned(),
        TimeMode::MODE_A_TM => "Mode A over TM".to_owned(),
        TimeMode::MODE_B => "Mode B, beacon carrier".to_owned(),
        TimeMode(other) => format!("time mode {other}"),
    }
}

/// The band, channel, width, PHY, signal and rate an interface reports.
fn wireless_link(status: &atdecc::wireless::WirelessStatus) -> String {
    let mut parts = vec![match status.band {
        Band::GHZ_2_4 => "2.4 GHz".to_owned(),
        Band::GHZ_5 => "5 GHz".to_owned(),
        Band::GHZ_6 => "6 GHz".to_owned(),
        Band(0) => "band not known".to_owned(),
        Band(other) => format!("band {other}"),
    }];
    if status.channel != 0 {
        parts.push(format!("channel {}", status.channel));
    }
    if status.channel_width != 0 {
        parts.push(format!("{} MHz", status.channel_width));
    }
    match status.phy_generation {
        0 => {}
        3 => parts.push("802.11a/b/g".to_owned()),
        generation => parts.push(format!("Wi-Fi {generation}")),
    }
    if let Some(rssi) = status.signal() {
        parts.push(format!("RSSI {rssi} dBm"));
    }
    if status.phy_rate != 0 {
        parts.push(format!("{} Mb/s", status.phy_rate));
    }
    parts.join(", ")
}

/// Reads an entity, which asks it for its AVB Lite status, listens for
/// `listen` more, then prints what each interface reports and the streams
/// it declares over CVU SRP.
fn lite(interface: &str, entity: EntityId, listen: Duration) -> std::io::Result<()> {
    let mut driver = read(interface, Some(entity))?;
    let started = Instant::now();
    while started.elapsed() < listen {
        driver.turn(Duration::from_millis(100))?;
        while driver.controller_mut().poll_event().is_some() {}
    }
    let controller = driver.controller();
    let (Some(found), Some(model)) = (controller.entity(entity), controller.model(entity)) else {
        println!("{entity} not found or without an AEM model");
        return driver.close();
    };
    println!("{}", summary(found));
    for (index, _) in model.descriptors(DescriptorType::AVB_INTERFACE) {
        let name = model
            .name_of(DescriptorType::AVB_INTERFACE, index)
            .unwrap_or_default();
        let Some(status) = model.lite_status(index) else {
            let why = match model.lite_supported {
                Some(false) => "does not implement the AVB Lite status query",
                Some(true) => "reports no AVB Lite status",
                None => "did not answer the AVB Lite status query",
            };
            println!("  interface {index} \"{name}\": {why}");
            continue;
        };
        let mode = if status.flags.contains(LiteFlags::ACTIVE) {
            format!(
                "AVB Lite active, fallback reason {}",
                status.fallback_reason.name().unwrap_or("unknown")
            )
        } else if status.flags.contains(LiteFlags::CAPABLE) {
            "AVB Lite capable, running standard AVB".to_owned()
        } else {
            "not AVB Lite capable".to_owned()
        };
        println!("  interface {index} \"{name}\": {mode}");
        let offset = status
            .offset()
            .map_or("not measured".to_owned(), |offset| format!("{offset} ns"));
        println!(
            "      PTP {} domain {}, grandmaster {}, offset {offset}",
            status.ptp_profile.name().unwrap_or("unknown"),
            status.ptp_domain,
            status.grandmaster
        );
        let egress = status
            .egress()
            .map_or("not kept".to_owned(), |kilobits| format!("{kilobits} kb/s"));
        println!(
            "      media VLAN {}, unicast fan-out {}, link {} Mb/s, egress {egress}",
            status.media_vlan_id, status.unicast_fanout_limit, status.link_speed
        );
        println!(
            "      escalation to multicast {}",
            if status.flags.contains(LiteFlags::ESCALATION_ALLOWED) {
                "allowed"
            } else {
                "not allowed"
            }
        );
        print_octets(&status.to_bytes());
    }
    for declaration in model.cvu_talkers() {
        println!(
            "  CVU SRP: stream {} to {}, VLAN {}, {} octets x {} a class interval, \
             priority {}, {} us accumulated latency",
            StreamId(declaration.stream_id),
            declaration.destination,
            declaration.vlan_id,
            declaration.max_frame_size,
            declaration.max_interval_frames,
            declaration.priority,
            declaration.accumulated_latency / 1000
        );
    }
    driver.close()
}

/// Reads an entity, then index 0 of each descriptor type it has, `repeat`
/// times, printing how each read went and how long it took.
fn harvest(interface: &str, entity: EntityId, repeat: u32) -> std::io::Result<()> {
    const LIMIT: Duration = Duration::from_secs(10);
    let mut driver = read(interface, Some(entity))?;
    let Some(model) = driver.controller().model(entity) else {
        println!("{entity} not found or without an AEM model");
        return driver.close();
    };
    let mut types = vec![
        (DescriptorType::ENTITY, 0),
        (DescriptorType::CONFIGURATION, model.configuration),
    ];
    if let Some(configuration) = model.configuration() {
        types.extend(
            configuration
                .descriptor_counts()
                .filter(|&(_, count)| count > 0)
                .map(|(descriptor_type, _)| (descriptor_type, 0)),
        );
    }
    println!(
        "{:<24} {:<22} {:>9} {:>7}",
        "descriptor", "status", "ms", "octets"
    );
    for (descriptor_type, index) in types {
        for _ in 0..repeat {
            let started = Instant::now();
            let now = driver.now();
            let command =
                driver
                    .controller_mut()
                    .read_descriptor(now, entity, descriptor_type, index);
            let outcome = loop {
                driver.turn(Duration::from_millis(20))?;
                let finished = std::iter::from_fn(|| driver.controller_mut().poll_event())
                    .find_map(|event| match event {
                        Event::CommandFinished(finished, outcome) if finished == command => {
                            Some(outcome)
                        }
                        _ => None,
                    });
                if finished.is_some() || started.elapsed() >= LIMIT {
                    break finished;
                }
            };
            let elapsed = started.elapsed().as_secs_f64() * 1000.0;
            let status = match outcome {
                Some(Outcome::Done) => "done".to_owned(),
                Some(Outcome::Refused(Refusal::Aem(status))) => format!("{status:?}"),
                Some(other) => format!("{other:?}"),
                None => "no answer".to_owned(),
            };
            let octets = driver
                .controller()
                .model(entity)
                .and_then(|model| model.descriptor(descriptor_type, index))
                .map_or(0, <[u8]>::len);
            println!(
                "{:<24} {status:<22} {elapsed:>9.1} {octets:>7}",
                format!("{descriptor_type:?} {index}")
            );
        }
    }
    driver.close()
}

fn descriptor(text: &str) -> Option<(DescriptorType, u16)> {
    let (name, index) = text.split_once(':')?;
    let wanted = name.replace('-', "_").to_uppercase();
    let descriptor_type = (0..=u16::from(u8::MAX))
        .map(DescriptorType)
        .find(|descriptor_type| descriptor_type.name() == Some(wanted.as_str()))?;
    Some((descriptor_type, index.parse().ok()?))
}

/// Reads `entity`, makes the change, and prints how it went and what the
/// entity holds afterwards.
fn set(interface: &str, entity: EntityId, change: Change) -> std::io::Result<()> {
    const LIMIT: Duration = Duration::from_secs(10);
    let mut driver = read(interface, Some(entity))?;
    if driver.controller().entity(entity).is_none() {
        println!("{entity} not found");
        return driver.close();
    }
    if let Change::Connect { talker, .. } = &change {
        // The connect carries the talker's class, which reading it tells.
        driver.controller_mut().discover(Some(talker.0));
        settle(&mut driver, Some(talker.0), Duration::from_secs(6))?;
    }
    let control_values = match &change {
        Change::Control { index, values } => {
            match control_values(driver.controller().model(entity), *index, values) {
                Ok(encoded) => encoded,
                Err(why) => {
                    println!("{why}");
                    return driver.close();
                }
            }
        }
        _ => Vec::new(),
    };
    let now = driver.now();
    let controller = driver.controller_mut();
    let command = match &change {
        Change::Name {
            descriptor_type,
            index,
            name_index,
            name,
        } => controller.set_name(now, entity, *descriptor_type, *index, *name_index, name),
        Change::Format {
            descriptor_type,
            index,
            format,
        } => controller.set_stream_format(now, entity, *descriptor_type, *index, *format),
        Change::Rate { unit, hertz } => controller.set_sampling_rate(
            now,
            entity,
            DescriptorType::AUDIO_UNIT,
            *unit,
            SamplingRate(*hertz),
        ),
        Change::Clock { domain, source } => {
            controller.set_clock_source(now, entity, *domain, *source)
        }
        Change::Configuration { index } => controller.set_configuration(now, entity, *index),
        Change::Mappings {
            change: MappingChange::Add,
            port,
            mappings,
        } => controller.add_audio_mappings(now, entity, *port, mappings),
        Change::Mappings {
            change: MappingChange::Remove,
            port,
            mappings,
        } => controller.remove_audio_mappings(now, entity, *port, mappings),
        Change::Control { index, .. } => {
            controller.set_control(now, entity, *index, &control_values)
        }
        Change::Connect { talker, input } => controller.connect(now, *talker, (entity, *input)),
        Change::Disconnect { input } => controller.disconnect(now, (entity, *input)),
        Change::DisconnectTalker { output, listener } => {
            controller.disconnect_talker(now, (entity, *output), *listener)
        }
        Change::Identify { seconds } => {
            controller.identify(now, entity, Duration::from_secs(*seconds))
        }
        Change::Transit {
            output,
            nanoseconds: Some(nanoseconds),
        } => controller.set_max_transit_time(now, entity, *output, *nanoseconds),
        Change::Transit { output, .. } => controller.max_transit_time(now, entity, *output),
        Change::Read {
            descriptor_type,
            index,
        } => controller.read_descriptor(now, entity, *descriptor_type, *index),
        Change::LiteConfig {
            interface,
            escalation,
        } => controller.set_lite_config(now, entity, *interface, *escalation),
        Change::WirelessConfig { interface, class_a } => {
            controller.set_wireless_config(now, entity, *interface, *class_a)
        }
    };
    let started = Instant::now();
    let outcome = loop {
        driver.turn(Duration::from_millis(100))?;
        let finished = std::iter::from_fn(|| driver.controller_mut().poll_event()).find_map(
            |event| match event {
                Event::CommandFinished(finished, outcome) if finished == command => Some(outcome),
                _ => None,
            },
        );
        if let Some(outcome) = finished {
            break Some(outcome);
        }
        if started.elapsed() >= LIMIT {
            break None;
        }
    };
    // What the command reads again afterwards.
    while driver.controller().busy(entity) && started.elapsed() < LIMIT {
        driver.turn(Duration::from_millis(100))?;
        while driver.controller_mut().poll_event().is_some() {}
    }
    match outcome {
        Some(Outcome::Done) => println!("done"),
        Some(Outcome::Refused(Refusal::Aem(status))) => println!("refused: {status:?}"),
        Some(Outcome::Refused(Refusal::Acmp(status))) => println!("refused: {status:?}"),
        Some(Outcome::NoResponse) => println!("no response"),
        Some(Outcome::NotPossible) => println!("not possible"),
        None => println!("no answer in {} seconds", LIMIT.as_secs()),
    }
    if let Change::WirelessConfig { interface, .. } = change {
        // The status the controller asks for after the change shows it.
        let asked = Instant::now();
        while asked.elapsed() < Duration::from_secs(2) {
            driver.turn(Duration::from_millis(100))?;
            while driver.controller_mut().poll_event().is_some() {}
        }
        if let Some(wireless) = driver
            .controller()
            .model(entity)
            .and_then(|model| model.wireless(interface))
        {
            println!(
                "Class A now {}",
                if wireless
                    .status
                    .flags
                    .contains(WirelessFlags::CLASS_A_ALLOWED)
                {
                    "allowed"
                } else {
                    "not allowed"
                }
            );
        }
    }
    if let Change::LiteConfig { interface, .. } = change {
        // The status the controller asks for after the change shows it.
        let asked = Instant::now();
        while asked.elapsed() < Duration::from_secs(2) {
            driver.turn(Duration::from_millis(100))?;
            while driver.controller_mut().poll_event().is_some() {}
        }
        if let Some(status) = driver
            .controller()
            .model(entity)
            .and_then(|model| model.lite_status(interface))
        {
            println!(
                "escalation to multicast now {}",
                if status.flags.contains(LiteFlags::ESCALATION_ALLOWED) {
                    "allowed"
                } else {
                    "not allowed"
                }
            );
        }
    }
    if let Some(model) = driver.controller().model(entity) {
        match change {
            Change::Name {
                descriptor_type,
                index,
                name_index,
                ..
            } => {
                let name = match (descriptor_type, name_index) {
                    (DescriptorType::ENTITY, 0) => model.entity().map(|entity| entity.entity_name),
                    (DescriptorType::ENTITY, _) => model.entity().map(|entity| entity.group_name),
                    _ => model.name_of(descriptor_type, index),
                };
                println!("name now \"{}\"", name.unwrap_or_default());
            }
            Change::Format {
                descriptor_type,
                index,
                ..
            } => {
                let input = descriptor_type == DescriptorType::STREAM_INPUT;
                if let Some(stream) = model.streams(input).find(|stream| stream.index == index) {
                    println!(
                        "format now {:#018x}, {}",
                        stream.current_format.0, stream.current_format
                    );
                }
            }
            Change::Rate { unit, .. } => {
                if let Some(found) = model.audio_units().find(|found| found.index == unit) {
                    println!(
                        "rate now {} Hz",
                        found.current_sampling_rate.base_frequency()
                    );
                }
            }
            Change::Clock { domain, .. } => {
                if let Some(found) = model.clock_domains().find(|found| found.index == domain) {
                    let source = found.clock_source_index;
                    let name = model
                        .name_of(DescriptorType::CLOCK_SOURCE, source)
                        .map(|name| format!(", \"{name}\""))
                        .unwrap_or_default();
                    println!("clock source now {source}{name}");
                }
            }
            Change::Configuration { .. } => {
                if let Some(entity) = model.entity() {
                    let name = model
                        .name_of(DescriptorType::CONFIGURATION, entity.current_configuration)
                        .unwrap_or_default();
                    println!(
                        "configuration now {} of {}, \"{name}\", {} inputs, {} outputs",
                        entity.current_configuration,
                        entity.configurations_count,
                        model.streams(true).count(),
                        model.streams(false).count()
                    );
                }
            }
            Change::Mappings { port, .. } => {
                let input = port.0 == DescriptorType::STREAM_PORT_INPUT;
                if let Some(found) = model
                    .stream_ports(input)
                    .find(|found| found.index == port.1)
                {
                    print_port_mappings(model, &found);
                }
            }
            Change::Control { index, .. } => {
                if let Some(control) = model.control(index) {
                    print_control(model, &control);
                }
            }
            Change::Connect { input, .. } | Change::Disconnect { input } => {
                // A command no one answered leaves the binding unknown,
                // which is not the same as unbound.
                match model.binding(input).map(|binding| binding.talker_stream()) {
                    Some(Some((talker, output))) => {
                        println!("input {input} now bound to {talker} output {output}")
                    }
                    Some(None) => println!("input {input} now not bound"),
                    None => println!("input {input}: binding not known"),
                }
            }
            Change::Transit { output, .. } => match model.max_transit_time(output) {
                Some(nanoseconds) => println!(
                    "output {output} max transit time now {nanoseconds} ns ({:.3} ms)",
                    nanoseconds as f64 / 1e6
                ),
                None => println!("output {output} max transit time not known"),
            },
            Change::Read {
                descriptor_type,
                index,
            } => {
                if let Some(bytes) = model.descriptor(descriptor_type, index) {
                    let name = model
                        .name_of(descriptor_type, index)
                        .map(|name| format!(", \"{name}\""))
                        .unwrap_or_default();
                    println!("{descriptor_type:?} {index}{name}, {} octets", bytes.len());
                    print_octets(bytes);
                }
            }
            Change::DisconnectTalker { .. }
            | Change::Identify { .. }
            | Change::LiteConfig { .. }
            | Change::WirelessConfig { .. } => {}
        }
    }
    driver.close()
}

/// Reads every entity and prints each media clock reference, with the
/// clock domains following it as a tree through their streams, then the
/// domains whose chain breaks and why.
fn clocks(interface: &str) -> std::io::Result<()> {
    let driver = read(interface, None)?;
    let controller = driver.controller();
    let models: BTreeMap<EntityId, &EntityModel> = controller
        .entities()
        .filter_map(|found| Some((found.entity_id(), controller.model(found.entity_id())?)))
        .collect();
    let clocks = media_clocks(models.iter().map(|(&entity, &model)| (entity, model)));
    if clocks.is_empty() {
        println!("no clock domains found");
        return driver.close();
    }
    let references: Vec<&DomainClock> = clocks
        .iter()
        .filter(|clock| clock.reference == Ok(clock.id))
        .collect();
    for (number, reference) in references.iter().enumerate() {
        if number > 0 {
            println!();
        }
        let rate = models[&reference.id.entity].sampling_rate(reference.id.domain);
        print_clock_tree(&models, &clocks, reference, rate, 0);
    }
    let broken: Vec<&DomainClock> = clocks
        .iter()
        .filter(|clock| clock.reference.is_err())
        .collect();
    if !broken.is_empty() {
        println!();
        println!("not clocked");
        for clock in broken {
            let reason = match clock.reference {
                Err(Broken::Unbound(at)) if at == clock.id => {
                    "its clock source is a stream input bound to nothing".to_owned()
                }
                Err(Broken::Unbound(at)) => {
                    format!("{} upstream is bound to nothing", domain_name(&models, at))
                }
                Err(Broken::TalkerUnknown(at)) => {
                    format!(
                        "the talker {} takes its clock from is not known",
                        domain_name(&models, at)
                    )
                }
                Err(Broken::Unknown(at)) => {
                    format!(
                        "the clock source of {} is not known",
                        domain_name(&models, at)
                    )
                }
                Err(Broken::Loop(_)) => "its clock goes round in a loop".to_owned(),
                Ok(_) => continue,
            };
            println!("  {}: {reason}", domain_name(&models, clock.id));
        }
    }
    driver.close()
}

/// An entity's name, and its clock domain's when it has more than one.
fn domain_name(models: &BTreeMap<EntityId, &EntityModel>, id: DomainId) -> String {
    let Some(model) = models.get(&id.entity) else {
        return id.entity.to_string();
    };
    let entity = model
        .entity_name()
        .map_or_else(|| id.entity.to_string(), str::to_owned);
    if model.clock_domains().count() > 1 {
        let domain = model
            .name_of(DescriptorType::CLOCK_DOMAIN, id.domain)
            .unwrap_or("clock domain");
        format!("{entity} \"{domain}\"")
    } else {
        entity
    }
}

/// A domain on its line, indented by `depth`, then the domains clocked
/// from it below.
fn print_clock_tree(
    models: &BTreeMap<EntityId, &EntityModel>,
    clocks: &[DomainClock],
    clock: &DomainClock,
    reference_rate: Option<SamplingRate>,
    depth: usize,
) {
    let model = models[&clock.id.entity];
    let source_name = |index: u16| {
        model
            .name_of(DescriptorType::CLOCK_SOURCE, index)
            .map(|name| format!(", \"{name}\""))
            .unwrap_or_default()
    };
    let source = model
        .clock_domains()
        .find(|domain| domain.index == clock.id.domain)
        .map(|domain| domain.clock_source_index)
        .unwrap_or_default();
    let from = match clock.from {
        ClockFrom::Internal => format!("own clock{}", source_name(source)),
        ClockFrom::External => format!("external clock{}", source_name(source)),
        ClockFrom::Stream {
            input,
            talker,
            output,
        } => {
            let input_name = model
                .name_of(DescriptorType::STREAM_INPUT, input)
                .unwrap_or("stream input");
            let output_name = models
                .get(&talker)
                .and_then(|talker_model| {
                    talker_model.name_of(DescriptorType::STREAM_OUTPUT, output)
                })
                .unwrap_or("stream output");
            let talker_name = models
                .get(&talker)
                .and_then(|talker_model| talker_model.entity_name())
                .map_or_else(|| talker.to_string(), str::to_owned);
            let flowing = model
                .stream_info(DescriptorType::STREAM_INPUT, input)
                .is_some_and(|info| info.settled());
            let waiting = if flowing { "" } else { ", not flowing" };
            format!("from \"{input_name}\", fed by {talker_name} \"{output_name}\"{waiting}")
        }
        ClockFrom::Unbound { .. } | ClockFrom::Unknown => "no clock".to_owned(),
    };
    let rate = model.sampling_rate(clock.id.domain);
    let rate_text = match (rate, reference_rate) {
        (Some(rate), Some(reference)) if rate != reference => format!(
            ", {} Hz, not the reference's {} Hz",
            rate.base_frequency(),
            reference.base_frequency()
        ),
        (Some(rate), _) => format!(", {} Hz", rate.base_frequency()),
        (None, _) => String::new(),
    };
    let priority = model
        .media_clock_reference(clock.id.domain)
        .map(|reference| {
            let domain = reference
                .domain_name()
                .map(|name| format!(", domain \"{name}\""))
                .unwrap_or_default();
            format!(", priority {}{domain}", reference.priority())
        })
        .unwrap_or_default();
    let role = if depth == 0 { "reference" } else { "follows" };
    println!(
        "{}{role} {}: {from}{rate_text}{priority}",
        "  ".repeat(depth),
        domain_name(models, clock.id)
    );
    for follower in clocks
        .iter()
        .filter(|follower| follower.parent == Some(clock.id) && follower.reference.is_ok())
    {
        print_clock_tree(models, clocks, follower, reference_rate, depth + 1);
    }
}

/// Reads `entity`, or every entity, and prints how each stream port's
/// channels map to and from the streams.
/// A control's new values, typed in its units, encoded as SET_CONTROL
/// takes them; why not, when they do not fit it.
fn control_values(
    model: Option<&EntityModel>,
    index: u16,
    values: &[String],
) -> Result<Vec<u8>, String> {
    let model = model.ok_or("no AEM model")?;
    let control = model
        .control(index)
        .ok_or_else(|| format!("no control {index}"))?;
    if control.read_only {
        return Err(format!("control {index} is read only"));
    }
    let shown = |text: &str| {
        text.parse::<f64>()
            .map_err(|_| format!("{text} is not a number"))
    };
    let count = |wanted: usize| {
        if values.len() == wanted {
            Ok(())
        } else {
            let plural = if wanted == 1 { "" } else { "s" };
            Err(format!("control {index} takes {wanted} value{plural}"))
        }
    };
    let mut out = vec![0; atdecc::aem::MAX_CONTROL_VALUES];
    let scalar = control.value_type.scalar();
    let numbers: Vec<Number> = match (control.value_type.shape(), scalar) {
        (Shape::Linear, Some(scalar)) => {
            let linear: Vec<Linear> = control.linear().collect();
            count(linear.len())?;
            values
                .iter()
                .zip(&linear)
                .map(
                    |(text, value)| Ok(value.nearest(value.unit.raw(shown(text)?, scalar), scalar)),
                )
                .collect::<Result<_, String>>()?
        }
        (Shape::Array, Some(scalar)) => {
            let array = control.array().ok_or("no array")?;
            count(usize::from(control.count()))?;
            values
                .iter()
                .map(|text| Ok(array.nearest(array.unit.raw(shown(text)?, scalar))))
                .collect::<Result<_, String>>()?
        }
        (Shape::Selector, Some(_)) => {
            let selector = control.selector().ok_or("no selector")?;
            count(1)?;
            let wanted = &values[0];
            let option = selector.options().find(|&option| {
                if selector.strings {
                    let name = model.localized(LocalizedStringRef(option.to_f64() as u16));
                    name == Some(wanted.as_str())
                } else {
                    shown(wanted).is_ok_and(|shown| {
                        (option.to_f64() * selector.unit.scale() - shown).abs() < 1e-9
                    })
                }
            });
            let options: Vec<String> = selector
                .options()
                .map(|option| option_name(model, &selector, option))
                .collect();
            vec![option.ok_or_else(|| format!("{wanted} is not one of {}", options.join(", ")))?]
        }
        (Shape::Utf8, _) => {
            let text = values.join(" ");
            if text.len() >= out.len() {
                return Err(format!("{} octets is too long", text.len()));
            }
            let mut encoded = text.into_bytes();
            encoded.push(0);
            return Ok(encoded);
        }
        _ => return Err(format!("control {index}'s values cannot be set here")),
    };
    let scalar = scalar.ok_or("no scalar")?;
    let length = encode_values(scalar, numbers, &mut out).ok_or("too many values")?;
    out.truncate(length);
    Ok(out)
}

/// A selector's option: its name for a string selector, else its value
/// in its unit.
fn option_name(model: &EntityModel, selector: &Selector<'_>, option: Number) -> String {
    if selector.strings {
        model
            .localized(LocalizedStringRef(option.to_f64() as u16))
            .map_or_else(|| format!("string {}", option.to_f64()), str::to_owned)
    } else {
        selector.unit.show(option).to_string()
    }
}

fn controls(interface: &str, entity: Option<EntityId>) -> std::io::Result<()> {
    let driver = read(interface, entity)?;
    let controller = driver.controller();
    let mut printed = 0;
    for found in controller.entities() {
        if entity.is_some_and(|entity_id| entity_id != found.entity_id()) {
            continue;
        }
        if printed > 0 {
            println!();
        }
        println!("{}", summary(found));
        printed += 1;
        let Some(model) = controller.model(found.entity_id()) else {
            println!("  no AEM model");
            continue;
        };
        let mut controls = model.controls().peekable();
        if controls.peek().is_none() {
            println!("  no controls");
        }
        for control in controls {
            print_control(model, &control);
        }
    }
    if printed == 0 {
        println!("no entity found");
    }
    driver.close()
}

/// A control's name and kind, then its values with their ranges.
fn print_control(model: &EntityModel, control: &ControlDescriptor<'_>) {
    let name = model
        .name_of(DescriptorType::CONTROL, control.index)
        .unwrap_or_default();
    let kind = control.control_type.name().map_or_else(
        || format!("vendor control {:#018x}", control.control_type.0),
        |kind| kind.to_lowercase().replace('_', " "),
    );
    let mut notes = Vec::new();
    if control.read_only {
        notes.push("read only");
    }
    if control.unknown {
        notes.push("value not known");
    }
    let notes = if notes.is_empty() {
        String::new()
    } else {
        format!(", {}", notes.join(", "))
    };
    println!("  control {:<3} \"{name}\"  {kind}{notes}", control.index);
    let range = |minimum: Number, maximum: Number, step: Number, unit: Unit| {
        format!(
            "from {} to {} in steps of {}",
            unit.show(minimum),
            unit.show(maximum),
            unit.show(step)
        )
    };
    match control.value_type.shape() {
        Shape::Linear => {
            for value in control.linear() {
                let label = model
                    .localized(value.string)
                    .map(|label| format!("{label}: "))
                    .unwrap_or_default();
                println!(
                    "    {label}{}  {}, default {}",
                    value.unit.show(value.current),
                    range(value.minimum, value.maximum, value.step, value.unit),
                    value.unit.show(value.default)
                );
            }
        }
        Shape::Selector => {
            if let Some(selector) = control.selector() {
                let options: Vec<String> = selector
                    .options()
                    .map(|option| option_name(model, &selector, option))
                    .collect();
                println!(
                    "    {}  of {}",
                    option_name(model, &selector, selector.current),
                    options.join(", ")
                );
            }
        }
        Shape::Array => {
            if let Some(array) = control.array() {
                let values: Vec<String> = array
                    .current()
                    .map(|value| array.unit.show(value).to_string())
                    .collect();
                println!(
                    "    {}  {}",
                    values.join(", "),
                    range(array.minimum, array.maximum, array.step, array.unit)
                );
            }
        }
        Shape::Utf8 => println!("    \"{}\"", control.text().unwrap_or_default()),
        Shape::Other => println!("    a {:?} value, not shown", control.value_type),
    }
}

fn maps(interface: &str, entity: Option<EntityId>) -> std::io::Result<()> {
    let driver = read(interface, entity)?;
    let controller = driver.controller();
    let mut printed = 0;
    for found in controller.entities() {
        if entity.is_some_and(|entity_id| entity_id != found.entity_id()) {
            continue;
        }
        if printed > 0 {
            println!();
        }
        println!("{}", summary(found));
        printed += 1;
        let Some(model) = controller.model(found.entity_id()) else {
            println!("  no AEM model");
            continue;
        };
        let mut ports = model
            .stream_ports(true)
            .chain(model.stream_ports(false))
            .peekable();
        if ports.peek().is_none() {
            println!("  no stream ports");
        }
        for port in ports {
            print_port_mappings(model, &port);
        }
    }
    if printed == 0 {
        println!("no entity found");
    }
    driver.close()
}

/// A stream port's clusters and mappings: from each stream channel to a
/// cluster channel on an input, the other way on an output.
fn print_port_mappings(model: &EntityModel, port: &atdecc::descriptor::StreamPortDescriptor) {
    let (side, stream_type, arrow) = if port.is_input() {
        ("input", DescriptorType::STREAM_INPUT, "->")
    } else {
        ("output", DescriptorType::STREAM_OUTPUT, "<-")
    };
    let clusters: BTreeMap<u16, String> = model
        .audio_clusters(port)
        .map(|(offset, cluster)| {
            let name = model
                .name_of(DescriptorType::AUDIO_CLUSTER, port.base_cluster + offset)
                .unwrap_or_default();
            let channels = cluster.channel_count;
            let plural = if channels == 1 { "" } else { "s" };
            (offset, format!("\"{name}\" ({channels} channel{plural})"))
        })
        .collect();
    let (kind, mappings): (&str, Option<Vec<AudioMapping>>) = if port.has_dynamic_mappings() {
        let read = model
            .dynamic_mappings(port.descriptor_type, port.index)
            .map(Iterator::collect);
        ("dynamic", read)
    } else {
        ("fixed", Some(model.static_mappings(port).collect()))
    };
    println!(
        "  stream port {side} {}, {} clusters, {kind} mappings",
        port.index, port.number_of_clusters
    );
    for (offset, cluster) in &clusters {
        println!("    cluster {offset}  {cluster}");
    }
    let Some(mut mappings) = mappings else {
        println!("    mappings not read");
        return;
    };
    if mappings.is_empty() {
        println!("    no mappings");
    }
    mappings.sort_by_key(|mapping| {
        (
            mapping.cluster_offset,
            mapping.cluster_channel,
            mapping.stream_index,
            mapping.stream_channel,
        )
    });
    for mapping in mappings {
        let stream = model
            .name_of(stream_type, mapping.stream_index)
            .map(|name| format!(" \"{name}\""))
            .unwrap_or_default();
        println!(
            "    stream {}{stream} channel {} {arrow} cluster {} channel {}",
            mapping.stream_index,
            mapping.stream_channel,
            mapping.cluster_offset,
            mapping.cluster_channel
        );
    }
}

/// Reads `entity`, or every entity, and prints its model.
fn describe(interface: &str, entity: Option<EntityId>) -> std::io::Result<()> {
    let driver = read(interface, entity)?;
    let controller = driver.controller();
    let mut printed = 0;
    for found in controller.entities() {
        if entity.is_some_and(|entity_id| entity_id != found.entity_id()) {
            continue;
        }
        if printed > 0 {
            println!();
        }
        print_entity(found, controller.model(found.entity_id()));
        printed += 1;
    }
    if printed == 0 {
        println!("no entity found");
    }
    driver.close()
}

fn print_entity(found: &DiscoveredEntity, model: Option<&EntityModel>) {
    println!("{}", summary(found));
    let Some(model) = model else {
        println!("  no AEM model");
        return;
    };
    match model.state {
        EnumerationState::Complete => {}
        state => println!("  state: {state:?}"),
    }
    if let Some(entity) = model.entity() {
        let text = |value: &str| {
            if value.is_empty() {
                "-".to_owned()
            } else {
                value.to_owned()
            }
        };
        println!("  name           {}", text(entity.entity_name));
        println!("  group          {}", text(entity.group_name));
        println!(
            "  vendor, model  {}, {}",
            model.localized(entity.vendor_name).unwrap_or("-"),
            model.localized(entity.model_name).unwrap_or("-")
        );
        println!("  firmware       {}", text(entity.firmware_version));
        println!("  serial         {}", text(entity.serial_number));
        println!(
            "  configuration  {} of {}{}",
            entity.current_configuration,
            entity.configurations_count,
            model
                .name_of(DescriptorType::CONFIGURATION, entity.current_configuration)
                .map(|name| format!(", \"{name}\""))
                .unwrap_or_default()
        );
        for configuration in model.configurations() {
            // Audio channels each way, and its media clock streams.
            let audio = |streams: &[atdecc::descriptor::StreamDescriptor<'_>]| {
                let channels: u16 = streams
                    .iter()
                    .filter(|stream| !stream.is_media_clock())
                    .filter_map(|stream| stream.current_format.channels())
                    .sum();
                let clocks = streams
                    .iter()
                    .filter(|stream| stream.is_media_clock())
                    .count();
                format!(
                    "{} streams, {channels} ch audio, {clocks} media clock",
                    streams.len()
                )
            };
            println!(
                "    {} {:2} \"{}\": in {}; out {}{}",
                if configuration.index == entity.current_configuration {
                    "*"
                } else {
                    " "
                },
                configuration.index,
                configuration.name,
                audio(&configuration.inputs),
                audio(&configuration.outputs),
                if configuration.read {
                    ""
                } else {
                    " (not all read)"
                }
            );
        }
    }
    match model.milan {
        Some(milan) => {
            let dotted = |version: [u8; 4]| {
                version
                    .iter()
                    .map(u8::to_string)
                    .collect::<Vec<_>>()
                    .join(".")
            };
            println!(
                "  Milan          specification {}, {}, features {:?}",
                milan
                    .specification_version
                    .map_or("before 1.3".to_owned(), dotted),
                if milan.is_certified() {
                    format!("certified {}", dotted(milan.certification_version))
                } else {
                    "not certified".to_owned()
                },
                milan.features.names().collect::<Vec<_>>()
            );
        }
        None => println!("  Milan          no"),
    }
    for unit in model.audio_units() {
        let rates: Vec<String> = unit
            .sampling_rates()
            .map(|rate| rate.base_frequency().to_string())
            .collect();
        println!(
            "  audio unit {}   {} Hz now, supports {}",
            unit.index,
            unit.current_sampling_rate.base_frequency(),
            rates.join(", ")
        );
    }
    for input in [true, false] {
        for stream in model.streams(input) {
            let name = model
                .name_of(stream.descriptor_type, stream.index)
                .unwrap_or("");
            println!(
                "  {} {:<3} {:<28} {}",
                if input { "input " } else { "output" },
                stream.index,
                format!("\"{name}\""),
                stream.current_format,
            );
            let mut state = Vec::new();
            if input {
                match model
                    .binding(stream.index)
                    .and_then(|binding| binding.talker_stream())
                {
                    Some((talker, output)) => {
                        state.push(format!("bound to {talker} output {output}"))
                    }
                    None if model.binding(stream.index).is_some() => {
                        state.push("not bound".to_owned())
                    }
                    None => {}
                }
            }
            if let Some(info) = model.stream_info(stream.descriptor_type, stream.index) {
                if input && info.bound() {
                    state.push(
                        if info.settled() {
                            "settled"
                        } else {
                            "not settled"
                        }
                        .to_owned(),
                    );
                    if info.registering() {
                        state.push(format!(
                            "talker heard, {} us accumulated latency",
                            info.msrp_accumulated_latency / 1000
                        ));
                    }
                }
                if info.talker_failed() {
                    state.push(format!("talker failed, code {}", info.msrp_failure_code));
                }
                if info.stream_id.is_valid() && info.settled() {
                    state.push(format!(
                        "stream {} to {}",
                        info.stream_id, info.stream_dest_mac
                    ));
                }
            }
            if !state.is_empty() {
                println!("{:16}{}", "", state.join(", "));
            }
            let counters = if input {
                model
                    .stream_input_counters(stream.index)
                    .map(|counters| stream_input_counters(&counters))
            } else {
                model
                    .stream_output_counters(stream.index)
                    .map(|counters| stream_output_counters(&counters))
            };
            if let Some(counters) = counters {
                println!("{:16}counters: {counters}", "");
            }
            // The formats the stream supports, in the hex the format
            // command takes, the current one marked.
            for format in stream.formats() {
                let mark = if format == stream.current_format {
                    "*"
                } else {
                    " "
                };
                println!("{:14}{mark} {:#018x}  {format}", "", format.0);
            }
        }
    }
    for interface in model.avb_interfaces() {
        println!(
            "  interface {}    \"{}\" {} clock {} {:?}",
            interface.index,
            model
                .name_of(DescriptorType::AVB_INTERFACE, interface.index)
                .unwrap_or(""),
            interface.mac_address,
            interface.clock_identity,
            interface.interface_flags.names().collect::<Vec<_>>()
        );
        print_network_state(model, interface.index);
    }
    for domain in model.clock_domains() {
        let source = model
            .name_of(DescriptorType::CLOCK_SOURCE, domain.clock_source_index)
            .unwrap_or("");
        println!(
            "  clock domain {} \"{}\" source {} \"{source}\" of {:?}",
            domain.index,
            model
                .name_of(DescriptorType::CLOCK_DOMAIN, domain.index)
                .unwrap_or(""),
            domain.clock_source_index,
            domain.clock_sources().collect::<Vec<_>>()
        );
        if let Some(counters) = model.clock_domain_counters(domain.index) {
            println!(
                "                counters: {}",
                counter_list(&[("locked", counters.locked), ("unlocked", counters.unlocked)])
            );
        }
    }
    for source in model.clock_sources() {
        let kind = match source.clock_source_type {
            ClockSourceType::INTERNAL => "internal".to_owned(),
            ClockSourceType::EXTERNAL => "external".to_owned(),
            ClockSourceType::INPUT_STREAM => {
                format!("from {:?} {}", source.location_type, source.location_index)
            }
            other => format!("{other:?}"),
        };
        println!(
            "  clock source {} \"{}\" {kind}",
            source.index,
            model
                .name_of(DescriptorType::CLOCK_SOURCE, source.index)
                .unwrap_or("")
        );
    }
    println!(
        "  {} descriptors read{}{}",
        model.descriptor_count(),
        if model.failed_reads > 0 {
            format!(", {} could not be", model.failed_reads)
        } else {
            String::new()
        },
        if model.registered {
            ", registered for notifications"
        } else {
            ""
        }
    );
}

/// The counters an entity keeps, by name, leaving out those it does not.
fn counter_list(counters: &[(&str, Option<u32>)]) -> String {
    let kept: Vec<String> = counters
        .iter()
        .filter_map(|(name, value)| value.map(|value| format!("{name} {value}")))
        .collect();
    if kept.is_empty() {
        "none kept".to_owned()
    } else {
        kept.join(", ")
    }
}

fn stream_input_counters(counters: &atdecc::aem::StreamInputCounters) -> String {
    counter_list(&[
        ("media locked", counters.media_locked),
        ("media unlocked", counters.media_unlocked),
        ("interrupted", counters.stream_interrupted),
        ("sequence mismatches", counters.seq_num_mismatch),
        ("media resets", counters.media_reset),
        ("timestamps uncertain", counters.timestamp_uncertain),
        ("timestamps valid", counters.timestamp_valid),
        ("timestamps not valid", counters.timestamp_not_valid),
        ("unsupported formats", counters.unsupported_format),
        ("late", counters.late_timestamp),
        ("early", counters.early_timestamp),
        ("frames in", counters.frames_rx),
    ])
}

fn stream_output_counters(counters: &atdecc::aem::StreamOutputCounters) -> String {
    counter_list(&[
        ("started", counters.stream_start),
        ("stopped", counters.stream_stop),
        ("interrupted", counters.stream_interrupted),
        ("media resets", counters.media_reset),
        ("timestamps uncertain", counters.timestamp_uncertain),
        ("timestamps valid", counters.timestamp_valid),
        ("timestamps not valid", counters.timestamp_not_valid),
        ("frames out", counters.frames_tx),
    ])
}

/// An AVB interface's gPTP state, path and counters, as far as the entity
/// reported them.
fn print_network_state(model: &EntityModel, index: u16) {
    let indent = format!("{:16}", "");
    match model.avb_info(index) {
        Some(info) => {
            let mut parts = vec![format!(
                "grandmaster {} domain {}",
                info.gptp_grandmaster_id, info.gptp_domain_number
            )];
            parts.push(format!("peer delay {} ns", info.propagation_delay));
            for (flag, name) in [
                (AvbInfoFlags::AS_CAPABLE, "asCapable"),
                (AvbInfoFlags::GPTP_ENABLED, "gPTP on"),
                (AvbInfoFlags::SRP_ENABLED, "SRP on"),
            ] {
                if info.flags.contains(flag) {
                    parts.push(name.to_owned());
                }
            }
            if info.flags.contains(AvbInfoFlags::AVTP_DOWN_VALID) {
                parts.push(
                    if info.flags.contains(AvbInfoFlags::AVTP_DOWN) {
                        "AVTP down"
                    } else {
                        "AVTP up"
                    }
                    .to_owned(),
                );
            }
            for mapping in info.msrp_mappings() {
                parts.push(format!(
                    "class {} on priority {} VLAN {}",
                    mapping.traffic_class, mapping.priority, mapping.vlan_id
                ));
            }
            println!("{indent}{}", parts.join(", "));
        }
        None => println!("{indent}no gPTP state reported"),
    }
    match model.as_path(index) {
        Some(path) => println!(
            "{indent}path {}",
            path.iter()
                .map(ClockIdentity::to_string)
                .collect::<Vec<_>>()
                .join(" > ")
        ),
        None => println!("{indent}no gPTP path reported"),
    }
    match model
        .counters(DescriptorType::AVB_INTERFACE, index)
        .and_then(|counters| counters.avb_interface())
    {
        Some(counters) => {
            let parts: Vec<String> = [
                ("link up", counters.link_up),
                ("link down", counters.link_down),
                ("frames out", counters.frames_tx),
                ("frames in", counters.frames_rx),
                ("CRC errors", counters.rx_crc_error),
                ("grandmaster changes", counters.gptp_gm_changed),
            ]
            .into_iter()
            .filter_map(|(name, value)| value.map(|value| format!("{name} {value}")))
            .collect();
            println!("{indent}counters: {}", parts.join(", "));
        }
        None => println!("{indent}no counters reported"),
    }
}

/// An entity's AVB interface, as a leaf of the network.
struct Leaf {
    entity_id: EntityId,
    name: String,
    interface: u16,
    /// Peer delay to the neighbor, in nanoseconds.
    delay: Option<u32>,
    link_downs: Option<u32>,
    /// A Wi-Fi station's hop to its access point: its time and signal.
    wifi: Option<String>,
}

/// Reads every entity and prints the gPTP tree their paths make: the
/// grandmaster, the bridges under it and the entities under those.
fn network(interface: &str) -> std::io::Result<()> {
    // Listening while the entities are read, as the bridge sends a peer
    // delay request each second.
    let listener = NeighborListener::open(interface);
    let driver = read(interface, None)?;
    let host = match listener {
        Ok(mut listener) => Host::Heard(listener.neighbor()),
        Err(error) => Host::CannotListen(error.to_string()),
    };
    let controller = driver.controller();
    // Each entity interface's clock identity, to tell entities from
    // bridges in the paths.
    let mut owners: BTreeMap<ClockIdentity, (EntityId, u16)> = BTreeMap::new();
    for found in controller.entities() {
        if let Some(model) = controller.model(found.entity_id()) {
            for avb_interface in model.avb_interfaces() {
                owners.insert(
                    avb_interface.clock_identity,
                    (found.entity_id(), avb_interface.index),
                );
            }
        }
    }
    // The bridges whose port a Wi-Fi station follows.
    let access_points: BTreeSet<ClockIdentity> = controller
        .entities()
        .filter_map(|found| controller.model(found.entity_id()))
        .flat_map(|model| model.wireless_interfaces())
        .map(|(_, wireless)| wireless.status.ap_clock)
        .collect();
    let mut roots: BTreeSet<ClockIdentity> = BTreeSet::new();
    let mut children: BTreeMap<ClockIdentity, BTreeSet<ClockIdentity>> = BTreeMap::new();
    let mut leaves: BTreeMap<Option<ClockIdentity>, Vec<Leaf>> = BTreeMap::new();
    // Entities whose AVB interfaces are not known.
    let mut unknown: Vec<String> = Vec::new();
    for found in controller.entities() {
        let entity_id = found.entity_id();
        let model = controller.model(entity_id);
        let name = model
            .and_then(EntityModel::entity_name)
            .unwrap_or("unnamed")
            .to_owned();
        let Some(model) = model.filter(|model| model.avb_interfaces().next().is_some()) else {
            unknown.push(format!("\"{name}\" {entity_id}"));
            continue;
        };
        for avb_interface in model.avb_interfaces() {
            let index = avb_interface.index;
            let reported = model.as_path(index);
            let mut path: Vec<ClockIdentity> = reported.unwrap_or_default().to_vec();
            // The path ends at the neighbor; some entities add themselves.
            if path.last() == Some(&avb_interface.clock_identity) {
                path.pop();
            }
            if reported.is_some() && path.is_empty() {
                // The entity is the grandmaster.
                roots.insert(avb_interface.clock_identity);
                continue;
            }
            if let Some(first) = path.first() {
                roots.insert(*first);
            }
            for pair in path.windows(2) {
                children.entry(pair[0]).or_default().insert(pair[1]);
            }
            let counters = model
                .counters(DescriptorType::AVB_INTERFACE, index)
                .and_then(|counters| counters.avb_interface());
            let station = model
                .wireless(index)
                .map(|wireless| wireless.status)
                .filter(|status| !status.access_point());
            let wifi = station.map(|status| {
                let time = if status.flags.contains(WirelessFlags::LOCKED) {
                    "time locked"
                } else if status.flags.contains(WirelessFlags::HOLDOVER) {
                    "holding over"
                } else {
                    "time not locked"
                };
                match status.signal() {
                    Some(rssi) => format!("Wi-Fi, {time}, {rssi} dBm"),
                    None => format!("Wi-Fi, {time}"),
                }
            });
            leaves.entry(path.last().copied()).or_default().push(Leaf {
                entity_id,
                name: name.clone(),
                interface: index,
                // A Wi-Fi hop's delay of 0 is one not measured.
                delay: model
                    .avb_info(index)
                    .map(|info| info.propagation_delay)
                    .filter(|&delay| station.is_none() || delay > 0),
                link_downs: counters.and_then(|counters| counters.link_down),
                wifi,
            });
        }
    }
    println!(
        "network on {interface} as gPTP paths show it, {} entities\n",
        controller.entities().count()
    );
    let tree = Tree {
        controller,
        access_points: &access_points,
        owners: &owners,
        children: &children,
        leaves: &leaves,
        host: match host {
            Host::Heard(neighbor) => neighbor,
            Host::CannotListen(_) => None,
        },
    };
    for root in &roots {
        tree.print(*root, 0);
    }
    let placed = |clock: ClockIdentity| {
        roots.contains(&clock) || children.values().any(|below| below.contains(&clock))
    };
    match &host {
        Host::Heard(Some(neighbor)) if placed(neighbor.clock) => {}
        Host::Heard(Some(neighbor)) => println!(
            "this computer  port {} of bridge {}, {}",
            neighbor.port,
            neighbor.clock,
            synced(neighbor.synced)
        ),
        Host::Heard(None) => println!("this computer  no bridge heard on {interface}"),
        Host::CannotListen(reason) => {
            println!("this computer  cannot listen for gPTP on {interface}: {reason}");
        }
    }
    if let Some(unplaced) = leaves.get(&None) {
        println!("not placed, no gPTP path reported:");
        for leaf in unplaced {
            print_leaf(leaf, 1);
        }
    }
    if !unknown.is_empty() {
        println!("not placed, no AVB interface read:");
        for line in unknown {
            println!("  {line}");
        }
    }
    driver.close()
}

/// What this computer heard of the bridge it is plugged into.
enum Host {
    Heard(Option<Neighbor>),
    CannotListen(String),
}

fn synced(synced: bool) -> &'static str {
    if synced { "synced" } else { "not synced" }
}

/// The gPTP tree the entities' paths make, to print.
struct Tree<'a> {
    controller: &'a Controller,
    access_points: &'a BTreeSet<ClockIdentity>,
    owners: &'a BTreeMap<ClockIdentity, (EntityId, u16)>,
    children: &'a BTreeMap<ClockIdentity, BTreeSet<ClockIdentity>>,
    leaves: &'a BTreeMap<Option<ClockIdentity>, Vec<Leaf>>,
    host: Option<Neighbor>,
}

impl Tree<'_> {
    fn print(&self, node: ClockIdentity, depth: usize) {
        let indent = "  ".repeat(depth);
        let what = match self.owners.get(&node) {
            Some((entity_id, index)) => {
                let name = self
                    .controller
                    .model(*entity_id)
                    .and_then(EntityModel::entity_name)
                    .unwrap_or("unnamed");
                format!("entity \"{name}\" {entity_id} interface {index}")
            }
            None if self.access_points.contains(&node) => "bridge, Wi-Fi access point".to_owned(),
            None => "bridge".to_owned(),
        };
        let role = if depth == 0 { "grandmaster, " } else { "" };
        println!("{indent}{node}  {role}{what}");
        for child in self.children.get(&node).into_iter().flatten() {
            self.print(*child, depth + 1);
        }
        for leaf in self.leaves.get(&Some(node)).into_iter().flatten() {
            print_leaf(leaf, depth + 1);
        }
        if let Some(neighbor) = self.host.filter(|neighbor| neighbor.clock == node) {
            println!(
                "{indent}  this computer  bridge port {}, {}",
                neighbor.port,
                synced(neighbor.synced)
            );
        }
    }
}

fn print_leaf(leaf: &Leaf, depth: usize) {
    let mut details = vec![format!("interface {}", leaf.interface)];
    details.extend(leaf.wifi.clone());
    if let Some(delay) = leaf.delay {
        details.push(format!("peer delay {delay} ns"));
    }
    if let Some(downs) = leaf.link_downs {
        details.push(format!("link down {downs} times"));
    }
    println!(
        "{}\"{}\" {}  {}",
        "  ".repeat(depth),
        leaf.name,
        leaf.entity_id,
        details.join(", ")
    );
}

/// One line about an entity: its ID, address, roles and clock.
fn summary(entity: &DiscoveredEntity) -> String {
    let adp = &entity.adp;
    let mut roles = Vec::new();
    if adp
        .talker_capabilities
        .contains(TalkerCapabilities::IMPLEMENTED)
    {
        roles.push(format!("talker {}", adp.talker_stream_sources));
    }
    if adp
        .listener_capabilities
        .contains(ListenerCapabilities::IMPLEMENTED)
    {
        roles.push(format!("listener {}", adp.listener_stream_sinks));
    }
    if adp
        .controller_capabilities
        .contains(ControllerCapabilities::IMPLEMENTED)
    {
        roles.push("controller".to_owned());
    }
    let mut classes = Vec::new();
    if adp
        .entity_capabilities
        .contains(EntityCapabilities::CLASS_A_SUPPORTED)
    {
        classes.push("A");
    }
    if adp
        .entity_capabilities
        .contains(EntityCapabilities::CLASS_B_SUPPORTED)
    {
        classes.push("B");
    }
    let clock = if adp
        .entity_capabilities
        .contains(EntityCapabilities::GPTP_SUPPORTED)
    {
        format!(
            "grandmaster {} domain {}",
            adp.gptp_grandmaster_id, adp.gptp_domain_number
        )
    } else {
        "no gPTP".to_owned()
    };
    format!(
        "{}  {}  {}  class {}  {}  model {}",
        adp.entity_id,
        entity.mac,
        if roles.is_empty() {
            "no roles".to_owned()
        } else {
            roles.join(", ")
        },
        if classes.is_empty() {
            "-".to_owned()
        } else {
            classes.join("/")
        },
        clock,
        adp.entity_model_id,
    )
}
