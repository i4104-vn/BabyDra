//! Dynamic desktop image preview application.
//! Base Rust + GTK4 image viewer entry point.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::Application;
use std::path::PathBuf;

mod widgets;

/// Application entry point: `main`.
fn main() {
    babydra_core::services::logger::init_logger("babydra-preview", "babydra-preview.log");

    let app = Application::builder()
        .application_id("com.babydra.preview")
        .build();

    app.connect_activate(|app| {
        babydra_ui_kit::ui::theme::init_theme();

        let arg_path = std::env::args().nth(1);
        if let Some(p) = arg_path {
            let path = PathBuf::from(p);
            if path.exists() {
                widgets::build_ui(app, path);
                return;
            }
        }

        // Fallback file selector if no path is given or if the path is invalid
        let fallback_window = gtk4::ApplicationWindow::new(app);
        fallback_window.set_title(Some(&trans("common.app_preview_title")));
        fallback_window.set_default_size(400, 200);

        let file_dialog = gtk4::FileDialog::new();
        file_dialog.set_title(&trans("common.open_image_file"));

        let filter = gtk4::FileFilter::new();
        filter.set_name(Some(&trans("common.images_filter")));
        filter.add_mime_type("image/png");
        filter.add_mime_type("image/jpeg");
        filter.add_mime_type("image/webp");
        file_dialog.set_default_filter(Some(&filter));

        let app_clone = app.clone();
        let fallback_win_clone = fallback_window.clone();
        file_dialog.open(
            Some(&fallback_window),
            None::<&gio::Cancellable>,
            move |res| {
                if let Ok(file) = res {
                    if let Some(path) = file.path() {
                        widgets::build_ui(&app_clone, path);
                        fallback_win_clone.close();
                    } else {
                        fallback_win_clone.close();
                    }
                } else {
                    fallback_win_clone.close();
                }
            },
        );

        fallback_window.present();
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
