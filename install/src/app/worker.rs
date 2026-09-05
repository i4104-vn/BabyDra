use crate::models::{InstallState, LogLevel, WizardStep};
use crate::system::sudo::MAX_PASSWORD_ATTEMPTS;
use crate::system::SudoSession;
use crate::tasks::{spawn_installation_worker, InstallEvent, InstallPlan};

use super::state::{App, BranchSwitchStatus};

impl App {
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
                                "Synchronized branch '{}' in branches/{} and loaded components.",
                                self.selected_branch, self.selected_branch
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

    pub fn cancel_sudo(&mut self) {
        self.show_sudo_modal = false;
        self.sudo_password.clear();
        self.sudo_error = None;
        self.install_state = InstallState::Idle;
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

    pub fn launch_worker(&mut self) {
        self.install_state = InstallState::Installing;
        self.progress_percent = 0;
        self.current_step_desc = "Starting installation pipeline...".to_string();
        self.logs.clear();
        self.log_scroll = 0;
        self.add_log(
            LogLevel::Info,
            "Installation worker launched with active configuration.",
        );

        let selected_variant = self
            .variant_options
            .iter()
            .find(|v| v.selected)
            .cloned()
            .unwrap_or_else(|| {
                crate::models::VariantItem {
                    name: "default".into(),
                    theme: "babydra-default".into(),
                    apps: Vec::new(),
                    selected: true,
                }
            });

        let build_from_source = self.is_build_from_source();
        let plan = InstallPlan {
            workspace_root: self.workspace_root.clone(),
            source_root: self.active_source_dir(),
            source_binary_dir: self.source_binary_dir.clone(),
            selected_binaries: self
                .binaries
                .iter()
                .filter(|b| b.selected && (b.exists_in_source || build_from_source))
                .cloned()
                .collect(),
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
}
