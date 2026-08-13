//! Main entry point for the BabyDra Screen Locker.
//! Sets up GTK Application, parses command line arguments for a custom wallpaper,
//! initializes theme context, and maps locker windows to all connected monitors.

mod widgets;
mod render;

use gtk4::prelude::*;

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

    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string());
    let wallpaper_path = if let Some(ref path) = custom_image {
        if std::path::Path::new(path).exists() {
            path.clone()
        } else {
            format!("{}/.babydra/wallpaper.png", home)
        }
    } else {
        format!("{}/.babydra/wallpaper.png", home)
    };

    let application = gtk4::Application::new(
        Some("org.babydra.lock"),
        Default::default(),
    );

    application.connect_activate(move |app| {
        babydra_utils::ui::theme::init_theme();
        render::build_lock_ui(app, &wallpaper_path);
    });

    application.run_with_args::<&str>(&[]);
}
