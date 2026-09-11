//! Native Arch Linux settings manager built with GTK4 + Rust.

use gtk4::prelude::*;

mod layout;
mod widgets;

/// Application entry point: `main`.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (should_exit, opts) = babydra_core::services::cli::parse_cli_args(&args);
    if should_exit {
        if let Some(action) = opts.action {
            babydra_core::services::cli::execute_cli_action(action);
        }
        return;
    }

    let _lifecycle = babydra_core::services::app_lifecycle::init_app("babydra-settings");

    let app = gtk4::Application::new(
        Some("com.babydra.settings"),
        gtk4::gio::ApplicationFlags::NON_UNIQUE,
    );

    let initial_page = opts.page.map(|p| p.as_str().to_string());
    app.connect_activate(move |app| {
        babydra_ui_kit::ui::theme::init_theme();

        layout::build_main_window(app, initial_page.as_deref());
    });

    app.run_with_args(&["babydra-settings"]);
}
