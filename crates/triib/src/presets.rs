//! Presets: how the network was set up when saved, as each entity's clock
//! sources, sampling rates, stream formats and controls and what each of
//! its stream inputs is bound to, and this computer's own talkers and
//! listeners; recalled by changing what differs now. Each is a TOML file
//! in triib's data folder, so it can be copied to another computer.

use std::path::PathBuf;

use atdecc::control::{ControlDescriptor, ControlType, Shape, encode_values};
use atdecc::descriptor::{DescriptorType, SamplingRate, StreamFormat};
use atdecc::model::{EntityModel, EnumerationState};
use atdecc::{EntityId, MacAddress};
use iced::widget::{container, mouse_area, opaque, text_input};
use iced::{Center, Element, Fill};
use scramble_ui::button::{self, Kind};
use scramble_ui::font::{Type, aligned, styled};
use scramble_ui::icon::Icon;
use scramble_ui::{column, row};
use scramble_ui::{component, style};
use serde::{Deserialize, Serialize};
use triib_endpointd::config::EndpointConfig;

use crate::app::{Message, Triib};
use crate::fl;
use crate::network::{Action, ControlValues};

const WIDTH: f32 = 560.0;
const HEIGHT: f32 = 640.0;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    #[serde(default)]
    pub entities: Vec<EntityPreset>,
    /// This computer's own entities, when it had any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<HostPreset>,
}

/// This computer's own entities as `entities.toml` had them, and the
/// address of the interface their entity IDs came from.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HostPreset {
    pub mac: String,
    #[serde(default, rename = "endpoint")]
    pub endpoints: Vec<EndpointConfig>,
}

impl HostPreset {
    /// Its endpoints' entity IDs on the interface with `mac`.
    pub fn entity_ids(&self, mac: MacAddress) -> Vec<EntityId> {
        self.endpoints
            .iter()
            .enumerate()
            .map(|(place, endpoint)| {
                triib_endpointd::runtime::entity_id(mac, endpoint.instance_at(place))
            })
            .collect()
    }
}

/// What one entity had. IDs and formats are kept as hex, as TOML's
/// integers stop short of them.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct EntityPreset {
    pub entity: String,
    /// Its name when saved, to say which is missing.
    pub name: String,
    #[serde(default)]
    pub clock_sources: Vec<ClockSourcePreset>,
    #[serde(default)]
    pub sampling_rates: Vec<RatePreset>,
    #[serde(default)]
    pub stream_formats: Vec<FormatPreset>,
    #[serde(default)]
    pub controls: Vec<ControlPreset>,
    #[serde(default)]
    pub bindings: Vec<BindingPreset>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClockSourcePreset {
    pub domain: u16,
    pub source: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RatePreset {
    pub unit: u16,
    /// The sampling rate's 32 bits, its pull included.
    pub rate: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormatPreset {
    pub input: bool,
    pub index: u16,
    pub format: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlPreset {
    pub index: u16,
    /// Its current values as SET_CONTROL carries them.
    pub values: String,
}

/// A stream input and the talker's stream output it is bound to, or
/// nothing for one not bound.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BindingPreset {
    pub input: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub talker: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<u16>,
}

impl Preset {
    /// How many stream inputs it binds.
    pub fn connections(&self) -> usize {
        self.entities
            .iter()
            .flat_map(|entity| &entity.bindings)
            .filter(|binding| binding.talker.is_some())
            .count()
    }
}

fn hex(value: u64) -> String {
    format!("{value:#018x}")
}

fn number(text: &str) -> Option<u64> {
    u64::from_str_radix(text.trim_start_matches("0x"), 16).ok()
}

fn octets_hex(octets: &[u8]) -> String {
    octets.iter().map(|octet| format!("{octet:02x}")).collect()
}

fn octets(text: &str) -> Option<Vec<u8>> {
    (0..text.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(text.get(at..at + 2)?, 16).ok())
        .collect()
}

/// A control's current values as SET_CONTROL carries them, for one a
/// preset keeps: a level or a selector that can be set, not identify.
fn kept_values(control: &ControlDescriptor<'_>) -> Option<Vec<u8>> {
    if control.read_only || control.control_type == ControlType::IDENTIFY {
        return None;
    }
    if !matches!(control.value_type.shape(), Shape::Linear | Shape::Selector) {
        return None;
    }
    let mut buffer = [0; atdecc::aem::MAX_CONTROL_VALUES];
    let length = encode_values(control.value_type.scalar()?, control.current(), &mut buffer)?;
    ControlValues::new(&buffer[..length]).map(|values| values.as_slice().to_vec())
}

/// The network as it is set up now, named `name`: every entity read in
/// full.
pub fn capture(triib: &Triib, name: &str) -> Preset {
    let mut entities = Vec::new();
    for (entity_id, model) in &triib.models {
        let Some(found) = triib.entities.get(entity_id) else {
            continue;
        };
        if model.state != EnumerationState::Complete {
            continue;
        }
        let mut entity = EntityPreset {
            entity: hex(entity_id.0),
            name: triib.entity_name(found),
            ..EntityPreset::default()
        };
        for domain in model.clock_domains() {
            entity.clock_sources.push(ClockSourcePreset {
                domain: domain.index,
                source: domain.clock_source_index,
            });
        }
        for unit in model.audio_units() {
            entity.sampling_rates.push(RatePreset {
                unit: unit.index,
                rate: unit.current_sampling_rate.0,
            });
        }
        for input in [true, false] {
            for stream in model.streams(input) {
                entity.stream_formats.push(FormatPreset {
                    input,
                    index: stream.index,
                    format: hex(stream.current_format.0),
                });
            }
        }
        for control in model.controls() {
            if let Some(values) = kept_values(&control) {
                entity.controls.push(ControlPreset {
                    index: control.index,
                    values: octets_hex(&values),
                });
            }
        }
        for stream in model.streams(true) {
            let Some(binding) = model.binding(stream.index) else {
                continue;
            };
            let talker = binding.talker_stream();
            entity.bindings.push(BindingPreset {
                input: stream.index,
                talker: talker.map(|(talker, _)| hex(talker.0)),
                output: talker.map(|(_, output)| output),
            });
        }
        entities.push(entity);
    }
    Preset {
        name: name.to_owned(),
        entities,
        host: None,
    }
}

/// `preset` with the entity IDs of this computer's own entities, in its
/// entities and their bindings, made this computer's on the interface
/// with `mac`, for one saved on another computer or interface.
pub fn moved_to(preset: &Preset, mac: MacAddress) -> Preset {
    let mut moved = preset.clone();
    let Some(host) = &mut moved.host else {
        return moved;
    };
    let Ok(from) = host.mac.parse::<MacAddress>() else {
        return moved;
    };
    if from == mac {
        return moved;
    }
    let renamed: Vec<(u64, String)> = host
        .entity_ids(from)
        .into_iter()
        .zip(host.entity_ids(mac))
        .map(|(old, new)| (old.0, hex(new.0)))
        .collect();
    host.mac = mac.to_string();
    let rename = |id: &mut String| {
        if let Some((_, new)) = renamed.iter().find(|(old, _)| number(id) == Some(*old)) {
            id.clone_from(new);
        }
    };
    for entity in &mut moved.entities {
        rename(&mut entity.entity);
        for binding in &mut entity.bindings {
            if let Some(talker) = &mut binding.talker {
                rename(talker);
            }
        }
    }
    if let Some(host) = &mut moved.host {
        for stream in host
            .endpoints
            .iter_mut()
            .flat_map(|endpoint| &mut endpoint.streams)
        {
            if let Some((talker, output)) = stream.bound_to() {
                let mut id = hex(talker.0);
                rename(&mut id);
                if let Some(talker) = number(&id) {
                    stream.bound = Some(triib_endpointd::config::binding_text(
                        EntityId(talker),
                        output,
                    ));
                }
            }
        }
    }
    moved
}

/// Whether the devices `preset` runs on this computer are
/// back after recalling it: the daemon started on them after `since`
/// (milliseconds since the Unix epoch), and each is here and read.
pub fn host_ready(triib: &Triib, preset: &Preset, since: u64) -> bool {
    let (Some(host), Some(interface), Some(status)) =
        (&preset.host, triib.interface(), &triib.endpoints)
    else {
        return false;
    };
    status.started >= since
        && host.entity_ids(interface.mac).iter().all(|entity_id| {
            triib.entities.contains_key(entity_id)
                && triib
                    .models
                    .get(entity_id)
                    .is_some_and(|model| model.state == EnumerationState::Complete)
        })
}

/// What recalling a preset changes, in the order to send it, and the
/// entities it names that are not here or not read.
#[derive(Debug, Default, PartialEq)]
pub struct Recall {
    pub actions: Vec<Action>,
    pub missing: Vec<String>,
}

/// The changes that bring the network to `preset`: formats, rates and
/// clock sources first, then controls, then bindings, unbinding before
/// binding.
pub fn recall(triib: &Triib, preset: &Preset) -> Recall {
    let mut recall = Recall::default();
    let (mut settings, mut controls, mut unbinds, mut binds) =
        (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for wanted in &preset.entities {
        let model = number(&wanted.entity).map(EntityId).and_then(|entity| {
            let model = triib.models.get(&entity)?;
            (triib.entities.contains_key(&entity) && model.state == EnumerationState::Complete)
                .then_some((entity, model))
        });
        let Some((entity, model)) = model else {
            recall.missing.push(wanted.name.clone());
            continue;
        };
        settings.extend(setting_changes(entity, model, wanted));
        for control in &wanted.controls {
            let Some(current) = model.control(control.index) else {
                continue;
            };
            let (Some(now), Some(values)) = (kept_values(&current), octets(&control.values)) else {
                continue;
            };
            if now != values
                && let Some(values) = ControlValues::new(&values)
            {
                controls.push(Action::SetControl {
                    entity,
                    index: control.index,
                    values,
                });
            }
        }
        for binding in &wanted.bindings {
            if model
                .descriptor(DescriptorType::STREAM_INPUT, binding.input)
                .is_none()
            {
                continue;
            }
            let now = model
                .binding(binding.input)
                .and_then(|bound| bound.talker_stream());
            let talker = binding
                .talker
                .as_deref()
                .and_then(number)
                .zip(binding.output)
                .map(|(talker, output)| (EntityId(talker), output));
            if now == talker {
                continue;
            }
            let listener = (entity, binding.input);
            match talker {
                Some(talker) => binds.push(Action::Connect { talker, listener }),
                None => unbinds.push(Action::Disconnect { listener }),
            }
        }
    }
    recall.actions = [settings, controls, unbinds, binds].concat();
    recall
}

/// The clock sources, sampling rates and stream formats that differ.
fn setting_changes(entity: EntityId, model: &EntityModel, wanted: &EntityPreset) -> Vec<Action> {
    let mut actions = Vec::new();
    for clock in &wanted.clock_sources {
        if model
            .clock_domains()
            .any(|domain| domain.index == clock.domain && domain.clock_source_index != clock.source)
        {
            actions.push(Action::SetClockSource {
                entity,
                domain: clock.domain,
                source: clock.source,
            });
        }
    }
    for rate in &wanted.sampling_rates {
        if model
            .audio_units()
            .any(|unit| unit.index == rate.unit && unit.current_sampling_rate.0 != rate.rate)
        {
            actions.push(Action::SetSamplingRate {
                entity,
                unit: rate.unit,
                rate: SamplingRate(rate.rate),
            });
        }
    }
    for format in &wanted.stream_formats {
        let Some(wanted_format) = number(&format.format).map(StreamFormat) else {
            continue;
        };
        if model
            .streams(format.input)
            .any(|stream| stream.index == format.index && stream.current_format != wanted_format)
        {
            actions.push(Action::SetStreamFormat {
                entity,
                descriptor_type: if format.input {
                    DescriptorType::STREAM_INPUT
                } else {
                    DescriptorType::STREAM_OUTPUT
                },
                index: format.index,
                format: wanted_format,
            });
        }
    }
    actions
}

/// The folder the settings name for presets, once they are read.
static FOLDER: std::sync::Mutex<Option<PathBuf>> = std::sync::Mutex::new(None);

/// Keeps presets in `folder` from now on.
pub fn set_folder(folder: PathBuf) {
    *FOLDER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(folder);
}

/// Where presets are kept: the folder the settings name, else `presets`
/// in the data folder.
fn folder() -> Option<PathBuf> {
    FOLDER
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
        .or_else(|| Some(crate::settings::default_presets_folder()))
        .filter(|folder| !folder.as_os_str().is_empty())
}

/// A preset's file name from its name: letters and digits, the rest dashes.
fn file_name(name: &str) -> String {
    let mut file = String::new();
    for character in name.trim().chars() {
        if character.is_alphanumeric() {
            file.extend(character.to_lowercase());
        } else if !file.ends_with('-') {
            file.push('-');
        }
    }
    let file = file.trim_matches('-');
    if file.is_empty() {
        "preset.toml".to_owned()
    } else {
        format!("{file}.toml")
    }
}

/// The presets saved, by name; none in tests, which never touch the
/// user's.
pub fn load_all() -> Vec<Preset> {
    if cfg!(test) {
        return Vec::new();
    }
    let Some(entries) = folder().and_then(|folder| std::fs::read_dir(folder).ok()) else {
        return Vec::new();
    };
    let mut presets: Vec<Preset> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "toml")
        })
        .filter_map(|path| triib_store::load(&path).ok())
        .collect();
    presets.sort_by_key(|preset| preset.name.to_lowercase());
    presets
}

/// Saves a preset, replacing one of the same name.
pub fn save(preset: &Preset) -> Result<(), String> {
    if cfg!(test) {
        return Ok(());
    }
    let path = folder()
        .ok_or_else(|| fl!("presets-no-place"))?
        .join(file_name(&preset.name));
    triib_store::save(&path, preset).map_err(|error| {
        fl!(
            "settings-unsaved",
            path = path.display().to_string(),
            error = error.to_string()
        )
    })
}

pub fn delete(name: &str) -> Result<(), String> {
    if cfg!(test) {
        return Ok(());
    }
    let Some(path) = folder().map(|folder| folder.join(file_name(name))) else {
        return Ok(());
    };
    match std::fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(fl!(
            "presets-undeletable",
            path = path.display().to_string(),
            error = error.to_string()
        )),
    }
}

/// The presets dialog, dimming the window under it: a name to save the
/// network as, and each preset saved, to recall or delete.
pub fn dialog(triib: &Triib) -> Element<'_, Message> {
    let name = triib.preset_name.trim();
    let save = (!name.is_empty()).then_some(Message::PresetSaved);
    let header = row![
        styled(fl!("toolbar-presets"), Type::HeadlineSmall)
            .width(Fill)
            .align_x(scramble_ui::dir::text_start()),
        component::tip(
            button::icon_button(Icon::Close).on_press(Message::PresetsClosed),
            fl!("common-close"),
        ),
    ]
    .align_y(Center);
    let mut note = column![aligned(
        styled(fl!("presets-note"), Type::BodyMedium).style(style::on_surface_variant)
    )]
    .spacing(4);
    if triib.hosts_endpoints() {
        note = note.push(aligned(
            styled(fl!("presets-host-note"), Type::BodyMedium).style(style::on_surface_variant),
        ));
    }
    let mut content = column![
        note,
        row![
            text_input(&fl!("inspector-name"), &triib.preset_name)
                .align_x(scramble_ui::dir::input_align(&triib.preset_name))
                .placeholder_align(scramble_ui::dir::horizontal_start())
                .on_input(Message::PresetNameChanged)
                .on_submit_maybe(save.clone())
                .padding([8, 12])
                .style(style::outlined_field)
                .width(Fill),
            button::with_icon(Kind::Filled, Icon::BookmarkAdd, fl!("common-save"))
                .on_press_maybe(save),
        ]
        .spacing(8)
        .align_y(Center),
    ]
    .spacing(16);
    if let Some(report) = &triib.preset_report {
        content = content.push(aligned(styled(report.as_str(), Type::BodyMedium)));
    }
    if triib.presets.is_empty() {
        content = content
            .push(styled(fl!("presets-none"), Type::BodyMedium).style(style::on_surface_variant));
    }
    for preset in &triib.presets {
        let entities = preset.entities.len();
        let connections = preset.connections();
        let mut counts = vec![
            fl!("status-entities", count = entities),
            fl!("presets-connections", count = connections),
        ];
        if let Some(host) = preset
            .host
            .as_ref()
            .filter(|host| !host.endpoints.is_empty())
        {
            counts.push(fl!("presets-host-endpoints", count = host.endpoints.len()));
        }
        content = content.push(
            row![
                column![
                    styled(preset.name.clone(), Type::BodyLarge),
                    styled(crate::i18n::list(counts), Type::BodySmall,)
                        .style(style::on_surface_variant),
                ]
                .spacing(2)
                .width(Fill),
                button::button(Kind::Tonal, fl!("presets-recall"))
                    .on_press(Message::PresetRecalled(preset.name.clone())),
                component::tip(
                    button::icon_button(Icon::Delete)
                        .on_press(Message::PresetDeleted(preset.name.clone())),
                    fl!("presets-delete"),
                ),
            ]
            .spacing(8)
            .align_y(Center),
        );
    }
    let card = container(column![
        container(header).padding(iced::Padding {
            top: 24.0,
            right: 24.0,
            bottom: 0.0,
            left: 24.0,
        }),
        container(component::scroll(container(content).padding([16, 24]))).height(Fill),
    ])
    .max_width(WIDTH)
    .max_height(HEIGHT)
    .width(Fill)
    .height(Fill)
    .style(style::dialog);
    mouse_area(
        // The card appears at once, even where animations are on.
        container(opaque(card))
            .padding(24)
            .center(Fill)
            .style(style::scrim),
    )
    .on_press(Message::PresetsClosed)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAC_MINI: EntityId = EntityId(0xd111_e597_f544_8000);
    const WIRED_ESP: EntityId = EntityId(0xe8f6_0ae0_9220_0000);

    fn sample() -> Triib {
        let (entities, models) = crate::view::tests::bench();
        let interface = avb_net::Interface {
            name: "enp6s0".to_owned(),
            mac: avb_net::MacAddress([0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b]),
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
    fn a_preset_of_the_network_as_it_is_changes_nothing() {
        let triib = sample();
        let preset = capture(&triib, "Show");
        assert_eq!(preset.entities.len(), 3);
        let wired = preset
            .entities
            .iter()
            .find(|entity| entity.entity == hex(WIRED_ESP.0))
            .unwrap();
        assert!(!wired.stream_formats.is_empty());
        assert!(!wired.clock_sources.is_empty());
        // Volume and gain, not identify.
        assert_eq!(wired.controls.len(), 2);
        assert_eq!(recall(&triib, &preset), Recall::default());
        // It survives being written and read back.
        let text = toml_round_trip(&preset);
        assert_eq!(text, preset);
    }

    fn toml_round_trip(preset: &Preset) -> Preset {
        // A file of its own: tests run side by side in one process.
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let file = std::env::temp_dir().join(format!(
            "triib-preset-{}-{}.toml",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        triib_store::save(&file, preset).unwrap();
        let loaded = triib_store::load(&file).unwrap();
        let _ = std::fs::remove_file(&file);
        loaded
    }

    #[test]
    fn recalling_changes_what_differs_in_order() {
        let triib = sample();
        let mut preset = capture(&triib, "Show");
        let wired = preset
            .entities
            .iter_mut()
            .find(|entity| entity.entity == hex(WIRED_ESP.0))
            .unwrap();
        // Bind the first input to the Mac mini, set a control and pick the
        // other clock source.
        wired.bindings[0].talker = Some(hex(MAC_MINI.0));
        wired.bindings[0].output = Some(0);
        wired.controls[0].values = "ff9c".to_owned();
        let source = wired.clock_sources[0].source;
        wired.clock_sources[0].source = if source == 0 { 1 } else { 0 };
        // An entity no longer here.
        preset.entities.push(EntityPreset {
            entity: hex(0x0011_2233_4455_6677),
            name: "Stage box".to_owned(),
            ..EntityPreset::default()
        });
        let recall = recall(&triib, &preset);
        assert_eq!(recall.missing, ["Stage box"]);
        assert!(
            matches!(
                recall.actions[..],
                [
                    Action::SetClockSource {
                        entity: WIRED_ESP,
                        ..
                    },
                    Action::SetControl {
                        entity: WIRED_ESP,
                        ..
                    },
                    Action::Connect {
                        talker: (MAC_MINI, 0),
                        listener: (WIRED_ESP, 0)
                    },
                ]
            ),
            "{:?}",
            recall.actions
        );
    }

    #[test]
    fn presets_are_saved_recalled_and_deleted() {
        let mut triib = sample();
        let _ = triib.update(Message::PresetNameChanged("Show".to_owned()));
        let _ = triib.update(Message::PresetSaved);
        assert_eq!(triib.presets.len(), 1);
        assert_eq!(
            triib.preset_report.as_deref(),
            Some("Saved \"Show\" with 3 entities.")
        );
        assert!(triib.preset_name.is_empty());
        let _ = triib.update(Message::PresetRecalled("Show".to_owned()));
        assert_eq!(
            triib.preset_report.as_deref(),
            Some("Nothing differs from \"Show\".")
        );
        let _ = triib.update(Message::PresetDeleted("Show".to_owned()));
        assert!(triib.presets.is_empty());
    }

    /// A device that talks and one that listens to it.
    fn host_endpoints() -> Vec<EndpointConfig> {
        use triib_endpointd::config::{Direction, StreamConfig};
        vec![
            EndpointConfig {
                streams: vec![StreamConfig {
                    audio: Some("default".to_owned()),
                    ..StreamConfig::new(Direction::Out)
                }],
                ..EndpointConfig::new("Host device 1", Some(0))
            },
            EndpointConfig {
                streams: vec![StreamConfig {
                    audio: Some("Speakers".to_owned()),
                    first_channel: 2,
                    format: Some("0x00a0020840000800".to_owned()),
                    // Bound to the device above, on the computer it was
                    // saved on.
                    bound: Some("0xf0a731ff00f40f14:0".to_owned()),
                    ..StreamConfig::new(Direction::In)
                }],
                ..EndpointConfig::new("Host device 2", Some(3))
            },
        ]
    }

    #[test]
    fn a_preset_moves_this_computers_endpoints_to_another() {
        let saved_on = MacAddress([0xf0, 0xa7, 0x31, 0xf4, 0x0f, 0x14]);
        let here = MacAddress([0x9c, 0x6b, 0x00, 0x30, 0x9a, 0x2b]);
        let host = HostPreset {
            mac: saved_on.to_string(),
            endpoints: host_endpoints(),
        };
        let [talker, listener] = host.entity_ids(saved_on)[..] else {
            panic!("two endpoints");
        };
        let bound_to_talker = || BindingPreset {
            input: 0,
            talker: Some(hex(talker.0)),
            output: Some(0),
        };
        let preset = Preset {
            name: "Show".to_owned(),
            entities: vec![
                EntityPreset {
                    entity: hex(listener.0),
                    name: "Host device 2".to_owned(),
                    bindings: vec![bound_to_talker()],
                    ..EntityPreset::default()
                },
                EntityPreset {
                    entity: hex(WIRED_ESP.0),
                    name: "ESP".to_owned(),
                    bindings: vec![bound_to_talker()],
                    ..EntityPreset::default()
                },
            ],
            host: Some(host),
        };
        assert_eq!(toml_round_trip(&preset), preset);
        let moved = moved_to(&preset, here);
        let talker_here = EntityId(0x9c6b_00ff_0030_9a2b);
        let listener_here = EntityId(0x9c6b_00ff_0330_9a2b);
        assert_eq!(
            moved.host.as_ref().unwrap().entity_ids(here),
            [talker_here, listener_here]
        );
        assert_eq!(moved.host.as_ref().unwrap().mac, here.to_string());
        assert_eq!(moved.entities[0].entity, hex(listener_here.0));
        assert_eq!(
            moved.entities[0].bindings[0].talker,
            Some(hex(talker_here.0))
        );
        // Another entity keeps its ID, bound to the talker where it is now.
        assert_eq!(moved.entities[1].entity, hex(WIRED_ESP.0));
        assert_eq!(
            moved.entities[1].bindings[0].talker,
            Some(hex(talker_here.0))
        );
        // So is the listening device's own record of its binding.
        assert_eq!(
            moved.host.as_ref().unwrap().endpoints[1].streams[0].bound_to(),
            Some((talker_here, 0))
        );
        // On the computer it was saved on, nothing changes.
        assert_eq!(moved_to(&preset, saved_on), preset);
    }

    #[test]
    fn a_recall_waits_for_this_computers_endpoints() {
        use triib_endpointd::status::DaemonStatus;

        let mut triib = sample();
        triib.settings.interface = Some("enp6s0".to_owned());
        let here = triib.interface().unwrap().mac;
        let host = HostPreset {
            mac: here.to_string(),
            endpoints: host_endpoints(),
        };
        let ids = host.entity_ids(here);
        let preset = Preset {
            name: "Show".to_owned(),
            entities: Vec::new(),
            host: Some(host),
        };
        // The daemon has not started on them yet.
        triib.endpoints = Some(DaemonStatus {
            pid: 1,
            started: 1_000,
            interface: "enp6s0".to_owned(),
            gptp: String::new(),
            foreign_mrp: false,
            config: None,
            clock: None,
            setup: None,
            sharers: Vec::new(),
            endpoints: Vec::new(),
        });
        assert!(!host_ready(&triib, &preset, 2_000));
        // It has, but their entities are not here yet.
        triib.endpoints.as_mut().unwrap().started = 3_000;
        assert!(!host_ready(&triib, &preset, 2_000));
        // Here and read, as the wired ESP is.
        for id in &ids {
            let entity = triib.entities[&WIRED_ESP];
            let model = triib.models[&WIRED_ESP].clone();
            triib.entities.insert(*id, entity);
            triib.models.insert(*id, model);
        }
        assert!(host_ready(&triib, &preset, 2_000));
        // The app goes on with the preset when the daemon next says what
        // it is doing.
        triib.pending_recall = Some((preset, 2_000, std::time::Instant::now()));
        let status = triib.endpoints.clone();
        let _ = triib.update(Message::External(crate::External::Endpoints(status)));
        assert!(triib.pending_recall.is_none());
        assert_eq!(
            triib.preset_report.as_deref(),
            Some("Nothing differs from \"Show\".")
        );
    }

    #[test]
    fn file_names_come_from_preset_names() {
        assert_eq!(file_name("Sunday Service"), "sunday-service.toml");
        assert_eq!(file_name("  Show #2 / FOH "), "show-2-foh.toml");
        assert_eq!(file_name("!!"), "preset.toml");
        assert_eq!(octets("ff9c"), Some(vec![0xff, 0x9c]));
        assert_eq!(number(&hex(WIRED_ESP.0)), Some(WIRED_ESP.0));
    }
}
