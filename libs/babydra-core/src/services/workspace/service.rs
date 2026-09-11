pub use crate::models::desktop::workspace::{WorkspaceReceiver, WorkspaceSnapshot};
use crate::models::DesktopApp;
use crate::services::window::get_active_window;
use crate::services::window::mru::get_running_apps;
use crate::services::workspace::{filter_apps_for_workspace, get_current_workspace};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;

static WORKSPACE_SENDERS: Mutex<Vec<std::sync::mpsc::Sender<WorkspaceSnapshot>>> = Mutex::new(Vec::new());
static WORKSPACE_STARTED: AtomicBool = AtomicBool::new(false);
static LAST_SIGNATURE: Mutex<String> = Mutex::new(String::new());

fn get_active_app_id() -> Option<String> {
    get_active_window().map(|(app_id, _)| app_id)
}

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
    let all_apps = get_running_apps();
    let active_app_id = get_active_app_id();
    let current_ws = get_current_workspace();
    let ws_apps = filter_apps_for_workspace(current_ws, &all_apps, current_ws);

    WorkspaceSnapshot {
        current_workspace: current_ws,
        apps: ws_apps,
        active_app_id,
    }
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

        std::thread::sleep(Duration::from_millis(200));
        let snapshot = collect_workspace_snapshot();
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