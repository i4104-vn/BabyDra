//! System update service.

use crate::models::system_update::PackageUpdate;
use std::process::Command;

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

/// Triggers system update in a terminal emulator.
pub fn update_system() -> Result<(), String> {
    let terminals = ["foot", "kitty", "alacritty", "wezterm", "gnome-terminal", "konsole", "xfce4-terminal"];
    let update_cmd = "sudo pacman -Syu";

    for term in terminals {
        let mut cmd = Command::new(term);

        match term {
            "foot" => {
                cmd.args(&["--title", "update-system", "sh", "-c", &format!("{}; echo; read -p 'Press Enter to close...' -n 1", update_cmd)]);
            }
            "gnome-terminal" | "xfce4-terminal" => {
                cmd.args(&["--title", "update-system", "--", "bash", "-c", &format!("{}; echo; read -p 'Press Enter to close...' -n 1", update_cmd)]);
            }
            "konsole" => {
                cmd.args(&["-p", "tabtitle=update-system", "-e", "bash", "-c", &format!("{}; echo; read -p 'Press Enter to close...' -n 1", update_cmd)]);
            }
            "kitty" | "alacritty" => {
                cmd.args(&["--title", "update-system", "-e", "sh", "-c", &format!("{}; echo; read -p 'Press Enter to close...' -n 1", update_cmd)]);
            }
            "wezterm" => {
                cmd.args(&["-e", "sh", "-c", &format!("{}; echo; read -p 'Press Enter to close...' -n 1", update_cmd)]);
            }
            _ => continue,
        }

        if let Ok(mut child) = cmd.spawn() {
            let _ = child.wait();
            return Ok(());
        }
    }

    Err("No supported terminal emulator found".to_string())
}

/// Executes a privileged command and streams stdout/stderr lines via channel.
pub fn execute_cmd_with_log_stream(args: &[&str], password: Option<&str>, sender: std::sync::mpsc::Sender<String>) -> Result<(), String> {
    use std::io::{BufRead, BufReader, Write};
    use std::process::Stdio;

    let mut command = if password.is_some() {
        let mut c = Command::new("sudo");
        c.arg("-S");
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
        }
    }

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let sender_stdout = sender.clone();
    let thread_stdout = std::thread::spawn(move || {
        if let Some(out) = stdout {
            let reader = BufReader::new(out);
            for line in reader.lines() {
                if let Ok(l) = line {
                    let _ = sender_stdout.send(l);
                }
            }
        }
    });

    let sender_stderr = sender;
    let thread_stderr = std::thread::spawn(move || {
        if let Some(err) = stderr {
            let reader = BufReader::new(err);
            for line in reader.lines() {
                if let Ok(l) = line {
                    let _ = sender_stderr.send(l);
                }
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

pub fn is_update_in_progress() -> bool {
    if is_pacman_running() {
        return true;
    }
    let path = get_update_log_path();
    if path.exists() {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if !content.is_empty()
                && !content.contains("System update completed successfully.")
                && !content.contains("Error:")
            {
                if let Ok(meta) = std::fs::metadata(&path) {
                    if let Ok(modified) = meta.modified() {
                        if let Ok(elapsed) = modified.elapsed() {
                            return elapsed.as_secs() < 30;
                        }
                    }
                }
            }
        }
    }
    false
}

pub fn read_update_log() -> String {
    let path = get_update_log_path();
    std::fs::read_to_string(path).unwrap_or_default()
}

pub fn start_background_update(password: Option<String>) {
    use std::fs::OpenOptions;
    use std::io::Write;

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
            if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_path) {
                let _ = writeln!(file, "{}", line);
                let _ = file.flush();
            }
        }
    });
}

/// Triggers system update streaming output via sender channel.
pub fn stream_update_system(password: Option<&str>, sender: std::sync::mpsc::Sender<String>) -> Result<(), String> {
    if std::path::Path::new("/var/lib/pacman/db.lck").exists() {
        if !is_pacman_running() {
            let _ = sender.send(":: Detected stale pacman lock file (/var/lib/pacman/db.lck). Cleaning lock file...".to_string());
            if let Some(pwd) = password {
                let _ = execute_cmd_with_log_stream(&["rm", "-f", "/var/lib/pacman/db.lck"], Some(pwd), sender.clone());
            } else {
                let _ = std::fs::remove_file("/var/lib/pacman/db.lck");
            }
        }
    }
    execute_cmd_with_log_stream(&["pacman", "-Syu", "--noconfirm"], password, sender)
}
