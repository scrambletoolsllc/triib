//! What the daemon is doing, for the app: `endpointd.toml` in the runtime
//! folder, written every two seconds while it runs and removed when it
//! stops.

use std::fs::{File, TryLockError};
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::Direction;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaemonStatus {
    pub pid: u32,
    /// When it started on the endpoints it runs, in milliseconds since the
    /// Unix epoch.
    #[serde(default)]
    pub started: u64,
    pub interface: String,
    pub gptp: String,
    /// Another program declares MSRP or MVRP on the interface from this
    /// computer's address, which can withdraw what the endpoints need.
    #[serde(default)]
    pub foreign_mrp: bool,
    /// The file it lists its entities in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// This computer's clock, as ptp4l runs it, when ptp4l answers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock: Option<ClockStatus>,
    /// What it is setting up, or misses, that the entities wait on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub setup: Option<Setup>,
    #[serde(rename = "endpoint", default)]
    pub endpoints: Vec<EndpointStatus>,
}

/// What the daemon is setting up, or misses, that its entities wait on
/// to reach controllers or keep the network's time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Setup {
    /// A link unit runs, which takes the link down for a moment.
    Link,
    /// The link is down.
    NoLink,
    /// ptp4l is starting, and does not answer yet.
    Ptp4l,
    /// No ptp4l answers, and linuxptp is not installed.
    NoLinuxptp,
    /// What a later daemon says, which this app does not know.
    #[serde(other)]
    Other,
}

/// This computer's clock as ptp4l runs it on the interface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClockStatus {
    /// The grandmaster it follows, as text, `0x` and sixteen hex digits.
    pub grandmaster: String,
    /// The servo's last offset from the grandmaster, in nanoseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// It runs the AVB Lite PTP profile, end to end, rather than gPTP.
    #[serde(default)]
    pub end_to_end: bool,
    /// Its port is as-capable and not FAULTY.
    #[serde(default)]
    pub as_capable: bool,
}

impl ClockStatus {
    /// The grandmaster it follows.
    pub fn grandmaster(&self) -> Option<atdecc::ClockIdentity> {
        self.grandmaster.parse().ok()
    }
}

/// One of this computer's entities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointStatus {
    /// As text, `0x` and sixteen hex digits.
    pub entity_id: String,
    pub name: String,
    /// What it is doing: streaming, waiting, listening, bound or unbound.
    pub state: String,
    /// Its streams, in order.
    #[serde(rename = "stream", default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<StreamStatus>,
}

/// One of a device's streams.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamStatus {
    pub direction: Direction,
    /// A media clock stream rather than audio.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub clock: bool,
    pub name: String,
    pub channels: u16,
    /// Where its audio comes from or goes, as entities.toml says it:
    /// "virtual" for its own virtual audio device.
    pub audio: String,
    /// What it is doing: streaming, waiting, listening, bound or unbound.
    pub state: String,
}

/// Where the daemon says what it is doing.
pub fn status_path() -> Option<PathBuf> {
    triib_store::paths::runtime_dir().map(|dir| dir.join("endpointd.toml"))
}

/// Holds the lock only one daemon of a user's may take, for as long as it
/// lives: `None` when another holds it.
pub fn take_lock() -> io::Result<Option<File>> {
    let dir = triib_store::paths::runtime_dir().unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&dir)?;
    let file = File::options()
        .create(true)
        .truncate(false)
        .write(true)
        .open(dir.join("endpointd.lock"))?;
    match file.try_lock() {
        Ok(()) => Ok(Some(file)),
        Err(TryLockError::WouldBlock) => Ok(None),
        Err(TryLockError::Error(error)) => Err(error),
    }
}

/// A daemon's hold on an interface, against daemons of every user: two on
/// one port would each run MSRP and MVRP there, and the one withdrawing
/// what the other still declares takes it from the bridge, as a
/// point-to-point port's participants do not hear each other's Leave.
/// Released when the daemon ends, however it ends.
pub struct Claim {
    #[cfg(target_os = "linux")]
    _socket: std::os::unix::net::UnixListener,
}

/// The name of an interface's claim, in Linux's abstract socket names,
/// which every user shares.
#[cfg(target_os = "linux")]
fn claim_name(interface: &str) -> io::Result<std::os::unix::net::SocketAddr> {
    use std::os::linux::net::SocketAddrExt;
    std::os::unix::net::SocketAddr::from_abstract_name(format!("triib-endpointd/{interface}"))
}

/// Claims `interface` for this daemon: `None` when a daemon of this or
/// another user, or root's, holds it.
pub fn claim_interface(interface: &str) -> io::Result<Option<Claim>> {
    #[cfg(target_os = "linux")]
    {
        match std::os::unix::net::UnixListener::bind_addr(&claim_name(interface)?) {
            Ok(socket) => Ok(Some(Claim { _socket: socket })),
            Err(error) if error.kind() == io::ErrorKind::AddrInUse => Ok(None),
            Err(error) => Err(error),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = interface;
        Ok(Some(Claim {}))
    }
}

/// Whether a daemon holds `interface`, this user's or another's, from
/// the socket names Linux lists, without connecting to the claim, which
/// no daemon accepts.
pub fn interface_claimed(interface: &str) -> bool {
    #[cfg(target_os = "linux")]
    {
        let name = format!(" @triib-endpointd/{interface}");
        std::fs::read_to_string("/proc/net/unix")
            .is_ok_and(|sockets| sockets.lines().any(|line| line.ends_with(&name)))
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = interface;
        false
    }
}

/// What the daemon says, while it runs: its file is there and its
/// process too.
pub fn read() -> Option<DaemonStatus> {
    let status: DaemonStatus = triib_store::load(&status_path()?).ok()?;
    running(status.pid).then_some(status)
}

/// Whether process `pid` runs: there, and not a zombie, which ended
/// with its parent yet to collect it, as the app is the parent of a
/// daemon it started until the app ends.
#[cfg(target_os = "linux")]
fn running(pid: u32) -> bool {
    std::fs::read_to_string(format!("/proc/{pid}/stat")).is_ok_and(|stat| {
        // The state follows the command's name, in parentheses that can
        // hold parentheses of its own.
        stat.rsplit_once(')')
            .and_then(|(_, rest)| rest.split_whitespace().next())
            .is_some_and(|state| !matches!(state, "Z" | "X"))
    })
}

#[cfg(not(target_os = "linux"))]
fn running(_pid: u32) -> bool {
    true
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

    /// The clock's table goes between the daemon's values and its entities,
    /// and names its grandmaster as it reads back.
    #[test]
    fn the_status_carries_this_computers_clock() {
        let grandmaster = atdecc::ClockIdentity(0xe8f6_0aff_fee0_9220);
        let status = DaemonStatus {
            pid: 1,
            started: 0,
            interface: "enp2s0".into(),
            gptp: String::new(),
            foreign_mrp: false,
            config: None,
            clock: Some(ClockStatus {
                grandmaster: grandmaster.to_string(),
                offset: Some(-120),
                end_to_end: true,
                as_capable: true,
            }),
            setup: Some(Setup::Ptp4l),
            endpoints: vec![EndpointStatus {
                entity_id: "0xa010a3ff00a51726".into(),
                name: "Host entity 1".into(),
                state: "waiting".into(),
                streams: Vec::new(),
            }],
        };
        let text = toml::to_string(&status).unwrap();
        let read: DaemonStatus = toml::from_str(&text).unwrap();
        assert_eq!(read, status);
        assert_eq!(read.clock.unwrap().grandmaster(), Some(grandmaster));
        assert!(text.contains("setup = \"ptp4l\""), "{text}");
        // A step a later daemon names is one this app does not know,
        // rather than no status at all.
        let later: DaemonStatus =
            toml::from_str(&text.replace("\"ptp4l\"", "\"later-step\"")).unwrap();
        assert_eq!(later.setup, Some(Setup::Other));
    }

    #[test]
    fn one_daemon_claims_an_interface() {
        let interface = format!("test{}", std::process::id());
        assert!(!interface_claimed(&interface));
        let claim = claim_interface(&interface).unwrap();
        assert!(claim.is_some());
        assert!(interface_claimed(&interface));
        // A second daemon, of any user, is turned away while it is held.
        assert!(claim_interface(&interface).unwrap().is_none());
        drop(claim);
        assert!(!interface_claimed(&interface));
        assert!(claim_interface(&interface).unwrap().is_some());
    }

    /// A process that ended but was not yet collected, a zombie, runs no
    /// longer; one gone altogether neither.
    #[test]
    fn a_zombie_does_not_run() {
        assert!(running(std::process::id()));
        let mut child = std::process::Command::new("true").spawn().unwrap();
        let pid = child.id();
        let until = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while running(pid) && std::time::Instant::now() < until {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert!(!running(pid));
        assert!(std::path::Path::new(&format!("/proc/{pid}")).exists());
        child.wait().unwrap();
        assert!(!running(pid));
    }
}
