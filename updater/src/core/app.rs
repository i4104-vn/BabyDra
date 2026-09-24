use crate::actions::runner::LogMessage;
use crate::config::{load_config, UpdaterConfig};
use crate::core::repo::find_repo_root;
use crate::core::state::*;
use crate::utils::logger::FileLogger;
use std::path::PathBuf;

pub struct App {
    pub repo_root: PathBuf,
    pub config: UpdaterConfig,
    pub config_path: Option<PathBuf>,
    pub view_state: ViewState,
    pub menu_items: Vec<ActionItem>,
    pub selected_menu: usize,
    pub logs: Vec<String>,
    pub log_scroll: usize,
    pub autoscroll: bool,
    pub spinner_frame: usize,
    pub active_action_name: Option<String>,
    pub last_exit_code: Option<i32>,
    pub status_message: String,
    pub should_quit: bool,

    // Sudo authentication state
    pub sudo_password_input: String,
    pub sudo_error_message: Option<String>,
    pub pending_sudo_action: Option<ActionId>,

    // Modal selection states
    pub reset_mode_selected: usize,
    pub component_selected: usize,

    // File logging
    pub file_logger: Option<FileLogger>,
}

const MAX_LOG_LINES: usize = 10_000;

impl App {
    pub fn new() -> Self {
        let repo_root = find_repo_root();
        let (config, config_path) = load_config(&repo_root);

        let menu_items = vec![
            ActionItem {
                id: ActionId::UpdateReload,
                title: "Hot Update & Reload",
                icon: "⚡",
                tag: "CORE",
                description: "Build release, install binaries, sync configs & restart running shell daemons.",
                requires_sudo: true,
            },
            ActionItem {
                id: ActionId::SafetyCheck,
                title: "Run Safety Checks",
                icon: "🧪",
                tag: "LINT",
                description: "Run cargo check, cargo clippy (-D warnings), and test suite.",
                requires_sudo: false,
            },
            ActionItem {
                id: ActionId::StartDesktop,
                title: "Start Desktop Shell",
                icon: "🚀",
                tag: "EXEC",
                description: "Prepare environment, sync configs, and start labwc compositor.",
                requires_sudo: false,
            },
            ActionItem {
                id: ActionId::FullInstall,
                title: "Full System Install",
                icon: "📦",
                tag: "SETUP",
                description: "Install pacman & yay packages, configure udev/i2c/greetd, build and deploy all.",
                requires_sudo: true,
            },
            ActionItem {
                id: ActionId::ComponentRestart,
                title: "Restart Component",
                icon: "🔄",
                tag: "DAEMON",
                description: "Select and restart an individual component (Panel, Desktop, Switcher, etc.).",
                requires_sudo: false,
            },
            ActionItem {
                id: ActionId::SyncConfigs,
                title: "Sync Configs & Themes",
                icon: "🎨",
                tag: "SYNC",
                description: "Instantly copy configs (labwc, GTK, kitty, fastfetch, themes) without compiling code.",
                requires_sudo: true,
            },
            ActionItem {
                id: ActionId::CleanWorkspace,
                title: "Clean Workspace",
                icon: "🧹",
                tag: "CLEAN",
                description: "Run cargo clean to free build disk space in target/ directory.",
                requires_sudo: false,
            },
            ActionItem {
                id: ActionId::FactoryReset,
                title: "Factory Reset Arch Linux",
                icon: "⚠️ ",
                tag: "RESET",
                description: "Restore system back to vanilla Arch Linux CLI / TTY login state.",
                requires_sudo: true,
            },
            ActionItem {
                id: ActionId::Quit,
                title: "Exit Updater",
                icon: "🚪",
                tag: "EXIT",
                description: "Quit the BabyDra Updater TUI.",
                requires_sudo: false,
            },
        ];

        Self {
            repo_root,
            config,
            config_path,
            view_state: ViewState::Menu,
            menu_items,
            selected_menu: 0,
            logs: Vec::new(),
            log_scroll: 0,
            autoscroll: true,
            spinner_frame: 0,
            active_action_name: None,
            last_exit_code: None,
            status_message: "Ready. Select an action with ↑/↓ and press Enter.".to_string(),
            should_quit: false,
            sudo_password_input: String::new(),
            sudo_error_message: None,
            pending_sudo_action: None,
            reset_mode_selected: 0,
            component_selected: 0,
            file_logger: None,
        }
    }

    pub fn selected_action(&self) -> &ActionItem {
        &self.menu_items[self.selected_menu]
    }

    pub fn menu_up(&mut self) {
        if self.menu_items.is_empty() {
            return;
        }
        if self.selected_menu > 0 {
            self.selected_menu -= 1;
        } else {
            self.selected_menu = self.menu_items.len() - 1;
        }
    }

    pub fn menu_down(&mut self) {
        if self.menu_items.is_empty() {
            return;
        }
        if self.selected_menu + 1 < self.menu_items.len() {
            self.selected_menu += 1;
        } else {
            self.selected_menu = 0;
        }
    }

    pub fn scroll_logs_up(&mut self, lines: usize) {
        self.autoscroll = false;
        if self.log_scroll >= lines {
            self.log_scroll -= lines;
        } else {
            self.log_scroll = 0;
        }
    }

    pub fn scroll_logs_down(&mut self, lines: usize) {
        if self.logs.is_empty() {
            return;
        }
        self.log_scroll = (self.log_scroll + lines).min(self.logs.len().saturating_sub(1));
    }

    pub fn tick_spinner(&mut self) {
        self.spinner_frame = (self.spinner_frame + 1) % 4;
    }

    pub fn spinner_char(&self) -> char {
        const FRAMES: [char; 4] = ['⠋', '⠙', '⠹', '⠸'];
        FRAMES[self.spinner_frame]
    }

    pub fn append_log(&mut self, line: String) {
        self.logs.push(line);
        let excess = self.logs.len().saturating_sub(MAX_LOG_LINES);
        if excess > 0 {
            self.logs.drain(..excess);
            self.log_scroll = self.log_scroll.saturating_sub(excess);
        }
        if self.autoscroll {
            self.log_scroll = self.logs.len().saturating_sub(1);
        }
    }

    pub fn clear_logs(&mut self) {
        self.logs.clear();
        self.log_scroll = 0;
        self.autoscroll = true;
    }

    pub fn start_file_logging(&mut self, prefix: &str, title: &str) {
        match FileLogger::start(&self.repo_root, prefix, title) {
            Ok(logger) => {
                if let Some(filename) = logger.path().file_name() {
                    self.append_log(format!("📄 Logging output to logs/{}", filename.to_string_lossy()));
                }
                self.file_logger = Some(logger);
            }
            Err(e) => {
                self.append_log(format!("⚠️ Failed to initialize log file: {}", e));
            }
        }
    }

    pub fn handle_log_message(&mut self, message: LogMessage) {
        match message {
            LogMessage::Line(line) => {
                if let Some(logger) = &mut self.file_logger {
                    logger.write_line(&line);
                }
                self.append_log(line);
            }
            LogMessage::Step(msg) => {
                if let Some(logger) = &mut self.file_logger {
                    logger.write_step(&msg);
                }
                self.status_message = msg.clone();
                self.append_log(msg);
            }
            LogMessage::Success(msg) => {
                if let Some(logger) = &mut self.file_logger {
                    logger.write_success(&msg);
                }
                self.status_message = msg.clone();
                self.append_log(msg);
            }
            LogMessage::Error(msg) => {
                if let Some(logger) = &mut self.file_logger {
                    logger.write_error(&msg);
                }
                self.status_message = msg.clone();
                self.append_log(msg);
            }
            LogMessage::Done(code) => {
                if let Some(mut logger) = self.file_logger.take() {
                    let log_path = logger.finish(code);
                    if let Some(name) = log_path.file_name() {
                        self.append_log(format!("✔ Log file saved to logs/{}", name.to_string_lossy()));
                    }
                }
                self.last_exit_code = Some(code);
                self.view_state = ViewState::Finished;
                self.status_message = if code == 0 {
                    "Operation finished successfully.".to_string()
                } else {
                    format!("Operation failed with exit code {}.", code)
                };
            }
        }
    }
}
