use crate::actions::runner::CommandRunner;
use crate::actions::update::desktop::install_desktop_integrations;
use crate::config::UpdaterConfig;
use crate::utils::fs::{copy_dir_all, mkdir_p, set_executable};
use crate::utils::process::{command_exists, kill_processes};
use crate::utils::system::{get_babydra_dir, get_home_dir};
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
    kill_processes(&config.kill_processes());

    let configs_root = if repo_root.join("assets/configs").is_dir() {
        repo_root.join("assets/configs")
    } else {
        repo_root.join("configs")
    };

    // 2. Ensure wallpaper is present in ~/.babydra
    let babydra_dir = get_babydra_dir();
    mkdir_p(&babydra_dir)?;
    let wallpaper_src = if repo_root.join("assets/wallpaper.png").is_file() {
        repo_root.join("assets/wallpaper.png")
    } else {
        repo_root.join("wallpaper.png")
    };
    let wallpaper_dest = babydra_dir.join("wallpaper.png");
    if wallpaper_src.exists() && !wallpaper_dest.exists() {
        let _ = fs::copy(&wallpaper_src, &wallpaper_dest);
    }

    // 3. Setup labwc autostart & rc.xml
    mkdir_p(&labwc_config)?;

    let autostart_dest = labwc_config.join("autostart");
    let autostart_src = configs_root.join("labwc/autostart");
    if autostart_src.exists() && !autostart_dest.exists() {
        fs::copy(&autostart_src, &autostart_dest)
            .map_err(|e| format!("Failed to install labwc autostart: {}", e))?;
        set_executable(&autostart_dest)
            .map_err(|e| format!("Failed to make labwc autostart executable: {}", e))?;
    }

    let rc_dest = labwc_config.join("rc.xml");
    let rc_src = configs_root.join("labwc/rc.xml");
    if rc_src.exists() {
        fs::copy(&rc_src, &rc_dest)
            .map_err(|e| format!("Failed to install labwc rc.xml: {}", e))?;
    }

    let scripts_dest = labwc_config.join("scripts");
    let scripts_src = configs_root.join("labwc/scripts");
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

    // 4. Ensure desktop entries & MIME associations are configured
    let _ = install_desktop_integrations(runner, repo_root, &config.mime_defaults);

    // 5. Verify ddcutil
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
