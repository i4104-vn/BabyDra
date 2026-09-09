pub mod cache;
pub mod compositor;
pub mod windows;

use crate::models::Workspace;
pub use cache::{
    get_workspace_cache_dir, get_workspace_cache_path, read_cached_workspace,
    reset_cached_workspace, write_cached_workspace, DEFAULT_WORKSPACE_COUNT,
};
pub use compositor::dispatch_compositor_switch;
pub use windows::{filter_apps_for_workspace, get_app_workspace, sync_workspace_apps};

pub fn get_current_workspace() -> u32 {
    read_cached_workspace().unwrap_or(1)
}

pub fn set_cached_workspace(id: u32) {
    write_cached_workspace(id);
}

pub fn get_workspaces() -> Vec<Workspace> {
    let active_id = get_current_workspace();
    (1..=DEFAULT_WORKSPACE_COUNT)
        .map(|id| {
            let name = format!("Workspace {}", id);
            Workspace::new(id, &name, id == active_id)
        })
        .collect()
}

pub fn switch_workspace(id: u32) -> bool {
    if id < 1 || id > DEFAULT_WORKSPACE_COUNT {
        return false;
    }
    write_cached_workspace(id);
    dispatch_compositor_switch(id);
    true
}

pub fn set_workspace_sync_only(id: u32) -> bool {
    if id < 1 || id > DEFAULT_WORKSPACE_COUNT {
        return false;
    }
    write_cached_workspace(id);
    true
}

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

pub fn next_workspace_sync_only() -> u32 {
    let current = get_current_workspace();
    let next = if current >= DEFAULT_WORKSPACE_COUNT {
        1
    } else {
        current + 1
    };
    write_cached_workspace(next);
    next
}

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

pub fn prev_workspace_sync_only() -> u32 {
    let current = get_current_workspace();
    let prev = if current <= 1 {
        DEFAULT_WORKSPACE_COUNT
    } else {
        current - 1
    };
    write_cached_workspace(prev);
    prev
}
