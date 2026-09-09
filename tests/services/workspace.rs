//! Integration tests: Workspace manager and models.

use babydra_workspace::manager::{
    get_current_workspace, get_workspaces, next_workspace, prev_workspace, switch_workspace,
    DEFAULT_WORKSPACE_COUNT,
};
use babydra_workspace::models::Workspace;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_workspace_model_creation() {
    let ws = Workspace::new(1, "Workspace 1", true);
    assert_eq!(ws.id, 1);
    assert_eq!(ws.name, "Workspace 1");
    assert_eq!(ws.icon, "workspace-1");
    assert!(ws.is_active);

    let ws4 = Workspace::new(4, "Workspace 4", false);
    assert_eq!(ws4.id, 4);
    assert_eq!(ws4.icon, "workspace-4");
    assert!(!ws4.is_active);

    let json = serde_json::to_string(&ws).expect("Serialize workspace");
    let deserialized: Workspace = serde_json::from_str(&json).expect("Deserialize workspace");
    assert_eq!(ws, deserialized);
}

#[test]
fn test_get_workspaces_list() {
    let _guard = LOCK.lock().unwrap();
    let list = get_workspaces();
    assert_eq!(list.len(), DEFAULT_WORKSPACE_COUNT as usize);

    // Exactly one workspace must be active
    let active_count = list.iter().filter(|w| w.is_active).count();
    assert_eq!(active_count, 1);

    for (idx, ws) in list.iter().enumerate() {
        assert_eq!(ws.id, (idx + 1) as u32);
        assert_eq!(ws.name, format!("Workspace {}", idx + 1));
    }
}

#[test]
fn test_workspace_switching_and_cycling() {
    let _guard = LOCK.lock().unwrap();

    // Out of bounds
    assert!(!switch_workspace(0));
    assert!(!switch_workspace(DEFAULT_WORKSPACE_COUNT + 1));

    // Valid switches
    assert!(switch_workspace(2));
    assert_eq!(get_current_workspace(), 2);

    assert!(switch_workspace(1));
    assert_eq!(get_current_workspace(), 1);

    // 1 -> 2
    assert_eq!(next_workspace(), 2);
    assert_eq!(get_current_workspace(), 2);

    // 2 -> 3
    assert_eq!(next_workspace(), 3);
    assert_eq!(get_current_workspace(), 3);

    // 3 -> 4
    assert_eq!(next_workspace(), 4);
    assert_eq!(get_current_workspace(), 4);

    // 4 -> wraps to 1
    assert_eq!(next_workspace(), 1);
    assert_eq!(get_current_workspace(), 1);

    // 1 -> wraps to 4
    assert_eq!(prev_workspace(), 4);
    assert_eq!(get_current_workspace(), 4);

    // 4 -> 3
    assert_eq!(prev_workspace(), 3);
    assert_eq!(get_current_workspace(), 3);

    // Reset back to 1
    switch_workspace(1);
    assert_eq!(get_current_workspace(), 1);
}
