pub mod widgets;

use gtk4::prelude::*;
use gtk4::Application;
use babydra_common::SessionState;

fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _guard = rt.enter();

    let app = Application::builder()
        .application_id("org.babydra.explore")
        .build();

    app.connect_activate(|app| {
        let mut target_dir = glib::home_dir();

        if let Some(arg) = std::env::args().nth(1) {
            let path_str = if arg.starts_with("file://") {
                babydra_common::desktop::mpris::decode_uri(&arg.replacen("file://", "", 1))
            } else {
                arg
            };
            let path = std::path::PathBuf::from(path_str);
            if path.exists() {
                target_dir = path;
            }
        }

        let session = std::rc::Rc::new(std::cell::RefCell::new(SessionState::new(target_dir)));
        
        let main_window = crate::widgets::window::create_explore_window(app, session);
        main_window.present();
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
