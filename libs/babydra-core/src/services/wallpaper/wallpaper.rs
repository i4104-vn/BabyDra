//! Wallpaper management utilities.
//! Handles desktop and lock/greeter wallpaper resolution and persistence.

use crate::error::CoreResult;
use std::path::{Path, PathBuf};

/// Sets the desktop wallpaper and persists the path in babydra.conf.
pub fn set_wallpaper(path: &Path) -> CoreResult<()> {
    if !path.exists() {
        return Err(format!("Wallpaper file does not exist at: {:?}", path).into());
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

    let mut conf = crate::config::load_babydra_config();
    conf.wallpaper.current = path_str.to_string();
    crate::config::save_babydra_config(&conf);

    Ok(())
}

/// Applies the currently saved wallpaper from babydra.conf.
pub fn apply_wallpaper() {
    if let Some(path) = get_wallpaper() {
        let _ = set_wallpaper(&path);
    }
}

/// Retrieves the path to the currently active wallpaper from user configuration or wallpaper directory.
pub fn get_wallpaper() -> Option<PathBuf> {
    crate::config::invalidate_cache();
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
                            .map(|ext| {
                                matches!(
                                    ext.to_lowercase().as_str(),
                                    "png" | "jpg" | "jpeg" | "webp"
                                )
                            })
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

/// Sets the greeter / lock background image.
/// - Saves the image to ~/.babydra/wallpaper/<filename> for reuse in user's wallpaper library.
/// - Copies to ~/.babydra/greeter_wallpaper.png and shared /var/lib/babydra/greeter_wallpaper.png for greetd.
pub fn set_greeter_wp(path: &Path) -> CoreResult<()> {
    if !path.exists() {
        return Err(format!("Greeter background file does not exist at: {:?}", path).into());
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let babydra_dir = PathBuf::from(&home).join(".babydra");
    let target_dir = babydra_dir.join("wallpaper");
    let _ = std::fs::create_dir_all(&target_dir);

    // Save to ~/.babydra/wallpaper for future reuse
    if path.parent() != Some(&target_dir) {
        if let Some(file_name) = path.file_name() {
            let dest = target_dir.join(file_name);
            if path != dest {
                let _ = std::fs::copy(path, &dest);
            }
        }
    }

    // Mirror to user directory
    let user_dest = babydra_dir.join("greeter_wallpaper.png");
    let _ = std::fs::remove_file(&user_dest);
    let _ = std::fs::copy(path, &user_dest);

    // Copy to shared system path accessible by greetd
    let system_dest = PathBuf::from("/var/lib/babydra/greeter_wallpaper.png");
    let _ = std::fs::remove_file(&system_dest);
    let _ = std::fs::copy(path, &system_dest);

    Ok(())
}

/// No longer mirrors to system path, just a no-op placeholder for compatibility
pub fn apply_greeter_wp() {
    // No-op
}

/// Retrieves the active greeter background as raw bytes.
pub fn get_greeter_wp_bytes() -> Option<Vec<u8>> {
    let mut candidates = Vec::new();

    if let Ok(home) = std::env::var("HOME") {
        candidates.push(PathBuf::from(&home).join(".babydra/greeter_wallpaper.png"));
    }

    candidates.push(PathBuf::from("/var/lib/babydra/greeter_wallpaper.png"));

    if let Ok(home) = std::env::var("HOME") {
        candidates.push(PathBuf::from(&home).join(".babydra/wallpaper.png"));
    }

    candidates.push(PathBuf::from("/usr/share/babydra/greeter_wallpaper.png"));
    candidates.push(PathBuf::from("/usr/share/babydra/wallpaper.png"));

    for c in &candidates {
        if c.exists() && c.is_file() {
            if let Ok(bytes) = std::fs::read(c) {
                return Some(bytes);
            }
        }
    }

    None
}

/// Retrieves the active greeter background as a CSS URL string.
pub fn get_greeter_wp_css() -> String {
    if let Ok(home) = std::env::var("HOME") {
        let user_wp = PathBuf::from(&home).join(".babydra/greeter_wallpaper.png");
        if user_wp.exists() {
            return format!("url('file://{}')", user_wp.display());
        }
    }

    let shared = PathBuf::from("/var/lib/babydra/greeter_wallpaper.png");
    if shared.exists() {
        return format!("url('file://{}')", shared.display());
    }

    if let Ok(home) = std::env::var("HOME") {
        let default_wp = PathBuf::from(&home).join(".babydra/wallpaper.png");
        if default_wp.exists() {
            return format!("url('file://{}')", default_wp.display());
        }
    }

    "url('file:///usr/share/babydra/wallpaper.png')".to_string()
}
