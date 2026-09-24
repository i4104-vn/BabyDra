pub mod binaries;
pub mod build;
pub mod daemons;
pub mod desktop;

use crate::actions::runner::CommandRunner;
use crate::actions::sync::sync_all_configs;
use crate::config::UpdaterConfig;
use crate::utils::fs::mkdir_p;
use crate::utils::process::kill_processes;
use crate::utils::system::{get_home_dir, get_local_bin_dir};
use std::path::Path;

pub fn execute_update(
    runner: &CommandRunner,
    repo_root: &Path,
    config: &UpdaterConfig,
) -> Result<(), String> {
    runner.step("Starting BabyDra Hot Update & Reload...");

    // 1. Rebuild workspace in release mode
    build::build_release(runner, repo_root)?;

    let local_bin = get_local_bin_dir();
    let log_dir = get_home_dir().join(".cache/babydra");
    mkdir_p(&local_bin)?;
    mkdir_p(&log_dir)?;

    // 2. Stop running processes
    runner.step("Stopping active shell and desktop processes...");
    kill_processes(&config.binaries.kill_processes);
    runner.success("Active processes stopped.");

    // 3. Install new binaries
    binaries::install_binaries(runner, repo_root, &config.binaries)?;

    // 4. Register desktop application entries & MIME
    desktop::install_desktop_integrations(runner, repo_root, &config.mime_defaults)?;

    // 5. Sync configurations & themes
    sync_all_configs(runner, repo_root, config)?;

    // 6. Start background shell components
    daemons::start_shell_daemons(runner, &config.binaries.shell_daemons)?;

    runner.success("Hot Update & Reload completed successfully!");
    Ok(())
}
