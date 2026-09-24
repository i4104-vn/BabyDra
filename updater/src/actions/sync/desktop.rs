use crate::actions::runner::CommandRunner;
use crate::utils::fs::{copy_dir_all, mkdir_p, set_executable};
use crate::utils::system::get_home_dir;
use std::fs;
use std::path::Path;

pub fn sync_desktop_configs(runner: &CommandRunner, repo_root: &Path) -> Result<(), String> {
    let home = get_home_dir();

    runner.step("Syncing labwc compositor configurations...");
    let labwc_dest = home.join(".config/labwc");
    let labwc_src = repo_root.join("configs/labwc");
    mkdir_p(&labwc_dest)?;

    if labwc_src.exists() {
        for entry in fs::read_dir(&labwc_src)
            .map_err(|e| format!("Failed to read {}: {}", labwc_src.display(), e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read labwc config: {}", e))?;
            let path = entry.path();
            let file_name = entry.file_name();
            let dest_file = labwc_dest.join(&file_name);

            if file_name == "autostart" && dest_file.exists() {
                continue;
            }

            if path.is_dir() {
                copy_dir_all(&path, &dest_file)
                    .map_err(|e| format!("Failed to copy {}: {}", path.display(), e))?;
            } else {
                fs::copy(&path, &dest_file)
                    .map_err(|e| format!("Failed to copy {}: {}", path.display(), e))?;
            }
        }
    }

    let autostart_file = labwc_dest.join("autostart");
    set_executable(&autostart_file)
        .map_err(|e| format!("Failed to make autostart executable: {}", e))?;

    let scripts_dir = labwc_dest.join("scripts");
    if scripts_dir.exists() {
        for entry in fs::read_dir(&scripts_dir)
            .map_err(|e| format!("Failed to read {}: {}", scripts_dir.display(), e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read labwc scripts: {}", e))?;
            set_executable(&entry.path()).map_err(|e| {
                format!(
                    "Failed to make {} executable: {}",
                    entry.path().display(),
                    e
                )
            })?;
        }
    }

    runner.step("Syncing GTK, Kitty, and Fastfetch configs...");
    let gtk3_dir = home.join(".config/gtk-3.0");
    let gtk4_dir = home.join(".config/gtk-4.0");
    let font_dir = home.join(".config/fontconfig");
    mkdir_p(&gtk3_dir)?;
    mkdir_p(&gtk4_dir)?;
    mkdir_p(&font_dir)?;

    let settings_ini = labwc_src.join("settings.ini");
    if settings_ini.exists() {
        fs::copy(&settings_ini, gtk3_dir.join("settings.ini"))
            .map_err(|e| format!("Failed to install GTK 3 settings: {}", e))?;
        fs::copy(&settings_ini, gtk4_dir.join("settings.ini"))
            .map_err(|e| format!("Failed to install GTK 4 settings: {}", e))?;
    }
    let fonts_conf = labwc_src.join("fonts.conf");
    if fonts_conf.exists() {
        fs::copy(&fonts_conf, font_dir.join("fonts.conf"))
            .map_err(|e| format!("Failed to install fontconfig settings: {}", e))?;
    }

    let kitty_src = repo_root.join("configs/kitty");
    if kitty_src.exists() {
        copy_dir_all(&kitty_src, &home.join(".config/kitty"))
            .map_err(|e| format!("Failed to sync Kitty config: {}", e))?;
    }
    let nvim_src = repo_root.join("configs/nvim");
    if nvim_src.exists() {
        copy_dir_all(&nvim_src, &home.join(".config/nvim"))
            .map_err(|e| format!("Failed to sync Neovim config: {}", e))?;
    }
    let fastfetch_src = repo_root.join("configs/fastfetch");
    if fastfetch_src.exists() {
        copy_dir_all(&fastfetch_src, &home.join(".config/fastfetch"))
            .map_err(|e| format!("Failed to sync Fastfetch config: {}", e))?;
    }

    Ok(())
}
