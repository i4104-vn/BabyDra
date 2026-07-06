//! Action triggers executed when clicking context menu options.

use gtk4::prelude::*;
use std::process::Command;

/// Launches the default system terminal emulator.
pub fn execute_terminal() {
    let _ = Command::new("foot").spawn().or_else(|_| Command::new("alacritty").spawn());
}

/// Launches the default system file manager GUI.
pub fn execute_file_manager() {
    let _ = Command::new("pcmanfm").spawn().or_else(|_| Command::new("thunar").spawn());
}

/// Opens a file chooser dialog to select and apply a new system desktop wallpaper.
pub fn execute_change_wallpaper(window: &gtk4::ApplicationWindow) {
    let dialog = gtk4::FileDialog::new();
    dialog.set_title("Select Wallpaper Image");
    
    let filter = gtk4::FileFilter::new();
    filter.set_name(Some("Images"));
    filter.add_mime_type("image/png");
    filter.add_mime_type("image/jpeg");
    dialog.set_default_filter(Some(&filter));

    let win = window.clone();
    dialog.open(Some(&win), None::<&gio::Cancellable>, move |res| {
        if let Ok(file) = res {
            if let Some(path) = file.path() {
                let _ = babydra_wallpaper::set_wallpaper(&path);
            }
        }
    });
}

/// Signals the compositor window manager to reload configurations.
pub fn execute_reconfigure_shell() {
    let _ = Command::new("labwc").arg("--reconfigure").spawn();
}

/// Exits the graphical Wayland shell session.
pub fn execute_exit_shell() {
    let _ = Command::new("labwc").arg("--exit").spawn();
}

