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
/// How long a daemon the app started must run for the next start to wait
/// no longer than `STARTING`, and the longest the app waits to start one
/// again after starts that did not last.
const LASTING: Duration = Duration::from_secs(60);
const LONGEST_WAIT: Duration = Duration::from_secs(5 * 60);
/// How long a daemon asked to stop has to go, before one on another file
/// starts anyway.
const STOPPING: Duration = Duration::from_secs(5);

/// The file the settings name for this computer's entities, if any.
static ENTITIES_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);

/// The interface whose entities the app keeps running, while it shows one
/// that runs entities of this computer's.
static KEPT: Mutex<Option<String>> = Mutex::new(None);

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
    if triib_endpointd::status::read().is_some() || elsewhere().is_some() {
        return Ok(());
    }
    let mut starts = STARTS.lock().unwrap_or_else(PoisonError::into_inner);
    if starts.at.is_some_and(|at| at.elapsed() < STARTING) {
        return Ok(());
    }
    starts.at = Some(Instant::now());
    drop(starts);
    start()
}

/// Keeps this computer's entities on `interface` running while the app
/// runs, or none.
pub fn keep_running(interface: Option<&str>) {
    let mut kept = KEPT.lock().unwrap_or_else(PoisonError::into_inner);
    if kept.as_deref() != interface {
        *kept = interface.map(str::to_owned);
    }
}

/// Starts triib-endpointd where it does not run and entities are listed
/// for the interface the app keeps: as triib opens, and whenever it stops
/// while triib runs, as after a crash. Starts that do not last wait
/// longer each time.
pub fn bring_back() {
    let Some(interface) = KEPT.lock().unwrap_or_else(PoisonError::into_inner).clone() else {
        return;
    };
    if triib_endpointd::status::read().is_some() || elsewhere().is_some() {
        return;
    }
    if endpoints(&interface).is_empty() {
        return;
    }
    let mut starts = STARTS.lock().unwrap_or_else(PoisonError::into_inner);
    if starts.at.is_some_and(|at| at.elapsed() < starts.wait) {
        return;
    }
    starts.at = Some(Instant::now());
    drop(starts);
    eprintln!("triib: triib-endpointd does not run; starting it");
    if let Err(error) = start() {
        eprintln!("triib: could not start triib-endpointd: {error}");
    }
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
    // The last run's log is kept beside the new one, for why it stopped.
    let log = triib_store::paths::state_dir()
        .map(|dir| {
            let _ = std::fs::create_dir_all(&dir);
            let path = dir.join("endpointd.log");
            let _ = std::fs::rename(&path, dir.join("endpointd.log.1"));
            path
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
    let mut child = command
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(error)
        // Its own process group, so it outlives the app.
        .process_group(0)
        .spawn()
        .map_err(|error| error.to_string())?;
    // Collected as it ends while the app runs, as the app is its parent
    // until then: else it would linger as a zombie, its process still
    // there to look at. How long it ran says how long to wait before
    // starting it again.
    let started = Instant::now();
    let _ = std::thread::Builder::new()
        .name("endpointd".into())
        .spawn(move || {
            let _ = child.wait();
            let mut starts = STARTS.lock().unwrap_or_else(PoisonError::into_inner);
            starts.wait = if started.elapsed() < LASTING {
                (starts.wait * 2).min(LONGEST_WAIT)
            } else {
                STARTING
            };
        });
    Ok(())
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
