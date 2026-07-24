//! System update service.

use crate::models::system_update::PackageUpdate;
use std::process::Command;

/// Checks for pending system updates.
pub fn check_updates() -> Result<Vec<PackageUpdate>, String> {
    let output = Command::new("checkupdates")
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut updates = Vec::new();

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            updates.push(PackageUpdate {
                name: parts[0].to_string(),
                old_version: parts[1].to_string(),
                new_version: parts[3].to_string(),
            });
        }
    }

    Ok(updates)
}

/// Triggers system update in a terminal emulator.
pub fn update_system() -> Result<(), String> {
    let terminals = ["kitty", "alacritty", "wezterm", "gnome-terminal", "konsole", "xfce4-terminal"];
    let update_cmd = "sudo pacman -Syu";

    for term in terminals {
        let mut cmd = Command::new(term);

        match term {
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
