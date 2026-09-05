use std::collections::HashMap;

use crate::models::{InstallState, LogLevel, WizardStep};
use crate::system::{
    checkout_and_pull, initial_binaries_list, initial_variant_options, update_binaries_status,
};
use crate::tasks::InstallEvent;

use super::state::{App, BranchSwitchStatus};

impl App {
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

    pub fn rescan_binaries(&mut self) {
        let source_root = self.active_source_dir();
        let fresh_binaries = initial_binaries_list(&source_root, &self.source_binary_dir);
        let old_selections: HashMap<String, bool> = self
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
                "Pulling branch '{}' into branches/{}...",
                self.selected_branch, self.selected_branch
            ),
        );

        let workspace_root = self.workspace_root.clone();
        let branch = self.selected_branch.clone();
        let tx = self.tx.clone();

        std::thread::spawn(move || match checkout_and_pull(&workspace_root, &branch) {
            Ok(_branch_dir) => {
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
        let source_root = self.active_source_dir();
        self.variant_options = initial_variant_options(&source_root);
        self.rescan_binaries();
    }
}
