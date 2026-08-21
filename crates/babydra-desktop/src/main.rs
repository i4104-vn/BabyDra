//! babydra-desktop — Desktop shell providing wallpaper rendering, icon grid management on ~/Desktop, and desktop context menus.

mod render;
pub mod state;
pub mod widgets;

use gtk4::prelude::*;
use gtk4::Application;

/// Application entry point: `main`.
fn main() {
    babydra_core::services::logger::init_logger("babydra-desktop", "babydra-desktop.log");

    let rt = tokio::runtime::Runtime::new().expect("Failed to initialize Tokio runtime");
    let _guard = rt.enter();

    let app = Application::builder()
        .application_id("org.babydra.desktop")
        .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE)
        .build();

    app.connect_activate(|app| {
        // Initialize BabyDra theme and styling
        babydra_ui_kit::ui::theme::init_theme();

        // Apply saved display monitor settings (resolution, refresh rate, orientation)
        babydra_core::services::system::display::apply_saved_displays();

        // Sync system color-scheme changes (GSettings) to GTK settings in real-time
        let gsettings = gtk4::gio::Settings::new("org.gnome.desktop.interface");
        gsettings.connect_changed(Some("color-scheme"), |_, _| {
            babydra_ui_kit::ui::theme::init_theme();
        });

        // Build and display desktop window on Wayland Background layer
        let window = render::build_desktop_window(app);
        window.present();
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
