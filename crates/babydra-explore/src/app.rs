use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use babydra_common::SessionState;

pub struct BabyExploreApp {
    app: Application,
}

impl BabyExploreApp {
    pub fn new() -> Self {
        let app = Application::builder()
            .application_id("org.babydra.explore")
            .build();

        Self { app }
    }

    pub fn run(&self) -> i32 {
        self.app.connect_activate(|app| {
            let window = ApplicationWindow::builder()
                .application(app)
                .title("BabyDra Explore")
                .default_width(1000)
                .default_height(700)
                .build();

            // Setup temporary state
            let home_dir = glib::home_dir();
            let _session = SessionState::new(home_dir);

            // TODO: Initialize MainWindow UI and bind session state here
            
            window.present();
        });

        self.app.run().value()
    }
}
