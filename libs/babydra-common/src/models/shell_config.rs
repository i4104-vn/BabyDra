//! Top-level desktop shell configuration model.

use serde::{Deserialize, Serialize};
use super::theme_config::ThemeConfig;

/// Root configuration struct for the BabyDra shell.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShellConfig {
    /// Theming configuration (blur, opacity, border).
    pub theme: ThemeConfig,
}
