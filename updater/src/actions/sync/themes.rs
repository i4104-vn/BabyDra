use crate::actions::runner::CommandRunner;
use crate::utils::fs::{copy_dir_all, mkdir_p, path_arg};
use crate::utils::system::{get_babydra_dir, get_home_dir};
use std::fs;
use std::path::Path;

pub fn sync_themes(runner: &CommandRunner, repo_root: &Path) -> Result<(), String> {
    let home = get_home_dir();
    let themes_dest = home.join(".local/share/themes");
    let icons_dest = home.join(".local/share/icons");
    let babydra_themes_dest = get_babydra_dir().join("themes");

    mkdir_p(&themes_dest)?;
    mkdir_p(&icons_dest)?;
    mkdir_p(&babydra_themes_dest)?;

    runner.step("Syncing themes, cursors, and icons...");

    let themes_src = repo_root.join("themes");
    if themes_src.exists() {
        copy_dir_all(&themes_src, &babydra_themes_dest)
            .map_err(|e| format!("Failed to sync BabyDra themes: {}", e))?;
        runner.run_sudo("mkdir", &["-p", "/usr/share/babydra/themes"], None)?;
        let source = path_arg(&themes_src)?;
        runner.run_sudo("cp", &["-r", &source, "/usr/share/babydra/"], None)?;
    }

    let babydra_theme_src = repo_root.join("configs/themes/BabyDra");
    if babydra_theme_src.exists() {
        copy_dir_all(&babydra_theme_src, &themes_dest.join("BabyDra"))
            .map_err(|e| format!("Failed to sync GTK theme: {}", e))?;
    }

    let cursor_src = repo_root.join("configs/themes/cursor");
    if cursor_src.exists() {
        let destination = path_arg(&icons_dest)?;
        for entry in fs::read_dir(&cursor_src)
            .map_err(|e| format!("Failed to read {}: {}", cursor_src.display(), e))?
        {
            let p = entry
                .map_err(|e| format!("Failed to read cursor archive: {}", e))?
                .path();
            if p.extension().is_some_and(|ext| ext == "tar") {
                let archive = path_arg(&p)?;
                runner.run_cmd("tar", &["-xf", &archive, "-C", &destination], None)?;
            }
        }
    }

    let icons_theme_src = repo_root.join("configs/themes/icons");
    if icons_theme_src.exists() {
        let destination = path_arg(&icons_dest)?;
        for entry in fs::read_dir(&icons_theme_src)
            .map_err(|e| format!("Failed to read {}: {}", icons_theme_src.display(), e))?
        {
            let p = entry
                .map_err(|e| format!("Failed to read icon archive: {}", e))?
                .path();
            if p.extension().is_some_and(|ext| ext == "tar") {
                let archive = path_arg(&p)?;
                runner.run_cmd("tar", &["-xf", &archive, "-C", &destination], None)?;
            }
        }
    }

    Ok(())
}
