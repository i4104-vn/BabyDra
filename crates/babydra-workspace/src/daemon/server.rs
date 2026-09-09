use crate::daemon::client::WORKSPACE_SOCKET_PATH;
use crate::switcher::build_workspace_switcher_ui;
use gtk4::glib;
use gtk4::prelude::*;
use std::io::Read;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc;
use std::thread;

/// Runs the background workspace daemon process.
pub fn run_daemon() {
    if UnixStream::connect(WORKSPACE_SOCKET_PATH).is_ok() {
        eprintln!("[babydra-workspace] Daemon already running.");
        return;
    }

    let _ = std::fs::remove_file(WORKSPACE_SOCKET_PATH);

    let app = gtk4::Application::new(Some("org.babydra.workspace.daemon"), Default::default());
    app.connect_activate(|app| {
        let controller = build_workspace_switcher_ui(app);
        let show_fn = std::rc::Rc::new(controller.show_fn);
        let hide_fn = std::rc::Rc::new(controller.hide_fn);
        let window = controller.window;

        let (tx, rx) = mpsc::channel::<String>();

        // Socket listener thread
        thread::spawn(move || {
            let listener = match UnixListener::bind(WORKSPACE_SOCKET_PATH) {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("[babydra-workspace daemon] Socket bind error: {e}");
                    return;
                }
            };

            for stream in listener.incoming() {
                if let Ok(mut s) = stream {
                    let mut buf = [0u8; 64];
                    if let Ok(n) = s.read(&mut buf) {
                        let msg = String::from_utf8_lossy(&buf[..n]).trim().to_string();
                        let _ = tx.send(msg);
                    }
                }
            }
        });

        // Message pump in GTK main thread
        let show_c = show_fn.clone();
        let hide_c = hide_fn.clone();
        let win_c = window.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(20), move || {
            while let Ok(msg) = rx.try_recv() {
                match msg.as_str() {
                    "show" => {
                        show_c();
                    }
                    "hide" => {
                        hide_c();
                    }
                    "toggle" => {
                        if win_c.is_visible() {
                            hide_c();
                        } else {
                            show_c();
                        }
                    }
                    _ => {}
                }
            }
            glib::ControlFlow::Continue
        });
    });

    app.run_with_args(&["babydra-workspace"]);
}
