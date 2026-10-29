//! The MRP the daemon declares on its port for every endpoint at once.
//! MRP runs per port, not per entity, so the endpoints share one MSRP and
//! one MVRP participant: two participants on one point-to-point port do
//! not hear each other's Leave, and one withdrawing VLAN 2 would take it
//! from the bridge while the other still needs it. VLAN 2 and the SR class
//! domains are declared once for the port; each talker's stream and each
//! listener's declaration is an attribute of its own.

use std::time::Duration;

use atdecc::StreamId;
use avb_mrp::msrp::{self, Domain};
use avb_mrp::{Participant, mvrp};

/// The VLAN every endpoint's streams use.
pub const VLAN: u16 = 2;

pub struct Port {
    /// A stream's Talker Advertise and Talker Failed displace each other
    /// in it, as a bridge turns a talker failed and back with Joins alone.
    pub msrp: Participant,
    pub mvrp: Participant,
    /// The SR classes the endpoints take part in: both, since a listener
    /// may hear a talker in either. A bridge passes a talker on only to a
    /// port whose neighbour declares the talker's class; elsewhere the
    /// port is the edge of that class's domain.
    domains: [Domain; 2],
}

impl Port {
    pub fn new(seed: u64) -> Self {
        Self {
            msrp: Participant::new(msrp::FORMAT, Duration::ZERO, seed).with_twins(msrp::twin),
            mvrp: Participant::new(mvrp::FORMAT, Duration::ZERO, seed ^ 0x5555),
            domains: [Domain::CLASS_A, Domain::CLASS_B],
        }
    }

    /// Declares what every endpoint takes part in: VLAN 2 and its SR
    /// classes.
    pub fn join(&mut self, since: Duration) {
        self.mvrp.declare(since, mvrp::VID, mvrp::value(VLAN), None);
        for domain in &self.domains {
            self.msrp
                .declare(since, msrp::attribute::DOMAIN, domain.value(), None);
        }
    }

    /// Withdraws VLAN 2 and the SR classes, as the port leaves AVB for AVB
    /// Lite: the one time the endpoints no longer need them.
    pub fn leave(&mut self, since: Duration) {
        for domain in &self.domains {
            self.msrp
                .withdraw(since, msrp::attribute::DOMAIN, &domain.value()[..1]);
        }
        self.mvrp.withdraw(since, mvrp::VID, &mvrp::value(VLAN));
    }

    /// Withdraws talkers' streams, as the daemon stops or starts again on
    /// a changed file. VLAN 2 and the domain stay declared: the endpoints
    /// that remain, or the daemon after it, still need them, and a new
    /// participant declares them again at once.
    pub fn withdraw_streams(
        &mut self,
        since: Duration,
        streams: impl IntoIterator<Item = StreamId>,
    ) {
        for stream in streams {
            self.msrp.withdraw(
                since,
                msrp::attribute::TALKER_ADVERTISE,
                &stream.0.to_be_bytes(),
            );
        }
    }

    /// Runs both participants' timers to `now`.
    pub fn handle_timeout(&mut self, now: Duration) {
        self.msrp.handle_timeout(now);
        self.mvrp.handle_timeout(now);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use avb_mrp::mrpdu::{self, Event, Value};

    const ONE: StreamId = StreamId(0xf0a7_31f4_0f14_0000);
    const TWO: StreamId = StreamId(0xf0a7_31f4_0f14_0100);

    /// A talker advertise for `stream`, to 91:e0:f0:00:xx:00 on VLAN 2.
    fn advertise(stream: StreamId) -> Vec<u8> {
        let mut value = stream.0.to_be_bytes().to_vec();
        value.extend_from_slice(&[0x91, 0xe0, 0xf0, 0x00, stream.0 as u8, 0x00]);
        value.extend_from_slice(&VLAN.to_be_bytes());
        value.extend_from_slice(&224u16.to_be_bytes());
        value.extend_from_slice(&1u16.to_be_bytes());
        value.push(0x60);
        value.extend_from_slice(&500_000u32.to_be_bytes());
        value
    }

    /// What the port sends from `from` to `to`, by participant, decoded.
    fn sent(port: &mut Port, from: Duration, to: Duration) -> (Vec<Value>, Vec<Value>) {
        let (mut msrp, mut mvrp) = (Vec::new(), Vec::new());
        let mut now = from;
        while now <= to {
            port.handle_timeout(now);
            while let Some(pdu) = port.msrp.poll_transmit() {
                msrp.extend(mrpdu::decode(&pdu, &msrp::FORMAT).values);
            }
            while let Some(pdu) = port.mvrp.poll_transmit() {
                mvrp.extend(mrpdu::decode(&pdu, &mvrp::FORMAT).values);
            }
            now += Duration::from_millis(50);
        }
        (msrp, mvrp)
    }

    fn withdrawn(values: &[Value], attribute_type: u8) -> Vec<Vec<u8>> {
        values
            .iter()
            .filter(|value| value.attribute_type == attribute_type && value.event == Event::Lv)
            .map(|value| value.value.clone())
            .collect()
    }

    /// Two talkers on the port, declared and sent until the declarations
    /// are past their New phase, as a running talker's are: MRP drops a
    /// declaration still new without a Leave.
    fn two_talkers() -> Port {
        let mut port = Port::new(7);
        port.join(Duration::ZERO);
        for stream in [ONE, TWO] {
            port.msrp.declare(
                Duration::ZERO,
                msrp::attribute::TALKER_ADVERTISE,
                advertise(stream),
                None,
            );
        }
        let (msrp, mvrp) = sent(&mut port, Duration::ZERO, Duration::from_secs(3));
        let vlans: Vec<&Value> = mvrp.iter().filter(|value| value.event.declares()).collect();
        assert!(!vlans.is_empty());
        assert!(
            vlans
                .iter()
                .all(|value| value.attribute_type == mvrp::VID && value.value == mvrp::value(VLAN))
        );
        assert!(msrp.iter().any(|value| {
            value.attribute_type == msrp::attribute::DOMAIN && value.event.declares()
        }));
        port
    }

    #[test]
    fn one_talker_stopping_leaves_the_vlan_and_domain() {
        let mut port = two_talkers();
        let since = Duration::from_secs(4);
        port.withdraw_streams(since, [ONE]);
        let (msrp, mvrp) = sent(&mut port, since, since + Duration::from_secs(1));
        let streams = withdrawn(&msrp, msrp::attribute::TALKER_ADVERTISE);
        assert!(!streams.is_empty());
        assert!(
            streams
                .iter()
                .all(|value| value[..8] == ONE.0.to_be_bytes())
        );
        assert!(withdrawn(&msrp, msrp::attribute::DOMAIN).is_empty());
        assert!(withdrawn(&mvrp, mvrp::VID).is_empty());
    }

    /// Starting again on a changed file: the old port withdraws every
    /// stream but not the VLAN or domain, and the new one declares them at
    /// once, so the bridge never sees VLAN 2 withdrawn.
    #[test]
    fn starting_again_keeps_the_vlan_declared() {
        let mut old = two_talkers();
        let since = Duration::from_secs(4);
        old.withdraw_streams(since, [ONE, TWO]);
        let (msrp, mvrp) = sent(&mut old, since, since + Duration::from_millis(400));
        assert!(withdrawn(&msrp, msrp::attribute::TALKER_ADVERTISE).len() >= 2);
        assert!(withdrawn(&msrp, msrp::attribute::DOMAIN).is_empty());
        assert!(withdrawn(&mvrp, mvrp::VID).is_empty());

        let mut new = Port::new(8);
        new.join(Duration::ZERO);
        let (_, mvrp) = sent(&mut new, Duration::ZERO, Duration::from_millis(500));
        assert!(mvrp.iter().any(|value| {
            value.attribute_type == mvrp::VID
                && value.value == mvrp::value(VLAN)
                && value.event.declares()
        }));
    }

    /// The port declares class B's domain beside class A's, with no
    /// class B talker of its own, so its listeners hear class B talkers.
    #[test]
    fn both_classes_are_declared() {
        let mut port = Port::new(9);
        port.join(Duration::ZERO);
        let (msrp, _) = sent(&mut port, Duration::ZERO, Duration::from_millis(500));
        let mut classes: Vec<u8> = msrp
            .iter()
            .filter(|value| value.attribute_type == msrp::attribute::DOMAIN)
            .map(|value| value.value[0])
            .collect();
        classes.sort_unstable();
        classes.dedup();
        assert_eq!(
            classes,
            [Domain::CLASS_B.class_id, Domain::CLASS_A.class_id]
        );
    }

    #[test]
    fn only_leaving_for_avb_lite_withdraws_the_vlan() {
        let mut port = two_talkers();
        let since = Duration::from_secs(4);
        port.leave(since);
        let (msrp, mvrp) = sent(&mut port, since, since + Duration::from_secs(1));
        assert!(!withdrawn(&mvrp, mvrp::VID).is_empty());
        assert!(!withdrawn(&msrp, msrp::attribute::DOMAIN).is_empty());
    }
}
