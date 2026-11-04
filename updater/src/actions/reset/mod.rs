pub mod cleanup;
pub mod display;
pub mod packages;
pub mod processes;

use crate::actions::runner::CommandRunner;
use crate::core::state::ResetMode;
use crate::utils::system::{get_current_user, get_home_dir, refresh_fc_cache};

pub fn execute_factory_reset(runner: &CommandRunner, mode: ResetMode) -> Result<(), String> {
    runner.step("Starting Arch Linux Factory Reset (Reverting BabyDra)...");
    let is_dry_run = mode == ResetMode::DryRun;

    let user = get_current_user();
    let home = get_home_dir();

    runner.log(format!("Target User: {}", user));
    runner.log(format!("Target Home: {}", home.display()));
    runner.log(format!("Reset Mode: {:?}", mode));

    // 1. Stop active processes
    processes::stop_shell_processes(runner, is_dry_run);

    // 2. Restore TTY console & disable greetd
    display::restore_tty_console(runner, is_dry_run)?;

    // 3. Clean system & user directories
    cleanup::clean_system_and_user_files(runner, is_dry_run)?;

    // 4. Uninstall packages
    packages::clean_packages(runner, mode, is_dry_run)?;

    // 5. Finalize font cache
    runner.step("[Step 6/6] Finalizing system state...");
    if !is_dry_run {
        refresh_fc_cache();
    }

    runner.success(
        "Factory Reset Complete! The system has been restored to clean Arch Linux baseline.",
    );
    Ok(())
}
