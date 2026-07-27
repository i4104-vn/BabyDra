//! Wallpaper management utilities.
//! Supported backends: swww, swaybg, feh.

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

    // Save active wallpaper path to config file for persistence
    let config_dir = PathBuf::from(&home).join(".config").join("babydra");
    let _ = std::fs::create_dir_all(&config_dir);
    let _ = std::fs::write(config_dir.join("current_wallpaper"), path_str);

    // 1. Try awww (Primary Wayland backend)
    if has_binary("awww") {
        let _ = Command::new("awww-daemon").spawn();
        let status = Command::new("awww")
            .args(["img", path_str])
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            return Ok(());
        }
    }

    // 2. Try swww (Alternative Wayland backend)
    if has_binary("swww") {
        let _ = Command::new("swww-daemon").spawn();
        let status = Command::new("swww")
            .args(["img", path_str])
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            return Ok(());
        }
    }

    // 3. Try swaybg (standard Wayland background setter)
    if has_binary("swaybg") {
        let _ = Command::new("killall").arg("swaybg").output();
        let status = Command::new("swaybg")
            .args(["-i", path_str, "-m", "fill"])
            .spawn();
        if status.is_ok() {
            return Ok(());
        }
    }

    // 4. Try feh (X11 backend fallback)
    if has_binary("feh") {
        let status = Command::new("feh")
            .args(["--bg-fill", path_str])
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            return Ok(());
        }
    }

    Err("No compatible wallpaper backend (awww, swww, swaybg, or feh) was found in PATH".to_string())
}

/// Retrieves the path to the currently active wallpaper from user configuration or daemon query.
pub fn get_current_wallpaper() -> Option<PathBuf> {
    // 1. Query awww daemon for active image
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

    // 2. Query swww daemon for active image
    if let Ok(output) = Command::new("swww").arg("query").output() {
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

    // 3. Read saved configuration file ~/.config/babydra/current_wallpaper
    if let Ok(home) = std::env::var("HOME") {
        let saved_file = PathBuf::from(&home).join(".config/babydra/current_wallpaper");
        if let Ok(content) = std::fs::read_to_string(&saved_file) {
            let path = PathBuf::from(content.trim());
            if path.exists() {
                return Some(path);
            }
        }

        // 4. Fallback: return first wallpaper found in ~/.babydra/wallpaper
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
