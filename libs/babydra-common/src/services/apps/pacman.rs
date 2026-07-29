//! Pacman explicitly installed package resolution and dependency heuristics.

use crate::models::app_info::InstalledPackage;
use std::path::Path;

pub fn get_explicitly_installed_packages() -> std::collections::HashSet<String> {
    let mut set = std::collections::HashSet::new();
    if let Ok(output) = std::process::Command::new("pacman").args(&["-Qqe"]).output() {
        if output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                set.insert(line.trim().to_string());
            }
        }
    }
    set
}

pub fn get_installed_packages_list() -> Vec<InstalledPackage> {
    let mut pkgs = Vec::new();
    if let Ok(output) = std::process::Command::new("pacman").args(&["-Qe"]).output() {
        if output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    pkgs.push(InstalledPackage {
                        name: parts[0].to_string(),
                        version: parts[1].to_string(),
                    });
                }
            }
        }
    }
    pkgs
}

pub fn get_package_owner(path: &Path) -> Option<String> {
    let output = std::process::Command::new("pacman")
        .args(&["-Qqo", path.to_str()?])
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

pub fn is_dependency_heuristic(filename: &str, _name: &str, exec: &str) -> bool {
    let filename_lower = filename.to_lowercase();
    let exec_lower = exec.to_lowercase();

    let known_deps = [
        "avahi-discover", "bssh", "bvnc", "qv4l2", "qvidcap", 
        "gcr-prompter", "gcr-viewer", "xdg-desktop-portal",
        "footclient", "foot-server", "kitty-open", "ktelnetservice",
        "pinentry", "xwayland", "fcitx5-wayland-launcher"
    ];

    for dep in &known_deps {
        if filename_lower.contains(dep) || exec_lower.contains(dep) {
            return true;
        }
    }

    if filename_lower.starts_with("kcm_") || filename_lower.starts_with("org.kde.kiod") || filename_lower.starts_with("org.kde.knewstuff") || filename_lower.starts_with("org.kde.ksecretd") {
        return true;
    }

    if filename_lower.contains("geo-handler") {
        return true;
    }

    if filename_lower.starts_with("org.fcitx.fcitx5-") && !filename_lower.contains("config") {
        return true;
    }

    if filename_lower.starts_with("fcitx5-") && !filename_lower.contains("config") {
        return true;
    }

    false
}

pub fn uninstall_package(name: &str) -> Result<(), String> {
    let output = std::process::Command::new("pkexec")
        .args(["pacman", "-R", "--noconfirm", name])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn uninstall_app_by_path(full_path: &str) -> Result<(), String> {
    let path = Path::new(full_path);
    if let Some(pkg) = get_package_owner(path) {
        uninstall_package(&pkg)
    } else {
        Err("Could not find package owner for app".to_string())
    }
}

pub fn stream_uninstall_package(pkg_name: &str, password: Option<&str>, sender: std::sync::mpsc::Sender<String>) -> Result<(), String> {
    let cmd = format!("yes | pacman -R --noconfirm {}", pkg_name);
    crate::services::system::updates::execute_cmd_with_log_stream(&["sh", "-c", &cmd], password, sender)
}

