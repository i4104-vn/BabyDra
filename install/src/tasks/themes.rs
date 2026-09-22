use std::fs;
use std::path::Path;

use crate::core::manifest::InstallManifest;
use crate::models::LogLevel;
use crate::runtime::{copy_recursive, expand_path, get_user_home, SudoSession};

pub fn install_themes_icons_cursors<F>(
    workspace_root: &Path,
    manifest: &InstallManifest,
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

    let archives_rel = manifest
        .themes
        .archives
        .as_deref()
        .unwrap_or("configs/themes");
    let themes_src = workspace_root.join(archives_rel);
    for child in direct_directories(&themes_src) {
        if contains_archive(&child) {
            continue;
        }
        if let Some(name) = child.file_name() {
            let _ = copy_recursive(&child, &themes_dst.join(name));
        }
    }

    for archive in find_archives(&themes_src) {
        log(
            LogLevel::Info,
            format!(
                "Extracting theme archive: {:?}",
                archive.file_name().unwrap_or_default()
            ),
        );
        let _ = sudo.run(
            "tar",
            &[
                "-xf",
                archive.to_str().unwrap_or(""),
                "-C",
                icons_dst.to_str().unwrap_or(""),
            ],
        );
    }
    copied += 1;

    copied
}

pub fn apply_gsettings_fontcache<F>(
    manifest: &InstallManifest,
    sudo: &SudoSession,
    mut log: F,
) -> usize
where
    F: FnMut(LogLevel, String),
{
    for (schema_and_key, value) in &manifest.gsettings {
        let Some((schema, key)) = schema_and_key.rsplit_once('.') else {
            continue;
        };
        let _ = sudo.run("gsettings", &["set", schema, key, value]);
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

/// Deploys the theme packages tree to user and system theme directories
/// and writes the selected variant's theme id into the config path.
pub fn deploy_theme_packages<F>(
    workspace_root: &Path,
    theme_id: &str,
    manifest: &InstallManifest,
    sudo: &SudoSession,
    mut log: F,
) where
    F: FnMut(LogLevel, String),
{
    let home = get_user_home();
    let pkg_rel = manifest.themes.packages.as_deref().unwrap_or("themes");
    let themes_src = workspace_root.join(pkg_rel);
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
            format!("{pkg_rel}/ not found in workspace — skipping theme packages deploy."),
        );
    }

    let conf_path = manifest
        .themes
        .conf_path
        .as_deref()
        .map(expand_path)
        .unwrap_or_else(|| home.join(".babydra/babydra.conf"));

    let selected_id = if theme_id.is_empty() {
        first_directory_name(&themes_src).unwrap_or_else(|| "default".to_owned())
    } else {
        theme_id.to_owned()
    };
    if let Err(e) = write_theme_selection(&conf_path, &selected_id) {
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

fn direct_directories(root: &Path) -> Vec<std::path::PathBuf> {
    fs::read_dir(root)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .collect()
}

fn contains_archive(root: &Path) -> bool {
    !find_archives(root).is_empty()
}

fn find_archives(root: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    collect_files(root, &mut files, |path| {
        path.extension().and_then(|ext| ext.to_str()) == Some("tar")
    });
    files
}

fn first_directory_name(root: &Path) -> Option<String> {
    direct_directories(root)
        .into_iter()
        .find(|path| path.join("tokens.json").is_file() || path.join("css").is_dir())
        .and_then(|path| {
            path.file_name()
                .map(|name| name.to_string_lossy().to_string())
        })
}

fn collect_files<F>(root: &Path, files: &mut Vec<std::path::PathBuf>, predicate: F)
where
    F: Fn(&Path) -> bool + Copy,
{
    if !root.is_dir() {
        return;
    }
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, files, predicate);
        } else if predicate(&path) {
            files.push(path);
        }
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
pub mod tests {
    use super::*;

    #[test]
    fn test_write_theme_selection() {
        let temp_dir = std::env::temp_dir().join(format!("babydra_test_themes_{}", std::process::id()));
        let _ = fs::create_dir_all(&temp_dir);
        let conf_file = temp_dir.join("babydra.conf");

        assert!(write_theme_selection(&conf_file, "babydra-purple").is_ok());
        let content = fs::read_to_string(&conf_file).unwrap();
        assert!(content.contains("babydra-purple"));

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
