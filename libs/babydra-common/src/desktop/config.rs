//! Configuration path resolvers and desktop shell models.

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Theme visual configuration values for the glassmorphic design system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Background blur radius in pixels.
    pub blur_radius: u32,
    /// Panel background opacity (0.0 – 1.0).
    pub opacity: f64,
    /// CSS hex color string for widget borders.
    pub border_color: String,
    /// Border thickness in pixels.
    pub border_width: u32,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            blur_radius: 20,
            opacity: 0.75,
            border_color: "#ffffff".to_string(),
            border_width: 1,
        }
    }
}

/// Root configuration struct for the BabyDra shell.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShellConfig {
    /// Theming configuration (blur, opacity, border).
    pub theme: ThemeConfig,
}

/// Resolves the absolute directory path to the user's config folder: `~/.config/babydra/`.
pub fn get_babydra_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".config")
        })
        .join("babydra")
}
