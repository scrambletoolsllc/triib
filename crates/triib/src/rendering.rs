//! Drawing in software where the GPU cannot draw triib's window. iced
//! falls back to its tiny-skia renderer only when wgpu finds no adapter;
//! a computer whose adapter cannot present to the window, as a virtual
//! machine without a usable GPU, gets one and then fails once the window's
//! surface is set up, where wgpu panics. The panic hook then starts triib
//! again drawing in software, and this version keeps drawing in software
//! on this computer; a new version tries the GPU again.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

/// iced's choice of renderer, and the one that draws in software.
const BACKEND: &str = "ICED_BACKEND";
const SOFTWARE: &str = "tiny-skia";

/// Set once the app, rather than `triib --mcp` or a helper, is about to
/// open its window.
static APP: AtomicBool = AtomicBool::new(false);

/// The file that remembers the GPU failing, in the state folder: the
/// version it failed in, then wgpu's error.
pub fn marker() -> Option<PathBuf> {
    triib_store::paths::state_dir().map(|dir| dir.join("software-rendering"))
}

/// Before iced starts: draws in software where this version found the GPU
/// unusable here before, unless `ICED_BACKEND` picks a renderer itself.
#[allow(unsafe_code)]
pub fn choose() {
    APP.store(true, Ordering::SeqCst);
    if std::env::var_os(BACKEND).is_some() {
        return;
    }
    if marker().is_some_and(|path| remembered(&path, env!("CARGO_PKG_VERSION"))) {
        // SAFETY: called before any other thread starts.
        unsafe { std::env::set_var(BACKEND, SOFTWARE) };
    }
}

/// From the panic hook: where the app's GPU failed, and no renderer was
/// picked, remembers it and starts triib again drawing in software. Says
/// whether it did, after which this process should end.
pub fn fall_back(message: &str) -> bool {
    if !APP.load(Ordering::SeqCst) || !gpu_failure(message) || std::env::var_os(BACKEND).is_some() {
        return false;
    }
    if let Some(path) = marker() {
        remember(&path, env!("CARGO_PKG_VERSION"), message);
    }
    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    std::process::Command::new(exe)
        .args(std::env::args_os().skip(1))
        .env(BACKEND, SOFTWARE)
        .spawn()
        .is_ok()
}

/// For trying the fallback in a development build: with
/// `TRIIB_TEST_GPU_FAILURE` set, the app fails as wgpu does where the
/// window's surface cannot be set up, until it draws in software.
#[cfg(debug_assertions)]
pub fn fail_for_test() {
    if std::env::var_os("TRIIB_TEST_GPU_FAILURE").is_some() && std::env::var_os(BACKEND).is_none() {
        panic!(
            "wgpu error: Validation Error\n\nCaused by:\n  In Surface::configure\n    Invalid surface\n"
        );
    }
}

/// Whether a panic is wgpu's, as its errors on a device or a surface are
/// by default.
fn gpu_failure(message: &str) -> bool {
    message.starts_with("wgpu error:")
}

/// Whether the file at `path` says the GPU failed in `version`.
fn remembered(path: &Path, version: &str) -> bool {
    std::fs::read_to_string(path).is_ok_and(|text| text.lines().next() == Some(version))
}

/// Writes `version` and wgpu's error, on one line, to the file at `path`.
fn remember(path: &Path, version: &str, message: &str) {
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let error: Vec<&str> = message
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    let _ = std::fs::write(path, format!("{version}\n{}\n", error.join(" ")));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SURFACE: &str = "wgpu error: Validation Error\n\nCaused by:\n  In Surface::configure\n    Invalid surface\n";

    #[test]
    fn only_wgpu_panics_are_the_gpu_failing() {
        assert!(gpu_failure(SURFACE));
        assert!(!gpu_failure(
            "index out of bounds: the len is 0 but the index is 0"
        ));
        assert!(!gpu_failure("called `Option::unwrap()` on a `None` value"));
    }

    /// The file keeps the version the GPU failed in, and wgpu's error on
    /// one line; another version tries the GPU again.
    #[test]
    fn a_failure_is_remembered_for_its_version() {
        let dir = std::env::temp_dir().join(format!("triib-rendering-{}", std::process::id()));
        let path = dir.join("state/software-rendering");
        let _ = std::fs::remove_dir_all(&dir);
        assert!(!remembered(&path, "1.5.0"));
        remember(&path, "1.5.0", SURFACE);
        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "1.5.0\nwgpu error: Validation Error Caused by: In Surface::configure Invalid surface\n"
        );
        assert!(remembered(&path, "1.5.0"));
        assert!(!remembered(&path, "1.6.0"));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
