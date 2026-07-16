mod widgets;
mod render;

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // Initialize D-Bus StatusNotifierWatcher system tray listener daemon
    babydra_common::desktop::tray::spawn_watcher_service();

    // Detect DDC/CI bus for desktop monitors on startup
    widgets::panel::detect_ddc_bus();

    // Spawn a background thread to refresh desktop apps cache asynchronously on startup
    std::thread::spawn(|| {
        babydra_common::refresh_desktop_apps_cache();
    });

    // Spawn background thread to track focused app and capture screenshots
    babydra_common::spawn_switcher_tracker();

    let application = gtk4::Application::new(
        Some("org.babydra.panel"),
        Default::default(),
    );

    application.connect_activate(|app| {
        // Initialize style provider
        babydra_utils::ui::theme::init_theme();

        // Sync system color-scheme changes (GSettings) to GTK settings in real-time
        let gsettings = gtk4::gio::Settings::new("org.gnome.desktop.interface");
        gsettings.connect_changed(Some("color-scheme"), |_, _| {
            babydra_utils::ui::theme::init_theme();
        });

        // Define shared window states for mutual exclusivity
        let control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>> = Rc::new(RefCell::new(None));
        let calendar_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>> = Rc::new(RefCell::new(None));
        let launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>> = Rc::new(RefCell::new(None));

        // Build panel UI via render module
        let window = render::build_panel_ui(
            app,
            control_center_window,
            calendar_window,
            launcher_window,
        );

        // Display the window on Wayland
        window.present();
    });

    // Run the GTK loop (this blocks until application exits)
    application.run();
}
