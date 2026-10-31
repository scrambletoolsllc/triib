//! gPTP as gptpd runs it on the interface: its status, read from its
//! socket as it comes, on every change and every few seconds. gptpd also
//! decides whether the port runs AVB Lite (profile 2.2), which the
//! endpoints follow.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};

use atdecc::ClockIdentity;
use atdecc::entity::Gptp;
use atdecc::lite::FallbackReason;
use gptp::status::{Fallback, Profile};

/// How long gptpd may say nothing before it counts as gone: it sends its
/// status every 2 s.
#[cfg(unix)]
const SILENCE: Duration = Duration::from_secs(5);
/// How often a gptpd that does not answer is asked again.
const RETRY: Duration = Duration::from_secs(2);
/// How long the last grandmaster stands while gptpd names none, or does
/// not answer, as across its restart: entities naming none for a moment
/// would have controllers see the grandmaster change twice.
const HOLD: Duration = Duration::from_secs(5);

/// Passes gptpd's statuses on, but for one naming no grandmaster, or
/// none at all, within [`HOLD`] of the last that named one.
#[derive(Default)]
struct Hold {
    named: bool,
    lost: Option<Instant>,
}

impl Hold {
    /// What to send for `status` at `now`, if anything.
    fn take(&mut self, now: Instant, status: Option<Status>) -> Option<Option<Status>> {
        if status
            .as_ref()
            .is_some_and(|status| status.gptp.grandmaster != ClockIdentity(0))
        {
            self.named = true;
            self.lost = None;
            return Some(status);
        }
        if self.named {
            let since = *self.lost.get_or_insert(now);
            if now.duration_since(since) < HOLD {
                return None;
            }
            self.named = false;
        }
        Some(status)
    }
}

/// What gptpd says, as an entity reports it, and this computer's own PTP
/// Instance; `None` when gptpd does not answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub gptp: Gptp,
    pub own_clock: ClockIdentity,
    /// The priority1 gptpd gives this computer: 255 never grandmaster.
    pub priority1: u8,
    /// This computer is the grandmaster.
    pub grandmaster_is_us: bool,
    /// AVB Lite, and why, where gptpd fell back to it.
    pub lite: Option<FallbackReason>,
    /// How far the last Sync stood from gptpd's mapping, in nanoseconds.
    pub residual: Option<i64>,
    /// The neighbor's rate against this computer's clock, in parts per
    /// billion off one.
    pub neighbor_rate_ppb: Option<i64>,
}

fn identity(clock: gptp::ClockIdentity) -> ClockIdentity {
    ClockIdentity(clock.0)
}

/// The fallback reason as GET_LITE_STATUS gives it.
fn reason(fallback: Fallback) -> FallbackReason {
    match fallback {
        Fallback::None => FallbackReason::NONE,
        Fallback::EndpointTlv => FallbackReason::ENDPOINT_TLV,
        Fallback::PdelayUnanswered => FallbackReason::PDELAY_UNANSWERED,
        Fallback::MultipleResponders => FallbackReason::MULTIPLE_RESPONDERS,
        Fallback::Configured => FallbackReason::CONFIGURED,
    }
}

/// What gptpd's status says for `interface`, its port or, failing that,
/// its first.
pub fn status_of(status: &gptp::status::Status, interface: &str) -> Status {
    let own_clock = identity(status.clock);
    let port = status
        .ports
        .iter()
        .find(|port| port.name == interface)
        .or_else(|| status.ports.first());
    let mut path: Vec<ClockIdentity> = status.path.iter().copied().map(identity).collect();
    if path.is_empty() {
        path.push(own_clock);
    }
    let lite = (status.profile == Profile::AvbLite).then(|| match status.fallback {
        Fallback::None => FallbackReason::CONFIGURED,
        fallback => reason(fallback),
    });
    Status {
        gptp: Gptp {
            // None while gptpd names none, rather than this computer,
            // which would tell controllers it leads.
            grandmaster: status
                .grandmaster
                .as_ref()
                .map_or(ClockIdentity(0), |grandmaster| {
                    identity(grandmaster.identity)
                }),
            domain: status.domain,
            propagation_delay: port
                .and_then(|port| port.link_delay)
                .map_or(0, |delay| delay.clamp(0, i64::from(u32::MAX)) as u32),
            as_capable: port.is_some_and(|port| port.up && port.as_capable),
            path,
        },
        own_clock,
        priority1: status.priority1,
        grandmaster_is_us: status
            .grandmaster
            .as_ref()
            .is_some_and(|grandmaster| grandmaster.is_us),
        lite,
        residual: status.residual,
        neighbor_rate_ppb: port
            .and_then(|port| port.neighbor_rate_ratio)
            .map(|ratio| ((ratio - 1.0) * 1e9).round() as i64),
    }
}

/// Why gptpd will not start on `interface`, as it wrote it, where it
/// refused.
pub fn refused(interface: &str) -> Option<String> {
    if interface.is_empty() || interface.contains(['/', '\0']) {
        return None;
    }
    let reason = std::fs::read_to_string(format!("/run/gptpd/{interface}.refused")).ok()?;
    let reason = reason.trim();
    (!reason.is_empty()).then(|| reason.to_owned())
}

/// Reads gptpd's status for `interface` as it comes, and sends what
/// changed, until `stop`; `None` while gptpd does not answer.
pub fn watch<T: Send + 'static>(
    interface: String,
    stop: Arc<AtomicBool>,
    sender: Sender<T>,
    wrap: fn(Option<Status>) -> T,
) {
    let _ = std::thread::Builder::new()
        .name("gptp".into())
        .spawn(move || {
            // gptpd runs where Unix sockets do; elsewhere it never answers.
            #[cfg(not(unix))]
            let _ = &interface;
            let mut last: Option<Option<Status>> = None;
            let mut hold = Hold::default();
            let mut send = |status: Option<Status>| {
                let Some(status) = hold.take(Instant::now(), status) else {
                    return true;
                };
                if last.as_ref() == Some(&status) {
                    return true;
                }
                last = Some(status.clone());
                sender.send(wrap(status)).is_ok()
            };
            while !stop.load(std::sync::atomic::Ordering::Relaxed) {
                #[cfg(unix)]
                if let Ok(mut client) = gptp::status::Client::connect(&interface) {
                    while !stop.load(std::sync::atomic::Ordering::Relaxed) {
                        match client.next(Some(SILENCE)) {
                            Ok(Some(gptp::status::Block::Status(status))) => {
                                if !send(Some(status_of(&status, &interface))) {
                                    return;
                                }
                            }
                            Ok(Some(gptp::status::Block::Reply(_))) => {}
                            Ok(None) | Err(_) => break,
                        }
                    }
                }
                if !send(None) {
                    return;
                }
                std::thread::sleep(RETRY);
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use gptp::relay::Role;
    use gptp::status::{Counters, Grandmaster, Port};

    const OURS: gptp::ClockIdentity = gptp::ClockIdentity(0xa010_a3ff_fea5_1726);
    const MOTU: gptp::ClockIdentity = gptp::ClockIdentity(0x0001_f2ff_feff_3b14);

    fn status() -> gptp::status::Status {
        gptp::status::Status {
            clock: OURS,
            domain: 0,
            priority1: 248,
            profile: Profile::Gptp,
            fallback: Fallback::None,
            grandmaster: Some(Grandmaster {
                identity: MOTU,
                priority1: 246,
                clock_class: 248,
                clock_accuracy: 0xfe,
                offset_scaled_log_variance: 0x436a,
                priority2: 248,
                steps_removed: 1,
                is_us: false,
            }),
            path: vec![MOTU, OURS],
            generation: 4,
            residual: Some(-12),
            ports: vec![Port {
                name: "enp2s0".into(),
                up: true,
                as_capable: true,
                role: Role::TimeReceiver,
                link_delay: Some(305),
                neighbor: None,
                neighbor_rate_ratio: Some(1.000_009_5),
            }],
            counters: Counters::default(),
        }
    }

    #[test]
    fn reads_what_the_entities_report() {
        let status = status_of(&status(), "enp2s0");
        assert_eq!(status.own_clock, ClockIdentity(OURS.0));
        assert_eq!(status.gptp.grandmaster, ClockIdentity(MOTU.0));
        assert_eq!(status.gptp.propagation_delay, 305);
        assert!(status.gptp.as_capable);
        assert_eq!(
            status.gptp.path,
            [ClockIdentity(MOTU.0), ClockIdentity(OURS.0)]
        );
        assert_eq!(status.residual, Some(-12));
        assert_eq!(status.neighbor_rate_ppb, Some(9_500));
        assert_eq!(status.priority1, 248);
        assert!(!status.grandmaster_is_us);
        assert_eq!(status.lite, None);
    }

    /// As grandmaster the entities name this computer's clock as gPTP's
    /// grandmaster; with none, they name none.
    #[test]
    fn this_computer_can_be_the_grandmaster() {
        let mut ours = status();
        ours.grandmaster = Some(Grandmaster {
            identity: OURS,
            priority1: 248,
            steps_removed: 0,
            is_us: true,
            ..ours.grandmaster.unwrap()
        });
        ours.path = vec![OURS];
        let status = status_of(&ours, "enp2s0");
        assert!(status.grandmaster_is_us);
        assert_eq!(status.gptp.grandmaster, ClockIdentity(OURS.0));
        ours.grandmaster = None;
        ours.path.clear();
        let status = status_of(&ours, "enp2s0");
        assert_eq!(status.gptp.grandmaster, ClockIdentity(0));
        assert!(!status.grandmaster_is_us);
        assert_eq!(status.gptp.path, [ClockIdentity(OURS.0)]);
    }

    /// gptpd naming no grandmaster, or not answering, for less than 5 s
    /// after it named one changes nothing the entities say; longer does.
    #[test]
    fn the_last_grandmaster_holds_for_a_while() {
        let named = status_of(&status(), "enp2s0");
        let mut none = status();
        none.grandmaster = None;
        let none = status_of(&none, "enp2s0");
        let mut hold = Hold::default();
        let start = Instant::now();
        let at = |seconds: u64| start + Duration::from_secs(seconds);
        assert_eq!(
            hold.take(at(0), Some(named.clone())),
            Some(Some(named.clone()))
        );
        assert_eq!(hold.take(at(1), Some(none.clone())), None);
        assert_eq!(hold.take(at(3), None), None);
        assert_eq!(
            hold.take(at(4), Some(named.clone())),
            Some(Some(named.clone()))
        );
        assert_eq!(hold.take(at(5), None), None);
        assert_eq!(hold.take(at(10), None), Some(None));
        // With none named since, nothing is held.
        assert_eq!(hold.take(at(11), Some(none.clone())), Some(Some(none)));
    }

    #[test]
    fn avb_lite_comes_with_its_reason() {
        let mut lite = status();
        lite.profile = Profile::AvbLite;
        lite.fallback = Fallback::PdelayUnanswered;
        assert_eq!(
            status_of(&lite, "enp2s0").lite,
            Some(FallbackReason::PDELAY_UNANSWERED)
        );
        lite.fallback = Fallback::None;
        assert_eq!(
            status_of(&lite, "enp2s0").lite,
            Some(FallbackReason::CONFIGURED)
        );
        // A port down is not asCapable, whatever it said last.
        let mut down = status();
        down.ports[0].up = false;
        assert!(!status_of(&down, "enp2s0").gptp.as_capable);
    }
}
