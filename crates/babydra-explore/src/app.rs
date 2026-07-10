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
            let home_dir = glib::home_dir();
            let session = std::rc::Rc::new(std::cell::RefCell::new(SessionState::new(home_dir)));
            
            let main_window = crate::ui::window::MainWindow::new(app, session);
            main_window.show();

            // Setup D-Bus channel for navigation requests
            let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<std::path::PathBuf>();

            let win_clone = main_window.clone();
            glib::MainContext::default().spawn_local(async move {
                while let Some(path) = rx.recv().await {
                    win_clone.navigate_to(path);
                }
            });

            // Start D-Bus service in tokio background thread
            tokio::spawn(async move {
                if let Err(e) = babydra_common::start_dbus_service(tx).await {
                    eprintln!("Failed to start D-Bus service: {}", e);
                }
            });
        });

        self.app.run().value()
    }
}
