//! Native Arch Linux settings manager built with GTK4 + Rust.

use gtk4::prelude::*;

mod layout;
mod widgets;

fn main() {
    let app = gtk4::Application::new(
        Some("com.babydra.settings"),
        Default::default(),
    );

    app.connect_activate(move |app| {
        // Load custom styles
        babydra_utils::ui::theme::init_theme();

        // Build and display main settings window
        layout::build_main_window(app);
    });

    app.run_with_args(&["babydra-settings"]);
}
