//! Greeter/lock screen wallpaper management using direct file paths.

use crate::config::{load_babydra_config, save_babydra_config};
use crate::error::CoreResult;
use std::path::{Path, PathBuf};

/// Reads raw image bytes from a path directly.
pub fn read_image_bytes(path: &Path) -> Option<Vec<u8>> {
    std::fs::read(path).ok()
}

/// Returns the most recently modified existing path from candidates.
pub fn newest_existing(paths: Vec<Option<PathBuf>>) -> Option<PathBuf> {
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

/// Retrieves the path to the currently active lock/greeter wallpaper image.
pub fn get_greeter_wp() -> Option<PathBuf> {
    let is_readable = |p: &Path| -> bool {
        std::fs::File::open(p).is_ok()
    };

    crate::config::invalidate_cache();
    let conf = load_babydra_config();
    let bg = conf.lockscreen.background.trim_start_matches("file://");
    if !bg.is_empty() {
        let path = PathBuf::from(bg);
        if is_readable(&path)
            && crate::services::wallpaper::utils::is_static_wallpaper_file(&path)
        {
            return Some(path);
        }
    }

    // Shared public fallback for greetd
    let shared_dir = PathBuf::from("/var/lib/babydra");
    for ext in ["png", "jpg", "jpeg", "webp"] {
        let shared_file = shared_dir.join(format!("lock_wallpaper.{}", ext));
        if is_readable(&shared_file) {
            return Some(shared_file);
        }
    }

    // Fallback to desktop wallpaper if it is a static image
    if let Some(wp_path) = get_wallpaper() {
        if is_readable(&wp_path) && crate::services::wallpaper::utils::is_static_wallpaper_file(&wp_path) {
            return Some(wp_path);
        }
    }

    let default_paths = [
        PathBuf::from("/usr/share/babydra/wallpaper.png"),
        dirs::home_dir().unwrap_or_default().join(".babydra/wallpaper.png"),
    ];
    for def in &default_paths {
        if is_readable(def) {
            return Some(def.clone());
        }
    }

    None
}

/// Sets the greeter / lock background image.
/// Only static image formats (PNG, JPG, JPEG, WEBP) are accepted. Videos and GIFs are rejected.
/// - Saves the image to ~/.babydra/wallpaper/<filename> for reuse in user's wallpaper library.
/// - Persists the real file path in babydra.conf under `[lockscreen] background`.
/// - Syncs a world-readable copy to `/var/lib/babydra/lock_wallpaper.<ext>` for greetd.
pub fn set_greeter_wp(path: &Path) -> CoreResult<()> {
    if !path.exists() {
        return Err(format!("Greeter background file does not exist at: {:?}", path).into());
    }

    if !crate::services::wallpaper::utils::is_static_wallpaper_file(path) {
        return Err("Lockscreen and greeter only support static image files (PNG, JPG, JPEG, WEBP)".into());
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let babydra_dir = PathBuf::from(&home).join(".babydra");
    let target_dir = babydra_dir.join("wallpaper");
    std::fs::create_dir_all(&target_dir)?;

    let target_image_path = if path.parent() != Some(&target_dir) {
        if let Some(file_name) = path.file_name() {
            let dest = target_dir.join(file_name);
            if path != dest {
                std::fs::copy(path, &dest)?;
            }
            dest
        } else {
            path.to_path_buf()
        }
    } else {
        path.to_path_buf()
    };

    let _ = std::fs::remove_file(babydra_dir.join("greeter_wallpaper.png"));

    let mut conf = load_babydra_config();
    conf.lockscreen.background = target_image_path.to_string_lossy().to_string();
    save_babydra_config(&conf);

    // Save fallback for greetd which runs as another user (greeter)
    use std::os::unix::fs::PermissionsExt;
    let shared_dir = PathBuf::from("/var/lib/babydra");
    if std::fs::create_dir_all(&shared_dir).is_ok() {
        let _ = std::fs::set_permissions(&shared_dir, std::fs::Permissions::from_mode(0o777));
    }
    let ext = target_image_path.extension().and_then(|e| e.to_str()).unwrap_or("png");
    let public_dest = shared_dir.join(format!("lock_wallpaper.{}", ext));
    if std::fs::copy(&target_image_path, &public_dest).is_ok() {
        let _ = std::fs::set_permissions(&public_dest, std::fs::Permissions::from_mode(0o666));
    }

    Ok(())
}

/// Applies the currently saved greeter/lock wallpaper from config.
pub fn apply_greeter_wp() {
    if let Some(path) = get_greeter_wp() {
        let _ = set_greeter_wp(&path);
    }
}

/// Retrieves the active greeter/lock background as raw bytes from the image file.
pub fn get_greeter_wp_bytes() -> Option<Vec<u8>> {
    let path = get_greeter_wp()?;
    std::fs::read(path).ok()
}

/// Retrieves the active greeter background as a CSS URL string.
pub fn get_greeter_wp_css() -> String {
    if let Some(path) = get_greeter_wp() {
        return format!("url('file://{}')", path.display());
    }
    "url('file:///usr/share/babydra/wallpaper.png')".to_string()
}

/// Refreshes the shared `/var/lib/babydra` copies of the lock wallpaper and avatar.
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

    if let Some(wp) = get_greeter_wp() {
        let ext = wp.extension().and_then(|e| e.to_str()).unwrap_or("png").to_string();
        sync(Some(wp), shared_dir.join(format!("lock_wallpaper.{}", ext)));
    }

    let home = dirs::home_dir();
    sync(
        home.as_ref().map(|h| h.join(".babydra/avatar.png")),
        shared_dir.join("avatar.png"),
    );
}

/// Helper to get wallpaper path (re-exported from config)
fn get_wallpaper() -> Option<PathBuf> {
    crate::services::wallpaper::config::get_wallpaper()
}