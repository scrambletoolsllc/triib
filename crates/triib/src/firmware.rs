//! Updating an entity's firmware: the images it takes new firmware in,
//! each a FIRMWARE_IMAGE memory object, as its own and its coprocessor's,
//! the file chosen for one and confirmed, and how its upload goes.
//!
//! ATDECC says nothing of what file an image takes, so triib reads what it
//! can of the file itself: an ESP-IDF app image names its chip, project
//! and version. Before sending, it doubts a coprocessor's file for an
//! image that is not one, or the other way round, and a file unlike the
//! one the same model's image took last time, which it keeps in its state
//! folder. After the entity restarts on its own new firmware, triib
//! watches for it going back to the firmware it had, as an ESP32 does
//! when the new firmware does not keep running.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use triib_firmware::esp_image::{self, Running};

use atdecc::EntityId;
use atdecc::controller::Outcome;
use atdecc::descriptor::{MemoryObjectDescriptor, MemoryObjectType};
use atdecc::model::EntityModel;
use atdecc::upload::{Failure, Stage};
use iced::widget::space;
use iced::{Center, Element, Fill, Task};
use scramble_ui::button::{self, Kind, Size};
use scramble_ui::component;
use scramble_ui::font::{Type, styled};
use scramble_ui::icon::Icon;
use scramble_ui::{dir, style};

use crate::app::Triib;
use crate::describe;
use crate::fl;

/// What the person asks of an update.
#[derive(Debug, Clone)]
pub enum Ask {
    /// Choose a file for one of an entity's firmware images.
    Choose(EntityId, u16),
    Chosen(EntityId, u16, Result<Option<PathBuf>, String>),
    /// Send the file chosen.
    Confirmed,
    /// Put the file chosen back.
    Canceled,
    /// Stop an upload.
    Stop(EntityId, u16),
    /// Clear an update that ended from the inspector.
    Dismissed(EntityId, u16),
}

/// A file chosen for one of an entity's firmware images, waiting for the
/// person to confirm it.
#[derive(Debug, Clone)]
pub struct Chosen {
    pub entity: EntityId,
    pub index: u16,
    /// The file's name, as shown.
    pub file: String,
    pub image: Vec<u8>,
    /// What the file is, where triib can tell.
    pub kind: Option<FileKind>,
    /// Why triib doubts the file suits the image, each in a sentence.
    pub doubts: Vec<String>,
    /// What triib notes of the file without doubting it.
    pub notes: Vec<String>,
}

/// What a firmware file is, where triib can tell: an ESP-IDF app image's
/// chip, project and version, the start of its ELF file's hash, whether
/// it is a coprocessor's and how many signatures it carries, as
/// [`esp_image`] reads them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileKind {
    pub chip: String,
    pub project: String,
    pub version: String,
    pub elf: String,
    #[serde(default)]
    pub coprocessor: bool,
    #[serde(default)]
    pub signatures: usize,
}

impl FileKind {
    /// Whether it is for the same chip and project as `other`.
    fn like(&self, other: &FileKind) -> bool {
        self.chip == other.chip && self.project == other.project
    }
}

/// How long after an entity is back on new firmware triib watches for it
/// going back to the firmware it had: an ESP32 keeps new firmware once
/// its gPTP is valid, and goes back after 60 s without.
const KEEP_WATCH: Duration = Duration::from_secs(75);

/// An update under way or ended.
#[derive(Debug, Clone)]
pub struct Update {
    pub file: String,
    pub stage: Stage,
    /// The entity is back, restarted on the image.
    pub back: bool,
    /// The available_index the entity last advertised as the image went
    /// to it: back from restarting, it advertises a lower one.
    pub advertised: Option<u32>,
    pub kind: Option<FileKind>,
    /// The image is a coprocessor's, whose firmware the entity's version
    /// does not name.
    pub coprocessor: bool,
    /// The entity's firmware version before the update, and once back.
    pub before: String,
    pub after: Option<String>,
    /// When the version once back was read.
    pub read_at: Option<Instant>,
    /// It went back to the firmware it had.
    pub went_back: bool,
    /// It came back running neither the new firmware nor the one it had.
    pub elsewhere: bool,
}

impl Update {
    pub fn new(file: impl Into<String>) -> Self {
        Self {
            file: file.into(),
            stage: Stage::Locking,
            back: false,
            advertised: None,
            kind: None,
            coprocessor: false,
            before: String::new(),
            after: None,
            read_at: None,
            went_back: false,
            elsewhere: false,
        }
    }

    /// Whether triib still watches for the entity going back to the
    /// firmware it had: for a while once back on a version of its own
    /// that differs from the one it had.
    pub fn watching(&self) -> bool {
        !self.went_back
            && !self.elsewhere
            && !self.coprocessor
            && self
                .after
                .as_ref()
                .is_some_and(|after| *after != self.before)
            && self.read_at.is_some_and(|read| read.elapsed() < KEEP_WATCH)
    }
}

#[derive(Debug, Default)]
pub struct Firmware {
    pub chosen: Option<Chosen>,
    /// By entity and memory object.
    pub updates: BTreeMap<(EntityId, u16), Update>,
    /// What each model's images took last, by [`learned_key`].
    pub learned: BTreeMap<String, FileKind>,
}

impl Firmware {
    /// What images took before, kept in the state folder.
    pub fn load() -> Self {
        let learned = learned_file()
            .and_then(|file| std::fs::read(file).ok())
            .and_then(|octets| serde_json::from_slice(&octets).ok())
            .unwrap_or_default();
        Self {
            learned,
            ..Self::default()
        }
    }

    /// Keeps what an image took, for the next file chosen for it.
    fn learn(&mut self, key: String, kind: FileKind) {
        if self.learned.get(&key) == Some(&kind) {
            return;
        }
        self.learned.insert(key, kind);
        if let Some(file) = learned_file()
            && let Ok(text) = serde_json::to_vec_pretty(&self.learned)
        {
            if let Some(folder) = file.parent() {
                let _ = std::fs::create_dir_all(folder);
            }
            let _ = std::fs::write(file, text);
        }
    }

    /// An entity's version, read in full: once back from an update, the
    /// one it restarted on, and while triib watches, whether it went back
    /// to the one it had.
    pub fn version_read(&mut self, entity: EntityId, version: &str) {
        for (&(updated, _), update) in &mut self.updates {
            if updated != entity || update.stage != Stage::Done || !update.back {
                continue;
            }
            // By the ELF hash an ESP32's version ends with, else by the
            // version itself.
            let verdict = update
                .kind
                .as_ref()
                .filter(|_| !update.coprocessor)
                .and_then(|kind| running(kind, &update.before, version));
            match &update.after {
                None => {
                    update.after = Some(version.to_owned());
                    update.read_at = Some(Instant::now());
                    match verdict {
                        Some(Running::Old) => update.went_back = true,
                        Some(Running::Other) => update.elsewhere = true,
                        _ => {}
                    }
                }
                Some(after) => {
                    let back_on_old = match verdict {
                        Some(verdict) => verdict == Running::Old,
                        None => version != after && version == update.before,
                    };
                    if update.watching() && back_on_old {
                        update.went_back = true;
                    }
                }
            }
        }
    }

    /// Whether an update to `entity` is under way.
    pub fn updating(&self, entity: EntityId) -> bool {
        self.updates
            .iter()
            .any(|(&(updated, _), update)| updated == entity && !ended(update))
    }

    /// An entity back online advertising `index`, as after restarting on
    /// new firmware. One that has not restarted advertises on from the
    /// index it had: an ESP32 that stored an image says it departs, then
    /// answers and advertises for 300 ms more before it restarts, and
    /// what is read of it then is the firmware it had.
    pub fn entity_back(&mut self, entity: EntityId, index: u32) {
        for (&(updated, _), update) in &mut self.updates {
            if updated == entity
                && update.stage == Stage::Done
                && update
                    .advertised
                    .is_none_or(|last| index < last || index == 0)
            {
                update.back = true;
            }
        }
    }

    /// The network stopped, and the uploads with it.
    pub fn network_stopped(&mut self) {
        self.chosen = None;
        self.updates.retain(|_, update| ended(update));
    }
}

/// Whether an update has ended: the entity back where it restarts, and
/// triib no longer watching for it going back.
fn ended(update: &Update) -> bool {
    match update.stage {
        Stage::Done => update.back && !update.watching(),
        Stage::Aborted | Stage::Failed(_) => true,
        _ => false,
    }
}

/// Where what images took is kept.
fn learned_file() -> Option<PathBuf> {
    triib_store::paths::state_dir().map(|dir| dir.join("firmware-images.json"))
}

/// What an image took is kept under its entity model and name.
fn learned_key(triib: &Triib, entity: EntityId, image: &str) -> Option<String> {
    let model = triib.entities.get(&entity)?.adp.entity_model_id;
    Some(format!("{:016x}/{image}", model.0))
}

/// What `file` is, where triib can tell: an ESP-IDF app image.
pub fn identify(file: &[u8]) -> Option<FileKind> {
    let image = esp_image::parse(file).ok()?;
    Some(FileKind {
        chip: image.chip(),
        project: image.project.clone(),
        version: image.version.clone(),
        elf: image.elf_sha9(),
        coprocessor: image.is_coprocessor(),
        signatures: image.signatures,
    })
}

/// What a device runs after an update of `kind`, from its version before
/// and after, where its version ends with an ELF hash, as an ESP32's does.
fn running(kind: &FileKind, before: &str, after: &str) -> Option<Running> {
    let now = esp_image::version_sha9(after)?;
    Some(if now == kind.elf {
        Running::New
    } else if Some(&now) == esp_image::version_sha9(before).as_ref() {
        Running::Old
    } else {
        Running::Other
    })
}

/// What triib notes of `kind` for the firmware image `index` of `entity`,
/// each in a sentence, without doubting it: a file not signed, and one the
/// entity runs already.
pub fn notes(triib: &Triib, entity: EntityId, kind: Option<&FileKind>) -> Vec<String> {
    let Some(kind) = kind else {
        return Vec::new();
    };
    let mut notes = Vec::new();
    if kind.signatures == 0 {
        notes.push(fl!("firmware-unsigned"));
    }
    let version = triib
        .models
        .get(&entity)
        .and_then(EntityModel::entity)
        .map(|descriptor| descriptor.firmware_version);
    if version.and_then(esp_image::version_sha9).as_ref() == Some(&kind.elf) {
        notes.push(fl!("firmware-runs-already"));
    }
    notes
}

/// Why triib doubts `kind` suits the firmware image `index` of `entity`,
/// each in a sentence: a coprocessor's file for an image that is not
/// one or the other way round, and a file unlike the one the image took
/// last time or like one another image took.
pub fn doubts(triib: &Triib, entity: EntityId, index: u16, kind: Option<&FileKind>) -> Vec<String> {
    let Some(kind) = kind else {
        return Vec::new();
    };
    let Some(model) = triib.models.get(&entity) else {
        return Vec::new();
    };
    let Some(object) = image_of(model, index) else {
        return Vec::new();
    };
    let image = image_name(&object);
    let mut doubts = Vec::new();
    match (kind.coprocessor, esp_image::object_is_coprocessor(&image)) {
        (true, false) => doubts.push(fl!("firmware-doubt-coprocessor", image = image.as_str())),
        (false, true) => {
            doubts.push(fl!(
                "firmware-doubt-not-coprocessor",
                image = image.as_str()
            ));
        }
        _ => {}
    }
    let learned = |name: &str| {
        learned_key(triib, entity, name).and_then(|key| triib.firmware.learned.get(&key))
    };
    if let Some(last) = learned(&image)
        && !kind.like(last)
    {
        doubts.push(fl!(
            "firmware-doubt-learned",
            image = image.as_str(),
            project = last.project.as_str(),
            chip = last.chip.as_str()
        ));
    }
    for other in images(model).filter(|other| other.index != index) {
        let other = image_name(&other);
        if learned(&other).is_some_and(|last| kind.like(last)) {
            doubts.push(fl!("firmware-doubt-other", image = other.as_str()));
        }
    }
    doubts
}

/// The memory objects an entity takes firmware in.
pub fn images(model: &EntityModel) -> impl Iterator<Item = MemoryObjectDescriptor<'_>> {
    model
        .memory_objects()
        .filter(|object| object.memory_object_type == MemoryObjectType::FIRMWARE_IMAGE)
}

/// An image's name: its memory object's, else "Firmware".
fn image_name(object: &MemoryObjectDescriptor<'_>) -> String {
    if object.object_name.is_empty() {
        fl!("firmware-image")
    } else {
        object.object_name.to_owned()
    }
}

fn image_of(model: &EntityModel, index: u16) -> Option<MemoryObjectDescriptor<'_>> {
    images(model).find(|object| object.index == index)
}

/// Carries out what the person asked.
pub fn update(triib: &mut Triib, ask: Ask) -> Task<crate::app::Message> {
    match ask {
        Ask::Choose(entity, index) => {
            let Some(object) = triib
                .models
                .get(&entity)
                .and_then(|model| image_of(model, index))
            else {
                return Task::none();
            };
            let title = fl!("firmware-choose-title", image = image_name(&object));
            return Task::perform(crate::dialog::choose_file(title), move |result| {
                crate::app::Message::Firmware(Ask::Chosen(entity, index, result))
            });
        }
        Ask::Chosen(entity, index, Ok(Some(path))) => {
            let file = path.file_name().map_or_else(
                || path.display().to_string(),
                |name| name.to_string_lossy().into_owned(),
            );
            let image = match std::fs::read(&path) {
                Ok(image) => image,
                Err(error) => {
                    triib.notice = Some(fl!(
                        "firmware-read-failed",
                        file = file,
                        error = error.to_string()
                    ));
                    return Task::none();
                }
            };
            match fits(triib, entity, index, &file, &image) {
                Ok(()) => {
                    let kind = identify(&image);
                    let doubts = doubts(triib, entity, index, kind.as_ref());
                    let notes = notes(triib, entity, kind.as_ref());
                    triib.firmware.chosen = Some(Chosen {
                        entity,
                        index,
                        file,
                        image,
                        kind,
                        doubts,
                        notes,
                    });
                }
                Err(why) => triib.notice = Some(why),
            }
        }
        Ask::Chosen(_, _, Ok(None)) => {}
        Ask::Chosen(_, _, Err(error)) => {
            triib.notice = Some(fl!("app-file-dialog-failed", error = error));
        }
        Ask::Confirmed => {
            let Some(chosen) = triib.firmware.chosen.take() else {
                return Task::none();
            };
            if let Err(why) = send(
                triib,
                chosen.entity,
                chosen.index,
                chosen.file,
                chosen.image,
            ) {
                triib.notice = Some(why);
            }
        }
        Ask::Canceled => triib.firmware.chosen = None,
        Ask::Stop(entity, index) => {
            if let Some(network) = triib.network() {
                network.abort_upload(entity, index);
            }
        }
        Ask::Dismissed(entity, index) => {
            triib.firmware.updates.remove(&(entity, index));
        }
    }
    Task::none()
}

/// Whether `image`, from `file`, fits the firmware image `index` of
/// `entity`, or why not.
fn fits(
    triib: &Triib,
    entity: EntityId,
    index: u16,
    file: &str,
    image: &[u8],
) -> Result<(), String> {
    let object = triib
        .models
        .get(&entity)
        .and_then(|model| image_of(model, index))
        .ok_or_else(|| fl!("firmware-failed-gone"))?;
    if image.is_empty() {
        Err(fl!("firmware-empty", file = file))
    } else if object.maximum_length > 0 && image.len() as u64 > object.maximum_length {
        Err(fl!(
            "firmware-too-large",
            file = file,
            size = size(image.len() as u64),
            most = size(object.maximum_length),
            image = image_name(&object)
        ))
    } else {
        Ok(())
    }
}

/// Sends `image`, from `file`, to the firmware image `index` of `entity`,
/// which restarts on it, as the person confirmed or an agent asked; or
/// says why it cannot go.
pub fn send(
    triib: &mut Triib,
    entity: EntityId,
    index: u16,
    file: String,
    image: Vec<u8>,
) -> Result<(), String> {
    fits(triib, entity, index, &file, &image)?;
    if triib.firmware.updating(entity) {
        return Err(fl!("firmware-busy"));
    }
    let Some(network) = triib.network() else {
        return Err(fl!("firmware-no-network"));
    };
    let kind = identify(&image);
    network.upload(entity, index, image, true);
    let model = triib.models.get(&entity);
    let coprocessor = model
        .and_then(|model| image_of(model, index))
        .is_some_and(|object| esp_image::object_is_coprocessor(&image_name(&object)));
    let before = model
        .and_then(EntityModel::entity)
        .map(|descriptor| descriptor.firmware_version.to_owned())
        .unwrap_or_default();
    triib.firmware.updates.insert(
        (entity, index),
        Update {
            kind,
            coprocessor,
            before,
            ..Update::new(file)
        },
    );
    Ok(())
}

/// Where an upload has got to, as the network reported it, answering the
/// agents that wait on it once it ends.
pub fn reported(
    triib: &mut Triib,
    entity: EntityId,
    index: u16,
    stage: Stage,
    advertised: Option<u32>,
) {
    let Some(update) = triib.firmware.updates.get_mut(&(entity, index)) else {
        return;
    };
    update.stage = stage;
    if advertised.is_some() {
        update.advertised = advertised;
    }
    if !matches!(stage, Stage::Done | Stage::Aborted | Stage::Failed(_)) {
        return;
    }
    let update = update.clone();
    // Stored: what the image took, for the next file chosen for it.
    if stage == Stage::Done
        && let Some(kind) = update.kind.clone()
        && let Some(object) = triib
            .models
            .get(&entity)
            .and_then(|model| image_of(model, index))
        && let Some(key) = learned_key(triib, entity, &image_name(&object))
    {
        triib.firmware.learn(key, kind);
    }
    let waits = std::mem::take(&mut triib.agents.firmware_waits);
    for (key, answer) in waits {
        if key == (entity, index) {
            answer.send(crate::tools::firmware_finished(
                triib, entity, index, &update,
            ));
        } else {
            triib.agents.firmware_waits.push((key, answer));
        }
    }
}

/// A size in octets, as kilobytes or megabytes, its number and unit kept
/// on one line.
fn size(octets: u64) -> String {
    let size = if octets >= 1_000_000 {
        fl!(
            "firmware-size-mb",
            size = crate::i18n::decimal(format!("{:.1}", octets as f64 / 1e6))
        )
    } else {
        fl!("firmware-size-kb", size = octets.div_ceil(1000))
    };
    // Placed in a sentence, without the right to left mark a right to
    // left language's text starts with, so it reads left to right, as
    // numbers and units do.
    size.trim_start_matches('\u{200f}').replace(' ', "\u{a0}")
}

/// The inspector's section of the entity's firmware images, each with a
/// button to update it, or how its update goes.
pub fn entity_section<'a>(
    triib: &'a Triib,
    entity: EntityId,
    model: &'a EntityModel,
) -> Vec<Element<'a, crate::app::Message>> {
    let mut items = Vec::new();
    let objects: Vec<_> = images(model).collect();
    if objects.is_empty() {
        return items;
    }
    items.push(component::section(fl!("firmware-section")));
    let running = triib.network().is_some();
    let updating = triib.firmware.updating(entity);
    for object in objects {
        let update = triib.firmware.updates.get(&(entity, object.index));
        let line: Element<'a, crate::app::Message> = match update {
            // Under the image's name, where the file's name has room.
            Some(update) => {
                items.push(crate::view::stacked(
                    image_name(&object),
                    progress(entity, object.index, update),
                ));
                continue;
            }
            None => {
                let message = (running && !updating).then_some(crate::app::Message::Firmware(
                    Ask::Choose(entity, object.index),
                ));
                let takes = if object.maximum_length > 0 {
                    fl!("firmware-takes", size = size(object.maximum_length))
                } else {
                    String::new()
                };
                scramble_ui::row![
                    styled(takes, Type::BodyMedium)
                        .style(style::on_surface_variant)
                        .width(Fill)
                        .align_x(dir::text_start()),
                    component::tip(
                        button::icon_button(Icon::Upload)
                            .size(Size::ExtraSmall)
                            .on_press_maybe(message),
                        fl!("firmware-update"),
                    ),
                ]
                .spacing(4)
                .align_y(Center)
                .into()
            }
        };
        items.push(crate::view::labeled(image_name(&object), line));
    }
    items
}

/// How an update goes, with a button to stop it while it can be, or to
/// clear it once ended.
fn progress<'a>(entity: EntityId, index: u16, update: &Update) -> Element<'a, crate::app::Message> {
    let failed = matches!(update.stage, Stage::Failed(_)) || update.went_back || update.elsewhere;
    let text = styled(stage_text(update), Type::BodyMedium)
        .width(Fill)
        .align_x(dir::text_start());
    let text = if failed {
        text.style(style::error_text)
    } else {
        text
    };
    let stoppable = matches!(
        update.stage,
        Stage::Locking | Stage::Starting | Stage::Writing { .. }
    );
    let button: Element<'a, crate::app::Message> = if stoppable {
        component::tip(
            button::icon_button(Icon::Close)
                .size(Size::ExtraSmall)
                .on_press(crate::app::Message::Firmware(Ask::Stop(entity, index))),
            fl!("firmware-stop"),
        )
    } else if ended(update) {
        component::tip(
            button::icon_button(Icon::Close)
                .size(Size::ExtraSmall)
                .on_press(crate::app::Message::Firmware(Ask::Dismissed(entity, index))),
            fl!("firmware-dismiss"),
        )
    } else {
        space().into()
    };
    let line = scramble_ui::row![text, button].spacing(4).align_y(Center);
    // How far, while it can tell.
    let fraction = match update.stage {
        Stage::Writing { sent, total } if total > 0 => Some(sent as f32 / total as f32),
        Stage::Storing {
            progress: Some(progress),
        } if progress <= 1000 => Some(f32::from(progress) / 1000.0),
        _ => None,
    };
    match fraction {
        Some(fraction) => scramble_ui::column![line, bar(fraction)].spacing(6).into(),
        None => line.into(),
    }
}

/// A thin bar `fraction` full.
fn bar<'a>(fraction: f32) -> Element<'a, crate::app::Message> {
    iced::widget::progress_bar(0.0..=1.0, fraction)
        .girth(4)
        .length(Fill)
        .style(|theme: &iced::Theme| {
            let scheme = scramble_ui::Scheme::of(theme);
            iced::widget::progress_bar::Style {
                background: iced::Background::Color(scheme.surface_container_highest),
                bar: iced::Background::Color(scheme.primary),
                border: iced::Border::default().rounded(2),
            }
        })
        .into()
}

/// Where an update has got to, in words.
pub fn stage_text(update: &Update) -> String {
    match update.stage {
        Stage::Locking | Stage::Starting => fl!("firmware-starting", file = update.file.as_str()),
        Stage::Writing { sent, total } => fl!(
            "firmware-sending",
            file = update.file.as_str(),
            percent = (sent * 100).checked_div(total).unwrap_or(0)
        ),
        Stage::Aborting => fl!("firmware-stopping"),
        Stage::Storing {
            progress: Some(progress),
        } if progress <= 1000 => {
            let percent = progress / 10;
            fl!("firmware-storing-percent", percent = percent)
        }
        Stage::Storing { .. } => fl!("firmware-storing"),
        Stage::Done if update.went_back => {
            fl!("firmware-went-back", version = update.before.as_str())
        }
        Stage::Done if update.elsewhere => fl!(
            "firmware-came-back-other",
            version = update.after.as_deref().unwrap_or_default()
        ),
        Stage::Done if update.back && update.watching() => {
            fl!("firmware-checking", file = update.file.as_str())
        }
        Stage::Done if update.back => fl!("firmware-updated", file = update.file.as_str()),
        Stage::Done => fl!("firmware-restarting"),
        Stage::Aborted => fl!("firmware-stopped"),
        Stage::Failed(failure) => failure_text(failure),
    }
}

fn failure_text(failure: Failure) -> String {
    match failure {
        Failure::DoesNotFit => fl!("firmware-failed-size"),
        Failure::Locked => fl!("firmware-failed-locked"),
        Failure::Refused(_, Outcome::NoResponse) => fl!("firmware-failed-no-response"),
        Failure::Refused(_, Outcome::NotPossible) | Failure::Gone => fl!("firmware-failed-gone"),
        Failure::Refused(_, outcome) => fl!(
            "firmware-failed",
            reason = describe::reason(outcome).unwrap_or_default()
        ),
        Failure::Write { offset, status } => fl!(
            "firmware-failed-write",
            offset = offset,
            reason = status
                .name()
                .map_or_else(|| status.0.to_string(), describe::flag_name)
        ),
        Failure::Rejected => fl!("firmware-failed-rejected"),
        Failure::NoStatus => fl!("firmware-failed-no-status"),
    }
}

/// The dialog asking the person to confirm the file chosen, over the
/// window.
pub fn dialog(triib: &Triib) -> Option<Element<'_, crate::app::Message>> {
    let chosen = triib.firmware.chosen.as_ref()?;
    let model = triib.models.get(&chosen.entity);
    let image = model
        .and_then(|model| image_of(model, chosen.index))
        .map_or_else(|| fl!("firmware-image"), |object| image_name(&object));
    let version = model
        .and_then(EntityModel::entity)
        .map(|descriptor| descriptor.firmware_version.to_owned())
        .filter(|version| !version.is_empty())
        .unwrap_or_else(|| fl!("common-not-set"));
    let mut body = vec![fl!(
        "firmware-confirm-body",
        entity = triib.entity_name_of(chosen.entity),
        file = chosen.file.as_str(),
        size = size(chosen.image.len() as u64),
        image = image.as_str(),
        version = version
    )];
    if let Some(kind) = &chosen.kind {
        body.push(fl!(
            "firmware-file",
            project = kind.project.as_str(),
            version = kind.version.as_str(),
            chip = kind.chip.as_str(),
            elf = kind.elf.as_str()
        ));
    }
    body.extend(chosen.notes.iter().cloned());
    if esp_image::object_is_coprocessor(&image) {
        body.push(fl!("firmware-confirm-coprocessor"));
    }
    body.extend(chosen.doubts.iter().cloned());
    let doubted = !chosen.doubts.is_empty();
    let update = if doubted {
        fl!("firmware-confirm-anyway")
    } else {
        fl!("firmware-confirm-update")
    };
    Some(component::dialog(
        space().width(Fill).height(Fill),
        Some(if doubted { Icon::Warning } else { Icon::Upload }),
        fl!("firmware-confirm-title"),
        body.join("\n\n"),
        vec![
            button::button(Kind::Text, fl!("firmware-confirm-cancel"))
                .on_press(crate::app::Message::Firmware(Ask::Canceled))
                .into(),
            button::button(Kind::Filled, update)
                .on_press(crate::app::Message::Firmware(Ask::Confirmed))
                .into(),
        ],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn update(stage: Stage, back: bool) -> Update {
        Update {
            stage,
            back,
            ..Update::new("avb_bridge.bin")
        }
    }

    #[test]
    fn an_update_ends_once_the_entity_is_back() {
        let mut firmware = Firmware::default();
        let entity = EntityId(1);
        firmware.updates.insert(
            (entity, 3),
            update(Stage::Writing { sent: 1, total: 2 }, false),
        );
        assert!(firmware.updating(entity));
        firmware.updates.get_mut(&(entity, 3)).unwrap().stage = Stage::Done;
        assert!(firmware.updating(entity));
        firmware.entity_back(entity, 0);
        assert!(!firmware.updating(entity));
    }

    /// An entity that says it departs and advertises again on the index it
    /// had has not restarted yet: what is read of it then is the firmware
    /// it had, and it is back once it advertises from a lower index.
    #[test]
    fn advertising_on_after_departing_is_not_back() {
        let mut firmware = Firmware::default();
        let entity = EntityId(1);
        let mut done = update(Stage::Done, false);
        done.before = "1.4.0".to_owned();
        done.advertised = Some(18_000);
        firmware.updates.insert((entity, 3), done);
        firmware.entity_back(entity, 18_001);
        firmware.version_read(entity, "1.4.0");
        let stayed = &firmware.updates[&(entity, 3)];
        assert!(!stayed.back && !stayed.went_back && stayed.after.is_none());
        firmware.entity_back(entity, 1);
        firmware.version_read(entity, "1.5.0");
        let back = &firmware.updates[&(entity, 3)];
        assert!(back.back && !back.went_back);
        assert_eq!(back.after.as_deref(), Some("1.5.0"));
    }

    #[test]
    fn the_network_stopping_drops_the_uploads_under_way() {
        let mut firmware = Firmware::default();
        let entity = EntityId(1);
        firmware.updates.insert(
            (entity, 3),
            update(Stage::Storing { progress: None }, false),
        );
        firmware
            .updates
            .insert((entity, 4), update(Stage::Failed(Failure::Rejected), false));
        firmware.network_stopped();
        assert_eq!(firmware.updates.keys().collect::<Vec<_>>(), [&(entity, 4)]);
    }

    /// An ESP-IDF app image's header and description, for `chip` and
    /// `project`.
    fn esp_image(chip: u16, project: &str) -> Vec<u8> {
        let mut image = vec![0; 4096];
        image[0] = 0xe9;
        image[12..14].copy_from_slice(&chip.to_le_bytes());
        image[32..36].copy_from_slice(&0xabcd_5432_u32.to_le_bytes());
        image[48..48 + 13].copy_from_slice(b"c159fad-dirty");
        image[80..80 + project.len()].copy_from_slice(project.as_bytes());
        image[176..181].copy_from_slice(&[0x08, 0x04, 0x88, 0x4a, 0x9f]);
        image
    }

    #[test]
    fn an_esp_app_image_says_what_it_is() {
        let kind = identify(&esp_image(0x000d, "avb_bridge_coprocessor")).unwrap();
        assert_eq!(kind.chip, "ESP32-C6");
        assert_eq!(kind.project, "avb_bridge_coprocessor");
        assert_eq!(kind.version, "c159fad-dirty");
        assert_eq!(kind.elf, "0804884a9");
        assert!(kind.coprocessor);
        assert_eq!(kind.signatures, 0);
        let own = identify(&esp_image(0x0012, "avb_endpoint")).unwrap();
        assert_eq!(own.chip, "ESP32-P4");
        assert!(!own.coprocessor);
        assert!(!own.like(&kind));
        // Anything else is not told apart.
        assert_eq!(identify(&[0x7f, b'E', b'L', b'F']), None);
        assert_eq!(identify(&vec![0; 4096]), None);
    }

    /// Back on new firmware, triib watches for a while, and an entity that
    /// restarts on the firmware it had went back.
    #[test]
    fn going_back_to_the_firmware_it_had_shows() {
        let mut firmware = Firmware::default();
        let entity = EntityId(1);
        let mut done = update(Stage::Done, false);
        done.before = "1.4.0".to_owned();
        firmware.updates.insert((entity, 3), done.clone());
        firmware.entity_back(entity, 0);
        firmware.version_read(entity, "1.5.0");
        assert!(firmware.updates[&(entity, 3)].watching());
        assert!(firmware.updating(entity), "still watching");
        assert!(stage_text(&firmware.updates[&(entity, 3)]).contains("avb_bridge.bin"));
        firmware.version_read(entity, "1.4.0");
        let went = &firmware.updates[&(entity, 3)];
        assert!(went.went_back);
        assert!(!firmware.updating(entity));
        assert!(stage_text(went).contains("1.4.0"));

        // The same version again, or a coprocessor's, tells nothing.
        for (after, coprocessor) in [("1.4.0", false), ("1.5.0", true)] {
            let mut firmware = Firmware::default();
            let mut done = done.clone();
            done.coprocessor = coprocessor;
            firmware.updates.insert((entity, 3), done);
            firmware.entity_back(entity, 0);
            firmware.version_read(entity, after);
            assert!(!firmware.updates[&(entity, 3)].watching());
            assert!(!firmware.updating(entity));
        }
    }

    /// An ESP32's version ends with its firmware's ELF hash, which tells
    /// the new firmware from the one it had and from anything else.
    #[test]
    fn the_elf_hash_tells_what_it_runs() {
        let entity = EntityId(1);
        let kind = identify(&esp_image(0x0012, "avb_endpoint")).unwrap();
        let before = "v003 (esp_avb 2.45.0 - dccd7ef2f)";
        let new = "v004 (esp_avb 2.46.0 - 0804884a9)";
        let started = |firmware: &mut Firmware| {
            let mut done = update(Stage::Done, false);
            done.before = before.to_owned();
            done.kind = Some(kind.clone());
            firmware.updates.insert((entity, 3), done);
            firmware.entity_back(entity, 0);
        };
        // On the new firmware, then back on the old one.
        let mut firmware = Firmware::default();
        started(&mut firmware);
        firmware.version_read(entity, new);
        assert!(firmware.updates[&(entity, 3)].watching());
        firmware.version_read(entity, before);
        assert!(firmware.updates[&(entity, 3)].went_back);
        // Back on the old one at once.
        let mut firmware = Firmware::default();
        started(&mut firmware);
        firmware.version_read(entity, before);
        assert!(firmware.updates[&(entity, 3)].went_back);
        // On something else.
        let mut firmware = Firmware::default();
        started(&mut firmware);
        firmware.version_read(entity, "v005 (esp_avb 2.46.0 - aaaaaaaaa)");
        let other = &firmware.updates[&(entity, 3)];
        assert!(other.elsewhere && !other.watching());
        assert!(stage_text(other).contains("aaaaaaaaa"));
    }

    #[test]
    fn sizes_read_in_kilobytes_and_megabytes() {
        assert_eq!(size(1_187_000), "1.2\u{a0}MB");
        assert_eq!(size(4096), "5\u{a0}KB");
    }

    #[test]
    fn progress_reads_in_percent() {
        let text = stage_text(&update(
            Stage::Writing {
                sent: 512,
                total: 1024,
            },
            false,
        ));
        assert!(text.contains("50"), "{text}");
        let text = stage_text(&update(
            Stage::Storing {
                progress: Some(995),
            },
            false,
        ));
        assert!(text.contains("99"), "{text}");
    }
}
