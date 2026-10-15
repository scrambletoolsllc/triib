//! ptp4l's profile on the interface, where triib's own systemd units run
//! it: `triib-ptp4l-gptp@<interface>` for gPTP and
//! `triib-ptp4l-lite@<interface>` for the AVB Lite PTP profile, each
//! stopping the other as it starts. A polkit rule lets the people at the
//! computer start them, so the daemon moves ptp4l to the AVB Lite profile
//! when its endpoints fall back, and to gPTP again when the link comes
//! back up (AVB Lite profile, 2.2). Where no ptp4l runs on the interface,
//! the daemon starts the gPTP unit itself, so installing triib is all the
//! network's time takes; ptp4l run any other way stays as it is. A third
//! unit, `triib-link@<interface>`, keeps Energy-Efficient
//! Ethernet and PAUSE off the link, as the profile asks (6); the daemon
//! runs it again should they come back. A fourth,
//! `triib-link-reset@<interface>`, takes the interface down and up for a
//! driver that keeps its time stamps on received PTP frames.

use std::io;
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::time::Duration;

/// How often the link and the units are looked at.
const POLL: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Profile {
    Gptp,
    Lite,
}

impl Profile {
    pub fn unit(self, interface: &str) -> String {
        let name = match self {
            Profile::Gptp => "gptp",
            Profile::Lite => "lite",
        };
        format!("triib-ptp4l-{name}@{interface}.service")
    }
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Profile::Gptp => "gPTP",
            Profile::Lite => "the AVB Lite PTP profile",
        })
    }
}

/// What the interface's link and triib's units are doing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Link {
    /// The link is up; `None` when the system does not say.
    pub carrier: Option<bool>,
    /// Which of triib's units runs ptp4l on the interface, if one does.
    pub unit: Option<Profile>,
    /// What the interface does about EEE and PAUSE, where the system says.
    pub power: Option<avb_net::LinkPower>,
}

/// Whether `interface` names a unit as it is: what Linux allows in an
/// interface name, less what a unit name would need escaped.
fn nameable(interface: &str) -> bool {
    !interface.is_empty()
        && interface
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
}

fn active(unit: &str) -> bool {
    Command::new("systemctl")
        .args(["is-active", "--quiet", unit])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

/// Which of triib's units runs ptp4l on `interface`, if one does.
pub fn running(interface: &str) -> Option<Profile> {
    if !nameable(interface) {
        return None;
    }
    [Profile::Gptp, Profile::Lite]
        .into_iter()
        .find(|profile| active(&profile.unit(interface)))
}

/// Whether linuxptp's ptp4l is installed where systemd looks for a unit's
/// program.
pub fn ptp4l_installed() -> bool {
    ["/usr/local/sbin", "/usr/local/bin", "/usr/sbin", "/usr/bin"]
        .iter()
        .any(|folder| std::path::Path::new(folder).join("ptp4l").exists())
}

/// Whether a ptp4l process names `interface` on its command line, as one
/// run by hand or by another unit does.
pub fn ptp4l_on(interface: &str) -> bool {
    let Ok(entries) = std::fs::read_dir("/proc") else {
        return false;
    };
    entries.flatten().any(|entry| {
        std::fs::read(entry.path().join("cmdline"))
            .is_ok_and(|line| runs_ptp4l_on(&line, interface))
    })
}

/// Whether a process's command line, its arguments each ended by a zero,
/// runs ptp4l on `interface`.
fn runs_ptp4l_on(line: &[u8], interface: &str) -> bool {
    let args: Vec<&[u8]> = line.split(|&byte| byte == 0).collect();
    let program = args
        .first()
        .and_then(|program| program.rsplit(|&byte| byte == b'/').next());
    program == Some(b"ptp4l".as_slice())
        && args
            .windows(2)
            .any(|pair| pair[0] == b"-i" && pair[1] == interface.as_bytes())
}

/// Starts the unit running `profile` on `interface`, which stops the
/// other; ptp4l then starts again in that profile, its clock locking in
/// a few seconds.
pub fn start(interface: &str, profile: Profile) -> io::Result<()> {
    if !nameable(interface) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{interface} cannot name a unit"),
        ));
    }
    let output = Command::new("systemctl")
        .args(["start", "--no-block", "--no-ask-password"])
        .arg(profile.unit(interface))
        .stdin(Stdio::null())
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::other(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ))
    }
}

/// Runs the link unit for `interface` again, putting EEE and PAUSE off
/// once more; the link renegotiates.
pub fn apply_link(interface: &str) -> io::Result<()> {
    restart(interface, &link_unit(interface))
}

/// Takes `interface` down and up again through its unit, for a driver
/// that keeps its time stamps on received PTP frames until then.
pub fn reset_link(interface: &str) -> io::Result<()> {
    restart(interface, &reset_unit(interface))
}

/// Runs `unit`, one of `interface`'s, again.
fn restart(interface: &str, unit: &str) -> io::Result<()> {
    if !nameable(interface) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{interface} cannot name a unit"),
        ));
    }
    let output = Command::new("systemctl")
        .args(["restart", "--no-block", "--no-ask-password"])
        .arg(unit)
        .stdin(Stdio::null())
        .output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err(io::Error::other(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ))
    }
}

/// The unit keeping EEE and PAUSE off `interface`.
pub fn link_unit(interface: &str) -> String {
    format!("triib-link@{interface}.service")
}

/// The unit taking `interface` down and up again.
pub fn reset_unit(interface: &str) -> String {
    format!("triib-link-reset@{interface}.service")
}

/// What `interface` does about EEE and PAUSE, where the system says.
pub fn power(interface: &str) -> Option<avb_net::LinkPower> {
    #[cfg(target_os = "linux")]
    {
        avb_net::interfaces::link_power(interface).ok()
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = interface;
        None
    }
}

/// Whether `interface`'s link is up, where the system says.
pub fn carrier(interface: &str) -> Option<bool> {
    if !nameable(interface) {
        return None;
    }
    let text = std::fs::read_to_string(format!("/sys/class/net/{interface}/carrier")).ok()?;
    match text.trim() {
        "1" => Some(true),
        "0" => Some(false),
        _ => None,
    }
}

/// Looks at the link and the units every few seconds and sends what
/// changed, until `stop`.
pub fn watch<T: Send + 'static>(
    interface: String,
    stop: Arc<AtomicBool>,
    sender: Sender<T>,
    wrap: fn(Link) -> T,
) {
    let _ = std::thread::Builder::new()
        .name("link".into())
        .spawn(move || {
            let mut last = None;
            while !stop.load(Ordering::Relaxed) {
                let link = Link {
                    carrier: carrier(&interface),
                    unit: running(&interface),
                    power: power(&interface),
                };
                if last != Some(link) {
                    if sender.send(wrap(link)).is_err() {
                        return;
                    }
                    last = Some(link);
                }
                std::thread::sleep(POLL);
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ptp4l_on_the_interface_is_told_from_others() {
        let on = |line: &str| runs_ptp4l_on(line.replace(' ', "\0").as_bytes(), "enp2s0");
        assert!(on("ptp4l -f /etc/triib/ptp4l-gptp.cfg -i enp2s0"));
        assert!(on("/usr/sbin/ptp4l -i enp2s0 -m"));
        assert!(!on("ptp4l -i enp6s0"));
        assert!(!on("phc2sys -s enp2s0 -w"));
        assert!(!on("ptp4lx -i enp2s0"));
    }

    #[test]
    fn units_are_named_for_the_interface() {
        assert_eq!(
            Profile::Lite.unit("enp2s0"),
            "triib-ptp4l-lite@enp2s0.service"
        );
        assert!(nameable("enp2s0.2"));
        assert!(!nameable("enp2s0 x"));
        assert!(!nameable("../x"));
        assert!(!nameable(""));
        assert_eq!(running("no/such"), None);
    }
}
