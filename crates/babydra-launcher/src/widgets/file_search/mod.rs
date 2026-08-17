//! Controller actions and local index crawling utilities for file queries.

use gtk4::prelude::*;
use std::path::Path;
use std::process::Command;

mod render;

/// Creates a clickable button row representing a discovered local file.
/// Binds clicking the button to launch the file using `xdg-open`.
pub fn create_file_row(path: &Path, window: &gtk4::ApplicationWindow) -> gtk4::Button {
    let (btn, _, _) = render::build_file_row_ui(path);
    let path_str = path.to_string_lossy().to_string();

    let win_to_close = window.clone();
    btn.connect_clicked(move |_| {
        let _ = Command::new("xdg-open").arg(&path_str).spawn();

        win_to_close.close();
    });

    let motion = gtk4::EventControllerMotion::new();
    let btn_clone = btn.clone();
    motion.connect_enter(move |_, _, _| {
        btn_clone.grab_focus();
    });
    btn.add_controller(motion);

    btn
}
