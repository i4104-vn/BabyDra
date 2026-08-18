//! Action triggers executed when clicking context menu options or launching default apps.

use std::process::Command;

/// Launches the default system terminal emulator.
pub fn execute_terminal() {
    let _ = Command::new("foot")
        .spawn()
        .or_else(|_| Command::new("alacritty").spawn());
}

/// Launches the default system file manager GUI.
pub fn execute_file_manager() {
    let _ = Command::new("pcmanfm")
        .spawn()
        .or_else(|_| Command::new("thunar").spawn());
}

/// Signals the compositor window manager to reload configurations.
pub fn reconfigure_shell() {
    let _ = Command::new("labwc").arg("--reconfigure").spawn();
}

/// Exits the graphical Wayland shell session.
pub fn execute_exit_shell() {
    let _ = Command::new("labwc").arg("--exit").spawn();
}
