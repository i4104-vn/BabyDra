pub use crate::models::desktop::workspace::{WorkspaceReceiver, WorkspaceSnapshot};
use crate::models::DesktopApp;
use crate::services::window::mru::get_running_apps_with_active;
use crate::services::workspace::{
    get_app_workspace, get_current_workspace, sync_workspace_apps, DEFAULT_WORKSPACE_COUNT,
};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;

static WORKSPACE_SENDERS: Mutex<Vec<std::sync::mpsc::Sender<WorkspaceSnapshot>>> =
    Mutex::new(Vec::new());
static WORKSPACE_STARTED: AtomicBool = AtomicBool::new(false);
static LAST_SIGNATURE: Mutex<String> = Mutex::new(String::new());
static LATEST_SNAPSHOT: Mutex<Option<WorkspaceSnapshot>> = Mutex::new(None);

pub fn get_apps_signature(ws_id: u32, running_apps: &[DesktopApp]) -> String {
    let mut counts = HashMap::new();
    for app in running_apps {
        let app_id = app.app_id.clone().unwrap_or_else(|| app.name.clone());
        *counts.entry(app_id).or_insert(0) += 1;
    }
    let mut sigs: Vec<String> = counts.iter().map(|(k, v)| format!("{}:{}", k, v)).collect();
    sigs.sort();
    format!("ws:{}||{}", ws_id, sigs.join("||"))
}

pub fn collect_workspace_snapshot() -> WorkspaceSnapshot {
    let (all_apps, active_window) = get_running_apps_with_active();
    let current_ws = get_current_workspace();
    let workspace_map = sync_workspace_apps(current_ws, &all_apps);
    let workspace_apps = (1..=DEFAULT_WORKSPACE_COUNT)
        .map(|workspace_id| {
            all_apps
                .iter()
                .filter(|app| {
                    get_app_workspace(app, &workspace_map, current_ws) == workspace_id
                })
                .cloned()
                .collect()
        })
        .collect::<Vec<Vec<DesktopApp>>>();
    let apps = workspace_apps
        .get(current_ws.saturating_sub(1) as usize)
        .cloned()
        .unwrap_or_default();

    WorkspaceSnapshot {
        current_workspace: current_ws,
        workspace_apps,
        apps,
        active_app_id: active_window.as_ref().map(|(app_id, _)| app_id.clone()),
        active_window_title: active_window.map(|(_, title)| title),
    }
}

/// Returns the most recently collected snapshot without performing I/O.
pub fn latest_snapshot() -> Option<WorkspaceSnapshot> {
    LATEST_SNAPSHOT.lock().unwrap().clone()
}

pub fn subscribe() -> WorkspaceReceiver {
    init_workspace_service();
    let (tx, rx) = std::sync::mpsc::channel();
    WORKSPACE_SENDERS.lock().unwrap().push(tx);
    WorkspaceReceiver::new(rx)
}

pub fn init_workspace_service() {
    if WORKSPACE_STARTED.swap(true, Ordering::SeqCst) {
        return;
    }
    std::thread::spawn(move || loop {
        let has_subscribers = {
            let senders = WORKSPACE_SENDERS.lock().unwrap();
            !senders.is_empty()
        };

        if !has_subscribers {
            std::thread::sleep(Duration::from_millis(1000));
            continue;
        }

        std::thread::sleep(Duration::from_millis(350));
        let snapshot = collect_workspace_snapshot();
        *LATEST_SNAPSHOT.lock().unwrap() = Some(snapshot.clone());
        let sig = format!(
            "{}:{:?}:{}",
            snapshot.current_workspace,
            snapshot.active_app_id,
            get_apps_signature(snapshot.current_workspace, &snapshot.apps)
        );

        let changed = {
            let mut last = LAST_SIGNATURE.lock().unwrap();
            if *last != sig {
                *last = sig;
                true
            } else {
                false
            }
        };

        if changed {
            let mut senders = WORKSPACE_SENDERS.lock().unwrap();
            senders.retain(|tx| tx.send(snapshot.clone()).is_ok());
        }
    });
}
