pub mod binaries;
pub mod configs;
pub mod display_manager;
pub mod packages;
pub mod varlib;

use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Instant;

use crate::models::{BinaryItem, GenericOptionItem, InstallChannel, LogLevel, LogMessage};
use crate::system::stop_process;

pub enum InstallEvent {
    Progress {
        current: usize,
        total: usize,
        current_step_name: String,
    },
    Log(LogMessage),
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
    pub install_channel: InstallChannel,
    pub selected_binaries: Vec<BinaryItem>,
    pub selected_packages: Vec<GenericOptionItem>,
    pub selected_varlib: Vec<GenericOptionItem>,
    pub selected_configs_themes: Vec<GenericOptionItem>,
    pub selected_display_manager: Vec<GenericOptionItem>,
}

pub fn spawn_installation_worker(plan: InstallPlan, tx: Sender<InstallEvent>) {
    thread::spawn(move || {
        let start_time = Instant::now();
        let mut total_copied = 0;
        let mut total_errors = 0;

        let total_steps = plan.selected_packages.len()
            + plan.selected_binaries.len()
            + plan.selected_varlib.len()
            + plan.selected_configs_themes.len()
            + plan.selected_display_manager.len()
            + 2;
        let mut current_step = 0;

        let send_log = |lvl: LogLevel, msg: String| {
            let _ = tx.send(InstallEvent::Log(LogMessage::new(lvl, msg)));
        };

        send_log(LogLevel::Info, "Starting BabyDra Installation Worker...".into());
        send_log(LogLevel::Info, format!("Target Channel: {}", plan.install_channel.name()));
        send_log(LogLevel::Info, format!("Source Directory: {:?}", plan.source_binary_dir));

        // 0. Pull & Build code if needed for selected channel
        match plan.install_channel {
            InstallChannel::Release | InstallChannel::Develop => {
                let target_branch = match plan.install_channel {
                    InstallChannel::Release => "release",
                    InstallChannel::Develop => "develop",
                    _ => "release",
                };

                let missing_any = plan.selected_binaries.iter().any(|b| !plan.source_binary_dir.join(&b.name).exists());
                if missing_any {
                    current_step += 1;
                    let _ = tx.send(InstallEvent::Progress {
                        current: current_step,
                        total: total_steps,
                        current_step_name: format!("Syncing source branch {}", target_branch),
                    });

                    send_log(LogLevel::Info, format!("Fetching source from origin/{}...", target_branch));
                    let fetch_st = Command::new("git")
                        .args(["fetch", "origin", target_branch])
                        .current_dir(&plan.workspace_root)
                        .status();

                    if let Ok(st) = fetch_st {
                        if st.success() {
                            send_log(LogLevel::Success, format!("Successfully fetched branch {}.", target_branch));
                        }
                    }

                    send_log(LogLevel::Info, "Compiling binary components (cargo build --release)...".into());
                    let build_st = Command::new("cargo")
                        .args(["build", "--release", "--workspace"])
                        .current_dir(&plan.workspace_root)
                        .status();

                    if let Ok(st) = build_st {
                        if st.success() {
                            send_log(LogLevel::Success, "Compiled workspace binary packages successfully.".into());
                        } else {
                            send_log(LogLevel::Warn, "Compilation finished with warnings.".into());
                        }
                    }
                }
            }
            InstallChannel::LocalSource => {
                send_log(LogLevel::Info, "Using pre-compiled local binaries from filesystem directory.".into());
            }
        }

        // 1. Terminate old processes if requested
        let terminate_enabled = plan
            .selected_configs_themes
            .iter()
            .any(|o| o.id == "terminate_processes" && o.selected);

        if terminate_enabled {
            send_log(LogLevel::Warn, "Terminating active processes before overwrite...".into());
            let procs = [
                "babydra-panel", "babydra-switcher", "babydra-screenshot",
                "babydra-lock", "babydra-launcher", "babydra-preview",
                "babydra-settings", "babydra-explore", "babydra-greeter",
                "fnott", "xfce4-notifyd",
            ];
            for p in procs {
                stop_process(p);
            }
            thread::sleep(std::time::Duration::from_millis(250));
        }

        // 2. Packages
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
            let (c, e) = packages::execute_packages_task(opt, &send_log);
            total_copied += c;
            total_errors += e;
        }

        // 3. Prebuilt Binaries
        for bin in &plan.selected_binaries {
            current_step += 1;
            let _ = tx.send(InstallEvent::Progress {
                current: current_step,
                total: total_steps,
                current_step_name: format!("Installing binary: {}", bin.name),
            });
            let (c, e) = binaries::execute_binary_copy_task(bin, &plan.source_binary_dir, &send_log);
            total_copied += c;
            total_errors += e;
        }

        // 4. /var/lib Staging
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
            let (c, e) = varlib::execute_varlib_task(opt, &plan.workspace_root, &plan.source_binary_dir, &send_log);
            total_copied += c;
            total_errors += e;
        }

        // 5. Configs & Themes
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
            let (c, e) = configs::execute_configs_task(opt, &plan.workspace_root, &send_log);
            total_copied += c;
            total_errors += e;
        }

        // 6. Display Manager
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
            let (c, e) = display_manager::execute_display_manager_task(opt, &send_log);
            total_copied += c;
            total_errors += e;
        }

        let duration = start_time.elapsed().as_secs_f64();
        let success = total_errors == 0;

        send_log(
            if success { LogLevel::Success } else { LogLevel::Warn },
            format!("Installation finished in {:.2}s. Tasks: {}, Errors: {}", duration, total_copied, total_errors),
        );

        let _ = tx.send(InstallEvent::Completed {
            success,
            total_copied,
            total_errors,
            duration_secs: duration,
        });
    });
}
