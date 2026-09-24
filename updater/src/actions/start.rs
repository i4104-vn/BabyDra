use crate::actions::runner::CommandRunner;
use crate::config::UpdaterConfig;
use crate::utils::fs::{copy_dir_all, mkdir_p, set_executable};
use crate::utils::process::{command_exists, kill_processes};
use crate::utils::system::get_home_dir;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn execute_start(
    runner: &CommandRunner,
    repo_root: &Path,
    config: &UpdaterConfig,
) -> Result<(), String> {
    runner.step("Preparing environment to launch labwc with BabyDra...");

    let home = get_home_dir();
    let labwc_config = home.join(".config/labwc");

    // 1. Stop old shell processes
    runner.step("Stopping stale shell processes...");
    kill_processes(&config.binaries.kill_processes);

    // 2. Setup labwc autostart & rc.xml
    mkdir_p(&labwc_config)?;

    let autostart_dest = labwc_config.join("autostart");
    let autostart_src = repo_root.join("configs/labwc/autostart");
    if autostart_src.exists() && !autostart_dest.exists() {
        fs::copy(&autostart_src, &autostart_dest)
            .map_err(|e| format!("Failed to install labwc autostart: {}", e))?;
        set_executable(&autostart_dest)
            .map_err(|e| format!("Failed to make labwc autostart executable: {}", e))?;
    }

    let rc_dest = labwc_config.join("rc.xml");
    let rc_src = repo_root.join("configs/labwc/rc.xml");
    if rc_src.exists() {
        fs::copy(&rc_src, &rc_dest)
            .map_err(|e| format!("Failed to install labwc rc.xml: {}", e))?;
    }

    let scripts_dest = labwc_config.join("scripts");
    let scripts_src = repo_root.join("configs/labwc/scripts");
    if scripts_src.exists() {
        mkdir_p(&scripts_dest)?;
        for entry in fs::read_dir(&scripts_src)
            .map_err(|e| format!("Failed to read labwc scripts: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read labwc script: {}", e))?;
            let source = entry.path();
            let dest = scripts_dest.join(entry.file_name());
            if source.is_dir() {
                copy_dir_all(&source, &dest)
                    .map_err(|e| format!("Failed to copy {}: {}", source.display(), e))?;
            } else {
                fs::copy(&source, &dest)
                    .map_err(|e| format!("Failed to copy {}: {}", source.display(), e))?;
                set_executable(&dest)
                    .map_err(|e| format!("Failed to make {} executable: {}", dest.display(), e))?;
            }
        }
    }

    // 3. Verify ddcutil
    if !command_exists("ddcutil") {
        runner.log("Warning: 'ddcutil' is not installed. Brightness controls for external monitors will not be available.");
    }

    runner.success("Environment prepared. Launching labwc compositor...");
    runner.log("=============================================");
    runner.log("Starting labwc compositor...");
    runner.log("Press Ctrl+Alt+Backspace inside labwc to exit.");
    runner.log("=============================================");

    Command::new("labwc")
        .spawn()
        .map_err(|e| format!("Failed to execute 'labwc': {}", e))?;

    Ok(())
}
