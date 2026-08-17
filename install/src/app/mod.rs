pub mod handlers;

use crossterm::event::KeyEvent;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::models::{
    BinaryItem, BranchItem, GenericOptionItem, InstallState, LogLevel, LogMessage, PresetProfile,
    VariantItem, WizardStep,
};
use crate::system::sudo::MAX_PASSWORD_ATTEMPTS;
use crate::system::{
    default_binary_source_dir, fetch_branch_metadata, find_workspace_root, initial_binaries_list,
    initial_configs_themes_options, initial_display_manager_options, initial_package_options,
    initial_variant_options, initial_varlib_options, list_branches, update_binaries_status,
    SudoSession,
};
use crate::tasks::{spawn_installation_worker, InstallEvent, InstallPlan};

pub struct App {
    pub current_step: WizardStep,
    pub current_profile: PresetProfile,
    pub install_channel: InstallChannel,
    pub channel_metadata: Vec<BranchMetadata>,

    // Step Data & Cursors
    pub package_options: Vec<GenericOptionItem>,
    pub package_cursor: usize,

    pub binaries: Vec<BinaryItem>,
    pub binary_cursor: usize,

    pub branches: Vec<BranchItem>,
    pub branch_cursor: usize,
    /// Empty string = pre-built only; otherwise the branch to check out,
    /// pull and rebuild from source.
    pub selected_branch: String,

    pub varlib_options: Vec<GenericOptionItem>,
    pub varlib_cursor: usize,

    pub configs_themes_options: Vec<GenericOptionItem>,
    pub configs_themes_cursor: usize,

    pub display_manager_options: Vec<GenericOptionItem>,
    pub display_manager_cursor: usize,

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
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let workspace_root = find_workspace_root();
        let source_binary_dir = default_binary_source_dir(&workspace_root);
        let binaries = initial_binaries_list(&source_binary_dir);
        let branches = list_branches(&workspace_root);

        let channel_metadata = vec![
            fetch_branch_metadata(&workspace_root, InstallChannel::Release),
            fetch_branch_metadata(&workspace_root, InstallChannel::Develop),
            fetch_branch_metadata(&workspace_root, InstallChannel::LocalSource),
        ];

        let mut app = Self {
            current_step: WizardStep::Welcome,
            current_profile: PresetProfile::FullDesktop,
            install_channel: InstallChannel::Release,
            channel_metadata,

            package_options: initial_package_options(),
            package_cursor: 0,

            binaries,
            binary_cursor: 0,

            branches,
            branch_cursor: 0,
            selected_branch: String::new(),

            varlib_options: initial_varlib_options(),
            varlib_cursor: 0,

            configs_themes_options: initial_configs_themes_options(),
            configs_themes_cursor: 0,

            display_manager_options: initial_display_manager_options(),
            display_manager_cursor: 0,

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

            sudo_password: String::new(),
            sudo_error: None,
            sudo_attempts: 0,

            install_state: InstallState::Idle,
            progress_percent: 0,
            current_step_desc: "Ready to configure and install BabyDra desktop.".to_string(),

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
            format!("Channel: {} | Source: {:?}", app.install_channel.name(), app.source_binary_dir),
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

    pub fn toggle_channel(&mut self) {
        self.install_channel = match self.install_channel {
            InstallChannel::Release => InstallChannel::Develop,
            InstallChannel::Develop => InstallChannel::LocalSource,
            InstallChannel::LocalSource => InstallChannel::Release,
        };
        self.add_log(LogLevel::Config, format!("Switched install channel to: {}", self.install_channel.name()));
    }

    pub fn get_current_channel_meta(&self) -> Option<&BranchMetadata> {
        self.channel_metadata.iter().find(|m| m.channel == self.install_channel)
    }

    pub fn rescan_binaries(&mut self) {
        update_binaries_status(&mut self.binaries, &self.source_binary_dir);
        let found_count = self.binaries.iter().filter(|b| b.exists_in_source).count();
        self.add_log(
            LogLevel::Info,
            format!(
                "Scanned source binary folder. Found {}/{} executables.",
                found_count,
                self.binaries.len()
            ),
        );
    }

    pub fn apply_profile(&mut self, profile: PresetProfile) {
        self.current_profile = profile;
        let build_from_source = self.is_build_from_source();
        match profile {
            PresetProfile::FullDesktop => {
                for b in &mut self.binaries {
                    b.selected = b.exists_in_source || build_from_source;
                }
                for opt in &mut self.varlib_options {
                    opt.selected = true;
                }
                for opt in &mut self.configs_themes_options {
                    opt.selected = true;
                }
                for opt in &mut self.display_manager_options {
                    opt.selected = true;
                }
                self.add_log(LogLevel::Config, "Applied 'Full Desktop' preset profile.");
            }
            PresetProfile::BinariesAndBundle => {
                for b in &mut self.binaries {
                    b.selected = b.exists_in_source || build_from_source;
                }
                for opt in &mut self.varlib_options {
                    opt.selected = true;
                }
                for opt in &mut self.configs_themes_options {
                    opt.selected = opt.id == "terminate_processes" || opt.id == "restart_services";
                }
                for opt in &mut self.display_manager_options {
                    opt.selected = false;
                }
                self.add_log(
                    LogLevel::Config,
                    "Applied 'Binaries & /var/lib Only' preset profile.",
                );
            }
            PresetProfile::Custom => {
                self.add_log(LogLevel::Config, "Switched to custom configuration profile.");
            }
        }
    }

    pub fn on_tick(&mut self) {
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

    pub fn next_step(&mut self) {
        if let Some(next) = self.current_step.next() {
            self.current_step = next;
        }
    }

    pub fn prev_step(&mut self) {
        if let Some(prev) = self.current_step.prev() {
            self.current_step = prev;
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
            selected_packages: self.package_options.clone(),
            selected_varlib: self.varlib_options.clone(),
            selected_configs_themes: self.configs_themes_options.clone(),
            selected_display_manager: self.display_manager_options.clone(),
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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
