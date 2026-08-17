//! System update service.

pub mod state;
pub use state::*;

use crate::error::CoreResult;
use crate::models::system_update::PackageUpdate;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

/// Checks for pending system updates using checkupdates with pacman -Qu fallback.
pub fn check_updates() -> CoreResult<Vec<PackageUpdate>> {
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

/// Validate sudo password.
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
pub fn update_single_package(pkg_name: &str, password: Option<&str>) -> CoreResult<()> {
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
            Err(format!(
                "Failed to update {}: {} {}",
                pkg_name, stderr_str, stdout_str
            )
            .into())
        }
    } else {
        let output = Command::new("pkexec")
            .args(["pacman", "-Sy", "--noconfirm", "--needed", pkg_name])
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string().into())
        }
    }
}

/// Updates `system`.
pub fn update_system() -> CoreResult<()> {
    let output = Command::new("pkexec")
        .args(["pacman", "-Syu", "--noconfirm"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string().into())
    }
}

/// Executes a privileged command and streams stdout/stderr lines via channel.
pub fn execute_cmd_with_log_stream(
    args: &[&str],
    password: Option<&str>,
    sender: std::sync::mpsc::Sender<String>,
) -> CoreResult<()> {
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
                if n == 0 {
                    break;
                }
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
                if n == 0 {
                    break;
                }
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
        Err(format!("Process exited with status: {}", status).into())
    }
}
