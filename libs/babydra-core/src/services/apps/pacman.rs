//! Pacman explicitly installed package resolution and dependency heuristics.

use crate::error::CoreResult;
use crate::models::app_info::InstalledPackage;
use std::path::Path;

/// Returns the set of packages explicitly installed by the user (`pacman -Qqe`).
pub fn get_explicit_pkgs() -> std::collections::HashSet<String> {
    let mut set = std::collections::HashSet::new();
    if let Ok(output) = std::process::Command::new("pacman")
        .args(&["-Qqe"])
        .output()
    {
        if output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                set.insert(line.trim().to_string());
            }
        }
    }
    set
}

/// Returns the current `installed packages list`.
pub fn get_installed_pkgs() -> Vec<InstalledPackage> {
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

/// Returns the current `package owner`.
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

/// Returns `true` when `dependency heuristic` holds, `false` otherwise.
pub fn is_dep_heuristic(filename: &str, _name: &str, exec: &str) -> bool {
    let filename_lower = filename.to_lowercase();
    let exec_lower = exec.to_lowercase();

    let known_deps = [
        "avahi-discover",
        "bssh",
        "bvnc",
        "qv4l2",
        "qvidcap",
        "gcr-prompter",
        "gcr-viewer",
        "xdg-desktop-portal",
        "footclient",
        "foot-server",
        "kitty-open",
        "ktelnetservice",
        "pinentry",
        "xwayland",
        "fcitx5-wayland-launcher",
    ];

    for dep in &known_deps {
        if filename_lower.contains(dep) || exec_lower.contains(dep) {
            return true;
        }
    }

    if filename_lower.starts_with("kcm_")
        || filename_lower.starts_with("org.kde.kiod")
        || filename_lower.starts_with("org.kde.knewstuff")
        || filename_lower.starts_with("org.kde.ksecretd")
    {
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

/// Uninstalls a package by name.
pub fn uninstall_package(name: &str) -> CoreResult<()> {
    let output = std::process::Command::new("pkexec")
        .args(["pacman", "-R", "--noconfirm", name])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string().into())
    }
}

/// Uninstall app by path.
pub fn uninstall_app(full_path: &str) -> CoreResult<()> {
    let path = Path::new(full_path);
    if let Some(pkg) = get_package_owner(path) {
        uninstall_package(&pkg)
    } else {
        Err("Could not find package owner for app".into())
    }
}

/// Stream uninstall package.
pub fn stream_uninstall(
    pkg_name: &str,
    password: Option<&str>,
    sender: std::sync::mpsc::Sender<String>,
) -> CoreResult<()> {
    crate::services::system::updates::clean_pacman_lock(password, sender.clone());
    let cmd = format!("yes | pacman -Rns --noconfirm {}", pkg_name.trim());
    crate::services::system::updates::exec_cmd_stream(
        &["sh", "-c", &cmd],
        password,
        sender,
    )
}

/// Find cached older package.
pub fn find_cached_pkg(pkg_name: &str) -> Option<std::path::PathBuf> {
    let cache_dir = Path::new("/var/cache/pacman/pkg");
    if !cache_dir.exists() {
        return None;
    }

    let current_ver = std::process::Command::new("pacman")
        .args(&["-Q", pkg_name])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                let out = String::from_utf8_lossy(&o.stdout);
                let parts: Vec<&str> = out.split_whitespace().collect();
                if parts.len() >= 2 {
                    Some(parts[1].to_string())
                } else {
                    None
                }
            } else {
                None
            }
        });

    let mut matches = Vec::new();
    if let Ok(entries) = std::fs::read_dir(cache_dir) {
        let prefix = format!("{}-", pkg_name);
        for entry in entries.flatten() {
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with(&prefix) && filename.ends_with(".pkg.tar.zst") {
                matches.push((entry.path(), filename));
            }
        }
    }

    if matches.is_empty() {
        return None;
    }

    matches.sort_by_key(|(path, _)| std::fs::metadata(path).and_then(|m| m.modified()).ok());
    matches.reverse();

    if let Some(ref cur) = current_ver {
        for (path, filename) in &matches {
            if !filename.contains(cur) {
                return Some(path.clone());
            }
        }
    }

    matches.first().map(|(p, _)| p.clone())
}

/// Stream downgrade package.
pub fn stream_downgrade(
    pkg_name: &str,
    password: Option<&str>,
    sender: std::sync::mpsc::Sender<String>,
) -> CoreResult<()> {
    crate::services::system::updates::clean_pacman_lock(password, sender.clone());
    if let Some(cached_file) = find_cached_pkg(pkg_name) {
        let path_str = cached_file.to_string_lossy().to_string();
        let cmd = format!("pacman -U --noconfirm {}", path_str);
        crate::services::system::updates::exec_cmd_stream(
            &["sh", "-c", &cmd],
            password,
            sender,
        )
    } else {
        Err(format!(
            "No cached older version found for '{}' in /var/cache/pacman/pkg/",
            pkg_name
        )
        .into())
    }
}
