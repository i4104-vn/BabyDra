//! Top-level desktop shell configuration model.

use super::theme_config::ThemeConfig;
use serde::{Deserialize, Serialize};

/// Root configuration struct for the BabyDra shell.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShellConfig {
    /// Theming configuration (blur, opacity, border).
    #[serde(default)]
    pub theme: ThemeConfig,
}
