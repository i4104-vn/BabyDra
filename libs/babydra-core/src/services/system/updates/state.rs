//! Update log & state helpers (split out of `mod.rs`).

use crate::error::CoreResult;
use crate::models::system_update::{PackageUpdate, SystemUpdateState};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use super::{exec_cmd_stream, validate_sudo};

pub fn get_update_log_path() -> PathBuf {
    std::env::temp_dir().join("babydra-update.log")
}

/// Returns `true` when `pacman running` holds, `false` otherwise.
pub fn is_pacman_running() -> bool {
    if let Ok(out) = Command::new("pgrep").arg("-x").arg("pacman").output() {
        out.status.success() && !out.stdout.is_empty()
    } else {
        false
    }
}

/// Read update log.
pub fn read_update_log() -> String {
    let path = get_update_log_path();
    std::fs::read_to_string(path).unwrap_or_default()
}

/// Starts `background update`.
pub fn start_bg_update(password: Option<String>) {
    start_bg_update_tx(password, None);
}

/// Starts `background update with sender`.
pub fn start_bg_update_tx(
    password: Option<String>,
    external_tx: Option<std::sync::mpsc::Sender<String>>,
) {
    let path = get_update_log_path();
    let _ = std::fs::write(&path, "");

    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel::<String>();
        let pwd_val = password;

        let log_path = get_update_log_path();
        let tx_clone = tx.clone();

        std::thread::spawn(move || {
            let res = stream_update_system(pwd_val.as_deref(), tx_clone.clone());
            if let Err(e) = res {
                let _ = tx_clone.send(format!("\nError: {}", e));
            } else {
                let _ = tx_clone.send("\nSystem update completed successfully.".to_string());
            }
        });

        while let Ok(line) = rx.recv() {
            if let Some(ref ext_tx) = external_tx {
                let _ = ext_tx.send(line.clone());
            }
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_path) {
                let _ = writeln!(file, "{}", line);
                let _ = file.flush();
            }
        }
    });
}

/// Clean pacman lock.
pub fn clean_pacman_lock(password: Option<&str>, sender: std::sync::mpsc::Sender<String>) {
    if std::path::Path::new("/var/lib/pacman/db.lck").exists() {
        if !is_pacman_running() {
            let _ = sender.send(":: Detected stale pacman lock file (/var/lib/pacman/db.lck). Cleaning lock file...".to_string());
            let _ = exec_cmd_stream(&["rm", "-f", "/var/lib/pacman/db.lck"], password, sender);
        }
    }
}

/// Triggers system update streaming output via sender channel.
pub fn stream_update_system(
    password: Option<&str>,
    sender: std::sync::mpsc::Sender<String>,
) -> CoreResult<()> {
    clean_pacman_lock(password, sender.clone());
    exec_cmd_stream(
        &["pacman", "-Syu", "--noconfirm", "--needed"],
        password,
        sender,
    )
}

/// Returns the current `update state path`.
pub fn get_update_path() -> std::path::PathBuf {
    std::env::temp_dir().join("babydra-update-state.json")
}

/// Persists `update state`.
pub fn save_update_state(is_updating: bool, is_syncing: bool, packages: &[PackageUpdate]) {
    let state = SystemUpdateState {
        is_updating,
        is_syncing,
        packages: packages.to_vec(),
    };
    if let Ok(json) = serde_json::to_string_pretty(&state) {
        let _ = std::fs::write(get_update_path(), json);
    }
}

/// Loads `update state`.
pub fn load_update_state() -> Option<SystemUpdateState> {
    let path = get_update_path();
    if path.exists() {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(state) = serde_json::from_str::<SystemUpdateState>(&content) {
                return Some(state);
            }
        }
    }
    None
}

/// Clear update state.
pub fn clear_update_state() {
    let path = get_update_path();
    if path.exists() {
        let _ = std::fs::remove_file(path);
    }
    let log_path = get_update_log_path();
    if log_path.exists() {
        let _ = std::fs::remove_file(log_path);
    }
    if std::path::Path::new("/var/lib/pacman/db.lck").exists() && !is_pacman_running() {
        let _ = std::fs::remove_file("/var/lib/pacman/db.lck");
    }
}

/// Parses `pacman progress line`.
pub fn parse_pacman_prog(line: &str) -> Option<(usize, usize, String)> {
    let line_trimmed = line.trim();

    if let Some(start) = line_trimmed.find('(') {
        if let Some(end) = line_trimmed[start..].find(')') {
            let inner = &line_trimmed[start + 1..start + end];
            let parts: Vec<&str> = inner.split('/').collect();
            if parts.len() == 2 {
                if let (Ok(curr), Ok(total)) = (
                    parts[0].trim().parse::<usize>(),
                    parts[1].trim().parse::<usize>(),
                ) {
                    let rest = line_trimmed[start + end + 1..].trim();
                    let words: Vec<&str> = rest.split_whitespace().collect();
                    if words.len() >= 2 {
                        let pkg_name = words[1].to_string();
                        return Some((curr, total, pkg_name));
                    } else if words.len() == 1 {
                        return Some((curr, total, words[0].to_string()));
                    }
                }
            }
        }
    }

    for action in &["upgrading ", "installing ", "reinstalling ", "downgrading "] {
        if let Some(idx) = line_trimmed.find(action) {
            let rest = line_trimmed[idx + action.len()..].trim();
            let pkg_name = rest
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim_matches('.')
                .to_string();
            if !pkg_name.is_empty() {
                return Some((1, 1, pkg_name));
            }
        }
    }

    None
}

/// Run background update loop.
pub fn run_bg_update_loop(password: Option<&str>) {
    let mut state = match load_update_state() {
        Some(s) if !s.packages.is_empty() => s,
        _ => return,
    };

    if let Some(pwd) = password {
        if !validate_sudo(pwd) {
            for pkg in state.packages.iter_mut() {
                pkg.status = crate::models::system_update::UpdateStatus::Failed;
            }
            state.is_updating = false;
            save_update_state(false, false, &state.packages);
            crate::send_settings_notif(
                &crate::i18n::trans("settings.update_title"),
                "Authentication Failed: Incorrect sudo password.",
            );
            return;
        }
    }

    state.is_updating = true;
    state.is_syncing = true;
    for pkg in state.packages.iter_mut() {
        pkg.status = crate::models::system_update::UpdateStatus::Pending;
    }
    save_update_state(true, true, &state.packages);

    let (tx, rx) = std::sync::mpsc::channel::<String>();

    let pwd_clone = password.map(|s| s.to_string());
    let tx_clone = tx.clone();
    std::thread::spawn(move || {
        let res = stream_update_system(pwd_clone.as_deref(), tx_clone);
        if let Err(e) = res {
            tracing::error!("Background system update error: {}", e);
        }
    });

    let mut current_updating_pkg: Option<String> = None;

    while let Ok(line) = rx.recv() {
        if let Some((_curr, _total, pkg_name)) = parse_pacman_prog(&line) {
            if let Some(prev) = current_updating_pkg.take() {
                if let Some(pkg) = state.packages.iter_mut().find(|p| p.name == prev) {
                    pkg.status = crate::models::system_update::UpdateStatus::Done;
                }
            }

            if let Some(pkg) = state.packages.iter_mut().find(|p| p.name == pkg_name) {
                pkg.status = crate::models::system_update::UpdateStatus::Updating;
            }
            current_updating_pkg = Some(pkg_name);
            save_update_state(true, false, &state.packages);
            std::thread::sleep(std::time::Duration::from_millis(600));
        } else if line.contains("completed successfully") {
            for pkg in state.packages.iter_mut() {
                pkg.status = crate::models::system_update::UpdateStatus::Done;
            }
            break;
        } else if line.contains("Error:") || line.contains("failed") {
            if let Some(curr) = current_updating_pkg.take() {
                if let Some(pkg) = state.packages.iter_mut().find(|p| p.name == curr) {
                    pkg.status = crate::models::system_update::UpdateStatus::Failed;
                }
            }
        }
    }

    let has_failed = state
        .packages
        .iter()
        .any(|p| p.status == crate::models::system_update::UpdateStatus::Failed);
    if !has_failed {
        for pkg in state.packages.iter_mut() {
            pkg.status = crate::models::system_update::UpdateStatus::Done;
        }
    }

    state.is_updating = false;
    save_update_state(false, false, &state.packages);

    let failed_count = state
        .packages
        .iter()
        .filter(|p| p.status == crate::models::system_update::UpdateStatus::Failed)
        .count();
    let title = crate::i18n::trans("settings.update_title");
    let msg = if failed_count == 0 {
        crate::i18n::trans("settings.update_complete")
    } else {
        crate::i18n::trans("settings.update_failed").replace("{count}", &failed_count.to_string())
    };

    crate::send_settings_notif(&title, &msg);

    if failed_count == 0 {
        let log_path = get_update_log_path();
        if log_path.exists() {
            let _ = std::fs::remove_file(log_path);
        }
    }
}
