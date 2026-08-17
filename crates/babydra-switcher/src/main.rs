//! Main entry point for the BabyDra Alt-Tab window switcher.
//!
//! Two modes of operation:
//!
//! ## Daemon mode (`babydra-switcher --daemon`)
//! Starts a persistent GTK process that keeps the overlay window in memory.
//! On each Alt+Tab keypress, the hotkey daemon sends `show` or `next` over the
//! Unix socket — response is near-instant because there is no cold start.
//!
//! ## Legacy one-shot mode (no flag)
//! Previous behaviour: spawns, shows window, exits.  Used as a fallback or for
//! testing without the daemon.

use gtk4::prelude::*;
use std::io::Write;
use std::os::unix::net::UnixStream;

mod daemon;
mod render;
mod widgets;

const SOCKET_PATH: &str = "/tmp/babydra-switcher.socket";

/// Tries to send a message to an already-running daemon.
/// Returns `true` if a daemon was reached (caller should exit), `false` if no daemon running.
fn try_signal_daemon(msg: &[u8]) -> bool {
    if let Ok(mut stream) = UnixStream::connect(SOCKET_PATH) {
        let _ = stream.write_all(msg);
        return true;
    }
    false
}

/// Application entry point: `main`.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let is_daemon = args.iter().any(|a| a == "--daemon");

    if is_daemon {
        run_daemon();
    } else {
        run_oneshot();
    }
}

// ---------------------------------------------------------------------------
// Daemon mode
// ---------------------------------------------------------------------------

/// Runs `daemon`.
fn run_daemon() {
    // If another daemon is already running, bail out
    if UnixStream::connect(SOCKET_PATH).is_ok() {
        eprintln!("[babydra-switcher] Daemon already running.");
        return;
    }

    let application = gtk4::Application::new(Some("org.babydra.switcher"), Default::default());

    application.connect_activate(|app| {
        let controller = render::build_switcher_ui(app);

        // Wrap callbacks in Rc so we can share between the socket pump and
        // the SwitcherController itself.
        let show_fn = std::rc::Rc::new(controller.show_fn);
        let hide_fn = std::rc::Rc::new(controller.hide_fn);
        let next_fn = std::rc::Rc::new(controller.next_fn);
        let window = controller.window;

        let is_visible = std::rc::Rc::new(std::cell::RefCell::new(false));

        let (tx, rx) = std::sync::mpsc::channel::<daemon::DaemonMessage>();
        daemon::spawn_socket_listener(SOCKET_PATH, tx);

        let show_fn_pump = show_fn.clone();
        let next_fn_pump = next_fn.clone();
        let hide_fn_pump = hide_fn.clone();
        let is_visible_pump = is_visible.clone();
        let window_pump = window.clone();

        daemon::setup_message_pump(
            rx,
            // on_show_or_next
            move || {
                if window_pump.is_visible() {
                    next_fn_pump();
                } else {
                    *is_visible_pump.borrow_mut() = true;
                    show_fn_pump();
                }
            },
            // on_hide
            move || {
                hide_fn_pump();
            },
        );
    });

    application.run();
    let _ = std::fs::remove_file(SOCKET_PATH);
}

// ---------------------------------------------------------------------------
// One-shot mode (legacy / testing)
// ---------------------------------------------------------------------------

/// Runs `oneshot`.
fn run_oneshot() {
    // If a daemon is already running, send "show" and exit
    if try_signal_daemon(b"show") {
        return;
    }

    let apps = babydra_core::get_running_apps();
    if apps.is_empty() {
        return;
    }

    // No daemon: fall back to the old spawn-per-keypress behaviour.
    // We reuse the daemon's socket protocol so a running daemon handles
    // subsequent Alt+Tab presses naturally.
    let application = gtk4::Application::new(Some("org.babydra.switcher"), Default::default());

    application.connect_activate(move |app| {
        let controller = render::build_switcher_ui(app);

        let window = controller.window.clone();
        let show_fn = std::rc::Rc::new(controller.show_fn);
        let next_fn = std::rc::Rc::new(controller.next_fn);
        let hide_fn = std::rc::Rc::new(controller.hide_fn);

        // Show immediately on first launch
        show_fn();

        // Listen for subsequent Alt+Tab presses from other process invocations.
        // Mirror daemon mode logic: show if hidden, cycle if already visible.
        let (tx, rx) = std::sync::mpsc::channel::<daemon::DaemonMessage>();
        daemon::spawn_socket_listener(SOCKET_PATH, tx);

        let show_fn_pump = show_fn.clone();
        let next_fn_pump = next_fn.clone();
        daemon::setup_message_pump(
            rx,
            move || {
                if window.is_visible() {
                    next_fn_pump();
                } else {
                    show_fn_pump();
                }
            },
            move || hide_fn(),
        );
    });

    application.run();
    let _ = std::fs::remove_file(SOCKET_PATH);
}
