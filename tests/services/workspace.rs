//! Integration tests: Workspace manager and models.

use babydra_core::{
    get_current_workspace, get_workspaces, next_workspace, prev_workspace, switch_workspace,
    DEFAULT_WORKSPACE_COUNT,
};
use babydra_core::models::Workspace;
use std::sync::Mutex;

static LOCK: Mutex<()> = Mutex::new(());

fn init_test_env() -> std::sync::MutexGuard<'static, ()> {
    let guard = LOCK.lock().unwrap();
    let temp_dir = std::env::temp_dir().join("babydra_test_cache");
    let _ = std::fs::create_dir_all(&temp_dir);
    std::env::set_var("BABYDRA_CACHE_DIR", temp_dir);
    guard
}

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
    let _guard = init_test_env();
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
    let _guard = init_test_env();

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

#[test]
fn test_workspace_cache_reset_and_sync() {
    let _guard = init_test_env();

    babydra_core::set_workspace_sync_only(3);
    assert_eq!(get_current_workspace(), 3);

    babydra_core::next_workspace_sync_only();
    assert_eq!(get_current_workspace(), 4);

    babydra_core::next_workspace_sync_only();
    assert_eq!(get_current_workspace(), 1);

    babydra_core::prev_workspace_sync_only();
    assert_eq!(get_current_workspace(), 4);

    babydra_core::reset_cached_workspace();
    assert_eq!(get_current_workspace(), 1);
}

#[test]
fn test_sync_workspace_apps_title_stability() {
    let _guard = init_test_env();
    babydra_core::reset_cached_workspace();

    let mut app = babydra_core::DesktopApp {
        name: "Browser".into(),
        exec: "browser".into(),
        icon: None,
        file_path: None,
        is_dependency: false,
        app_id: Some("browser".into()),
        window_title: Some("Tab 1".into()),
    };

    // First opened on workspace 1
    let apps = vec![app.clone()];
    let map = babydra_core::sync_workspace_apps(1, &apps);
    assert_eq!(babydra_core::get_app_workspace(&app, &map, 1), 1);

    // Switch to workspace 2, browser title changes in background to "Tab 2"
    app.window_title = Some("Tab 2".into());
    let apps_updated = vec![app.clone()];
    let map_updated = babydra_core::sync_workspace_apps(2, &apps_updated);
    // Should still stay on workspace 1
    assert_eq!(babydra_core::get_app_workspace(&app, &map_updated, 2), 1);

    babydra_core::reset_cached_workspace();
}

#[test]
fn test_core_workspace_service_parity() {
    let _guard = init_test_env();
    babydra_core::reset_cached_workspace();
    assert_eq!(babydra_core::get_current_workspace(), 1);

    babydra_core::set_workspace_sync_only(3);
    assert_eq!(babydra_core::get_current_workspace(), 3);
    assert_eq!(babydra_workspace::get_current_workspace(), 3);

    let workspaces = babydra_core::get_workspaces();
    assert_eq!(workspaces.len(), babydra_core::DEFAULT_WORKSPACE_COUNT as usize);
    assert!(workspaces[2].is_active);

    babydra_core::reset_cached_workspace();
    assert_eq!(babydra_core::get_current_workspace(), 1);
}

#[test]
fn test_window_stays_on_creation_workspace_when_switching_workspaces() {
    let _guard = init_test_env();
    babydra_core::reset_cached_workspace();

    let app = babydra_core::DesktopApp {
        name: "Chromium".into(),
        exec: "chromium".into(),
        icon: None,
        file_path: None,
        is_dependency: false,
        app_id: Some("chromium".into()),
        window_title: Some("Google".into()),
    };

    let apps = vec![app.clone()];
    let map = babydra_core::sync_workspace_apps(1, &apps);
    assert_eq!(babydra_core::get_app_workspace(&app, &map, 1), 1);

    // Switch to workspace 3
    let map3 = babydra_core::sync_workspace_apps(3, &apps);
    assert_eq!(babydra_core::get_app_workspace(&app, &map3, 3), 1);

    // Filter for workspace 3: should NOT contain app from workspace 1
    let ws3_apps = babydra_core::filter_apps_for_workspace(3, &apps, 3);
    assert!(ws3_apps.is_empty());

    // Filter for workspace 1: MUST contain app from workspace 1
    let ws1_apps = babydra_core::filter_apps_for_workspace(1, &apps, 3);
    assert_eq!(ws1_apps.len(), 1);
    assert_eq!(ws1_apps[0].name, "Chromium");

    babydra_core::reset_cached_workspace();
}

#[test]
fn test_new_window_on_different_workspace() {
    let _guard = init_test_env();
    babydra_core::reset_cached_workspace();

    let app1 = babydra_core::DesktopApp {
        name: "Terminal".into(),
        exec: "kitty".into(),
        icon: None,
        file_path: None,
        is_dependency: false,
        app_id: Some("kitty".into()),
        window_title: Some("Terminal 1".into()),
    };

    // Terminal 1 opened on WS 1
    let apps_ws1 = vec![app1.clone()];
    let map = babydra_core::sync_workspace_apps(1, &apps_ws1);
    assert_eq!(babydra_core::get_app_workspace(&app1, &map, 1), 1);

    // Switch to WS 2, Terminal 2 opened on WS 2
    let app2 = babydra_core::DesktopApp {
        name: "Terminal".into(),
        exec: "kitty".into(),
        icon: None,
        file_path: None,
        is_dependency: false,
        app_id: Some("kitty".into()),
        window_title: Some("Terminal 2".into()),
    };

    let both_apps = vec![app1.clone(), app2.clone()];
    let map2 = babydra_core::sync_workspace_apps(2, &both_apps);

    assert_eq!(babydra_core::get_app_workspace(&app1, &map2, 2), 1);
    assert_eq!(babydra_core::get_app_workspace(&app2, &map2, 2), 2);

    let ws1_filtered = babydra_core::filter_apps_for_workspace(1, &both_apps, 2);
    assert_eq!(ws1_filtered.len(), 1);
    assert_eq!(ws1_filtered[0].window_title.as_deref(), Some("Terminal 1"));

    let ws2_filtered = babydra_core::filter_apps_for_workspace(2, &both_apps, 2);
    assert_eq!(ws2_filtered.len(), 1);
    assert_eq!(ws2_filtered[0].window_title.as_deref(), Some("Terminal 2"));

    babydra_core::reset_cached_workspace();
}

#[test]
fn test_empty_apps_does_not_wipe_map() {
    let _guard = init_test_env();
    babydra_core::reset_cached_workspace();

    let app = babydra_core::DesktopApp {
        name: "App".into(),
        exec: "app".into(),
        icon: None,
        file_path: None,
        is_dependency: false,
        app_id: Some("app".into()),
        window_title: Some("Window".into()),
    };

    let apps = vec![app.clone()];
    babydra_core::sync_workspace_apps(1, &apps);

    // When empty list passed (e.g. background query loading)
    let empty_map = babydra_core::sync_workspace_apps(2, &[]);
    assert!(!empty_map.is_empty());
    assert_eq!(empty_map.get("app::Window"), Some(&1));

    babydra_core::reset_cached_workspace();
}
