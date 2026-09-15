//! Media kind classification for previewable files.

use std::path::Path;

/// Supported media classification for preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaKind {
    Image,
    Video,
    Unsupported,
}

impl MediaKind {
    /// Detects the media kind based on file extension and path.
    pub fn detect(path: &Path) -> Self {
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_ascii_lowercase())
            .unwrap_or_default();

        match ext.as_str() {
            // Images
            "png" | "jpg" | "jpeg" | "webp" | "bmp" | "svg" | "gif" | "ico" | "tiff" | "tif"
            | "avif" | "heic" | "jfif" => MediaKind::Image,

            // Videos
            "mp4" | "mkv" | "webm" | "avi" | "mov" | "flv" | "wmv" | "m4v" | "ts" | "mpg"
            | "mpeg" | "ogv" | "3gp" | "vob" => MediaKind::Video,

            _ => MediaKind::Unsupported,
        }
    }
}
