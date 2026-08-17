//! Glassmorphic theme configuration model.

use serde::{Deserialize, Serialize};

fn default_blur_radius() -> u32 {
    20
}
fn default_opacity() -> f64 {
    0.75
}
fn default_border_color() -> String {
    "#ffffff".to_string()
}
fn default_border_width() -> u32 {
    1
}

/// Theme package selection: which `themes/<id>` to use and dark/light mode.
///
/// `id == ""` means "engine default theme" (backward compatible with configs
/// written before theme packages existed). `dark == None` means "follow the
/// system default".
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ThemeSelection {
    /// Theme package id (`themes/<id>`). Empty = engine default.
    #[serde(default)]
    pub id: String,
    /// Dark mode preference. `None` = follow system default.
    #[serde(default)]
    pub dark: Option<bool>,
}

/// Theme visual configuration values for the glassmorphic design system.
///
/// The four legacy visual fields are kept for compatibility; new theme-driven
/// deployments should use `selection` to point at a `themes/` package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeConfig {
    /// Background blur radius in pixels.
    #[serde(default = "default_blur_radius")]
    pub blur_radius: u32,
    /// Panel background opacity (0.0 – 1.0).
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    /// CSS hex color string for widget borders.
    #[serde(default = "default_border_color")]
    pub border_color: String,
    /// Border thickness in pixels.
    #[serde(default = "default_border_width")]
    pub border_width: u32,
    /// Theme package selection (id + dark mode). Empty id = engine default.
    #[serde(default)]
    pub selection: ThemeSelection,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            blur_radius: default_blur_radius(),
            opacity: default_opacity(),
            border_color: default_border_color(),
            border_width: default_border_width(),
            selection: ThemeSelection::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ShellConfig;

    #[test]
    fn old_config_without_selection_still_deserializes() {
        let old = r##"{
            "theme": {
                "blur_radius": 24,
                "opacity": 0.8,
                "border_color": "#ff0000",
                "border_width": 2
            }
        }"##;
        let cfg: ShellConfig = serde_json::from_str(old).expect("old config must parse");
        assert_eq!(cfg.theme.blur_radius, 24);
        assert_eq!(cfg.theme.opacity, 0.8);
        assert_eq!(cfg.theme.border_color, "#ff0000");
        assert_eq!(cfg.theme.border_width, 2);
        assert!(cfg.theme.selection.id.is_empty());
        assert_eq!(cfg.theme.selection.dark, None);
    }

    #[test]
    fn empty_config_uses_defaults() {
        let cfg: ShellConfig = serde_json::from_str("{}").expect("empty config must parse");
        assert_eq!(cfg.theme.blur_radius, default_blur_radius());
        assert_eq!(cfg.theme.opacity, default_opacity());
        assert_eq!(cfg.theme.border_width, default_border_width());
    }

    #[test]
    fn new_config_with_selection_roundtrips() {
        let json = r##"{
            "theme": {
                "blur_radius": 30,
                "selection": { "id": "babydra-blue", "dark": false }
            }
        }"##;
        let cfg: ShellConfig = serde_json::from_str(json).expect("new config must parse");
        assert_eq!(cfg.theme.selection.id, "babydra-blue");
        assert_eq!(cfg.theme.selection.dark, Some(false));
        assert_eq!(cfg.theme.blur_radius, 30);
        // untouched legacy fields fall back to defaults
        assert_eq!(cfg.theme.border_width, default_border_width());
    }
}
