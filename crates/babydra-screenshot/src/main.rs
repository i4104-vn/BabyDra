use gtk4::prelude::*;
use std::env;

mod capture;
mod widgets;
mod render;

use capture::{capture_screen_to_temp, get_screenshot_save_path};
use render::build_editor_ui;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // 1. Handle Fullscreen Immediate Capture
    if args.contains(&"--full".to_string()) {
        if let Some(temp_path) = capture_screen_to_temp() {
            let save_path = get_screenshot_save_path();
            if std::fs::copy(&temp_path, &save_path).is_ok() {
                let notif_title = babydra_common::i18n::t("screenshot.full_saved_title");
                let notif_msg = babydra_common::i18n::t("screenshot.saved_msg")
                    .replace("{}", &format!("{:?}", save_path));
                
                let _ = std::process::Command::new("notify-send")
                    .args(&["-i", "image-x-generic", &notif_title, &notif_msg])
                    .spawn();
            }
            let _ = std::fs::remove_file(temp_path);
        }
        return;
    }

    // 2. Interactive Regional Capture (Default)
    let temp_path = match capture_screen_to_temp() {
        Some(path) => path,
        None => return,
    };

    let temp_path_for_activate = temp_path.clone();
    let temp_path_for_cleanup = temp_path.clone();

    let application = gtk4::Application::new(
        Some("org.babydra.screenshot"),
        Default::default(),
    );

    application.connect_activate(move |app| {
        babydra_common::init_theme();
        let window = build_editor_ui(app, &temp_path_for_activate);
        window.present();
    });

    application.run();

    // Clean up temporary screenshot file on exit
    std::fs::remove_file(&temp_path_for_cleanup).ok();
}
