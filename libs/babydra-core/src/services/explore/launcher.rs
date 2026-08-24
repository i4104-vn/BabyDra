//! Desktop-integration launchers (background shell commands, MIME defaults,
//! opening files/directories with the system handler or a new explore window).
//! Keeps process spawning out of the UI layer.

use std::path::Path;
use std::process::Command;

/// Spawns a background shell command (`sh -c "<script>"`), ignoring errors.
pub fn spawn_sh_background(script: String) {
    let _ = Command::new("sh").arg("-c").arg(script).spawn();
}

/// Registers `desktop_name` as the default handler for `mime_type` via xdg-mime.
pub fn set_default_mime_handler(desktop_name: &str, mime_type: &str) {
    let _ = Command::new("xdg-mime")
        .arg("default")
        .arg(desktop_name)
        .arg(mime_type)
        .spawn();
}

/// Opens `path` with the platform default handler.
pub fn open_with_system(path: &Path) {
    let _ = Command::new("xdg-open").arg(path).spawn();
}

/// Opens a directory in a new babydra-explore window, preferring the
/// user-local binary, then the currently running executable, then PATH.
pub fn spawn_explore_window(path: &Path) {
    if let Ok(home) = std::env::var("HOME") {
        let local_bin = format!("{}/.local/bin/babydra-explore", home);
        if Path::new(&local_bin).exists() && Command::new(&local_bin).arg(path).spawn().is_ok() {
            return;
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if Command::new(exe).arg(path).spawn().is_ok() {
            return;
        }
    }
    let _ = Command::new("babydra-explore").arg(path).spawn();
}
