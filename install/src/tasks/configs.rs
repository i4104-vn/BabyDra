use crate::models::{GenericOptionItem, LogLevel};
use crate::system::{copy_recursive, get_user_home, get_user_local_bin, SudoSession};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

pub fn execute_configs_task<F>(
    opt: &GenericOptionItem,
    workspace_root: &Path,
    sudo: &SudoSession,
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
            let preview_path = apps_dir.join("babydra-preview.desktop");
            let _ = fs::write(&preview_path, preview_desktop);
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(&preview_path, fs::Permissions::from_mode(0o755));
            }

            let settings_desktop = format!(
                "[Desktop Entry]\nType=Application\nName=BabyDra Settings\nComment=Configure system settings\nExec={}/.local/bin/babydra-settings\nIcon=/usr/share/babydra/babydra-settings.png\nTerminal=false\nCategories=Settings;HardwareSettings;GTK;\nNoDisplay=false\n",
                home.display()
            );
            let settings_path = apps_dir.join("babydra-settings.desktop");
            let _ = fs::write(&settings_path, settings_desktop);
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(&settings_path, fs::Permissions::from_mode(0o755));
            }

            let explore_desktop = format!(
                "[Desktop Entry]\nType=Application\nName=BabyDra Explore\nComment=Explore files and folders\nExec={}/.local/bin/babydra-explore %u\nIcon=system-file-manager\nTerminal=false\nCategories=System;FileTools;FileManager;GTK;\nMimeType=inode/directory;\nNoDisplay=false\n",
                home.display()
            );
            let explore_path = apps_dir.join("babydra-explore.desktop");
            let _ = fs::write(&explore_path, explore_desktop);
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let _ = fs::set_permissions(&explore_path, fs::Permissions::from_mode(0o755));
            }

            // Register DBus service for FileManager1
            let dbus_services_dir = home.join(".local/share/dbus-1/services");
            let _ = fs::create_dir_all(&dbus_services_dir);
            let file_manager_service = format!(
                "[D-BUS Service]\nName=org.freedesktop.FileManager1\nExec={}/.local/bin/babydra-explore\n",
                home.display()
            );
            let _ = fs::write(
                dbus_services_dir.join("org.freedesktop.FileManager1.service"),
                file_manager_service,
            );

            // All external commands run through the safe executor: stdout/stderr
            // are captured, never printed over the raw-mode TUI.
            let _ = sudo.run(
                "update-desktop-database",
                &[apps_dir.to_str().unwrap_or("")],
            );
            let _ = sudo.run(
                "xdg-mime",
                &[
                    "default",
                    "babydra-preview.desktop",
                    "image/png",
                    "image/jpeg",
                    "image/gif",
                    "image/webp",
                    "image/bmp",
                ],
            );
            let _ = sudo.run(
                "xdg-mime",
                &["default", "babydra-explore.desktop", "inode/directory"],
            );

            log(
                LogLevel::Success,
                "Registered .desktop files, FileManager1 DBus service & MIME associations.".into(),
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

            // Also deploy BabyDra CSS theme packages to ~/.babydra/themes & /usr/share/babydra/themes
            let theme_pkgs_src = workspace_root.join("themes");
            let user_themes_dst = home.join(".babydra/themes");
            if theme_pkgs_src.is_dir() {
                let _ = copy_recursive(&theme_pkgs_src, &user_themes_dst);
                let _ = sudo.run_root_quiet(&["mkdir", "-p", "/usr/share/babydra/themes"]);
                let _ = sudo.run_root_quiet(&[
                    "cp",
                    "-r",
                    &format!("{}/.", theme_pkgs_src.to_str().unwrap_or("")),
                    "/usr/share/babydra/themes/",
                ]);
                log(
                    LogLevel::Success,
                    "Installed BabyDra CSS theme packages (default, blue, green, purple, rose).".into(),
                );
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
                            let _ = sudo.run(
                                "tar",
                                &[
                                    "-xf",
                                    path.to_str().unwrap_or(""),
                                    "-C",
                                    icons_dst.to_str().unwrap_or(""),
                                ],
                            );
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
                            let _ = sudo.run(
                                "tar",
                                &[
                                    "-xf",
                                    path.to_str().unwrap_or(""),
                                    "-C",
                                    icons_dst.to_str().unwrap_or(""),
                                ],
                            );
                        }
                    }
                }
            }
            copied += 1;
        }

        "gsettings_fontcache" => {
            let font_cmds: &[&[&str]] = &[
                &[
                    "gsettings",
                    "set",
                    "org.gnome.desktop.interface",
                    "font-name",
                    "Segoe UI Variable Static Text 13",
                ],
                &[
                    "gsettings",
                    "set",
                    "org.gnome.desktop.interface",
                    "document-font-name",
                    "Segoe UI Variable Static Text 13",
                ],
                &[
                    "gsettings",
                    "set",
                    "org.gnome.desktop.interface",
                    "monospace-font-name",
                    "CaskaydiaCove Nerd Font 11",
                ],
                &[
                    "gsettings",
                    "set",
                    "org.gnome.desktop.interface",
                    "icon-theme",
                    "We10X",
                ],
                &[
                    "gsettings",
                    "set",
                    "org.gnome.desktop.interface",
                    "cursor-theme",
                    "Twilight-cursors",
                ],
            ];
            for cmd in font_cmds {
                let _ = sudo.run(cmd[0], &cmd[1..]);
            }

            log(
                LogLevel::Info,
                "Rebuilding font cache (fc-cache -fv)...".into(),
            );
            let _ = sudo.run("fc-cache", &["-fv"]);

            log(
                LogLevel::Success,
                "Applied system GSettings and rebuilt font cache.".into(),
            );
            copied += 1;
        }

        "restart_services" => {
            let labwc_running = sudo
                .run("pgrep", &["-x", "labwc"])
                .map(|o| o.success)
                .unwrap_or(false);
            if labwc_running {
                let _ = sudo.run("labwc", &["--reconfigure"]);
                log(
                    LogLevel::Success,
                    "Reloaded labwc compositor configuration.".into(),
                );
            }

            // Remove stale socket and restart switcher daemon
            let switcher_bin = user_bin_dir.join("babydra-switcher");
            if switcher_bin.exists() {
                let _ = fs::remove_file("/tmp/babydra-switcher.socket");
                let _ = Command::new(&switcher_bin)
                    .arg("--daemon")
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();
                log(
                    LogLevel::Success,
                    "babydra-switcher --daemon started in background.".into(),
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
                let log_file = log_dir.join("panel.log");
                // Spawn detached with output redirected to a log file so the
                // TUI is never written to by the child process.
                let opened = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&log_file);
                if let Ok(f) = opened {
                    let out = f.try_clone().unwrap_or_else(|_| {
                        fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&log_file)
                            .expect("reopen panel log")
                    });
                    let _ = Command::new(&panel_bin)
                        .stdout(std::process::Stdio::from(out))
                        .stderr(std::process::Stdio::from(f))
                        .spawn();
                }
                log(
                    LogLevel::Success,
                    format!("babydra-panel started (logs: {}).", log_file.display()),
                );
            }

            let desktop_bin = user_bin_dir.join("babydra-desktop");
            if desktop_bin.exists() {
                let _ = Command::new(&desktop_bin)
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .spawn();
                log(
                    LogLevel::Success,
                    "babydra-desktop started in background.".into(),
                );
            }
            copied += 1;
        }

        _ => {}
    }

    (copied, 0)
}

/// Deploys the theme packages tree (`themes/`) to `~/.babydra/themes` and `/usr/share/babydra/themes`
/// and writes the selected variant's theme id into `~/.babydra/babydra.conf`
/// (`[theme] selection = { id = "..." }`), which the UI reads at startup.
///
/// This makes the installer's variant step actually switch the theme
/// the running desktop renders with — no code change required.
pub fn deploy_theme_packages<F>(workspace_root: &Path, theme_id: &str, sudo: &SudoSession, mut log: F)
where
    F: FnMut(LogLevel, String),
{
    let home = get_user_home();
    let themes_src = workspace_root.join("themes");
    let themes_dst = home.join(".babydra/themes");

    if themes_src.is_dir() {
        let _ = copy_recursive(&themes_src, &themes_dst);
        let _ = sudo.run_root_quiet(&["mkdir", "-p", "/usr/share/babydra/themes"]);
        let _ = sudo.run_root_quiet(&[
            "cp",
            "-r",
            &format!("{}/.", themes_src.to_str().unwrap_or("")),
            "/usr/share/babydra/themes/",
        ]);
        log(
            LogLevel::Success,
            format!("Deployed theme packages to {} and /usr/share/babydra/themes", themes_dst.display()),
        );
    } else {
        log(
            LogLevel::Warn,
            "themes/ not found in workspace — skipping theme packages deploy.".into(),
        );
    }

    // Persist the selected theme id into ~/.babydra/babydra.conf
    let conf_path = home.join(".babydra/babydra.conf");
    let selected_id = if theme_id.is_empty() { "babydra-default" } else { theme_id };
    if let Err(e) = write_theme_selection(&conf_path, selected_id) {
        log(
            LogLevel::Warn,
            format!("Could not write theme selection: {e}"),
        );
    } else {
        log(
            LogLevel::Info,
            format!("babydra.conf theme.selection.id = {selected_id}"),
        );
    }
}

/// Sets `[theme] selection.id` inside the TOML config, preserving other keys.
/// Writes `theme.selection.id = "<theme_id>"` into a `babydra.conf` TOML file.
///
/// Preserves all existing keys/sections and replaces any previous
/// `selection` value under `[theme]`. Returns an error string on I/O or
/// parse failure.
pub fn write_theme_selection(conf_path: &Path, theme_id: &str) -> Result<(), String> {
    let mut root: toml::Table = if conf_path.exists() {
        let content = fs::read_to_string(conf_path)
            .map_err(|e| format!("cannot read {}: {e}", conf_path.display()))?;
        content
            .parse::<toml::Table>()
            .map_err(|e| format!("invalid TOML in {}: {e}", conf_path.display()))?
    } else {
        toml::Table::new()
    };

    let theme_table = root
        .entry("theme".to_string())
        .or_insert_with(|| toml::Value::Table(toml::Table::new()));
    let theme_table = theme_table
        .as_table_mut()
        .ok_or_else(|| "[theme] is not a table".to_string())?;

    let selection = theme_table
        .entry("selection".to_string())
        .or_insert_with(|| toml::Value::Table(toml::Table::new()));
    let selection = selection
        .as_table_mut()
        .ok_or_else(|| "[theme.selection] is not a table".to_string())?;

    selection.insert("id".to_string(), toml::Value::String(theme_id.to_string()));

    if let Some(parent) = conf_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
    }
    let out = toml::to_string_pretty(&root).map_err(|e| format!("cannot serialize config: {e}"))?;
    fs::write(conf_path, out).map_err(|e| format!("cannot write {}: {e}", conf_path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_theme_selection() {
        let temp_dir = std::env::temp_dir().join(format!("babydra_test_{}", std::process::id()));
        let _ = fs::create_dir_all(&temp_dir);
        let conf_file = temp_dir.join("babydra.conf");

        assert!(write_theme_selection(&conf_file, "babydra-purple").is_ok());
        let content = fs::read_to_string(&conf_file).unwrap();
        assert!(content.contains("babydra-purple"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
