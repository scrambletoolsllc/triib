//! The application: its state, messages and how they change it.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use atdecc::controller::Outcome;
use atdecc::descriptor::DescriptorType;
use atdecc::model::EntityModel;
use atdecc::{DiscoveredEntity, EntityId};
use avb_net::Interface;
use iced::{Color, Element, Subscription, Task, Theme};
use scramble_ui::appearance::{self, Appearance, Inputs};
use scramble_ui::{desktop, motion, omarchy, resize, scheme};

use crate::External;
use crate::controls_view::ControlDrag;
use crate::entity_table::Column;
use crate::log_view::{Log, LogMessage};
use crate::matrix::{FormatMenu, Hover, Side};
use crate::netmap::{Focus, NodeKey};
use crate::network::{Action, Failure, Name, NameTarget, Network, Peer, Report, ReportKind, Watch};
use crate::presets::Preset;
use crate::settings::{EntityField, NetworkShows, Settings, Streams, View};
use crate::settings_view::SettingsTab;
use crate::view::InspectorTab;

/// The desktop app ID, also the Wayland app ID.
pub const APP_ID: &str = if triib_store::paths::PRODUCTION {
    "run.triib"
} else {
    "run.triib.dev"
};

/// How long a recalled preset waits for this computer's endpoints to
/// start again before it goes on without them.
const HOST_RESTART: Duration = Duration::from_secs(20);

/// triib's own accent, when the desktop gives none.
pub const TRIIB_SEED: Color = Color::from_rgb8(0x00, 0x89, 0x7b);

/// How often the Mac's audio device is looked for while its entity is
/// listed.
const MAC_AUDIO_CHECK: std::time::Duration = std::time::Duration::from_secs(2);

pub struct Triib {
    pub settings: Settings,
    theme: Theme,
    omarchy_dir: Option<PathBuf>,
    pub omarchy: Option<omarchy::Palette>,
    /// The desktop's accent color, when it has one.
    pub system_accent: Option<Color>,
    system_mode: iced::theme::Mode,
    /// The desktop allows animations.
    pub system_animations: bool,
    /// Whether the keyboard layout in use types right to left, when the
    /// system says.
    keyboard_rtl: Option<bool>,
    pub interfaces: Vec<Interface>,
    pub search: String,
    /// Shift is held, which turns asking every entity to announce itself
    /// into clearing and rescanning them all.
    pub shift: bool,
    network: Option<Network>,
    generation: u64,
    pub network_state: NetworkState,
    /// The entities online on the chosen interface.
    pub entities: BTreeMap<EntityId, DiscoveredEntity>,
    /// Controllers that do not advertise, heard in their commands.
    pub heard: BTreeMap<EntityId, crate::heard::Heard>,
    /// What has been read of each.
    pub models: BTreeMap<EntityId, EntityModel>,
    pub selected: Option<EntityId>,
    /// Actions sent to the network and not finished yet.
    pub pending: Vec<Action>,
    /// Steps that wait for one before them: the action on its way, and
    /// what follows it once it succeeds.
    pub chains: Vec<(Action, Vec<Action>)>,
    /// The formats a talker's output and a listener's input could share,
    /// offered from the matrix cell clicked.
    pub format_menu: Option<FormatMenu>,
    /// How far the entity list's rows are scrolled, sideways and down.
    pub entity_list_offset: (f32, f32),
    /// What happened to the last action that did not succeed.
    pub notice: Option<String>,
    /// What the pointer is over in the matrix.
    pub hover: Hover,
    /// The ports sending gPTP peer delay requests on this computer's link,
    /// which tell what it is plugged into.
    pub peers: Vec<Peer>,
    /// Why this computer cannot listen for the bridge's gPTP messages,
    /// when it cannot.
    pub cannot_listen: Option<String>,
    /// What triib-endpointd says this computer's entities are doing, while
    /// it runs.
    pub endpoints: Option<triib_endpointd::status::DaemonStatus>,
    /// Whether macOS runs AVB on the interface, giving the Mac an entity
    /// of its own; `None` off macOS.
    pub avb_enabled: Option<bool>,
    /// The Core Audio device macOS makes of the Mac's own entity, while
    /// one of its streams is connected, and when it was last looked for.
    pub mac_audio_device: Option<String>,
    /// Its output streams' channels, which the monitor can tap.
    pub mac_output_streams: Vec<u16>,
    /// The height of the system title bar over the window's content, which
    /// triib leaves room for in its toolbar's color: on macOS, where the
    /// bar is transparent; zero elsewhere and in full screen.
    pub title_bar: f32,
    /// The window's content view on macOS, which the title bar's height
    /// and double-clicks go through.
    #[cfg(target_os = "macos")]
    window_view: Option<usize>,
    mac_audio_checked: Option<std::time::Instant>,
    /// Entities that took another configuration and are being read again
    /// in it, with the one they took.
    pub reconfiguring: BTreeMap<EntityId, u16>,
    /// The listening helper while listening, and what it last said.
    monitor: Option<crate::monitor::Supervisor>,
    pub monitor_status: crate::monitor::Status,
    /// The interface another user's or root's triib-endpointd holds.
    pub endpoints_elsewhere: Option<String>,
    /// This computer's entities added this session that triib has yet to
    /// hear.
    pub starting: Vec<crate::host::Starting>,
    /// triib-endpointd was started this session for the host entities
    /// listed, as triib opened.
    /// The audio devices' names: inputs, then outputs.
    pub audio_devices: (Vec<String>, Vec<String>),
    /// How loud this computer's running streams are, for the Streams tab's
    /// meters.
    pub meter_levels: crate::meters::Levels,
    /// What the network view brings forward.
    pub network_focus: Option<Focus>,
    /// The toolbar's "More" menu is open.
    pub overflow_open: bool,
    /// The toolbar ends with a close button, as the window manager draws
    /// no title bar.
    pub close_button: bool,
    /// The menu of the interface picker's filters is open.
    pub interface_filters_open: bool,
    /// Each storage place as typed in Settings, not yet applied, and why
    /// the one given could not be used.
    pub storage_drafts: [String; 2],
    pub storage_errors: [Option<String>; 2],
    /// On a narrow window, the network view shows its list instead of the map.
    pub network_list: bool,
    /// The name being edited in the inspector, and its text so far.
    pub editing: Option<(NameTarget, String)>,
    /// The descriptor types opened in the inspector's tree, by entity.
    pub tree_open: BTreeSet<(EntityId, DescriptorType)>,
    /// The control slider being dragged.
    pub control_drag: Option<ControlDrag>,
    /// The inspector's open tab.
    pub inspector_tab: InspectorTab,
    /// The ATDECC frames sent and heard.
    pub log: Log,
    /// The log panel's height and the most it could take when a drag of
    /// its edge started.
    log_resize: Option<(f32, f32)>,
    /// The Settings dialog is open, on this tab.
    pub settings_open: bool,
    /// The presets saved, the presets dialog is open, the name typed in
    /// it, and what the last save, recall or delete did.
    pub presets: Vec<Preset>,
    pub presets_open: bool,
    pub preset_name: String,
    pub preset_report: Option<String>,
    /// A preset recalled while this computer's endpoints it runs start
    /// again: the preset, from when (milliseconds since the Unix epoch),
    /// and how long to wait for them.
    pub(crate) pending_recall: Option<(Preset, u64, std::time::Instant)>,
    pub settings_tab: SettingsTab,
    /// Why the settings could not be read or saved, shown in Settings.
    pub settings_error: Option<String>,
    /// Agents asking to control triib, and the changes they wait on.
    pub agents: crate::agents::Agents,
    /// The stream the Streams tab shows: the entity, whether it is an
    /// input, and its index. The entity's first stream when it is another.
    pub stream_selected: Option<(EntityId, bool, u16)>,
    /// Whether the Streams tab's menu of streams a device adds is open.
    pub add_stream_open: bool,
    /// What the network thread was last told the window shows.
    watched: Watch,
    /// The entities' logos, decoded as they show.
    pub logos: crate::logos::Logos,
}

/// What the network thread for the chosen interface is doing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkState {
    /// No interface chosen.
    Idle,
    Starting,
    Running {
        controller: EntityId,
    },
    Failed(Failure),
}

#[derive(Debug, Clone)]
pub enum Message {
    InterfacePicked(String),
    ViewPicked(View),
    InspectorToggled,
    /// The window's content view on macOS, once it is open.
    #[cfg(target_os = "macos")]
    WindowView(Option<usize>),
    /// The window took another size, as it does entering or leaving full
    /// screen, where the title bar leaves or comes back.
    #[cfg(target_os = "macos")]
    WindowResized,
    /// A double-click on the strip under the transparent title bar.
    TitleBarDoubleClicked,
    /// Show the log panel under the view, or hide it.
    LogToggled,
    /// A drag of the log panel's top edge, with its height and the most
    /// it may take as shown.
    LogResized(resize::Drag, f32, f32),
    /// The interface picker leaves out interfaces of a kind, or puts them
    /// back.
    InterfaceFilterToggled(crate::settings::InterfaceFilter),
    /// Open or close the menu of the interface picker's filters.
    InterfaceFiltersMenu(bool),
    /// The Entities view lists only this computer's endpoints, or all.
    HostEntitiesOnlyToggled,
    /// Open macOS's Audio MIDI Setup, which turns AVB on and off.
    OpenAudioMidiSetup,
    /// Listen to an input through an output, or stop.
    Listen(crate::monitor::Listen),
    SearchChanged(String),
    EntitySelected(EntityId),
    /// A press on a view's empty space: nothing selected.
    SelectionCleared,
    /// Show an entity's alarms: select it and open its diagnostics.
    AlarmOpened(EntityId),
    /// Open one of the inspector's tabs.
    InspectorTab(InspectorTab),
    /// A stream picked in the Streams tab's list: the entity, whether it
    /// is an input, and its index.
    StreamSelected(EntityId, bool, u16),
    Log(LogMessage),
    /// A control's slider moved, not yet let go.
    ControlDragged(ControlDrag),
    /// The slider being dragged was let go: its value is sent.
    ControlReleased,
    Rediscover,
    /// Forgets every entity and the entity models kept between runs, and
    /// reads them all again.
    Rescan,
    /// Shift was pressed or let go, or the window lost the keyboard.
    ShiftHeld(bool),
    RetryNetwork,
    /// Raw Ethernet is tried again where a permission kept it out, as the
    /// command that grants it may have run since.
    PermissionCheck,
    Copy(String),
    /// Opens a link in the browser.
    OpenLink(String),
    /// Adds a device on this computer.
    HostAdded,
    /// Removes one of this computer's entities.
    HostRemoved(EntityId),
    /// One of this computer's entities changes one of its streams.
    HostStream(EntityId, crate::host::StreamChange),
    Act(Action),
    /// Several actions, sent in order, such as removing a channel's mapping
    /// before mapping it anew.
    ActInOrder(Vec<Action>),
    /// Several actions, each sent once the one before it succeeds, such as
    /// changing formats before connecting.
    ActInTurn(Vec<Action>),
    DismissNotice,
    MatrixStreams(Streams),
    /// Show another field in the entity list's column at this place, or
    /// remove the column when `None`.
    EntityColumn(usize, Option<EntityField>),
    /// Add a column showing this field at the entity list's right.
    EntityColumnAdded(EntityField),
    /// Move the entity list's column at this place right, or left when
    /// false.
    EntityColumnMoved(usize, bool),
    /// An entity list column dragged to this width.
    EntityColumnResized(Column, f32),
    /// A column's drag ended, so its width is kept.
    EntityColumnResizeEnded,
    /// Fit a column to its text again.
    EntityColumnWidthReset(Column),
    MatrixConnectableOnlyToggled,
    MatrixCollapseToggled(Side, EntityId),
    MatrixHovered(Hover),
    /// A part of the entity list scrolled, which the others follow.
    EntityListScrolled(
        crate::entity_table::ListPart,
        iced::widget::scrollable::Viewport,
    ),
    /// Open or close the menu of streams a device adds.
    AddStreamMenu(bool),
    /// Offer the formats both ends of a matrix cell take.
    MatrixFormatsOpened(FormatMenu),
    MatrixFormatsClosed,
    NetworkShows(NetworkShows),
    OverflowToggled,
    NetworkListToggled,
    OverflowClosed,
    /// Close the window, which quits triib.
    CloseWindow,
    NetworkFocused(Option<Focus>),
    /// Start editing a name in the inspector.
    EditName(NameTarget),
    EditChanged(String),
    EditSubmitted,
    EditCanceled,
    /// Open or close a descriptor type in the inspector's tree.
    TreeToggled(EntityId, DescriptorType),
    SettingsOpened,
    PresetsOpened,
    PresetsClosed,
    PresetNameChanged(String),
    /// Save the network as a preset under the name typed.
    PresetSaved,
    PresetRecalled(String),
    PresetDeleted(String),
    SettingsClosed,
    SettingsTab(SettingsTab),
    LanguageSelected(String),
    AppearanceSelected(Appearance),
    SystemAccentToggled(bool),
    /// An accent color picked in Settings, as "#RRGGBB".
    AccentPicked(String),
    AnimationsToggled(bool),
    SystemTheme(iced::theme::Mode),
    /// The keyboard layout's direction changed, or the system stopped
    /// saying.
    KeyboardDirection(Option<bool>),
    SystemAccent(Option<(u8, u8, u8)>),
    AnimationsEnabled(Option<bool>),
    External(External),
    /// A new reading of the meters, or none, and who measured it.
    Meters(
        crate::meters::Origin,
        Option<triib_endpointd::meters::Meters>,
    ),
    /// The user answered whether the agent asking may control triib.
    AgentAnswered(bool),
    /// The agent's prompt takes answers now.
    AgentPromptReady,
    OutsideControlToggled(bool),
    ForgetAgents,
    /// A storage place, as typed in Settings.
    StorageDraft(crate::storage::Storage, String),
    /// Keeps it where typed.
    StorageApply(crate::storage::Storage),
    /// Asks where to keep it, with the folder dialog.
    StorageChoose(crate::storage::Storage),
    /// The folder the dialog gave, none when cancelled.
    StorageChosen(crate::storage::Storage, Result<Option<PathBuf>, String>),
    Nothing,
}

/// Where a controller with no model of its own runs.
enum Computer<'a> {
    This,
    /// The computer an entity of this name runs on.
    Of(&'a str),
    Unknown,
}

impl Triib {
    /// The app with these entities and settings, discovering on `interface`
    /// without a network thread, for drawing it in tests.
    #[cfg(test)]
    pub fn sample(
        settings: Settings,
        interface: Interface,
        entities: BTreeMap<EntityId, DiscoveredEntity>,
        models: BTreeMap<EntityId, EntityModel>,
    ) -> Self {
        Self {
            settings,
            theme: Theme::Dark,
            omarchy_dir: None,
            omarchy: None,
            system_accent: None,
            system_mode: iced::theme::Mode::Dark,
            system_animations: true,
            keyboard_rtl: None,
            interfaces: vec![interface],
            search: String::new(),
            shift: false,
            network: None,
            generation: 0,
            network_state: NetworkState::Running {
                controller: EntityId(0x9c6b_00ff_fe30_9a2b),
            },
            entities,
            heard: BTreeMap::new(),
            models,
            selected: None,
            pending: Vec::new(),
            chains: Vec::new(),
            format_menu: None,
            entity_list_offset: (0.0, 0.0),
            notice: None,
            hover: Hover::default(),
            peers: Vec::new(),
            cannot_listen: None,
            endpoints: None,
            avb_enabled: None,
            mac_audio_device: None,
            mac_audio_checked: None,
            mac_output_streams: Vec::new(),
            title_bar: 0.0,
            #[cfg(target_os = "macos")]
            window_view: None,
            reconfiguring: BTreeMap::new(),
            monitor: None,
            monitor_status: crate::monitor::Status::off(),
            endpoints_elsewhere: None,
            starting: Vec::new(),
            audio_devices: (Vec::new(), Vec::new()),
            meter_levels: crate::meters::Levels::default(),
            network_focus: None,
            overflow_open: false,
            close_button: false,
            interface_filters_open: false,
            storage_drafts: Default::default(),
            storage_errors: Default::default(),
            network_list: false,
            editing: None,
            tree_open: BTreeSet::new(),
            control_drag: None,
            inspector_tab: InspectorTab::default(),
            log: Log::default(),
            log_resize: None,
            presets: crate::presets::load_all(),
            presets_open: false,
            preset_name: String::new(),
            preset_report: None,
            pending_recall: None,
            settings_open: false,
            settings_tab: SettingsTab::default(),
            settings_error: None,
            agents: crate::agents::Agents::default(),
            stream_selected: None,
            add_stream_open: false,
            watched: Watch::default(),
            logos: crate::logos::Logos::default(),
        }
    }

    pub fn boot(omarchy_dir: Option<PathBuf>) -> (Self, Task<Message>) {
        let (settings, settings_error) = Settings::load();
        triib_endpointd::config::move_endpoints_file();
        crate::host::set_entities_file(Some(settings.entities_file.clone()));
        crate::presets::set_folder(settings.presets_folder.clone());
        crate::i18n::set_language(settings.chosen_language());
        let mut triib = Self {
            settings,
            theme: Theme::Dark,
            omarchy_dir,
            omarchy: None,
            system_accent: None,
            system_mode: iced::theme::Mode::Dark,
            system_animations: true,
            keyboard_rtl: None,
            interfaces: avb_net::interfaces(),
            search: String::new(),
            shift: false,
            network: None,
            generation: 0,
            network_state: NetworkState::Idle,
            entities: BTreeMap::new(),
            heard: BTreeMap::new(),
            models: BTreeMap::new(),
            selected: None,
            pending: Vec::new(),
            chains: Vec::new(),
            format_menu: None,
            entity_list_offset: (0.0, 0.0),
            notice: None,
            hover: Hover::default(),
            peers: Vec::new(),
            cannot_listen: None,
            endpoints: None,
            avb_enabled: None,
            mac_audio_device: None,
            mac_audio_checked: None,
            mac_output_streams: Vec::new(),
            title_bar: 0.0,
            #[cfg(target_os = "macos")]
            window_view: None,
            reconfiguring: BTreeMap::new(),
            monitor: None,
            monitor_status: crate::monitor::Status::off(),
            endpoints_elsewhere: None,
            starting: Vec::new(),
            audio_devices: (Vec::new(), Vec::new()),
            meter_levels: crate::meters::Levels::default(),
            network_focus: None,
            overflow_open: false,
            close_button: !scramble_ui::desktop::draws_title_bars(),
            interface_filters_open: false,
            storage_drafts: Default::default(),
            storage_errors: Default::default(),
            network_list: false,
            editing: None,
            tree_open: BTreeSet::new(),
            control_drag: None,
            inspector_tab: InspectorTab::default(),
            log: Log::default(),
            log_resize: None,
            presets: crate::presets::load_all(),
            presets_open: false,
            preset_name: String::new(),
            preset_report: None,
            pending_recall: None,
            settings_open: false,
            settings_tab: SettingsTab::default(),
            settings_error,
            agents: crate::agents::Agents::default(),
            stream_selected: None,
            add_stream_open: false,
            watched: Watch::default(),
            logos: crate::logos::Logos::default(),
        };
        triib.reload_omarchy();
        triib.apply_motion();
        triib.apply_input_direction();
        #[cfg(target_os = "macos")]
        crate::menu_macos::install(&triib.settings);
        triib.start_network(false);
        triib.start_listening();
        let system = iced::system::theme().map(Message::SystemTheme);
        // The portal's own answer, in case it came after iced stopped
        // waiting for it.
        #[cfg(target_os = "linux")]
        let system = Task::batch([
            system,
            Task::perform(desktop::color_scheme(), |mode| match mode {
                Some(mode) => Message::SystemTheme(mode),
                None => Message::Nothing,
            }),
        ]);
        let tasks = Task::batch([
            system,
            Task::perform(desktop::animations_enabled(), Message::AnimationsEnabled),
            Task::perform(desktop::accent_color(), Message::SystemAccent),
        ]);
        // The window is open by the time this runs.
        #[cfg(target_os = "macos")]
        let tasks = Task::batch([
            tasks,
            iced::window::oldest()
                .and_then(|id| iced::window::run(id, crate::title_bar_macos::content_view))
                .map(Message::WindowView),
        ]);
        (triib, tasks)
    }

    pub fn title(&self) -> String {
        match self.interface() {
            Some(interface) => format!("triib, {}", interface.name),
            None => "triib".to_owned(),
        }
    }

    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    /// The chosen interface, when it is present.
    /// Whether the chosen interface can carry this computer's own talkers
    /// and listeners: wired, with a PTP hardware clock.
    pub fn hosts_endpoints(&self) -> bool {
        crate::host::SUPPORTED
            && self.interface().is_some_and(|interface| {
                interface.physical && !interface.wireless && interface.hardware_clock.is_some()
            })
    }

    /// Whether `entity_id` is the Mac's own AVB entity, which macOS runs:
    /// a talker or listener from the interface's own address, where
    /// macOS runs AVB.
    pub fn is_this_mac(&self, entity_id: EntityId) -> bool {
        self.avb_enabled.is_some()
            && self.entities.get(&entity_id).is_some_and(|entity| {
                self.interface()
                    .is_some_and(|interface| interface.mac == entity.mac)
                    && (crate::describe::is_talker(&entity.adp)
                        || crate::describe::is_listener(&entity.adp))
            })
    }

    /// The Mac's own AVB entity, while it is online.
    #[cfg(target_os = "macos")]
    pub fn this_mac(&self) -> Option<EntityId> {
        self.entities
            .keys()
            .copied()
            .find(|&entity_id| self.is_this_mac(entity_id))
    }

    /// Listens as `listen` says, or stops, keeping it in the settings.
    pub fn set_listen(&mut self, listen: crate::monitor::Listen) {
        let listen = listen.bounded();
        self.settings.listen = listen.clone();
        self.save_settings();
        self.start_listening();
    }

    /// Starts the listening helper the settings ask for, stopping one
    /// running before.
    fn start_listening(&mut self) {
        self.monitor = None;
        let listen = self.settings.listen.clone();
        self.monitor_status = if listen.enabled {
            crate::monitor::Status {
                state: crate::monitor::State::Starting,
                peak: 0.0,
            }
        } else {
            crate::monitor::Status::off()
        };
        if listen.enabled && !cfg!(test) {
            self.monitor = Some(crate::monitor::Supervisor::start(listen, |status| {
                crate::post(External::Monitor(status));
            }));
        }
    }

    /// Asks macOS again whether it runs AVB on the interface.
    fn refresh_avb(&mut self) {
        self.avb_enabled = self
            .settings
            .interface
            .as_deref()
            .and_then(atdecc::blocking::avb_enabled);
    }

    /// Looks up the Core Audio device macOS made of the Mac's own entity.
    fn refresh_mac_audio_device(&mut self, entity_id: EntityId) {
        self.mac_audio_checked = Some(std::time::Instant::now());
        #[cfg(target_os = "macos")]
        {
            self.mac_audio_device = crate::mac::audio_device_of(entity_id);
            self.mac_output_streams = self
                .mac_audio_device
                .as_deref()
                .map(crate::mac::output_streams)
                .unwrap_or_default();
        }
        #[cfg(not(target_os = "macos"))]
        let _ = entity_id;
    }

    /// Whether `entity_id` is one of this computer's entities.
    pub fn is_host(&self, entity_id: EntityId) -> bool {
        crate::host::status_of(self.endpoints.as_ref(), entity_id).is_some()
    }

    /// The icon of one of this computer's entities, the computer's; `None`
    /// for others' entities.
    pub fn host_icon(&self, entity_id: EntityId) -> Option<scramble_ui::icon::Icon> {
        // Until the daemon's status names it, as it does a moment after the
        // entity advertises: one of its models, from this computer's
        // address.
        let ours = self.is_host(entity_id)
            || self.entities.get(&entity_id).is_some_and(|entity| {
                self.interface()
                    .is_some_and(|interface| interface.mac == entity.mac)
                    && triib_endpointd::runtime::is_device_model(entity.adp.entity_model_id)
            });
        (ours || self.is_this_mac(entity_id)).then_some(scramble_ui::icon::Icon::Computer)
    }

    /// Goes on with a preset recalled while this computer's endpoints
    /// start again, once they are back or have taken too long.
    fn go_on_recalling(&mut self) -> Task<Message> {
        let Some((preset, since, until)) = &self.pending_recall else {
            return Task::none();
        };
        if !crate::presets::host_ready(self, preset, *since) && std::time::Instant::now() < *until {
            return Task::none();
        }
        let preset = preset.clone();
        self.pending_recall = None;
        crate::host::RECALLING.store(false, std::sync::atomic::Ordering::Relaxed);
        self.recall_now(&preset)
    }

    /// Changes what differs from `preset` and says what it did.
    fn recall_now(&mut self, preset: &Preset) -> Task<Message> {
        let name = preset.name.as_str();
        let recall = crate::presets::recall(self, preset);
        let changes = recall.actions.len();
        let mut report = match changes {
            0 => crate::fl!("presets-nothing-differs", name = name),
            _ => crate::fl!("presets-recalling", name = name, count = changes),
        };
        if !recall.missing.is_empty() {
            report = crate::fl!(
                "presets-missing",
                report = report,
                missing = crate::i18n::list(recall.missing.clone())
            );
        }
        self.preset_report = Some(report);
        if changes > 0 {
            return self.update(Message::ActInOrder(recall.actions));
        }
        Task::none()
    }

    /// Shows each storage place in Settings as it is kept.
    fn reset_storage_drafts(&mut self) {
        for storage in crate::storage::Storage::ALL {
            self.storage_drafts[storage.index()] =
                crate::storage::shown_path(storage.path(&self.settings));
            self.storage_errors[storage.index()] = None;
        }
    }

    /// Keeps `storage` at `path` from now on, in the settings file too,
    /// once the place checks out; otherwise says why under its row. The
    /// entities file in use is copied to a place that has none, and the
    /// daemon starts again on it.
    fn set_storage(&mut self, storage: crate::storage::Storage, path: &std::path::Path) {
        use crate::storage::Storage;
        let path = storage.take(&self.settings, path);
        if let Err(problem) = crate::storage::check(storage, &path) {
            self.storage_errors[storage.index()] = Some(problem);
            return;
        }
        if path != storage.path(&self.settings) {
            match storage {
                Storage::Presets => {
                    crate::presets::set_folder(path.clone());
                    self.presets = crate::presets::load_all();
                    self.settings.presets_folder = path;
                }
                Storage::Entities => {
                    if let Err(problem) = crate::host::move_entities(Some(path.clone())) {
                        crate::host::set_entities_file(Some(self.settings.entities_file.clone()));
                        self.storage_errors[storage.index()] = Some(problem);
                        return;
                    }
                    self.settings.entities_file = path;
                }
            }
            self.save_settings();
        }
        self.storage_errors[storage.index()] = None;
        self.reset_storage_drafts();
    }

    fn add_host(&mut self) {
        let Some((interface, mac)) = self
            .interface()
            .map(|interface| (interface.name.clone(), interface.mac))
        else {
            return;
        };
        let count = self
            .endpoints
            .as_ref()
            .map_or(0, |status| status.endpoints.len())
            + 1;
        let name = crate::fl!("host-new-entity", number = count);
        let result = crate::host::add(&interface, name.clone()).and_then(|instance| {
            crate::host::ensure_running()?;
            Ok(instance)
        });
        match result {
            // Listed at once, starting, until triib hears it.
            Ok(instance) => self.starting.push(crate::host::Starting {
                interface,
                entity_id: triib_endpointd::runtime::entity_id(mac, instance),
                name,
            }),
            Err(error) => self.notice = Some(crate::fl!("host-failed", reason = error)),
        }
    }

    pub fn interface(&self) -> Option<&Interface> {
        let name = self.settings.interface.as_deref()?;
        self.interfaces
            .iter()
            .find(|interface| interface.name == name)
    }

    /// The interfaces the picker offers: all but the kinds its filters
    /// leave out, virtual, wireless or without a hardware clock, and the
    /// chosen one always.
    pub fn listed_interfaces(&self) -> impl Iterator<Item = &Interface> {
        use crate::settings::InterfaceFilter;
        let chosen = self.settings.interface.as_deref();
        let settings = &self.settings;
        self.interfaces.iter().filter(move |interface| {
            let left_out = (settings.hides(InterfaceFilter::Virtual) && !interface.physical)
                || (settings.hides(InterfaceFilter::Wireless) && interface.wireless)
                || (settings.hides(InterfaceFilter::Unclocked)
                    && interface.hardware_clock.is_none());
            !left_out || chosen == Some(interface.name.as_str())
        })
    }

    /// Runs `message`; the network thread then hears what the window
    /// shows, should that have changed.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let task = self.handle(message);
        // An entity starting is listed as such until it is heard, or the
        // list shows another interface.
        let interface = self.settings.interface.as_deref();
        let entities = &self.entities;
        self.starting.retain(|starting| {
            Some(starting.interface.as_str()) == interface
                && !entities.contains_key(&starting.entity_id)
        });
        // The entities of the interface shown, where it runs this
        // computer's, stay running while triib does.
        let kept = self
            .interface()
            .filter(|_| self.hosts_endpoints())
            .map(|interface| interface.name.as_str());
        crate::host::keep_running(kept);
        let shown = self.shown();
        if shown != self.watched {
            if let Some(network) = &self.network {
                network.watch(shown.clone());
            }
            self.watched = shown;
        }
        task
    }

    /// What the window shows that entities elsewhere can change
    /// unannounced: every entity not from this computer's address, whose
    /// entities send notifications, and the stream the inspector's Streams
    /// tab shows.
    pub(crate) fn shown(&self) -> Watch {
        let here = self.interface().map(|interface| interface.mac);
        let elsewhere = |entity_id: &EntityId| {
            self.entities
                .get(entity_id)
                .is_some_and(|entity| Some(entity.mac) != here)
        };
        let stream = self
            .selected
            .filter(|_| self.settings.inspector && self.inspector_tab == InspectorTab::Streams)
            .filter(elsewhere)
            .and_then(|entity_id| {
                let model = self.models.get(&entity_id)?;
                let streams: Vec<_> = model.streams(false).chain(model.streams(true)).collect();
                let stream = streams.get(crate::view::picked_stream(self, entity_id, &streams)?)?;
                Some((entity_id, stream.descriptor_type, stream.index))
            });
        Watch {
            entities: self.entities.keys().copied().filter(elsewhere).collect(),
            stream,
        }
    }

    fn handle(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InterfacePicked(name) => {
                if self.settings.interface.as_deref() != Some(name.as_str()) {
                    self.settings.interface = Some(name);
                    self.save_settings();
                    self.start_network(false);
                }
            }
            Message::ViewPicked(view) => {
                self.settings.view = view;
                self.save_settings();
            }
            Message::HostEntitiesOnlyToggled => {
                self.settings.host_entities_only = !self.settings.host_entities_only;
                self.save_settings();
            }
            Message::InterfaceFilterToggled(filter) => {
                self.settings.toggle_filter(filter);
                self.save_settings();
            }
            Message::InterfaceFiltersMenu(open) => self.interface_filters_open = open,
            Message::InspectorToggled => {
                self.settings.inspector = !self.settings.inspector;
                self.save_settings();
            }
            #[cfg(target_os = "macos")]
            Message::WindowView(view) => {
                self.window_view = view;
                self.title_bar = crate::title_bar_macos::height(view.unwrap_or(0));
            }
            #[cfg(target_os = "macos")]
            Message::WindowResized => {
                self.title_bar = crate::title_bar_macos::height(self.window_view.unwrap_or(0));
            }
            Message::TitleBarDoubleClicked =>
            {
                #[cfg(target_os = "macos")]
                if let Some(view) = self.window_view {
                    crate::title_bar_macos::double_clicked(view);
                }
            }
            Message::LogToggled => {
                self.settings.log = !self.settings.log;
                self.save_settings();
            }
            Message::LogResized(drag, shown, most) => match drag {
                resize::Drag::Started => self.log_resize = Some((shown, most)),
                resize::Drag::Moved(moved) => {
                    if let Some((start, most)) = self.log_resize {
                        // The edge is the panel's top, so dragging up
                        // makes it taller.
                        self.settings.log_height =
                            (start - moved).clamp(crate::log_view::LEAST_HEIGHT, most);
                    }
                }
                resize::Drag::Ended => {
                    if self.log_resize.take().is_some() {
                        self.save_settings();
                    }
                }
            },
            Message::SearchChanged(search) => self.search = search,
            Message::EntitySelected(entity_id) => {
                if self
                    .editing
                    .as_ref()
                    .is_some_and(|(target, _)| target.entity != entity_id)
                {
                    self.editing = None;
                }
                self.selected = Some(entity_id);
                if !self.settings.inspector {
                    self.settings.inspector = true;
                    self.save_settings();
                }
            }
            Message::InspectorTab(tab) => self.inspector_tab = tab,
            Message::StreamSelected(entity, input, index) => {
                self.stream_selected = Some((entity, input, index));
            }
            Message::Log(message) => self.log.update(message),
            Message::ControlDragged(drag) => self.control_drag = Some(drag),
            Message::ControlReleased => {
                if let Some(drag) = self.control_drag.take()
                    && let Some(message) = crate::controls_view::released(self, drag)
                {
                    return self.update(message);
                }
            }
            Message::AlarmOpened(entity_id) => {
                self.selected = Some(entity_id);
                self.inspector_tab = InspectorTab::Diagnostics;
                if !self.settings.inspector {
                    self.settings.inspector = true;
                    self.save_settings();
                }
            }
            Message::SelectionCleared => {
                self.selected = None;
                self.editing = None;
                self.network_focus = None;
            }
            Message::Rediscover => {
                if let Some(network) = &self.network {
                    network.discover();
                }
            }
            Message::Rescan => {
                self.shift = false;
                self.start_network(true);
            }
            Message::ShiftHeld(held) => self.shift = held,
            Message::RetryNetwork => self.start_network(false),
            Message::PermissionCheck => self.try_network_quietly(),
            Message::Act(action) => {
                if let Some(network) = &self.network {
                    self.notice = None;
                    self.pending.push(action);
                    network.act(action);
                }
            }
            Message::ActInOrder(actions) => {
                if let Some(network) = &self.network {
                    self.notice = None;
                    for action in actions {
                        self.pending.push(action);
                        network.act(action);
                    }
                }
            }
            Message::ActInTurn(actions) => {
                self.notice = None;
                self.act_in_turn(actions);
            }
            Message::DismissNotice => self.notice = None,
            Message::EntityColumn(index, field) => {
                let columns = &mut self.settings.entity_columns;
                if index < columns.len() {
                    match field {
                        Some(field) if !columns.contains(&field) => columns[index] = field,
                        Some(_) => {}
                        None => {
                            columns.remove(index);
                        }
                    }
                    self.save_settings();
                }
            }
            Message::EntityColumnMoved(index, right) => {
                let columns = &mut self.settings.entity_columns;
                let other = if right {
                    index.checked_add(1)
                } else {
                    index.checked_sub(1)
                };
                if let Some(other) = other.filter(|&other| other < columns.len())
                    && index < columns.len()
                {
                    columns.swap(index, other);
                    self.save_settings();
                }
            }
            Message::EntityColumnResized(column, width) => match column {
                Column::Name => self.settings.entity_name_width = Some(width),
                Column::Field(field) => {
                    self.settings.entity_column_widths.insert(field, width);
                }
            },
            Message::EntityColumnResizeEnded => self.save_settings(),
            Message::EntityColumnWidthReset(column) => {
                match column {
                    Column::Name => self.settings.entity_name_width = None,
                    Column::Field(field) => {
                        self.settings.entity_column_widths.remove(&field);
                    }
                }
                self.save_settings();
            }
            Message::EntityColumnAdded(field) => {
                if !self.settings.entity_columns.contains(&field) {
                    self.settings.entity_columns.push(field);
                    self.save_settings();
                }
            }
            Message::MatrixStreams(streams) => {
                self.settings.matrix_streams = streams;
                self.save_settings();
            }
            Message::MatrixConnectableOnlyToggled => {
                self.settings.matrix_connectable_only = !self.settings.matrix_connectable_only;
                self.save_settings();
            }
            Message::MatrixCollapseToggled(side, entity_id) => {
                self.settings.matrix_collapsed.toggle(side, entity_id);
                self.save_settings();
            }
            Message::MatrixHovered(hover) => self.hover = hover,
            Message::EntityListScrolled(part, viewport) => {
                return self.entity_list_scrolled(part, viewport);
            }
            Message::MatrixFormatsOpened(menu) => self.format_menu = Some(menu),
            Message::MatrixFormatsClosed => self.format_menu = None,
            Message::AddStreamMenu(open) => self.add_stream_open = open,
            Message::NetworkShows(shows) => {
                self.settings.network_shows = shows;
                self.save_settings();
                self.network_focus = None;
            }
            Message::NetworkFocused(focus) => {
                // A device brought forward is the one the inspector shows.
                if let Some(Focus::Node(NodeKey::Entity(entity_id))) = focus {
                    self.selected = Some(entity_id);
                }
                self.network_focus = focus;
            }
            Message::EditName(target) => {
                self.editing = Some((target, self.current_name(target)));
                let field = iced::widget::Id::new(crate::view::NAME_FIELD);
                return Task::batch([
                    iced::widget::operation::focus(field.clone()),
                    iced::widget::operation::select_all(field),
                ]);
            }
            Message::EditChanged(text) => {
                // Names longer than an entity holds are not taken.
                if let Some((_, editing)) = &mut self.editing
                    && Name::new(&text).is_some()
                {
                    *editing = text;
                }
            }
            Message::EditSubmitted => {
                if let Some((target, text)) = self.editing.take()
                    && text != self.current_name(target)
                    && let Some(name) = Name::new(&text)
                {
                    return self.update(Message::Act(Action::Rename { target, name }));
                }
            }
            Message::EditCanceled => self.editing = None,
            Message::PresetsOpened => {
                self.presets_open = true;
                self.preset_report = None;
            }
            Message::PresetsClosed => self.presets_open = false,
            Message::PresetNameChanged(name) => self.preset_name = name,
            Message::PresetSaved => {
                let name = self.preset_name.trim().to_owned();
                if name.is_empty() {
                    return Task::none();
                }
                let mut preset = crate::presets::capture(self, &name);
                if self.hosts_endpoints()
                    && let Some(interface) = self.interface()
                {
                    let endpoints = crate::host::endpoints(&interface.name);
                    preset.host = (!endpoints.is_empty()).then(|| crate::presets::HostPreset {
                        mac: interface.mac.to_string(),
                        endpoints,
                    });
                }
                let entities = preset.entities.len();
                self.preset_report = Some(match crate::presets::save(&preset) {
                    Ok(()) => {
                        self.presets.retain(|kept| kept.name != name);
                        self.presets.push(preset);
                        self.presets.sort_by_key(|kept| kept.name.to_lowercase());
                        self.preset_name.clear();
                        crate::fl!("presets-saved", name = name, count = entities)
                    }
                    Err(error) => error,
                });
            }
            Message::PresetRecalled(name) => {
                let Some(preset) = self.presets.iter().find(|preset| preset.name == name) else {
                    return Task::none();
                };
                let preset = match self.interface() {
                    Some(interface) => crate::presets::moved_to(preset, interface.mac),
                    None => preset.clone(),
                };
                // This computer's endpoints first: when they start again,
                // the rest waits for them.
                if let Some(host) = &preset.host
                    && self.hosts_endpoints()
                    && let Some(interface) = self.interface().map(|found| found.name.clone())
                {
                    let since = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .map_or(0, |since| since.as_millis() as u64);
                    match crate::host::restore(&interface, &host.endpoints) {
                        Ok(true) => {
                            self.preset_report =
                                Some(crate::fl!("presets-starting-host", name = name.as_str()));
                            let until = std::time::Instant::now() + HOST_RESTART;
                            self.pending_recall = Some((preset, since, until));
                            crate::host::RECALLING
                                .store(true, std::sync::atomic::Ordering::Relaxed);
                            return Task::none();
                        }
                        Ok(false) => {}
                        Err(error) => {
                            self.notice = Some(crate::fl!("host-failed", reason = error));
                        }
                    }
                }
                return self.recall_now(&preset);
            }
            Message::PresetDeleted(name) => {
                self.preset_report = Some(match crate::presets::delete(&name) {
                    Ok(()) => {
                        self.presets.retain(|preset| preset.name != name);
                        crate::fl!("presets-deleted", name = name.as_str())
                    }
                    Err(error) => error,
                });
            }
            Message::SettingsOpened => {
                self.settings_open = true;
                self.overflow_open = false;
                self.reset_storage_drafts();
            }
            Message::SettingsClosed => self.settings_open = false,
            Message::SettingsTab(tab) => self.settings_tab = tab,
            Message::LanguageSelected(tag) => {
                self.settings.language = tag;
                crate::i18n::set_language(self.settings.chosen_language());
                self.apply_input_direction();
                #[cfg(target_os = "macos")]
                crate::menu_macos::install(&self.settings);
                self.save_settings();
            }
            Message::KeyboardDirection(right_to_left) => {
                self.keyboard_rtl = right_to_left;
                self.apply_input_direction();
            }
            Message::AppearanceSelected(appearance) => {
                self.settings.appearance = appearance;
                self.save_settings();
                self.refresh_theme();
            }
            Message::SystemAccentToggled(enabled) => {
                self.settings.system_accent = enabled;
                self.save_settings();
                self.refresh_theme();
            }
            Message::AccentPicked(hex) => {
                self.settings.accent_color = Some(hex);
                self.save_settings();
                self.refresh_theme();
            }
            Message::AnimationsToggled(enabled) => {
                self.settings.animations = enabled;
                self.save_settings();
                self.apply_motion();
            }
            Message::TreeToggled(entity_id, descriptor_type) => {
                if !self.tree_open.remove(&(entity_id, descriptor_type)) {
                    self.tree_open.insert((entity_id, descriptor_type));
                }
            }
            Message::OverflowToggled => self.overflow_open = !self.overflow_open,
            Message::NetworkListToggled => self.network_list = !self.network_list,
            Message::OverflowClosed => self.overflow_open = false,
            Message::CloseWindow => {
                return iced::window::oldest().and_then(iced::window::close);
            }
            Message::Copy(text) => return iced::clipboard::write(text),
            Message::OpenLink(link) => {
                return Task::perform(crate::dialog::open_uri(link), |_| Message::Nothing);
            }
            Message::SystemTheme(mode) => {
                self.system_mode = mode;
                self.refresh_theme();
            }
            Message::SystemAccent(accent) => {
                self.system_accent =
                    accent.map(|(red, green, blue)| Color::from_rgb8(red, green, blue));
                self.refresh_theme();
            }
            Message::AnimationsEnabled(enabled) => {
                if let Some(enabled) = enabled {
                    self.system_animations = enabled;
                    self.apply_motion();
                }
            }
            Message::External(External::OmarchyThemeChanged) => self.reload_omarchy(),
            Message::External(External::Interfaces(interfaces)) => {
                self.interfaces = interfaces;
                self.refresh_avb();
            }
            Message::Listen(listen) => self.set_listen(listen),
            Message::External(External::Monitor(status)) => {
                if self.monitor.is_some() {
                    self.monitor_status = status;
                }
            }
            Message::OpenAudioMidiSetup => {
                let _ = std::process::Command::new("open")
                    .args(["-b", "com.apple.audio.AudioMIDISetup"])
                    .spawn();
            }
            Message::External(External::Endpoints(status)) => {
                self.endpoints = status;
                return self.go_on_recalling();
            }
            Message::External(External::EndpointsElsewhere(interface)) => {
                self.endpoints_elsewhere = interface;
            }
            Message::External(External::AudioDevices(inputs, outputs)) => {
                self.audio_devices = (inputs, outputs);
            }
            Message::Meters(origin, reading) => {
                #[cfg(target_os = "macos")]
                let reading = match origin {
                    crate::meters::Origin::Mac => {
                        reading.map(|meters| crate::meters::by_stream_index(self, meters))
                    }
                    crate::meters::Origin::Daemon => reading,
                };
                self.meter_levels
                    .take(origin, reading, std::time::Instant::now());
            }
            Message::HostAdded => self.add_host(),
            Message::HostRemoved(entity_id) => {
                let done = self
                    .interface()
                    .map(|interface| (interface.name.clone(), interface.mac));
                if let Some((name, mac)) = done
                    && let Err(error) = crate::host::remove(&name, mac, entity_id)
                {
                    self.notice = Some(error);
                }
            }
            Message::HostStream(entity_id, change) => {
                // A new stream is picked in the Streams tab, so its details
                // show once the device has it: the next index its way.
                if let crate::host::StreamChange::Add { direction, .. } = &change {
                    self.add_stream_open = false;
                    let input = *direction == triib_endpointd::config::Direction::In;
                    let next = self
                        .models
                        .get(&entity_id)
                        .map_or(0, |model| model.streams(input).count());
                    self.stream_selected = Some((entity_id, input, next as u16));
                }
                let done = self
                    .interface()
                    .map(|interface| (interface.name.clone(), interface.mac));
                if let Some((name, mac)) = done
                    && let Err(error) = crate::host::change_stream(&name, mac, entity_id, change)
                {
                    self.notice = Some(error);
                }
            }
            Message::External(External::Network(report)) => {
                self.network_report(report);
                return self.go_on_recalling();
            }
            Message::External(External::Control(call)) => return self.control(call),
            #[cfg(target_os = "macos")]
            Message::External(External::Menu(item)) => return self.menu_item(item),
            Message::AgentAnswered(allow) => return self.agent_answered(allow),
            Message::AgentPromptReady => {}
            Message::OutsideControlToggled(on) => {
                self.settings.outside_control = on;
                self.save_settings();
                if !on {
                    self.outside_control_off();
                }
            }
            Message::ForgetAgents => {
                self.settings.allowed_agents.clear();
                self.save_settings();
            }
            Message::StorageDraft(storage, text) => {
                self.storage_drafts[storage.index()] = text;
                self.storage_errors[storage.index()] = None;
            }
            Message::StorageApply(storage) => {
                let typed = PathBuf::from(self.storage_drafts[storage.index()].trim());
                self.set_storage(storage, &typed);
            }
            Message::StorageChoose(storage) => {
                return Task::perform(
                    crate::dialog::choose_folder(
                        storage.choose_title(),
                        storage.current_folder(&self.settings),
                    ),
                    move |result| Message::StorageChosen(storage, result),
                );
            }
            Message::StorageChosen(storage, Ok(Some(folder))) => {
                self.set_storage(storage, &folder);
            }
            Message::StorageChosen(_, Ok(None)) => {}
            Message::StorageChosen(_, Err(error)) => {
                self.settings_error = Some(crate::fl!("app-file-dialog-failed", error = error));
            }
            Message::Nothing => {}
        }
        Task::none()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // The meters, read while the Streams tab shows, where this
        // computer runs entities whose streams they measure, and on a Mac
        // where its own entity has its audio device.
        let streams_shown = self.settings.inspector
            && self.inspector_tab == InspectorTab::Streams
            && self.selected.is_some();
        let meters = if streams_shown && self.endpoints.is_some() {
            Subscription::run(crate::meters::readings)
                .map(|reading| Message::Meters(crate::meters::Origin::Daemon, reading))
        } else {
            Subscription::none()
        };
        #[cfg(target_os = "macos")]
        let mac_meters = match (streams_shown, self.this_mac(), &self.mac_audio_device) {
            (true, Some(entity_id), Some(device)) => {
                Subscription::run_with((device.clone(), entity_id), crate::mac_meters::readings)
                    .map(|reading| Message::Meters(crate::meters::Origin::Mac, reading))
            }
            _ => Subscription::none(),
        };
        #[cfg(not(target_os = "macos"))]
        let mac_meters = Subscription::none();
        // Raw Ethernet tried again now and then while it wants a permission
        // a command grants, so the network starts once it has run.
        let permission = match &self.network_state {
            NetworkState::Failed(Failure {
                remedy: Some(crate::network::Remedy::Command(_)),
                ..
            }) => Subscription::run(crate::network::permission_checks)
                .map(|()| Message::PermissionCheck),
            _ => Subscription::none(),
        };
        Subscription::batch([
            meters,
            mac_meters,
            permission,
            iced::system::theme_changes().map(Message::SystemTheme),
            Subscription::run(desktop::accent_changes).map(Message::SystemAccent),
            Subscription::run(crate::external_events).map(Message::External),
            Subscription::run(scramble_ui::input::keyboard_changes).map(Message::KeyboardDirection),
            iced::event::listen_with(|event, _, _| match event {
                iced::Event::Keyboard(iced::keyboard::Event::ModifiersChanged(modifiers)) => {
                    Some(Message::ShiftHeld(modifiers.shift()))
                }
                iced::Event::Window(iced::window::Event::Unfocused) => {
                    Some(Message::ShiftHeld(false))
                }
                #[cfg(target_os = "macos")]
                iced::Event::Window(iced::window::Event::Resized(_)) => {
                    Some(Message::WindowResized)
                }
                _ => None,
            }),
        ])
    }

    pub fn view(&self) -> Element<'_, Message> {
        crate::view::window(self)
    }

    /// Stops the network thread of the previous interface and starts one
    /// for the chosen interface, forgetting the entities seen before, and
    /// when `fresh` the entity models kept between runs too.
    fn start_network(&mut self, fresh: bool) {
        self.network = None;
        self.entities.clear();
        self.heard.clear();
        self.models.clear();
        self.selected = None;
        self.pending.clear();
        self.agent_actions_dropped();
        self.hover = Hover::default();
        self.peers.clear();
        self.cannot_listen = None;
        self.network_focus = None;
        self.refresh_avb();
        self.mac_audio_device = None;
        self.reconfiguring.clear();
        self.watched = Watch::default();
        let Some(interface) = self.settings.interface.clone() else {
            self.network_state = NetworkState::Idle;
            return;
        };
        self.generation += 1;
        self.network_state = NetworkState::Starting;
        self.network = Some(Network::start(interface, self.generation, fresh));
    }

    /// Tries the network again where it failed, unless a try is under way,
    /// still showing why it failed until it starts.
    fn try_network_quietly(&mut self) {
        if self.network.is_some() || !matches!(self.network_state, NetworkState::Failed(_)) {
            return;
        }
        let Some(interface) = self.settings.interface.clone() else {
            return;
        };
        self.generation += 1;
        self.network = Some(Network::start(interface, self.generation, false));
    }

    fn network_report(&mut self, report: Report) {
        if report.generation != self.generation {
            return;
        }
        match report.kind {
            ReportKind::Started { controller } => {
                self.network_state = NetworkState::Running { controller };
            }
            ReportKind::Failed(failure) => {
                self.network = None;
                self.network_state = NetworkState::Failed(failure);
                self.agent_actions_dropped();
            }
            ReportKind::Online(entity)
            | ReportKind::Changed(entity)
            | ReportKind::Restarted(entity) => {
                // A controller heard that now advertises is listed as such.
                self.heard.remove(&entity.entity_id());
                let entity_id = entity.entity_id();
                self.entities.insert(entity_id, entity);
                if self.is_this_mac(entity_id) {
                    self.refresh_mac_audio_device(entity_id);
                }
            }
            ReportKind::Offline(entity_id, _) => {
                self.entities.remove(&entity_id);
                self.models.remove(&entity_id);
            }
            ReportKind::Model(entity_id, model) => {
                // An entity read again, as one whose model changed, keeps
                // showing what was read before until the new read is done,
                // so the list and the inspector stay as they are.
                let rereading = model.state == atdecc::model::EnumerationState::Reading
                    && self.models.get(&entity_id).is_some_and(|known| {
                        known.state == atdecc::model::EnumerationState::Complete
                    });
                if !rereading {
                    if model.state != atdecc::model::EnumerationState::Reading {
                        self.reconfiguring.remove(&entity_id);
                    }
                    self.models.insert(entity_id, *model);
                }
            }
            ReportKind::Peers(peers) => self.peers = peers,
            ReportKind::CannotListen(reason) => self.cannot_listen = Some(reason),
            ReportKind::Frames(frames) => {
                // The Mac's audio device comes and goes a few seconds
                // after its streams connect and disconnect.
                if self
                    .mac_audio_checked
                    .is_none_or(|checked| checked.elapsed() >= MAC_AUDIO_CHECK)
                    && let Some(entity_id) = self
                        .entities
                        .keys()
                        .copied()
                        .find(|&entity_id| self.is_this_mac(entity_id))
                {
                    self.refresh_mac_audio_device(entity_id);
                }
                let own = match self.network_state {
                    NetworkState::Running { controller } => Some(controller),
                    _ => None,
                };
                let entities = &self.entities;
                crate::heard::note(&mut self.heard, &frames, |controller| {
                    own == Some(controller) || entities.contains_key(&controller)
                });
                if !self.log.paused {
                    let described = frames
                        .into_iter()
                        .map(|frame| {
                            let line = crate::log_view::describe(self, &frame);
                            (frame, line)
                        })
                        .collect();
                    self.log.add(described);
                }
            }
            ReportKind::Finished(action, outcome) => {
                if let Some(position) = self.pending.iter().position(|pending| *pending == action) {
                    self.pending.remove(position);
                }
                self.notice = crate::describe::outcome(self, action, outcome);
                if let Action::SetConfiguration {
                    entity,
                    configuration,
                } = action
                    && outcome == Outcome::Done
                {
                    self.reconfiguring.insert(entity, configuration);
                }
                self.agent_action_finished(action, outcome);
                // The steps after it go on only once it is done.
                if let Some(position) = self.chains.iter().position(|(head, _)| *head == action) {
                    let (_, rest) = self.chains.remove(position);
                    if outcome == Outcome::Done {
                        self.act_in_turn(rest);
                    }
                }
            }
        }
    }

    /// The entities matching the search, themselves or by a stream's
    /// name, by name, then entity ID.
    pub fn shown_entities(&self) -> Vec<&DiscoveredEntity> {
        let search = self.search_text();
        let mut shown: Vec<&DiscoveredEntity> = self
            .entities
            .values()
            .filter(|entity| {
                search.is_empty()
                    || self.entity_matches(entity, &search)
                    || self.models.get(&entity.entity_id()).is_some_and(|model| {
                        [true, false].into_iter().any(|input| {
                            model
                                .streams(input)
                                .any(|stream| stream_matches(model, &stream, &search))
                        })
                    })
            })
            .collect();
        shown.sort_by_key(|entity| (self.entity_name(entity).to_lowercase(), entity.entity_id()));
        shown
    }

    /// The search, trimmed and in lower case.
    pub fn search_text(&self) -> String {
        self.search.trim().to_lowercase()
    }

    /// Whether the entity's name, group, IDs or MAC address contain
    /// `search`, in lower case.
    pub fn entity_matches(&self, entity: &DiscoveredEntity, search: &str) -> bool {
        let group = self
            .models
            .get(&entity.entity_id())
            .and_then(EntityModel::entity)
            .map(|descriptor| descriptor.group_name.to_lowercase());
        self.entity_name(entity).to_lowercase().contains(search)
            || group.is_some_and(|group| group.contains(search))
            || entity.entity_id().to_string().contains(search)
            || entity.mac.to_string().contains(search)
            || entity.adp.entity_model_id.to_string().contains(search)
    }

    /// The name to show for an entity by ID.
    pub fn entity_name_of(&self, entity_id: EntityId) -> String {
        match self.entity_or_heard(entity_id) {
            Some(entity) => self.entity_name(entity),
            None => entity_id.to_string(),
        }
    }

    /// An entity that advertises, else a controller heard.
    pub fn entity_or_heard(&self, entity_id: EntityId) -> Option<&DiscoveredEntity> {
        self.entities
            .get(&entity_id)
            .or_else(|| self.heard.get(&entity_id).map(|heard| &heard.entity))
    }

    /// The entities the entity list shows: those shown, and the
    /// controllers heard that the search finds, by name.
    pub fn listed_entities(&self) -> Vec<&DiscoveredEntity> {
        let search = self.search_text();
        let mut listed = self.shown_entities();
        listed.extend(
            self.heard
                .values()
                .map(|heard| &heard.entity)
                .filter(|entity| search.is_empty() || self.entity_matches(entity, &search)),
        );
        listed.sort_by_key(|entity| (self.entity_name(entity).to_lowercase(), entity.entity_id()));
        listed
    }

    /// This computer's entities starting, which the entity list shows as
    /// such: on the interface shown, unless another user's triib-endpointd
    /// holds it, and that the search finds, by name.
    pub fn starting_entities(&self) -> impl Iterator<Item = &crate::host::Starting> {
        let search = self.search_text();
        let here = self.endpoints_elsewhere.is_none();
        self.starting.iter().filter(move |starting| {
            here && (search.is_empty() || starting.name.to_lowercase().contains(&search))
        })
    }

    /// "Entity name: stream name" for a stream of an entity.
    /// A name as the inspector shows it: the one being set when a rename
    /// is on its way, else what the entity holds, falling back for a
    /// descriptor to its localized description.
    pub fn current_name(&self, target: NameTarget) -> String {
        let renaming = self.pending.iter().rev().find_map(|action| match action {
            Action::Rename {
                target: pending,
                name,
            } if *pending == target => Some(name.as_str()),
            _ => None,
        });
        if let Some(name) = renaming {
            return name.to_owned();
        }
        let Some(model) = self.models.get(&target.entity) else {
            return String::new();
        };
        match (target.descriptor_type, target.name_index) {
            (DescriptorType::ENTITY, 0) => model.entity().map(|entity| entity.entity_name),
            (DescriptorType::ENTITY, _) => model.entity().map(|entity| entity.group_name),
            (descriptor_type, _) => model.name_of(descriptor_type, target.index),
        }
        .unwrap_or_default()
        .to_owned()
    }

    /// Whether a change to `entity` is on its way, by what it changes.
    pub fn changing(&self, wanted: impl Fn(&Action) -> bool) -> Option<&Action> {
        self.pending.iter().rev().find(|action| wanted(action))
    }

    /// Moves the entity list's other parts with the one that scrolled:
    /// the headings sideways with the rest of the rows, the names up and
    /// down with them, and the rest with either.
    fn entity_list_scrolled(
        &mut self,
        part: crate::entity_table::ListPart,
        viewport: iced::widget::scrollable::Viewport,
    ) -> Task<Message> {
        use crate::entity_table::{ListPart, NAMES, REST, REST_HEADER};
        use iced::widget::operation::scroll_to;
        use iced::widget::scrollable::AbsoluteOffset;
        let offset = viewport.absolute_offset();
        let moved = |a: f32, b: f32| (a - b).abs() > 0.5;
        let (x, y) = self.entity_list_offset;
        let mut tasks = Vec::new();
        match part {
            ListPart::Rest => {
                if moved(offset.x, x) {
                    tasks.push(scroll_to(
                        REST_HEADER,
                        AbsoluteOffset {
                            x: offset.x,
                            y: 0.0,
                        },
                    ));
                }
                if moved(offset.y, y) {
                    tasks.push(scroll_to(
                        NAMES,
                        AbsoluteOffset {
                            x: 0.0,
                            y: offset.y,
                        },
                    ));
                }
                self.entity_list_offset = (offset.x, offset.y);
            }
            ListPart::Headings if moved(offset.x, x) => {
                self.entity_list_offset.0 = offset.x;
                tasks.push(scroll_to(REST, AbsoluteOffset { x: offset.x, y }));
            }
            ListPart::Names if moved(offset.y, y) => {
                self.entity_list_offset.1 = offset.y;
                tasks.push(scroll_to(REST, AbsoluteOffset { x, y: offset.y }));
            }
            _ => {}
        }
        Task::batch(tasks)
    }

    /// Sends the first of `actions`, keeping the rest to send once it
    /// succeeds.
    fn act_in_turn(&mut self, mut actions: Vec<Action>) {
        let Some(network) = &self.network else {
            return;
        };
        if actions.is_empty() {
            return;
        }
        let first = actions.remove(0);
        self.pending.push(first);
        network.act(first);
        if !actions.is_empty() {
            self.chains.push((first, actions));
        }
    }

    /// How many stream inputs triib knows of are bound to a talker's
    /// stream output.
    pub fn listeners_of(&self, talker: (EntityId, u16)) -> usize {
        self.models
            .values()
            .map(|model| {
                model
                    .streams(true)
                    .filter(|input| {
                        model
                            .binding(input.index)
                            .and_then(|binding| binding.talker_stream())
                            == Some(talker)
                    })
                    .count()
            })
            .sum()
    }

    /// Whether `action` waits in a chain for a step before it.
    pub fn waiting(&self, wanted: impl Fn(&Action) -> bool) -> bool {
        self.chains.iter().any(|(_, rest)| rest.iter().any(&wanted))
    }

    pub fn stream_name(
        &self,
        entity_id: EntityId,
        descriptor_type: atdecc::DescriptorType,
        index: u16,
    ) -> String {
        let stream = self
            .models
            .get(&entity_id)
            .and_then(|model| model.name_of(descriptor_type, index))
            .map_or_else(|| format!("stream {index}"), str::to_owned);
        format!("{}: {stream}", self.entity_name_of(entity_id))
    }

    /// The name to show for an entity: the one it was given; else for
    /// triib's controller, which has no model to read, the computer it
    /// runs on; else its entity ID.
    pub fn entity_name(&self, entity: &DiscoveredEntity) -> String {
        if let Some(name) = self
            .models
            .get(&entity.entity_id())
            .and_then(EntityModel::entity_name)
        {
            return name.to_owned();
        }
        let triib = entity.adp.entity_model_id == crate::network::ENTITY_MODEL_ID;
        let heard = self.heard.contains_key(&entity.entity_id());
        if !triib && !heard {
            return entity.entity_id().to_string();
        }
        match (self.computer_of(entity), triib) {
            (Computer::This, true) => crate::fl!("entity-triib-here"),
            (Computer::Of(host), true) => crate::fl!("entity-triib-on", host = host),
            (Computer::Unknown, true) => crate::fl!("entity-triib-controller"),
            (Computer::This, false) => crate::fl!("entity-controller-here"),
            (Computer::Of(host), false) => crate::fl!("entity-controller-on", host = host),
            (Computer::Unknown, false) => {
                crate::fl!("entity-controller-at", mac = entity.mac.to_string())
            }
        }
    }

    /// The computer a controller with no model of its own runs on: this
    /// one, or the one an entity read from the same address names, as the
    /// Mac's AVB entity is named for the Mac.
    fn computer_of(&self, controller: &DiscoveredEntity) -> Computer<'_> {
        if self
            .interface()
            .is_some_and(|interface| interface.mac == controller.mac)
        {
            return Computer::This;
        }
        self.entities
            .values()
            .filter(|entity| {
                entity.mac == controller.mac
                    && entity.adp.entity_model_id != crate::network::ENTITY_MODEL_ID
            })
            .find_map(|entity| {
                self.models
                    .get(&entity.entity_id())
                    .and_then(EntityModel::entity_name)
            })
            .map_or(Computer::Unknown, Computer::Of)
    }

    /// Saves the settings, keeping why when that fails.
    /// Sets the side empty text fields start on: the keyboard layout's,
    /// else the interface's.
    fn apply_input_direction(&self) {
        scramble_ui::dir::set_input_right_to_left(
            self.keyboard_rtl.unwrap_or_else(scramble_ui::dir::rtl),
        );
    }

    pub(crate) fn save_settings(&mut self) {
        self.settings_error = self.settings.save().err();
    }

    /// Does what the menu bar's `item` asks, as its toolbar tool would. A
    /// dialog opened from it takes the place of the other, as the window
    /// shows one at a time.
    #[cfg(target_os = "macos")]
    fn menu_item(&mut self, item: crate::menu_macos::Item) -> Task<Message> {
        use crate::menu_macos::Item;
        let message = match item {
            Item::Settings => {
                self.presets_open = false;
                Message::SettingsOpened
            }
            Item::Presets => {
                self.settings_open = false;
                Message::PresetsOpened
            }
            // Closing triib's one window quits it.
            Item::Quit | Item::CloseWindow => Message::CloseWindow,
            Item::View(view) => Message::ViewPicked(view),
            Item::Log => Message::LogToggled,
            Item::Inspector => Message::InspectorToggled,
            Item::Website => Message::OpenLink("https://triib.run/".to_owned()),
            Item::ReleaseNotes => Message::OpenLink(format!(
                "https://github.com/scrambletoolsllc/triib/releases/tag/v{}",
                env!("CARGO_PKG_VERSION")
            )),
        };
        self.update(message)
    }

    fn reload_omarchy(&mut self) {
        self.omarchy = self.omarchy_dir.as_deref().and_then(omarchy::load);
        self.refresh_theme();
    }

    /// Rebuilds the M3 scheme after a setting, the system mode or the
    /// Omarchy theme changed.
    fn refresh_theme(&mut self) {
        let (seed, dark) = appearance::choose(Inputs {
            appearance: self.settings.appearance,
            system_accent: self.settings.system_accent,
            omarchy: self.omarchy.as_ref(),
            system_accent_color: self.system_accent,
            system_dark: self.system_mode == iced::theme::Mode::Dark,
            chosen_accent: self
                .settings
                .accent_color
                .as_deref()
                .and_then(appearance::hex_to_color)
                .unwrap_or(TRIIB_SEED),
        });
        let name = match (&self.omarchy, self.settings.system_accent) {
            (Some(palette), true) => format!("triib ({})", palette.name),
            _ => "triib".to_owned(),
        };
        self.theme = scheme::theme(name, seed, dark);
    }

    fn apply_motion(&self) {
        motion::set_reduced(!self.settings.animations || !self.system_animations);
    }
}

/// Whether the stream's name contains `search`, in lower case.
pub fn stream_matches(
    model: &EntityModel,
    stream: &atdecc::descriptor::StreamDescriptor<'_>,
    search: &str,
) -> bool {
    model
        .name_of(stream.descriptor_type, stream.index)
        .is_some_and(|name| name.to_lowercase().contains(search))
}
