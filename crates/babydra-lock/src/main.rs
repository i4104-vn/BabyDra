//! Main entry point for the BabyDra Screen Locker.
//! Sets up GTK Application, parses command line arguments for a custom wallpaper,
//! initializes theme context, and maps locker windows to all connected monitors.

mod render;
mod widgets;

use gtk4::prelude::*;
use std::cell::{Cell, RefCell};
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::rc::Rc;
use std::time::Duration;

/// Application entry point: `main`.
fn main() {
    babydra_core::services::logger::init_logger("babydra-lock", "babydra-lock.log");

    let args: Vec<String> = std::env::args().collect();
    let mut custom_image = None;
    if args.len() > 1 {
        let mut i = 1;
        while i < args.len() {
            if (args[i] == "--image" || args[i] == "-i") && i + 1 < args.len() {
                custom_image = Some(args[i + 1].clone());
                i += 2;
            } else if !args[i].starts_with('-') {
                custom_image = Some(args[i].clone());
                i += 1;
            } else {
                i += 1;
            }
        }
    }

    let application = gtk4::Application::new(Some("org.babydra.lock"), Default::default());

    let desktop_frozen = Rc::new(Cell::new(false));
    let desktop_frozen_on_activate = desktop_frozen.clone();
    let desktop_executable = Rc::new(RefCell::new(None));
    let desktop_executable_on_activate = desktop_executable.clone();
    let transition_paths = Rc::new(RefCell::new(Vec::new()));
    let transition_paths_on_activate = transition_paths.clone();
    let transition_stacks = Rc::new(RefCell::new(Vec::new()));
    let transition_stacks_on_activate = transition_stacks.clone();
    let unlock_started = Rc::new(Cell::new(false));

    application.connect_activate(move |app| {
        babydra_ui_kit::ui::theme::init_theme();

        *desktop_executable_on_activate.borrow_mut() = find_running_desktop_executable();
        transition_stacks_on_activate.borrow_mut().clear();
        unlock_started.set(false);

        // Keep the captured desktop above the lock UI during the short blur
        // transition, then freeze the desktop shell without exiting it.
        let transition_images = render::capture_transition_images();
        *transition_paths_on_activate.borrow_mut() = transition_images
            .iter()
            .filter_map(|image| image.clone())
            .collect();

        let app_on_unlock = app.clone();
        let stacks_on_unlock = transition_stacks_on_activate.clone();
        let desktop_executable_on_unlock = desktop_executable_on_activate.borrow().clone();
        let desktop_frozen_on_unlock = desktop_frozen_on_activate.clone();
        let unlock_started_on_unlock = unlock_started.clone();
        let on_unlock = Rc::new(move || {
            if unlock_started_on_unlock.replace(true) {
                return;
            }

            if desktop_frozen_on_unlock.get()
                && set_desktop_mode(desktop_executable_on_unlock.as_deref(), "--resume")
            {
                desktop_frozen_on_unlock.set(false);
            }

            let app = app_on_unlock.clone();
            render::reverse_lock_transition(
                stacks_on_unlock.borrow().clone(),
                Rc::new(move || app.quit()),
            );
        });

        render::build_lock_ui(
            app,
            custom_image.as_deref(),
            &transition_images,
            transition_stacks_on_activate.clone(),
            on_unlock,
        );

        let desktop_executable = desktop_executable_on_activate.borrow().clone();
        let desktop_frozen = desktop_frozen_on_activate.clone();
        let unlock_started = unlock_started.clone();
        gtk4::glib::timeout_add_local_once(Duration::from_millis(100), move || {
            if !unlock_started.get() {
                desktop_frozen.set(set_desktop_mode(desktop_executable.as_deref(), "--frozen"));
            }
        });
    });

    application.run_with_args::<&str>(&[]);

    for image in transition_paths.borrow().iter() {
        let _ = std::fs::remove_file(image);
    }

    if desktop_frozen.get() {
        set_desktop_mode(desktop_executable.borrow().as_deref(), "--resume");
    }
}

/// Captures the path of the currently running desktop process before it stops.
fn find_running_desktop_executable() -> Option<PathBuf> {
    if let Ok(output) = Command::new("pgrep")
        .arg("-u")
        .arg(unsafe { libc::geteuid() }.to_string())
        .args(["-x", "babydra-desktop"])
        .output()
    {
        for pid in String::from_utf8_lossy(&output.stdout).lines() {
            if let Ok(executable) = std::fs::read_link(format!("/proc/{pid}/exe")) {
                let executable_text = executable.to_string_lossy();
                let executable_text = executable_text
                    .strip_suffix(" (deleted)")
                    .unwrap_or(executable_text.as_ref());
                let executable = PathBuf::from(executable_text);
                if executable.is_file() {
                    return Some(executable);
                }
            }
        }
    }

    if let Ok(lock_executable) = env::current_exe() {
        if let Some(directory) = lock_executable.parent() {
            let sibling = directory.join("babydra-desktop");
            if sibling.is_file() {
                return Some(sibling);
            }
        }
    }

    env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join(".local/bin/babydra-desktop"))
        .filter(|executable| executable.is_file())
}

/// Sends a freeze or resume request to the already running desktop process.
fn set_desktop_mode(executable: Option<&std::path::Path>, mode: &str) -> bool {
    let Some(executable) = executable else {
        tracing::error!("Could not locate babydra-desktop to apply {}", mode);
        return false;
    };

    if !Command::new(executable)
        .arg(mode)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
    {
        tracing::error!("Failed to send {} to babydra-desktop", mode);
        return false;
    }
    tracing::info!("Applied {} to babydra-desktop", mode);
    true
}
