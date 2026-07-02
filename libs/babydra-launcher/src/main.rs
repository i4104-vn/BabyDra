use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Starting BabyDra Launcher...");

    let application = gtk4::Application::new(
        Some("org.babydra.launcher"),
        Default::default(),
    );

    application.connect_activate(|app| {
        babydra_common::init_theme();
        let launcher_window = Rc::new(RefCell::new(None));
        let window = babydra_launcher::build_launcher_ui(app, launcher_window.clone());
        window.present();
        *launcher_window.borrow_mut() = Some(window);
    });

    application.run();
}
