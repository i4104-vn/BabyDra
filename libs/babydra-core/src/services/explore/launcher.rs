//! Desktop-integration launcher for opening babydra-explore windows.

use std::path::Path;
use std::process::Command;

/// Opens a directory in a new babydra-explore window, preferring the
/// user-local binary, then the currently running executable, then PATH.
pub fn spawn_explore_window(path: &Path) {
    let home = crate::services::utils::get_home_dir();
    let local_bin = format!("{}/.local/bin/babydra-explore", home);
    if Path::new(&local_bin).exists() && Command::new(&local_bin).arg(path).spawn().is_ok() {
        return;
    }
    if let Ok(exe) = std::env::current_exe() {
        if Command::new(exe).arg(path).spawn().is_ok() {
            return;
        }
    }
    let _ = Command::new("babydra-explore").arg(path).spawn();
}
