use crate::actions::runner::CommandRunner;
use crate::utils::fs::mkdir_p;
use crate::utils::system::get_babydra_dir;
use std::fs;
use std::path::Path;

pub fn sync_assets(runner: &CommandRunner, repo_root: &Path) -> Result<(), String> {
    runner.step("Syncing wallpapers and brand assets...");
    let babydra_dir = get_babydra_dir();
    mkdir_p(&babydra_dir)?;
    runner.run_sudo(
        "mkdir",
        &["-p", "/usr/share/babydra", "/var/lib/babydra"],
        None,
    )?;
    runner.run_sudo("chmod", &["777", "/var/lib/babydra"], None)?;

    let wallpaper_src = repo_root.join("wallpaper.png");
    if wallpaper_src.exists() {
        let source = path_arg(&wallpaper_src)?;
        fs::copy(&wallpaper_src, babydra_dir.join("wallpaper.png"))
            .map_err(|e| format!("Failed to install local wallpaper: {}", e))?;
        runner.run_sudo(
            "cp",
            &[&source, "/var/lib/babydra/lock_wallpaper.png"],
            None,
        )?;
        runner.run_sudo(
            "chmod",
            &["666", "/var/lib/babydra/lock_wallpaper.png"],
            None,
        )?;
        runner.run_sudo("cp", &[&source, "/usr/share/babydra/wallpaper.png"], None)?;
    }

    let logo_src = repo_root.join("libs/babydra-core/src/services/logo.png");
    if logo_src.exists() {
        let source = path_arg(&logo_src)?;
        fs::copy(&logo_src, babydra_dir.join("logo.png"))
            .map_err(|e| format!("Failed to install local logo: {}", e))?;
        runner.run_sudo(
            "cp",
            &[&source, "/usr/share/babydra/babydra-preview.png"],
            None,
        )?;
        runner.run_sudo(
            "cp",
            &[&source, "/usr/share/babydra/babydra-settings.png"],
            None,
        )?;
        runner.run_sudo("cp", &[&source, "/usr/share/babydra/logo.png"], None)?;
        runner.run_sudo("cp", &[&source, "/var/lib/babydra/logo.png"], None)?;
    }

    let avatar_src = babydra_dir.join("avatar.png");
    if avatar_src.exists() {
        let source = path_arg(&avatar_src)?;
        runner.run_sudo("cp", &[&source, "/var/lib/babydra/avatar.png"], None)?;
        runner.run_sudo("chmod", &["666", "/var/lib/babydra/avatar.png"], None)?;
    }

    Ok(())
}

fn path_arg(path: &Path) -> Result<String, String> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| format!("Path is not valid UTF-8: {}", path.display()))
}
