//! The files and folders triib keeps whose places Settings choose, as
//! prev's Storage tab has them: each typed, then applied with Enter or
//! Apply, or chosen with the folder dialog, and checked first.

use std::path::{Path, PathBuf};

use crate::fl;
use crate::settings::Settings;

/// A file or folder triib keeps where the settings say.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Storage {
    Presets,
}

impl Storage {
    pub const ALL: [Storage; 1] = [Storage::Presets];

    /// Those Settings show.
    pub fn shown() -> impl Iterator<Item = Storage> {
        Self::ALL.into_iter()
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn label(self) -> String {
        match self {
            Storage::Presets => fl!("settings-storage-presets"),
        }
    }

    /// The title of the dialog that chooses where to keep it.
    pub fn choose_title(self) -> String {
        match self {
            Storage::Presets => fl!("dialog-choose-presets"),
        }
    }

    pub fn path(self, settings: &Settings) -> &Path {
        match self {
            Storage::Presets => &settings.presets_folder,
        }
    }

    /// The folder its dialog starts in: the one it is in now, or the
    /// nearest one above that is there, as a folder not made yet, such as
    /// presets before the first is saved, is in hidden folders the dialog
    /// would not show.
    pub fn current_folder(self, settings: &Settings) -> Option<PathBuf> {
        self.path(settings)
            .ancestors()
            .find(|folder| folder.is_dir())
            .map(Path::to_path_buf)
    }

    /// The path it takes from `path` as typed or chosen: `~` the home
    /// folder.
    pub fn take(self, _settings: &Settings, path: &Path) -> PathBuf {
        triib_store::paths::expand_home(path)
    }
}

/// Whether triib can keep `storage` at `path`: a full path to a folder
/// that is there and that it can write in.
pub fn check(_storage: Storage, path: &Path) -> Result<(), String> {
    if !path.is_absolute() {
        return Err(fl!("settings-full-path"));
    }
    let folder = path;
    if !folder.exists() {
        return Err(fl!("settings-folder-missing", path = shown_path(folder)));
    }
    if !folder.is_dir() {
        return Err(fl!("settings-path-is-file", path = shown_path(folder)));
    }
    // Writing a file is the only sure test of permission.
    let probe = folder.join(format!(".triib-write-test-{}", std::process::id()));
    match std::fs::write(&probe, b"") {
        Ok(()) => {
            let _ = std::fs::remove_file(&probe);
            Ok(())
        }
        Err(error) => Err(fl!(
            "settings-cannot-write",
            path = shown_path(folder),
            error = error.to_string()
        )),
    }
}

/// A path as Settings show it, with `~` for the home folder.
pub fn shown_path(path: &Path) -> String {
    triib_store::paths::abbreviate_home(path)
        .to_string_lossy()
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A place is taken once it is a full path to a folder triib can write
    /// in; a file is not a folder.
    #[test]
    fn places_are_checked() {
        let dir = std::env::temp_dir().join(format!("triib-storage-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert!(check(Storage::Presets, &dir).is_ok());
        assert_eq!(
            check(Storage::Presets, Path::new("presets")),
            Err("Use a full path, such as ~/Documents/triib.".to_owned())
        );
        assert!(
            check(Storage::Presets, &dir.join("none"))
                .unwrap_err()
                .starts_with("There is no folder")
        );
        let file = dir.join("file");
        std::fs::write(&file, b"").unwrap();
        assert!(
            check(Storage::Presets, &file)
                .unwrap_err()
                .ends_with("is a file, not a folder.")
        );
        // The dialog starts in the nearest folder there.
        let settings = Settings {
            presets_folder: dir.join("presets/not/yet"),
            ..Settings::default()
        };
        assert_eq!(
            Storage::Presets.current_folder(&settings),
            Some(dir.clone())
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
