//! Controller wrappers for individual launcher application row and grid item buttons.

use babydra_core::DesktopApp;
use gtk4::prelude::*;
use std::process::Command;

mod render;

/// Creates a grid layout application button widget, binding its click event to launch the app.
pub fn create_grid_app(app: &DesktopApp, window: &gtk4::ApplicationWindow) -> gtk4::Button {
    let (btn, _, _) = render::build_grid_app_ui(app);

    let exec_cmd = app.exec.clone();
    let win_to_close = window.clone();
    btn.connect_clicked(move |_| {
        let parts: Vec<&str> = exec_cmd.split_whitespace().collect();
        if !parts.is_empty() {
            let program = parts[0];
            let args = &parts[1..];
            let _ = Command::new(program).args(args).spawn();
        }

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

/// Creates a list row application button widget, binding its click event to launch the app.
pub fn create_list_app(app: &DesktopApp, window: &gtk4::ApplicationWindow) -> gtk4::Button {
    let (btn, _, _) = render::build_list_app_ui(app);

    let exec_cmd = app.exec.clone();
    let win_to_close = window.clone();
    btn.connect_clicked(move |_| {
        let parts: Vec<&str> = exec_cmd.split_whitespace().collect();
        if !parts.is_empty() {
            let program = program_part(parts[0]);
            let args = &parts[1..];
            let _ = Command::new(program).args(args).spawn();
        }

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

/// Strip any field code specifiers from Exec fields (e.g. %u, %U, %f, %F).
fn program_part(raw: &str) -> &str {
    raw.trim_end_matches('%')
}
