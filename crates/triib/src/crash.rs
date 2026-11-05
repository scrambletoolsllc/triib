//! A report of each panic, kept in `crash.log` in the state folder: a
//! release build on Windows has no console for the message, so a crash
//! leaves only its exit code otherwise.

use std::io::Write;
use std::panic::Location;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Past this size the file starts over, so a crash loop cannot fill it.
const LIMIT: u64 = 1 << 20;

/// Where the reports go: `%LOCALAPPDATA%\triib\crash.log`,
/// `$XDG_STATE_HOME/triib/crash.log`, or
/// `~/Library/Application Support/triib/crash.log`.
pub fn file() -> Option<PathBuf> {
    triib_store::paths::state_dir().map(|dir| dir.join("crash.log"))
}

/// Writes a report for every panic, on any thread, before Rust's own hook
/// prints it.
pub fn install() {
    let default = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let thread = std::thread::current();
        let mut text = report(
            thread.name().unwrap_or("unnamed"),
            info.location(),
            info.payload_as_str().unwrap_or("a panic without a message"),
            SystemTime::now(),
        );
        // Only where RUST_BACKTRACE asks: release builds carry no symbols.
        let backtrace = std::backtrace::Backtrace::capture();
        if backtrace.status() == std::backtrace::BacktraceStatus::Captured {
            text.push_str(&format!("{backtrace}\n"));
        }
        if let Some(path) = file() {
            let _ = append(&path, &text);
        }
        default(info);
    }));
}

/// One report: when, which build, and the panic as Rust prints it.
fn report(thread: &str, location: Option<&Location>, message: &str, at: SystemTime) -> String {
    let place = location
        .map(|location| format!(" at {location}"))
        .unwrap_or_default();
    let build = if triib_store::paths::PRODUCTION {
        ""
    } else {
        ", development build"
    };
    format!(
        "{} triib {} ({} {}{build})\nthread '{thread}' panicked{place}:\n{message}\n\n",
        utc(at),
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        std::env::consts::ARCH,
    )
}

/// Adds `text` to the file at `path`, starting it over once past LIMIT.
fn append(path: &Path, text: &str) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let full = std::fs::metadata(path).is_ok_and(|metadata| metadata.len() >= LIMIT);
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(!full)
        .truncate(full)
        .open(path)?
        .write_all(text.as_bytes())
}

/// A moment in UTC, as `2026-11-05 20:03:36 UTC`.
fn utc(at: SystemTime) -> String {
    let seconds = at.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
    let (year, month, day) = civil(seconds / 86_400);
    let time = seconds % 86_400;
    format!(
        "{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02} UTC",
        time / 3600,
        time / 60 % 60,
        time % 60
    )
}

/// The calendar date `days` after 1970-01-01, by Howard Hinnant's
/// civil_from_days.
fn civil(days: u64) -> (u64, u64, u64) {
    let days = days + 719_468;
    let era = days / 146_097;
    let day_of_era = days % 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_from_march = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_from_march + 2) / 5 + 1;
    let month = if month_from_march < 10 {
        month_from_march + 3
    } else {
        month_from_march - 9
    };
    (era * 400 + year_of_era + u64::from(month <= 2), month, day)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn a_report_says_when_where_and_what() {
        let at = UNIX_EPOCH + Duration::from_secs(1_793_909_016);
        let location = Location::caller();
        let text = report("main", Some(location), "no adapter", at);
        let lines: Vec<&str> = text.lines().collect();
        assert!(lines[0].starts_with("2026-11-05 20:03:36 UTC triib "));
        assert!(lines[0].contains(std::env::consts::OS));
        assert_eq!(lines[1], format!("thread 'main' panicked at {location}:"));
        assert_eq!(lines[2], "no adapter");
        assert!(text.ends_with("\n\n"));
        let text = report("network", None, "lost", at);
        assert!(text.contains("\nthread 'network' panicked:\nlost\n"));
    }

    #[test]
    fn dates_are_in_the_calendar() {
        assert_eq!(utc(UNIX_EPOCH), "1970-01-01 00:00:00 UTC");
        let leap = UNIX_EPOCH + Duration::from_secs(1_709_251_199);
        assert_eq!(utc(leap), "2024-02-29 23:59:59 UTC");
        assert_eq!(
            utc(leap + Duration::from_secs(1)),
            "2024-03-01 00:00:00 UTC"
        );
        let new_year = UNIX_EPOCH + Duration::from_secs(1_798_761_600);
        assert_eq!(utc(new_year), "2027-01-01 00:00:00 UTC");
    }

    #[test]
    fn reports_add_up_until_the_file_is_full() {
        let dir = std::env::temp_dir().join(format!("triib-crash-{}", std::process::id()));
        let path = dir.join("state/crash.log");
        let _ = std::fs::remove_dir_all(&dir);
        append(&path, "first\n").unwrap();
        append(&path, "second\n").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "first\nsecond\n");
        std::fs::write(&path, vec![b'x'; LIMIT as usize]).unwrap();
        append(&path, "third\n").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "third\n");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
