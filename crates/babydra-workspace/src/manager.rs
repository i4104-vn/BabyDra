use crate::models::Workspace;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub const DEFAULT_WORKSPACE_COUNT: u32 = 4;

/// Resolves the cache path for the currently active workspace.
pub fn get_workspace_cache_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_default();
    PathBuf::from(home)
        .join(".cache")
        .join("babydra")
        .join("current_workspace")
}

/// Reads the currently active workspace ID (1-indexed). Defaults to 1.
pub fn get_current_workspace() -> u32 {
    let path = get_workspace_cache_path();
    if let Ok(content) = fs::read_to_string(&path) {
        if let Ok(id) = content.trim().parse::<u32>() {
            if id >= 1 && id <= DEFAULT_WORKSPACE_COUNT {
                return id;
            }
        }
    }
    1
}

/// Sets the currently active workspace ID in cache.
pub fn set_cached_workspace(id: u32) {
    let path = get_workspace_cache_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, id.to_string());
}

/// Returns the list of all workspaces with active state populated.
pub fn get_workspaces() -> Vec<Workspace> {
    let active_id = get_current_workspace();
    (1..=DEFAULT_WORKSPACE_COUNT)
        .map(|id| {
            let name = format!("Workspace {}", id);
            Workspace::new(id, &name, id == active_id)
        })
        .collect()
}

/// Dispatches compositor desktop switch command to labwc.
fn dispatch_compositor_switch(id: u32) {
    let key_str = id.to_string();

    // 1. Try wtype: Super + <id>
    let wtype_paths = ["/home/i4104/.local/bin/wtype", "wtype"];
    for bin in &wtype_paths {
        if let Ok(status) = Command::new(bin)
            .args(&["-M", "logo", "-k", &key_str, "-m", "logo"])
            .status()
        {
            if status.success() {
                return;
            }
        }
    }

    // 2. Fallback to wlrctl keyboard type
    let _ = Command::new("wlrctl")
        .args(&["keyboard", "type", &key_str, "SUPER"])
        .status();
}

/// Switches to the specified workspace ID (1..=4).
pub fn switch_workspace(id: u32) -> bool {
    if id < 1 || id > DEFAULT_WORKSPACE_COUNT {
        return false;
    }
    set_cached_workspace(id);
    dispatch_compositor_switch(id);
    true
}

/// Cycles to the next workspace. Returns the new active ID.
pub fn next_workspace() -> u32 {
    let current = get_current_workspace();
    let next = if current >= DEFAULT_WORKSPACE_COUNT {
        1
    } else {
        current + 1
    };
    switch_workspace(next);
    next
}

/// Cycles to the previous workspace. Returns the new active ID.
pub fn prev_workspace() -> u32 {
    let current = get_current_workspace();
    let prev = if current <= 1 {
        DEFAULT_WORKSPACE_COUNT
    } else {
        current - 1
    };
    switch_workspace(prev);
    prev
}
