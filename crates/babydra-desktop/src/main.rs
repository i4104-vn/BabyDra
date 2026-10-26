//! babydra-desktop — Desktop shell providing wallpaper rendering, icon grid management on ~/Desktop, and desktop context menus.

mod render;
pub mod state;
pub mod widgets;

use gtk4::prelude::*;
use gtk4::Application;
use std::process::Command;
use std::rc::Rc;

/// Application entry point: `main`.
fn main() {
    // Force GStreamer GstPlay to use stable playbin instead of playbin3.
    std::env::set_var("GST_PLAY_USE_PLAYBIN3", "0");
    babydra_core::services::logger::init_logger("babydra-desktop", "babydra-desktop.log");

    if let Some(command) = std::env::args()
        .skip(1)
        .find(|arg| arg == "--frozen" || arg == "--resume")
    {
        let signal = if command == "--frozen" {
            libc::SIGUSR1
        } else {
            libc::SIGUSR2
        };
        if !send_control_signal(signal, &command) {
            std::process::exit(1);
        }
        return;
    }

    let rt = tokio::runtime::Runtime::new().expect("Failed to initialize Tokio runtime");
    let _guard = rt.enter();

    let app = Application::builder()
        .application_id("org.babydra.desktop")
        .flags(gtk4::gio::ApplicationFlags::NON_UNIQUE)
        .build();

    app.connect_activate(|app| {
        // Initialize BabyDra theme and styling
        babydra_ui_kit::ui::theme::init_theme();

        // Apply saved display monitor settings (resolution, refresh rate, orientation)
        babydra_core::services::system::display::apply_saved_displays();

        // Refresh the shared greeter assets (lock wallpaper, avatar) for greetd
        std::thread::spawn(babydra_core::sync_shared_assets);

        // Sync system color-scheme changes (GSettings) to GTK settings in real-time
        let gsettings = gtk4::gio::Settings::new("org.gnome.desktop.interface");
        gsettings.connect_changed(Some("color-scheme"), |_, _| {
            babydra_ui_kit::ui::theme::init_theme();
        });

        // Build and display desktop window on Wayland Background layer
        let (window, set_frozen) = render::build_desktop_window(app);
        window.present();

        register_freeze_signals(set_frozen);
    });

    let exit_code = app.run().value();
    std::process::exit(exit_code);
}

fn send_control_signal(signal: i32, command: &str) -> bool {
    let uid = unsafe { libc::geteuid() }.to_string();
    let output = match Command::new("pgrep")
        .arg("-u")
        .arg(uid)
        .args(["-x", "babydra-desktop"])
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            tracing::error!("Could not find babydra-desktop for {}: {}", command, error);
            return false;
        }
    };

    let mut signaled = false;
    for pid in String::from_utf8_lossy(&output.stdout).lines() {
        let Ok(pid) = pid.parse::<libc::pid_t>() else {
            continue;
        };
        if pid as u32 == std::process::id() {
            continue;
        }

        let is_control_process = std::fs::read(format!("/proc/{pid}/cmdline"))
            .map(|cmdline| {
                cmdline
                    .split(|byte| *byte == 0)
                    .any(|arg| arg == b"--frozen" || arg == b"--resume")
            })
            .unwrap_or(false);
        if is_control_process {
            continue;
        }

        // SAFETY: `pid` comes from pgrep for this user and the signal is a
        // process-control signal handled by the desktop's GLib main loop.
        if unsafe { libc::kill(pid, signal) } == 0 {
            signaled = true;
        }
    }

    if signaled {
        tracing::info!("Sent {} to babydra-desktop", command);
    } else {
        tracing::error!("No running babydra-desktop process accepted {}", command);
    }
    signaled
}

fn register_freeze_signals(set_frozen: Rc<dyn Fn(bool)>) {
    let freeze = set_frozen.clone();
    glib::source::unix_signal_add_local(libc::SIGUSR1, move || {
        freeze(true);
        glib::ControlFlow::Continue
    });

    glib::source::unix_signal_add_local(libc::SIGUSR2, move || {
        set_frozen(false);
        glib::ControlFlow::Continue
    });
}
