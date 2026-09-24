use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::core::manifest::ConfigRule;
use crate::models::LogLevel;
use crate::runtime::{copy_recursive, expand_path, get_user_home};

/// Syncs a single declarative configuration rule from `workspace.toml`.
pub fn sync_config_rule<F>(
    workspace_root: &Path,
    rule: &ConfigRule,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let src = workspace_root.join(&rule.source);
    let dst = expand_path(&rule.target);

    if !src.exists() {
        log(
            LogLevel::Warn,
            format!("Config source '{}' does not exist in workspace; skipped.", rule.source),
        );
        return (0, 0);
    }

    if src.is_file() {
        if let Some(parent) = dst.parent() {
            let _ = fs::create_dir_all(parent);
        }
        match fs::copy(&src, &dst) {
            Ok(_) => {
                log(
                    LogLevel::Success,
                    format!("Synced file {} -> {}", rule.source, dst.display()),
                );
                (1, 0)
            }
            Err(e) => {
                log(
                    LogLevel::Error,
                    format!("Failed to copy file {} -> {}: {e}", rule.source, dst.display()),
                );
                (0, 1)
            }
        }
    } else if src.is_dir() {
        match copy_recursive(&src, &dst) {
            Ok(_) => {
                // Apply executable permissions based on patterns in `rule.executable`
                for pattern in &rule.executable {
                    apply_executable_pattern(&dst, pattern);
                }
                log(
                    LogLevel::Success,
                    format!("Synced directory {} -> {}", rule.source, dst.display()),
                );
                (1, 0)
            }
            Err(e) => {
                log(
                    LogLevel::Error,
                    format!("Failed to sync directory {} -> {}: {e}", rule.source, dst.display()),
                );
                (0, 1)
            }
        }
    } else {
        (0, 0)
    }
}

/// Convention-based fallback for Labwc compositor when no [[installer.configs]] declared.
pub fn sync_labwc_fallback<F>(workspace_root: &Path, mut log: F) -> usize
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let home = get_user_home();
    let labwc_src = {
        let assets_labwc = workspace_root.join("assets").join("configs").join("labwc");
        if assets_labwc.exists() {
            assets_labwc
        } else {
            workspace_root.join("configs/labwc")
        }
    };
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

    copied
}

/// Convention-based fallback for dotfiles when no [[installer.configs]] declared.
pub fn sync_dotfiles_fallback<F>(workspace_root: &Path, mut log: F) -> usize
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let home = get_user_home();
    let configs_src = {
        let assets_configs = workspace_root.join("assets").join("configs");
        if assets_configs.exists() {
            assets_configs
        } else {
            workspace_root.join("configs")
        }
    };
    let mut synced = Vec::new();
    if let Ok(entries) = fs::read_dir(&configs_src) {
        for entry in entries.flatten() {
            let source = entry.path();
            let name = entry.file_name();
            if source.is_dir() && name.to_string_lossy() != "themes" && name.to_string_lossy() != "labwc" {
                let destination = home.join(".config").join(&name);
                if copy_recursive(&source, &destination).is_ok() {
                    synced.push(name.to_string_lossy().to_string());
                }
            }
        }
    }

    let labwc_src = configs_src.join("labwc");
    let gtk3 = home.join(".config/gtk-3.0");
    let gtk4 = home.join(".config/gtk-4.0");
    let fontconfig = home.join(".config/fontconfig");
    let _ = fs::create_dir_all(&gtk3);
    let _ = fs::create_dir_all(&gtk4);
    let _ = fs::create_dir_all(&fontconfig);
    let settings_ini = labwc_src.join("settings.ini");
    if settings_ini.is_file() {
        let _ = fs::copy(&settings_ini, gtk3.join("settings.ini"));
        let _ = fs::copy(&settings_ini, gtk4.join("settings.ini"));
    }
    let fonts_conf = labwc_src.join("fonts.conf");
    if fonts_conf.is_file() {
        let _ = fs::copy(&fonts_conf, fontconfig.join("fonts.conf"));
    }

    if !synced.is_empty() {
        log(
            LogLevel::Success,
            format!("Synced source-defined config directories: {}.", synced.join(", ")),
        );
        copied += 1;
    }

    copied
}

fn apply_executable_pattern(base: &Path, pattern: &str) {
    if pattern.ends_with("/*") {
        let sub = pattern.trim_end_matches("/*");
        let dir = base.join(sub);
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                make_executable(&entry.path());
            }
        }
    } else {
        make_executable(&base.join(pattern));
    }
}

fn make_executable(path: &Path) {
    if path.exists() {
        let mut perms = fs::metadata(path)
            .map(|m| m.permissions())
            .unwrap_or_else(|_| fs::Permissions::from_mode(0o755));
        perms.set_mode(0o755);
        let _ = fs::set_permissions(path, perms);
    }
}
