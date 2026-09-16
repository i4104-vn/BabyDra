//! babydra-panel — Top bar panel with system tray, clock, and control center.

mod render;
mod widgets;

use gtk4::prelude::*;
use gtk4::Application;
use std::cell::RefCell;
use std::rc::Rc;

/// Application entry point: `main`.
fn main() {
    // Workspace shortcuts use the panel binary as their lightweight CLI.
    // This keeps workspace state and UI integration in one application.
    if std::env::args().nth(1).is_some() {
        babydra_core::run_workspace_cli();
        return;
    }

    let _lifecycle = babydra_core::services::app_lifecycle::init_app("babydra-panel");

    // Detect DDC/CI bus for desktop monitors on startup
    widgets::panel::detect_ddc_bus();

    // Spawn org.freedesktop.Notifications DBus daemon immediately on startup
    babydra_core::services::notification::spawn_notif_dbus();

    let app = Application::builder()
        .application_id("org.babydra.panel")
        .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE)
        .build();

    app.connect_activate(|app| {
        // Initialize style provider (already done in init_app, but safe to call again)
        babydra_ui_kit::ui::theme::init_theme();

        // Define shared window states for mutual exclusivity
        let control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>> =
            Rc::new(RefCell::new(None));
        let calendar_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>> =
            Rc::new(RefCell::new(None));
        let launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>> =
            Rc::new(RefCell::new(None));

        let window =
            render::build_panel_ui(app, control_center_window, calendar_window, launcher_window);

        // Display the window on Wayland
        window.present();
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
