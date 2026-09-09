pub mod manager;
pub mod models;
pub mod switcher;

pub use manager::{
    get_current_workspace, get_workspaces, next_workspace, prev_workspace, switch_workspace,
    DEFAULT_WORKSPACE_COUNT,
};
pub use models::Workspace;
pub use switcher::{build_workspace_switcher_ui, try_signal_daemon, WORKSPACE_SOCKET_PATH};

/// Toggles the workspace switcher UI (signals running daemon, or launches one-shot).
pub fn toggle_switcher() {
    if !try_signal_daemon(b"toggle") {
        let _ = std::process::Command::new("babydra-workspace")
            .arg("show")
            .spawn();
    }
}

/// Shows the workspace switcher UI.
pub fn show_switcher() {
    if !try_signal_daemon(b"show") {
        let _ = std::process::Command::new("babydra-workspace")
            .arg("show")
            .spawn();
    }
}
