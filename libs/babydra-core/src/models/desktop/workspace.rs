use crate::models::desktop::app::DesktopApp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workspace {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub is_active: bool,
}

impl Workspace {
    pub fn new(id: u32, name: &str, is_active: bool) -> Self {
        let icon = match id {
            1 => "workspace-1",
            2 => "workspace-2",
            3 => "workspace-3",
            4 => "workspace-4",
            _ => "window",
        };
        Self {
            id,
            name: name.to_string(),
            icon: icon.to_string(),
            is_active,
        }
    }
}

/// Unified workspace state snapshot broadcast by WorkspaceService.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WorkspaceSnapshot {
    pub current_workspace: u32,
    /// Windows grouped by workspace, ordered from workspace 1 through 4.
    /// Keeping this in the snapshot lets UI consumers render workspace menus without
    /// synchronously querying the compositor again.
    pub workspace_apps: Vec<Vec<DesktopApp>>,
    pub apps: Vec<DesktopApp>,
    pub active_app_id: Option<String>,
    pub active_window_title: Option<String>,
}

impl WorkspaceSnapshot {
    pub fn apps_for_workspace(&self, workspace_id: u32) -> &[DesktopApp] {
        workspace_id
            .checked_sub(1)
            .and_then(|index| self.workspace_apps.get(index as usize))
            .map(Vec::as_slice)
            .unwrap_or_default()
    }
}

pub struct WorkspaceReceiver {
    rx: std::sync::mpsc::Receiver<WorkspaceSnapshot>,
}

impl WorkspaceReceiver {
    pub fn new(rx: std::sync::mpsc::Receiver<WorkspaceSnapshot>) -> Self {
        Self { rx }
    }

    pub fn attach<F: FnMut(WorkspaceSnapshot) -> glib::ControlFlow + 'static>(
        self,
        _context: Option<&()>,
        mut func: F,
    ) {
        glib::timeout_add_local(std::time::Duration::from_millis(150), move || {
            let mut latest = None;
            while let Ok(snap) = self.rx.try_recv() {
                latest = Some(snap);
            }
            if let Some(snap) = latest {
                func(snap)
            } else {
                glib::ControlFlow::Continue
            }
        });
    }
}
