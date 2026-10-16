//! How loud each running stream is, for the app's meters: `meters.toml` in
//! the runtime folder, written 20 times a second while an app asks for
//! it, as it does by touching `meters-wanted` there while it shows them.

use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

/// How often the daemon writes the meters, and the app reads them.
pub const EVERY: Duration = Duration::from_millis(50);

/// How long after an app last asked the daemon goes on writing.
pub const WANTED_FOR: Duration = Duration::from_secs(3);

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Meters {
    /// Counts the writes, so a reader tells a new one from the last.
    pub sequence: u64,
    #[serde(rename = "stream", default, skip_serializing_if = "Vec::is_empty")]
    pub streams: Vec<StreamMeter>,
}

/// One of an entity's streams while it runs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamMeter {
    /// As text, `0x` and sixteen hex digits.
    pub entity_id: String,
    pub input: bool,
    pub index: u16,
    /// The loudest sample in each of its first two channels over the last
    /// two writes, from 0 to 1, so a reader taking one write in two still
    /// sees every peak.
    pub peaks: [f32; 2],
}

fn meters_path() -> Option<PathBuf> {
    triib_store::paths::runtime_dir().map(|dir| dir.join("meters.toml"))
}

fn wanted_path() -> Option<PathBuf> {
    triib_store::paths::runtime_dir().map(|dir| dir.join("meters-wanted"))
}

/// Asks the daemon for its meters for the next [`WANTED_FOR`].
pub fn want() {
    let Some(path) = wanted_path() else {
        return;
    };
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Ok(file) = File::options().create(true).append(true).open(&path) {
        let _ = file.set_modified(SystemTime::now());
    }
}

/// Whether an app asked for the meters within [`WANTED_FOR`].
pub fn wanted() -> bool {
    wanted_path()
        .and_then(|path| std::fs::metadata(path).ok())
        .and_then(|metadata| metadata.modified().ok())
        .and_then(|asked| asked.elapsed().ok())
        .is_some_and(|since| since < WANTED_FOR)
}

/// The meters the daemon last wrote.
pub fn read() -> Option<Meters> {
    triib_store::load(&meters_path()?).ok()
}

/// Replaces the meters, by a sibling file renamed over them so a reader
/// never sees half, without syncing it to disk: they last a moment.
pub fn write(meters: &Meters) -> io::Result<()> {
    let path = meters_path().ok_or_else(|| io::Error::other("no runtime folder"))?;
    let text = toml::to_string(meters).map_err(io::Error::other)?;
    let temp = path.with_extension("toml.new");
    File::create(&temp)?.write_all(text.as_bytes())?;
    std::fs::rename(&temp, &path)
}

/// Removes the meters, as the daemon stops.
pub fn remove() {
    if let Some(path) = meters_path() {
        let _ = std::fs::remove_file(path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meters_read_back() {
        let meters = Meters {
            sequence: 7,
            streams: vec![StreamMeter {
                entity_id: "0xa010a3ff00a51726".into(),
                input: true,
                index: 1,
                peaks: [0.5, 0.25],
            }],
        };
        let text = toml::to_string(&meters).unwrap();
        assert_eq!(toml::from_str::<Meters>(&text).unwrap(), meters);
        // No streams running.
        let none = Meters {
            sequence: 8,
            streams: Vec::new(),
        };
        let text = toml::to_string(&none).unwrap();
        assert_eq!(toml::from_str::<Meters>(&text).unwrap(), none);
    }
}
