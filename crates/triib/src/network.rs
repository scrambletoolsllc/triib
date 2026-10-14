//! The network thread: one per chosen interface, running the ATDECC
//! controller and posting what it learns to the app.

use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

use atdecc::aem::{AudioMapping, MappingChange, NAME_LENGTH, aem_name};
use atdecc::blocking::{Driver, Frame};
use atdecc::controller::{Advertise, CommandId, Outcome};
use atdecc::descriptor::{DescriptorType, SamplingRate};
use atdecc::model::EntityModel;
pub use atdecc::neighbor::Neighbor;
use atdecc::neighbor::NeighborListener;
use atdecc::stream_format::StreamFormat;
use atdecc::{DiscoveredEntity, EntityId, EntityModelId, Event, OfflineReason};

use crate::External;

/// The longest the thread waits before looking for commands and the stop
/// flag.
const TURN: Duration = Duration::from_millis(200);

/// triib's controller entity model ID, under the Scramble Tools MA-S
/// `8C-1F-64-36-C`.
pub const ENTITY_MODEL_ID: EntityModelId = EntityModelId(0x8c1f_6436_c000_0001);

/// How long other controllers keep triib after its last advertisement, in
/// units of 2 s: the standard's default of 62 s.
const VALID_TIME: u8 = 31;

/// What the network thread reports, tagged with the generation of the
/// thread that sent it so reports from a replaced thread can be ignored.
#[derive(Debug, Clone)]
pub struct Report {
    pub generation: u64,
    pub kind: ReportKind,
}

#[derive(Debug, Clone)]
pub enum ReportKind {
    Started {
        controller: EntityId,
    },
    Failed(Failure),
    Online(DiscoveredEntity),
    Changed(DiscoveredEntity),
    Restarted(DiscoveredEntity),
    Offline(EntityId, OfflineReason),
    /// What has been read of an entity, sent when reading starts, ends or
    /// fails, and when a notification changes it.
    Model(EntityId, Box<EntityModel>),
    /// An action the app asked for finished.
    Finished(Action, Outcome),
    /// The bridge this computer is plugged into, as its gPTP messages show,
    /// or none heard.
    Neighbor(Option<Neighbor>),
    /// This computer cannot listen for gPTP on the interface, and why.
    CannotListen(String),
    /// ATDECC frames sent and received since the last report, oldest
    /// first.
    Frames(Vec<Frame>),
}

/// Something the app asks the network to do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    /// Bind a listener's stream input to a talker's stream output.
    Connect {
        talker: (EntityId, u16),
        listener: (EntityId, u16),
    },
    /// Unbind a listener's stream input.
    Disconnect { listener: (EntityId, u16) },
    /// Make an entity identify itself for a few seconds.
    Identify(EntityId),
    /// Make another of an entity's configurations its current one.
    SetConfiguration {
        entity: EntityId,
        configuration: u16,
    },
    /// Rename an entity, its group or one of its descriptors.
    Rename { target: NameTarget, name: Name },
    /// Change a stream input's or output's format.
    SetStreamFormat {
        entity: EntityId,
        descriptor_type: DescriptorType,
        index: u16,
        format: StreamFormat,
    },
    /// Change an audio unit's sampling rate.
    SetSamplingRate {
        entity: EntityId,
        unit: u16,
        rate: SamplingRate,
    },
    /// Pick a clock domain's clock source.
    SetClockSource {
        entity: EntityId,
        domain: u16,
        source: u16,
    },
    /// Add or remove one of a stream port's dynamic mappings.
    Map {
        entity: EntityId,
        port: (DescriptorType, u16),
        change: MappingChange,
        mapping: AudioMapping,
    },
    /// Set a control's current values.
    SetControl {
        entity: EntityId,
        index: u16,
        values: ControlValues,
    },
    /// Set how far ahead of sending a stream output's presentation times
    /// are (Milan's SET_MAX_TRANSIT_TIME), in nanoseconds.
    SetMaxTransitTime {
        entity: EntityId,
        output: u16,
        nanoseconds: u64,
    },
}

/// The most octets of a control's values an action carries: a linear
/// control's at most (IEEE 1722.1-2021, Table 7-39).
const CONTROL_VALUES: usize = 72;

/// A control's new values, as SET_CONTROL carries them.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ControlValues {
    octets: [u8; CONTROL_VALUES],
    length: u8,
}

impl ControlValues {
    /// `octets` as values, or `None` when there are none or too many.
    pub fn new(octets: &[u8]) -> Option<Self> {
        if octets.is_empty() || octets.len() > CONTROL_VALUES {
            return None;
        }
        let mut values = Self {
            octets: [0; CONTROL_VALUES],
            length: octets.len() as u8,
        };
        values.octets[..octets.len()].copy_from_slice(octets);
        Some(values)
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.octets[..usize::from(self.length)]
    }
}

impl std::fmt::Debug for ControlValues {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "ControlValues({:02x?})", self.as_slice())
    }
}

/// A name an entity holds: for the entity, `name_index` 0 is its name and
/// 1 its group; for its other descriptors 0 is their name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NameTarget {
    pub entity: EntityId,
    pub descriptor_type: DescriptorType,
    pub index: u16,
    pub name_index: u16,
}

/// A name as an entity holds it: up to 64 octets of UTF-8.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Name([u8; NAME_LENGTH]);

impl Name {
    /// `text` as a name, or `None` when it is longer than an entity holds.
    pub fn new(text: &str) -> Option<Self> {
        aem_name(text).map(Self)
    }

    pub fn as_str(&self) -> &str {
        let end = self
            .0
            .iter()
            .position(|&octet| octet == 0)
            .unwrap_or(NAME_LENGTH);
        std::str::from_utf8(&self.0[..end]).unwrap_or_default()
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.as_str())
    }
}

/// How long an entity identifies itself.
const IDENTIFY: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Failure {
    pub message: String,
    /// What gets raw Ethernet going again, when it is out of reach.
    pub remedy: Option<Remedy>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Remedy {
    /// A command that grants the missing permission, to copy.
    Command(String),
    /// Npcap is missing, from here.
    GetNpcap(String),
    /// Run as administrator, or set the system up again.
    Permission,
}

impl Failure {
    fn from_io(error: &io::Error) -> Self {
        let program = || {
            std::env::current_exe()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|_| "triib".to_owned())
        };
        let (message, remedy) = match avb_net::Blocked::of(error) {
            Some(avb_net::Blocked::RawCapability) => (
                crate::fl!("network-permission"),
                Remedy::Command(format!("sudo setcap cap_net_raw+ep {}", program())),
            ),
            // Until the next restart; the installer's helper keeps it.
            Some(avb_net::Blocked::CaptureDevices) => (
                crate::fl!("network-permission"),
                Remedy::Command(format!(
                    "sudo chown {} /dev/bpf*",
                    std::env::var("USER").unwrap_or_else(|_| "$USER".to_owned())
                )),
            ),
            Some(avb_net::Blocked::Npcap) => (
                crate::fl!("network-needs-npcap"),
                Remedy::GetNpcap(NPCAP_DOWNLOAD.to_owned()),
            ),
            Some(avb_net::Blocked::NpcapAdministrators) => (
                crate::fl!("network-npcap-administrators"),
                Remedy::Permission,
            ),
            _ => {
                return Self {
                    message: error.to_string(),
                    remedy: None,
                };
            }
        };
        Self {
            message,
            remedy: Some(remedy),
        }
    }
}

/// Where Npcap's installer is.
const NPCAP_DOWNLOAD: &str = "https://npcap.com/#download";

enum Command {
    Discover,
    Act(Action),
}

/// A running network thread. Dropping it stops the thread.
pub struct Network {
    stop: Arc<AtomicBool>,
    commands: Sender<Command>,
}

impl Network {
    /// Starts reading `interface`, first forgetting the entity models kept
    /// between runs when `fresh`.
    pub fn start(interface: String, generation: u64, fresh: bool) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let (commands, received) = mpsc::channel();
        let stopping = stop.clone();
        let spawned = std::thread::Builder::new()
            .name(format!("atdecc-{interface}"))
            .spawn(move || run(&interface, generation, fresh, &stopping, &received));
        if let Err(error) = spawned {
            report(generation, ReportKind::Failed(Failure::from_io(&error)));
        }
        Self { stop, commands }
    }

    /// Asks every entity to advertise itself now.
    pub fn discover(&self) {
        let _ = self.commands.send(Command::Discover);
    }

    /// Carries out `action`, reporting how it went.
    pub fn act(&self, action: Action) {
        let _ = self.commands.send(Command::Act(action));
    }
}

impl Drop for Network {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

/// The most frames waiting between two turns of the loop, the oldest
/// dropped past it.
const FRAMES_WAITING: usize = 4096;

fn report(generation: u64, kind: ReportKind) {
    crate::post(External::Network(Report { generation, kind }));
}

fn run(
    interface: &str,
    generation: u64,
    fresh: bool,
    stop: &AtomicBool,
    commands: &Receiver<Command>,
) {
    let opened = Driver::open_with(interface, |config| {
        config.advertise = Some(Advertise {
            entity_model_id: ENTITY_MODEL_ID,
            valid_time: VALID_TIME,
        });
    });
    let mut driver = match opened {
        Ok(driver) => driver,
        Err(error) => {
            report(generation, ReportKind::Failed(Failure::from_io(&error)));
            return;
        }
    };
    driver.keep_frames(FRAMES_WAITING);
    report(
        generation,
        ReportKind::Started {
            controller: driver.controller().entity_id(),
        },
    );
    if fresh {
        models::clear();
    }
    for model in models::load() {
        driver.controller_mut().remember_model(model);
    }
    driver.controller_mut().discover(None);
    let mut actions: HashMap<CommandId, Action> = HashMap::new();
    // Without it the network view only lacks this computer's place.
    let mut gptp = match NeighborListener::open(interface) {
        Ok(listener) => Some(listener),
        Err(error) => {
            report(generation, ReportKind::CannotListen(error.to_string()));
            None
        }
    };
    loop {
        if stop.load(Ordering::Relaxed) {
            // Tell other controllers triib is leaving.
            let _ = driver.close();
            return;
        }
        while let Ok(command) = commands.try_recv() {
            match command {
                Command::Discover => driver.controller_mut().discover(None),
                Command::Act(action) => {
                    let now = driver.now();
                    let controller = driver.controller_mut();
                    let command = match action {
                        Action::Connect { talker, listener } => {
                            controller.connect(now, talker, listener)
                        }
                        Action::Disconnect { listener } => controller.disconnect(now, listener),
                        Action::Identify(entity_id) => {
                            controller.identify(now, entity_id, IDENTIFY)
                        }
                        Action::SetConfiguration {
                            entity,
                            configuration,
                        } => controller.set_configuration(now, entity, configuration),
                        Action::Rename { target, name } => controller.set_name(
                            now,
                            target.entity,
                            target.descriptor_type,
                            target.index,
                            target.name_index,
                            name.as_str(),
                        ),
                        Action::SetStreamFormat {
                            entity,
                            descriptor_type,
                            index,
                            format,
                        } => controller.set_stream_format(
                            now,
                            entity,
                            descriptor_type,
                            index,
                            format,
                        ),
                        Action::SetSamplingRate { entity, unit, rate } => controller
                            .set_sampling_rate(now, entity, DescriptorType::AUDIO_UNIT, unit, rate),
                        Action::SetClockSource {
                            entity,
                            domain,
                            source,
                        } => controller.set_clock_source(now, entity, domain, source),
                        Action::Map {
                            entity,
                            port,
                            change: MappingChange::Add,
                            mapping,
                        } => controller.add_audio_mappings(now, entity, port, &[mapping]),
                        Action::Map {
                            entity,
                            port,
                            change: MappingChange::Remove,
                            mapping,
                        } => controller.remove_audio_mappings(now, entity, port, &[mapping]),
                        Action::SetControl {
                            entity,
                            index,
                            values,
                        } => controller.set_control(now, entity, index, values.as_slice()),
                        Action::SetMaxTransitTime {
                            entity,
                            output,
                            nanoseconds,
                        } => controller.set_max_transit_time(now, entity, output, nanoseconds),
                    };
                    actions.insert(command, action);
                }
            }
        }
        if let Err(error) = driver.turn(TURN) {
            report(generation, ReportKind::Failed(Failure::from_io(&error)));
            return;
        }
        if let Some(gptp) = &mut gptp
            && let Some(neighbor) = gptp.poll()
        {
            report(generation, ReportKind::Neighbor(neighbor));
        }
        let frames: Vec<Frame> = std::iter::from_fn(|| driver.poll_frame()).collect();
        if !frames.is_empty() {
            report(generation, ReportKind::Frames(frames));
        }
        while let Some(event) = driver.controller_mut().poll_event() {
            let controller = driver.controller();
            let snapshot = |entity_id| controller.entity(entity_id).copied();
            let kind = match event {
                Event::EntityOnline(entity_id) => snapshot(entity_id).map(ReportKind::Online),
                Event::EntityChanged(entity_id) => snapshot(entity_id).map(ReportKind::Changed),
                Event::EntityRestarted(entity_id) => snapshot(entity_id).map(ReportKind::Restarted),
                Event::EntityOffline(entity_id, reason) => {
                    Some(ReportKind::Offline(entity_id, reason))
                }
                Event::CommandFinished(command, outcome) => actions
                    .remove(&command)
                    .map(|action| ReportKind::Finished(action, outcome)),
                Event::EntityEnumerated(entity_id) => {
                    // Read in full: kept, so entities of its model read
                    // faster from here on and next time.
                    if let Some(model) = controller.static_model(entity_id) {
                        models::save(&model);
                        driver.controller_mut().remember_model(model);
                    }
                    let controller = driver.controller();
                    controller
                        .model(entity_id)
                        .map(|model| ReportKind::Model(entity_id, Box::new(model.clone())))
                }
                Event::EnumerationStarted(entity_id)
                | Event::EnumerationFailed(entity_id, _)
                | Event::EntityModelChanged(entity_id) => controller
                    .model(entity_id)
                    .map(|model| ReportKind::Model(entity_id, Box::new(model.clone()))),
            };
            if let Some(kind) = kind {
                report(generation, kind);
            }
        }
    }
}

/// Entity models kept between runs in the cache folder, one file each.
mod models {
    use std::path::PathBuf;

    use atdecc::cache::{CachedModel, ModelKey};

    fn folder() -> Option<PathBuf> {
        triib_store::paths::cache_dir().map(|dir| dir.join("models"))
    }

    /// The file a model is kept in, named by what it is for.
    fn file(key: &ModelKey) -> Option<PathBuf> {
        // FNV-1a, which stays the same between builds, unlike the hasher
        // the standard library picks.
        let firmware = key
            .firmware
            .bytes()
            .fold(0xcbf2_9ce4_8422_2325u64, |hash, octet| {
                (hash ^ u64::from(octet)).wrapping_mul(0x0100_0000_01b3)
            });
        folder().map(|folder| {
            folder.join(format!(
                "{:016x}-{}-{firmware:016x}.aem",
                key.entity_model_id.0, key.configuration
            ))
        })
    }

    /// Every model kept, leaving out files that do not read as one.
    pub fn load() -> Vec<CachedModel> {
        let Some(entries) = folder().and_then(|folder| std::fs::read_dir(folder).ok()) else {
            return Vec::new();
        };
        entries
            .flatten()
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .is_some_and(|extension| extension == "aem")
            })
            .filter_map(|entry| std::fs::read(entry.path()).ok())
            .filter_map(|bytes| CachedModel::decode(&bytes).ok())
            .collect()
    }

    /// Forgets every model kept.
    pub fn clear() {
        let Some(entries) = folder().and_then(|folder| std::fs::read_dir(folder).ok()) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|extension| extension == "aem")
                && let Err(error) = std::fs::remove_file(&path)
            {
                eprintln!("triib: could not forget {}: {error}", path.display());
            }
        }
    }

    /// Keeps a model, replacing one kept for the same entities.
    pub fn save(model: &CachedModel) {
        let Some(path) = file(&model.key) else {
            return;
        };
        if let Some(folder) = path.parent()
            && std::fs::create_dir_all(folder).is_err()
        {
            return;
        }
        if let Err(error) = triib_store::atomic::write(&path, &model.encode()) {
            eprintln!("triib: could not keep {}: {error}", path.display());
        }
    }
}
