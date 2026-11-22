//! A device's log and crash dump: the log it keeps in a LOG_OBJECT memory
//! object, followed while its panel is open under the view, and the crash
//! dump it keeps in a CRASH_DUMP one, saved to a file or erased from the
//! inspector (see [`triib_devicelog`]). A device is asked nothing unless
//! its log is shown or its crash dump asked for; closing the panel stops
//! following, and showing the log again goes on where it stopped.

use std::collections::{HashMap, VecDeque};
use std::path::PathBuf;

use atdecc::EntityId;
use atdecc::model::EntityModel;
use iced::widget::{container, space};
use iced::{Center, Element, Fill, Font, Task};
use scramble_ui::button::{self, Kind, Size};
use scramble_ui::font::{Type, styled};
use scramble_ui::icon::Icon;
use scramble_ui::{column, component, dir, resize, row, style};
use triib_devicelog::crash::{self, CrashDump};
use triib_devicelog::log::{self, Entry, LogHeader, reset_reason_name};

use crate::app::{Message, Triib};
use crate::fl;
use crate::network::CrashReport;

/// The most lines kept of a device's log, the oldest dropped past it.
pub const LIMIT: usize = 20_000;
/// The most lines drawn at once, the newest.
const SHOWN: usize = 2_000;
/// The narrowest panel whose buttons keep their labels.
const LABELS_FIT: f32 = 760.0;

/// What the person asks of a device's log or crash dump.
#[derive(Debug, Clone)]
pub enum Ask {
    /// Show an entity's log in the panel, following it.
    Show(EntityId),
    /// Close the panel, no longer following the log.
    Close,
    /// Stop showing new lines, or show them again.
    Paused(bool),
    Cleared,
    /// Copy the log shown.
    Copy,
    /// Save the log shown to a file.
    Save,
    SaveTo(Result<Option<PathBuf>, String>),
    /// Save an entity's crash dump to a file.
    SaveCrash(EntityId),
    CrashTo(EntityId, Result<Option<PathBuf>, String>),
    /// Erase an entity's crash dump, once confirmed.
    EraseCrash(EntityId),
    EraseConfirmed,
    EraseCanceled,
}

/// A line of a device's log as the panel shows it: one the device wrote,
/// or what happened between them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Text(String),
    Boot(LogHeader),
    Restarted,
    Lost(u64),
    Dropped(u32),
    /// The device went off the network.
    Away,
    /// It came back.
    Back,
}

impl Line {
    /// The line as text, as copied and saved.
    pub fn text(&self, name: &str) -> String {
        match self {
            Line::Text(text) => text.clone(),
            Line::Boot(header) => format!("[{name}: {}]", log::boot_text(header)),
            Line::Restarted => format!("[{name} restarted]"),
            Line::Lost(octets) => {
                format!("[{name}: {octets} bytes lost, written faster than read]")
            }
            Line::Dropped(lines) => format!("[{name} dropped {lines} lines]"),
            Line::Away => format!("[{name} went away]"),
            Line::Back => format!("[{name} is back]"),
        }
    }
}

/// How saving or erasing an entity's crash dump goes, as the inspector
/// shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Crash {
    Reading {
        done: usize,
        total: usize,
    },
    Saved {
        path: PathBuf,
        octets: usize,
    },
    /// The entity keeps no crash now.
    Empty,
    Erasing,
    Erased,
    Failed(String),
}

/// The logs read, the one the panel shows, and the crash dumps asked for.
#[derive(Debug, Default)]
pub struct DeviceLogs {
    /// The entity whose log the panel shows.
    pub shown: Option<EntityId>,
    logs: HashMap<EntityId, VecDeque<Line>>,
    /// Lines that came while paused, shown on resuming.
    held: Vec<(EntityId, Line)>,
    pub paused: bool,
    pub crash: HashMap<EntityId, Crash>,
    /// Where each crash dump being read goes.
    crash_to: HashMap<EntityId, PathBuf>,
    /// The entity whose crash dump the person asked to erase, waiting to
    /// confirm.
    pub erase_asked: Option<EntityId>,
}

impl DeviceLogs {
    /// An entity's log as read so far.
    pub fn lines(&self, entity: EntityId) -> impl DoubleEndedIterator<Item = &Line> {
        self.logs.get(&entity).into_iter().flatten()
    }

    /// Whether an entity's log has been read.
    pub fn has(&self, entity: EntityId) -> bool {
        self.logs.contains_key(&entity)
    }

    /// An entity's log starts being read.
    pub fn start(&mut self, entity: EntityId) {
        self.logs.entry(entity).or_default();
    }

    fn add(&mut self, entity: EntityId, line: Line) {
        if self.paused && self.shown == Some(entity) {
            self.held.push((entity, line));
            return;
        }
        let lines = self.logs.entry(entity).or_default();
        lines.push_back(line);
        if lines.len() > LIMIT {
            lines.pop_front();
        }
    }

    /// What reading an entity's log brought.
    pub fn read(&mut self, entity: EntityId, entries: Vec<Entry>) {
        for entry in entries {
            let line = match entry {
                Entry::Line(text) => Line::Text(text),
                Entry::Boot(header) => Line::Boot(header),
                Entry::Restarted => Line::Restarted,
                Entry::Lost(octets) => Line::Lost(octets),
                Entry::Dropped(lines) => Line::Dropped(lines),
            };
            self.add(entity, line);
        }
    }

    /// An entity went off the network, or came back: marked in its log, if
    /// it is being read.
    pub fn away(&mut self, entity: EntityId, away: bool) {
        if self.has(entity) {
            self.add(entity, if away { Line::Away } else { Line::Back });
        }
    }

    fn resume(&mut self) {
        self.paused = false;
        for (entity, line) in std::mem::take(&mut self.held) {
            self.add(entity, line);
        }
    }

    /// The network stopped: nothing is followed, and the crash dumps asked
    /// for go unread.
    pub fn network_stopped(&mut self) {
        self.crash_to.clear();
        for crash in self.crash.values_mut() {
            if matches!(crash, Crash::Reading { .. } | Crash::Erasing) {
                *crash = Crash::Failed(fl!("device-log-network-stopped"));
            }
        }
    }
}

/// Whether an entity keeps a log, or a crash dump, for the inspector to
/// offer them.
pub fn keeps(model: &EntityModel) -> (bool, Option<CrashDump>) {
    (log::log_object(model).is_some(), crash::crash_dump(model))
}

pub fn update(triib: &mut Triib, ask: Ask) -> Task<Message> {
    match ask {
        Ask::Show(entity) => {
            if let (Some(shown), Some(network)) = (triib.device_logs.shown, triib.network())
                && shown != entity
            {
                network.follow_log(shown, false);
            }
            if let Some(network) = triib.network() {
                network.follow_log(entity, true);
            }
            triib.device_logs.logs.entry(entity).or_default();
            triib.device_logs.resume();
            triib.device_logs.shown = Some(entity);
        }
        Ask::Close => {
            if let Some(shown) = triib.device_logs.shown.take()
                && let Some(network) = triib.network()
            {
                network.follow_log(shown, false);
            }
            triib.device_logs.resume();
        }
        Ask::Paused(true) => triib.device_logs.paused = true,
        Ask::Paused(false) => triib.device_logs.resume(),
        Ask::Cleared => {
            if let Some(shown) = triib.device_logs.shown
                && let Some(lines) = triib.device_logs.logs.get_mut(&shown)
            {
                lines.clear();
            }
            triib.device_logs.held.clear();
        }
        Ask::Copy => {
            if let Some(text) = shown_text(triib) {
                return iced::clipboard::write(text);
            }
        }
        Ask::Save => {
            let Some(entity) = triib.device_logs.shown else {
                return Task::none();
            };
            let name = triib.entity_name_of(entity);
            let title = fl!("device-log-save-title", entity = name.as_str());
            let file = format!("{}.log", file_stem(&name, entity));
            return Task::perform(crate::dialog::choose_save_file(title, file), |result| {
                Message::DeviceLog(Ask::SaveTo(result))
            });
        }
        Ask::SaveTo(Ok(Some(path))) => {
            if let Some(text) = shown_text(triib) {
                triib.notice = Some(match std::fs::write(&path, text) {
                    Ok(()) => fl!("device-log-saved", path = path.display().to_string()),
                    Err(error) => fl!(
                        "device-log-save-failed",
                        path = path.display().to_string(),
                        error = error.to_string()
                    ),
                });
            }
        }
        Ask::SaveTo(Ok(None)) | Ask::CrashTo(_, Ok(None)) => {}
        Ask::SaveTo(Err(error)) | Ask::CrashTo(_, Err(error)) => {
            triib.notice = Some(fl!("app-file-dialog-failed", error = error));
        }
        Ask::SaveCrash(entity) => {
            let name = triib.entity_name_of(entity);
            let title = fl!("device-log-crash-save-title", entity = name.as_str());
            let file = crash::file_name(entity);
            return Task::perform(
                crate::dialog::choose_save_file(title, file),
                move |result| Message::DeviceLog(Ask::CrashTo(entity, result)),
            );
        }
        Ask::CrashTo(entity, Ok(Some(path))) => {
            if let Err(why) = read_crash(triib, entity, path) {
                triib.notice = Some(why);
            }
        }
        Ask::EraseCrash(entity) => triib.device_logs.erase_asked = Some(entity),
        Ask::EraseCanceled => triib.device_logs.erase_asked = None,
        Ask::EraseConfirmed => {
            if let Some(entity) = triib.device_logs.erase_asked.take()
                && let Some(network) = triib.network()
            {
                network.crash_dump(entity, true);
                triib.device_logs.crash.insert(entity, Crash::Erasing);
            }
        }
    }
    Task::none()
}

/// For trying the log in a development build: with
/// `TRIIB_TEST_DEVICE_LOG` naming an entity, its log shows once it is read.
#[cfg(debug_assertions)]
pub fn show_for_test(triib: &mut Triib, entity: EntityId) {
    let Ok(wanted) = std::env::var("TRIIB_TEST_DEVICE_LOG") else {
        return;
    };
    let Some(model) = triib.models.get(&entity) else {
        return;
    };
    let named = model.entity_name() == Some(wanted.as_str());
    if named && keeps(model).0 && triib.device_logs.shown.is_none() {
        // Showing a log starts no task.
        let _ = update(triib, Ask::Show(entity));
    }
}

/// Starts reading an entity's crash dump into `path`, unless one is being
/// read.
pub fn read_crash(triib: &mut Triib, entity: EntityId, path: PathBuf) -> Result<(), String> {
    let busy = triib
        .device_logs
        .crash
        .values()
        .any(|crash| matches!(crash, Crash::Reading { .. } | Crash::Erasing));
    if busy {
        return Err(fl!("device-log-crash-busy"));
    }
    let Some(network) = triib.network() else {
        return Err(fl!("device-log-network-stopped"));
    };
    network.crash_dump(entity, false);
    triib.device_logs.crash_to.insert(entity, path);
    triib
        .device_logs
        .crash
        .insert(entity, Crash::Reading { done: 0, total: 0 });
    Ok(())
}

/// How reading or erasing an entity's crash dump went, from the network.
pub fn reported(triib: &mut Triib, entity: EntityId, report: CrashReport) {
    let logs = &mut triib.device_logs;
    let crash = match report {
        CrashReport::Reading { done, total } => Crash::Reading { done, total },
        CrashReport::Read(octets) => match logs.crash_to.remove(&entity) {
            Some(path) => match std::fs::write(&path, &octets) {
                Ok(()) => Crash::Saved {
                    path,
                    octets: octets.len(),
                },
                Err(error) => Crash::Failed(fl!(
                    "device-log-save-failed",
                    path = path.display().to_string(),
                    error = error.to_string()
                )),
            },
            None => return,
        },
        CrashReport::Empty => {
            logs.crash_to.remove(&entity);
            Crash::Empty
        }
        CrashReport::NoDump => {
            logs.crash_to.remove(&entity);
            Crash::Failed(fl!("device-log-crash-none"))
        }
        CrashReport::Erased => Crash::Erased,
        CrashReport::Failed(why) => {
            logs.crash_to.remove(&entity);
            Crash::Failed(why)
        }
    };
    let over = !matches!(crash, Crash::Reading { .. });
    logs.crash.insert(entity, crash);
    if over {
        crate::tools::crash_finished(triib, entity);
    }
}

/// The log shown, as text.
fn shown_text(triib: &Triib) -> Option<String> {
    let entity = triib.device_logs.shown?;
    let name = triib.entity_name_of(entity);
    let mut text = String::new();
    for line in triib.device_logs.lines(entity) {
        text.push_str(&line.text(&name));
        text.push('\n');
    }
    Some(text)
}

/// A file name's stem from an entity's name, else its ID.
fn file_stem(name: &str, entity: EntityId) -> String {
    let stem: String = name
        .chars()
        .map(|character| {
            if character.is_alphanumeric() || matches!(character, '-' | '_' | '.') {
                character
            } else {
                '-'
            }
        })
        .collect();
    if stem.trim_matches('-').is_empty() {
        format!("{:016x}", entity.0)
    } else {
        stem
    }
}

/// A boot as the panel tells it.
fn boot_text(header: &LogHeader) -> String {
    let up = header.ms / 1000;
    let mut text = fl!(
        "device-log-boot",
        boot = format!("{:08x}", header.boot),
        seconds = up
    );
    if let Some(reason) = header.reset_reason {
        let name = reset_reason_name(reason).unwrap_or("?");
        text.push_str(&fl!(
            "device-log-boot-reason",
            reason = format!("{name} ({reason})")
        ));
    }
    if let Some(resets) = header.resets {
        text.push_str(&fl!("device-log-boot-resets", count = resets));
    }
    text
}

/// The inspector's section of the entity's log and crash dump: a button
/// to show the log, and one to save the crash dump and one to erase it,
/// with how saving or erasing goes.
pub fn entity_section<'a>(
    triib: &'a Triib,
    entity: EntityId,
    model: &'a EntityModel,
) -> Vec<Element<'a, Message>> {
    let mut items = Vec::new();
    let (log, dump) = keeps(model);
    if !log && dump.is_none() {
        return items;
    }
    items.push(component::section(fl!("device-log-section")));
    let running = triib.network().is_some();
    if log {
        let shown = triib.device_logs.shown == Some(entity);
        // Followed in the panel under the view while it shows there.
        let state = if shown {
            fl!("device-log-following")
        } else {
            String::new()
        };
        let show = component::tip(
            button::icon_button(Icon::Toc)
                .size(Size::ExtraSmall)
                .selected(shown)
                .on_press_maybe(
                    (running && !shown).then_some(Message::DeviceLog(Ask::Show(entity))),
                ),
            fl!("device-log-show"),
        );
        items.push(crate::view::labeled(
            fl!("device-log-log"),
            row![
                styled(state, Type::BodyMedium)
                    .style(style::on_surface_variant)
                    .width(Fill)
                    .align_x(dir::text_start()),
                show,
            ]
            .spacing(4)
            .align_y(Center),
        ));
    }
    if let Some(dump) = dump {
        let crash = triib.device_logs.crash.get(&entity);
        let busy = matches!(crash, Some(Crash::Reading { .. } | Crash::Erasing));
        let kept = match crash {
            Some(Crash::Empty | Crash::Erased) => false,
            _ => dump.length > 0,
        };
        let status = match crash {
            Some(Crash::Reading { done, total }) if *total > 0 => {
                let (done, total) = (*done as u64, *total as u64);
                fl!("device-log-crash-reading", done = done, total = total)
            }
            Some(Crash::Reading { .. }) => fl!("device-log-crash-starting"),
            Some(Crash::Saved { path, octets }) => {
                let size = *octets as u64;
                fl!(
                    "device-log-crash-saved",
                    size = size,
                    path = path.display().to_string()
                )
            }
            Some(Crash::Empty) => fl!("device-log-crash-empty"),
            Some(Crash::Erasing) => fl!("device-log-crash-erasing"),
            Some(Crash::Erased) => fl!("device-log-crash-erased"),
            Some(Crash::Failed(why)) => why.clone(),
            None if dump.length > 0 => fl!("device-log-crash-kept", size = dump.length),
            None => fl!("device-log-crash-empty"),
        };
        let failed = matches!(crash, Some(Crash::Failed(_)));
        let status = styled(status, Type::BodyMedium)
            .width(Fill)
            .align_x(dir::text_start());
        let status = if failed {
            status.style(style::error_text)
        } else {
            status.style(style::on_surface_variant)
        };
        let save = component::tip(
            button::icon_button(Icon::FileExport)
                .size(Size::ExtraSmall)
                .on_press_maybe(
                    (running && !busy).then_some(Message::DeviceLog(Ask::SaveCrash(entity))),
                ),
            fl!("device-log-crash-save"),
        );
        let erase = component::tip(
            button::icon_button(Icon::Delete)
                .size(Size::ExtraSmall)
                .on_press_maybe(
                    (running && !busy && kept)
                        .then_some(Message::DeviceLog(Ask::EraseCrash(entity))),
                ),
            fl!("device-log-crash-erase"),
        );
        // Under its label, where a file's path has room.
        items.push(crate::view::stacked(
            fl!("device-log-crash"),
            row![status, save, erase].spacing(4).align_y(Center),
        ));
    }
    items
}

/// Asks to confirm erasing a crash dump.
pub fn dialog(triib: &Triib) -> Option<Element<'_, Message>> {
    let entity = triib.device_logs.erase_asked?;
    Some(component::dialog(
        space().width(Fill).height(Fill),
        Some(Icon::Delete),
        fl!("device-log-erase-title"),
        fl!(
            "device-log-erase-body",
            entity = triib.entity_name_of(entity)
        ),
        vec![
            button::button(Kind::Text, fl!("device-log-erase-cancel"))
                .on_press(Message::DeviceLog(Ask::EraseCanceled))
                .into(),
            button::button(Kind::Filled, fl!("device-log-erase-confirm"))
                .on_press(Message::DeviceLog(Ask::EraseConfirmed))
                .into(),
        ],
    ))
}

/// The panel under the view with the log of the entity shown, in the
/// order written, the newest at the bottom.
pub fn panel(triib: &Triib, width: f32, height: f32, most: f32) -> Element<'_, Message> {
    let logs = &triib.device_logs;
    let Some(entity) = logs.shown else {
        return space().into();
    };
    let name = triib.entity_name_of(entity);
    let lines: Vec<&Line> = logs.lines(entity).collect();
    let wide = width >= LABELS_FIT;
    let title = styled(
        fl!("device-log-title", entity = name.as_str()),
        Type::TitleLarge,
    )
    .wrapping(iced::widget::text::Wrapping::None);
    let away = !triib.entities.contains_key(&entity);
    let state = if away {
        fl!("device-log-away")
    } else if logs.paused {
        fl!("device-log-paused")
    } else {
        fl!("device-log-following")
    };
    let state = styled(state, Type::BodyMedium)
        .style(style::on_surface_variant)
        .wrapping(iced::widget::text::Wrapping::None);
    let pause_message = Message::DeviceLog(Ask::Paused(!logs.paused));
    let (pause_icon, pause_label) = if logs.paused {
        (Icon::ProgressActivity, fl!("log-resume"))
    } else {
        (Icon::Stop, fl!("log-pause"))
    };
    let some = !lines.is_empty();
    let action = |icon, label, message| panel_action(wide, icon, label, message);
    let pause: Element<'_, Message> = if wide {
        let pause = if logs.paused {
            button::with_icon(Kind::Tonal, pause_icon, pause_label)
        } else {
            button::with_icon(Kind::Filled, pause_icon, pause_label).selected(false)
        };
        pause.size(Size::ExtraSmall).on_press(pause_message).into()
    } else {
        component::tip(
            button::icon_button(pause_icon)
                .selected(logs.paused)
                .on_press(pause_message),
            pause_label,
        )
    };
    let actions = row![
        pause,
        action(
            Icon::ContentCopy,
            fl!("device-log-copy"),
            some.then_some(Message::DeviceLog(Ask::Copy))
        ),
        action(
            Icon::FileExport,
            fl!("device-log-save"),
            some.then_some(Message::DeviceLog(Ask::Save))
        ),
        action(
            Icon::Delete,
            fl!("log-clear"),
            some.then_some(Message::DeviceLog(Ask::Cleared))
        ),
        component::tip(
            button::icon_button(Icon::Close).on_press(Message::DeviceLog(Ask::Close)),
            scramble_ui::labels::get().close,
        ),
    ]
    .spacing(8)
    .align_y(Center);
    let header = row![title, state, space::horizontal(), actions]
        .spacing(16)
        .align_y(Center);
    let body: Element<'_, Message> = if lines.is_empty() {
        container(
            styled(fl!("device-log-empty"), Type::BodyMedium)
                .style(style::on_surface_variant)
                .center(),
        )
        .center(Fill)
        .into()
    } else {
        let skip = lines.len().saturating_sub(SHOWN);
        let mut list = column![].spacing(1);
        for line in lines[skip..].iter().copied() {
            list = list.push(line_view(line));
        }
        component::scroll(list.width(Fill).padding([0, 4]))
            .anchor_bottom()
            .width(Fill)
            .height(Fill)
            .into()
    };
    let handle = resize::handle(move |drag| Message::LogResized(drag, height, most)).height();
    container(column![
        handle,
        container(column![header, body].spacing(4))
            .padding(dir::padding(0.0, 12.0, 8.0, 24.0))
            .height(Fill),
    ])
    .width(Fill)
    .height(height)
    .style(style::chrome)
    .into()
}

/// One of the panel's buttons, with its label where the panel is wide
/// enough, else with the label as its tip.
fn panel_action<'a>(
    wide: bool,
    icon: Icon,
    label: String,
    message: Option<Message>,
) -> Element<'a, Message> {
    if wide {
        button::with_icon(Kind::Text, icon, label)
            .size(Size::ExtraSmall)
            .on_press_maybe(message)
            .into()
    } else {
        component::tip(button::icon_button(icon).on_press_maybe(message), label)
    }
}

/// A line of a device's log: what it wrote in a fixed width font, an
/// error's in the error color and a debug or verbose line's muted, and
/// what happened between lines in the body font.
fn line_view(line: &Line) -> Element<'_, Message> {
    fn mark<'a>(text: String) -> Element<'a, Message> {
        styled(text, Type::BodySmall)
            .style(style::primary_text)
            .into()
    }
    match line {
        Line::Text(text) => {
            let shown = styled(text.as_str(), Type::BodySmall).font(Font::MONOSPACE);
            match text.as_bytes() {
                [b'E', b' ', b'(', ..] => shown.style(style::error_text).into(),
                [b'D' | b'V', b' ', b'(', ..] => shown.style(style::on_surface_variant).into(),
                _ => shown.into(),
            }
        }
        Line::Boot(header) => mark(boot_text(header)),
        Line::Restarted => mark(fl!("device-log-restarted")),
        Line::Lost(octets) => {
            let count = *octets;
            mark(fl!("device-log-lost", count = count))
        }
        Line::Dropped(lines) => {
            let count = *lines;
            mark(fl!("device-log-dropped", count = count))
        }
        Line::Away => mark(fl!("device-log-went-away")),
        Line::Back => mark(fl!("device-log-came-back")),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// An ESP board's log from its boot on, as pictures show it.
    pub(crate) fn board_lines() -> Vec<Entry> {
        let boot = LogHeader {
            boot: 0x812c_21f8,
            ms: 181_400,
            ring: 16_384,
            reset_reason: Some(4),
            resets: Some(3),
            ..LogHeader::default()
        };
        let mut entries = vec![Entry::Boot(boot)];
        entries.extend(
            [
                "I (31) boot: ESP-IDF v5.4.1 2nd stage bootloader",
                "I (38) boot: chip revision: v0.2",
                "I (612) cpu_start: Pro cpu start user code",
                "I (640) app_init: Application information:",
                "I (642) app_init: Project name:     avb_endpoint",
                "I (648) app_init: App version:      1.5.0-0804884a9",
                "I (905) esp_eth: link up, 100 Mbps, full duplex",
                "I (1203) gptp: grandmaster 0001f2fffeff3b14, as capable",
                "I (1854) avb: talker 0 bound, stream 0x9c6b00309a2b0000",
                "W (90311) avb: listener 0: 3 frames late",
                "I (120544) avb: listener 0 bound to Mac mini, output 0",
                "E (181022) i2s: DMA underrun on channel 1",
                "I (181040) i2s: restarted channel 1",
            ]
            .map(|line| Entry::Line(line.to_owned())),
        );
        entries
    }

    #[test]
    fn lines_that_come_while_paused_wait() {
        let entity = EntityId(7);
        let mut logs = DeviceLogs {
            shown: Some(entity),
            ..DeviceLogs::default()
        };
        logs.read(entity, vec![Entry::Line("one".into())]);
        logs.paused = true;
        logs.read(entity, vec![Entry::Line("two".into()), Entry::Restarted]);
        assert_eq!(logs.lines(entity).count(), 1);
        logs.resume();
        let lines: Vec<&Line> = logs.lines(entity).collect();
        assert_eq!(
            lines,
            [
                &Line::Text("one".into()),
                &Line::Text("two".into()),
                &Line::Restarted
            ]
        );
    }

    #[test]
    fn a_log_keeps_its_newest_lines() {
        let entity = EntityId(7);
        let mut logs = DeviceLogs::default();
        let entries = (0..LIMIT + 5)
            .map(|line| Entry::Line(line.to_string()))
            .collect();
        logs.read(entity, entries);
        assert_eq!(logs.lines(entity).count(), LIMIT);
        assert_eq!(logs.lines(entity).next(), Some(&Line::Text("5".into())));
    }

    #[test]
    fn going_away_is_marked_only_in_a_log_being_read() {
        let mut logs = DeviceLogs::default();
        logs.away(EntityId(1), true);
        assert!(!logs.has(EntityId(1)));
        logs.read(EntityId(2), vec![Entry::Line("x".into())]);
        logs.away(EntityId(2), true);
        logs.away(EntityId(2), false);
        assert_eq!(logs.lines(EntityId(2)).count(), 3);
    }

    #[test]
    fn a_file_is_named_after_its_entity() {
        assert_eq!(file_stem("esp-mic", EntityId(1)), "esp-mic");
        assert_eq!(file_stem("Room / A", EntityId(1)), "Room---A");
        assert_eq!(file_stem("", EntityId(0xab)), "00000000000000ab");
    }
}
