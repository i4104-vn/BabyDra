//! Wallpaper configuration and persistence.

use crate::config::{load_babydra_config, save_babydra_config};
use crate::error::CoreResult;
use crate::services::wallpaper::types::{WallpaperKind, WallpaperMode};
use crate::services::wallpaper::utils::{get_video_duration, is_gstreamer_plugin_available};
use std::path::{Path, PathBuf};

/// Retrieves the current wallpaper mode ("static" or "live").
pub fn get_wallpaper_mode() -> WallpaperMode {
    crate::config::invalidate_cache();
    let conf = load_babydra_config();
    if !conf.wallpaper.mode.is_empty() {
        conf.wallpaper.mode.parse().unwrap_or(WallpaperMode::Static)
    } else if let Some(wp) = get_wallpaper() {
        WallpaperKind::from_path(&wp).mode()
    } else {
        WallpaperMode::Static
    }
}

/// Sets the desktop wallpaper with an explicit mode and persists it.
pub fn set_wallpaper_with_mode(path: &Path, mode: WallpaperMode) -> CoreResult<()> {
    if !path.exists() {
        return Err(format!("Wallpaper file does not exist at: {:?}", path).into());
    }

    if mode == WallpaperMode::Live {
        let kind = WallpaperKind::from_path(path);
        if kind == WallpaperKind::Video {
            if !is_gstreamer_plugin_available() {
                return Err(crate::i18n::trans("settings.missing_gst_plugin_desc").into());
            }
            if let Some(dur) = get_video_duration(path) {
                if dur > 20.05 {
                    return Err(crate::i18n::trans("settings.video_too_long_msg")
                        .replace("{duration}", &format!("{:.1}", dur))
                        .into());
                }
            }
        }
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let target_dir = PathBuf::from(&home).join(".babydra").join("wallpaper");
    std::fs::create_dir_all(&target_dir)?;

    let target_path = if path.parent() != Some(&target_dir) {
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

    let mut conf = load_babydra_config();
    conf.wallpaper.current = target_path.to_string_lossy().to_string();
    conf.wallpaper.mode = mode.to_string();
    save_babydra_config(&conf);

    Ok(())
}

/// Sets the desktop wallpaper (auto-detects mode from file extension).
pub fn set_wallpaper(path: &Path) -> CoreResult<()> {
    let mode = WallpaperKind::from_path(path).mode();
    set_wallpaper_with_mode(path, mode)
}

/// Applies the currently saved wallpaper from config.
pub fn apply_wallpaper() {
    if let Some(path) = get_wallpaper() {
        let mode = get_wallpaper_mode();
        let _ = set_wallpaper_with_mode(&path, mode);
    }
}

/// Retrieves the path to the currently active wallpaper.
pub fn get_wallpaper() -> Option<PathBuf> {
    crate::config::invalidate_cache();
    let conf = load_babydra_config();
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
                        && matches!(
                            WallpaperKind::from_path(p),
                            WallpaperKind::StaticImage | WallpaperKind::Video | WallpaperKind::AnimatedGif
                        )
                })
                .collect();
            files.sort();
            return files.first().cloned();
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

/// Retrieves all local static wallpaper image files.
pub fn get_static_wallpapers() -> Vec<PathBuf> {
    let dir = get_wallpaper_dir();
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && WallpaperKind::from_path(&path) == WallpaperKind::StaticImage {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// Retrieves all local live wallpaper files (video, gif).
pub fn get_live_wallpapers() -> Vec<PathBuf> {
    let dir = get_wallpaper_dir();
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && WallpaperKind::from_path(&path).is_live() {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// Retrieves all local wallpaper files.
pub fn get_local_wallpapers() -> Vec<PathBuf> {
    let dir = get_wallpaper_dir();
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                let kind = WallpaperKind::from_path(&path);
                if matches!(kind, WallpaperKind::StaticImage | WallpaperKind::Video | WallpaperKind::AnimatedGif) {
                    files.push(path);
                }
            }
        }
    }
    files.sort();
    files
}