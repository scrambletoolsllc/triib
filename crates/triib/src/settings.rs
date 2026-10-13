//! The app's settings and window state, kept in `triib.toml`.

use std::collections::{BTreeMap, BTreeSet};

use atdecc::EntityId;

use scramble_ui::appearance::Appearance;
use serde::{Deserialize, Serialize};

use crate::matrix::Side;

/// The language setting's value for following the system's language.
pub const SYSTEM_LANGUAGE: &str = "system";

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
    /// The Entities view lists only this computer's own talkers and
    /// listeners.
    pub host_entities_only: bool,
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
            host_entities_only: false,
            view: View::Matrix,
            inspector: false,
            log: false,
            log_height: 240.0,
            matrix_streams: Streams::All,
            matrix_connectable_only: false,
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
    // Host talkers and listeners are entities like any other, and each
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
    Vendor,
    Model,
    Firmware,
    SerialNumber,
    Milan,
    Roles,
    /// The clock source of the entity's clock domain, picked in its cell.
    #[serde(alias = "clock-source")]
    MediaClock,
    SamplingRate,
    Btc,
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
    pub const ALL: [Self; 21] = [
        Self::Group,
        Self::Product,
        Self::Vendor,
        Self::Model,
        Self::Firmware,
        Self::SerialNumber,
        Self::Milan,
        Self::Roles,
        Self::MediaClock,
        Self::SamplingRate,
        Self::Btc,
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
        Self::Btc,
        Self::State,
        Self::EntityId,
    ];

    /// The column's heading.
    pub fn label(self) -> String {
        match self {
            Self::Group => crate::fl!("entity-group"),
            Self::Product => crate::fl!("entity-product"),
            Self::Vendor => crate::fl!("column-vendor"),
            Self::Model => crate::fl!("column-model"),
            Self::Firmware => crate::fl!("entity-firmware"),
            Self::SerialNumber => crate::fl!("entity-serial-number"),
            Self::Milan => crate::fl!("entity-milan"),
            Self::Roles => crate::fl!("advert-roles"),
            Self::MediaClock => crate::fl!("entity-media-clock"),
            Self::SamplingRate => crate::fl!("entity-sampling-rate"),
            Self::Btc => crate::fl!("advert-btc"),
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
        // triib saved them, beside the media clock column.
        let saved = std::fs::read_to_string(&file).unwrap();
        let older = saved
            .replace("view = \"matrix\"", "view = \"clocks\"")
            .replace("\"media-clock\"", "\"clock-source\", \"media-clock\"");
        assert_ne!(older, saved);
        std::fs::write(&file, older).unwrap();
        let loaded = triib_store::load_or_create::<Settings>(&file)
            .unwrap()
            .tidied();
        let _ = std::fs::remove_file(&file);
        assert_eq!(loaded.view, View::Entities);
        assert_eq!(loaded.entity_columns, EntityField::DEFAULT_COLUMNS);

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
