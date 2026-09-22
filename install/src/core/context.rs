use std::path::PathBuf;
use std::sync::mpsc::Sender;

use crate::core::event::{InstallEvent, InstallPlan};
use crate::core::manifest::{load_install_manifest, InstallManifest};
use crate::models::{BinaryItem, LogLevel, LogMessage, VariantItem};
use crate::runtime::SudoSession;

/// Shared execution context passed to all task executors.
pub struct TaskContext {
    pub sudo: SudoSession,
    pub workspace_root: PathBuf,
    pub source_root: PathBuf,
    pub source_binary_dir: PathBuf,
    pub selected_binaries: Vec<BinaryItem>,
    pub install_all_binaries: bool,
    pub variant: VariantItem,
    pub branch: String,
    pub manifest: InstallManifest,
    pub tx: Sender<InstallEvent>,
}

impl TaskContext {
    pub fn new(plan: InstallPlan, tx: Sender<InstallEvent>) -> Self {
        let manifest = load_install_manifest(&plan.source_root, &plan.workspace_root);
        let source_binary_dir = if plan.branch.is_empty() {
            plan.source_binary_dir
        } else {
            plan.source_root.join("target/release")
        };

        Self {
            sudo: SudoSession::new(plan.sudo_password),
            workspace_root: plan.workspace_root,
            source_root: plan.source_root,
            source_binary_dir,
            selected_binaries: plan.selected_binaries,
            install_all_binaries: plan.install_all_binaries,
            variant: plan.variant,
            branch: plan.branch,
            manifest,
            tx,
        }
    }

    pub fn from_branch(&self) -> bool {
        !self.branch.is_empty()
    }

    pub fn send_log(&self, level: LogLevel, msg: impl Into<String>) {
        let _ = self.tx.send(InstallEvent::Log(LogMessage::new(level, msg)));
    }

    pub fn send_progress(&self, current: usize, total: usize, current_step_name: impl Into<String>) {
        let _ = self.tx.send(InstallEvent::Progress {
            current,
            total,
            current_step_name: current_step_name.into(),
        });
    }

    pub fn preauth(&self) -> Result<(), String> {
        self.sudo.preauth().map_err(|e| e.to_string())
    }

    /// Re-loads the manifest after branch checkout.
    pub fn reload_manifest(&mut self) {
        self.manifest = load_install_manifest(&self.source_root, &self.workspace_root);
    }
}
