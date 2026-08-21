//! Lightweight, debounced persistence for desktop icon positions.
//!
//! Positions are kept in a `LazyLock<RwLock<HashMap>>` in-memory cache.
//! All mutations go into the cache instantly (zero disk I/O on the GTK thread).
//! A background debounce timer coalesces rapid changes and flushes to
//! `~/.babydra/configs/desktop_layout.json` after 500ms of quiet time.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, RwLock};

static POSITIONS: LazyLock<RwLock<HashMap<String, (i32, i32)>>> =
    LazyLock::new(|| RwLock::new(load_from_disk()));
static DIRTY: AtomicBool = AtomicBool::new(false);

fn layout_path() -> PathBuf {
    crate::config::get_config_dir().join("desktop_layout.json")
}

fn load_from_disk() -> HashMap<String, (i32, i32)> {
    std::fs::read_to_string(layout_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

/// Read all positions (zero-copy from cache, no disk I/O).
pub fn get_all_positions() -> HashMap<String, (i32, i32)> {
    POSITIONS.read().unwrap().clone()
}

/// Read one position.
pub fn get_position(name: &str) -> Option<(i32, i32)> {
    POSITIONS.read().unwrap().get(name).copied()
}

/// Set position — instant in-memory, marks dirty for async flush.
pub fn set_position(name: String, x: i32, y: i32) {
    let mut map = POSITIONS.write().unwrap();
    if map.get(&name) != Some(&(x, y)) {
        map.insert(name, (x, y));
        DIRTY.store(true, Ordering::Relaxed);
    }
}

/// Set multiple positions at once — single lock acquisition.
pub fn set_positions(batch: Vec<(String, i32, i32)>) {
    let mut map = POSITIONS.write().unwrap();
    let mut changed = false;
    for (name, x, y) in batch {
        if map.get(&name) != Some(&(x, y)) {
            map.insert(name, (x, y));
            changed = true;
        }
    }
    if changed {
        DIRTY.store(true, Ordering::Relaxed);
    }
}

/// Remove position entries that no longer exist on disk.
pub fn cleanup_stale(existing_names: &[String]) {
    let mut map = POSITIONS.write().unwrap();
    let set: std::collections::HashSet<&String> = existing_names.iter().collect();
    let initial_len = map.len();
    map.retain(|k, _| set.contains(k));
    if map.len() != initial_len {
        DIRTY.store(true, Ordering::Relaxed);
    }
}

/// Clears all saved positions.
pub fn clear() {
    let mut map = POSITIONS.write().unwrap();
    map.clear();
    DIRTY.store(true, Ordering::Relaxed);
}

/// Flush to disk if dirty. Called by debounce timer.
pub fn flush_if_dirty() {
    if DIRTY
        .compare_exchange(true, false, Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
    {
        let data = POSITIONS.read().unwrap().clone();
        let path = layout_path();

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        // Fire-and-forget async write
        std::thread::spawn(move || {
            if let Ok(json) = serde_json::to_string_pretty(&data) {
                let _ = std::fs::write(&path, json);
            }
        });
    }
}
