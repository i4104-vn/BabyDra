//! Widgets coordinator module for BabyDra Notepad.

use gtk4::Application;
use std::path::PathBuf;

pub mod editor;
pub mod settings_dialog;

/// Opens the editor with the specified file on disk.
pub fn build_ui(app: &Application, path: PathBuf) {
    editor::build_ui(app, path);
}

/// Opens the editor with a new blank document.
pub fn build_ui_new(app: &Application) {
    editor::build_ui_new(app);
}
