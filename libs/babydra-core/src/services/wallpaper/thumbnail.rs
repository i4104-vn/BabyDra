//! Thumbnail generation and caching for wallpapers.

use std::path::{Path, PathBuf};
use crate::services::wallpaper::types::WallpaperKind;
use std::process::Command;

/// Returns an image thumbnail path for the given wallpaper.
/// If it's a video, generates and caches a 1st frame JPEG thumbnail.
pub fn get_or_create_thumbnail(path: &Path) -> PathBuf {
    if WallpaperKind::from_path(path) != WallpaperKind::Video {
        return path.to_path_buf();
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let thumb_dir = Path::new(&home).join(".cache/babydra/thumbnails");
    let _ = std::fs::create_dir_all(&thumb_dir);

    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("thumb");
    let meta_len = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let thumb_name = format!("{}_{}.jpg", file_stem, meta_len);
    let thumb_path = thumb_dir.join(thumb_name);

    if thumb_path.exists() {
        return thumb_path;
    }

    let _ = Command::new("ffmpeg")
        .args([
            "-y",
            "-ss",
            "00:00:00.5",
            "-i",
        ])
        .arg(path)
        .args([
            "-update",
            "1",
            "-vframes",
            "1",
            "-vf",
            "scale=320:-1",
        ])
        .arg(&thumb_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    if thumb_path.exists() {
        thumb_path
    } else {
        path.to_path_buf()
    }
}

/// Returns a full-resolution first frame image path for the given wallpaper.
/// If it's a video, extracts and caches the 1st frame (at 00:00:00) as a full-resolution JPEG.
/// If it's an image or GIF, returns the original path directly.
pub fn get_or_create_first_frame(path: &Path) -> PathBuf {
    if WallpaperKind::from_path(path) != WallpaperKind::Video {
        return path.to_path_buf();
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let thumb_dir = Path::new(&home).join(".cache/babydra/thumbnails");
    let _ = std::fs::create_dir_all(&thumb_dir);

    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("frame");
    let meta_len = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
    let frame_name = format!("{}_{}_frame0.jpg", file_stem, meta_len);
    let frame_path = thumb_dir.join(frame_name);

    if frame_path.exists() {
        return frame_path;
    }

    let _ = Command::new("ffmpeg")
        .args([
            "-y",
            "-ss",
            "00:00:00",
            "-i",
        ])
        .arg(path)
        .args([
            "-update",
            "1",
            "-vframes",
            "1",
        ])
        .arg(&frame_path)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();

    if frame_path.exists() {
        frame_path
    } else {
        path.to_path_buf()
    }
}