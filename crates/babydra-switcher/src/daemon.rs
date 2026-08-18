//! Daemon lifecycle management for babydra-switcher.
//! Keeps a single GTK instance alive and shows/hides the window on demand
//! via a Unix socket, eliminating the cold-start penalty of spawning a new process.

use std::io::Read;
use std::os::unix::net::UnixListener;
use std::sync::mpsc::{Receiver, Sender};

pub use babydra_core::models::shell::DaemonMessage;

/// Spawns the background thread that listens on the Unix socket and forwards
/// messages to the GTK main thread via the provided channel.
pub fn spawn_socket(socket_path: &str, tx: Sender<DaemonMessage>) {
    let socket_path = socket_path.to_string();
    std::thread::spawn(move || {
        loop {
            // Remove stale socket from previous run
            let _ = std::fs::remove_file(&socket_path);

            match UnixListener::bind(&socket_path) {
                Ok(listener) => {
                    for stream in listener.incoming() {
                        if let Ok(mut stream) = stream {
                            let mut buf = [0u8; 8];
                            if let Ok(n) = stream.read(&mut buf) {
                                let msg = &buf[..n];
                                if msg == b"show" || msg == b"next" {
                                    let _ = tx.send(DaemonMessage::ShowOrNext);
                                } else if msg == b"hide" {
                                    let _ = tx.send(DaemonMessage::Hide);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[babydra-switcher daemon] Socket bind error: {e}");
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            }
        }
    });
}

/// Polls the daemon message receiver on the GTK main thread using a periodic timer.
/// Calls `on_show_or_next` or `on_hide` as appropriate.
pub fn setup_message_pump(
    rx: Receiver<DaemonMessage>,
    on_show_or_next: impl Fn() + 'static,
    on_hide: impl Fn() + 'static,
) {
    let rx = std::sync::Mutex::new(rx);
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(8), move || {
        if let Ok(lock) = rx.try_lock() {
            while let Ok(msg) = lock.try_recv() {
                match msg {
                    DaemonMessage::ShowOrNext => on_show_or_next(),
                    DaemonMessage::Hide => on_hide(),
                }
            }
        }
        gtk4::glib::ControlFlow::Continue
    });
}
