//! babydra-screenshot — Regional and fullscreen screen capture with annotation editor.

mod widgets;

use babydra_core::{capture_screen_to_temp, handle_fullscreen_capture};
use gtk4::prelude::*;
use gtk4::Application;
use std::env;
use widgets::editor::build_editor_ui;

/// Application entry point: `main`.
fn main() {
    babydra_core::services::logger::init_logger("babydra-screenshot", "babydra-screenshot.log");

    let args: Vec<String> = env::args().collect();

    // 1. Handle Fullscreen Immediate Capture
    if args.contains(&"--full".to_string()) {
        handle_fullscreen_capture();
        return;
    }

    // 2. Interactive Regional Capture (Default)
    let temp_path = match capture_screen_to_temp() {
        Some(path) => path,
        None => return,
    };

    let temp_path_for_activate = temp_path.clone();
    let temp_path_for_cleanup = temp_path.clone();

    let app = Application::builder()
        .application_id("org.babydra.screenshot")
        .build();

    app.connect_activate(move |app| {
        babydra_ui_kit::ui::theme::init_theme();
        let window = build_editor_ui(app, &temp_path_for_activate);
        window.present();
    });

    let exit_code = app.run().value();

    // Clean up temporary screenshot file on exit
    std::fs::remove_file(&temp_path_for_cleanup).ok();

    std::process::exit(exit_code);
}
