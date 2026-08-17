//! Main entry point for the BabyDra Screen Locker.
//! Sets up GTK Application, parses command line arguments for a custom wallpaper,
//! initializes theme context, and maps locker windows to all connected monitors.

mod render;
mod widgets;

use gtk4::prelude::*;

/// Application entry point: `main`.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut custom_image = None;
    if args.len() > 1 {
        let mut i = 1;
        while i < args.len() {
            if (args[i] == "--image" || args[i] == "-i") && i + 1 < args.len() {
                custom_image = Some(args[i + 1].clone());
                i += 2;
            } else if !args[i].starts_with('-') {
                custom_image = Some(args[i].clone());
                i += 1;
            } else {
                i += 1;
            }
        }
    }

    let application = gtk4::Application::new(Some("org.babydra.lock"), Default::default());

    application.connect_activate(move |app| {
        babydra_ui_kit::ui::theme::init_theme();
        render::build_lock_ui(app, custom_image.as_deref());
    });

    application.run_with_args::<&str>(&[]);
}
