//! The app's settings and window state, kept in `triib.toml`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use atdecc::EntityId;

use scramble_ui::appearance::Appearance;
use serde::{Deserialize, Serialize};

use crate::matrix::Side;

/// The language setting's value for following the system's language.
pub const SYSTEM_LANGUAGE: &str = "system";

/// Where presets are kept unless the settings say otherwise: `presets` in
/// the data folder.
pub fn default_presets_folder() -> PathBuf {
    triib_store::paths::data_dir()
        .map(|dir| dir.join("presets"))
        .unwrap_or_default()
}

/// Paths written with `~` for the home folder, and read back expanded.
mod home_path {
    use std::path::{Path, PathBuf};

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(path: &Path, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&triib_store::paths::abbreviate_home(path).to_string_lossy())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<PathBuf, D::Error> {
        let text = String::deserialize(deserializer)?;
        Ok(triib_store::paths::expand_home(Path::new(&text)))
    }
}

/// A kind of interface the interface picker can leave out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceFilter {
    Virtual,
    Wireless,
    /// Without a PTP hardware clock.
    Unclocked,
}

impl Settings {
    /// Whether the picker leaves out interfaces of `filter`'s kind.
    pub fn hides(&self, filter: InterfaceFilter) -> bool {
        match filter {
            InterfaceFilter::Virtual => self.hide_virtual_interfaces,
            InterfaceFilter::Wireless => self.hide_wireless_interfaces,
            InterfaceFilter::Unclocked => self.hide_unclocked_interfaces,
        }
    }

    fn hides_mut(&mut self, filter: InterfaceFilter) -> &mut bool {
        match filter {
            InterfaceFilter::Virtual => &mut self.hide_virtual_interfaces,
            InterfaceFilter::Wireless => &mut self.hide_wireless_interfaces,
            InterfaceFilter::Unclocked => &mut self.hide_unclocked_interfaces,
        }
    }

    /// Leaves interfaces of `filter`'s kind out of the picker, or puts
    /// them back.
    pub fn toggle_filter(&mut self, filter: InterfaceFilter) {
        let hides = self.hides_mut(filter);
        *hides = !*hides;
    }

    /// Whether the connection matrix leaves out what `filter` names.
    pub fn hides_streams(&self, filter: MatrixFilter) -> bool {
        match filter {
            MatrixFilter::Unconnectable => self.matrix_connectable_only,
            MatrixFilter::Lite => self.matrix_hide_lite,
            MatrixFilter::Wireless => self.matrix_hide_wireless,
        }
    }

    /// Leaves what `filter` names out of the connection matrix, or puts it
    /// back.
    pub fn toggle_matrix_filter(&mut self, filter: MatrixFilter) {
        let hides = match filter {
            MatrixFilter::Unconnectable => &mut self.matrix_connectable_only,
            MatrixFilter::Lite => &mut self.matrix_hide_lite,
            MatrixFilter::Wireless => &mut self.matrix_hide_wireless,
        };
        *hides = !*hides;
    }

    /// Whether the entity list leaves out the entities `filter` names.
    pub fn hides_entities(&self, filter: EntityFilter) -> bool {
        match filter {
            EntityFilter::Others => self.host_entities_only,
            EntityFilter::Controllers => self.hide_controllers,
            EntityFilter::OtherDomains => self.hide_other_domains,
        }
    }

    /// Leaves the entities `filter` names out of the entity list, or puts
    /// them back.
    pub fn toggle_entity_filter(&mut self, filter: EntityFilter) {
        let hides = match filter {
            EntityFilter::Others => &mut self.host_entities_only,
            EntityFilter::Controllers => &mut self.hide_controllers,
            EntityFilter::OtherDomains => &mut self.hide_other_domains,
        };
        *hides = !*hides;
    }
}

/// What the connection matrix can leave out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixFilter {
    /// Streams nothing shown can connect to.
    Unconnectable,
    /// Entities running AVB Lite, with their streams.
    Lite,
    /// Entities on Wi-Fi, with their streams.
    Wireless,
}

/// Entities the entity list can leave out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityFilter {
    /// All but this computer's own.
    Others,
    /// Those that are only controllers, with no streams.
    Controllers,
    /// Those keeping time in none of the clock domains this computer does.
    OtherDomains,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    /// The interface's language: a tag such as "en", or
    /// [`SYSTEM_LANGUAGE`].
    pub language: String,
    pub appearance: Appearance,
    /// Follow the desktop's accent color.
    pub system_accent: bool,
    /// "#RRGGBB" picked in the settings, else triib's own.
    pub accent_color: Option<String>,
    pub animations: bool,
    /// The interface last used, chosen again at startup when present.
    pub interface: Option<String>,
    /// Leave bridges, tunnels and other virtual interfaces out of the
    /// interface picker.
    pub hide_virtual_interfaces: bool,
    /// Leave wireless interfaces out of it.
    pub hide_wireless_interfaces: bool,
    /// Leave interfaces without a PTP hardware clock out of it.
    pub hide_unclocked_interfaces: bool,
    /// The folder presets are kept in.
    #[serde(with = "home_path")]
    pub presets_folder: PathBuf,
    /// The file this computer's entities are listed in.
    #[serde(with = "home_path")]
    pub entities_file: PathBuf,
    /// What this computer listens to, and through which output.
    pub listen: crate::monitor::Listen,
    /// The entity list leaves out all but this computer's own entities.
    pub host_entities_only: bool,
    /// It leaves out entities that are only controllers.
    pub hide_controllers: bool,
    /// It leaves out entities in clock domains this computer is not in.
    pub hide_other_domains: bool,
    pub view: View,
    pub inspector: bool,
    /// The log shows in a panel under the view.
    pub log: bool,
    /// The height the user dragged the log panel to.
    pub log_height: f32,
    /// The streams the connection matrix shows.
    pub matrix_streams: Streams,
    /// Leave out of the matrix the streams nothing shown can connect to.
    pub matrix_connectable_only: bool,
    /// Leave out of it the entities running AVB Lite.
    pub matrix_hide_lite: bool,
    /// Leave out of it the entities on Wi-Fi.
    pub matrix_hide_wireless: bool,
    /// What the network view shows flowing along its wires.
    pub network_shows: NetworkShows,
    /// The entity list's columns after the name, in order.
    pub entity_columns: Vec<EntityField>,
    /// Widths the user dragged the entity list's columns to, by field,
    /// kept when a column is removed; the others fit their text.
    pub entity_column_widths: BTreeMap<EntityField, f32>,
    /// The width the user dragged the entity list's name column to.
    pub entity_name_width: Option<f32>,
    /// The entities the connection matrix folds into one line.
    pub matrix_collapsed: Collapsed,
    /// Programs such as AI agents may control triib, through `triib
    /// --mcp`, once the user allows each.
    pub outside_control: bool,
    /// The agents the user allowed, by the names they give.
    pub allowed_agents: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: SYSTEM_LANGUAGE.to_owned(),
            appearance: Appearance::System,
            system_accent: true,
            accent_color: None,
            animations: true,
            interface: None,
            hide_virtual_interfaces: true,
            hide_wireless_interfaces: false,
            hide_unclocked_interfaces: false,
            presets_folder: default_presets_folder(),
            entities_file: triib_endpointd::config::path().unwrap_or_default(),
            listen: crate::monitor::Listen::default(),
            host_entities_only: false,
            hide_controllers: false,
            hide_other_domains: false,
            view: View::Matrix,
            inspector: false,
            log: false,
            log_height: 240.0,
            matrix_streams: Streams::All,
            matrix_connectable_only: false,
            matrix_hide_lite: false,
            matrix_hide_wireless: false,
            network_shows: NetworkShows::Audio,
            entity_columns: EntityField::DEFAULT_COLUMNS.to_vec(),
            entity_column_widths: BTreeMap::new(),
            entity_name_width: None,
            matrix_collapsed: Collapsed::default(),
            outside_control: true,
            allowed_agents: Vec::new(),
        }
    }
}

/// The entities the connection matrix folds into one line, by entity ID:
/// as talkers across the top, and as listeners down the side. Kept for
/// entities offline too, so they come back as they were.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Collapsed {
    pub talkers: BTreeSet<String>,
    pub listeners: BTreeSet<String>,
}

impl Collapsed {
    pub fn contains(&self, side: Side, entity_id: EntityId) -> bool {
        self.side(side).contains(&entity_id.to_string())
    }

    /// Folds an expanded entity, or unfolds a folded one.
    pub fn toggle(&mut self, side: Side, entity_id: EntityId) {
        let entities = match side {
            Side::Talker => &mut self.talkers,
            Side::Listener => &mut self.listeners,
        };
        let key = entity_id.to_string();
        if !entities.remove(&key) {
            entities.insert(key);
        }
    }

    fn side(&self, side: Side) -> &BTreeSet<String> {
        match side {
            Side::Talker => &self.talkers,
            Side::Listener => &self.listeners,
        }
    }
}

/// What the content area shows.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum View {
    /// Talker streams against listener streams.
    // The log was a view before it became a panel under any of them.
    #[default]
    #[serde(alias = "log")]
    Matrix,
    // This computer's entities are like any other, and each
    // entity's media clock is picked in its row.
    #[serde(alias = "host", alias = "clocks")]
    Entities,
    /// The network as gPTP paths show it.
    Network,
}

/// What a column of the entity list after the name shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EntityField {
    Group,
    Product,
    /// The entity's own logo.
    EntityImage,
    Vendor,
    /// Its vendor's logo.
    VendorImage,
    Model,
    Firmware,
    SerialNumber,
    Milan,
    Roles,
    /// The clock source of the entity's clock domain, picked in its cell.
    #[serde(alias = "clock-source")]
    MediaClock,
    SamplingRate,
    /// The gPTP grandmaster the entity advertises.
    #[serde(alias = "btc")]
    Grandmaster,
    State,
    EntityId,
    MacAddress,
    EntityModelId,
    Configuration,
    TalkerStreams,
    ListenerStreams,
    /// How it runs: AVB Lite, capable of it, or not.
    AvbLite,
    /// What its talker streams take of its link.
    Egress,
    /// Its wireless interface's role, and how its time holds.
    Wireless,
}

impl EntityField {
    /// Every field, in the order the column menus offer them.
    pub const ALL: [Self; 23] = [
        Self::Group,
        Self::Product,
        Self::EntityImage,
        Self::Vendor,
        Self::VendorImage,
        Self::Model,
        Self::Firmware,
        Self::SerialNumber,
        Self::Milan,
        Self::Roles,
        Self::MediaClock,
        Self::SamplingRate,
        Self::Grandmaster,
        Self::State,
        Self::EntityId,
        Self::MacAddress,
        Self::EntityModelId,
        Self::Configuration,
        Self::TalkerStreams,
        Self::ListenerStreams,
        Self::AvbLite,
        Self::Egress,
        Self::Wireless,
    ];

    /// The columns shown until the user changes them.
    pub const DEFAULT_COLUMNS: [Self; 9] = [
        Self::Group,
        Self::Product,
        Self::Firmware,
        Self::Milan,
        Self::Roles,
        Self::MediaClock,
        Self::Grandmaster,
        Self::State,
        Self::EntityId,
    ];

    /// The column's heading.
    pub fn label(self) -> String {
        match self {
            Self::Group => crate::fl!("entity-group"),
            Self::Product => crate::fl!("entity-product"),
            Self::EntityImage => crate::fl!("column-entity-image"),
            Self::Vendor => crate::fl!("column-vendor"),
            Self::VendorImage => crate::fl!("column-vendor-image"),
            Self::Model => crate::fl!("column-model"),
            Self::Firmware => crate::fl!("entity-firmware"),
            Self::SerialNumber => crate::fl!("entity-serial-number"),
            Self::Milan => crate::fl!("entity-milan"),
            Self::Roles => crate::fl!("advert-roles"),
            Self::MediaClock => crate::fl!("entity-media-clock"),
            Self::SamplingRate => crate::fl!("entity-sampling-rate"),
            Self::Grandmaster => crate::fl!("avb-interface-grandmaster"),
            Self::State => crate::fl!("column-state"),
            Self::EntityId => crate::fl!("advert-entity-id"),
            Self::MacAddress => crate::fl!("common-mac-address"),
            Self::EntityModelId => crate::fl!("column-entity-model-id"),
            Self::Configuration => crate::fl!("entity-configuration"),
            Self::TalkerStreams => crate::fl!("column-talker-streams"),
            Self::ListenerStreams => crate::fl!("column-listener-streams"),
            Self::AvbLite => crate::fl!("column-avb-lite"),
            Self::Egress => crate::fl!("column-egress"),
            Self::Wireless => crate::fl!("column-wireless"),
        }
    }
}

/// Which streams the connection matrix shows.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Streams {
    #[default]
    All,
    Audio,
    /// Media clock streams, such as CRF.
    Clock,
}

/// What the network view shows flowing along its wires.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkShows {
    /// The gPTP clock, from the grandmaster down.
    Clock,
    /// The bound audio streams, each on a wire of its own.
    #[default]
    // Settings saved when the view showed all streams together.
    #[serde(alias = "streams")]
    Audio,
    /// The bound media clock streams, such as CRF.
    MediaClock,
}

impl Settings {
    /// Loads `triib.toml` (`triib-dev.toml` for development builds),
    /// writing it so it lists every setting; when it cannot be used, the
    /// defaults and why.
    pub fn load() -> (Self, Option<String>) {
        // Tests start from the defaults, whatever the user has.
        if cfg!(test) {
            return (Self::default(), None);
        }
        let Some(path) = triib_store::paths::config_file() else {
            return (Self::default(), Some(crate::fl!("settings-no-place")));
        };
        match triib_store::load_or_create::<Self>(&path) {
            Ok(settings) => (settings.tidied(), None),
            Err(error) => (
                Self::default(),
                Some(crate::fl!(
                    "settings-unusable",
                    path = path.display().to_string(),
                    error = error.to_string()
                )),
            ),
        }
    }

    /// The settings with each entity list column once, as a field that
    /// became another's name can leave two.
    fn tidied(mut self) -> Self {
        let mut seen = std::collections::BTreeSet::new();
        self.entity_columns.retain(|field| seen.insert(*field));
        self
    }

    /// Saves the settings, or says why they could not be saved.
    /// The language picked in Settings, or none to follow the system's.
    pub fn chosen_language(&self) -> Option<&str> {
        (self.language != SYSTEM_LANGUAGE).then_some(self.language.as_str())
    }

    pub fn save(&self) -> Result<(), String> {
        // Tests drive the app, which saves as it goes; never over the
        // user's own settings.
        if cfg!(test) {
            return Ok(());
        }
        let path =
            triib_store::paths::config_file().ok_or_else(|| crate::fl!("settings-no-place"))?;
        triib_store::save(&path, self).map_err(|error| {
            crate::fl!(
                "settings-unsaved",
                path = path.display().to_string(),
                error = error.to_string()
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_given_up_load_as_what_took_their_place() {
        let file = std::env::temp_dir().join(format!("triib-renamed-{}.toml", std::process::id()));
        triib_store::save(&file, &Settings::default()).unwrap();
        // The media clock view and the clock source column, as an older
        // triib saved them, beside the media clock column, and the
        // grandmaster's column by its older name.
        let saved = std::fs::read_to_string(&file).unwrap();
        let older = saved
            .replace("view = \"matrix\"", "view = \"clocks\"")
            .replace("\"media-clock\"", "\"clock-source\", \"media-clock\"")
            .replace("\"grandmaster\"", "\"btc\"");
        assert_ne!(older, saved);
        std::fs::write(&file, older).unwrap();
        let loaded = triib_store::load_or_create::<Settings>(&file)
            .unwrap()
            .tidied();
        let _ = std::fs::remove_file(&file);
        assert_eq!(loaded.view, View::Entities);
        assert_eq!(loaded.entity_columns, EntityField::DEFAULT_COLUMNS);

        // A column's width kept by the grandmaster's older name.
        let older = saved.replace(
            "[entity_column_widths]\n",
            "[entity_column_widths]\nbtc = 73.5\n",
        );
        assert_ne!(older, saved);
        std::fs::write(&file, older).unwrap();
        let loaded = triib_store::load_or_create::<Settings>(&file).unwrap();
        let _ = std::fs::remove_file(&file);
        assert_eq!(
            loaded.entity_column_widths.get(&EntityField::Grandmaster),
            Some(&73.5)
        );

        // The log view, now a panel.
        std::fs::write(&file, saved.replace("view = \"matrix\"", "view = \"log\"")).unwrap();
        let loaded = triib_store::load_or_create::<Settings>(&file).unwrap();
        let _ = std::fs::remove_file(&file);
        assert_eq!(loaded.view, View::Matrix);
        assert!(!loaded.log);
    }

    #[test]
    fn folded_entities_are_kept_by_side() {
        let synthia = EntityId(0xe8f6_0ae0_9220_0000);
        let amp = EntityId(0xe8f6_0ae0_9721_0000);
        let mut settings = Settings::default();
        settings.matrix_collapsed.toggle(Side::Talker, synthia);
        settings.matrix_collapsed.toggle(Side::Listener, amp);
        settings.matrix_collapsed.toggle(Side::Listener, synthia);
        // Folded again, unfolded.
        settings.matrix_collapsed.toggle(Side::Listener, amp);
        let file = std::env::temp_dir().join(format!("triib-folded-{}.toml", std::process::id()));
        triib_store::save(&file, &settings).unwrap();
        let loaded = triib_store::load::<Settings>(&file).unwrap();
        let _ = std::fs::remove_file(&file);
        let folded = &loaded.matrix_collapsed;
        assert!(folded.contains(Side::Talker, synthia));
        assert!(folded.contains(Side::Listener, synthia));
        assert!(!folded.contains(Side::Talker, amp));
        assert!(!folded.contains(Side::Listener, amp));
        assert_eq!(
            folded.talkers,
            BTreeSet::from(["0xe8f60ae092200000".to_owned()])
        );
    }
}
