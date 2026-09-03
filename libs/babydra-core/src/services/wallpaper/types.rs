//! Shared wallpaper types and traits for all crates.

use std::path::PathBuf;

/// Wallpaper mode: static image or live (video/GIF).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WallpaperMode {
    Static,
    Live,
}

impl Default for WallpaperMode {
    fn default() -> Self {
        WallpaperMode::Static
    }
}

impl std::str::FromStr for WallpaperMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "live" => Ok(WallpaperMode::Live),
            _ => Ok(WallpaperMode::Static),
        }
    }
}

impl std::fmt::Display for WallpaperMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallpaperMode::Static => write!(f, "static"),
            WallpaperMode::Live => write!(f, "live"),
        }
    }
}

/// Wallpaper file classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallpaperKind {
    StaticImage,
    Video,
    AnimatedGif,
}

impl WallpaperKind {
    pub fn from_path(path: &std::path::Path) -> Self {
        let ext = path.extension().and_then(|e| e.to_str()).map(|s| s.to_lowercase());
        match ext.as_deref() {
            Some("mp4" | "webm" | "mkv") => WallpaperKind::Video,
            Some("gif") => WallpaperKind::AnimatedGif,
            Some("png" | "jpg" | "jpeg" | "webp") => WallpaperKind::StaticImage,
            _ => WallpaperKind::StaticImage,
        }
    }

    pub fn is_live(&self) -> bool {
        matches!(self, WallpaperKind::Video | WallpaperKind::AnimatedGif)
    }

    pub fn mode(&self) -> WallpaperMode {
        if self.is_live() {
            WallpaperMode::Live
        } else {
            WallpaperMode::Static
        }
    }
}

/// Monitor resolution with scale factor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonitorResolution {
    pub width: i32,
    pub height: i32,
    pub scale: i32,
}

impl MonitorResolution {
    pub fn logical_size(&self) -> (i32, i32) {
        (self.width * self.scale, self.height * self.scale)
    }
}
