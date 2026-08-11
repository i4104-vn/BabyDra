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
            });
        } else if parts.len() == 3 {
            updates.push(PackageUpdate {
                name: parts[0].to_string(),
                old_version: parts[1].to_string(),
                new_version: parts[2].to_string(),
            });
        }
    }

    Ok(updates)
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
    execute_cmd_with_log_stream(&["sh", "-c", "yes | pacman -Syu --noconfirm --needed"], password, sender)
}
