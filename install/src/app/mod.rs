pub mod handlers;

use crossterm::event::KeyEvent;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::models::{
    BinaryItem, BranchItem, InstallState, LogLevel, LogMessage, VariantItem, WizardStep,
};
use crate::system::sudo::MAX_PASSWORD_ATTEMPTS;
use crate::system::{
    checkout_and_pull, default_binary_source_dir, find_workspace_root, initial_binaries_list,
    initial_variant_options, list_branches, update_binaries_status, SudoSession,
};
use crate::tasks::{spawn_installation_worker, InstallEvent, InstallPlan};

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
    #[allow(clippy::new_without_default)] // Default is never used — main constructs via App::new()
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let workspace_root = find_workspace_root();
        let source_binary_dir = default_binary_source_dir(&workspace_root);
        let binaries = initial_binaries_list(&workspace_root, &source_binary_dir);
        let branches = list_branches(&workspace_root);

        let mut app = Self {
            current_step: WizardStep::Welcome,

            binaries,
            binary_cursor: 0,

            branches,
            branch_cursor: 0,
            selected_branch: String::new(),

            variant_options: initial_variant_options(&workspace_root),
            variant_cursor: 0,
            selected_variant: "default".to_string(),

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
        if app.binaries.is_empty() {
            app.add_log(
                LogLevel::Warn,
                "No components discovered (no crates/ in the checked-out branch and no pre-built binaries in the source directory).",
            );
        }
        app
    }

    /// True when the user picked a branch to install from — the install will
    /// checkout + pull + `cargo build --release` before copying binaries.
    pub fn is_build_from_source(&self) -> bool {
        !self.selected_branch.is_empty()
    }

    pub fn add_log(&mut self, level: LogLevel, msg: impl Into<String>) {
        self.logs.push(LogMessage::new(level, msg));
        if self.auto_scroll_logs && self.logs.len() > 10 {
            self.log_scroll = self.logs.len().saturating_sub(10);
        }
    }

    pub fn rescan_binaries(&mut self) {
        let fresh_binaries = initial_binaries_list(&self.workspace_root, &self.source_binary_dir);
        let old_selections: std::collections::HashMap<String, bool> = self
            .binaries
            .iter()
            .map(|b| (b.name.clone(), b.selected))
            .collect();

        self.binaries = fresh_binaries
            .into_iter()
            .map(|mut b| {
                if let Some(&sel) = old_selections.get(&b.name) {
                    b.selected = sel;
                }
                b
            })
            .collect();

        update_binaries_status(&mut self.binaries, &self.source_binary_dir);
        let found_count = self.binaries.iter().filter(|b| b.exists_in_source).count();
        self.add_log(
            LogLevel::Info,
            format!(
                "Scanned source directory. Found {}/{} pre-built binaries.",
                found_count,
                self.binaries.len()
            ),
        );
    }

    pub fn start_branch_switch(&mut self) {
        if self.selected_branch.is_empty() {
            self.next_step();
            return;
        }

        self.show_branch_switching_modal = true;
        self.branch_switch_status = BranchSwitchStatus::Switching;
        self.branch_switch_spinner_tick = 0;
        self.add_log(
            LogLevel::Info,
            format!(
                "Switching to branch '{}' in background...",
                self.selected_branch
            ),
        );

        let workspace_root = self.workspace_root.clone();
        let branch = self.selected_branch.clone();
        let tx = self.tx.clone();

        std::thread::spawn(move || match checkout_and_pull(&workspace_root, &branch) {
            Ok(()) => {
                let _ = tx.send(InstallEvent::BranchSwitched {
                    success: true,
                    error_msg: None,
                });
            }
            Err(e) => {
                let _ = tx.send(InstallEvent::BranchSwitched {
                    success: false,
                    error_msg: Some(e.to_string()),
                });
            }
        });
    }

    pub fn rescan_after_branch_switch(&mut self) {
        self.variant_options = initial_variant_options(&self.workspace_root);
        self.rescan_binaries();
    }

    pub fn on_tick(&mut self) {
        if self.show_branch_switching_modal {
            self.branch_switch_spinner_tick = (self.branch_switch_spinner_tick + 1) % 1000;
        }

        while let Ok(event) = self.rx.try_recv() {
            match event {
                InstallEvent::Progress {
                    current,
                    total,
                    current_step_name,
                } => {
                    self.current_step_desc = current_step_name;
                    if total > 0 {
                        self.progress_percent =
                            ((current as f64 / total as f64) * 100.0).clamp(0.0, 100.0) as u16;
                    }
                }
                InstallEvent::Log(log_msg) => {
                    self.logs.push(log_msg);
                    if self.auto_scroll_logs && self.logs.len() > 14 {
                        self.log_scroll = self.logs.len().saturating_sub(14);
                    }
                }
                InstallEvent::BranchSwitched { success, error_msg } => {
                    if success {
                        self.branch_switch_status = BranchSwitchStatus::Done(Ok(()));
                        self.rescan_after_branch_switch();
                        self.add_log(
                            LogLevel::Success,
                            format!(
                                "Switched to branch '{}' and loaded variants & components.",
                                self.selected_branch
                            ),
                        );
                        self.show_branch_switching_modal = false;
                        self.next_step();
                    } else {
                        let msg = error_msg.unwrap_or_else(|| "Unknown git error".into());
                        self.branch_switch_status = BranchSwitchStatus::Done(Err(msg.clone()));
                        self.add_log(LogLevel::Error, format!("Branch switch failed: {}", msg));
                    }
                }
                InstallEvent::SudoFailed(msg) => {
                    // Wrong password: go back to Idle and re-open the modal.
                    self.install_state = InstallState::Idle;
                    self.sudo_attempts += 1;
                    self.sudo_password.clear();
                    if self.sudo_attempts >= MAX_PASSWORD_ATTEMPTS {
                        self.sudo_error = Some(format!(
                            "Too many failed attempts ({MAX_PASSWORD_ATTEMPTS}). Restart the installer."
                        ));
                    } else {
                        self.sudo_error = Some(msg);
                    }
                    self.show_sudo_modal = true;
                    self.current_step = WizardStep::ExecuteInstall;
                }
                InstallEvent::Completed {
                    success,
                    total_copied,
                    total_errors,
                    duration_secs,
                } => {
                    self.progress_percent = 100;
                    self.current_step_desc = if success {
                        format!(
                            "Installation completed successfully in {:.2}s!",
                            duration_secs
                        )
                    } else {
                        format!(
                            "Completed in {:.2}s with {} warnings/errors.",
                            duration_secs, total_errors
                        )
                    };
                    self.install_state = InstallState::Completed {
                        success,
                        total_copied,
                        total_errors,
                    };
                    self.current_step = WizardStep::Summary;
                }
            }
        }
    }

    pub fn set_step(&mut self, step: WizardStep) {
        self.current_step = step;
        if self.current_step == WizardStep::ExecuteInstall && self.install_state == InstallState::Idle {
            self.show_confirm_dialog = true;
        }
    }

    pub fn next_step(&mut self) {
        if let Some(next) = self.current_step.next() {
            self.set_step(next);
        }
    }

    pub fn prev_step(&mut self) {
        if let Some(prev) = self.current_step.prev() {
            self.set_step(prev);
        }
    }

    /// User confirmed the plan: ask for the sudo password first (when not
    /// root), then start the actual installation.
    pub fn begin_install(&mut self) {
        if self.install_state == InstallState::Installing {
            return;
        }
        if !SudoSession::is_root() {
            self.show_sudo_modal = true;
            self.sudo_password.clear();
            self.sudo_error = None;
            self.add_log(
                LogLevel::Config,
                "Sudo password required before starting installation.",
            );
            return;
        }
        self.launch_worker();
    }

    /// Enter pressed in the sudo modal — validate and launch the worker.
    pub fn submit_sudo(&mut self) {
        if self.sudo_attempts >= MAX_PASSWORD_ATTEMPTS {
            self.sudo_error = Some(format!(
                "Too many failed attempts ({MAX_PASSWORD_ATTEMPTS}). Restart the installer."
            ));
            return;
        }
        if self.sudo_password.is_empty() {
            self.sudo_error = Some("Password cannot be empty.".into());
            return;
        }
        self.show_sudo_modal = false;
        self.launch_worker();
    }

    /// Esc in the sudo modal — abort, back to Idle.
    pub fn cancel_sudo(&mut self) {
        self.show_sudo_modal = false;
        self.sudo_password.clear();
        self.sudo_error = None;
        self.install_state = InstallState::Idle;
    }

    fn launch_worker(&mut self) {
        if self.install_state == InstallState::Installing {
            return;
        }

        let build_from_source = self.is_build_from_source();

        let selected_binaries: Vec<BinaryItem> = if build_from_source {
            // A fresh `cargo build --release` produces every binary, so the
            // full canonical list is installed (mirrors scripts/install.sh).
            self.binaries.clone()
        } else {
            self.binaries
                .iter()
                .filter(|b| b.selected && b.exists_in_source)
                .cloned()
                .collect()
        };

        let selected_variant = self
            .variant_options
            .iter()
            .find(|v| v.selected)
            .cloned()
            .unwrap_or_else(|| VariantItem {
                name: "default".to_string(),
                theme: "babydra-default".to_string(),
                apps: Vec::new(),
                selected: true,
            });
        self.selected_variant = selected_variant.name.clone();
        self.add_log(
            LogLevel::Config,
            format!(
                "Selected variant '{}' (theme: {})",
                selected_variant.name, selected_variant.theme
            ),
        );

        if build_from_source {
            self.add_log(
                LogLevel::Config,
                format!(
                    "Install mode: build from branch '{}' (checkout -> pull -> cargo build --release).",
                    self.selected_branch
                ),
            );
        }

        self.install_state = InstallState::Installing;
        self.progress_percent = 0;
        self.current_step = WizardStep::ExecuteInstall;
        self.auto_scroll_logs = true;

        let plan = InstallPlan {
            workspace_root: self.workspace_root.clone(),
            source_binary_dir: if build_from_source {
                default_binary_source_dir(&self.workspace_root)
            } else {
                self.source_binary_dir.clone()
            },
            selected_binaries,
            variant: selected_variant,
            branch: self.selected_branch.clone(),
            sudo_password: if SudoSession::is_root() {
                None
            } else {
                Some(self.sudo_password.clone())
            },
        };

        spawn_installation_worker(plan, self.tx.clone());
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        handlers::handle_key_event(self, key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_execute_install_auto_confirm_modal() {
        let mut app = App::new();

        assert_eq!(app.current_step, WizardStep::Welcome);
        assert!(!app.show_confirm_dialog);

        // Jump or navigate to Step 5
        app.set_step(WizardStep::ExecuteInstall);
        assert_eq!(app.current_step, WizardStep::ExecuteInstall);
        assert!(
            app.show_confirm_dialog,
            "Modal should auto-open on ExecuteInstall step"
        );

        // Cancel modal with 'n'
        app.handle_key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
        assert!(!app.show_confirm_dialog, "Modal should dismiss on 'n'");
        assert_eq!(app.current_step, WizardStep::ExecuteInstall);

        // Re-open modal with 'i'
        app.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
        assert!(app.show_confirm_dialog, "Modal should reopen on 'i'");

        // Go back to previous step with 'b'
        app.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
        assert!(!app.show_confirm_dialog);
        assert_eq!(app.current_step, WizardStep::VariantSelection);

        // Navigate forward from VariantSelection to ExecuteInstall
        app.next_step();
        assert_eq!(app.current_step, WizardStep::ExecuteInstall);
        assert!(
            app.show_confirm_dialog,
            "Modal should auto-open when navigating forward into ExecuteInstall"
        );
    }
}
