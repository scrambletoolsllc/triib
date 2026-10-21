//! gPTP as ptp4l runs it: asked through its read-only management socket
//! with linuxptp's pmc every few seconds. ptp4l running the AVB Lite PTP
//! profile answers too, end to end and without gPTP's transportSpecific.

use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::time::Duration;

use atdecc::ClockIdentity;
use atdecc::entity::Gptp;

/// How often ptp4l is asked.
const POLL: Duration = Duration::from_secs(2);

/// What ptp4l says, as an entity reports it, and this computer's clock
/// identity; `None` when ptp4l does not answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub gptp: Gptp,
    pub own_clock: ClockIdentity,
    /// The servo's last offset from the grandmaster, in nanoseconds.
    pub offset: Option<i64>,
    /// ptp4l measures delay end to end, as the AVB Lite PTP profile does,
    /// not peer to peer as gPTP does.
    pub end_to_end: bool,
    /// ptp4l's port is FAULTY, as after a fault of its own such as a
    /// transmit time stamp coming late; it resets after its
    /// fault_reset_interval.
    pub faulty: bool,
}

/// A clock identity as linuxptp prints it, such as `0001f2.fffe.ff3b14`.
fn identity(text: &str) -> Option<ClockIdentity> {
    let digits: String = text
        .split('-')
        .next()?
        .chars()
        .filter(|c| *c != '.')
        .collect();
    (digits.len() == 16)
        .then(|| u64::from_str_radix(&digits, 16).ok())
        .flatten()
        .map(ClockIdentity)
}

/// Reads pmc's answers: `key value` lines.
pub fn parse(output: &str) -> Option<Status> {
    let value = |key: &str| {
        output.lines().find_map(|line| {
            let mut words = line.split_whitespace();
            (words.next() == Some(key)).then(|| words.next()).flatten()
        })
    };
    let present = value("gmPresent") == Some("true");
    let grandmaster = value("gmIdentity").and_then(identity)?;
    let own_clock = value("clockIdentity").and_then(identity)?;
    let parent = value("parentPortIdentity").and_then(identity);
    let delay: f64 = value("peerMeanPathDelay")
        .and_then(|text| text.parse().ok())
        .unwrap_or(0.0);
    let as_capable = value("asCapable") == Some("1");
    let domain = value("domainNumber")
        .and_then(|text| text.parse().ok())
        .unwrap_or(0);
    let offset = value("master_offset").and_then(|text| text.parse().ok());
    // PORT_DATA_SET's delayMechanism: 1 end to end, 2 peer to peer.
    let end_to_end = value("delayMechanism") == Some("1");
    let faulty = value("portState") == Some("FAULTY");
    let mut path = Vec::new();
    if present {
        path.push(grandmaster);
        if let Some(parent) = parent.filter(|parent| *parent != grandmaster && *parent != own_clock)
        {
            path.push(parent);
        }
    }
    path.push(own_clock);
    Some(Status {
        gptp: Gptp {
            grandmaster: if present { grandmaster } else { own_clock },
            domain,
            propagation_delay: delay.max(0.0) as u32,
            as_capable,
            path,
        },
        own_clock,
        offset,
        end_to_end,
        faulty,
    })
}

/// Asks ptp4l once, as gPTP, then as standard PTP: ptp4l ignores
/// management messages whose transportSpecific is not its own.
fn ask(socket: &str, client: &str) -> Option<Status> {
    ask_as(socket, client, "1").or_else(|| ask_as(socket, client, "0"))
}

fn ask_as(socket: &str, client: &str, transport: &str) -> Option<Status> {
    let output = Command::new("pmc")
        .args(["-u", "-b", "0", "-t", transport, "-s", socket, "-i", client])
        .args([
            "GET TIME_STATUS_NP",
            "GET PORT_DATA_SET",
            "GET PORT_DATA_SET_NP",
            "GET PARENT_DATA_SET",
            "GET DEFAULT_DATA_SET",
        ])
        .output()
        .ok()?;
    parse(&String::from_utf8_lossy(&output.stdout))
}

/// Asks ptp4l every few seconds and sends what changed, until `stop`.
pub fn watch<T: Send + 'static>(
    socket: String,
    stop: Arc<AtomicBool>,
    sender: Sender<T>,
    wrap: fn(Option<Status>) -> T,
) {
    let client = triib_store::paths::runtime_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join(format!("pmc.{}", std::process::id()));
    let _ = std::fs::create_dir_all(client.parent().unwrap_or(&client));
    let client = client.display().to_string();
    let _ = std::thread::Builder::new()
        .name("gptp".into())
        .spawn(move || {
            let mut last: Option<Option<Status>> = None;
            while !stop.load(Ordering::Relaxed) {
                let status = ask(&socket, &client);
                if last.as_ref() != Some(&status) {
                    if sender.send(wrap(status.clone())).is_err() {
                        return;
                    }
                    last = Some(status);
                }
                std::thread::sleep(POLL);
            }
            let _ = std::fs::remove_file(&client);
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    const ANSWER: &str = "\
        gmPresent                  true
        gmIdentity                 0001f2.fffe.ff3b14
        portIdentity            f0a731.fffe.f40f14-1
        peerMeanPathDelay       305
        asCapable               1
        parentPortIdentity                    0001f2.fffe.ff3b14-6
        clockIdentity           f0a731.fffe.f40f14
        domainNumber            0
        master_offset              -12
    ";

    #[test]
    fn reads_what_the_entities_report() {
        let status = parse(ANSWER).unwrap();
        assert_eq!(status.own_clock, ClockIdentity(0xf0a7_31ff_fef4_0f14));
        assert_eq!(
            status.gptp.grandmaster,
            ClockIdentity(0x0001_f2ff_feff_3b14)
        );
        assert_eq!(status.gptp.propagation_delay, 305);
        assert_eq!(status.offset, Some(-12));
        assert!(status.gptp.as_capable);
        assert!(!status.end_to_end);
        let lite = ANSWER.replace("asCapable", "delayMechanism 1\n asCapable");
        assert!(parse(&lite).unwrap().end_to_end);
        assert!(!status.faulty);
        let faulty = ANSWER.replace("asCapable", "portState FAULTY\n asCapable");
        assert!(parse(&faulty).unwrap().faulty);
        assert_eq!(
            status.gptp.path,
            [
                ClockIdentity(0x0001_f2ff_feff_3b14),
                ClockIdentity(0xf0a7_31ff_fef4_0f14)
            ]
        );
        assert_eq!(parse("nothing here"), None);
    }
}
