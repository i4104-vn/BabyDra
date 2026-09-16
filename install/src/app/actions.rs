use std::collections::HashMap;

use crate::models::{BinaryItem, BranchItem, InstallState, LogLevel, VariantItem, WizardStep};
use crate::system::{
    checkout_and_pull, initial_binaries_list, initial_variant_options, refresh_branches,
};
use crate::tasks::InstallEvent;

use super::state::{App, BranchSwitchStatus};

impl App {
    pub fn set_step(&mut self, step: WizardStep) {
        self.current_step = step;
        if self.current_step == WizardStep::ExecuteInstall
            && self.install_state == InstallState::Idle
        {
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

    /// Starts source discovery away from the render/input loop. A request id
    /// prevents an older scan from overwriting a newer path or branch choice.
    pub fn request_discovery(&mut self) {
        self.discovery_request_id = self.discovery_request_id.wrapping_add(1);
        let request_id = self.discovery_request_id;
        let source_root = self.active_source_dir();
        let source_binary_dir = self.active_binary_dir();
        self.discovery_in_progress = true;

        let tx = self.tx.clone();
        std::thread::spawn(move || {
            let binaries = initial_binaries_list(&source_root, &source_binary_dir);
            let variants = initial_variant_options(&source_root);
            let _ = tx.send(InstallEvent::DiscoveryUpdated {
                request_id,
                source_root,
                source_binary_dir,
                binaries,
                variants,
            });
        });
    }

    pub fn apply_discovery(
        &mut self,
        request_id: u64,
        source_root: std::path::PathBuf,
        source_binary_dir: std::path::PathBuf,
        mut binaries: Vec<BinaryItem>,
        mut variants: Vec<VariantItem>,
    ) {
        if request_id != self.discovery_request_id
            || source_root != self.active_source_dir()
            || source_binary_dir != self.active_binary_dir()
        {
            return;
        }

        let old_selections: HashMap<String, bool> = self
            .binaries
            .iter()
            .map(|b| (b.name.clone(), b.selected))
            .collect();

        for binary in &mut binaries {
            if let Some(&selected) = old_selections.get(&binary.name) {
                binary.selected = selected;
            }
        }
        self.binaries = binaries;
        self.binary_cursor = self
            .binary_cursor
            .min(self.binaries.len().saturating_sub(1));

        let old_variant = self.selected_variant.clone();
        let selected_variant = variants
            .iter()
            .position(|variant| variant.name == old_variant && !old_variant.is_empty())
            .or_else(|| variants.iter().position(|variant| variant.selected))
            .or_else(|| (!variants.is_empty()).then_some(0));
        for variant in &mut variants {
            variant.selected = false;
        }
        if let Some(index) = selected_variant {
            if let Some(variant) = variants.get_mut(index) {
                variant.selected = true;
                self.selected_variant = variant.name.clone();
            }
            self.variant_cursor = index;
        } else {
            self.selected_variant.clear();
            self.variant_cursor = 0;
        }
        self.variant_options = variants;
        self.discovery_in_progress = false;

        let found_count = self.binaries.iter().filter(|b| b.exists_in_source).count();
        self.add_log(
            LogLevel::Info,
            format!(
                "Scanned source directory. Found {}/{} pre-built binaries.",
                found_count,
                self.binaries.len()
            ),
        );
        if self.binaries.is_empty() {
            self.add_log(
                LogLevel::Warn,
                format!(
                    "No components found in {} or the selected source refs.",
                    source_root.display()
                ),
            );
        }
    }

    pub fn start_branch_refresh(&mut self) {
        let workspace_root = self.workspace_root.clone();
        let tx = self.tx.clone();
        std::thread::spawn(move || {
            let branches = refresh_branches(&workspace_root);
            let _ = tx.send(InstallEvent::BranchesUpdated { branches });
        });
    }

    pub fn apply_branch_list(&mut self, branches: Vec<BranchItem>) {
        let selected_name = self.selected_branch.clone();
        let cursor_name = self
            .branches
            .get(self.branch_cursor)
            .map(|branch| branch.name.clone());
        self.branches = branches;

        let cursor = if selected_name.is_empty() {
            cursor_name
                .as_deref()
                .and_then(|name| self.branches.iter().position(|branch| branch.name == name))
        } else {
            self.branches
                .iter()
                .position(|branch| branch.name == selected_name)
        };
        if let Some(cursor) = cursor {
            self.branch_cursor = cursor;
        } else {
            self.branch_cursor = self
                .branch_cursor
                .min(self.branches.len().saturating_sub(1));
            if !selected_name.is_empty() {
                self.selected_branch.clear();
            }
        }

        for (index, branch) in self.branches.iter_mut().enumerate() {
            branch.selected = !self.selected_branch.is_empty() && index == self.branch_cursor;
        }
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
        self.source_binary_dir = source_root.join("target/release");
        self.custom_path_input = self.source_binary_dir.to_string_lossy().to_string();
        self.request_discovery();
    }
}
