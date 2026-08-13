//! Wallpaper management utilities.
//! Supported backends: swww, swaybg, feh.

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Checks if a command binary exists in the system's PATH.
fn has_binary(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Sets the desktop wallpaper using the best available backend utility (awww, swww, swaybg, feh).
pub fn set_wallpaper(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("Wallpaper file does not exist at: {:?}", path));
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let target_dir = PathBuf::from(&home).join(".babydra").join("wallpaper");
    let _ = std::fs::create_dir_all(&target_dir);

    // Save image to ~/.babydra/wallpaper if not already there
    let target_path = if path.parent() != Some(&target_dir) {
        if let Some(file_name) = path.file_name() {
            let dest = target_dir.join(file_name);
            if path != dest {
                let _ = std::fs::copy(path, &dest);
            }
            dest
        } else {
            path.to_path_buf()
        }
    } else {
        path.to_path_buf()
    };

    let path_str = target_path.to_str().ok_or("Invalid path encoding")?;

    // Save active wallpaper path to unified babydra.conf
    let mut conf = crate::config::load_babydra_config();
    conf.wallpaper.current = path_str.to_string();
    crate::config::save_babydra_config(&conf);

    if has_binary("awww") {
        let daemon_running = Command::new("pgrep")
            .arg("-x")
            .arg("awww-daemon")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if !daemon_running {
            let _ = Command::new("awww-daemon").spawn();
        }
        let status = Command::new("awww")
            .args(["img", path_str])
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            return Ok(());
        }
    } 

    Err("No compatible wallpaper backend - awww was found in PATH".to_string())
}

/// Applies the currently saved wallpaper from babydra.conf.
pub fn apply_saved_wallpaper() {
    if let Some(path) = get_current_wallpaper() {
        let _ = set_wallpaper(&path);
    }
}

/// Retrieves the path to the currently active wallpaper from user configuration or daemon query.
pub fn get_current_wallpaper() -> Option<PathBuf> {
    if let Ok(output) = Command::new("awww").arg("query").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if let Some(idx) = line.find("image:") {
                let raw_path = line[idx + "image:".len()..].trim();
                let path = PathBuf::from(raw_path);
                if path.exists() {
                    return Some(path);
                }
            }
        }
    }

    let conf = crate::config::load_babydra_config();
    if !conf.wallpaper.current.is_empty() {
        let path = PathBuf::from(&conf.wallpaper.current);
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let wp_dir = PathBuf::from(&home).join(".babydra/wallpaper");
        if let Ok(entries) = std::fs::read_dir(wp_dir) {
            let mut files: Vec<PathBuf> = entries
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| {
                    p.is_file()
                        && p.extension()
                            .and_then(|ext| ext.to_str())
                            .map(|ext| matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "webp"))
                            .unwrap_or(false)
                })
                .collect();
            files.sort();
            if let Some(first) = files.first() {
                return Some(first.clone());
            }
        }
    }

    None
}

/// Returns the path to the user's wallpaper directory (~/.babydra/wallpaper).
pub fn get_wallpaper_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let dir = PathBuf::from(home).join(".babydra").join("wallpaper");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Retrieves all local wallpaper image files from ~/.babydra/wallpaper.
pub fn get_local_wallpapers() -> Vec<PathBuf> {
    let dir = get_wallpaper_dir();
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if matches!(ext_lower.as_str(), "png" | "jpg" | "jpeg" | "webp") {
                        files.push(path);
                    }
                }
            }
        }
    }
    files.sort();
    files
}

/// Helper to scan all user homes for saved greeter background in babydra.conf
fn find_user_conf_wallpaper() -> Option<PathBuf> {
    if let Ok(entries) = std::fs::read_dir("/home") {
        for entry in entries.filter_map(Result::ok) {
            let user_home = entry.path();
            let conf_path = user_home.join(".babydra").join("babydra.conf");
            if conf_path.exists() {
                if let Ok(content) = std::fs::read_to_string(&conf_path) {
                    if let Ok(conf) = toml::from_str::<crate::config::BabyDraConfig>(&content) {
                        if !conf.greeter.background.is_empty() {
                            let p = PathBuf::from(&conf.greeter.background);
                            if p.exists() {
                                return Some(p);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Mirrors the chosen greeter wallpaper to the world-readable system location
/// (`/var/lib/babydra/greeter_wallpaper.png`) so the greetd-hosted greeter process
/// (which runs as the unprivileged `greeter` user and cannot read a locked-down
/// user home) can always display it.
fn sync_greeter_wallpaper_to_system(source: &Path) {
    let sync_dir = PathBuf::from("/var/lib/babydra");
    if std::fs::create_dir_all(&sync_dir).is_err() {
        return;
    }
    // Keep the sync directory writable by regular users (install.sh also does this)
    let _ = std::fs::set_permissions(&sync_dir, std::fs::Permissions::from_mode(0o777));
    let sync_file = sync_dir.join("greeter_wallpaper.png");
    if std::fs::copy(source, &sync_file).is_err() {
        return;
    }
    // Force world-readable permissions regardless of the source file's mode
    let _ = std::fs::set_permissions(&sync_file, std::fs::Permissions::from_mode(0o644));
}

/// Sets the greeter background image path in babydra.conf and mirrors it to the
/// world-readable system path consumed by the greetd greeter process.
pub fn set_greeter_wallpaper(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("Greeter background file does not exist at: {:?}", path));
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let target_dir = PathBuf::from(&home).join(".babydra").join("wallpaper");
    let _ = std::fs::create_dir_all(&target_dir);

    let target_path = if path.parent() != Some(&target_dir) {
        if let Some(file_name) = path.file_name() {
            let dest = target_dir.join(file_name);
            if path != dest {
                let _ = std::fs::copy(path, &dest);
            }
            dest
        } else {
            path.to_path_buf()
        }
    } else {
        path.to_path_buf()
    };

    let path_str = target_path.to_str().ok_or("Invalid path encoding")?;

    // Mirror to the system location so the greeter process can always read it
    sync_greeter_wallpaper_to_system(&target_path);

    let mut conf = crate::config::load_babydra_config();
    conf.greeter.background = path_str.to_string();
    crate::config::save_babydra_config(&conf);
    Ok(())
}

/// Reads the saved greeter wallpaper from the current user's config and mirrors it
/// to the world-readable system path used by the greetd greeter process.
pub fn apply_saved_greeter_wallpaper() {
    let conf = crate::config::load_babydra_config();
    if !conf.greeter.background.is_empty() {
        let path = PathBuf::from(&conf.greeter.background);
        if path.exists() {
            sync_greeter_wallpaper_to_system(&path);
        }
    }
}

/// Retrieves the active greeter background path.
///
/// The greetd-hosted greeter runs as the unprivileged `greeter` user whose home is
/// not the real user's home, so it cannot read `~/.babydra` when the home directory
/// is locked down (e.g. `drwx------`). For that reason the world-readable system
/// copy (`/var/lib/babydra/greeter_wallpaper.png`) is preferred first. Whenever a
/// user-level path can be resolved by a process that has access to it, the system
/// copy is refreshed so the next boot stays in sync.
pub fn get_greeter_wallpaper() -> Option<PathBuf> {
    // 1. Current user config (only reachable when this process can read it — i.e.
    //    the real user, e.g. Settings preview or a terminal run). Refreshes the
    //    system copy so the next boot stays in sync.
    let conf = crate::config::load_babydra_config();
    if !conf.greeter.background.is_empty() {
        let path = PathBuf::from(&conf.greeter.background);
        if path.exists() {
            sync_greeter_wallpaper_to_system(&path);
            return Some(path);
        }
    }

    // 2. World-readable system copy (always accessible to the greeter process at
    //    boot, which runs as the unprivileged `greeter` user)
    let system_sync = PathBuf::from("/var/lib/babydra/greeter_wallpaper.png");
    if system_sync.exists() {
        return Some(system_sync);
    }

    // 3. Scan user homes for saved greeter background in babydra.conf
    if let Some(user_wp) = find_user_conf_wallpaper() {
        sync_greeter_wallpaper_to_system(&user_wp);
        return Some(user_wp);
    }

    // 4. Shared greeter system wallpaper paths
    let system_candidates = ["/usr/share/babydra/wallpaper.png"];
    for c in &system_candidates {
        let p = PathBuf::from(c);
        if p.exists() {
            return Some(p);
        }
    }

    // 5. User home default wallpaper
    if let Ok(home) = std::env::var("HOME") {
        let p = PathBuf::from(home).join(".babydra/wallpaper.png");
        if p.exists() {
            return Some(p);
        }
    }

    // 6. Source repository fallback wallpaper
    let src_wp = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../../wallpaper.png"));
    if src_wp.exists() {
        return Some(src_wp);
    }

    None
}




