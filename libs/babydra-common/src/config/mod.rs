//! Configuration path resolvers and desktop shell models.

use std::path::PathBuf;

pub use crate::models::{ThemeConfig, ShellConfig};

pub mod settings;
pub use settings::{ExploreSettings, load_explore_settings, save_explore_settings};

/// Resolves the absolute directory path to the user's config folder: `~/.babydra/configs/`.
pub fn get_babydra_config_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".babydra")
        .join("configs")
}
