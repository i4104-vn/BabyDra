//! babydra-panel — Top bar panel with system tray, clock, and control center.

mod render;
mod widgets;

use gtk4::prelude::*;
use gtk4::Application;
use std::cell::RefCell;
use std::rc::Rc;

/// Application entry point: `main`.
fn main() {
    babydra_core::services::logger::init_logger("babydra-panel", "babydra-panel.log");

    // Initialize D-Bus StatusNotifierWatcher system tray listener daemon
    babydra_core::tray::spawn_watcher();

    // Detect DDC/CI bus for desktop monitors on startup
    widgets::panel::detect_ddc_bus();

    // Spawn a background thread to refresh desktop apps cache asynchronously on startup
    std::thread::spawn(|| {
        babydra_core::refresh_desktop_apps();
    });

    babydra_core::spawn_switcher();

    let app = Application::builder()
        .application_id("org.babydra.panel")
        .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE)
        .build();

    app.connect_activate(|app| {
        // Initialize style provider
        babydra_ui_kit::ui::theme::init_theme();

        // Sync system color-scheme changes (GSettings) to GTK settings in real-time
        let gsettings = gtk4::gio::Settings::new("org.gnome.desktop.interface");
        gsettings.connect_changed(Some("color-scheme"), |_, _| {
            babydra_ui_kit::ui::theme::init_theme();
        });

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
