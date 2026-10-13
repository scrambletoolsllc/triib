//! A controller and two entities, a talker and a listener, on one
//! network in memory: the controller finds and reads both, binds the
//! listener to the talker, the listener probes the talker for its stream,
//! and unbinding undoes it.

use std::collections::VecDeque;
use std::time::Duration;

use atdecc::controller::{Config, Controller, Event, Outcome};
use atdecc::descriptor::DescriptorType;
use atdecc::entity::{EndpointModel, Entity, EntityEvent, StreamModel};
use atdecc::model::EnumerationState;
use atdecc::stream_format::StreamFormat;
use atdecc::{
    ADP_ACMP_MULTICAST, ClockIdentity, EntityId, EntityModelId, Instant, MacAddress, StreamId,
};

const CONTROLLER_MAC: MacAddress = MacAddress([0x02, 0, 0, 0, 0, 1]);
const HOST_MAC: MacAddress = MacAddress([0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b]);
const TALKER: EntityId = EntityId(0x9c6b_00ff_0030_9a2b);
const LISTENER: EntityId = EntityId(0x9c6b_00ff_0130_9a2b);
const AAF_48K_8CH: StreamFormat = StreamFormat(0x0205_0220_0200_6000);
const STREAM: StreamId = StreamId(0x9c6b_0030_9a2b_0000);
const DESTINATION: MacAddress = MacAddress([0x91, 0xe0, 0xf0, 0x00, 0x12, 0x34]);

fn stream(name: &str) -> StreamModel {
    StreamModel {
        name: name.into(),
        formats: vec![AAF_48K_8CH],
        current_format: AAF_48K_8CH,
        channels: 8,
        class_b: false,
    }
}

fn model(entity_id: EntityId, name: &str, talker: bool) -> EndpointModel {
    EndpointModel {
        entity_id,
        entity_model_id: EntityModelId(0x8c1f_6436_c000_0002),
        entity_name: name.into(),
        group_name: String::new(),
        vendor_name: "Scramble Tools".into(),
        model_name: "triib endpoint".into(),
        firmware_version: "0.1.0".into(),
        serial_number: String::new(),
        mac: HOST_MAC,
        interface_name: "enp2s0".into(),
        clock_identity: ClockIdentity(0x9c6b_00ff_fe30_9a2b),
        sampling_rates: vec![48_000],
        current_sampling_rate: 48_000,
        outputs: if talker {
            vec![stream("Host out")]
        } else {
            vec![]
        },
        inputs: if talker {
            vec![]
        } else {
            vec![stream("Host in")]
        },
        clock_source: 0,
        clock_source_inputs: if talker { vec![] } else { vec![0] },
    }
}

struct Network {
    now: Instant,
    controller: Controller,
    talker: Entity,
    listener: Entity,
    entity_events: Vec<EntityEvent>,
    controller_events: Vec<Event>,
}

impl Network {
    fn new() -> Self {
        let mut talker = Entity::new(model(TALKER, "Talker", true), &[(STREAM, DESTINATION)]);
        let mut listener = Entity::new(model(LISTENER, "Listener", false), &[]);
        talker.start(Instant::ZERO);
        listener.start(Instant::ZERO);
        Network {
            now: Instant::ZERO,
            controller: Controller::new(Config::new(EntityId(0x0200_00ff_fe00_0001))),
            talker,
            listener,
            entity_events: Vec::new(),
            controller_events: Vec::new(),
        }
    }

    /// Runs everyone for `millis`, handing each frame to whoever it is for.
    fn run(&mut self, millis: u64) {
        let until = self.now + Duration::from_millis(millis);
        while self.now < until {
            self.now += Duration::from_millis(5);
            let now = self.now;
            self.controller.handle_timeout(now);
            self.talker.handle_timeout(now);
            self.listener.handle_timeout(now);
            // Frames in flight: their source, destination and octets.
            let mut frames: VecDeque<(MacAddress, MacAddress, Vec<u8>)> = VecDeque::new();
            loop {
                let mut out = [0; 1500];
                while let Ok(Some(transmit)) = self.controller.poll_transmit(&mut out) {
                    frames.push_back((
                        CONTROLLER_MAC,
                        transmit.destination,
                        out[..transmit.length].to_vec(),
                    ));
                }
                for entity in [&mut self.talker, &mut self.listener] {
                    while let Some((destination, bytes)) = entity.poll_transmit() {
                        frames.push_back((HOST_MAC, destination, bytes));
                    }
                }
                let Some((source, destination, bytes)) = frames.pop_front() else {
                    break;
                };
                let multicast = destination == ADP_ACMP_MULTICAST;
                if multicast || destination == CONTROLLER_MAC {
                    let _ = self.controller.handle_frame(now, source, &bytes);
                }
                if multicast || destination == HOST_MAC {
                    self.talker.handle_frame(now, source, &bytes);
                    self.listener.handle_frame(now, source, &bytes);
                }
            }
            for entity in [&mut self.talker, &mut self.listener] {
                self.entity_events
                    .extend(std::iter::from_fn(|| entity.poll_event()));
            }
            self.controller_events
                .extend(std::iter::from_fn(|| self.controller.poll_event()));
        }
    }
}

#[test]
fn a_controller_reads_both_entities() {
    let mut network = Network::new();
    network.controller.discover(None);
    network.run(3000);
    for (entity_id, name, stream) in [
        (
            TALKER,
            "Talker",
            (DescriptorType::STREAM_OUTPUT, "Host out"),
        ),
        (
            LISTENER,
            "Listener",
            (DescriptorType::STREAM_INPUT, "Host in"),
        ),
    ] {
        let model = network.controller.model(entity_id).expect("read");
        assert_eq!(model.state, EnumerationState::Complete, "{name}");
        assert_eq!(model.entity_name(), Some(name));
        assert_eq!(model.name_of(stream.0, 0), Some(stream.1));
        let milan = model.milan.expect("GET_MILAN_INFO answered");
        assert_eq!(milan.specification_version, Some([1, 3, 0, 0]));
        let info = model.stream_info(stream.0, 0).expect("GET_STREAM_INFO");
        assert_eq!(info.stream_format, AAF_48K_8CH);
    }
    let talker = network.controller.model(TALKER).unwrap();
    let output = talker
        .stream_info(DescriptorType::STREAM_OUTPUT, 0)
        .unwrap();
    assert_eq!(
        (output.stream_id, output.stream_dest_mac),
        (STREAM, DESTINATION)
    );
}

#[test]
fn binding_probes_the_talker_and_unbinding_undoes_it() {
    let mut network = Network::new();
    network.controller.discover(None);
    network.run(3000);
    let now = network.now;
    let bind = network.controller.connect(now, (TALKER, 0), (LISTENER, 0));
    network.run(1000);
    assert!(
        network
            .controller_events
            .contains(&Event::CommandFinished(bind, Outcome::Done)),
        "{:?}",
        network.controller_events
    );
    assert!(network.entity_events.iter().any(|event| matches!(
        event,
        EntityEvent::InputBound { index: 0, binding } if binding.talker == TALKER
    )));
    let settled = network.entity_events.iter().find_map(|event| match event {
        EntityEvent::InputSettled { index: 0, stream } => Some(*stream),
        _ => None,
    });
    let settled = settled.expect("the probe found the stream");
    assert_eq!(
        (settled.stream_id, settled.destination),
        (STREAM, DESTINATION)
    );
    let binding = network
        .controller
        .model(LISTENER)
        .and_then(|model| model.binding(0))
        .copied()
        .expect("the controller knows the binding");
    assert_eq!(binding.talker_stream(), Some((TALKER, 0)));

    let now = network.now;
    let unbind = network.controller.disconnect(now, (LISTENER, 0));
    network.run(1000);
    assert!(
        network
            .controller_events
            .contains(&Event::CommandFinished(unbind, Outcome::Done))
    );
    assert!(
        network
            .entity_events
            .contains(&EntityEvent::InputUnbound { index: 0 })
    );
    assert_eq!(network.listener.input_binding(0), None);
}

#[test]
fn a_restored_binding_probes_the_talker_again() {
    let mut network = Network::new();
    network.controller.discover(None);
    network.run(3000);
    // The listener starts again bound as it was, without a controller.
    let mut listener = Entity::new(model(LISTENER, "Listener", false), &[]);
    listener.start(network.now);
    let binding = atdecc::entity::InputBinding {
        talker: TALKER,
        talker_unique_id: 0,
        controller: EntityId(0),
        flags: atdecc::AcmpFlags::empty(),
    };
    listener.restore_binding(network.now, 0, binding);
    network.listener = listener;
    network.run(1000);
    assert!(
        network
            .entity_events
            .contains(&EntityEvent::InputBound { index: 0, binding })
    );
    let settled = network.entity_events.iter().find_map(|event| match event {
        EntityEvent::InputSettled { index: 0, stream } => Some(*stream),
        _ => None,
    });
    assert_eq!(
        settled.map(|stream| (stream.stream_id, stream.destination)),
        Some((STREAM, DESTINATION))
    );
    assert_eq!(network.listener.input_binding(0), Some(binding));
}

#[test]
fn a_talker_without_an_address_is_probed_again_until_it_has_one() {
    let mut network = Network::new();
    let mut talker = Entity::new(
        model(TALKER, "Talker", true),
        &[(STREAM, atdecc::entity::UNADDRESSED)],
    );
    talker.start(network.now);
    network.talker = talker;
    network.controller.discover(None);
    network.run(3000);
    let now = network.now;
    network.controller.connect(now, (TALKER, 0), (LISTENER, 0));
    network.run(1000);
    let settled = |network: &Network| {
        network.entity_events.iter().find_map(|event| match event {
            EntityEvent::InputSettled { index: 0, stream } => Some(*stream),
            _ => None,
        })
    };
    assert_eq!(settled(&network), None, "no address to settle on yet");
    // The address comes, and the next probe finds it.
    network.talker.set_output_destination(0, DESTINATION);
    network.run(5000);
    assert_eq!(
        settled(&network).map(|stream| stream.destination),
        Some(DESTINATION)
    );
}

#[test]
fn a_controller_reads_an_entitys_avb_lite_status() {
    use atdecc::ClockIdentity as Clock;
    use atdecc::lite::{FallbackReason, LiteFlags, LiteStatus, PtpProfile};

    let mut network = Network::new();
    let status = LiteStatus {
        interface: 0,
        flags: LiteFlags::CAPABLE.union(LiteFlags::ACTIVE),
        fallback_reason: FallbackReason::PDELAY_UNANSWERED,
        ptp_profile: PtpProfile::AVB_LITE_PTP,
        ptp_domain: 0,
        media_vlan_id: 2,
        unicast_fanout_limit: 2,
        link_speed: 1000,
        committed_egress: 0,
        grandmaster: Clock(0),
        offset_from_grandmaster: 0,
    };
    network.talker.set_lite_status(status);
    network.controller.discover(None);
    network.run(3000);
    let model = network.controller.model(TALKER).expect("read");
    assert_eq!(model.lite_supported, Some(true));
    assert_eq!(model.lite_status(0), Some(&status));
    // The listener has no AVB Lite to report.
    let listener = network.controller.model(LISTENER).expect("read");
    assert_ne!(listener.lite_supported, Some(true));
    assert_eq!(listener.lite_status(0), None);
}

#[test]
fn a_controller_allows_escalation_with_set_lite_config() {
    use atdecc::ClockIdentity as Clock;
    use atdecc::aecp::VendorUniquePdu;
    use atdecc::lite::{
        FallbackReason, LiteConfigFlags, LiteFlags, LiteStatus, PtpProfile, encode_set_lite_config,
    };

    let mut network = Network::new();
    let status = LiteStatus {
        interface: 0,
        flags: LiteFlags::CAPABLE.union(LiteFlags::ACTIVE),
        fallback_reason: FallbackReason::ENDPOINT_TLV,
        ptp_profile: PtpProfile::AVB_LITE_PTP,
        ptp_domain: 0,
        media_vlan_id: 2,
        unicast_fanout_limit: 2,
        link_speed: 1000,
        committed_egress: 0,
        grandmaster: Clock(0),
        offset_from_grandmaster: 0,
    };
    network.talker.set_lite_status(status);
    let now = network.now;
    let controller = MacAddress([0x02, 0, 0, 0, 0, 1]);
    let set = |talker: &mut Entity, interface: u16, flags: u8| {
        let mut out = [0; 64];
        let length = encode_set_lite_config(
            TALKER,
            EntityId(0x0200_00ff_fe00_0001),
            9,
            interface,
            LiteConfigFlags(flags),
            &mut out,
        )
        .unwrap();
        talker.handle_frame(now, controller, &out[..length]);
        let (to, response) = talker.poll_transmit().expect("an answer");
        assert_eq!(to, controller);
        let response = VendorUniquePdu::decode(&response).unwrap();
        (response.header.status, response.payload.to_vec())
    };
    // Allowed: the status says so, and the daemon hears of it.
    let (status_code, payload) = set(&mut network.talker, 0, 0x01);
    assert_eq!(status_code, 0);
    assert_eq!(payload, [0x00, 0x01, 0x00, 0x00, 0x01, 0, 0, 0]);
    assert!(
        network
            .talker
            .lite_status()
            .unwrap()
            .flags
            .contains(LiteFlags::ESCALATION_ALLOWED)
    );
    assert_eq!(
        network.talker.poll_event(),
        Some(EntityEvent::LiteConfigChanged {
            escalation_allowed: true
        })
    );
    // A reserved bit, or an interface it does not have, changes nothing.
    assert_eq!(set(&mut network.talker, 0, 0x03).0, 7);
    assert_eq!(set(&mut network.talker, 1, 0x00).0, 2);
    assert!(
        network
            .talker
            .lite_status()
            .unwrap()
            .flags
            .contains(LiteFlags::ESCALATION_ALLOWED)
    );
    assert_eq!(network.talker.poll_event(), None);
}

#[test]
fn a_controller_sets_an_entitys_lite_config() {
    use atdecc::ClockIdentity as Clock;
    use atdecc::controller::Outcome;
    use atdecc::lite::{FallbackReason, LiteFlags, LiteStatus, PtpProfile};

    let mut network = Network::new();
    network.talker.set_lite_status(LiteStatus {
        interface: 0,
        flags: LiteFlags::CAPABLE.union(LiteFlags::ACTIVE),
        fallback_reason: FallbackReason::ENDPOINT_TLV,
        ptp_profile: PtpProfile::AVB_LITE_PTP,
        ptp_domain: 0,
        media_vlan_id: 2,
        unicast_fanout_limit: 2,
        link_speed: 1000,
        committed_egress: 0,
        grandmaster: Clock(0),
        offset_from_grandmaster: 0,
    });
    network.controller.discover(None);
    network.run(3000);
    let now = network.now;
    let command = network.controller.set_lite_config(now, TALKER, 0, true);
    network.run(1000);
    assert!(
        network.controller_events.iter().any(
            |event| matches!(event, Event::CommandFinished(id, Outcome::Done) if *id == command)
        )
    );
    let model = network.controller.model(TALKER).expect("read");
    assert!(
        model
            .lite_status(0)
            .unwrap()
            .flags
            .contains(LiteFlags::ESCALATION_ALLOWED)
    );
}

#[test]
fn the_sampling_rate_and_stream_formats_move_together() {
    use atdecc::descriptor::SamplingRate;

    // AAF, 8 channels of 32-bit, at 96 kHz: 12 samples a frame.
    const AAF_96K_8CH: StreamFormat = StreamFormat(0x0207_0220_0200_c000);
    let mut network = Network::new();
    let mut model = model(TALKER, "Talker", true);
    model.sampling_rates = vec![48_000, 96_000];
    model.outputs[0].formats = vec![AAF_48K_8CH, AAF_96K_8CH];
    let mut talker = Entity::new(model, &[(STREAM, DESTINATION)]);
    talker.start(network.now);
    network.talker = talker;
    network.controller.discover(None);
    network.run(3000);

    // The rate goes to 96 kHz, and the stream with it.
    let now = network.now;
    network.controller.set_sampling_rate(
        now,
        TALKER,
        DescriptorType::AUDIO_UNIT,
        0,
        SamplingRate(96_000),
    );
    network.run(1000);
    assert_eq!(network.talker.model().current_sampling_rate, 96_000);
    assert_eq!(
        network.talker.model().outputs[0].current_format,
        AAF_96K_8CH
    );
    assert!(
        network
            .entity_events
            .contains(&EntityEvent::StreamFormatChanged {
                descriptor_type: DescriptorType::STREAM_OUTPUT,
                index: 0,
                format: AAF_96K_8CH,
            })
    );

    // A stream format at 48 kHz brings the rate back.
    let now = network.now;
    network.controller.set_stream_format(
        now,
        TALKER,
        DescriptorType::STREAM_OUTPUT,
        0,
        AAF_48K_8CH,
    );
    network.run(1000);
    assert_eq!(network.talker.model().current_sampling_rate, 48_000);
    assert!(
        network
            .entity_events
            .contains(&EntityEvent::SamplingRateChanged(48_000))
    );
    // The controller hears both, unsolicited or in answers.
    let read = network.controller.model(TALKER).unwrap();
    assert_eq!(
        read.audio_units()
            .next()
            .map(|unit| unit.current_sampling_rate.0),
        Some(48_000)
    );
    assert_eq!(
        read.streams(false)
            .next()
            .map(|stream| stream.current_format),
        Some(AAF_48K_8CH)
    );
}

/// A media clock output, CRF, has no stream port, clusters or map: the
/// audio output after it has the one port, mapped to its own stream, and
/// a new rate moves the audio alone.
#[test]
fn a_media_clock_stream_has_no_port_and_keeps_its_rate() {
    use atdecc::descriptor::SamplingRate;

    const CRF_48K: StreamFormat = StreamFormat(0x0410_6001_0000_bb80);
    const AAF_96K_8CH: StreamFormat = StreamFormat(0x0207_0220_0200_c000);
    let mut network = Network::new();
    let mut model = model(TALKER, "Talker", true);
    model.sampling_rates = vec![48_000, 96_000];
    model.outputs[0].formats = vec![AAF_48K_8CH, AAF_96K_8CH];
    model.outputs.insert(
        0,
        StreamModel {
            name: "Media Clock Out 1".into(),
            formats: vec![CRF_48K],
            current_format: CRF_48K,
            channels: 0,
            class_b: false,
        },
    );
    let clock_stream = StreamId(0x9c6b_0030_9a2b_0001);
    let mut talker = Entity::new(model, &[(clock_stream, DESTINATION), (STREAM, DESTINATION)]);
    talker.start(network.now);
    network.talker = talker;
    network.controller.discover(None);
    network.run(3000);

    let read = network.controller.model(TALKER).expect("read");
    assert_eq!(read.state, EnumerationState::Complete);
    assert_eq!(read.streams(false).count(), 2);
    let ports: Vec<_> = read.stream_ports(false).collect();
    assert_eq!(ports.len(), 1, "the audio output's alone");
    let streams: Vec<u16> = read
        .static_mappings(&ports[0])
        .map(|mapping| mapping.stream_index)
        .collect();
    assert_eq!(streams, [1; 8]);

    let now = network.now;
    network.controller.set_sampling_rate(
        now,
        TALKER,
        DescriptorType::AUDIO_UNIT,
        0,
        SamplingRate(96_000),
    );
    network.run(1000);
    let outputs = &network.talker.model().outputs;
    assert_eq!(
        (outputs[0].current_format, outputs[1].current_format),
        (CRF_48K, AAF_96K_8CH)
    );
}

/// A listener whose media clock input is a clock source: the controller
/// reads it as one, after the internal clock, flagged on its input alone,
/// and picking it tells the entity.
#[test]
fn a_media_clock_input_is_a_clock_source() {
    use atdecc::descriptor::{ClockSourceType, StreamFlags};

    const CRF_48K: StreamFormat = StreamFormat(0x0410_6001_0000_bb80);
    let mut network = Network::new();
    let mut model = model(LISTENER, "Listener", false);
    model.inputs.push(StreamModel {
        name: "Media Clock In 1".into(),
        formats: vec![CRF_48K],
        current_format: CRF_48K,
        channels: 0,
        class_b: false,
    });
    model.clock_source_inputs = vec![1];
    let mut listener = Entity::new(model, &[]);
    listener.start(network.now);
    network.listener = listener;
    network.controller.discover(None);
    network.run(3000);

    let read = network.controller.model(LISTENER).expect("read");
    let sources: Vec<(String, ClockSourceType, DescriptorType, u16)> = read
        .clock_sources()
        .map(|source| {
            (
                source.object_name.to_owned(),
                source.clock_source_type,
                source.location_type,
                source.location_index,
            )
        })
        .collect();
    assert_eq!(sources.len(), 2);
    assert_eq!(sources[0].1, ClockSourceType::INTERNAL);
    assert_eq!(
        sources[1],
        (
            "Media Clock In 1".to_owned(),
            ClockSourceType::INPUT_STREAM,
            DescriptorType::STREAM_INPUT,
            1
        )
    );
    let synced: Vec<bool> = read
        .streams(true)
        .map(|stream| stream.stream_flags.contains(StreamFlags::CLOCK_SYNC_SOURCE))
        .collect();
    assert_eq!(synced, [false, true]);

    let now = network.now;
    network.controller.set_clock_source(now, LISTENER, 0, 1);
    network.run(500);
    assert!(
        network
            .entity_events
            .contains(&EntityEvent::ClockSourceChanged(1))
    );
    assert_eq!(network.listener.model().clock_source, 1);
}

/// A talker whose streams change takes over from itself: the controller
/// reads its new model without it ever going offline or restarting.
#[test]
fn a_talker_taking_over_from_itself_is_read_again_in_place() {
    let mut network = Network::new();
    network.controller.discover(None);
    network.run(3000);
    assert_eq!(
        network
            .controller
            .model(TALKER)
            .unwrap()
            .streams(false)
            .count(),
        1
    );
    network.controller_events.clear();

    let mut model = model(TALKER, "Talker", true);
    model.entity_model_id = EntityModelId(0x8c1f_6436_c0f3_59f0);
    model.outputs.push(stream("Host out 2"));
    let second = StreamId(0x9c6b_0030_9a2b_0001);
    let mut talker = Entity::new(model, &[(STREAM, DESTINATION), (second, DESTINATION)]);
    talker.take_over(network.now, &network.talker);
    talker.start(network.now);
    network.talker = talker;
    network.run(2000);

    let events = &network.controller_events;
    assert!(
        !events
            .iter()
            .any(|event| matches!(event, Event::EntityOffline(..) | Event::EntityRestarted(_))),
        "{events:?}"
    );
    assert!(events.contains(&Event::EntityChanged(TALKER)), "{events:?}");
    let read = network.controller.model(TALKER).unwrap();
    assert_eq!(read.state, EnumerationState::Complete);
    assert_eq!(read.streams(false).count(), 2);
}

/// An entity's input bound to its own output probes its own talker and
/// finds its stream, when its frames come back to it, as the daemon hands
/// them.
#[test]
fn an_input_bound_to_its_own_output_settles() {
    let mut network = Network::new();
    let mut model = model(TALKER, "Talker", true);
    model.inputs.push(stream("Host in"));
    model.clock_source_inputs = vec![0];
    let mut talker = Entity::new(model, &[(STREAM, DESTINATION)]);
    talker.start(network.now);
    network.talker = talker;
    network.controller.discover(None);
    network.run(3000);
    let now = network.now;
    let bind = network.controller.connect(now, (TALKER, 0), (TALKER, 0));
    network.run(1000);
    assert!(
        network
            .controller_events
            .contains(&Event::CommandFinished(bind, Outcome::Done)),
        "{:?}",
        network.controller_events
    );
    let settled = network.entity_events.iter().find_map(|event| match event {
        EntityEvent::InputSettled { index: 0, stream } => Some(*stream),
        _ => None,
    });
    assert_eq!(
        settled.map(|stream| (stream.stream_id, stream.destination)),
        Some((STREAM, DESTINATION))
    );
}

/// A listener hearing its own probe, as entities that hear their own
/// frames do, leaves it to the talker it names to answer.
#[test]
fn a_listener_does_not_answer_its_own_probe() {
    use atdecc::acmp::{AcmpMessageType, Acmpdu};

    let mut listener = Entity::new(model(LISTENER, "Listener", false), &[]);
    let now = Instant::ZERO;
    listener.start(now);
    let bind = Acmpdu::command(
        AcmpMessageType::BIND_RX_COMMAND,
        EntityId(0x0200_00ff_fe00_0001),
        (TALKER, 0),
        (LISTENER, 0),
        1,
    );
    let mut out = [0; 128];
    let length = bind.encode(&mut out).unwrap();
    listener.handle_frame(now, CONTROLLER_MAC, &out[..length]);
    listener.handle_timeout(now);
    let probe = std::iter::from_fn(|| listener.poll_transmit())
        .filter_map(|(_, bytes)| Acmpdu::decode(&bytes).ok())
        .find(|acmpdu| acmpdu.message_type == AcmpMessageType::PROBE_TX_COMMAND)
        .expect("the probe");
    let length = probe.encode(&mut out).unwrap();
    listener.handle_frame(now, HOST_MAC, &out[..length]);
    let answers: Vec<Acmpdu> = std::iter::from_fn(|| listener.poll_transmit())
        .filter_map(|(_, bytes)| Acmpdu::decode(&bytes).ok())
        .collect();
    assert!(answers.is_empty(), "{answers:?}");
}

/// A listener's input port has dynamic mappings, one to one at first: a
/// controller reads them, moves a channel to another cluster, and is
/// refused one that maps a cluster twice.
#[test]
fn a_listener_maps_its_inputs_dynamically() {
    use atdecc::aem::AudioMapping;

    let mut network = Network::new();
    network.controller.discover(None);
    network.run(3000);
    let port = (DescriptorType::STREAM_PORT_INPUT, 0);
    let read = |network: &Network| -> Vec<(u16, u16)> {
        network
            .controller
            .model(LISTENER)
            .and_then(|model| model.dynamic_mappings(port.0, port.1))
            .map(|mappings| {
                mappings
                    .map(|mapping| (mapping.stream_channel, mapping.cluster_offset))
                    .collect()
            })
            .unwrap_or_default()
    };
    assert_eq!(
        read(&network),
        (0..8).map(|channel| (channel, channel)).collect::<Vec<_>>()
    );
    let mapping = |stream_channel, cluster_offset| AudioMapping {
        stream_index: 0,
        stream_channel,
        cluster_offset,
        cluster_channel: 0,
    };
    // Channel 0 to cluster 7, which first lets go of channel 7.
    let now = network.now;
    network
        .controller
        .remove_audio_mappings(now, LISTENER, port, &[mapping(7, 7)]);
    network.run(500);
    let now = network.now;
    let added = network
        .controller
        .add_audio_mappings(now, LISTENER, port, &[mapping(0, 7)]);
    network.run(500);
    assert!(
        network
            .controller_events
            .contains(&Event::CommandFinished(added, Outcome::Done))
    );
    let mut expected: Vec<(u16, u16)> = (0..7).map(|channel| (channel, channel)).collect();
    expected.push((0, 7));
    expected.sort_by_key(|&(channel, cluster)| (channel, cluster));
    let mut got = read(&network);
    got.sort_by_key(|&(channel, cluster)| (channel, cluster));
    assert_eq!(got, expected);
    assert_eq!(
        network
            .entity_events
            .iter()
            .filter(|event| matches!(event, EntityEvent::MappingsChanged { index: 0 }))
            .count(),
        2
    );
    assert_eq!(network.listener.input_mappings(0).map(<[_]>::len), Some(8));
    // Cluster 1 takes channel 1 already.
    let now = network.now;
    let refused = network
        .controller
        .add_audio_mappings(now, LISTENER, port, &[mapping(2, 1)]);
    network.run(500);
    assert!(network.controller_events.iter().any(|event| matches!(
        event,
        Event::CommandFinished(command, Outcome::Refused(_)) if *command == refused
    )));
}

/// A listener whose streams changed carries an input that runs on into
/// the entity taking over: its binding and the stream its probe found,
/// so it listens on without probing again.
#[test]
fn a_kept_input_carries_its_binding_and_stream() {
    let mut network = Network::new();
    network.controller.discover(None);
    network.run(3000);
    let now = network.now;
    network.controller.connect(now, (TALKER, 0), (LISTENER, 0));
    network.run(1000);
    let found = network.listener.input_stream(0).expect("settled");

    let mut model = model(LISTENER, "Listener", false);
    model.entity_model_id = EntityModelId(0x8c1f_6436_c0f3_59f0);
    model.inputs.push(stream("Host in 2"));
    let mut listener = Entity::new(model, &[]);
    listener.take_over(network.now, &network.listener);
    listener.carry_streams(&network.listener, &[], &[0]);
    assert_eq!(listener.input_stream(0), Some(found));
    assert_eq!(
        listener.input_binding(0).map(|binding| binding.talker),
        Some(TALKER)
    );
    assert_eq!(listener.input_binding(1), None);
}
