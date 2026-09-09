use std::fs;
use std::path::PathBuf;

/// Resolves the cache path for the currently active workspace.
pub fn get_workspace_cache_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home)
        .join(".cache")
        .join("babydra")
        .join("current_workspace")
}

/// Reads the cached workspace ID. Returns None if missing or invalid.
pub fn read_cached_workspace() -> Option<u32> {
    let path = get_workspace_cache_path();
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(id) = content.trim().parse::<u32>() {
            return Some(id);
        }
    }
    None
}

/// Sets the currently active workspace ID in cache.
pub fn write_cached_workspace(id: u32) {
    let path = get_workspace_cache_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, id.to_string());
}
