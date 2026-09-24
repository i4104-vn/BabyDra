pub mod greetd;
pub mod kernel;
pub mod packages;
pub mod tools;

use crate::actions::runner::CommandRunner;
use crate::actions::sync::sync_all_configs;
use crate::actions::update::{binaries, build, desktop};
use crate::config::UpdaterConfig;
use crate::utils::fs::mkdir_p;
use crate::utils::process::kill_processes;
use crate::utils::system::get_local_bin_dir;
use std::path::Path;

pub fn execute_install(
    runner: &CommandRunner,
    repo_root: &Path,
    config: &UpdaterConfig,
) -> Result<(), String> {
    runner.step("Starting Full BabyDra System Installation...");

    let local_bin = get_local_bin_dir();
    mkdir_p(&local_bin)?;

    // 1. Official Arch packages
    packages::install_pacman_packages(runner, &config.packages)?;

    // 2. Kernel modules & permissions
    kernel::configure_kernel_and_permissions(runner)?;

    // 3. Build & AUR tools
    tools::ensure_yay_installed(runner)?;
    packages::install_yay_packages(runner, &config.packages)?;
    tools::ensure_wtype_installed(runner)?;

    // 4. Compile workspace
    runner.step("Cleaning old build artifacts (cargo clean)...");
    runner.run_cmd("cargo", &["clean"], Some(repo_root))?;
    build::build_release(runner, repo_root)?;

    // 5. Stop old processes & install binaries
    kill_processes(&config.binaries.kill_processes);
    binaries::install_binaries(runner, repo_root, &config.binaries)?;

    // 6. Sync configs & desktop entries
    sync_all_configs(runner, repo_root, config)?;
    desktop::install_desktop_integrations(runner, repo_root, &config.mime_defaults)?;

    // 7. Greetd configuration
    greetd::configure_greetd(runner, &config.greetd)?;

    runner.success("BabyDra Full Installation completed successfully!");
    Ok(())
}
