//! Configuration path resolvers.

use std::path::PathBuf;

/// Resolves the absolute directory path to the user's config folder: `~/.config/babydra/`.
pub fn get_babydra_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".config")
        })
        .join("babydra")
}

