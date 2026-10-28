//! AVB Lite and bandwidth in the inspector: how each interface of an
//! entity runs as it reports it (AVB Lite profile, 2.4), what its talker
//! streams take of its link, and the alarms the profile calls for: a PTP
//! offset past 50 µs and egress past 75% of the link.

use atdecc::EntityId;
use atdecc::aem::StreamInfoFlags;
use atdecc::descriptor::DescriptorType;
use atdecc::lite::{FallbackReason, LiteFlags, LiteStatus, PtpProfile};
use atdecc::model::EntityModel;
use iced::widget::container;
use iced::{Element, Fill, Length, Theme};
use scramble_ui::button::{self, Kind, Size};
use scramble_ui::font::{Type, styled};
use scramble_ui::{Scheme, component, shape, style};
use scramble_ui::{column, dir, row};

use crate::app::{Message, Triib};
use crate::fl;
use crate::network::Action;

/// The PTP offset past which the profile calls for an alarm (9).
pub(crate) const OFFSET_ALARM: i32 = 50_000;
/// The share of a link streams may take (4 and 6.5).
const EGRESS_LIMIT: f64 = 0.75;
/// The link speed taken when an entity does not report it.
const ASSUMED_LINK: u64 = 1_000_000_000;

/// What an interface's talker streams take of its link.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Egress {
    /// Bits per second.
    pub used: u64,
    pub link: u64,
    /// The entity reported the link speed; else a gigabit is assumed.
    pub link_reported: bool,
    /// The entity reported what its streams take; else it is worked out
    /// from the formats of its connected stream outputs.
    pub used_reported: bool,
}

impl Egress {
    pub fn share(&self) -> f64 {
        self.used as f64 / self.link as f64
    }

    pub fn over(&self) -> bool {
        self.share() >= EGRESS_LIMIT
    }
}

/// What an interface's talker streams take: as the entity reports it, else
/// from the formats of the stream outputs that are connected; none for an
/// entity without talker streams or a report.
pub fn egress(model: &EntityModel, interface: u16) -> Option<Egress> {
    let status = model.lite_status(interface);
    let link = status
        .map(|status| u64::from(status.link_speed) * 1_000_000)
        .filter(|&link| link > 0);
    let reported = status.and_then(LiteStatus::egress);
    let worked_out: u64 = model
        .streams(false)
        .filter(|stream| {
            model
                .stream_info(DescriptorType::STREAM_OUTPUT, stream.index)
                .is_some_and(|info| info.flags.contains(StreamInfoFlags::CONNECTED))
        })
        .filter_map(|stream| stream.current_format.wire_bandwidth())
        .sum();
    if reported.is_none() && model.streams(false).next().is_none() {
        return None;
    }
    Some(Egress {
        used: reported.map_or(worked_out, |kilobits| u64::from(kilobits) * 1000),
        link: link.unwrap_or(ASSUMED_LINK),
        link_reported: link.is_some(),
        used_reported: reported.is_some(),
    })
}

/// A rate in bits per second, as "16.5 Mb/s" or "1 Gb/s".
pub fn rate(bits: u64) -> String {
    let (value, unit) = match bits {
        bits if bits >= 1_000_000_000 => (bits as f64 / 1e9, "Gb/s"),
        bits if bits >= 1_000_000 => (bits as f64 / 1e6, "Mb/s"),
        bits => (bits as f64 / 1e3, "kb/s"),
    };
    let text = format!("{value:.1}");
    let text = crate::i18n::decimal(text.trim_end_matches(".0").to_owned());
    format!("{text} {unit}")
}

/// A time in nanoseconds in the unit that suits it: "180 ns", "72 µs".
pub(crate) fn duration(nanoseconds: i32) -> String {
    if nanoseconds.unsigned_abs() >= 10_000 {
        format!("{} µs", nanoseconds / 1000)
    } else {
        format!("{nanoseconds} ns")
    }
}

/// Whether an entity runs AVB Lite, as it reports it on an interface or as
/// its CVU SRP declarations show, rather than only being able to.
pub fn runs_lite(model: &EntityModel) -> bool {
    model
        .descriptors(DescriptorType::AVB_INTERFACE)
        .filter_map(|(interface, _)| model.lite_status(interface))
        .any(|status| status.flags.contains(LiteFlags::ACTIVE))
        || model.cvu_talkers().next().is_some()
}

/// How an entity runs, for the entity list: AVB Lite or not, as it reports
/// it or as its CVU SRP declarations show.
pub fn mode(model: &EntityModel) -> String {
    let statuses: Vec<&LiteStatus> = model
        .descriptors(DescriptorType::AVB_INTERFACE)
        .filter_map(|(interface, _)| model.lite_status(interface))
        .collect();
    if let Some(active) = statuses
        .iter()
        .find(|status| status.flags.contains(LiteFlags::ACTIVE))
    {
        return match active.media_vlan_id {
            0 => fl!("lite-active-untagged"),
            vlan => fl!("lite-active-vlan", vlan = vlan),
        };
    }
    if model.cvu_talkers().next().is_some() {
        return fl!("lite-active");
    }
    if statuses
        .iter()
        .any(|status| status.flags.contains(LiteFlags::CAPABLE))
    {
        return fl!("lite-capable");
    }
    match model.lite_supported {
        Some(false) => fl!("milan-no"),
        _ => String::new(),
    }
}

pub(crate) fn fallback(reason: FallbackReason) -> String {
    match reason {
        FallbackReason::NONE => fl!("lite-fallback-none"),
        FallbackReason::ENDPOINT_TLV => fl!("lite-fallback-endpoint"),
        FallbackReason::PDELAY_UNANSWERED => fl!("lite-fallback-unanswered"),
        FallbackReason::MULTIPLE_RESPONDERS => fl!("lite-fallback-responders"),
        FallbackReason::CONFIGURED => fl!("lite-fallback-configured"),
        _ => fl!("lite-fallback-other"),
    }
}

/// A PTP profile's name.
pub(crate) fn profile_name(profile: PtpProfile) -> String {
    match profile {
        PtpProfile::GPTP => "gPTP (802.1AS)".to_owned(),
        PtpProfile::AVB_LITE_PTP => "AVB Lite PTP".to_owned(),
        _ => fl!("lite-other-profile"),
    }
}

/// The AVB Lite section of the inspector's Entity tab: how each interface
/// runs, from entities that report it.
pub fn entity_section<'a>(model: &EntityModel) -> Vec<Element<'a, Message>> {
    let mut items = Vec::new();
    for (interface, _) in model.descriptors(DescriptorType::AVB_INTERFACE) {
        let Some(status) = model.lite_status(interface) else {
            continue;
        };
        if items.is_empty() {
            items.push(component::section(fl!("column-avb-lite")));
        }
        let active = status.flags.contains(LiteFlags::ACTIVE);
        let capable = status.flags.contains(LiteFlags::CAPABLE);
        let mode = match (active, capable) {
            (true, _) => fl!("column-avb-lite"),
            (false, true) => fl!("lite-mode-capable"),
            (false, false) => "AVB".to_owned(),
        };
        items.push(property(&fl!("lite-mode"), mode, false));
        if active {
            items.push(property(
                &fl!("lite-because"),
                fallback(status.fallback_reason),
                false,
            ));
        }
        let profile = profile_name(status.ptp_profile);
        items.push(property(
            "PTP",
            fl!(
                "lite-ptp-domain",
                profile = profile,
                domain = status.ptp_domain
            ),
            false,
        ));
        if let Some(offset) = status.offset() {
            items.push(property(
                &fl!("lite-offset"),
                fl!(
                    "lite-offset-from",
                    offset = duration(offset),
                    grandmaster = status.grandmaster.to_string()
                ),
                active && offset.abs() > OFFSET_ALARM,
            ));
        }
        if active {
            items.push(property(
                &fl!("lite-media-vlan"),
                match status.media_vlan_id {
                    0 => fl!("lite-untagged"),
                    vlan => vlan.to_string(),
                },
                false,
            ));
            if status.unicast_fanout_limit > 0 {
                items.push(property(
                    &fl!("lite-unicast"),
                    fl!("lite-fanout", count = status.unicast_fanout_limit),
                    false,
                ));
            }
        }
        if status.link_speed > 0 {
            items.push(property(
                &fl!("lite-link"),
                rate(u64::from(status.link_speed) * 1_000_000),
                false,
            ));
        }
    }
    items
}

pub(crate) fn property<'a>(label: &str, value: String, alarm: bool) -> Element<'a, Message> {
    let value = styled(value, Type::BodyMedium)
        .width(Fill)
        .align_x(dir::text_start());
    row![
        styled(label.to_owned(), Type::BodyMedium)
            .style(style::on_surface_variant)
            .width(Length::Fixed(132.0))
            .align_x(dir::text_start()),
        if alarm {
            value.style(style::error_text)
        } else {
            value
        },
    ]
    .spacing(8)
    .into()
}

/// The bandwidth section of the Diagnostics tab: each interface's egress
/// against its link, with a bar marked past 75%.
pub fn bandwidth<'a>(model: &EntityModel) -> Vec<Element<'a, Message>> {
    let mut items = Vec::new();
    for (interface, _) in model.descriptors(DescriptorType::AVB_INTERFACE) {
        let Some(egress) = egress(model, interface) else {
            continue;
        };
        if items.is_empty() {
            items.push(component::section(fl!("lite-bandwidth")));
        }
        let share = egress.share();
        let used = rate(egress.used);
        let link = rate(egress.link);
        let percent = fl!(
            "common-percent",
            value = crate::i18n::decimal(format!("{:.1}", share * 100.0))
        );
        let text = if egress.link_reported {
            fl!("lite-egress-of", used = used, link = link, share = percent)
        } else {
            fl!(
                "lite-egress-of-assumed",
                used = used,
                link = link,
                share = percent
            )
        };
        let over = egress.over();
        let line = styled(text, Type::BodySmall);
        let note = if egress.used_reported {
            fl!("lite-egress-reported")
        } else {
            fl!("lite-egress-worked-out")
        };
        items.push(
            column![
                styled(fl!("column-egress"), Type::BodyMedium),
                meter(share.min(1.0) as f32, over),
                if over {
                    line.style(style::error_text)
                } else {
                    line.style(style::on_surface_variant)
                },
                styled(note, Type::BodySmall).style(style::on_surface_variant),
            ]
            .spacing(4)
            .into(),
        );
    }
    items
}

/// A bar filled to `share`, in the error color when `over`.
fn meter<'a>(share: f32, over: bool) -> Element<'a, Message> {
    let filled = container(iced::widget::space())
        .width(portion(share))
        .height(6)
        .style(move |theme: &Theme| {
            let scheme = Scheme::of(theme);
            container::Style {
                background: Some(if over { scheme.error } else { scheme.primary }.into()),
                border: iced::border::rounded(shape::EXTRA_SMALL),
                ..container::Style::default()
            }
        });
    let rest = iced::widget::space().width(portion(1.0 - share));
    container(row![filled, rest])
        .width(Fill)
        .height(6)
        .style(|theme: &Theme| container::Style {
            background: Some(Scheme::of(theme).secondary_container.into()),
            border: iced::border::rounded(shape::EXTRA_SMALL),
            ..container::Style::default()
        })
        .into()
}

/// The width of `share` of a bar: none for nothing, as a row lays out a
/// part of no portion across all of it.
fn portion(share: f32) -> Length {
    match (share * 1000.0).round() as u16 {
        0 => Length::Fixed(0.0),
        thousandths => Length::FillPortion(thousandths),
    }
}

/// An alarm the profile calls for, on an entity.
#[derive(Debug, Clone, PartialEq)]
pub struct Alarm {
    pub entity: EntityId,
    pub text: String,
    /// A button's label and what it does to set it right, where triib
    /// can.
    pub fix: Option<(String, Action)>,
}

/// Every entity's alarms: a PTP offset past 50 µs on an interface running
/// AVB Lite, egress past 75% of a link, and AVB Wireless's (see
/// [`crate::wireless_view::alarms`]).
pub fn alarms(triib: &Triib) -> Vec<Alarm> {
    let mut alarms = Vec::new();
    for (&entity, model) in &triib.models {
        if !triib.entities.contains_key(&entity) {
            continue;
        }
        for (interface, _) in model.descriptors(DescriptorType::AVB_INTERFACE) {
            if let Some(offset) = model
                .lite_status(interface)
                .filter(|status| status.flags.contains(LiteFlags::ACTIVE))
                .and_then(LiteStatus::offset)
                .filter(|offset| offset.abs() > OFFSET_ALARM)
            {
                alarms.push(Alarm {
                    entity,
                    text: fl!("lite-alarm-offset", offset = duration(offset)),
                    fix: None,
                });
            }
            if let Some(egress) = egress(model, interface).filter(Egress::over) {
                alarms.push(Alarm {
                    entity,
                    text: fl!(
                        "lite-alarm-egress",
                        share = fl!(
                            "common-percent",
                            value = format!("{:.0}", egress.share() * 100.0)
                        ),
                        limit = fl!("common-percent", value = "75")
                    ),
                    fix: None,
                });
            }
        }
    }
    alarms.extend(crate::wireless_view::alarms(triib));
    alarms.extend(foreign_mrp_alarm(triib));
    alarms.extend(crate::diagnostics_view::alarms(triib));
    alarms
}

/// Another MRP participant on this computer's port, as its daemon heard
/// it, on the first of its endpoints triib sees.
fn foreign_mrp_alarm(triib: &Triib) -> Option<Alarm> {
    let status = triib
        .endpoints
        .as_ref()
        .filter(|status| status.foreign_mrp)?;
    let entity = status
        .endpoints
        .iter()
        .filter_map(|endpoint| endpoint.entity_id.parse::<EntityId>().ok())
        .find(|entity| triib.entities.contains_key(entity))?;
    Some(Alarm {
        entity,
        text: fl!(
            "host-alarm-foreign-mrp",
            interface = status.interface.clone()
        ),
        fix: None,
    })
}

/// The alarms of one entity, for the top of its Diagnostics tab, each
/// with its button where triib can set it right.
pub fn entity_alarms<'a>(triib: &Triib, entity: EntityId) -> Vec<Element<'a, Message>> {
    alarms(triib)
        .into_iter()
        .filter(|alarm| alarm.entity == entity)
        .map(|alarm| {
            let text = styled(alarm.text, Type::BodyMedium).style(style::error_text);
            match alarm.fix {
                Some((label, action)) => {
                    let pending = triib.changing(|pending| *pending == action).is_some();
                    column![
                        text,
                        button::button(Kind::Tonal, label)
                            .size(Size::ExtraSmall)
                            .on_press_maybe((!pending).then_some(Message::Act(action))),
                    ]
                    .spacing(8)
                    .into()
                }
                None => text.into(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use avb_net::{Interface, MacAddress};

    const MAC_MINI: EntityId = EntityId(0xd111_e597_f544_8000);
    const WIRED_ESP: EntityId = EntityId(0xe8f6_0ae0_9220_0000);
    const WIFI_ESP: EntityId = EntityId(0xfc01_2cfd_fe80_0000);

    /// An empty bar has no filled part, and a full one no rest, rather
    /// than a part of no portion taking the whole bar.
    #[test]
    fn bars_empty_and_full_are_drawn_so() {
        assert_eq!(portion(0.0), Length::Fixed(0.0));
        assert_eq!(portion(0.0004), Length::Fixed(0.0));
        assert_eq!(portion(0.25), Length::FillPortion(250));
        assert_eq!(portion(1.0), Length::FillPortion(1000));
    }

    /// The bench, after the wired ESP reports AVB Lite with `offset` and
    /// `egress` and the Mac mini declares a stream over CVU SRP.
    fn lite_bench(offset: i32, egress: u32) -> Triib {
        bench_of(&crate::view::tests::lite_frames(offset, egress))
    }

    /// The bench after `frames`.
    fn bench_of(frames: &[Vec<u8>]) -> Triib {
        let (entities, models) = crate::view::tests::bench_then(frames);
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
    fn modes_follow_reports_and_declarations() {
        let triib = lite_bench(-180, 6_336);
        assert_eq!(mode(&triib.models[&WIRED_ESP]), "Active, VLAN 2");
        // Declaring over CVU SRP shows it runs AVB Lite without a report.
        assert_eq!(mode(&triib.models[&MAC_MINI]), "Active");
        assert_eq!(triib.models[&MAC_MINI].cvu_talkers().count(), 1);
        // Nothing heard from it, and never asked.
        assert_eq!(mode(&triib.models[&WIFI_ESP]), "");
    }

    #[test]
    fn reported_egress_wins_over_formats() {
        let triib = lite_bench(-180, 6_336);
        let egress = egress(&triib.models[&WIRED_ESP], 0).unwrap();
        assert_eq!(egress.used, 6_336_000);
        assert_eq!(egress.link, 1_000_000_000);
        assert!(egress.used_reported && egress.link_reported);
        assert!(!egress.over());
        // Quiet entities raise no alarms.
        assert!(alarms(&triib).is_empty());
    }

    #[test]
    fn offsets_and_egress_past_the_limits_raise_alarms() {
        let triib = lite_bench(-72_400, 800_000);
        let alarms = alarms(&triib);
        assert_eq!(
            alarms,
            [
                Alarm {
                    entity: WIRED_ESP,
                    text: "PTP offset -72 µs, past the 50 µs AVB Lite allows".to_owned(),
                    fix: None,
                },
                Alarm {
                    entity: WIRED_ESP,
                    text: "Egress at 80% of the link, past the 75% streams may take".to_owned(),
                    fix: None,
                },
            ]
        );
        // An offset past the limit on an interface that does not run AVB
        // Lite is not the profile's to judge.
        let capable = bench_of(&crate::view::tests::lite_frames_in(
            false, -2_643_000, 6_336,
        ));
        assert!(super::alarms(&capable).is_empty());
        // An entity gone from the network raises none.
        let mut gone = lite_bench(-72_400, 800_000);
        gone.entities.remove(&WIRED_ESP);
        assert!(super::alarms(&gone).is_empty());
    }

    /// Another MRP participant on this computer's port, which its daemon
    /// heard, raises an alarm on the first of its endpoints triib sees.
    #[test]
    fn another_participant_on_the_port_raises_an_alarm() {
        use triib_endpointd::status::{DaemonStatus, EndpointStatus};
        let mut triib = lite_bench(-180, 6_336);
        let endpoint = |entity: EntityId| EndpointStatus {
            entity_id: entity.to_string(),
            name: "Host device 1".into(),
            state: "listening".into(),
            streams: Vec::new(),
        };
        let mut status = DaemonStatus {
            pid: 1,
            started: 0,
            interface: "enp2s0".into(),
            gptp: String::new(),
            foreign_mrp: false,
            config: None,
            clock: None,
            setup: None,
            sharers: Vec::new(),
            endpoints: vec![endpoint(EntityId(0x1234)), endpoint(WIRED_ESP)],
        };
        triib.endpoints = Some(status.clone());
        assert!(alarms(&triib).is_empty());
        status.foreign_mrp = true;
        triib.endpoints = Some(status);
        assert_eq!(
            alarms(&triib),
            [Alarm {
                entity: WIRED_ESP,
                text: "Another program on this computer declares MSRP or MVRP on enp2s0".to_owned(),
                fix: None,
            }]
        );
    }

    #[test]
    fn rates_and_times_read_naturally() {
        assert_eq!(rate(16_512_000), "16.5 Mb/s");
        assert_eq!(rate(1_000_000_000), "1 Gb/s");
        assert_eq!(rate(224_000), "224 kb/s");
        assert_eq!(duration(180), "180 ns");
        assert_eq!(duration(-72_400), "-72 µs");
    }

    #[test]
    fn egress_past_three_quarters_is_over() {
        let egress = Egress {
            used: 760_000_000,
            link: 1_000_000_000,
            link_reported: true,
            used_reported: true,
        };
        assert!(egress.over());
        assert!(
            !Egress {
                used: 740_000_000,
                ..egress
            }
            .over()
        );
    }
}
