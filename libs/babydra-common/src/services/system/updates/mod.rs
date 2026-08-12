//! System update service.

use crate::models::system_update::PackageUpdate;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

/// Checks for pending system updates using checkupdates with pacman -Qu fallback.
pub fn check_updates() -> Result<Vec<PackageUpdate>, String> {
    let mut updates = Vec::new();

    // Try checkupdates first
    let output = Command::new("checkupdates").output();
    
    let stdout = match output {
        Ok(ref out) if out.status.success() => String::from_utf8_lossy(&out.stdout).to_string(),
        _ => {
            // Fallback to pacman -Qu
            if let Ok(out) = Command::new("pacman").arg("-Qu").output() {
                String::from_utf8_lossy(&out.stdout).to_string()
            } else {
                String::new()
            }
        }
    };

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            updates.push(PackageUpdate {
                name: parts[0].to_string(),
                old_version: parts[1].to_string(),
                new_version: parts[3].to_string(),
                status: crate::models::system_update::UpdateStatus::Pending,
            });
        } else if parts.len() == 3 {
            updates.push(PackageUpdate {
                name: parts[0].to_string(),
                old_version: parts[1].to_string(),
                new_version: parts[2].to_string(),
                status: crate::models::system_update::UpdateStatus::Pending,
            });
        }
    }

    Ok(updates)
}

pub fn validate_sudo_password(password: &str) -> bool {
    let mut child = match Command::new("sudo")
        .arg("-S")
        .arg("-v")
        .arg("-p")
        .arg("")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "{}", password);
        let _ = stdin.flush();
    }

    child.wait().map(|s| s.success()).unwrap_or(false)
}

/// Updates a single package securely using sudo stdin password or pkexec.
pub fn update_single_package(pkg_name: &str, password: Option<&str>) -> Result<(), String> {
    if std::path::Path::new("/var/lib/pacman/db.lck").exists() && !is_pacman_running() {
        let _ = std::fs::remove_file("/var/lib/pacman/db.lck");
    }

    if let Some(pwd) = password {
        let mut child = Command::new("sudo")
            .arg("-S")
            .arg("-p")
            .arg("")
            .arg("pacman")
            .arg("-Sy")
            .arg("--noconfirm")
            .arg("--needed")
            .arg(pkg_name)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = writeln!(stdin, "{}", pwd);
            let _ = stdin.flush();
        }

        let output = child.wait_with_output().map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(())
        } else {
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            let stdout_str = String::from_utf8_lossy(&output.stdout);
            Err(format!("Failed to update {}: {} {}", pkg_name, stderr_str, stdout_str))
        }
    } else {
        let output = Command::new("pkexec")
            .args(["pacman", "-Sy", "--noconfirm", "--needed", pkg_name])
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}

pub fn update_system() -> Result<(), String> {
    let output = Command::new("pkexec")
        .args(["pacman", "-Syu", "--noconfirm"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Executes a privileged command and streams stdout/stderr lines via channel.
pub fn execute_cmd_with_log_stream(args: &[&str], password: Option<&str>, sender: std::sync::mpsc::Sender<String>) -> Result<(), String> {
    let mut command = if password.is_some() {
        let mut c = Command::new("sudo");
        c.arg("-S");
        c.arg("-p");
        c.arg("");
        c.args(args);
        c
    } else {
        let mut c = Command::new("pkexec");
        c.args(args);
        c
    };

    command.stdout(Stdio::piped());
    command.stderr(Stdio::piped());
    if password.is_some() {
        command.stdin(Stdio::piped());
    }

    let mut child = command.spawn().map_err(|e| e.to_string())?;

    if let Some(pwd) = password {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = writeln!(stdin, "{}", pwd);
            let _ = stdin.flush();
        }
    }

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let sender_stdout = sender.clone();
    let thread_stdout = std::thread::spawn(move || {
        if let Some(mut out) = stdout {
            let mut buf = [0u8; 1024];
            let mut current_line = String::new();
            while let Ok(n) = out.read(&mut buf) {
                if n == 0 { break; }
                let chunk = String::from_utf8_lossy(&buf[..n]);
                for c in chunk.chars() {
                    if c == '\n' || c == '\r' {
                        if !current_line.is_empty() {
                            let _ = sender_stdout.send(current_line.clone());
                            current_line.clear();
                        }
                    } else {
                        current_line.push(c);
                    }
                }
            }
            if !current_line.is_empty() {
                let _ = sender_stdout.send(current_line);
            }
        }
    });

    let sender_stderr = sender;
    let thread_stderr = std::thread::spawn(move || {
        if let Some(mut err) = stderr {
            let mut buf = [0u8; 1024];
            let mut current_line = String::new();
            while let Ok(n) = err.read(&mut buf) {
                if n == 0 { break; }
                let chunk = String::from_utf8_lossy(&buf[..n]);
                for c in chunk.chars() {
                    if c == '\n' || c == '\r' {
                        if !current_line.is_empty() {
                            let _ = sender_stderr.send(current_line.clone());
                            current_line.clear();
                        }
                    } else {
                        current_line.push(c);
                    }
                }
            }
            if !current_line.is_empty() {
                let _ = sender_stderr.send(current_line);
            }
        }
    });

    let _ = thread_stdout.join();
    let _ = thread_stderr.join();
    let status = child.wait().map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("Process exited with status: {}", status))
    }
}

pub fn get_update_log_path() -> std::path::PathBuf {
    std::env::temp_dir().join("babydra-update.log")
}

pub fn is_pacman_running() -> bool {
    if let Ok(out) = Command::new("pgrep").arg("-x").arg("pacman").output() {
        out.status.success() && !out.stdout.is_empty()
    } else {
        false
    }
}


pub fn read_update_log() -> String {
    let path = get_update_log_path();
    std::fs::read_to_string(path).unwrap_or_default()
}

pub fn start_background_update(password: Option<String>) {
    start_background_update_with_sender(password, None);
}

pub fn start_background_update_with_sender(password: Option<String>, external_tx: Option<std::sync::mpsc::Sender<String>>) {
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

pub fn clean_pacman_lock(password: Option<&str>, sender: std::sync::mpsc::Sender<String>) {
    if std::path::Path::new("/var/lib/pacman/db.lck").exists() {
        if !is_pacman_running() {
            let _ = sender.send(":: Detected stale pacman lock file (/var/lib/pacman/db.lck). Cleaning lock file...".to_string());
            let _ = execute_cmd_with_log_stream(&["rm", "-f", "/var/lib/pacman/db.lck"], password, sender);
        }
    }
}

/// Triggers system update streaming output via sender channel.
pub fn stream_update_system(password: Option<&str>, sender: std::sync::mpsc::Sender<String>) -> Result<(), String> {
    clean_pacman_lock(password, sender.clone());
    execute_cmd_with_log_stream(&["pacman", "-Syu", "--noconfirm", "--needed"], password, sender)
}

pub fn get_update_state_path() -> std::path::PathBuf {
    std::env::temp_dir().join("babydra-update-state.json")
}

pub fn save_update_state(is_updating: bool, is_syncing: bool, packages: &[PackageUpdate]) {
    let state = crate::models::system_update::SystemUpdateState {
        is_updating,
        is_syncing,
        packages: packages.to_vec(),
    };
    if let Ok(json) = serde_json::to_string_pretty(&state) {
        let _ = std::fs::write(get_update_state_path(), json);
    }
}

pub fn load_update_state() -> Option<crate::models::system_update::SystemUpdateState> {
    let path = get_update_state_path();
    if path.exists() {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(state) = serde_json::from_str::<crate::models::system_update::SystemUpdateState>(&content) {
                return Some(state);
            }
        }
    }
    None
}

pub fn clear_update_state() {
    let path = get_update_state_path();
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

pub fn parse_pacman_progress_line(line: &str) -> Option<(usize, usize, String)> {
    let line_trimmed = line.trim();

    if let Some(start) = line_trimmed.find('(') {
        if let Some(end) = line_trimmed[start..].find(')') {
            let inner = &line_trimmed[start + 1..start + end];
            let parts: Vec<&str> = inner.split('/').collect();
            if parts.len() == 2 {
                if let (Ok(curr), Ok(total)) = (parts[0].trim().parse::<usize>(), parts[1].trim().parse::<usize>()) {
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
            let pkg_name = rest.split_whitespace().next().unwrap_or("").trim_matches('.').to_string();
            if !pkg_name.is_empty() {
                return Some((1, 1, pkg_name));
            }
        }
    }

    None
}

pub fn run_background_update_loop(password: Option<&str>) {
    let mut state = match load_update_state() {
        Some(s) if !s.packages.is_empty() => s,
        _ => return,
    };

    if let Some(pwd) = password {
        if !validate_sudo_password(pwd) {
            for pkg in state.packages.iter_mut() {
                pkg.status = crate::models::system_update::UpdateStatus::Failed;
            }
            state.is_updating = false;
            save_update_state(false, false, &state.packages);
            crate::send_settings_notification(
                &crate::i18n::t("settings.update_title"),
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
        if let Some((_curr, _total, pkg_name)) = parse_pacman_progress_line(&line) {
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

    let has_failed = state.packages.iter().any(|p| p.status == crate::models::system_update::UpdateStatus::Failed);
    if !has_failed {
        for pkg in state.packages.iter_mut() {
            pkg.status = crate::models::system_update::UpdateStatus::Done;
        }
    }

    state.is_updating = false;
    save_update_state(false, false, &state.packages);

    let failed_count = state.packages.iter().filter(|p| p.status == crate::models::system_update::UpdateStatus::Failed).count();
    let title = crate::i18n::t("settings.update_title");
    let msg = if failed_count == 0 {
        crate::i18n::t("settings.update_complete")
    } else {
        crate::i18n::t("settings.update_failed").replace("{count}", &failed_count.to_string())
    };

    crate::send_settings_notification(&title, &msg);

    if failed_count == 0 {
        let log_path = get_update_log_path();
        if log_path.exists() {
            let _ = std::fs::remove_file(log_path);
        }
    }
}
