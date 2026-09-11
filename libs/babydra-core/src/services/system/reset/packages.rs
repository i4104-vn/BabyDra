//! Pacman package cleanup and removal logic for factory reset.

use crate::services::system::reset::system::run_sudo_cmd;
use std::process::Command;
use std::sync::mpsc::Sender;

pub const PROTECTED_PACKAGES: &[&str] = &[
    "base",
    "base-devel",
    "linux",
    "linux-lts",
    "linux-zen",
    "linux-hardened",
    "linux-firmware",
    "intel-ucode",
    "amd-ucode",
    "btrfs-progs",
    "e2fsprogs",
    "dosfstools",
    "efibootmgr",
    "grub",
    "systemd",
    "systemd-sysvcompat",
    "sudo",
    "networkmanager",
    "iwd",
    "dhcpcd",
    "iproute2",
    "git",
    "bash",
    "zsh",
    "coreutils",
    "util-linux",
    "pacman",
    "archlinux-keyring",
];

/// Uninstalls BabyDra shell packages via pacman if requested.
pub fn remove_shell_packages(password: &str, sender: &Sender<String>) {
    let _ = sender.send("[5/6] Cleaning BabyDra shell packages...".into());
    let candidate_pkgs = [
        "labwc",
        "greetd",
        "cage",
        "gtk4-layer-shell",
        "wlrctl",
        "ddcutil-service",
        "gammastep",
        "wlsunset",
        "kvantum-qt5",
    ];

    let mut to_remove = Vec::new();
    for pkg in candidate_pkgs {
        if let Ok(output) = Command::new("pacman").args(["-Q", pkg]).output() {
            if output.status.success() {
                to_remove.push(pkg);
            }
        }
    }

    if !to_remove.is_empty() {
        let _ = sender.send(format!(
            "  Uninstalling shell packages: {}",
            to_remove.join(", ")
        ));
        let mut pacman_args = vec!["-Rns", "--noconfirm"];
        pacman_args.extend_from_slice(&to_remove);
        let _ = run_sudo_cmd(password, "pacman", &pacman_args, sender);
    } else {
        let _ = sender.send("  No BabyDra-specific packages needed removal.".into());
    }

    clean_orphan_packages(password, sender);
}

/// Uninstalls ALL user-installed applications, keeping only Arch Linux base system packages.
pub fn remove_all_user_applications(password: &str, sender: &Sender<String>) {
    let _ = sender.send(
        "[5/6] Scanning and removing ALL user-installed applications (keeping Arch Linux base)..."
            .into(),
    );

    let output = match Command::new("pacman").args(["-Qeq"]).output() {
        Ok(out) => out,
        Err(e) => {
            let _ = sender.send(format!("  Failed to query installed packages: {}", e));
            return;
        }
    };

    let installed_str = String::from_utf8_lossy(&output.stdout);
    let mut pkgs_to_remove: Vec<String> = Vec::new();

    for line in installed_str.lines() {
        let pkg = line.trim();
        if pkg.is_empty() {
            continue;
        }
        if !PROTECTED_PACKAGES.contains(&pkg) {
            pkgs_to_remove.push(pkg.to_string());
        }
    }

    if pkgs_to_remove.is_empty() {
        let _ = sender.send("  No user packages to remove; system already at baseline.".into());
        return;
    }

    let _ = sender.send(format!(
        "  Found {} user-installed packages to uninstall: {}",
        pkgs_to_remove.len(),
        pkgs_to_remove.join(", ")
    ));

    let slice_refs: Vec<&str> = pkgs_to_remove.iter().map(|s| s.as_str()).collect();
    let mut pacman_args = vec!["-Rns", "--noconfirm"];
    pacman_args.extend_from_slice(&slice_refs);

    if let Err(e) = run_sudo_cmd(password, "pacman", &pacman_args, sender) {
        let _ = sender.send(format!(
            "  Batch removal returned notice ({}). Retrying individual packages...",
            e
        ));
        for pkg in &slice_refs {
            let _ = run_sudo_cmd(password, "pacman", &["-R", "--noconfirm", pkg], sender);
        }
    }

    clean_orphan_packages(password, sender);
}

/// Cleans orphaned dependencies left behind by uninstalled packages.
pub fn clean_orphan_packages(password: &str, sender: &Sender<String>) {
    if let Ok(output) = Command::new("pacman").args(["-Qtdq"]).output() {
        if output.status.success() {
            let orphans_str = String::from_utf8_lossy(&output.stdout);
            let orphans: Vec<&str> = orphans_str.split_whitespace().collect();
            if !orphans.is_empty() {
                let _ = sender.send(format!(
                    "  Cleaning orphan packages: {}",
                    orphans.join(", ")
                ));
                let mut pacman_args = vec!["-Rns", "--noconfirm"];
                pacman_args.extend_from_slice(&orphans);
                let _ = run_sudo_cmd(password, "pacman", &pacman_args, sender);
            }
        }
    }
}
