//! Folder dialogs and links: through the XDG desktop portal on Linux,
//! and the system's own dialogs (through rfd) elsewhere, as prev has them.

use std::path::PathBuf;

#[cfg(target_os = "linux")]
pub use portal::*;

#[cfg(not(target_os = "linux"))]
pub use native::*;

/// On Linux the portal answers only a process it can look into, to tell
/// a sandboxed app from the host's, and it cannot look into one holding a
/// capability, as triib holds cap_net_raw. So this binary, run again
/// without any, asks it, and prints the answer.
#[cfg(target_os = "linux")]
mod portal {
    use std::ffi::OsString;
    use std::io::Write;
    use std::os::unix::ffi::{OsStrExt, OsStringExt};
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    use ashpd::desktop::file_chooser::SelectedFiles;
    use iced::futures::channel::oneshot;

    use super::file_uri_to_path;

    /// The flags that run this binary as the helper asking the portal.
    pub const CHOOSE_FOLDER: &str = "--choose-folder";
    pub const OPEN_URI: &str = "--open-uri";

    /// Asks for a folder, starting in `current`. Cancelling yields `None`.
    pub async fn choose_folder(
        title: String,
        current: Option<PathBuf>,
    ) -> Result<Option<PathBuf>, String> {
        let mut arguments = vec![OsString::from(CHOOSE_FOLDER), OsString::from(title)];
        arguments.extend(current.map(OsString::from));
        let answer = helper(arguments).await?;
        Ok((!answer.is_empty()).then(|| PathBuf::from(OsString::from_vec(answer))))
    }

    /// Opens `uri` in the app the desktop picks for it.
    pub async fn open_uri(uri: String) -> Result<(), String> {
        helper(vec![OsString::from(OPEN_URI), OsString::from(uri)])
            .await
            .map(drop)
    }

    /// What the helper prints, run with `arguments`, once it is done.
    async fn helper(arguments: Vec<OsString>) -> Result<Vec<u8>, String> {
        let (sender, receiver) = oneshot::channel();
        std::thread::spawn(move || {
            let _ = sender.send(run(arguments));
        });
        receiver.await.map_err(|error| error.to_string())?
    }

    #[allow(unsafe_code)]
    fn run(arguments: Vec<OsString>) -> Result<Vec<u8>, String> {
        use std::os::unix::process::CommandExt;
        let program = std::env::current_exe().map_err(|error| error.to_string())?;
        let mut command = Command::new(program);
        command.args(arguments).stdin(Stdio::null());
        // SAFETY: between fork and exec it makes only system calls.
        unsafe {
            command.pre_exec(drop_capabilities);
        }
        let output = command.output().map_err(|error| error.to_string())?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
        }
        let mut answer = output.stdout;
        if answer.last() == Some(&b'\n') {
            answer.pop();
        }
        Ok(answer)
    }

    /// Leaves this process without capabilities, and with no_new_privs, so
    /// the exec that follows does not take the binary's own back.
    #[allow(unsafe_code)]
    fn drop_capabilities() -> std::io::Result<()> {
        // `struct __user_cap_header_struct` and, for version 3's 64 bits,
        // two `struct __user_cap_data_struct`s.
        #[repr(C)]
        struct Header {
            version: u32,
            pid: i32,
        }
        #[repr(C)]
        struct Data {
            effective: u32,
            permitted: u32,
            inheritable: u32,
        }
        const VERSION_3: u32 = 0x2008_0522;
        let header = Header {
            version: VERSION_3,
            pid: 0,
        };
        let none = [
            Data {
                effective: 0,
                permitted: 0,
                inheritable: 0,
            },
            Data {
                effective: 0,
                permitted: 0,
                inheritable: 0,
            },
        ];
        // SAFETY: both point at structs of the layout capset takes.
        if unsafe { libc::syscall(libc::SYS_capset, &header, none.as_ptr()) } != 0 {
            return Err(std::io::Error::last_os_error());
        }
        // SAFETY: PR_SET_NO_NEW_PRIVS takes plain integers.
        if unsafe { libc::prctl(libc::PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0) } != 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }

    /// Runs as the helper, with the arguments after the program: asks the
    /// portal, prints a folder chosen, and returns the exit code.
    pub fn serve(arguments: &[OsString]) -> i32 {
        use iced::futures::executor::block_on;
        let result = match arguments {
            [flag, title, current @ ..] if flag == CHOOSE_FOLDER => block_on(ask_folder(
                title.to_string_lossy().into_owned(),
                current.first().map(PathBuf::from),
            ))
            .and_then(|folder| {
                let Some(folder) = folder else {
                    return Ok(());
                };
                let mut out = std::io::stdout().lock();
                out.write_all(folder.as_os_str().as_bytes())
                    .and_then(|()| out.write_all(b"\n"))
                    .map_err(|error| error.to_string())
            }),
            [flag, uri] if flag == OPEN_URI => block_on(scramble_ui::desktop::open_uri(
                uri.to_string_lossy().into_owned(),
            )),
            _ => Err(format!(
                "usage: triib {CHOOSE_FOLDER} TITLE [FOLDER] | {OPEN_URI} URI"
            )),
        };
        match result {
            Ok(()) => 0,
            Err(error) => {
                eprintln!("{error}");
                1
            }
        }
    }

    async fn ask_folder(
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

    /// Opens `uri` in the app the system picks for it.
    pub async fn open_uri(uri: String) -> Result<(), String> {
        scramble_ui::desktop::open_uri(uri).await
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
