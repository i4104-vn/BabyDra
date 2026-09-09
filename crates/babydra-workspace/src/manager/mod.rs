pub mod cache;
pub mod compositor;

use crate::models::Workspace;
pub use cache::{get_workspace_cache_path, read_cached_workspace, write_cached_workspace};
pub use compositor::dispatch_compositor_switch;

pub const DEFAULT_WORKSPACE_COUNT: u32 = 4;

/// Reads the currently active workspace ID (1-indexed). Defaults to 1.
pub fn get_current_workspace() -> u32 {
    if let Some(id) = read_cached_workspace() {
        if id >= 1 && id <= DEFAULT_WORKSPACE_COUNT {
            return id;
        }
    }
    1
}

/// Sets the currently active workspace ID in cache.
pub fn set_cached_workspace(id: u32) {
    write_cached_workspace(id);
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

/// Switches to the specified workspace ID (1..=DEFAULT_WORKSPACE_COUNT).
pub fn switch_workspace(id: u32) -> bool {
    if id < 1 || id > DEFAULT_WORKSPACE_COUNT {
        return false;
    }
    write_cached_workspace(id);
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
