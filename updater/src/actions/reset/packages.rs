use crate::actions::runner::CommandRunner;
use crate::core::state::ResetMode;
use std::process::Command;

pub fn clean_packages(
    runner: &CommandRunner,
    mode: ResetMode,
    is_dry_run: bool,
) -> Result<(), String> {
    runner.step("[Step 5/6] Cleaning packages...");

    if is_dry_run {
        runner.log("[dry-run] Would uninstall packages according to reset mode");
        return Ok(());
    }

    match mode {
        ResetMode::RemoveShellPackages => {
            let shell_pkgs = [
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
            let mut pkgs_to_remove = Vec::new();
            for pkg in &shell_pkgs {
                if runner.run_cmd("pacman", &["-Q", pkg], None).is_ok() {
                    pkgs_to_remove.push(*pkg);
                }
            }
            if !pkgs_to_remove.is_empty() {
                let mut args = vec!["-Rns", "--noconfirm"];
                args.extend_from_slice(&pkgs_to_remove);
                runner.run_sudo("pacman", &args, None)?;
            }
        }
        ResetMode::RemoveAllApps => {
            runner.log("Scanning non-base packages to revert to pure Arch baseline...");
            const PROTECTED: &[&str] = &[
                "base", "base-devel", "linux", "linux-lts", "linux-zen", "linux-hardened",
                "linux-firmware", "intel-ucode", "amd-ucode", "btrfs-progs", "e2fsprogs",
                "dosfstools", "efibootmgr", "grub", "systemd", "systemd-sysvcompat",
                "sudo", "networkmanager", "iwd", "dhcpcd", "iproute2", "git", "bash",
                "zsh", "coreutils", "util-linux", "pacman", "archlinux-keyring",
            ];

            let explicit_output = Command::new("pacman")
                .args(["-Qeq"])
                .output()
                .map_err(|e| format!("Failed to query explicit packages: {}", e))?;

            if explicit_output.status.success() {
                let package_list = String::from_utf8_lossy(&explicit_output.stdout);
                let pkgs_to_remove: Vec<&str> = package_list
                    .lines()
                    .map(str::trim)
                    .filter(|p| !p.is_empty() && !PROTECTED.contains(p))
                    .collect();

                if !pkgs_to_remove.is_empty() {
                    runner.log(format!(
                        "Uninstalling user packages ({} packages)...",
                        pkgs_to_remove.len()
                    ));
                    let mut args = vec!["-Rns", "--noconfirm"];
                    args.extend_from_slice(&pkgs_to_remove);
                    if runner.run_sudo("pacman", &args, None).is_err() {
                        runner.log("Retrying with -R --noconfirm...");
                        let mut fallback_args = vec!["-R", "--noconfirm"];
                        fallback_args.extend_from_slice(&pkgs_to_remove);
                        let _ = runner.run_sudo("pacman", &fallback_args, None);
                    }
                } else {
                    runner.log("No user packages to remove; system already at baseline.");
                }
            }

            runner.log("Cleaning orphan packages...");
            let orphan_output = Command::new("pacman").args(["-Qdtq"]).output();
            if let Ok(output) = orphan_output {
                if output.status.success() {
                    let orphans = String::from_utf8_lossy(&output.stdout);
                    let orphan_list: Vec<&str> = orphans
                        .lines()
                        .map(str::trim)
                        .filter(|p| !p.is_empty())
                        .collect();

                    if !orphan_list.is_empty() {
                        let mut args = vec!["-Rns", "--noconfirm"];
                        args.extend(orphan_list);
                        let _ = runner.run_sudo("pacman", &args, None);
                    } else {
                        runner.log("No orphan packages found.");
                    }
                }
            }
        }
        _ => {
            runner.log("Skipping package uninstallation as requested.");
        }
    }

    Ok(())
}
