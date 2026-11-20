//! Updating an entity's firmware: the images it takes new firmware in,
//! each a FIRMWARE_IMAGE memory object, as its own and its coprocessor's,
//! the file chosen for one and confirmed, and how its upload goes.

use std::collections::BTreeMap;
use std::path::PathBuf;

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
}

/// An update under way or ended.
#[derive(Debug, Clone)]
pub struct Update {
    pub file: String,
    pub stage: Stage,
    /// The entity is back, restarted on the image.
    pub back: bool,
}

#[derive(Debug, Default)]
pub struct Firmware {
    pub chosen: Option<Chosen>,
    /// By entity and memory object.
    pub updates: BTreeMap<(EntityId, u16), Update>,
}

impl Firmware {
    /// Whether an update to `entity` is under way.
    pub fn updating(&self, entity: EntityId) -> bool {
        self.updates
            .iter()
            .any(|(&(updated, _), update)| updated == entity && !ended(update))
    }

    /// An entity back online, as after restarting on new firmware.
    pub fn entity_back(&mut self, entity: EntityId) {
        for (&(updated, _), update) in &mut self.updates {
            if updated == entity && update.stage == Stage::Done {
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

/// Whether an update has ended, the entity back where it restarts.
fn ended(update: &Update) -> bool {
    match update.stage {
        Stage::Done => update.back,
        Stage::Aborted | Stage::Failed(_) => true,
        _ => false,
    }
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
                    triib.firmware.chosen = Some(Chosen {
                        entity,
                        index,
                        file,
                        image,
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
    network.upload(entity, index, image, true);
    triib.firmware.updates.insert(
        (entity, index),
        Update {
            file,
            stage: Stage::Locking,
            back: false,
        },
    );
    Ok(())
}

/// Where an upload has got to, as the network reported it, answering the
/// agents that wait on it once it ends.
pub fn reported(triib: &mut Triib, entity: EntityId, index: u16, stage: Stage) {
    let Some(update) = triib.firmware.updates.get_mut(&(entity, index)) else {
        return;
    };
    update.stage = stage;
    if !matches!(stage, Stage::Done | Stage::Aborted | Stage::Failed(_)) {
        return;
    }
    let update = update.clone();
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
    let failed = matches!(update.stage, Stage::Failed(_));
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
    let body = fl!(
        "firmware-confirm-body",
        entity = triib.entity_name_of(chosen.entity),
        file = chosen.file.as_str(),
        size = size(chosen.image.len() as u64),
        image = image,
        version = version
    );
    Some(component::dialog(
        space().width(Fill).height(Fill),
        Some(Icon::Upload),
        fl!("firmware-confirm-title"),
        body,
        vec![
            button::button(Kind::Text, fl!("firmware-confirm-cancel"))
                .on_press(crate::app::Message::Firmware(Ask::Canceled))
                .into(),
            button::button(Kind::Filled, fl!("firmware-confirm-update"))
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
            file: "avb_bridge.bin".into(),
            stage,
            back,
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
        firmware.entity_back(entity);
        assert!(!firmware.updating(entity));
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
