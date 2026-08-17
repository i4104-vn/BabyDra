pub mod binaries;
pub mod configs;
pub mod display_manager;
pub mod packages;
pub mod varlib;

use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Instant;

use crate::models::{BinaryItem, GenericOptionItem, LogLevel, LogMessage, VariantItem};
use crate::system::{build_workspace, checkout_and_pull, stop_process, SudoSession};

pub enum InstallEvent {
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

pub struct InstallPlan {
    pub workspace_root: PathBuf,
    pub source_binary_dir: PathBuf,
    pub selected_binaries: Vec<BinaryItem>,
    pub selected_packages: Vec<GenericOptionItem>,
    pub selected_varlib: Vec<GenericOptionItem>,
    pub selected_configs_themes: Vec<GenericOptionItem>,
    pub selected_display_manager: Vec<GenericOptionItem>,
    /// Variant selected in step 6 (theme + app list + keybinds source).
    pub variant: VariantItem,
    /// Branch to check out + pull before building (empty = skip git step).
    pub branch: String,
    /// Sudo password provided by the user (None when running as root).
    pub sudo_password: Option<String>,
}

pub fn spawn_installation_worker(plan: InstallPlan, tx: Sender<InstallEvent>) {
    thread::spawn(move || {
        let start_time = Instant::now();
        let mut total_copied = 0;
        let mut total_errors = 0;

        // Sudo session created BEFORE any task: pre-auth runs here, safely
        // (password via piped stdin — no TTY prompt, no TUI breakage).
        let sudo = SudoSession::new(plan.sudo_password.clone());

        let send_log = |lvl: LogLevel, msg: String| {
            let _ = tx.send(InstallEvent::Log(LogMessage::new(lvl, msg)));
        };

        // Phase 0: pre-auth sudo (verifies the password exactly once).
        if !SudoSession::is_root() {
            send_log(
                LogLevel::Info,
                "Validating sudo credentials before starting...".into(),
            );
            match sudo.preauth() {
                Ok(()) => {
                    send_log(LogLevel::Success, "Sudo credentials validated.".into());
                }
                Err(e) => {
                    send_log(LogLevel::Error, format!("Sudo validation failed: {e}"));
                    // Tell the TUI to re-prompt for the password. Never fall
                    // through to partial installation.
                    let _ = tx.send(InstallEvent::SudoFailed(e.to_string()));
                    return;
                }
            }
        }

        let total_steps = plan.selected_packages.len()
            + plan.selected_binaries.len()
            + plan.selected_varlib.len()
            + plan.selected_configs_themes.len()
            + plan.selected_display_manager.len()
            + 1 // theme packages
            + 2 * usize::from(!plan.branch.is_empty()); // checkout+pull, then build
        let mut current_step = 0;

        send_log(
            LogLevel::Info,
            "Starting BabyDra Installation Worker...".into(),
        );
        send_log(
            LogLevel::Info,
            format!("Binary Source: {:?}", plan.source_binary_dir),
        );
        send_log(
            LogLevel::Info,
            format!(
                "Variant: {} (theme: {})",
                plan.variant.name, plan.variant.theme
            ),
        );
        if !plan.branch.is_empty() {
            send_log(
                LogLevel::Info,
                format!("Install source branch: {}", plan.branch),
            );
        }

        // Phase 1: checkout branch + pull + build source (branch-based installs).
        if !plan.branch.is_empty() {
            current_step += 1;
            let _ = tx.send(InstallEvent::Progress {
                current: current_step,
                total: total_steps,
                current_step_name: format!("Checkout & pull branch '{}'", plan.branch),
            });
            send_log(
                LogLevel::Info,
                format!(
                    "Checking out branch '{}' and pulling latest code...",
                    plan.branch
                ),
            );
            match checkout_and_pull(&plan.workspace_root, &plan.branch) {
                Ok(()) => send_log(
                    LogLevel::Success,
                    format!("Checked out '{}' and pulled latest.", plan.branch),
                ),
                Err(e) => {
                    send_log(LogLevel::Error, format!("Git checkout/pull failed: {e}"));
                    total_errors += 1;
                }
            }

            current_step += 1;
            let _ = tx.send(InstallEvent::Progress {
                current: current_step,
                total: total_steps,
                current_step_name: "Build workspace (cargo build --release)".to_string(),
            });
            send_log(
                LogLevel::Info,
                "Building workspace in release mode (this can take a while)...".into(),
            );
            let (ok, tail) = build_workspace(&plan.workspace_root);
            for line in tail {
                send_log(LogLevel::Info, line);
            }
            if ok {
                send_log(LogLevel::Success, "Release build completed.".into());
            } else {
                send_log(
                    LogLevel::Error,
                    "Release build failed — binaries may be missing.".into(),
                );
                total_errors += 1;
            }
        }

        // Phase 2: terminate old processes if requested.
        let terminate_enabled = plan
            .selected_configs_themes
            .iter()
            .any(|o| o.id == "terminate_processes" && o.selected);

        if terminate_enabled {
            send_log(
                LogLevel::Warn,
                "Terminating active processes before overwrite...".into(),
            );
            let procs = [
                "babydra-panel",
                "babydra-switcher",
                "babydra-screenshot",
                "babydra-lock",
                "babydra-launcher",
                "babydra-image-preview",
                "babydra-preview",
                "babydra-settings",
                "babydra-explore",
                "babydra-greeter",
                "fnott",
                "xfce4-notifyd",
            ];
            for p in procs {
                stop_process(p);
            }
            thread::sleep(std::time::Duration::from_millis(250));
        }

        // Phase 3: Packages.
        for opt in &plan.selected_packages {
            if !opt.selected {
                continue;
            }
            current_step += 1;
            let _ = tx.send(InstallEvent::Progress {
                current: current_step,
                total: total_steps,
                current_step_name: opt.title.clone(),
            });
            let (c, e) = packages::execute_packages_task(opt, &sudo, &send_log);
            total_copied += c;
            total_errors += e;
        }

        // Phase 4: Prebuilt Binaries.
        for bin in &plan.selected_binaries {
            current_step += 1;
            let _ = tx.send(InstallEvent::Progress {
                current: current_step,
                total: total_steps,
                current_step_name: format!("Installing binary: {}", bin.name),
            });
            let (c, e) =
                binaries::execute_binary_copy_task(bin, &plan.source_binary_dir, &sudo, &send_log);
            total_copied += c;
            total_errors += e;
        }

        // Phase 5: /var/lib Staging.
        for opt in &plan.selected_varlib {
            if !opt.selected {
                continue;
            }
            current_step += 1;
            let _ = tx.send(InstallEvent::Progress {
                current: current_step,
                total: total_steps,
                current_step_name: opt.title.clone(),
            });
            let (c, e) = varlib::execute_varlib_task(
                opt,
                &plan.workspace_root,
                &plan.source_binary_dir,
                &sudo,
                &send_log,
            );
            total_copied += c;
            total_errors += e;
        }

        // Phase 6: Theme packages — deploy themes/ to ~/.babydra/themes & /usr/share/babydra/themes + persist variant theme selection
        {
            current_step += 1;
            let _ = tx.send(InstallEvent::Progress {
                current: current_step,
                total: total_steps,
                current_step_name: "Deploy theme packages".to_string(),
            });
            configs::deploy_theme_packages(&plan.workspace_root, &plan.variant.theme, &sudo, &send_log);
            total_copied += 1;
        }

        // Phase 7: Configs & Desktop Environment Setup (including restarting panel service).
        for opt in &plan.selected_configs_themes {
            if !opt.selected || opt.id == "terminate_processes" {
                continue;
            }
            current_step += 1;
            let _ = tx.send(InstallEvent::Progress {
                current: current_step,
                total: total_steps,
                current_step_name: opt.title.clone(),
            });
            let (c, e) = configs::execute_configs_task(opt, &plan.workspace_root, &sudo, &send_log);
            total_copied += c;
            total_errors += e;
        }

        // Phase 8: Display Manager.
        for opt in &plan.selected_display_manager {
            if !opt.selected {
                continue;
            }
            current_step += 1;
            let _ = tx.send(InstallEvent::Progress {
                current: current_step,
                total: total_steps,
                current_step_name: opt.title.clone(),
            });
            let (c, e) = display_manager::execute_display_manager_task(opt, &sudo, &send_log);
            total_copied += c;
            total_errors += e;
        }

        let duration = start_time.elapsed().as_secs_f64();
        let success = total_errors == 0;

        send_log(
            if success {
                LogLevel::Success
            } else {
                LogLevel::Warn
            },
            format!(
                "Installation finished in {:.2}s. Tasks: {}, Errors: {}",
                duration, total_copied, total_errors
            ),
        );

        let _ = tx.send(InstallEvent::Completed {
            success,
            total_copied,
            total_errors,
            duration_secs: duration,
        });
    });
}
