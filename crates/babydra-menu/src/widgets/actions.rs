//! Action triggers executed when clicking context menu options.

use gtk4::prelude::*;

pub use babydra_common::desktop::actions::{
    execute_terminal,
    execute_file_manager,
    execute_reconfigure_shell,
    execute_exit_shell,
};

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
                let _ = babydra_common::set_wallpaper(&path);
            }
        }
    });
}
