//! Where triib keeps its own files: the XDG base directories, on Windows
//! `%APPDATA%` and `%LOCALAPPDATA%`, and on macOS `~/Library`.

use std::env;
use std::path::{Path, PathBuf};

/// Set by release builds. Every other build is a development build, which
/// keeps its own settings and cache so it never disturbs the installed copy.
pub const PRODUCTION: bool = option_env!("TRIIB_PRODUCTION").is_some();

const APP_DIR: &str = if PRODUCTION { "triib" } else { "triib-dev" };

/// The home folder: `$HOME`, or `%USERPROFILE%` on Windows.
pub fn home() -> Option<PathBuf> {
    let variable = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    env::var_os(variable).map(PathBuf::from)
}

/// A path written with `~` for the home folder, made whole.
pub fn expand_home(path: &Path) -> PathBuf {
    match (path.strip_prefix("~"), home()) {
        (Ok(rest), Some(home)) => home.join(rest),
        _ => path.to_path_buf(),
    }
}

/// A path under the home folder written with `~`, for a readable file
/// and field.
pub fn abbreviate_home(path: &Path) -> PathBuf {
    match home() {
        Some(home) if path.starts_with(&home) && home != Path::new("/") => {
            Path::new("~").join(path.strip_prefix(&home).unwrap_or(path))
        }
        _ => path.to_path_buf(),
    }
}

/// The settings file: `$XDG_CONFIG_HOME/triib.toml`,
/// `%APPDATA%\triib\triib.toml` or
/// `~/Library/Application Support/triib/triib.toml`.
pub fn config_file() -> Option<PathBuf> {
    let name = format!("{APP_DIR}.toml");
    if cfg!(any(windows, target_os = "macos")) {
        data_dir().map(|dir| dir.join(name))
    } else {
        base("XDG_CONFIG_HOME", ".config").map(|dir| dir.join(name))
    }
}

/// Presets and saved layouts.
pub fn data_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        known("APPDATA").map(|dir| dir.join(APP_DIR))
    } else if cfg!(target_os = "macos") {
        home().map(|home| home.join("Library/Application Support").join(APP_DIR))
    } else {
        base("XDG_DATA_HOME", ".local/share").map(|dir| dir.join(APP_DIR))
    }
}

/// Window state.
pub fn state_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        known("LOCALAPPDATA").map(|dir| dir.join(APP_DIR))
    } else if cfg!(target_os = "macos") {
        data_dir()
    } else {
        base("XDG_STATE_HOME", ".local/state").map(|dir| dir.join(APP_DIR))
    }
}

/// The entity model cache.
pub fn cache_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        state_dir().map(|dir| dir.join("cache"))
    } else if cfg!(target_os = "macos") {
        home().map(|home| home.join("Library/Caches").join(APP_DIR))
    } else {
        base("XDG_CACHE_HOME", ".cache").map(|dir| dir.join(APP_DIR))
    }
}

/// Sockets between the app and `triib-endpointd`. macOS has no runtime
/// folder; there it is under `$TMPDIR`, which macOS gives each user.
pub fn runtime_dir() -> Option<PathBuf> {
    env::var_os("XDG_RUNTIME_DIR")
        .or_else(|| {
            cfg!(target_os = "macos")
                .then(|| env::var_os("TMPDIR"))
                .flatten()
        })
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .map(|dir| dir.join(APP_DIR))
}

/// An XDG base directory, or its default under the home folder.
fn base(variable: &str, fallback_under_home: &str) -> Option<PathBuf> {
    env::var_os(variable)
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .or_else(|| home().map(|home| home.join(fallback_under_home)))
}

/// A folder named by a Windows environment variable, such as `APPDATA`.
fn known(variable: &str) -> Option<PathBuf> {
    env::var_os(variable)
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
}
