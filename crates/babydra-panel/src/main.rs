mod widgets;
mod render;

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Starting BabyDra Panel...");

    // Initialize D-Bus StatusNotifierWatcher system tray listener daemon
    babydra_tray::spawn_watcher_service();

    // Detect DDC/CI bus for desktop monitors on startup
    widgets::panel::detect_ddc_bus();

    // Spawn a background thread to refresh desktop apps cache asynchronously on startup
    std::thread::spawn(|| {
        println!("Background thread refreshing desktop apps cache...");
        babydra_common::refresh_desktop_apps_cache();
        println!("Background desktop apps cache refresh complete.");
    });

    // Spawn background thread to track focused app and capture screenshots
    std::thread::spawn(|| {
        use std::process::Command;
        use std::time::{Instant, Duration};
        use std::fs;

        let cache_dir = "/tmp/babydra-switcher-cache";
        let _ = fs::create_dir_all(cache_dir);

        let mut current_focused_window: Option<(String, String)> = None;
        let mut focus_start = Instant::now();
        let mut screenshot_taken = false;

        loop {
            std::thread::sleep(Duration::from_millis(500));

            let switcher_open = std::path::Path::new("/tmp/babydra-switcher.socket").exists();
            if switcher_open {
                if let Some((ref old_app, ref old_title)) = current_focused_window {
                    if screenshot_taken {
                        let temp_file = format!("{}/temp_active.png", cache_dir);
                        // Save window-specific screenshot
                        let hash = babydra_common::desktop::get_window_hash(old_app, old_title);
                        let dest_file = format!("{}/{}.png", cache_dir, hash);
                        let _ = fs::copy(&temp_file, &dest_file);
                        // Save generic fallback screenshot
                        let dest_generic = format!("{}/{}.png", cache_dir, old_app);
                        let _ = fs::copy(&temp_file, &dest_generic);
                    }
                }
                current_focused_window = None;
                screenshot_taken = false;
                continue;
            }

            // Run wlrctl to get the currently focused window
            let output = Command::new("wlrctl")
                .args(&["window", "list", "state:focused"])
                .output();

            let active_window = if let Ok(out) = output {
                let stdout = String::from_utf8_lossy(&out.stdout);
                if let Some(line) = stdout.lines().next() {
                    if let Some(pos) = line.find(':') {
                        let app_id = line[..pos].trim().to_string();
                        let title = line[pos + 1..].trim().to_string();
                        if !app_id.is_empty() {
                            Some((app_id, title))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            // Ignore the switcher itself if it gets focused
            let is_switcher = active_window.as_ref().map(|(s, _)| s == "babydra-switcher" || s == "org.babydra.switcher").unwrap_or(false);
            if is_switcher {
                continue;
            }

            if active_window != current_focused_window {
                // User switched away from current_focused_window
                if let Some((ref old_app, ref old_title)) = current_focused_window {
                    if screenshot_taken {
                        // Copy the temp screenshot to the old window's cache file
                        let temp_file = format!("{}/temp_active.png", cache_dir);
                        let hash = babydra_common::desktop::get_window_hash(old_app, old_title);
                        let dest_file = format!("{}/{}.png", cache_dir, hash);
                        let _ = fs::copy(&temp_file, &dest_file);
                        // Copy to generic fallback screenshot
                        let dest_generic = format!("{}/{}.png", cache_dir, old_app);
                        let _ = fs::copy(&temp_file, &dest_generic);
                    }
                }

                // Clean up stale cache files for windows that are no longer running
                if let Ok(entries) = fs::read_dir(cache_dir) {
                    let mut running_hashes = std::collections::HashSet::new();
                    let mut running_app_ids = std::collections::HashSet::new();
                    if let Ok(out) = Command::new("wlrctl").args(&["window", "list"]).output() {
                        let list_stdout = String::from_utf8_lossy(&out.stdout);
                        for line in list_stdout.lines() {
                            if let Some(pos) = line.find(':') {
                                let id = line[..pos].trim().to_string();
                                  let title = line[pos + 1..].trim().to_string();
                                if !id.is_empty() {
                                    running_hashes.insert(babydra_common::desktop::get_window_hash(&id, &title));
                                    running_app_ids.insert(id);
                                }
                            }
                        }
                    }
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.is_file() {
                                if let Some(file_name) = path.file_name() {
                                    let name_str = file_name.to_string_lossy().to_string();
                                    if name_str != "temp_active.png" && name_str.ends_with(".png") {
                                        let key = name_str.trim_end_matches(".png").to_string();
                                        if !running_hashes.contains(&key) && !running_app_ids.contains(&key) {
                                            let _ = fs::remove_file(&path);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Reset for the new active window
                current_focused_window = active_window;
                focus_start = Instant::now();
                screenshot_taken = false;
            } else if current_focused_window.is_some() && !screenshot_taken {
                // If they have stayed in the same window for >= 5 seconds, take a screenshot
                if focus_start.elapsed() >= Duration::from_secs(5) {
                    let temp_file = format!("{}/temp_active.png", cache_dir);
                    // Run grim to capture the screen
                    let status = Command::new("grim")
                        .arg(&temp_file)
                        .status();
                    if let Ok(s) = status {
                        if s.success() {
                            screenshot_taken = true;
                        }
                    }
                }
            }
        }
    });

    let application = gtk4::Application::new(
        Some("org.babydra.panel"),
        Default::default(),
    );

    application.connect_activate(|app| {
        // Initialize style provider
        babydra_common::init_theme();

        // Define shared window states for mutual exclusivity
        let control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>> = Rc::new(RefCell::new(None));
        let calendar_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>> = Rc::new(RefCell::new(None));
        let launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>> = Rc::new(RefCell::new(None));

        // Build panel UI via render module
        let window = render::build_panel_ui(
            app,
            control_center_window,
            calendar_window,
            launcher_window,
        );

        // Display the window on Wayland
        window.present();
    });

    // Run the GTK loop (this blocks until application exits)
    application.run();
}
