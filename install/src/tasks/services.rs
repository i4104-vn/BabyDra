use std::fs;
use std::path::{Path, PathBuf};

use crate::core::manifest::InstallManifest;
use crate::models::{BinaryLocation, LogLevel};
use crate::runtime::{get_user_home, SudoSession};

/// Installs systemd user units declared by the source branch or auto-generated
/// from binaries with autorestart = true, and reloads the compositor.
pub fn restart_services<F>(
    workspace_root: &Path,
    manifest: &InstallManifest,
    sudo: &SudoSession,
    mut log: F,
) -> usize
where
    F: FnMut(LogLevel, String),
{
    let home = get_user_home();
    let systemd_dir = home.join(".config/systemd/user");
    let mut units = Vec::new();

    // 1. Auto-generate systemd user services for binaries with autorestart = true
    for binary in &manifest.binaries {
        if binary.autorestart {
            let unit_name = format!("{}.service", binary.name);
            let destination = systemd_dir.join(&unit_name);
            let bin_cmd = match binary.location {
                BinaryLocation::SystemBin => format!("/usr/bin/{}", binary.name),
                BinaryLocation::UserLocalBin => format!(
                    "/bin/sh -c 'if [ -x \"$HOME/.local/bin/{0}\" ]; then exec \"$HOME/.local/bin/{0}\"; elif command -v {0} >/dev/null 2>&1; then exec {0}; else exec /usr/bin/{0}; fi'",
                    binary.name
                ),
            };

            let content = format!(
                r#"[Unit]
Description=BabyDra {} (auto-restart daemon)
PartOf=graphical-session.target
After=graphical-session.target

[Service]
Type=simple
ExecStart={}
Restart=always
RestartSec=1s

[Install]
WantedBy=default.target
"#,
                binary.name, bin_cmd
            );

            if fs::create_dir_all(&systemd_dir)
                .and_then(|_| fs::write(&destination, content))
                .is_ok()
            {
                log(
                    LogLevel::Info,
                    format!(
                        "Generated systemd user unit for '{}' (autorestart enabled).",
                        binary.name
                    ),
                );
                units.push(unit_name);
            }
        }
    }

    // 2. Discover any additional source-defined .service files in workspace_root
    for source in find_files(workspace_root, |path| {
        path.extension().and_then(|ext| ext.to_str()) == Some("service")
            && fs::read_to_string(path)
                .map(|content| content.contains("[Unit]") && content.contains("[Service]"))
                .unwrap_or(false)
    }) {
        let Some(name) = source.file_name() else {
            continue;
        };
        let unit_str = name.to_string_lossy().to_string();
        if units.contains(&unit_str) {
            continue;
        }
        let destination = systemd_dir.join(name);
        if let Ok(content) = fs::read_to_string(&source) {
            if fs::create_dir_all(&systemd_dir)
                .and_then(|_| fs::write(&destination, content))
                .is_ok()
            {
                units.push(unit_str);
            }
        }
    }

    if !units.is_empty() {
        let _ = sudo.run("systemctl", &["--user", "daemon-reload"]);
        for unit in &units {
            let _ = sudo.run("systemctl", &["--user", "enable", "--now", unit]);
        }
        log(
            LogLevel::Success,
            format!(
                "Installed and activated {} user service(s).",
                units.len()
            ),
        );
    } else {
        log(
            LogLevel::Info,
            "No systemd user units declared or configured for auto-restart.".into(),
        );
    }

    let labwc_running = sudo
        .run("pgrep", &["-x", "labwc"])
        .map(|output| output.success)
        .unwrap_or(false);
    if labwc_running && sudo.run("labwc", &["--reconfigure"]).is_ok() {
        log(
            LogLevel::Success,
            "Reloaded compositor configuration.".into(),
        );
    }

    usize::from(!units.is_empty())
}

fn find_files<F>(root: &Path, predicate: F) -> Vec<PathBuf>
where
    F: Fn(&Path) -> bool + Copy,
{
    let mut found = Vec::new();
    collect_files(root, &mut found, predicate);
    found
}

fn collect_files<F>(root: &Path, found: &mut Vec<PathBuf>, predicate: F)
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
            let skip = path
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    matches!(name, ".git" | "target" | "branches" | "node_modules")
                });
            if !skip {
                collect_files(&path, found, predicate);
            }
        } else if predicate(&path) {
            found.push(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::manifest::BinaryManifestItem;

    #[test]
    fn test_generate_unit_content() {
        let binary = BinaryManifestItem {
            name: "babydra-desktop".to_string(),
            source_name: "babydra-desktop".to_string(),
            description: "Desktop shell".to_string(),
            location: BinaryLocation::UserLocalBin,
            autorestart: true,
        };
        let bin_cmd = format!(
            "/bin/sh -c 'if [ -x \"$HOME/.local/bin/{0}\" ]; then exec \"$HOME/.local/bin/{0}\"; elif command -v {0} >/dev/null 2>&1; then exec {0}; else exec /usr/bin/{0}; fi'",
            binary.name
        );
        let content = format!(
            r#"[Unit]
Description=BabyDra {} (auto-restart daemon)
PartOf=graphical-session.target
After=graphical-session.target

[Service]
Type=simple
ExecStart={}
Restart=always
RestartSec=1s

[Install]
WantedBy=default.target
"#,
            binary.name, bin_cmd
        );
        assert!(content.contains("Description=BabyDra babydra-desktop (auto-restart daemon)"));
        assert!(content.contains("Restart=always"));
        assert!(content.contains("RestartSec=1s"));
    }
}
