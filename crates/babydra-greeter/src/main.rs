//! Main entry point for the BabyDra greeter (greetd display manager).
//! Minimal bootstrap: shared logging, GTK Application, then hand off to the
//! render and handlers modules.

mod auth;
mod handlers;
mod render;
mod theme;
mod widgets;

use gtk4::prelude::*;
use tracing::info;

fn main() {
    // 1. Initialize reusable logging system from babydra-common
    babydra_common::init_logger("babydra-greeter", "displaymanager.log");

    let pid = std::process::id();
    let greetd_sock = std::env::var("GREETD_SOCK").unwrap_or_else(|_| "<NOT SET>".to_string());
    let wayland_display = std::env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "<NOT SET>".to_string());
    let xdg_config = std::env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| "<DEFAULT>".to_string());

    info!(
        target: "babydra-greeter",
        "Process environment: PID={}, GREETD_SOCK={}, WAYLAND_DISPLAY={}, XDG_CONFIG_HOME={}",
        pid, greetd_sock, wayland_display, xdg_config
    );

    info!(target: "babydra-greeter", "Creating GTK Application (id: com.babydra.greeter)");
    let app = gtk4::Application::builder()
        .application_id("com.babydra.greeter")
        .build();

    app.connect_activate(|app| {
        let greeter = render::build_greeter_ui(app);
        handlers::setup_handlers(&greeter);
        greeter.window.present();
    });

    info!(target: "babydra-greeter", "Starting GTK Application main loop");
    app.run();
    info!(target: "babydra-greeter", "GTK Application main loop terminated");
}
