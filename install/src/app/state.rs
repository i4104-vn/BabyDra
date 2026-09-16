use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::models::{
    BinaryItem, BranchItem, InstallState, LogLevel, LogMessage, VariantItem, WizardStep,
};
use crate::system::{
    branch_worktree_dir, default_binary_source_dir, find_workspace_root, list_branches,
};
use crate::tasks::InstallEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchSwitchStatus {
    Idle,
    Switching,
    Done(Result<(), String>),
}

pub struct App {
    pub current_step: WizardStep,

    // Step Data & Cursors
    pub binaries: Vec<BinaryItem>,
    pub binary_cursor: usize,

    pub branches: Vec<BranchItem>,
    pub branch_cursor: usize,
    /// Empty string = pre-built only; otherwise the branch to check out,
    /// pull and rebuild from source.
    pub selected_branch: String,

    pub variant_options: Vec<VariantItem>,
    pub variant_cursor: usize,
    pub selected_variant: String,
    pub discovery_in_progress: bool,
    pub discovery_request_id: u64,

    // Logs & Progress
    pub logs: Vec<LogMessage>,
    pub log_scroll: usize,
    pub auto_scroll_logs: bool,

    // Path & Environment
    pub workspace_root: PathBuf,
    pub source_binary_dir: PathBuf,
    pub custom_path_input: String,
    pub is_editing_path: bool,

    // Modals
    pub show_help: bool,
    pub show_confirm_dialog: bool,
    pub show_sudo_modal: bool,
    pub show_branch_switching_modal: bool,
    pub branch_switch_status: BranchSwitchStatus,
    pub branch_switch_spinner_tick: usize,

    // Sudo
    /// In-memory sudo password (masked in the UI, fed via `sudo -S` stdin).
    pub sudo_password: String,
    pub sudo_error: Option<String>,
    pub sudo_attempts: u32,

    // Execution State
    pub install_state: InstallState,
    pub progress_percent: u16,
    pub current_step_desc: String,

    pub tx: Sender<InstallEvent>,
    pub rx: Receiver<InstallEvent>,
    pub should_quit: bool,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let workspace_root = find_workspace_root();
        let source_binary_dir = default_binary_source_dir(&workspace_root);
        let branches = list_branches(&workspace_root);

        // Prefer the current source branch; otherwise use the first
        // installable branch discovered from git. No branch name is baked
        // into the installer.
        let branch_cursor = branches
            .iter()
            .position(|branch| branch.is_current)
            .unwrap_or(0);
        let selected_branch = branches
            .get(branch_cursor)
            .map(|branch| branch.name.clone())
            .unwrap_or_default();

        let mut branches = branches;
        for (idx, b) in branches.iter_mut().enumerate() {
            b.selected = idx == branch_cursor && !selected_branch.is_empty();
        }

        let mut app = Self {
            current_step: WizardStep::Welcome,

            binaries: Vec::new(),
            binary_cursor: 0,

            branches,
            branch_cursor,
            selected_branch,

            variant_options: Vec::new(),
            variant_cursor: 0,
            selected_variant: String::new(),
            discovery_in_progress: true,
            discovery_request_id: 0,

            logs: Vec::new(),
            log_scroll: 0,
            auto_scroll_logs: true,

            workspace_root,
            source_binary_dir: source_binary_dir.clone(),
            custom_path_input: source_binary_dir.to_string_lossy().to_string(),
            is_editing_path: false,

            show_help: false,
            show_confirm_dialog: false,
            show_sudo_modal: false,
            show_branch_switching_modal: false,
            branch_switch_status: BranchSwitchStatus::Idle,
            branch_switch_spinner_tick: 0,

            sudo_password: String::new(),
            sudo_error: None,
            sudo_attempts: 0,

            install_state: InstallState::Idle,
            progress_percent: 0,
            current_step_desc: "Ready to install BabyDra packages.".to_string(),

            tx,
            rx,
            should_quit: false,
        };

        app.add_log(
            LogLevel::Info,
            "BabyDra Step-by-Step TUI Installer initialized.",
        );
        app.add_log(
            LogLevel::Info,
            format!("Detected source binary path: {:?}", app.source_binary_dir),
        );
        if app.branches.is_empty() {
            app.add_log(
                LogLevel::Warn,
                "No git branches detected — pre-built mode only.",
            );
        } else {
            app.add_log(
                LogLevel::Info,
                format!("Detected {} git branch(es).", app.branches.len()),
            );
        }
        app.request_discovery();
        app.start_branch_refresh();
        app
    }

    /// True when the user picked a branch to install from — the install will
    /// checkout + pull + `cargo build --release` before copying binaries.
    pub fn is_build_from_source(&self) -> bool {
        !self.selected_branch.is_empty()
    }

    pub fn add_log(&mut self, level: LogLevel, msg: impl Into<String>) {
        self.push_log(LogMessage::new(level, msg));
    }

    pub fn push_log(&mut self, log: LogMessage) {
        self.logs.push(log);
        self.trim_logs();
        if self.auto_scroll_logs && self.logs.len() > 10 {
            self.log_scroll = self.logs.len().saturating_sub(10);
        }
    }

    pub(crate) fn trim_logs(&mut self) {
        const MAX_LOG_ENTRIES: usize = 500;
        let removed = self.logs.len().saturating_sub(MAX_LOG_ENTRIES);
        if removed == 0 {
            return;
        }
        self.logs.drain(..removed);
        self.log_scroll = self.log_scroll.saturating_sub(removed);
    }

    /// Returns the active source directory: either `branches/<branch>` when
    /// building from a branch, or `workspace_root` for pre-built binaries.
    pub fn active_source_dir(&self) -> PathBuf {
        if self.is_build_from_source() {
            branch_worktree_dir(&self.workspace_root, &self.selected_branch)
        } else {
            self.workspace_root.clone()
        }
    }

    /// Uses the checked-out branch's target directory when building from
    /// source. A custom path remains available for pre-built mode.
    pub fn active_binary_dir(&self) -> PathBuf {
        if self.is_build_from_source() {
            let branch_target = self.active_source_dir().join("target/release");
            if branch_target.exists() || !self.source_binary_dir.exists() {
                return branch_target;
            }
        }
        self.source_binary_dir.clone()
    }
}
