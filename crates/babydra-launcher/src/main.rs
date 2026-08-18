//! babydra-launcher — Application launcher overlay window.

use gtk4::prelude::*;
use gtk4::Application;
use std::cell::RefCell;
use std::rc::Rc;

/// Application entry point: `main`.
fn main() {
    babydra_core::services::logger::init_logger("babydra-launcher", "babydra-launcher.log");

    let app = Application::builder()
        .application_id("org.babydra.launcher")
        .build();

    app.connect_activate(|app| {
        babydra_ui_kit::ui::theme::init_theme();
        let launcher_window = Rc::new(RefCell::new(None));
        let window = babydra_launcher::build_launcher_ui(app, launcher_window.clone());
        window.present();
        *launcher_window.borrow_mut() = Some(window);
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
