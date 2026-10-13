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
    /// This computer's entities, which triib-endpointd runs.
    Entities,
}

impl Storage {
    pub const ALL: [Storage; 2] = [Storage::Presets, Storage::Entities];

    /// Those Settings show: the entities file only where this computer
    /// runs entities of its own.
    pub fn shown() -> impl Iterator<Item = Storage> {
        Self::ALL
            .into_iter()
            .filter(|storage| *storage != Storage::Entities || crate::host::SUPPORTED)
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn label(self) -> String {
        match self {
            Storage::Presets => fl!("settings-storage-presets"),
            Storage::Entities => fl!("settings-storage-entities"),
        }
    }

    /// The title of the dialog that chooses where to keep it.
    pub fn choose_title(self) -> String {
        match self {
            Storage::Presets => fl!("dialog-choose-presets"),
            Storage::Entities => fl!("dialog-choose-entities"),
        }
    }

    pub fn path(self, settings: &Settings) -> &Path {
        match self {
            Storage::Presets => &settings.presets_folder,
            Storage::Entities => &settings.entities_file,
        }
    }

    /// Whether it is a file, kept in a folder, rather than a folder.
    pub fn is_file(self) -> bool {
        self == Storage::Entities
    }

    /// The folder its dialog starts in: the one it is in now, where that
    /// is there.
    pub fn current_folder(self, settings: &Settings) -> Option<PathBuf> {
        let path = self.path(settings);
        let folder = if self.is_file() {
            path.parent()
        } else {
            Some(path)
        };
        folder
            .map(Path::to_path_buf)
            .filter(|folder| folder.is_dir())
    }

    /// The path it takes from `path` as typed or chosen: `~` the home
    /// folder, and for a file, a folder given its file name.
    pub fn take(self, settings: &Settings, path: &Path) -> PathBuf {
        let path = triib_store::paths::expand_home(path);
        if self.is_file() && path.is_dir() {
            let name = self
                .path(settings)
                .file_name()
                .unwrap_or(std::ffi::OsStr::new("entities.toml"));
            return path.join(name);
        }
        path
    }
}

/// Whether triib can keep `storage` at `path`: a full path to a folder
/// that is there and that it can write in (for a file, a file in one).
pub fn check(storage: Storage, path: &Path) -> Result<(), String> {
    if !path.is_absolute() {
        return Err(fl!("settings-full-path"));
    }
    let folder = if storage.is_file() {
        if path.is_dir() {
            return Err(fl!("settings-path-is-folder", path = shown_path(path)));
        }
        path.parent().unwrap_or(path)
    } else {
        path
    };
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
    /// in; a file's folder must be there, and a file is not a folder.
    #[test]
    fn places_are_checked() {
        let dir = std::env::temp_dir().join(format!("triib-storage-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert!(check(Storage::Presets, &dir).is_ok());
        assert!(check(Storage::Entities, &dir.join("entities.toml")).is_ok());
        assert_eq!(
            check(Storage::Presets, Path::new("presets")),
            Err("Use a full path, such as ~/Documents/triib.".to_owned())
        );
        assert!(
            check(Storage::Presets, &dir.join("none"))
                .unwrap_err()
                .starts_with("There is no folder")
        );
        assert!(
            check(Storage::Entities, &dir)
                .unwrap_err()
                .ends_with("is a folder, not a file.")
        );
        let file = dir.join("file");
        std::fs::write(&file, b"").unwrap();
        assert!(
            check(Storage::Presets, &file)
                .unwrap_err()
                .ends_with("is a file, not a folder.")
        );
        // A folder given for the entities file gets its file name.
        let settings = Settings::default();
        assert_eq!(
            Storage::Entities.take(&settings, &dir),
            dir.join("entities.toml")
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
