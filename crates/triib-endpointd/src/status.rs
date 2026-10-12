//! What the daemon is doing, for the app: `endpointd.toml` in the runtime
//! folder, written every two seconds while it runs and removed when it
//! stops.

use std::fs::{File, TryLockError};
use std::io;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::{Direction, Kind};

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
    #[serde(rename = "endpoint", default)]
    pub endpoints: Vec<EndpointStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointStatus {
    /// As text, `0x` and sixteen hex digits.
    pub entity_id: String,
    pub name: String,
    pub kind: Kind,
    /// What it is doing: streaming, waiting, listening, bound or unbound.
    pub state: String,
    /// Where its audio comes from or goes, as endpoints.toml says it.
    #[serde(default)]
    pub audio: String,
    /// Its stream's channels.
    #[serde(default = "eight")]
    pub channels: u16,
    /// A device's streams, in order.
    #[serde(rename = "stream", default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<StreamStatus>,
}

/// One of a device's streams.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamStatus {
    pub direction: Direction,
    pub name: String,
    pub channels: u16,
    /// Where its audio comes from or goes, as endpoints.toml says it:
    /// "virtual" for its own virtual audio device.
    pub audio: String,
    /// What it is doing: streaming, waiting, listening, bound or unbound.
    pub state: String,
}

fn eight() -> u16 {
    8
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

#[cfg(target_os = "linux")]
fn running(pid: u32) -> bool {
    std::path::Path::new(&format!("/proc/{pid}")).exists()
}

#[cfg(not(target_os = "linux"))]
fn running(_pid: u32) -> bool {
    true
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;

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
}
