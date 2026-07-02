//! Glassmorphic theme configuration model.

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

