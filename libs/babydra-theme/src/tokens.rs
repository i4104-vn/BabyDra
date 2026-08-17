//! Design token models for theme packages.

use serde::{Deserialize, Serialize};

/// Radius token values.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RadiusTokens {
    #[serde(default)]
    pub pill: u32,
    #[serde(default)]
    pub lg: u32,
    #[serde(default)]
    pub md: u32,
    #[serde(default)]
    pub sm: u32,
}

impl RadiusTokens {
    fn merge(&mut self, other: &RadiusTokens) {
        if other.pill != 0 {
            self.pill = other.pill;
        }
        if other.lg != 0 {
            self.lg = other.lg;
        }
        if other.md != 0 {
            self.md = other.md;
        }
        if other.sm != 0 {
            self.sm = other.sm;
        }
    }
}

/// Tokens for one color mode (dark or light).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DarkLightTokens {
    #[serde(default)]
    pub surface: String,
    #[serde(default)]
    pub border: String,
    #[serde(default)]
    pub accent: String,
    #[serde(default)]
    pub font: String,
    #[serde(default)]
    pub radius: RadiusTokens,
}

impl DarkLightTokens {
    /// Merge `other` on top of `self` — values present in `other` win.
    pub(crate) fn merge(&mut self, other: &DarkLightTokens) {
        if !other.surface.is_empty() {
            self.surface.clone_from(&other.surface);
        }
        if !other.border.is_empty() {
            self.border.clone_from(&other.border);
        }
        if !other.accent.is_empty() {
            self.accent.clone_from(&other.accent);
        }
        if !other.font.is_empty() {
            self.font.clone_from(&other.font);
        }
        self.radius.merge(&other.radius);
    }
}

/// Full `tokens.json` payload.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ThemeTokens {
    #[serde(default)]
    pub name: String,
    /// Id of the base theme this one inherits from (optional).
    #[serde(default)]
    pub base: Option<String>,
    #[serde(default)]
    pub dark: DarkLightTokens,
    #[serde(default)]
    pub light: DarkLightTokens,
}
