use crate::models::{GenericOptionItem, LogLevel};
use crate::system::{copy_recursive, get_user_home, get_user_local_bin};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

pub fn execute_configs_task<F>(
    opt: &GenericOptionItem,
    workspace_root: &Path,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let home = get_user_home();
    let user_bin_dir = get_user_local_bin();

    match opt.id.as_str() {
        "labwc_configs" => {
            let labwc_src = workspace_root.join("configs/labwc");
            let labwc_dst = home.join(".config/labwc");

            if labwc_src.exists() {
                let _ = copy_recursive(&labwc_src, &labwc_dst);

                let autostart_file = labwc_dst.join("autostart");
                if autostart_file.exists() {
                    let mut perms = fs::metadata(&autostart_file)
                        .map(|m| m.permissions())
                        .unwrap_or_else(|_| fs::Permissions::from_mode(0o755));
                    perms.set_mode(0o755);
                    let _ = fs::set_permissions(&autostart_file, perms);
                }

                let scripts_dir = labwc_dst.join("scripts");
                if scripts_dir.is_dir() {
                    if let Ok(entries) = fs::read_dir(&scripts_dir) {
                        for e in entries.flatten() {
                            if let Ok(mut p) = fs::metadata(e.path()).map(|m| m.permissions()) {
                                p.set_mode(0o755);
                                let _ = fs::set_permissions(e.path(), p);
                            }
                        }
                    }
                }
                log(
                    LogLevel::Success,
                    "Synced labwc autostart, rc.xml, scripts to ~/.config/labwc".into(),
                );
                copied += 1;
            }
        }

        "desktop_entries" => {
            let apps_dir = home.join(".local/share/applications");
            let _ = fs::create_dir_all(&apps_dir);

            let preview_desktop = format!(
                "[Desktop Entry]\nType=Application\nName=BabyDra Preview\nComment=Viewer for images\nExec={}/.local/bin/babydra-preview %f\nIcon=/usr/share/babydra/babydra-preview.png\nTerminal=false\nCategories=Graphics;Viewer;GTK;\nMimeType=image/png;image/jpeg;image/gif;image/webp;image/bmp;\nNoDisplay=false\n",
                home.display()
            );
            let _ = fs::write(apps_dir.join("babydra-preview.desktop"), preview_desktop);

            let settings_desktop = format!(
                "[Desktop Entry]\nType=Application\nName=BabyDra Settings\nComment=Configure system settings\nExec={}/.local/bin/babydra-settings\nIcon=/usr/share/babydra/babydra-settings.png\nTerminal=false\nCategories=Settings;HardwareSettings;GTK;\nNoDisplay=false\n",
                home.display()
            );
            let _ = fs::write(apps_dir.join("babydra-settings.desktop"), settings_desktop);

            let explore_desktop = format!(
                "[Desktop Entry]\nType=Application\nName=BabyDra Explore\nComment=Explore files and folders\nExec={}/.local/bin/babydra-explore %u\nIcon=system-file-manager\nTerminal=false\nCategories=System;FileTools;FileManager;GTK;\nMimeType=inode/directory;\nNoDisplay=false\n",
                home.display()
            );
            let _ = fs::write(apps_dir.join("babydra-explore.desktop"), explore_desktop);

            let _ = Command::new("update-desktop-database")
                .arg(apps_dir.to_str().unwrap())
                .status();
            let _ = Command::new("xdg-mime")
                .args([
                    "default",
                    "babydra-preview.desktop",
                    "image/png",
                    "image/jpeg",
                    "image/gif",
                    "image/webp",
                    "image/bmp",
                ])
                .status();
            let _ = Command::new("xdg-mime")
                .args(["default", "babydra-explore.desktop", "inode/directory"])
                .status();

            log(
                LogLevel::Success,
                "Registered .desktop files & MIME associations.".into(),
            );
            copied += 1;
        }

        "dotfiles_gtk_terminal" => {
            let labwc_src = workspace_root.join("configs/labwc");
            let settings_ini = labwc_src.join("settings.ini");
            let fonts_conf = labwc_src.join("fonts.conf");

            let gtk3 = home.join(".config/gtk-3.0");
            let gtk4 = home.join(".config/gtk-4.0");
            let fontconfig = home.join(".config/fontconfig");

            let _ = fs::create_dir_all(&gtk3);
            let _ = fs::create_dir_all(&gtk4);
            let _ = fs::create_dir_all(&fontconfig);

            if settings_ini.exists() {
                let _ = fs::copy(&settings_ini, gtk3.join("settings.ini"));
                let _ = fs::copy(&settings_ini, gtk4.join("settings.ini"));
            }
            if fonts_conf.exists() {
                let _ = fs::copy(&fonts_conf, fontconfig.join("fonts.conf"));
            }

            let kitty_src = workspace_root.join("configs/kitty");
            if kitty_src.exists() {
                let _ = copy_recursive(&kitty_src, &home.join(".config/kitty"));
            }
            let nvim_src = workspace_root.join("configs/nvim");
            if nvim_src.exists() {
                let _ = copy_recursive(&nvim_src, &home.join(".config/nvim"));
            }
            let ff_src = workspace_root.join("configs/fastfetch");
            if ff_src.exists() {
                let _ = copy_recursive(&ff_src, &home.join(".config/fastfetch"));
            }

            log(
                LogLevel::Success,
                "Synced GTK-3/4, Fontconfig, Kitty, Neovim, and Fastfetch dotfiles.".into(),
            );
            copied += 1;
        }

        "themes_icons_cursors" => {
            let themes_dst = home.join(".local/share/themes");
            let icons_dst = home.join(".local/share/icons");
            let _ = fs::create_dir_all(&themes_dst);
            let _ = fs::create_dir_all(&icons_dst);

            let babydra_theme = workspace_root.join("configs/themes/BabyDra");
            if babydra_theme.exists() {
                let _ = copy_recursive(&babydra_theme, &themes_dst.join("BabyDra"));
                log(LogLevel::Success, "Installed BabyDra GTK theme.".into());
            }

            let cursor_dir = workspace_root.join("configs/themes/cursor");
            if cursor_dir.exists() {
                if let Ok(entries) = fs::read_dir(&cursor_dir) {
                    for e in entries.flatten() {
                        let path = e.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("tar") {
                            log(
                                LogLevel::Info,
                                format!(
                                    "Extracting cursor archive: {:?}",
                                    path.file_name().unwrap()
                                ),
                            );
                            let _ = Command::new("tar")
                                .args([
                                    "-xf",
                                    path.to_str().unwrap(),
                                    "-C",
                                    icons_dst.to_str().unwrap(),
                                ])
                                .status();
                        }
                    }
                }
            }

            let icon_dir = workspace_root.join("configs/themes/icons");
            if icon_dir.exists() {
                if let Ok(entries) = fs::read_dir(&icon_dir) {
                    for e in entries.flatten() {
                        let path = e.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("tar") {
                            log(
                                LogLevel::Info,
                                format!("Extracting icon theme: {:?}", path.file_name().unwrap()),
                            );
                            let _ = Command::new("tar")
                                .args([
                                    "-xf",
                                    path.to_str().unwrap(),
                                    "-C",
                                    icons_dst.to_str().unwrap(),
                                ])
                                .status();
                        }
                    }
                }
            }
            copied += 1;
        }

        "gsettings_fontcache" => {
            let _ = Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "font-name",
                    "Segoe UI Variable Static Text 13",
                ])
                .status();
            let _ = Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "document-font-name",
                    "Segoe UI Variable Static Text 13",
                ])
                .status();
            let _ = Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "monospace-font-name",
                    "CaskaydiaCove Nerd Font 11",
                ])
                .status();
            let _ = Command::new("gsettings")
                .args(["set", "org.gnome.desktop.interface", "icon-theme", "We10X"])
                .status();
            let _ = Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.desktop.interface",
                    "cursor-theme",
                    "Twilight-cursors",
                ])
                .status();

            log(
                LogLevel::Info,
                "Rebuilding font cache (fc-cache -fv)...".into(),
            );
            let _ = Command::new("fc-cache").arg("-fv").status();

            log(
                LogLevel::Success,
                "Applied system GSettings and rebuilt font cache.".into(),
            );
            copied += 1;
        }

        "restart_services" => {
            let labwc_running = Command::new("pgrep")
                .arg("-x")
                .arg("labwc")
                .status()
                .map(|s| s.success())
                .unwrap_or(false);
            if labwc_running {
                let _ = Command::new("labwc").arg("--reconfigure").status();
                log(
                    LogLevel::Success,
                    "Reloaded labwc compositor configuration.".into(),
                );
            }

            let panel_bin = user_bin_dir.join("babydra-panel");
            if panel_bin.exists() {
                log(
                    LogLevel::Info,
                    "Starting babydra-panel background service...".into(),
                );
                let log_dir = home.join(".cache/babydra");
                let _ = fs::create_dir_all(&log_dir);
                let _ = Command::new(&panel_bin).spawn();
                log(LogLevel::Success, "babydra-panel started.".into());
            }
            copied += 1;
        }

        _ => {}
    }

    (copied, 0)
}
