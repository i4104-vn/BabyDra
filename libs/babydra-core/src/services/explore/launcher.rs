//! Desktop-integration launcher for opening babydra-explore windows.

use std::path::Path;
use std::process::Command;

#[zbus::proxy(
    gen_blocking = true,
    interface = "org.freedesktop.FileManager1",
    default_service = "org.freedesktop.FileManager1",
    default_path = "/org/freedesktop/FileManager1"
)]
trait FileManager1 {
    fn show_items(&self, uris: Vec<&str>, startup_id: &str) -> zbus::Result<()>;
    fn show_folders(&self, uris: Vec<&str>, startup_id: &str) -> zbus::Result<()>;
}

/// Opens a directory in a new babydra-explore window, preferring the
/// user-local binary, then the currently running executable, then PATH.
pub fn spawn_explore_window(path: &Path) {
    let home = crate::services::utils::get_home_dir();
    let local_bin = format!("{}/.local/bin/babydra-explore", home);
    if Path::new(&local_bin).exists() && Command::new(&local_bin).arg(path).spawn().is_ok() {
        return;
    }
    if let Ok(exe) = std::env::current_exe() {
        if exe.file_name().and_then(|n| n.to_str()) == Some("babydra-explore")
            && Command::new(exe).arg(path).spawn().is_ok()
        {
            return;
        }
    }
    let _ = Command::new("babydra-explore").arg(path).spawn();
}

/// Attempts to forward the path to an already running FileManager1 instance via D-Bus.
/// Returns true if successfully delivered, false if no running instance was found or call failed.
pub fn try_open_in_running_instance(path: &Path) -> bool {
    let uri = if path.to_string_lossy().starts_with("file://") {
        path.to_string_lossy().to_string()
    } else {
        let abs = if path.is_relative() {
            std::env::current_dir()
                .map(|cwd| cwd.join(path))
                .unwrap_or_else(|_| path.to_path_buf())
        } else {
            path.to_path_buf()
        };
        format!("file://{}", abs.to_string_lossy())
    };

    if let Ok(conn) = zbus::blocking::Connection::session() {
        let has_owner = conn
            .call_method(
                Some("org.freedesktop.DBus"),
                "/org/freedesktop/DBus",
                Some("org.freedesktop.DBus"),
                "NameHasOwner",
                &("org.freedesktop.FileManager1",),
            )
            .map(|reply| reply.body().deserialize::<bool>().unwrap_or(false))
            .unwrap_or(false);

        if has_owner {
            if let Ok(proxy) = FileManager1ProxyBlocking::new(&conn) {
                let res = if path.is_dir() {
                    proxy.show_folders(vec![&uri], "")
                } else {
                    proxy.show_items(vec![&uri], "")
                };
                if res.is_ok() {
                    crate::services::window::jump_to_app("babydra-explore", None);
                    return true;
                }
            }
        }
    }
    false
}

/// Opens the file manager, navigating to the file's parent directory and selecting the file.
/// If `path` is a directory, it navigates into that directory.
/// If babydra-explore is already running, sends a D-Bus request and switches workspace to it.
/// If not running, launches babydra-explore with the path as an argument.
pub fn show_in_file_manager(path: &Path) {
    if !try_open_in_running_instance(path) {
        spawn_explore_window(path);
    }
}

