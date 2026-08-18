//! Window focus active tracker and switcher screenshot capturer daemon.

use std::fs;
use std::process::Command;
use std::time::{Duration, Instant};

/// Spawns a background thread to track the focused application, clean up stale caches,
/// and capture window screenshots after 5 seconds of active focus.
pub fn spawn_switcher() {
    std::thread::spawn(|| {
        let cache_dir = "/tmp/babydra-switcher-cache";
        let _ = fs::create_dir_all(cache_dir);

        let mut current_focused_window: Option<(String, String)> = None;
        let mut focus_start = Instant::now();
        let mut last_screenshot_time = Instant::now();
        let mut screenshot_taken = false;

        loop {
            std::thread::sleep(Duration::from_millis(300));

            let switcher_open = std::path::Path::new("/tmp/babydra-switcher.socket").exists();
            if switcher_open {
                if let Some((ref old_app, ref old_title)) = current_focused_window {
                    if screenshot_taken {
                        let temp_file = format!("{}/temp_active.png", cache_dir);
                        let hash = crate::services::apps::get_window_hash(old_app, old_title);
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

            let active_window = super::get_active_window();

            // Ignore the switcher itself if it gets focused
            let is_switcher = active_window
                .as_ref()
                .map(|(s, _)| s == "babydra-switcher" || s == "org.babydra.switcher")
                .unwrap_or(false);
            if is_switcher {
                continue;
            }

            if active_window != current_focused_window {
                // User switched away from current_focused_window
                if let Some((ref old_app, ref old_title)) = current_focused_window {
                    if screenshot_taken {
                        // Copy the temp screenshot to the old window's cache file
                        let temp_file = format!("{}/temp_active.png", cache_dir);
                        let hash = crate::services::apps::get_window_hash(old_app, old_title);
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
                    let running_windows = super::get_running_windows();
                    for (id, title) in running_windows {
                        running_hashes.insert(crate::services::apps::get_window_hash(&id, &title));
                        running_app_ids.insert(id);
                    }
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.is_file() {
                                if let Some(file_name) = path.file_name() {
                                    let name_str = file_name.to_string_lossy().to_string();
                                    if name_str != "temp_active.png" && name_str.ends_with(".png") {
                                        let key = name_str.trim_end_matches(".png").to_string();
                                        if !running_hashes.contains(&key)
                                            && !running_app_ids.contains(&key)
                                        {
                                            let _ = fs::remove_file(&path);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                current_focused_window = active_window;
                if let Some((ref app, ref title)) = current_focused_window {
                    let is_switcher = app == "babydra-switcher" || app == "org.babydra.switcher";
                    if !is_switcher {
                        let key = if title.is_empty() {
                            app.clone()
                        } else {
                            title.clone()
                        };
                        super::mru::save_history(&key);
                    }
                }
                focus_start = Instant::now();
                last_screenshot_time = Instant::now();
                screenshot_taken = false;
            } else if current_focused_window.is_some() {
                // If they have stayed in the same window for >= 2 seconds, take a screenshot
                let now = Instant::now();
                let should_take = if !screenshot_taken {
                    now.duration_since(focus_start) >= Duration::from_secs(2)
                } else {
                    now.duration_since(last_screenshot_time) >= Duration::from_secs(30)
                };

                if should_take {
                    let temp_file = format!("{}/temp_active.png", cache_dir);
                    let status = Command::new("grim").arg(&temp_file).status();
                    if let Ok(s) = status {
                        if s.success() {
                            screenshot_taken = true;
                            last_screenshot_time = now;
                            // Copy to active cache immediately so switcher can use it right away
                            if let Some((ref app, ref title)) = current_focused_window {
                                let hash = crate::services::apps::get_window_hash(app, title);
                                let dest_file = format!("{}/{}.png", cache_dir, hash);
                                let _ = fs::copy(&temp_file, &dest_file);
                                let dest_generic = format!("{}/{}.png", cache_dir, app);
                                let _ = fs::copy(&temp_file, &dest_generic);
                            }
                        }
                    }
                }
            }
        }
    });
}
