use std::fs;
use std::path::PathBuf;

pub const DEFAULT_WORKSPACE_COUNT: u32 = 4;

pub fn get_workspace_cache_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("BABYDRA_CACHE_DIR") {
        return PathBuf::from(dir);
    }
    dirs::cache_dir()
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_default();
            PathBuf::from(home).join(".cache")
        })
        .join("babydra")
}

pub fn get_workspace_cache_path() -> PathBuf {
    get_workspace_cache_dir().join("current_workspace")
}

pub fn read_cached_workspace() -> Option<u32> {
    let path = get_workspace_cache_path();
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(id) = content.trim().parse::<u32>() {
            if id >= 1 && id <= DEFAULT_WORKSPACE_COUNT {
                return Some(id);
            }
        }
    }
    None
}

pub fn write_cached_workspace(id: u32) {
    if id < 1 || id > DEFAULT_WORKSPACE_COUNT {
        return;
    }
    let path = get_workspace_cache_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, id.to_string());
}

pub fn reset_cached_workspace() {
    write_cached_workspace(1);
    let win_path = get_workspace_cache_dir().join("workspace_windows.json");
    let _ = fs::remove_file(win_path);
}
