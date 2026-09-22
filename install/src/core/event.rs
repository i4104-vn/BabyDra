use std::path::PathBuf;

use crate::models::{BinaryItem, BranchItem, LogMessage, VariantItem};

/// Events sent between the installer worker/background tasks and the TUI event loop.
#[derive(Debug, Clone)]
pub enum InstallEvent {
    /// Branch refs were refreshed without blocking the TUI.
    BranchesUpdated {
        branches: Vec<BranchItem>,
    },
    /// A background source discovery request completed.
    DiscoveryUpdated {
        request_id: u64,
        source_root: PathBuf,
        source_binary_dir: PathBuf,
        binaries: Vec<BinaryItem>,
        variants: Vec<VariantItem>,
    },
    Progress {
        current: usize,
        total: usize,
        current_step_name: String,
    },
    Log(LogMessage),
    /// Sudo pre-authentication failed. The TUI re-opens the password modal
    /// with the error message instead of aborting the whole install.
    SudoFailed(String),
    /// Background branch checkout & pull completed.
    BranchSwitched {
        success: bool,
        error_msg: Option<String>,
    },
    Completed {
        success: bool,
        total_copied: usize,
        total_errors: usize,
        duration_secs: f64,
    },
}

/// Execution parameters constructed by the UI before spawning the install worker.
#[derive(Debug, Clone)]
pub struct InstallPlan {
    pub workspace_root: PathBuf,
    pub source_root: PathBuf,
    pub source_binary_dir: PathBuf,
    pub selected_binaries: Vec<BinaryItem>,
    /// True when the user left every discovered binary selected. In that
    /// case a branch update may add new binaries between the UI scan and the
    /// actual build, so the worker can include them automatically.
    pub install_all_binaries: bool,
    /// Variant selected in step 4 (theme + app list + keybinds source).
    pub variant: VariantItem,
    /// Branch to check out + pull before building (empty = skip git step).
    pub branch: String,
    /// Sudo password provided by the user (None when running as root).
    pub sudo_password: Option<String>,
}
