pub mod handlers;

use crossterm::event::KeyEvent;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::models::{
    BinaryItem, GenericOptionItem, InstallState, LogLevel, LogMessage, PresetProfile, VariantItem,
    WizardStep,
};
use crate::system::{
    default_binary_source_dir, find_workspace_root, initial_binaries_list,
    initial_configs_themes_options, initial_display_manager_options, initial_package_options,
    initial_variant_options, initial_varlib_options, update_binaries_status,
};
use crate::tasks::{spawn_installation_worker, InstallEvent, InstallPlan};

pub struct App {
    pub current_step: WizardStep,
    pub current_profile: PresetProfile,

    // Step Data & Cursors
    pub package_options: Vec<GenericOptionItem>,
    pub package_cursor: usize,

    pub binaries: Vec<BinaryItem>,
    pub binary_cursor: usize,

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

        let mut app = Self {
            current_step: WizardStep::Welcome,
            current_profile: PresetProfile::FullDesktop,

            package_options: initial_package_options(),
            package_cursor: 0,

            binaries,
            binary_cursor: 0,

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
        app
    }

    pub fn add_log(&mut self, level: LogLevel, msg: impl Into<String>) {
        self.logs.push(LogMessage::new(level, msg));
        if self.auto_scroll_logs && self.logs.len() > 10 {
            self.log_scroll = self.logs.len().saturating_sub(10);
        }
    }

    pub fn rescan_binaries(&mut self) {
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

    pub fn apply_profile(&mut self, profile: PresetProfile) {
        self.current_profile = profile;
        match profile {
            PresetProfile::FullDesktop => {
                for b in &mut self.binaries {
                    b.selected = b.exists_in_source;
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
                    b.selected = b.exists_in_source;
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
                self.add_log(LogLevel::Config, "Switched to 'Custom' profile.");
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

    pub fn start_installation(&mut self) {
        if self.install_state == InstallState::Installing {
            return;
        }

        let selected_binaries: Vec<BinaryItem> = self
            .binaries
            .iter()
            .filter(|b| b.selected && b.exists_in_source)
            .cloned()
            .collect();

        let selected_variant = self
            .variant_options
            .iter()
            .find(|v| v.selected)
            .map(|v| v.clone())
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

        self.install_state = InstallState::Installing;
        self.progress_percent = 0;
        self.current_step = WizardStep::ExecuteInstall;
        self.auto_scroll_logs = true;

        let plan = InstallPlan {
            workspace_root: self.workspace_root.clone(),
            source_binary_dir: self.source_binary_dir.clone(),
            selected_binaries,
            selected_packages: self.package_options.clone(),
            selected_varlib: self.varlib_options.clone(),
            selected_configs_themes: self.configs_themes_options.clone(),
            selected_display_manager: self.display_manager_options.clone(),
            variant: selected_variant,
        };

        spawn_installation_worker(plan, self.tx.clone());
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        handlers::handle_key_event(self, key);
    }
}
