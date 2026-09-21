//! Global shortcut (babydra-keymap) configuration service.
//!
//! Shortcuts live in `~/.babydra/keymap.toml`.  The `babydra-keymap`
//! daemon hot-reloads the file whenever it changes, so saving from Settings
//! takes effect immediately.

pub mod catalog;
pub mod config;
pub mod labwc;
pub mod service;

pub use catalog::{SystemCrateDef, SYSTEM_CRATE_CATALOG};
pub use config::{
    get_config_path, is_paused, pause_shortcuts, resume_shortcuts, PAUSE_TIMEOUT_SECS,
};
pub use labwc::sync_labwc_swallow;
pub use service::{
    get_custom_shortcuts, get_shortcuts, get_system_shortcuts, save_keymap_configuration,
    save_shortcuts,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_crate_catalog_has_all_apps() {
        assert_eq!(SYSTEM_CRATE_CATALOG.len(), 7);
        let ids: Vec<&str> = SYSTEM_CRATE_CATALOG.iter().map(|d| d.id).collect();
        assert!(ids.contains(&"launcher"));
        assert!(ids.contains(&"switcher"));
        assert!(ids.contains(&"screenshot"));
        assert!(ids.contains(&"recording_island"));
        assert!(ids.contains(&"lock"));
        assert!(ids.contains(&"settings"));
        assert!(ids.contains(&"explore"));
    }

    #[test]
    fn test_system_crate_defaults_non_empty() {
        for def in SYSTEM_CRATE_CATALOG {
            assert!(!def.id.is_empty());
            assert!(!def.crate_name.is_empty());
            assert!(!def.command.is_empty());
            assert!(!def.default_key.is_empty());
        }
    }
}
