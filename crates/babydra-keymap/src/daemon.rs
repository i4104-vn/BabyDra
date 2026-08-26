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

/// Loads shortcuts via the core keymap service and logs invalid entries.
fn load() -> Vec<Shortcut> {
    let all = babydra_core::services::system::keymap::get_shortcuts();
    let mut valid = Vec::new();

    for sc in all {
        if sc.key.is_empty() || sc.command.trim().is_empty() {
            tracing::warn!("skipping shortcut #{}: empty key or command", sc.id);
            continue;
        }
        valid.push(sc);
    }

    tracing::info!("loaded {} shortcut(s)", valid.len());
    valid
}

/// Watches the config file and hot-reloads it into `shared` on change.
async fn watch(shared: SharedShortcuts) {
    let path = babydra_core::services::system::keymap::get_config_path();
    let mut last_modified = file_mtime(&path);

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let mtime = file_mtime(&path);
        if mtime != last_modified && mtime.is_some() {
            last_modified = mtime;
            tracing::info!("config changed, reloading");
            *shared.write().unwrap() = load();
        }
    }
}

fn file_mtime(path: &std::path::Path) -> Option<std::time::SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}
