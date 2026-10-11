//! AVB Wireless's ATDECC messages (AVB Wireless profile, 5): the status
//! query a station or an access point reports its wireless interface
//! with, and the configuration a controller sets on it. Both are AECP
//! vendor unique messages under the AVB Lite MA-S OUI `8C-1F-64-36-C`.

use crate::aecp::{AecpHeader, AecpMessageType, VendorUniquePdu};
use crate::avtp::{read_u16, read_u32, read_u64};
use crate::error::{DecodeError, EncodeError};
use crate::id::{ClockIdentity, EntityId};
use crate::macros::{code, flags};
use avb_net::MacAddress;

/// The status query, sub-protocol `0x005`.
pub const WIRELESS_PROTOCOL_ID: [u8; 6] = [0x8c, 0x1f, 0x64, 0x36, 0xc0, 0x05];

code! {
    /// The status query's command_type.
    pub struct WirelessCommandType(u16) {
        const GET_WIRELESS_STATUS = 0x0000;
        const SET_WIRELESS_CONFIG = 0x0001;
    }
}

flags! {
    /// GET_WIRELESS_STATUS flags.
    pub struct WirelessFlags(u8) {
        /// The interface is an access point's wireless port, else a
        /// station's.
        const ACCESS_POINT = 0x01;
        /// A station's time is locked (profile 2.6).
        const LOCKED = 0x02;
        /// A station is holding over (profile 2.6).
        const HOLDOVER = 0x04;
        const SERVO_ERROR_VALID = 0x08;
        const RTT_VALID = 0x10;
        /// An access point admits Class A toward its wireless port.
        const CLASS_A_ALLOWED = 0x20;
    }
}

flags! {
    /// SET_WIRELESS_CONFIG's config_flags (profile 5.2).
    pub struct WirelessConfigFlags(u8) {
        /// An access point admits Class A toward its wireless port.
        const CLASS_A_ALLOWED = 0x01;
    }
}

code! {
    /// How a station gets the grandmaster's time (profile 2.1).
    pub struct TimeMode(u8) {
        const NONE = 0;
        /// IEEE 802.1AS-2020 12 over Fine Timing Measurement.
        const MODE_A_FTM = 1;
        /// IEEE 802.1AS-2020 12 over Timing Measurement.
        const MODE_A_TM = 2;
        /// The grandmaster's time in the access point's beacons.
        const MODE_B = 3;
    }
}

code! {
    pub struct Band(u8) {
        const UNKNOWN = 0;
        const GHZ_2_4 = 1;
        const GHZ_5 = 2;
        const GHZ_6 = 3;
    }
}

code! {
    /// Why asCapable is FALSE on a Mode A station.
    pub struct AsCapableReason(u8) {
        /// asCapable is TRUE, or the interface is not a Mode A station's.
        const NONE = 0;
        /// FTM bursts granted of other than three or two frames.
        const BURST_FRAMES = 1;
        /// Neither FTM nor TM with the other end.
        const NO_MEASUREMENT = 2;
        /// No gPTP-capable Signaling from the other end.
        const NO_SIGNALING = 3;
    }
}

flags! {
    /// What an access point knows of an associated station.
    pub struct StationFlags(u8) {
        const FTM_INITIATOR = 0x01;
        /// The access point knows whether the station is an FTM initiator.
        const FTM_KNOWN = 0x02;
    }
}

/// An RSSI or other signed octet the profile marks unknown with `0x7F`.
const UNKNOWN_RSSI: i8 = 0x7f;

/// The status query part of a vendor unique PDU: the u flag, the command
/// type and the command specific data after them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WirelessMessage<'a> {
    pub unsolicited: bool,
    pub command_type: WirelessCommandType,
    pub data: &'a [u8],
}

impl<'a> WirelessMessage<'a> {
    /// The status query message in a vendor unique PDU, if it is one.
    pub fn from_pdu(pdu: &VendorUniquePdu<'a>) -> Result<Self, DecodeError> {
        if pdu.protocol_id != WIRELESS_PROTOCOL_ID {
            return Err(DecodeError::WrongProtocol);
        }
        let Some(word) = pdu.payload.get(..2) else {
            return Err(DecodeError::Truncated {
                needed: 2,
                available: pdu.payload.len(),
            });
        };
        let word = read_u16(word, 0);
        Ok(Self {
            unsolicited: word & 0x8000 != 0,
            command_type: WirelessCommandType(word & 0x7fff),
            data: &pdu.payload[2..],
        })
    }
}

fn command_header(target: EntityId, controller: EntityId, sequence_id: u16) -> AecpHeader {
    AecpHeader {
        message_type: AecpMessageType::VENDOR_UNIQUE_COMMAND,
        status: 0,
        target_entity_id: target,
        controller_entity_id: controller,
        sequence_id,
    }
}

/// Encodes a GET_WIRELESS_STATUS command for an AVB_INTERFACE.
pub fn encode_get_wireless_status(
    target: EntityId,
    controller: EntityId,
    sequence_id: u16,
    interface: u16,
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    let command = WirelessCommandType::GET_WIRELESS_STATUS.0.to_be_bytes();
    command_header(target, controller, sequence_id).encode(
        &[&WIRELESS_PROTOCOL_ID, &command, &interface.to_be_bytes()],
        out,
    )
}

/// Encodes a SET_WIRELESS_CONFIG command for an AVB_INTERFACE.
pub fn encode_set_wireless_config(
    target: EntityId,
    controller: EntityId,
    sequence_id: u16,
    interface: u16,
    flags: WirelessConfigFlags,
    out: &mut [u8],
) -> Result<usize, EncodeError> {
    let command = WirelessCommandType::SET_WIRELESS_CONFIG.0.to_be_bytes();
    command_header(target, controller, sequence_id).encode(
        &[
            &WIRELESS_PROTOCOL_ID,
            &command,
            &interface.to_be_bytes(),
            &[flags.0, 0, 0, 0],
        ],
        out,
    )
}

/// What a station or an access point reports of a wireless interface in a
/// GET_WIRELESS_STATUS response, without its station list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WirelessStatus {
    pub interface: u16,
    pub flags: WirelessFlags,
    pub time_mode: TimeMode,
    pub band: Band,
    /// The primary channel, 0 when not known.
    pub channel: u8,
    /// MHz, 0 when not known.
    pub channel_width: u16,
    /// The Wi-Fi generation of the negotiated PHY, 4 for 802.11n to 7 for
    /// 802.11be, 3 for 802.11a, b or g, 0 when not known.
    pub phy_generation: u8,
    /// A station's RSSI from its access point in dBm, `0x7F` when not
    /// known and on an access point.
    pub rssi: i8,
    /// A station's transmit PHY rate in Mb/s, 0 when not known.
    pub phy_rate: u16,
    /// Percent of FTM measurements valid over the last 10 s, `0xFF`
    /// without FTM.
    pub ftm_success: u8,
    pub as_capable_reason: AsCapableReason,
    /// Frames per FTM burst granted, 0 without FTM, `0xFF` when not known.
    pub ftm_burst_frames: u8,
    /// An access point's Ready listeners beyond its fan-out limit, `0xFF`
    /// when not known.
    pub listeners_unserved: u8,
    /// IEEE 802.11's Burst Duration code, 0 without FTM, `0xFF` when not
    /// known.
    pub ftm_burst_duration: u8,
    /// Min Delta FTM in units of 100 µs, 0 without FTM, `0xFF` when not
    /// known.
    pub ftm_min_delta: u8,
    /// Nanoseconds (RTT_VALID).
    pub ftm_rtt: i32,
    /// Nanoseconds (SERVO_ERROR_VALID).
    pub servo_error: i32,
    /// Seconds since a station associated or an access point started.
    pub association_age: u32,
    /// Milliseconds since a station last applied a time element, `0xFFFF`
    /// for none.
    pub time_age: u16,
    pub ap_resets: u16,
    pub bssid: MacAddress,
    /// The gPTP port identity of the access point's wireless port.
    pub ap_clock: ClockIdentity,
    pub ap_port: u16,
    pub downlink_readdressed: u32,
    pub downlink_unmapped: u32,
    pub downlink_dropped: u32,
    pub uplink_restored: u32,
    /// An access point's frame budget for its wireless port, in frames a
    /// second (profile 4); 0 on a station.
    pub frame_budget: u32,
    /// The frames a second of the streams an access point sends on its
    /// wireless port, each listener station's copy counted; 0 on a
    /// station.
    pub frames_committed: u32,
    /// The station entries in the response, on an access point.
    pub station_count: u16,
}

impl WirelessStatus {
    /// The response's octets after the command_type, before the stations.
    pub const LEN: usize = 76;

    /// Decodes the data after the command_type of a response.
    pub fn decode(data: &[u8]) -> Result<Self, DecodeError> {
        if data.len() < Self::LEN {
            return Err(DecodeError::Truncated {
                needed: Self::LEN,
                available: data.len(),
            });
        }
        let mut bssid = [0; 6];
        bssid.copy_from_slice(&data[34..40]);
        Ok(Self {
            interface: read_u16(data, 0),
            flags: WirelessFlags(data[2]),
            time_mode: TimeMode(data[3]),
            band: Band(data[4]),
            channel: data[5],
            channel_width: read_u16(data, 6),
            phy_generation: data[8],
            rssi: data[9] as i8,
            phy_rate: read_u16(data, 10),
            ftm_success: data[12],
            as_capable_reason: AsCapableReason(data[13]),
            ftm_burst_frames: data[14],
            listeners_unserved: data[15],
            ftm_burst_duration: data[16],
            ftm_min_delta: data[17],
            ftm_rtt: read_u32(data, 18) as i32,
            servo_error: read_u32(data, 22) as i32,
            association_age: read_u32(data, 26),
            time_age: read_u16(data, 30),
            ap_resets: read_u16(data, 32),
            bssid: MacAddress(bssid),
            ap_clock: ClockIdentity(read_u64(data, 40)),
            ap_port: read_u16(data, 48),
            downlink_readdressed: read_u32(data, 50),
            downlink_unmapped: read_u32(data, 54),
            downlink_dropped: read_u32(data, 58),
            uplink_restored: read_u32(data, 62),
            frame_budget: read_u32(data, 66),
            frames_committed: read_u32(data, 70),
            station_count: read_u16(data, 74),
        })
    }

    /// The data after the command_type, before the stations, as a
    /// response carries it.
    pub fn to_bytes(&self) -> [u8; Self::LEN] {
        let mut out = [0; Self::LEN];
        out[0..2].copy_from_slice(&self.interface.to_be_bytes());
        out[2] = self.flags.0;
        out[3] = self.time_mode.0;
        out[4] = self.band.0;
        out[5] = self.channel;
        out[6..8].copy_from_slice(&self.channel_width.to_be_bytes());
        out[8] = self.phy_generation;
        out[9] = self.rssi as u8;
        out[10..12].copy_from_slice(&self.phy_rate.to_be_bytes());
        out[12] = self.ftm_success;
        out[13] = self.as_capable_reason.0;
        out[14] = self.ftm_burst_frames;
        out[15] = self.listeners_unserved;
        out[16] = self.ftm_burst_duration;
        out[17] = self.ftm_min_delta;
        out[18..22].copy_from_slice(&self.ftm_rtt.to_be_bytes());
        out[22..26].copy_from_slice(&self.servo_error.to_be_bytes());
        out[26..30].copy_from_slice(&self.association_age.to_be_bytes());
        out[30..32].copy_from_slice(&self.time_age.to_be_bytes());
        out[32..34].copy_from_slice(&self.ap_resets.to_be_bytes());
        out[34..40].copy_from_slice(&self.bssid.0);
        out[40..48].copy_from_slice(&self.ap_clock.0.to_be_bytes());
        out[48..50].copy_from_slice(&self.ap_port.to_be_bytes());
        out[50..54].copy_from_slice(&self.downlink_readdressed.to_be_bytes());
        out[54..58].copy_from_slice(&self.downlink_unmapped.to_be_bytes());
        out[58..62].copy_from_slice(&self.downlink_dropped.to_be_bytes());
        out[62..66].copy_from_slice(&self.uplink_restored.to_be_bytes());
        out[66..70].copy_from_slice(&self.frame_budget.to_be_bytes());
        out[70..74].copy_from_slice(&self.frames_committed.to_be_bytes());
        out[74..76].copy_from_slice(&self.station_count.to_be_bytes());
        out
    }

    pub fn access_point(&self) -> bool {
        self.flags.contains(WirelessFlags::ACCESS_POINT)
    }

    /// A station's RSSI, when it knows it.
    pub fn signal(&self) -> Option<i8> {
        (!self.access_point() && self.rssi != UNKNOWN_RSSI).then_some(self.rssi)
    }

    /// The last valid FTM round trip.
    pub fn rtt(&self) -> Option<i32> {
        self.flags
            .contains(WirelessFlags::RTT_VALID)
            .then_some(self.ftm_rtt)
    }

    /// A station's servo error estimate.
    pub fn servo(&self) -> Option<i32> {
        self.flags
            .contains(WirelessFlags::SERVO_ERROR_VALID)
            .then_some(self.servo_error)
    }

    /// An access point's listeners it does not serve, when it knows.
    pub fn unserved(&self) -> Option<u8> {
        (self.access_point() && self.listeners_unserved != 0xff).then_some(self.listeners_unserved)
    }

    /// The station list after the fixed fields of a response's `data`, as
    /// many entries as both `station_count` and the data hold.
    pub fn stations<'a>(&self, data: &'a [u8]) -> impl Iterator<Item = Station> + 'a {
        data.get(Self::LEN..)
            .unwrap_or_default()
            .as_chunks::<8>()
            .0
            .iter()
            .take(usize::from(self.station_count))
            .map(Station::decode)
    }
}

/// An access point's associated station, from its GET_WIRELESS_STATUS
/// response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Station {
    pub mac: MacAddress,
    /// dBm, `0x7F` when not known.
    pub rssi: i8,
    pub flags: StationFlags,
}

impl Station {
    fn decode(entry: &[u8; 8]) -> Self {
        let mut mac = [0; 6];
        mac.copy_from_slice(&entry[..6]);
        Self {
            mac: MacAddress(mac),
            rssi: entry[6] as i8,
            flags: StationFlags(entry[7]),
        }
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        let mut out = [0; 8];
        out[..6].copy_from_slice(&self.mac.0);
        out[6] = self.rssi as u8;
        out[7] = self.flags.0;
        out
    }

    pub fn signal(&self) -> Option<i8> {
        (self.rssi != UNKNOWN_RSSI).then_some(self.rssi)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The profile's offsets count from the command_type; the data here
    /// starts after it.
    fn at(profile_offset: usize) -> usize {
        profile_offset - 2
    }

    fn station() -> WirelessStatus {
        WirelessStatus {
            interface: 0,
            flags: WirelessFlags::LOCKED | WirelessFlags::RTT_VALID,
            time_mode: TimeMode::MODE_A_FTM,
            band: Band::GHZ_5,
            channel: 36,
            channel_width: 80,
            phy_generation: 6,
            rssi: -52,
            phy_rate: 866,
            ftm_success: 98,
            as_capable_reason: AsCapableReason::BURST_FRAMES,
            ftm_burst_frames: 8,
            listeners_unserved: 0,
            ftm_burst_duration: 7,
            ftm_min_delta: 20,
            ftm_rtt: 42,
            servo_error: -380,
            association_age: 3_600,
            time_age: 120,
            ap_resets: 1,
            bssid: MacAddress([0x30, 0xed, 0xa0, 0x11, 0x22, 0x33]),
            ap_clock: ClockIdentity(0x30ed_a0ff_fe11_2233),
            ap_port: 2,
            downlink_readdressed: 0,
            downlink_unmapped: 0,
            downlink_dropped: 0,
            uplink_restored: 0,
            frame_budget: 0,
            frames_committed: 0,
            station_count: 0,
        }
    }

    #[test]
    fn status_round_trips_at_the_profiles_offsets() {
        let status = station();
        let bytes = status.to_bytes();
        assert_eq!(WirelessStatus::decode(&bytes).unwrap(), status);
        assert_eq!(bytes[at(5)], 1, "time_mode");
        assert_eq!(bytes[at(11)] as i8, -52, "rssi");
        assert_eq!(read_u16(&bytes, at(12)), 866, "phy_rate");
        assert_eq!(read_u32(&bytes, at(24)) as i32, -380, "servo_error");
        assert_eq!(bytes[at(36)..at(42)], status.bssid.0, "bssid");
        assert_eq!(read_u64(&bytes, at(42)), status.ap_clock.0);
        assert_eq!(read_u16(&bytes, at(50)), 2, "ap port");
        assert!(WirelessStatus::decode(&bytes[..WirelessStatus::LEN - 1]).is_err());
    }

    #[test]
    fn a_station_says_what_it_knows() {
        let status = station();
        assert_eq!(status.signal(), Some(-52));
        assert_eq!(status.rtt(), Some(42));
        assert_eq!(status.servo(), None);
        assert_eq!(status.unserved(), None);
        let unknown = WirelessStatus {
            rssi: 0x7f,
            ..status
        };
        assert_eq!(unknown.signal(), None);
    }

    #[test]
    fn an_access_point_lists_its_stations() {
        let status = WirelessStatus {
            flags: WirelessFlags::ACCESS_POINT,
            rssi: 0x7f,
            listeners_unserved: 1,
            frame_budget: 6500,
            frames_committed: 4000,
            station_count: 2,
            ..station()
        };
        let bytes = status.to_bytes();
        assert_eq!(read_u32(&bytes, at(68)), 6500, "frame_budget");
        assert_eq!(read_u32(&bytes, at(72)), 4000, "frames_committed");
        assert_eq!(read_u16(&bytes, at(76)), 2, "station_count");
        let first = Station {
            mac: MacAddress([0xfc, 0x01, 0x2c, 0xfd, 0x80, 0x00]),
            rssi: -48,
            flags: StationFlags::FTM_INITIATOR | StationFlags::FTM_KNOWN,
        };
        let second = Station {
            mac: MacAddress([0xfc, 0x01, 0x2c, 0xfd, 0x80, 0x01]),
            rssi: 0x7f,
            flags: StationFlags::empty(),
        };
        let mut data = status.to_bytes().to_vec();
        data.extend_from_slice(&first.to_bytes());
        data.extend_from_slice(&second.to_bytes());
        // A third entry past station_count is left out.
        data.extend_from_slice(&first.to_bytes());
        let decoded = WirelessStatus::decode(&data).unwrap();
        let stations: Vec<Station> = decoded.stations(&data).collect();
        assert_eq!(stations, [first, second]);
        assert_eq!(stations[1].signal(), None);
        assert_eq!(decoded.signal(), None);
        assert_eq!(decoded.unserved(), Some(1));
        // A response cut short keeps the entries it holds.
        assert_eq!(
            decoded.stations(&data[..WirelessStatus::LEN + 12]).count(),
            1
        );
    }

    /// An ESP32 Wi-Fi station's answer, Mode A over FTM, unlocked, its FTM
    /// failing as it sits too close to the access point, as the draft
    /// before the frame budget fields had it; with them, 0 on a station,
    /// it decodes the same.
    #[test]
    fn decodes_an_esp_stations_answer() {
        const EARLIER: [u8; 70] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x01, 0x06, 0x00, 0x14, 0x04, 0xc8, 0x00, 0x00,
            0x00, 0x01, 0xff, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x08, 0xff, 0xff, 0x00, 0x00, 0xd8, 0x85, 0xac, 0xfa, 0x2c, 0x59,
            0x80, 0xf1, 0xb2, 0xff, 0xfe, 0xd2, 0xca, 0xa9, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        // The earlier layout is too short for this one.
        assert!(WirelessStatus::decode(&EARLIER[2..]).is_err());
        let mut answer = EARLIER[..68].to_vec();
        answer.extend_from_slice(&[0; 8]);
        answer.extend_from_slice(&EARLIER[68..]);
        assert_eq!(
            read_u16(&answer, 0),
            WirelessCommandType::GET_WIRELESS_STATUS.0
        );
        let status = WirelessStatus::decode(&answer[2..]).unwrap();
        assert_eq!(status.interface, 0);
        assert!(!status.access_point());
        assert!(!status.flags.contains(WirelessFlags::LOCKED));
        assert_eq!(status.time_mode, TimeMode::MODE_A_FTM);
        assert_eq!(
            (status.band, status.channel, status.channel_width),
            (Band::GHZ_2_4, 6, 20)
        );
        assert_eq!(status.phy_generation, 4);
        assert_eq!(status.signal(), Some(-56));
        assert_eq!(status.phy_rate, 0);
        assert_eq!(status.ftm_success, 0);
        assert_eq!(status.as_capable_reason, AsCapableReason::BURST_FRAMES);
        assert_eq!(
            (
                status.ftm_burst_frames,
                status.ftm_burst_duration,
                status.ftm_min_delta
            ),
            (0xff, 0xff, 0xff)
        );
        assert_eq!((status.rtt(), status.servo()), (None, None));
        assert_eq!(status.association_age, 8);
        assert_eq!(status.time_age, 0xffff);
        assert_eq!(
            status.bssid,
            MacAddress([0xd8, 0x85, 0xac, 0xfa, 0x2c, 0x59])
        );
        assert_eq!(status.ap_clock, ClockIdentity(0x80f1_b2ff_fed2_caa9));
        assert_eq!(status.ap_port, 2);
        assert_eq!(status.station_count, 0);
        assert_eq!((status.frame_budget, status.frames_committed), (0, 0));
        assert_eq!(status.stations(&answer[2..]).count(), 0);
        assert_eq!(status.to_bytes(), answer[2..]);
    }

    #[test]
    fn queries_carry_the_wireless_protocol() {
        let mut out = [0; 64];
        let length = encode_get_wireless_status(
            EntityId(0xfc01_2cfd_fe80_0000),
            EntityId(0x9c6b_00ff_fe30_9a2b),
            9,
            1,
            &mut out,
        )
        .unwrap();
        let pdu = VendorUniquePdu::decode(&out[..length]).unwrap();
        let message = WirelessMessage::from_pdu(&pdu).unwrap();
        assert_eq!(
            message.command_type,
            WirelessCommandType::GET_WIRELESS_STATUS
        );
        assert!(!message.unsolicited);
        assert_eq!(message.data, [0, 1]);

        let length = encode_set_wireless_config(
            EntityId(0x30ed_a0ff_fe11_2233),
            EntityId(0x9c6b_00ff_fe30_9a2b),
            10,
            0,
            WirelessConfigFlags::CLASS_A_ALLOWED,
            &mut out,
        )
        .unwrap();
        let pdu = VendorUniquePdu::decode(&out[..length]).unwrap();
        let message = WirelessMessage::from_pdu(&pdu).unwrap();
        assert_eq!(
            message.command_type,
            WirelessCommandType::SET_WIRELESS_CONFIG
        );
        assert_eq!(message.data, [0, 0, 1, 0, 0, 0]);
        assert!(crate::lite::LiteMessage::from_pdu(&pdu).is_err());
    }
}
