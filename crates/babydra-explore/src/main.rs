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
        .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE)
        .build();

    app.connect_activate(|app| {
        let target_dir = babydra_ui_kit::components::explore::parse_target_dir();

        let session = std::rc::Rc::new(std::cell::RefCell::new(SessionState::new(target_dir)));

        let main_window = crate::widgets::window::create_explore_win(app, session);
        main_window.present();
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
