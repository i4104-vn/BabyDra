use std::fs;
use std::path::{Path, PathBuf};

use crate::models::LogLevel;
use crate::system::{get_user_home, SudoSession};

/// Installs systemd user units declared by the source branch and reloads the
/// compositor. Unit names and executable names are intentionally not encoded
/// in the installer; adding a daemon is a source-branch change only.
pub fn restart_services<F>(workspace_root: &Path, sudo: &SudoSession, mut log: F) -> usize
where
    F: FnMut(LogLevel, String),
{
    let home = get_user_home();
    let systemd_dir = home.join(".config/systemd/user");
    let mut units = Vec::new();

    for source in find_files(workspace_root, |path| {
        path.extension().and_then(|ext| ext.to_str()) == Some("service")
            && fs::read_to_string(path)
                .map(|content| content.contains("[Unit]") && content.contains("[Service]"))
                .unwrap_or(false)
    }) {
        let Some(name) = source.file_name() else {
            continue;
        };
        let destination = systemd_dir.join(name);
        if let Ok(content) = fs::read_to_string(&source) {
            if fs::create_dir_all(&systemd_dir)
                .and_then(|_| fs::write(&destination, content))
                .is_ok()
            {
                units.push(name.to_string_lossy().to_string());
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
                "Installed and activated {} source-defined user service(s).",
                units.len()
            ),
        );
    } else {
        log(
            LogLevel::Info,
            "No systemd user units declared by this source branch.".into(),
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
