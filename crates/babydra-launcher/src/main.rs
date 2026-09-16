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

    let launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>> = Rc::new(RefCell::new(None));
    let launcher_window_for_activate = launcher_window.clone();

    app.connect_activate(move |app| {
        babydra_ui_kit::ui::theme::init_theme();
        if let Some(window) = launcher_window_for_activate.borrow().as_ref() {
            window.present();
            return;
        }

        let window = babydra_launcher::build_launcher_ui(app, launcher_window_for_activate.clone());
        window.present();
        *launcher_window_for_activate.borrow_mut() = Some(window);
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}
