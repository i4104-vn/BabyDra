//! Main entry point for the BabyDra Desktop Context Menu.
//! Initializes theme contexts and maps the popup menu window.

mod widgets;
mod render;

use gtk4::prelude::*;

fn main() {
    let application = gtk4::Application::new(
        Some("org.babydra.menu"),
        Default::default(),
    );

    application.connect_activate(|app| {
        babydra_common::init_theme();
        let window = render::build_menu_ui(app);
        window.present();
    });

    application.run();
}

