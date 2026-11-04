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
use crate::runtime::{
    build_app_streaming, build_workspace_streaming, checkout_and_pull, clone_or_sync_app,
    stop_process,
};

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

            // Abort immediately on critical failures (checkout or release build) to prevent
            // configuring broken system services or deploying missing binaries.
            if errors > 0 && (step.id == "git_checkout" || step.id == "cargo_build") {
                let duration = start_time.elapsed().as_secs_f64();
                ctx.send_log(
                    LogLevel::Error,
                    format!(
                        "Installation aborted due to critical failure in '{}'.",
                        step.title
                    ),
                );
                let _ = ctx.tx.send(InstallEvent::Completed {
                    success: false,
                    total_copied,
                    total_errors,
                    duration_secs: duration,
                });
                return;
            }

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
                format!("Syncing applications for branch '{}'...", ctx.branch),
            );
            if let Err(e) = checkout_and_pull(&ctx.workspace_root, &ctx.branch) {
                send_log(LogLevel::Error, format!("Failed to prepare branch directory: {e}"));
                return (0, 1);
            }

            let git_base = ctx.manifest.git_base.as_deref().unwrap_or("https://github.com/i4104-vn");
            let mut clone_errors = 0;
            let mut clone_success = 0;

            for bin in &ctx.selected_binaries {
                let git_url = ctx
                    .manifest
                    .binaries
                    .iter()
                    .find(|b| b.name == bin.name)
                    .and_then(|b| b.git.clone())
                    .unwrap_or_else(|| format!("{}/{}.git", git_base, bin.name));

                let app_dir = ctx.source_root.join(&bin.name);
                send_log(
                    LogLevel::Info,
                    format!(
                        "Syncing {} from {} (branch: '{}')...",
                        bin.name, git_url, ctx.branch
                    ),
                );

                match clone_or_sync_app(&bin.name, &git_url, &ctx.branch, &app_dir) {
                    Ok(path) => {
                        send_log(
                            LogLevel::Success,
                            format!("{} synced at {}.", bin.name, path.display()),
                        );
                        clone_success += 1;
                    }
                    Err(e) => {
                        send_log(LogLevel::Error, format!("Failed to clone {}: {e}", bin.name));
                        clone_errors += 1;
                    }
                }
            }

            if clone_errors > 0 {
                (clone_success, clone_errors)
            } else {
                (1, 0)
            }
        }
        "cargo_build" => {
            send_log(
                LogLevel::Info,
                format!(
                    "Building selected applications in release mode for branch '{}'...",
                    ctx.branch
                ),
            );
            let _ = std::fs::create_dir_all(&ctx.source_binary_dir);

            let mut build_errors = 0;
            let mut build_success = 0;

            for bin in &ctx.selected_binaries {
                let app_dir = ctx.source_root.join(&bin.name);
                if app_dir.join("Cargo.toml").exists() {
                    let send_log_cb = send_log.clone();
                    let ok = build_app_streaming(&app_dir, send_log_cb);
                    if ok {
                        let built_bin = app_dir.join("target").join("release").join(&bin.source_name);
                        let dest_bin = ctx.source_binary_dir.join(&bin.name);
                        if built_bin.exists() {
                            let _ = std::fs::copy(&built_bin, &dest_bin);
                            send_log(
                                LogLevel::Success,
                                format!("Compiled and staged binary: {}", bin.name),
                            );
                            build_success += 1;
                        } else {
                            send_log(
                                LogLevel::Error,
                                format!("Compiled binary not found at {:?}", built_bin),
                            );
                            build_errors += 1;
                        }
                    } else {
                        send_log(
                            LogLevel::Error,
                            format!("Release build failed for {}", bin.name),
                        );
                        build_errors += 1;
                    }
                } else if ctx.source_root.join("Cargo.toml").exists() {
                    // Fallback to workspace build if source_root is a cargo workspace
                    let send_log_cb = send_log.clone();
                    let ok = build_workspace_streaming(&ctx.source_root, send_log_cb);
                    if ok {
                        build_success += 1;
                    } else {
                        build_errors += 1;
                    }
                    break;
                } else {
                    let existing_bin = ctx.source_binary_dir.join(&bin.name);
                    if existing_bin.exists() {
                        send_log(
                            LogLevel::Info,
                            format!("Using existing pre-built binary for {}", bin.name),
                        );
                        build_success += 1;
                    } else {
                        send_log(
                            LogLevel::Error,
                            format!("No source Cargo.toml or pre-built binary found for {}", bin.name),
                        );
                        build_errors += 1;
                    }
                }
            }

            if build_errors > 0 {
                (build_success, build_errors)
            } else {
                (1, 0)
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
            let count = services::restart_services(&ctx.source_root, &ctx.manifest, &ctx.sudo, send_log);
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
