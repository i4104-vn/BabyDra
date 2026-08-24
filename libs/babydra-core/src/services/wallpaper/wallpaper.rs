//! Wallpaper management utilities.
//! Handles desktop and lock/greeter wallpaper resolution and persistence.

use crate::error::CoreResult;
use base64::prelude::*;
use std::path::{Path, PathBuf};

/// Reads raw image bytes from a path, transparently decoding Base64 `.bb` files.
pub fn read_image_bytes(path: &Path) -> Option<Vec<u8>> {
    if path.extension().and_then(|e| e.to_str()) == Some("bb") {
        let content = std::fs::read_to_string(path).ok()?;
        BASE64_STANDARD.decode(content.trim().as_bytes()).ok()
    } else {
        std::fs::read(path).ok()
    }
}

/// Returns the most recently modified existing path from the given candidates.
///
/// The greeter runs as the `greeter` user (HOME=/) so `$HOME`-relative paths are
/// unreadable there; picking the freshest existing copy lets the session-owned
/// file in `~/.babydra` and the shared copy in `/var/lib/babydra` stay in sync
/// regardless of which one was updated last.
pub(crate) fn newest_existing(paths: Vec<Option<PathBuf>>) -> Option<PathBuf> {
    let mut best: Option<(std::time::SystemTime, PathBuf)> = None;
    for path in paths.into_iter().flatten() {
        if !path.is_file() {
            continue;
        }
        let mtime = std::fs::metadata(&path).and_then(|m| m.modified()).ok();
        let better = match (&best, mtime) {
            (_, None) => best.is_none(),
            (None, Some(_)) => true,
            (Some((best_time, _)), Some(mtime)) => mtime > *best_time,
        };
        if better {
            best = Some((mtime.unwrap_or(std::time::UNIX_EPOCH), path));
        }
    }
    best.map(|(_, path)| path)
}

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

/// Retrieves the path to the currently active lock/greeter wallpaper (.bb file or image).
pub fn get_greeter_wp() -> Option<PathBuf> {
    crate::config::invalidate_cache();
    let conf = crate::config::load_babydra_config();
    if !conf.lockscreen.background.is_empty() {
        let path = PathBuf::from(&conf.lockscreen.background);
        if path.exists() && path.is_file() {
            return Some(path);
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let lock_bb = PathBuf::from(&home).join(".babydra/lock_wallpaper.bb");
        if lock_bb.exists() && lock_bb.is_file() {
            return Some(lock_bb);
        }
        let greeter_bb = PathBuf::from(&home).join(".babydra/greeter_wallpaper.bb");
        if greeter_bb.exists() && greeter_bb.is_file() {
            return Some(greeter_bb);
        }
    }

    let fallback = PathBuf::from("/var/lib/babydra/lock_wallpaper.bb");
    if fallback.exists() && fallback.is_file() {
        return Some(fallback);
    }

    get_wallpaper()
}

/// Sets the greeter / lock background image.
/// - Saves the image to ~/.babydra/wallpaper/<filename> for reuse in user's wallpaper library.
/// - Encodes the image bytes to Base64 and persists in `~/.babydra/lock_wallpaper.bb`.
/// - Persists the path in babydra.conf under `[lockscreen] background`.
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

    let raw_bytes = std::fs::read(path)?;
    let encoded = BASE64_STANDARD.encode(&raw_bytes);

    let user_dest = babydra_dir.join("lock_wallpaper.bb");
    std::fs::write(&user_dest, &encoded)?;

    // Clean up legacy files if present
    let _ = std::fs::remove_file(babydra_dir.join("greeter_wallpaper.bb"));
    let _ = std::fs::remove_file(babydra_dir.join("greeter_wallpaper.png"));

    let path_str = user_dest.to_str().ok_or("Invalid path encoding")?;
    let mut conf = crate::config::load_babydra_config();
    conf.lockscreen.background = path_str.to_string();
    crate::config::save_babydra_config(&conf);

    // Save fallback for greetd which runs as another user.
    use std::os::unix::fs::PermissionsExt;
    let shared_dir = PathBuf::from("/var/lib/babydra");
    if std::fs::create_dir_all(&shared_dir).is_ok() {
        let _ = std::fs::set_permissions(&shared_dir, std::fs::Permissions::from_mode(0o777));
    }
    let public_dest = shared_dir.join("lock_wallpaper.bb");
    if std::fs::write(&public_dest, &encoded).is_ok() {
        let _ = std::fs::set_permissions(&public_dest, std::fs::Permissions::from_mode(0o666));
    }

    Ok(())
}

/// Applies the currently saved greeter/lock wallpaper from babydra.conf.
pub fn apply_greeter_wp() {
    if let Some(path) = get_greeter_wp() {
        let _ = set_greeter_wp(&path);
    }
}

/// Retrieves the active greeter/lock background as raw bytes decoded from Base64 `.bb` (or raw image fallback).
pub fn get_greeter_wp_bytes() -> Option<Vec<u8>> {
    crate::config::invalidate_cache();
    let conf = crate::config::load_babydra_config();

    // Explicit user selection wins when it exists
    if !conf.lockscreen.background.is_empty() {
        let path = PathBuf::from(&conf.lockscreen.background);
        if path.is_file() {
            if let Some(bytes) = read_image_bytes(&path) {
                if !bytes.is_empty() {
                    return Some(bytes);
                }
            }
        }
    }

    // Otherwise use the freshest copy across the user home and shared store
    if let Some(path) = newest_existing(vec![
        dirs::home_dir().map(|h| h.join(".babydra").join("lock_wallpaper.bb")),
        dirs::home_dir().map(|h| h.join(".babydra").join("greeter_wallpaper.bb")),
        Some(PathBuf::from("/var/lib/babydra/lock_wallpaper.bb")),
    ]) {
        if let Some(bytes) = read_image_bytes(&path) {
            if !bytes.is_empty() {
                return Some(bytes);
            }
        }
    }

    // Fallback to desktop wallpaper
    if let Some(wp_path) = get_wallpaper() {
        if let Some(bytes) = read_image_bytes(&wp_path) {
            return Some(bytes);
        }
    }

    let default_paths = [
        PathBuf::from("/usr/share/babydra/wallpaper.png"),
        dirs::home_dir().unwrap_or_default().join(".babydra/wallpaper.png"),
    ];
    for def in &default_paths {
        if def.exists() {
            if let Ok(bytes) = std::fs::read(def) {
                return Some(bytes);
            }
        }
    }

    None
}

/// Retrieves the active greeter background as a CSS URL string.
pub fn get_greeter_wp_css() -> String {
    if let Some(path) = get_greeter_wp() {
        if path.extension().and_then(|e| e.to_str()) != Some("bb") {
            return format!("url('file://{}')", path.display());
        }
    }

    "url('file:///usr/share/babydra/wallpaper.png')".to_string()
}

/// Refreshes the shared `/var/lib/babydra` copies of the lock wallpaper and
/// avatar so the greeter (which runs as the `greeter` user and cannot read the
/// session user's home) always sees the current assets after a reboot.
/// Called once when the desktop session starts.
pub fn sync_shared_assets() {
    let shared_dir = PathBuf::from("/var/lib/babydra");
    if std::fs::create_dir_all(&shared_dir).is_err() {
        return;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&shared_dir, std::fs::Permissions::from_mode(0o777));
    }

    let sync = |src: Option<PathBuf>, dest: PathBuf| {
        let Some(src) = src else { return };
        if !src.is_file() {
            return;
        }
        // Skip when the shared copy is already up to date
        if let (Ok(a), Ok(b)) = (std::fs::metadata(&src), std::fs::metadata(&dest)) {
            if let (Ok(a), Ok(b)) = (a.modified(), b.modified()) {
                if a <= b {
                    return;
                }
            }
        }
        if std::fs::copy(&src, &dest).is_ok() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o666));
            }
        }
    };

    let home = dirs::home_dir();
    sync(
        home.as_ref().map(|h| h.join(".babydra/lock_wallpaper.bb")),
        shared_dir.join("lock_wallpaper.bb"),
    );
    sync(
        home.as_ref().map(|h| h.join(".babydra/avatar.bb")),
        shared_dir.join("avatar_fallback.bb"),
    );
}
