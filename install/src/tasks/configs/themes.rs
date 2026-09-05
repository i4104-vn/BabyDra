use std::fs;
use std::path::Path;

use crate::models::LogLevel;
use crate::system::{copy_recursive, get_user_home, SudoSession};

pub fn install_themes_icons_cursors<F>(
    workspace_root: &Path,
    sudo: &SudoSession,
    mut log: F,
) -> usize
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let home = get_user_home();
    let themes_dst = home.join(".local/share/themes");
    let icons_dst = home.join(".local/share/icons");
    let _ = fs::create_dir_all(&themes_dst);
    let _ = fs::create_dir_all(&icons_dst);

    let babydra_theme = workspace_root.join("configs/themes/BabyDra");
    if babydra_theme.exists() {
        let _ = copy_recursive(&babydra_theme, &themes_dst.join("BabyDra"));
        log(LogLevel::Success, "Installed BabyDra GTK theme.".into());
    }

    for (dir, label) in [
        ("configs/themes/cursor", "cursor archive"),
        ("configs/themes/icons", "icon theme"),
    ] {
        let archive_dir = workspace_root.join(dir);
        if !archive_dir.exists() {
            continue;
        }
        if let Ok(entries) = fs::read_dir(&archive_dir) {
            for e in entries.flatten() {
                let path = e.path();
                if path.extension().and_then(|s| s.to_str()) == Some("tar") {
                    log(
                        LogLevel::Info,
                        format!("Extracting {label}: {:?}", path.file_name().unwrap()),
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

    copied
}

pub fn apply_gsettings_fontcache<F>(sudo: &SudoSession, mut log: F) -> usize
where
    F: FnMut(LogLevel, String),
{
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

    1
}

/// Deploys the theme packages tree (`themes/`) to `~/.babydra/themes` and `/usr/share/babydra/themes`
/// and writes the selected variant's theme id into `~/.babydra/babydra.conf`.
pub fn deploy_theme_packages<F>(
    workspace_root: &Path,
    theme_id: &str,
    sudo: &SudoSession,
    mut log: F,
) where
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
            format!(
                "Deployed theme packages to {} and /usr/share/babydra/themes",
                themes_dst.display()
            ),
        );
    } else {
        log(
            LogLevel::Warn,
            "themes/ not found in workspace — skipping theme packages deploy.".into(),
        );
    }

    let conf_path = home.join(".babydra/babydra.conf");
    let selected_id = if theme_id.is_empty() {
        "babydra-default"
    } else {
        theme_id
    };
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
