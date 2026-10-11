//! AVB Wireless in the inspector and the entity list: what each wireless
//! interface reports of its link and time (AVB Wireless profile, 5.1),
//! and the alarms for a station whose time is not locked and an access
//! point with listeners it cannot serve.

use atdecc::descriptor::DescriptorType;
use atdecc::model::{EntityModel, Wireless};
use atdecc::wireless::{
    AsCapableReason, Band, StationFlags, TimeMode, WirelessFlags, WirelessStatus,
};
use iced::Element;
use scramble_ui::component;

use crate::app::{Message, Triib};
use crate::fl;
use crate::lite_view::{Alarm, property};

/// How a station's time holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Time {
    Locked,
    Holdover,
    NotLocked,
}

impl Time {
    pub fn of(status: &WirelessStatus) -> Self {
        if status.flags.contains(WirelessFlags::LOCKED) {
            Self::Locked
        } else if status.flags.contains(WirelessFlags::HOLDOVER) {
            Self::Holdover
        } else {
            Self::NotLocked
        }
    }

    pub fn text(self) -> String {
        match self {
            Self::Locked => fl!("wireless-locked"),
            Self::Holdover => fl!("wireless-holdover"),
            Self::NotLocked => fl!("wireless-not-locked"),
        }
    }
}

/// How a station gets the grandmaster's time.
pub fn mode_text(mode: TimeMode) -> String {
    match mode {
        TimeMode::NONE => fl!("wireless-no-time"),
        TimeMode::MODE_A_FTM => fl!("wireless-mode-a-ftm"),
        TimeMode::MODE_A_TM => fl!("wireless-mode-a-tm"),
        TimeMode::MODE_B => fl!("wireless-mode-b"),
        _ => fl!("wireless-other-mode"),
    }
}

pub fn band_text(band: Band) -> Option<String> {
    match band {
        Band::GHZ_2_4 => Some(crate::i18n::decimal("2.4 GHz".to_owned())),
        Band::GHZ_5 => Some("5 GHz".to_owned()),
        Band::GHZ_6 => Some("6 GHz".to_owned()),
        _ => None,
    }
}

/// The band, channel, width and PHY an interface reports, as far as it
/// knows them: "5 GHz, channel 36, 80 MHz, Wi-Fi 6".
pub fn link_text(status: &WirelessStatus) -> String {
    let mut parts: Vec<String> = band_text(status.band).into_iter().collect();
    if status.channel != 0 {
        parts.push(fl!("wireless-channel", channel = status.channel));
    }
    if status.channel_width != 0 {
        parts.push(format!("{} MHz", status.channel_width));
    }
    match status.phy_generation {
        0 => {}
        3 => parts.push("802.11a/b/g".to_owned()),
        generation => parts.push(format!("Wi-Fi {generation}")),
    }
    if parts.is_empty() {
        fl!("wireless-not-known")
    } else {
        crate::i18n::list(parts)
    }
}

/// An RSSI in dBm.
pub fn signal_text(rssi: i8) -> String {
    format!("{rssi} dBm")
}

/// How an entity's wireless interface runs, for the entity list: its role,
/// and a station's time or an access point's station count.
pub fn role(model: &EntityModel) -> String {
    let Some((_, wireless)) = model.wireless_interfaces().next() else {
        return String::new();
    };
    let status = &wireless.status;
    if status.access_point() {
        return fl!("wireless-row-access-point", count = status.station_count);
    }
    match Time::of(status) {
        Time::Locked => fl!("wireless-row-locked"),
        Time::Holdover => fl!("wireless-row-holdover"),
        Time::NotLocked => fl!("wireless-row-not-locked"),
    }
}

/// The entity whose MAC address an access point lists, by name, else the
/// address.
fn station_name(triib: &Triib, mac: atdecc::MacAddress) -> String {
    triib
        .entities
        .values()
        .find(|entity| entity.mac == mac)
        .map_or_else(|| mac.to_string(), |entity| triib.entity_name(entity))
}

/// The Wireless section of the inspector's Entity tab: what each wireless
/// interface reports.
pub fn entity_section<'a>(triib: &Triib, model: &EntityModel) -> Vec<Element<'a, Message>> {
    let mut items = Vec::new();
    for (interface, _) in model.descriptors(DescriptorType::AVB_INTERFACE) {
        let Some(wireless) = model.wireless(interface) else {
            continue;
        };
        if items.is_empty() {
            items.push(component::section(fl!("column-wireless")));
        }
        if wireless.status.access_point() {
            access_point(triib, wireless, &mut items);
        } else {
            station(&wireless.status, &mut items);
        }
    }
    items
}

fn station<'a>(status: &WirelessStatus, items: &mut Vec<Element<'a, Message>>) {
    let time = Time::of(status);
    items.push(property(
        &fl!("wireless-role"),
        fl!("wireless-station"),
        false,
    ));
    items.push(property(
        &fl!("wireless-mode"),
        mode_text(status.time_mode),
        false,
    ));
    items.push(property(
        &fl!("wireless-time"),
        time.text(),
        time != Time::Locked,
    ));
    items.push(property(&fl!("wireless-link"), link_text(status), false));
    if let Some(rssi) = status.signal() {
        items.push(property(&fl!("wireless-signal"), signal_text(rssi), false));
    }
    if status.phy_rate != 0 {
        items.push(property(
            &fl!("wireless-rate"),
            crate::lite_view::rate(u64::from(status.phy_rate) * 1_000_000),
            false,
        ));
    }
    items.push(property(
        &fl!("wireless-access-point"),
        status.bssid.to_string(),
        false,
    ));
    let mut ftm = Vec::new();
    if status.ftm_success != 0xff {
        ftm.push(fl!(
            "wireless-ftm-valid",
            share = fl!("common-percent", value = status.ftm_success.to_string())
        ));
    }
    if let Some(rtt) = status.rtt() {
        ftm.push(fl!("wireless-rtt", rtt = crate::lite_view::duration(rtt)));
    }
    if !matches!(status.ftm_burst_frames, 0 | 0xff) {
        ftm.push(fl!("wireless-bursts", count = status.ftm_burst_frames));
    }
    if !ftm.is_empty() {
        items.push(property("FTM", crate::i18n::list(ftm), false));
    }
    let reason = match status.as_capable_reason {
        AsCapableReason::NONE => None,
        AsCapableReason::BURST_FRAMES => Some(fl!("wireless-reason-bursts")),
        AsCapableReason::NO_MEASUREMENT => Some(fl!("wireless-reason-measurement")),
        AsCapableReason::NO_SIGNALING => Some(fl!("wireless-reason-signaling")),
        _ => Some(fl!("wireless-reason-other")),
    };
    if let Some(reason) = reason {
        items.push(property(
            "asCapable",
            fl!("wireless-not-as-capable", reason = reason),
            false,
        ));
    }
    if let Some(servo) = status.servo() {
        items.push(property(
            &fl!("wireless-servo"),
            crate::lite_view::duration(servo),
            false,
        ));
    }
}

fn access_point<'a>(triib: &Triib, wireless: &Wireless, items: &mut Vec<Element<'a, Message>>) {
    let status = &wireless.status;
    items.push(property(
        &fl!("wireless-role"),
        fl!("wireless-access-point"),
        false,
    ));
    items.push(property(&fl!("wireless-link"), link_text(status), false));
    items.push(property("BSSID", status.bssid.to_string(), false));
    let stations = if wireless.stations.is_empty() {
        fl!("wireless-station-count", count = status.station_count)
    } else {
        crate::i18n::list(wireless.stations.iter().map(|station| {
            let name = station_name(triib, station.mac);
            let mut parts = vec![name];
            if let Some(rssi) = station.signal() {
                parts.push(signal_text(rssi));
            }
            if station.flags.contains(StationFlags::FTM_KNOWN)
                && !station.flags.contains(StationFlags::FTM_INITIATOR)
            {
                parts.push(fl!("wireless-no-ftm"));
            }
            parts.join(" ")
        }))
    };
    items.push(property(&fl!("wireless-stations"), stations, false));
    // How much of what it can send on its wireless port its streams take,
    // each listener station's copy counted.
    if status.frame_budget > 0 {
        items.push(property(
            &fl!("wireless-frame-budget"),
            fl!(
                "wireless-frame-use",
                used = status.frames_committed,
                budget = status.frame_budget
            ),
            status.frames_committed > status.frame_budget,
        ));
    }
    if let Some(unserved) = status.unserved() {
        items.push(property(
            &fl!("wireless-unserved"),
            unserved.to_string(),
            unserved > 0,
        ));
    }
    items.push(property(
        &fl!("wireless-stream-frames"),
        fl!(
            "wireless-frames-of",
            readdressed = status.downlink_readdressed,
            unmapped = status.downlink_unmapped,
            dropped = status.downlink_dropped,
            restored = status.uplink_restored
        ),
        false,
    ));
    items.push(property(
        "Class A",
        if status.flags.contains(WirelessFlags::CLASS_A_ALLOWED) {
            fl!("wireless-class-a-allowed")
        } else {
            fl!("wireless-class-a-not-allowed")
        },
        false,
    ));
}

/// Every entity's AVB Wireless alarms: a station whose time is not locked,
/// and an access point with Ready listeners it does not serve.
pub fn alarms(triib: &Triib) -> Vec<Alarm> {
    let mut alarms = Vec::new();
    for (&entity, model) in &triib.models {
        if !triib.entities.contains_key(&entity) {
            continue;
        }
        for (_, wireless) in model.wireless_interfaces() {
            let status = &wireless.status;
            if status.access_point() {
                if let Some(unserved) = status.unserved().filter(|&count| count > 0) {
                    alarms.push(Alarm {
                        entity,
                        text: fl!("wireless-alarm-unserved", count = unserved),
                    });
                }
                continue;
            }
            let text = match Time::of(status) {
                Time::Locked => continue,
                Time::Holdover => fl!("wireless-alarm-holdover"),
                Time::NotLocked => fl!("wireless-alarm-not-locked"),
            };
            alarms.push(Alarm { entity, text });
        }
    }
    alarms
}

#[cfg(test)]
mod tests {
    use super::*;
    use atdecc::EntityId;
    use avb_net::{Interface, MacAddress};

    const WIFI_ESP: EntityId = EntityId(0xfc01_2cfd_fe80_0000);

    /// The bench, after the Wi-Fi ESP reports itself a station with its
    /// time as `flags` say.
    fn wireless_bench(flags: WirelessFlags) -> Triib {
        let (entities, models) =
            crate::view::tests::bench_then(&crate::view::tests::wireless_frames(flags));
        let interface = Interface {
            name: "enp6s0".to_owned(),
            mac: MacAddress([0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b]),
            up: true,
            speed: None,
            physical: true,
            wireless: false,
            hardware_clock: None,
        };
        Triib::sample(
            crate::settings::Settings::default(),
            interface,
            entities,
            models,
        )
    }

    #[test]
    fn a_locked_station_reads_as_one_and_raises_no_alarm() {
        let triib = wireless_bench(WirelessFlags::LOCKED);
        let model = &triib.models[&WIFI_ESP];
        assert_eq!(role(model), "Station, locked");
        let status = model.wireless(0).unwrap().status;
        assert_eq!(link_text(&status), "5 GHz, channel 36, 80 MHz, Wi-Fi 6");
        assert_eq!(mode_text(status.time_mode), "Mode A, 802.1AS over FTM");
        // Wired entities report nothing of Wi-Fi.
        let wired = &triib.models[&EntityId(0xe8f6_0ae0_9220_0000)];
        assert_eq!(role(wired), "");
        assert!(alarms(&triib).is_empty());
    }

    #[test]
    fn a_station_losing_its_time_raises_an_alarm() {
        let triib = wireless_bench(WirelessFlags::HOLDOVER);
        assert_eq!(
            alarms(&triib),
            [Alarm {
                entity: WIFI_ESP,
                text: "Wi-Fi time holding over, lost from the access point".to_owned(),
            }]
        );
        // The status bar's alarms include it.
        assert_eq!(crate::lite_view::alarms(&triib).len(), 1);
        let triib = wireless_bench(WirelessFlags::empty());
        assert_eq!(
            alarms(&triib)[0].text,
            "Wi-Fi time not locked to the access point"
        );
    }
}
