//! Shared utilities for wallpaper services.

use std::path::Path;
use std::process::Command;

/// Checks whether GStreamer plugins required for video playback are available.
/// Flexibly checks for MP4 (qtdemux) or WebM/MKV (matroskademux) demuxers.
pub fn is_gstreamer_plugin_available() -> bool {
    let check_plugin = |element: &str| {
        Command::new("gst-inspect-1.0")
            .arg(element)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    };

    check_plugin("qtdemux") || check_plugin("matroskademux")
}

/// Returns `true` if the path has a supported video extension (mp4, webm, mkv).
pub fn is_video_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext.to_lowercase().as_str(), "mp4" | "webm" | "mkv"))
        .unwrap_or(false)
}

/// Returns `true` if the path has a supported GIF extension.
pub fn is_gif_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase() == "gif")
        .unwrap_or(false)
}

/// Returns `true` if the path is a live wallpaper (video or animated gif).
pub fn is_live_wallpaper_file(path: &Path) -> bool {
    is_video_file(path) || is_gif_file(path)
}

/// Returns `true` if the path has a supported static image extension.
pub fn is_static_wallpaper_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| matches!(ext.to_lowercase().as_str(), "png" | "jpg" | "jpeg" | "webp"))
        .unwrap_or(false)
}

/// Extracts the video duration in seconds using ffprobe or gst-discoverer.
pub fn get_video_duration(path: &Path) -> Option<f64> {
    if !path.is_file() {
        return None;
    }

    // 1. Try ffprobe first
    if let Ok(output) = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
        ])
        .arg(path)
        .output()
    {
        if output.status.success() {
            let dur_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(dur) = dur_str.trim().parse::<f64>() {
                return Some(dur);
            }
        }
    }

    // 2. Fallback: try gst-discoverer-1.0
    if let Ok(output) = Command::new("gst-discoverer-1.0")
        .arg(path)
        .output()
    {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("Duration:") {
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if let Some(time_str) = parts.get(1) {
                        let t_parts: Vec<&str> = time_str.split(':').collect();
                        if t_parts.len() == 3 {
                            let h: f64 = t_parts[0].parse().unwrap_or(0.0);
                            let m: f64 = t_parts[1].parse().unwrap_or(0.0);
                            let s: f64 = t_parts[2].parse().unwrap_or(0.0);
                            return Some(h * 3600.0 + m * 60.0 + s);
                        }
                    }
                }
            }
        }
    }

    None
}