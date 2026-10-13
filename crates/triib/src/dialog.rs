//! Folder dialogs: through the XDG desktop portal on Linux, and the
//! system's own dialogs (through rfd) elsewhere, as prev has them.

use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub use portal::*;

#[cfg(not(target_os = "linux"))]
pub use native::*;

#[cfg(target_os = "linux")]
mod portal {
    use std::path::PathBuf;

    use ashpd::desktop::file_chooser::SelectedFiles;

    use super::file_uri_to_path;

    /// Asks for a folder, starting in `current`. Cancelling yields `None`.
    pub async fn choose_folder(
        title: String,
        current: Option<PathBuf>,
    ) -> Result<Option<PathBuf>, String> {
        let request = SelectedFiles::open_file()
            .title(title.as_str())
            .directory(true)
            .current_folder::<&std::path::Path>(current.as_deref())
            .map_err(|error| error.to_string())?
            .send()
            .await
            .map_err(|error| error.to_string())?;
        match request.response() {
            Ok(selected) => Ok(selected
                .uris()
                .first()
                .and_then(|uri| file_uri_to_path(uri.as_str()))),
            // Cancelled, or closed some other way, as GTK's chooser reports
            // Escape: either way the user chose nothing.
            Err(ashpd::Error::Response(_)) => Ok(None),
            Err(error) => Err(error.to_string()),
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod native {
    use std::path::PathBuf;

    use rfd::AsyncFileDialog;

    /// Asks for a folder, starting in `current`, else Documents. Cancelling
    /// yields `None`.
    pub async fn choose_folder(
        title: String,
        current: Option<PathBuf>,
    ) -> Result<Option<PathBuf>, String> {
        let mut dialog = AsyncFileDialog::new().set_title(title);
        let start = current.or_else(|| {
            triib_store::paths::home()
                .map(|home| home.join("Documents"))
                .filter(|documents| documents.is_dir())
        });
        if let Some(start) = start {
            dialog = dialog.set_directory(start);
        }
        Ok(dialog
            .pick_folder()
            .await
            .map(|folder| folder.path().to_path_buf()))
    }
}

/// Converts a local `file://` URI to a path, decoding percent escapes.
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn file_uri_to_path(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix("file://")?;
    let path = rest.strip_prefix("localhost").unwrap_or(rest);
    if !path.starts_with('/') {
        return None;
    }
    let mut bytes = Vec::with_capacity(path.len());
    let mut input = path.bytes();
    while let Some(byte) = input.next() {
        if byte == b'%' {
            let high = input
                .next()
                .and_then(|digit| (digit as char).to_digit(16))?;
            let low = input
                .next()
                .and_then(|digit| (digit as char).to_digit(16))?;
            bytes.push((high * 16 + low) as u8);
        } else {
            bytes.push(byte);
        }
    }
    path_from_bytes(bytes)
}

#[cfg(unix)]
fn path_from_bytes(bytes: Vec<u8>) -> Option<PathBuf> {
    use std::os::unix::ffi::OsStringExt;
    Some(PathBuf::from(std::ffi::OsString::from_vec(bytes)))
}

/// `/C:/Users/...` from a `file:///C:/Users/...` URI becomes `C:/Users/...`.
#[cfg(not(unix))]
fn path_from_bytes(bytes: Vec<u8>) -> Option<PathBuf> {
    let path = String::from_utf8(bytes).ok()?;
    let path = match path.as_bytes() {
        [b'/', drive, b':', ..] if drive.is_ascii_alphabetic() => &path[1..],
        _ => &path,
    };
    Some(PathBuf::from(path))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn decodes_file_uris() {
        assert_eq!(
            file_uri_to_path("file:///home/me/a%20b"),
            Some(PathBuf::from("/home/me/a b"))
        );
        assert_eq!(
            file_uri_to_path("file://localhost/tmp/x"),
            Some(PathBuf::from("/tmp/x"))
        );
        assert_eq!(
            file_uri_to_path("file:///tmp/%C3%A9t%C3%A9"),
            Some(PathBuf::from("/tmp/été"))
        );
        assert_eq!(file_uri_to_path("file://server/share/x"), None);
        assert_eq!(file_uri_to_path("https://example.com/x"), None);
        assert_eq!(file_uri_to_path("file:///bad%zz"), None);
    }
}
