//! A controller reading the descriptors of three real entities: every
//! AECP frame decodes to what tshark reads, every descriptor decodes, and
//! replaying the entities' side of the capture through a controller makes
//! it send the same commands, byte for byte, and read the same models.

mod common;

use std::collections::HashMap;

use atdecc::aecp::{AecpMessageType, AemPdu, AemStatus, VendorUniquePdu};
use atdecc::aem::{ReadDescriptorResponse, StreamInfo};
use atdecc::controller::{Config, Controller};
use atdecc::descriptor::{
    AudioUnitDescriptor, AvbInterfaceDescriptor, ClockDomainDescriptor, ClockSourceDescriptor,
    ConfigurationDescriptor, DescriptorType, EntityDescriptor, LocaleDescriptor, StreamDescriptor,
    StreamPortDescriptor, StringsDescriptor, names,
};
use atdecc::model::EnumerationState;
use atdecc::mvu::MVU_PROTOCOL_ID;
use atdecc::{AemCommandType, EntityId, Instant, MacAddress, Pdu};
use common::{ethernet_payload, expected_rows, number, pcap_records};

const CAPTURE: &[u8] = include_bytes!("captures/bench-enumeration.pcap");
const EXPECTED: &str = include_str!("captures/bench-enumeration.tsv");

/// The controller that made the capture.
const HOST: MacAddress = MacAddress([0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b]);
const CONTROLLER: EntityId = EntityId(0x9c6b_00ff_fe30_9a2b);

fn source(frame: &[u8]) -> MacAddress {
    MacAddress(frame[6..12].try_into().unwrap())
}

fn expect_text(row: &HashMap<&str, &str>, frame: &str, field: &str, actual: &str) {
    // tshark leaves empty strings out of its export.
    let expected = row.get(field).copied().unwrap_or("");
    assert_eq!(expected, actual, "frame {frame}: {field}");
}

fn expect_number(row: &HashMap<&str, &str>, frame: &str, field: &str, actual: u64) {
    let expected = row
        .get(field)
        .unwrap_or_else(|| panic!("frame {frame}: tshark has no {field}"));
    assert_eq!(number(expected), actual, "frame {frame}: {field}");
}

/// Decodes a descriptor with the view for its type, checking against
/// tshark what both read.
fn check_descriptor(row: &HashMap<&str, &str>, frame: &str, descriptor: &[u8]) {
    let descriptor_type = DescriptorType(u16::from_be_bytes([descriptor[0], descriptor[1]]));
    match descriptor_type {
        DescriptorType::ENTITY => {
            let entity = EntityDescriptor::decode(descriptor).unwrap();
            expect_text(row, frame, "ieee17221.entity_name", entity.entity_name);
            expect_text(
                row,
                frame,
                "ieee17221.firmware_version",
                entity.firmware_version,
            );
            expect_text(row, frame, "ieee17221.group_name", entity.group_name);
            expect_text(row, frame, "ieee17221.serial_number", entity.serial_number);
        }
        DescriptorType::CONFIGURATION => {
            let configuration = ConfigurationDescriptor::decode(descriptor).unwrap();
            assert!(
                configuration.descriptor_counts().count() > 0,
                "frame {frame}"
            );
        }
        DescriptorType::AUDIO_UNIT => {
            let unit = AudioUnitDescriptor::decode(descriptor).unwrap();
            assert!(unit.sampling_rates().count() > 0, "frame {frame}");
        }
        DescriptorType::STREAM_INPUT | DescriptorType::STREAM_OUTPUT => {
            let stream = StreamDescriptor::decode(descriptor).unwrap();
            expect_number(
                row,
                frame,
                "ieee17221.stream_flags",
                u64::from(stream.stream_flags.0),
            );
            assert!(
                stream
                    .formats()
                    .any(|format| format == stream.current_format)
            );
        }
        DescriptorType::AVB_INTERFACE => {
            AvbInterfaceDescriptor::decode(descriptor).unwrap();
        }
        DescriptorType::CLOCK_SOURCE => {
            ClockSourceDescriptor::decode(descriptor).unwrap();
        }
        DescriptorType::CLOCK_DOMAIN => {
            let domain = ClockDomainDescriptor::decode(descriptor).unwrap();
            expect_number(
                row,
                frame,
                "ieee17221.clock_source_index",
                u64::from(domain.clock_source_index),
            );
            assert!(
                domain
                    .clock_sources()
                    .any(|source| source == domain.clock_source_index)
            );
        }
        DescriptorType::LOCALE => {
            let locale = LocaleDescriptor::decode(descriptor).unwrap();
            expect_text(
                row,
                frame,
                "ieee17221.locale_identifier",
                locale.locale_identifier,
            );
            expect_number(
                row,
                frame,
                "ieee17221.base_strings",
                u64::from(locale.base_strings),
            );
            expect_number(
                row,
                frame,
                "ieee17221.number_of_strings",
                u64::from(locale.number_of_strings),
            );
        }
        DescriptorType::STRINGS => {
            StringsDescriptor::decode(descriptor).unwrap();
        }
        DescriptorType::STREAM_PORT_INPUT | DescriptorType::STREAM_PORT_OUTPUT => {
            StreamPortDescriptor::decode(descriptor).unwrap();
        }
        _ => {}
    }
    // tshark exports the name of an AVB_INTERFACE under another field.
    if let Some((object_name, _)) = names(descriptor)
        && descriptor_type != DescriptorType::CONFIGURATION
        && descriptor_type != DescriptorType::AVB_INTERFACE
    {
        expect_text(row, frame, "ieee17221.object_name", object_name);
    }
}

#[test]
fn every_aecp_frame_matches_tshark() {
    let records = pcap_records(CAPTURE);
    let rows = expected_rows(EXPECTED);
    assert_eq!(records.len(), rows.len());
    let (mut commands, mut responses, mut descriptors, mut milan) = (0, 0, 0, 0);
    let (mut stream_infos, mut acmp) = (0, 0);
    for ((_, frame), row) in records.iter().zip(&rows) {
        let number_text = row["frame.number"];
        let (_, avtpdu) = ethernet_payload(frame);
        match atdecc::decode(avtpdu).unwrap_or_else(|error| panic!("frame {number_text}: {error}"))
        {
            Pdu::Aem(aem) => {
                expect_number(
                    row,
                    number_text,
                    "ieee17221.message_type",
                    u64::from(aem.header.message_type.0),
                );
                expect_number(
                    row,
                    number_text,
                    "ieee17221.status",
                    u64::from(aem.header.status),
                );
                expect_number(
                    row,
                    number_text,
                    "ieee17221.sequence_id",
                    u64::from(aem.header.sequence_id),
                );
                expect_number(
                    row,
                    number_text,
                    "ieee17221.command_type",
                    u64::from(aem.command_type.0),
                );
                if aem.header.message_type == AecpMessageType::AEM_COMMAND {
                    commands += 1;
                    continue;
                }
                responses += 1;
                assert_eq!(aem.status(), AemStatus::SUCCESS, "frame {number_text}");
                if aem.command_type == AemCommandType::READ_DESCRIPTOR {
                    let response = ReadDescriptorResponse::decode(aem.payload).unwrap();
                    expect_number(
                        row,
                        number_text,
                        "ieee17221.descriptor_type",
                        u64::from(response.descriptor_type.0),
                    );
                    expect_number(
                        row,
                        number_text,
                        "ieee17221.descriptor_index",
                        u64::from(response.index),
                    );
                    check_descriptor(row, number_text, response.descriptor);
                    descriptors += 1;
                }
                if aem.command_type == AemCommandType::GET_STREAM_INFO {
                    let info = StreamInfo::decode(aem.payload).unwrap();
                    expect_number(
                        row,
                        number_text,
                        "ieee17221.descriptor_type",
                        u64::from(info.descriptor_type.0),
                    );
                    expect_number(
                        row,
                        number_text,
                        "ieee17221.descriptor_index",
                        u64::from(info.index),
                    );
                    assert!(
                        info.flags
                            .contains(atdecc::aem::StreamInfoFlags::STREAM_FORMAT_VALID)
                    );
                    stream_infos += 1;
                }
            }
            Pdu::VendorUnique(vendor_unique) => {
                assert_eq!(vendor_unique.protocol_id, MVU_PROTOCOL_ID);
                milan += 1;
            }
            Pdu::Acmp(acmpdu) => {
                assert!(acmpdu.status.is_success(), "frame {number_text}");
                acmp += 1;
            }
            Pdu::Adp(_) | Pdu::Other { .. } => {}
            other => panic!("frame {number_text}: unexpected {other:?}"),
        }
    }
    // 77 READ_DESCRIPTOR and 14 GET_STREAM_INFO exchanges; four
    // GET_MILAN_INFO commands (the macOS entity does not answer, so it gets
    // a retry) and two responses; six GET_RX_STATE exchanges.
    assert_eq!((commands, responses, descriptors, milan), (91, 91, 77, 6));
    assert_eq!((stream_infos, acmp), (14, 12));
}

/// Replays the entities' frames, with their timing, into a controller
/// configured like the one that made the capture, waking it at its
/// deadlines as the blocking driver does.
#[test]
fn replaying_the_entities_reproduces_the_commands_and_models() {
    let records = pcap_records(CAPTURE);
    let start = records[0].0;
    let sent: Vec<&[u8]> = records
        .iter()
        .filter(|(_, frame)| source(frame) == HOST)
        .map(|(_, frame)| ethernet_payload(frame).1)
        .filter(|avtpdu| matches!(avtpdu[0], 0xfb | 0xfc))
        .collect();
    let first_sequence_id = sent
        .iter()
        .find_map(|frame| {
            AemPdu::decode(frame)
                .map(|aem| aem.header.sequence_id)
                .or_else(|_| VendorUniquePdu::decode(frame).map(|pdu| pdu.header.sequence_id))
                .ok()
        })
        .unwrap();

    let mut config = Config::new(CONTROLLER);
    config.register_unsolicited = false;
    // The live run predates the network and media clock queries.
    config.network_info = false;
    config.media_clock_info = false;
    config.read_counters = false;
    config.read_transit_times = false;
    config.read_configurations = false;
    config.lite_status = false;
    config.wireless_status = false;
    config.first_sequence_id = first_sequence_id;
    let mut controller = Controller::new(config);
    let mut replayed: Vec<Vec<u8>> = Vec::new();
    let mut buffer = [0; 1500];
    let mut collect = |controller: &mut Controller| {
        while let Some(transmit) = controller.poll_transmit(&mut buffer).unwrap() {
            let frame = &buffer[..transmit.length];
            if matches!(frame[0], 0xfb | 0xfc) {
                replayed.push(frame.to_vec());
            }
        }
    };
    controller.discover(None);
    collect(&mut controller);
    for (time, frame) in &records {
        let now = Instant::from_nanos((*time - start).as_nanos() as u64);
        while let Some(deadline) = controller
            .poll_timeout()
            .filter(|deadline| *deadline <= now)
        {
            controller.handle_timeout(deadline);
            collect(&mut controller);
        }
        if source(frame) == HOST {
            continue;
        }
        let (_, avtpdu) = ethernet_payload(frame);
        controller.handle_frame(now, source(frame), avtpdu).unwrap();
        controller.handle_timeout(now);
        collect(&mut controller);
    }

    assert_eq!(
        replayed.len(),
        sent.len(),
        "as many commands as the live run"
    );
    for (index, (replayed, sent)) in replayed.iter().zip(&sent).enumerate() {
        assert_eq!(&replayed[..], &sent[..replayed.len()], "command {index}");
    }

    let expected = [
        (EntityId(0xd111_e597_f544_8000), "Mac mini", 27, false),
        (
            EntityId(0xe8f6_0ae0_9220_0000),
            "AVB Example Entity",
            25,
            true,
        ),
        (
            EntityId(0xfc01_2cfd_fe80_0000),
            "AVB Example Entity",
            25,
            true,
        ),
    ];
    for (entity_id, name, descriptors, is_milan) in expected {
        let model = controller
            .model(entity_id)
            .unwrap_or_else(|| panic!("{entity_id}"));
        assert_eq!(model.state, EnumerationState::Complete, "{entity_id}");
        assert_eq!(model.entity_name(), Some(name));
        assert_eq!(model.descriptor_count(), descriptors, "{entity_id}");
        assert_eq!(model.failed_reads, 0, "{entity_id}");
        assert_eq!(model.milan.is_some(), is_milan, "{entity_id}");
    }
    let macos = controller.model(expected[0].0).unwrap();
    let entity = macos.entity().unwrap();
    assert_eq!(macos.localized(entity.vendor_name), Some("Apple Inc."));
    assert_eq!(
        macos.name_of(DescriptorType::CLOCK_SOURCE, 2),
        Some("Mac System Clock")
    );
}
