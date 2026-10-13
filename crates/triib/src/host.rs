//! This computer's own entities, which triib-endpointd runs: adding,
//! changing and removing them and their streams in its `entities.toml`,
//! starting it, and what it says each is doing.

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

use atdecc::{EntityId, MacAddress};
use triib_endpointd::config::{self, Config, Direction, EndpointConfig, StreamConfig};
use triib_endpointd::status::{DaemonStatus, EndpointStatus};

/// Whether this system runs entities of its own.
pub const SUPPORTED: bool = cfg!(target_os = "linux");
/// A preset waits for this computer's endpoints, so the app hears what
/// the daemon says every time it looks, changed or not, to keep its
/// deadline.
pub static RECALLING: AtomicBool = AtomicBool::new(false);

/// How long a daemon just started has to say it runs, before the app
/// would start another; the daemon's lock turns that one away anyway.
const STARTING: Duration = Duration::from_secs(5);
/// How long a daemon asked to stop has to go, before one on another file
/// starts anyway.
const STOPPING: Duration = Duration::from_secs(5);

/// The file the settings name for this computer's entities, if any.
static ENTITIES_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);

/// Lists this computer's entities in `file`, or in the default file when
/// none.
pub fn set_entities_file(file: Option<PathBuf>) {
    *ENTITIES_FILE.lock().unwrap_or_else(PoisonError::into_inner) = file;
}

/// Where this computer's entities are listed: the file the settings
/// name, else `entities.toml` in the data folder.
pub fn entities_path() -> Option<PathBuf> {
    ENTITIES_FILE
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .clone()
        .or_else(config::path)
}

/// Moves this computer's entities to `to`: a file that is not there yet
/// starts as a copy of the one in use, and the daemon, when it runs on
/// another file, starts again on it. `None` is the default file.
pub fn move_entities(to: Option<PathBuf>) -> Result<(), String> {
    let from = entities_path();
    set_entities_file(to);
    if cfg!(test) {
        return Ok(());
    }
    let to = entities_path().ok_or("no data folder")?;
    if !to.exists()
        && let Some(from) = from.filter(|from| from.exists() && *from != to)
    {
        if let Some(folder) = to.parent() {
            std::fs::create_dir_all(folder).map_err(|error| error.to_string())?;
        }
        std::fs::copy(&from, &to).map_err(|error| error.to_string())?;
    }
    let Some(status) = triib_endpointd::status::read() else {
        return Ok(());
    };
    if status.config.as_deref() == Some(&to.display().to_string()) {
        return Ok(());
    }
    // Stops it, then starts one on the new file once it has gone.
    std::process::Command::new("kill")
        .arg("-TERM")
        .arg(status.pid.to_string())
        .status()
        .map_err(|error| error.to_string())?;
    std::thread::spawn(|| {
        let asked = Instant::now();
        while triib_endpointd::status::read().is_some() && asked.elapsed() < STOPPING {
            std::thread::sleep(Duration::from_millis(100));
        }
        let _ = start();
    });
    Ok(())
}

/// The endpoints file, or a new one for `interface`, with the interface
/// set to it either way.
fn load(interface: &str) -> Result<(std::path::PathBuf, Config), String> {
    let path = entities_path().ok_or("no data folder")?;
    let mut config = if path.exists() {
        triib_store::load::<Config>(&path).map_err(|error| error.to_string())?
    } else {
        Config {
            ptp4l_socket: "/var/run/ptp4lro".into(),
            ..Config::default()
        }
    };
    config.interface = interface.to_owned();
    Ok((path, config))
}

fn save(path: &std::path::Path, config: &Config) -> Result<(), String> {
    triib_store::save(path, config).map_err(|error| error.to_string())
}

/// The place in the file of the endpoint whose entity is `entity_id`.
fn place_of(config: &Config, mac: MacAddress, entity_id: EntityId) -> Option<usize> {
    config
        .endpoints
        .iter()
        .enumerate()
        .position(|(place, endpoint)| {
            triib_endpointd::runtime::entity_id(mac, endpoint.instance_at(place)) == entity_id
        })
}

/// Adds an entity named `name`, with a stream each way, each with its own
/// virtual audio device; its inspector adds more and sets each up.
pub fn add(interface: &str, name: String) -> Result<(), String> {
    let (path, mut config) = load(interface)?;
    let taken: Vec<u8> = config
        .endpoints
        .iter()
        .enumerate()
        .map(|(place, endpoint)| endpoint.instance_at(place))
        .collect();
    let free = (0..240u8)
        .find(|candidate| !taken.contains(candidate))
        .ok_or("no room")?;
    config.endpoints.push(EndpointConfig::new(name, Some(free)));
    save(&path, &config)
}

/// Removes the endpoint whose entity is `entity_id`; the others keep
/// their entity IDs.
pub fn remove(interface: &str, mac: MacAddress, entity_id: EntityId) -> Result<(), String> {
    let (path, mut config) = load(interface)?;
    let place = place_of(&config, mac, entity_id).ok_or("not one of this computer's")?;
    config.endpoints = pinned(std::mem::take(&mut config.endpoints));
    config.endpoints.remove(place);
    save(&path, &config)
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
    let (path, mut config) = load(interface)?;
    let place = place_of(&config, mac, entity_id).ok_or("not one of this computer's")?;
    let endpoint = &mut config.endpoints[place];
    // An entity's streams as they are, written down once one changes.
    let mut streams = endpoint.stream_configs();
    match change {
        StreamChange::Add { direction, clock } => {
            let same = streams
                .iter()
                .filter(|stream| stream.direction == direction)
                .count();
            if same >= triib_endpointd::config::MOST_STREAMS {
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
            streams[at].audio = (audio != triib_endpointd::config::VIRTUAL).then_some(audio);
        }
        _ => return Err("no such stream".into()),
    }
    endpoint.streams = streams;
    save(&path, &config)
}

/// The endpoints on `interface`, each with its instance written down so
/// its entity ID stays the same, for a preset to keep.
pub fn endpoints(interface: &str) -> Vec<EndpointConfig> {
    if cfg!(test) {
        return Vec::new();
    }
    let Some(config) = entities_path().and_then(|path| triib_store::load::<Config>(&path).ok())
    else {
        return Vec::new();
    };
    if config.interface != interface {
        return Vec::new();
    }
    pinned(config.endpoints)
}

fn pinned(mut endpoints: Vec<EndpointConfig>) -> Vec<EndpointConfig> {
    for (place, endpoint) in endpoints.iter_mut().enumerate() {
        endpoint.instance = Some(endpoint.instance_at(place));
    }
    endpoints
}

/// Runs `endpoints` on `interface` in place of the ones there, starting
/// the daemon when it isn't running. Says whether they will start again,
/// which they do when anything changed.
pub fn restore(interface: &str, endpoints: &[EndpointConfig]) -> Result<bool, String> {
    if cfg!(test) {
        return Ok(false);
    }
    let path = entities_path().ok_or("no data folder")?;
    let running = triib_endpointd::status::read().is_some();
    let now = triib_store::load::<Config>(&path).ok();
    let same = now.as_ref().is_some_and(|config| {
        config.interface == interface && pinned(config.endpoints.clone()) == endpoints
    });
    if !same {
        let (path, mut config) = load(interface)?;
        config.endpoints = endpoints.to_vec();
        save(&path, &config)?;
    }
    ensure_running()?;
    Ok(!same || !running)
}

/// The interface this user's endpoints would run on, when another
/// user's or root's daemon holds it: a daemon of this user's would be
/// turned away.
pub fn elsewhere() -> Option<String> {
    let interface = triib_store::load::<Config>(&entities_path()?)
        .ok()?
        .interface;
    (!interface.is_empty() && triib_endpointd::status::interface_claimed(&interface))
        .then_some(interface)
}

/// Starts triib-endpointd, from beside triib, unless it runs already, was
/// started a moment ago and has not said so yet, or another user's holds
/// the interface.
pub fn ensure_running() -> Result<(), String> {
    static STARTED: Mutex<Option<Instant>> = Mutex::new(None);
    if triib_endpointd::status::read().is_some() || elsewhere().is_some() {
        return Ok(());
    }
    let mut started = STARTED.lock().unwrap_or_else(PoisonError::into_inner);
    if started.is_some_and(|at| at.elapsed() < STARTING) {
        return Ok(());
    }
    *started = Some(Instant::now());
    start()
}

#[cfg(unix)]
fn start() -> Result<(), String> {
    use std::os::unix::process::CommandExt;
    use std::process::{Command, Stdio};

    // Beside triib, else wherever the PATH finds it.
    let program = std::env::current_exe()
        .map(|path| path.with_file_name("triib-endpointd"))
        .ok()
        .filter(|path| path.exists())
        .unwrap_or_else(|| "triib-endpointd".into());
    let log = triib_store::paths::state_dir()
        .map(|dir| {
            let _ = std::fs::create_dir_all(&dir);
            dir.join("endpointd.log")
        })
        .and_then(|path| std::fs::File::create(path).ok());
    let (out, error) = match log {
        Some(file) => match file.try_clone() {
            Ok(copy) => (Stdio::from(file), Stdio::from(copy)),
            Err(_) => (Stdio::null(), Stdio::null()),
        },
        None => (Stdio::null(), Stdio::null()),
    };
    let mut command = Command::new(program);
    if let Some(file) = entities_path() {
        command.arg("--config").arg(file);
    }
    command
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(error)
        // Its own process group, so it outlives the app.
        .process_group(0)
        .spawn()
        .map(drop)
        .map_err(|error| error.to_string())
}

#[cfg(not(unix))]
fn start() -> Result<(), String> {
    Err("not on this system".into())
}

/// What the daemon says of the endpoint whose entity is `entity_id`.
pub fn status_of(status: Option<&DaemonStatus>, entity_id: EntityId) -> Option<&EndpointStatus> {
    let id = entity_id.to_string();
    status?
        .endpoints
        .iter()
        .find(|endpoint| endpoint.entity_id == id)
}
