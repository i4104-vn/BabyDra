//! Daemon orchestration for `babydra-keymap`.
//!
//! Loads shortcuts through the shared `babydra-core` keymap service, spawns
//! the config watcher and hands control to the evdev keyboard listener.

use crate::keyboard;
use babydra_core::models::shortcut::Shortcut;
use std::sync::{Arc, RwLock};

/// Type alias for the shared, hot-reloadable shortcut list.
pub type SharedShortcuts = Arc<RwLock<Vec<Shortcut>>>;

/// Daemon entry point: loads config, starts hot-reload and keyboard listening.
pub async fn run() {
    let shortcuts: SharedShortcuts = Arc::new(RwLock::new(load()));

    // Hot-reload config in the background (picks up Settings edits).
    tokio::spawn(watch(shortcuts.clone()));

    tracing::info!("babydra-keymap daemon started");
    keyboard::run(shortcuts).await;
}

/// Loads all valid global shortcuts, merging custom keymap with active feature shortcuts.
fn load() -> Vec<Shortcut> {
    let mut shortcuts = babydra_core::services::system::keymap::get_shortcuts();
    if let Some(clipboard_sc) = babydra_core::get_shortcut() {
        shortcuts.push(clipboard_sc);
    }
    shortcuts.retain(|s| !s.key.is_empty() && !s.command.trim().is_empty());
    tracing::info!("loaded {} shortcut(s)", shortcuts.len());
    shortcuts
}

/// Watches configuration files (keymap.toml and babydra.conf), hot-reloading on change.
async fn watch(shared: SharedShortcuts) {
    let paths = [
        babydra_core::services::system::keymap::get_config_path(),
        babydra_core::config::get_conf_path(),
    ];
    let mut last_mtimes = paths.each_ref().map(|p| file_mtime(p));

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let cur_mtimes = paths.each_ref().map(|p| file_mtime(p));
        if cur_mtimes != last_mtimes && cur_mtimes.iter().any(Option::is_some) {
            last_mtimes = cur_mtimes;
            tracing::info!("config changed, reloading shortcuts");
            *shared.write().unwrap() = load();
        }
    }
}

fn file_mtime(path: &std::path::Path) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}
