//! Main entry point for the BabyDra Alt-Tab window switcher.
//! Handles single-instance Unix sockets to cycle through open windows on subsequent keypresses.

use gtk4::prelude::*;
use std::os::unix::net::UnixStream;
use std::io::Write;

mod history;
mod apps;
mod widgets;
mod render;

use apps::get_running_apps;

/// Connects to the active switcher instance socket to cycle the selection.
/// Returns true if no other instance is running and this process should start the GUI.
fn handle_single_instance() -> bool {
    let socket_path = "/tmp/babydra-switcher.socket";
    
    if let Ok(mut stream) = UnixStream::connect(socket_path) {
        let _ = stream.write_all(b"next");
        return false;
    }
    
    let _ = std::fs::remove_file(socket_path);
    true
}

fn main() {
    if !handle_single_instance() {
        return;
    }

    let apps = get_running_apps();
    if apps.is_empty() {
        return;
    }

    let application = gtk4::Application::new(
        Some("org.babydra.switcher"),
        Default::default(),
    );

    let apps_clone = apps.clone();
    application.connect_activate(move |app| {
        let apps = apps_clone.clone();
        render::build_switcher_ui(app, apps);
    });

    application.run();

    std::fs::remove_file("/tmp/babydra-switcher.socket").ok();
}

