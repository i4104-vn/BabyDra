pub mod widgets;

use babydra_core::SessionState;
use gtk4::prelude::*;
use gtk4::Application;

/// Application entry point: `main`.
fn main() {
    babydra_core::services::logger::init_logger("babydra-explore", "babydra-explore.log");

    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    let app = Application::builder()
        .application_id("org.babydra.explore")
        .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE | gtk4::gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_activate(|app| {
        let (target_dir, focus_item) = babydra_ui_kit::components::explore::parse_target_dir();

        let session = std::rc::Rc::new(std::cell::RefCell::new(SessionState::new(target_dir)));
        let main_window = crate::widgets::window::create_explore_win(app, session, focus_item);
        main_window.present();
    });

    app.connect_open(|app, files, _hint| {
        let mut target_dir = glib::home_dir();
        let mut focus_item = None;

        if let Some(file) = files.first() {
            if let Some(path) = file.path() {
                if path.is_dir() {
                    target_dir = path;
                } else if path.is_file() {
                    if let Some(parent) = path.parent() {
                        target_dir = parent.to_path_buf();
                        focus_item = Some(path);
                    }
                }
            } else {
                let uri = file.uri();
                let decoded = babydra_core::mpris::decode_uri(&uri.as_str().replace("file://", ""));
                let path = std::path::PathBuf::from(decoded);
                if path.is_dir() {
                    target_dir = path;
                } else if path.is_file() {
                    if let Some(parent) = path.parent() {
                        target_dir = parent.to_path_buf();
                        focus_item = Some(path);
                    }
                }
            }
        }

        let session = std::rc::Rc::new(std::cell::RefCell::new(SessionState::new(target_dir)));
        let main_window = crate::widgets::window::create_explore_win(app, session, focus_item);
        main_window.present();
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
