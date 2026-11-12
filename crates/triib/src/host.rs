//! This computer's own entities, which avb-endpointd runs, one daemon per
//! interface: adding, changing and removing them and their streams in its
//! `<interface>.toml`, under the file's lock, starting its unit, and what
//! it says each is doing.
//!
//! Beside triib's entities the daemon runs the interface's default entity,
//! this computer on the network for any app, such as plaa. triib shows it
//! with the others, but neither removes it nor changes its streams, and
//! leaves the daemon running for it when triib's own entities are gone.

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

use atdecc::{EntityId, MacAddress};
use avb_endpointd::config::{self, Config, Direction, EndpointConfig, StreamConfig};
use avb_endpointd::status::{self, DaemonStatus, EndpointStatus};

/// Whether this system runs entities of its own.
pub const SUPPORTED: bool = cfg!(target_os = "linux");
/// A preset waits for this computer's endpoints, so the app hears what
/// the daemon says every time it looks, changed or not, to keep its
/// deadline.
pub static RECALLING: AtomicBool = AtomicBool::new(false);

/// How long a daemon just started has to say it runs, before the app
/// would start it again.
const STARTING: Duration = Duration::from_secs(5);
/// The longest the app waits to start the daemon again after starts that
/// did not last.
const LONGEST_WAIT: Duration = Duration::from_secs(5 * 60);

/// The interface whose entities the app shows, and reads the daemon's
/// status and meters for.
static SHOWN: Mutex<Option<String>> = Mutex::new(None);

/// The interface whose entities the app keeps running, while it shows one
/// that runs entities of this computer's.
static KEPT: Mutex<Option<String>> = Mutex::new(None);

/// triib's entities file from before avb-endpointd ran them, which a
/// person may have moved: what an interface without a file of its own
/// starts from.
static OLD_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);

/// When the app last started the daemon, and how long it waits to start
/// it again by itself: `STARTING`, and twice as long after each start
/// that did not last, up to `LONGEST_WAIT`, as one failing at once would.
struct Starts {
    at: Option<Instant>,
    wait: Duration,
}

static STARTS: Mutex<Starts> = Mutex::new(Starts {
    at: None,
    wait: STARTING,
});

/// The interface the app shows, or none.
pub fn show(interface: Option<&str>) {
    let mut shown = SHOWN.lock().unwrap_or_else(PoisonError::into_inner);
    if shown.as_deref() != interface {
        *shown = interface.map(str::to_owned);
    }
}

/// The interface the app shows.
pub fn shown() -> Option<String> {
    SHOWN.lock().unwrap_or_else(PoisonError::into_inner).clone()
}

/// What the daemon on the interface shown says, while it runs.
pub fn status() -> Option<DaemonStatus> {
    status::read(&shown()?)
}

/// Where triib kept this computer's entities before avb-endpointd ran
/// them, as the settings named it.
pub fn set_old_file(file: Option<PathBuf>) {
    *OLD_FILE.lock().unwrap_or_else(PoisonError::into_inner) = file;
}

/// Stops a triib-endpointd an older triib left running for this person,
/// which would hold its interface against avb-endpointd: found by the
/// status it wrote, and its process's name.
pub fn stop_old_daemon() {
    #[cfg(target_os = "linux")]
    {
        if cfg!(test) {
            return;
        }
        let Some(path) = triib_store::paths::runtime_dir().map(|dir| dir.join("endpointd.toml"))
        else {
            return;
        };
        let Ok(text) = std::fs::read_to_string(path) else {
            return;
        };
        let Some(pid) = text
            .lines()
            .find_map(|line| line.strip_prefix("pid = ")?.trim().parse::<u32>().ok())
        else {
            return;
        };
        let named = std::fs::read_to_string(format!("/proc/{pid}/comm"))
            .is_ok_and(|name| name.trim() == "triib-endpointd");
        if named {
            eprintln!("triib: stopping triib-endpointd, as avb-endpointd runs the entities now");
            let _ = std::process::Command::new("kill")
                .arg("-TERM")
                .arg(pid.to_string())
                .status();
        }
    }
}

/// Where the entities on `interface` are listed.
pub fn entities_path(interface: &str) -> Option<PathBuf> {
    config::path(interface)
}

/// The entities on `interface`: its file's, or with none yet triib's
/// from before, which the daemon takes too as it starts. For reading; a
/// change goes through `edit`.
fn current(interface: &str) -> Result<(PathBuf, Config), String> {
    let path = entities_path(interface).ok_or("no configuration folder")?;
    let config = match config::load(&path) {
        Ok(config) => config,
        Err(error) if error.is_missing() => old_entities(interface).unwrap_or_default(),
        Err(error) => return Err(format!("{}: {error}", path.display())),
    };
    Ok((path, config))
}

/// triib's entities from before for `interface`: from the file the
/// settings named, else from triib's data folder.
fn old_entities(interface: &str) -> Option<Config> {
    let file = OLD_FILE
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone();
    file.as_deref()
        .and_then(|file| config::triib_entities_at(file, interface))
        .or_else(|| config::triib_entities(interface))
}

/// triib's model, which triib's package installs for the daemon: its
/// entities' vendor, model name, version and logos.
const MODEL: &str = "triib";

/// Changes the entities on `interface` under the file's lock, so neither
/// another app's change nor the daemon's in between is lost: with no file
/// yet, from triib's entities from before.
fn edit<R>(
    interface: &str,
    change: impl FnOnce(&mut Config) -> Result<R, String>,
) -> Result<R, String> {
    let path = entities_path(interface).ok_or("no configuration folder")?;
    config::edit_at(&path, |config| {
        if !path.exists() {
            *config = old_entities(interface).unwrap_or_default();
        }
        change(config)
    })
    .map_err(|error| format!("{}: {error}", path.display()))?
}

/// triib's own entities among `endpoints`: all but the default entity.
fn own(endpoints: &[EndpointConfig]) -> impl Iterator<Item = &EndpointConfig> {
    endpoints.iter().filter(|endpoint| !endpoint.is_default)
}

/// The place in the file of the endpoint whose entity is `entity_id`.
fn place_of(config: &Config, mac: MacAddress, entity_id: EntityId) -> Option<usize> {
    config
        .endpoints
        .iter()
        .enumerate()
        .position(|(place, endpoint)| {
            avb_endpointd::entity_id(mac, endpoint.instance_at(place)) == entity_id
        })
}

/// One of this computer's entities added this session that triib has yet
/// to hear on the network, which the entity list shows starting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Starting {
    pub interface: String,
    pub entity_id: EntityId,
    pub name: String,
}

/// Adds an entity named `name`, with a stream each way, each with its own
/// virtual audio device; its inspector adds more and sets each up. Gives
/// the instance its entity ID is made from.
pub fn add(interface: &str, name: String) -> Result<u8, String> {
    edit(interface, |config| {
        config.pin_instances();
        let free = config.free_instance().ok_or("no room")?;
        config.endpoints.push(EndpointConfig {
            model: Some(MODEL.into()),
            ..EndpointConfig::new(name, Some(free))
        });
        Ok(free)
    })
}

/// Removes the endpoint whose entity is `entity_id`; the others keep
/// their entity IDs. The daemon goes on running the default entity, for
/// other apps, when triib's are all gone.
pub fn remove(interface: &str, mac: MacAddress, entity_id: EntityId) -> Result<(), String> {
    edit(interface, |config| {
        let place = place_of(config, mac, entity_id).ok_or("not one of this computer's")?;
        if config.endpoints[place].is_default {
            return Err("the default entity stays, for other apps".into());
        }
        config.pin_instances();
        config.endpoints.remove(place);
        Ok(())
    })
}

/// A change to one of an entity's streams.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamChange {
    /// A new stream of this way at the end: audio, with its own virtual
    /// device, or a media clock.
    Add { direction: Direction, clock: bool },
    /// The stream at this place goes.
    Remove(usize),
    /// The stream at this place has this many channels.
    Channels(usize, u16),
    /// The stream at this place takes its audio from or plays it to this:
    /// "virtual", "silence", "tone", "discard", "default" or a device's
    /// name.
    Audio(usize, String),
}

/// Changes one of an entity's streams.
pub fn change_stream(
    interface: &str,
    mac: MacAddress,
    entity_id: EntityId,
    change: StreamChange,
) -> Result<(), String> {
    edit(interface, |config| {
        let place = place_of(config, mac, entity_id).ok_or("not one of this computer's")?;
        change_streams(&mut config.endpoints[place], change)
    })
}

/// Changes one of `endpoint`'s streams, but not the default entity's,
/// whose streams other apps play into and record from.
fn change_streams(endpoint: &mut EndpointConfig, change: StreamChange) -> Result<(), String> {
    if endpoint.is_default {
        return Err("the default entity's streams stay, for other apps".into());
    }
    // An entity's streams as they are, written down once one changes.
    let mut streams = endpoint.stream_configs();
    match change {
        StreamChange::Add { direction, clock } => {
            let same = streams
                .iter()
                .filter(|stream| stream.direction == direction)
                .count();
            if same >= avb_endpointd::config::MOST_STREAMS {
                return Err("no room for another".into());
            }
            streams.push(if clock {
                StreamConfig::clock(direction)
            } else {
                StreamConfig::new(direction)
            });
        }
        StreamChange::Remove(at) if at < streams.len() => {
            streams.remove(at);
        }
        StreamChange::Channels(at, channels) if at < streams.len() => {
            streams[at].channels = channels;
            // A format chosen for other channels no longer holds.
            streams[at].format = None;
        }
        StreamChange::Audio(at, audio) if at < streams.len() => {
            streams[at].audio = (audio != avb_endpointd::config::VIRTUAL).then_some(audio);
        }
        _ => return Err("no such stream".into()),
    }
    endpoint.streams = streams;
    Ok(())
}

/// The endpoints on `interface`, each with its instance written down so
/// its entity ID stays the same, for a preset to keep.
pub fn endpoints(interface: &str) -> Vec<EndpointConfig> {
    if cfg!(test) {
        return Vec::new();
    }
    let Ok((_, mut config)) = current(interface) else {
        return Vec::new();
    };
    config.pin_instances();
    config.endpoints
}

/// Runs `endpoints` on `interface` in place of the ones there, starting
/// the daemon when it isn't running and triib's own are among them. The
/// default entity stays, with the settings `endpoints` give it where they
/// have it. Says whether they will start again, which they do when
/// anything changed.
pub fn restore(interface: &str, endpoints: &[EndpointConfig]) -> Result<bool, String> {
    if cfg!(test) {
        return Ok(false);
    }
    let running = status::read(interface).is_some();
    let path = entities_path(interface).ok_or("no configuration folder")?;
    let on_file = path.exists();
    let changed = edit(interface, |config| {
        let mut now = config.clone();
        now.pin_instances();
        if on_file && now.endpoints == endpoints {
            return Ok(false);
        }
        config.endpoints = endpoints.to_vec();
        Ok(true)
    })?;
    if own(endpoints).next().is_none() {
        return Ok(false);
    }
    ensure_running(interface)?;
    Ok(changed || !running)
}

/// The interface shown, when another person's or root's daemon holds it
/// and this person lists entities of triib's there: a daemon of this
/// person's would be turned away.
pub fn elsewhere() -> Option<String> {
    let interface = shown()?;
    (status::read(&interface).is_none()
        && status::interface_claimed(&interface)
        && own(&endpoints(&interface)).next().is_some())
    .then_some(interface)
}

/// Starts the daemon's unit on `interface`, enabled so that it starts
/// with the desktop after, unless it runs already, was started a moment
/// ago and has not said so yet, or another person's holds the interface.
pub fn ensure_running(interface: &str) -> Result<(), String> {
    if status::read(interface).is_some() || status::interface_claimed(interface) {
        return Ok(());
    }
    let mut starts = STARTS.lock().unwrap_or_else(PoisonError::into_inner);
    if starts.at.is_some_and(|at| at.elapsed() < STARTING) {
        return Ok(());
    }
    starts.at = Some(Instant::now());
    drop(starts);
    start(interface)
}

/// Keeps this computer's entities on `interface` running while the app
/// runs, or none.
pub fn keep_running(interface: Option<&str>) {
    let mut kept = KEPT.lock().unwrap_or_else(PoisonError::into_inner);
    if kept.as_deref() != interface {
        *kept = interface.map(str::to_owned);
    }
}

/// Starts the daemon's unit where it does not run and entities of triib's
/// are listed for the interface the app keeps: as triib opens, as for entities
/// listed before the unit was enabled, and should it have stopped. Starts
/// that do not last wait longer each time.
pub fn bring_back() {
    let Some(interface) = KEPT.lock().unwrap_or_else(PoisonError::into_inner).clone() else {
        return;
    };
    let mut starts = STARTS.lock().unwrap_or_else(PoisonError::into_inner);
    if status::read(&interface).is_some() {
        starts.wait = STARTING;
        return;
    }
    if status::interface_claimed(&interface) || own(&endpoints(&interface)).next().is_none() {
        return;
    }
    if starts.at.is_some_and(|at| at.elapsed() < starts.wait) {
        return;
    }
    // A start before this one that never came to run.
    if starts.at.is_some() {
        starts.wait = (starts.wait * 2).min(LONGEST_WAIT);
    }
    starts.at = Some(Instant::now());
    drop(starts);
    eprintln!("triib: avb-endpointd does not run on {interface}; starting it");
    if let Err(error) = start(&interface) {
        eprintln!("triib: could not start avb-endpointd on {interface}: {error}");
    }
}

/// The daemon's unit on `interface`, which must be named as it is: what
/// Linux allows in an interface name, less what a unit name would need
/// escaped.
#[cfg(any(target_os = "linux", test))]
fn unit(interface: &str) -> Result<String, String> {
    let nameable = !interface.is_empty()
        && interface
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'));
    if nameable {
        Ok(format!("avb-endpointd@{interface}.service"))
    } else {
        Err(format!("{interface} cannot name a unit"))
    }
}

/// Runs `systemctl --user` with `arguments` for `unit`.
#[cfg(target_os = "linux")]
fn user_systemctl(arguments: &[&str], unit: &str) -> Result<(), String> {
    let output = std::process::Command::new("systemctl")
        .arg("--user")
        .args(arguments)
        .arg(unit)
        .stdin(std::process::Stdio::null())
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_owned())
    }
}

/// Starts the daemon's unit on `interface`, enabled, after clearing a
/// failure that would keep systemd from starting it.
#[cfg(target_os = "linux")]
fn start(interface: &str) -> Result<(), String> {
    let unit = unit(interface)?;
    let _ = user_systemctl(&["reset-failed"], &unit);
    user_systemctl(&["enable", "--now"], &unit)
}

#[cfg(not(target_os = "linux"))]
fn start(_interface: &str) -> Result<(), String> {
    Err("not on this system".into())
}

/// Whether `entity_id` is the interface's default entity, which triib
/// shows but neither removes nor changes the streams of.
pub fn is_default(status: Option<&DaemonStatus>, entity_id: EntityId) -> bool {
    status_of(status, entity_id).is_some_and(|endpoint| endpoint.default)
}

/// What the daemon says of the endpoint whose entity is `entity_id`.
pub fn status_of(status: Option<&DaemonStatus>, entity_id: EntityId) -> Option<&EndpointStatus> {
    let id = entity_id.to_string();
    status?
        .endpoints
        .iter()
        .find(|endpoint| endpoint.entity_id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_unit_is_named_for_the_interface() {
        assert_eq!(unit("enp2s0").unwrap(), "avb-endpointd@enp2s0.service");
        assert_eq!(unit("br-lan.2").unwrap(), "avb-endpointd@br-lan.2.service");
        assert!(unit("").is_err());
        assert!(unit("../x").is_err());
        assert!(unit("a b").is_err());
    }
}
