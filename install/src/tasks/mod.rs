pub mod binaries;
pub mod build_deps;
pub mod configs;
pub mod desktop;
pub mod display_manager;
pub mod packages;
pub mod permissions;
pub mod services;
pub mod staging;
pub mod themes;

use std::sync::mpsc::Sender;
use std::thread;
use std::time::Instant;

pub use crate::core::event::{InstallEvent, InstallPlan};
use crate::core::pipeline::{build_pipeline, TaskStep};
use crate::core::context::TaskContext;
use crate::discovery::initial_binaries_list;
use crate::models::LogLevel;
use crate::runtime::{build_workspace_streaming, checkout_and_pull, stop_process};

/// Spawns the background installation worker thread.
pub fn spawn_installation_worker(plan: InstallPlan, tx: Sender<InstallEvent>) {
    thread::spawn(move || {
        let start_time = Instant::now();
        let mut ctx = TaskContext::new(plan, tx);

        ctx.send_log(
            LogLevel::Info,
            "Starting BabyDra Installation Worker...",
        );

        // Pre-authenticate sudo credentials before starting
        if let Err(e) = ctx.preauth() {
            ctx.send_log(
                LogLevel::Error,
                format!("Sudo pre-authentication failed: {e}"),
            );
            let _ = ctx.tx.send(InstallEvent::SudoFailed(e));
            return;
        }

        let mut total_copied = 0;
        let mut total_errors = 0;

        // Build pipeline steps
        let mut steps = build_pipeline(
            &ctx.manifest,
            &ctx.selected_binaries,
            ctx.from_branch(),
            &ctx.branch,
        );

        let mut i = 0;
        while i < steps.len() {
            let step = &steps[i];
            ctx.send_progress(i + 1, steps.len(), &step.title);

            let (copied, errors) = dispatch_step(&mut ctx, step);
            total_copied += copied;
            total_errors += errors;

            // If we just checked out and built the branch, refresh the manifest and binaries
            if step.id == "git_checkout" {
                ctx.reload_manifest();
                if ctx.install_all_binaries {
                    let discovered = initial_binaries_list(&ctx.source_root, &ctx.source_binary_dir);
                    if !discovered.is_empty() {
                        ctx.selected_binaries = discovered;
                    }
                }
                // Rebuild pipeline so newly pulled branch manifest changes take effect
                let new_steps = build_pipeline(
                    &ctx.manifest,
                    &ctx.selected_binaries,
                    false, // git is already done
                    &ctx.branch,
                );
                // Retain already executed steps up to current index + append remaining new steps
                let mut rebuilt = steps[..=i].to_vec();
                rebuilt.extend(new_steps.into_iter().filter(|s| s.id != "git_checkout"));
                steps = rebuilt;
            }

            i += 1;
        }

        let duration = start_time.elapsed().as_secs_f64();
        let success = total_errors == 0;

        ctx.send_log(
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

        let _ = ctx.tx.send(InstallEvent::Completed {
            success,
            total_copied,
            total_errors,
            duration_secs: duration,
        });
    });
}

fn dispatch_step(ctx: &mut TaskContext, step: &TaskStep) -> (usize, usize) {
    let tx = ctx.tx.clone();
    let send_log = move |level: LogLevel, msg: String| {
        let _ = tx.send(InstallEvent::Log(crate::models::LogMessage::new(level, msg)));
    };

    let prefix = step.id.split(':').next().unwrap_or(&step.id);

    match prefix {
        "git_checkout" => {
            send_log(
                LogLevel::Info,
                format!("Syncing branch '{}' in branches/{}...", ctx.branch, ctx.branch),
            );
            match checkout_and_pull(&ctx.workspace_root, &ctx.branch) {
                Ok(branch_dir) => {
                    send_log(
                        LogLevel::Success,
                        format!("Branch '{}' ready at {}.", ctx.branch, branch_dir.display()),
                    );
                    (1, 0)
                }
                Err(e) => {
                    send_log(LogLevel::Error, format!("Git worktree pull failed: {e}"));
                    (0, 1)
                }
            }
        }
        "cargo_build" => {
            send_log(
                LogLevel::Info,
                format!(
                    "Building branch '{}' in release mode (cargo build --release in {})...",
                    ctx.branch,
                    ctx.source_root.display()
                ),
            );
            let send_log_cb = send_log.clone();
            let ok = build_workspace_streaming(&ctx.source_root, send_log_cb);
            if ok {
                send_log(LogLevel::Success, "Release build completed successfully.".into());
                (1, 0)
            } else {
                send_log(
                    LogLevel::Error,
                    "Release build failed — binaries may be missing.".into(),
                );
                (0, 1)
            }
        }
        "terminate" => {
            send_log(
                LogLevel::Warn,
                "Terminating active processes before overwrite...".into(),
            );
            for bin in &ctx.selected_binaries {
                stop_process(&bin.name);
            }
            thread::sleep(std::time::Duration::from_millis(250));
            (1, 0)
        }
        "pacman" => packages::install_pacman_packages(&ctx.sudo, &ctx.manifest.pacman_packages, send_log),
        "aur_helper" => packages::ensure_yay_installed(&ctx.sudo, send_log),
        "aur_packages" => packages::install_aur_packages(&ctx.sudo, &ctx.manifest.aur_packages, send_log),
        "build_dep" => {
            let dep_name = step.id.strip_prefix("build_dep:").unwrap_or("");
            if let Some(dep) = ctx.manifest.build_deps.iter().find(|d| d.name == dep_name) {
                build_deps::build_dependency(dep, send_log)
            } else {
                (0, 0)
            }
        }
        "permissions" => {
            if let Some(cfg) = &ctx.manifest.permissions {
                permissions::configure_permissions(&ctx.sudo, cfg, send_log)
            } else {
                (0, 0)
            }
        }
        "binary" => {
            let bin_name = step.id.strip_prefix("binary:").unwrap_or("");
            if let Some(bin) = ctx.selected_binaries.iter().find(|b| b.name == bin_name) {
                binaries::execute_binary_copy_task(bin, &ctx.source_binary_dir, &ctx.sudo, send_log)
            } else {
                (0, 0)
            }
        }
        "staging_binaries" => {
            staging::stage_binaries(
                &ctx.manifest.staging,
                &ctx.source_binary_dir,
                &ctx.selected_binaries,
                &ctx.sudo,
                send_log,
            )
        }
        "staging_permissions" => {
            staging::set_staging_permissions(&ctx.manifest.staging, &ctx.sudo, send_log)
        }
        "themes_deploy" => {
            themes::deploy_theme_packages(
                &ctx.source_root,
                "babydra-default",
                &ctx.manifest,
                &ctx.sudo,
                send_log,
            );
            (1, 0)
        }
        "config" => {
            let config_key = step.id.strip_prefix("config:").unwrap_or("");
            match config_key {
                "labwc" => (configs::sync_labwc_fallback(&ctx.source_root, send_log), 0),
                "dotfiles" => (configs::sync_dotfiles_fallback(&ctx.source_root, send_log), 0),
                source_rel => {
                    if let Some(rule) = ctx.manifest.configs.iter().find(|r| r.source == source_rel) {
                        configs::sync_config_rule(&ctx.source_root, rule, send_log)
                    } else {
                        (0, 0)
                    }
                }
            }
        }
        "themes_icons" => {
            let count = themes::install_themes_icons_cursors(&ctx.source_root, &ctx.manifest, &ctx.sudo, send_log);
            (count, 0)
        }
        "desktop" => {
            let count = desktop::register_desktop_entries(&ctx.source_root, &ctx.manifest, &ctx.sudo, send_log);
            (count, 0)
        }
        "gsettings" => {
            let count = themes::apply_gsettings_fontcache(&ctx.manifest, &ctx.sudo, send_log);
            (count, 0)
        }
        "services" => {
            let count = services::restart_services(&ctx.source_root, &ctx.sudo, send_log);
            (count, 0)
        }
        "greetd_config" => {
            if let Some(greetd) = &ctx.manifest.greetd {
                display_manager::configure_greetd_session(
                    &ctx.source_root,
                    greetd,
                    &ctx.selected_binaries,
                    &ctx.sudo,
                    send_log,
                )
            } else {
                (0, 0)
            }
        }
        "mask_gettys" => {
            if let Some(greetd) = &ctx.manifest.greetd {
                display_manager::mask_secondary_gettys(greetd, &ctx.sudo, send_log)
            } else {
                (0, 0)
            }
        }
        "enable_greetd" => {
            if let Some(greetd) = &ctx.manifest.greetd {
                display_manager::enable_greetd_service(greetd, &ctx.sudo, send_log)
            } else {
                (0, 0)
            }
        }
        _ => (0, 0),
    }
}
