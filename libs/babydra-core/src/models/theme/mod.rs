//! Theme selection, appearance and shell styling models.

pub mod appearance;
pub mod shell_config;
pub mod theme_config;
pub mod wallpaper;

pub use appearance::CurrentAppearance;
pub use shell_config::ShellConfig;
pub use theme_config::{ThemeConfig, ThemeSelection};
pub use wallpaper::{MonitorResolution, WallpaperKind, WallpaperMode};
