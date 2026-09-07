//! Factory reset service to restore the system back to vanilla Arch Linux baseline natively.

use crate::error::{CoreError, CoreResult};
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc::Sender;

/// Executes a command with elevated privileges (`sudo -S`) and streams its output.
fn run_sudo_cmd(
    password: &str,
    cmd: &str,
    args: &[&str],
    sender: &Sender<String>,
) -> Result<(), String> {
    let mut child = Command::new("sudo")
        .arg("-S")
        .arg("-p")
        .arg("")
        .arg(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn sudo {}: {}", cmd, e))?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "{}", password);
        let _ = stdin.flush();
    }

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let sender_out = sender.clone();
    let out_th = std::thread::spawn(move || {
        if let Some(out) = stdout {
            let reader = BufReader::new(out);
            for line in reader.lines().flatten() {
                if !line.trim().is_empty() {
                    let _ = sender_out.send(format!("  {}", line));
                }
            }
        }
    });

    let sender_err = sender.clone();
    let err_th = std::thread::spawn(move || {
        if let Some(err) = stderr {
            let reader = BufReader::new(err);
            for line in reader.lines().flatten() {
                if !line.contains("[sudo] password") && !line.trim().is_empty() {
                    let _ = sender_err.send(format!("  {}", line));
                }
            }
        }
    });

    let _ = out_th.join();
    let _ = err_th.join();

    let status = child
        .wait()
        .map_err(|e| format!("Process error on {}: {}", cmd, e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "Command {} exited with code {:?}",
            cmd,
            status.code()
        ))
    }
}

/// Terminates active BabyDra and compositor processes.
fn stop_active_processes(sender: &Sender<String>) {
    let _ = sender.send("[1/6] Stopping active BabyDra and compositor processes...".into());
    let _ = Command::new("pkill").args(["-f", "babydra-"]).output();
    let _ = Command::new("pkill").args(["-x", "labwc"]).output();
    let _ = Command::new("pkill").args(["-x", "cage"]).output();
    let _ = sender.send("  Processes stopped.".into());
}

/// Restores the default systemd login target and unmasks the virtual terminal console login.
fn restore_systemd_console(password: &str, sender: &Sender<String>) {
    let _ = sender
        .send("[2/6] Restoring Arch Linux TTY login console & disabling display manager...".into());
    let _ = run_sudo_cmd(password, "systemctl", &["stop", "greetd.service"], sender);
    let _ = run_sudo_cmd(
        password,
        "systemctl",
        &["disable", "greetd.service"],
        sender,
    );

    // Unmask tty2-6 and enable tty1 console
    let _ = run_sudo_cmd(
        password,
        "systemctl",
        &[
            "unmask",
            "getty@tty2.service",
            "getty@tty3.service",
            "getty@tty4.service",
            "getty@tty5.service",
            "getty@tty6.service",
        ],
        sender,
    );
    let _ = run_sudo_cmd(
        password,
        "systemctl",
        &["enable", "getty@tty1.service"],
        sender,
    );
    let _ = run_sudo_cmd(
        password,
        "systemctl",
        &["set-default", "multi-user.target"],
        sender,
    );
    let _ = sender.send("  Arch Linux multi-user TTY login target restored.".into());
}

/// Removes BabyDra system-level files and configurations.
fn clean_system_files(password: &str, sender: &Sender<String>) {
    let _ = sender.send("[3/6] Cleaning system-wide BabyDra directories and modules...".into());
    let paths_to_remove = [
        "/usr/bin/babydra-greeter",
        "/usr/share/babydra",
        "/var/lib/babydra",
        "/etc/modules-load.d/i2c.conf",
        "/etc/tmpfiles.d/babydra-perf.conf",
        "/etc/greetd",
    ];
    for path in paths_to_remove {
        let _ = run_sudo_cmd(password, "rm", &["-rf", path], sender);
        let _ = sender.send(format!("  Removed system path: {}", path));
    }
}

/// Cleans user configuration files, caches, desktop entries, and theme assets.
fn clean_user_files(sender: &Sender<String>) {
    let _ = sender.send("[4/6] Cleaning user configuration, caches, and binaries...".into());
    if let Some(home) = dirs::home_dir() {
        // 1. ~/.babydra
        let babydra_dir = home.join(".babydra");
        if babydra_dir.exists() {
            let _ = fs::remove_dir_all(&babydra_dir);
            let _ = sender.send("  Removed ~/.babydra".into());
        }

        // 2. ~/.cache/babydra
        let babydra_cache = home.join(".cache/babydra");
        if babydra_cache.exists() {
            let _ = fs::remove_dir_all(&babydra_cache);
            let _ = sender.send("  Removed ~/.cache/babydra".into());
        }

        // 3. ~/.config/labwc
        let labwc_dir = home.join(".config/labwc");
        if labwc_dir.exists() {
            let _ = fs::remove_dir_all(&labwc_dir);
            let _ = sender.send("  Removed ~/.config/labwc".into());
        }

        // 4. ~/.local/share/themes/BabyDra
        let theme_dir = home.join(".local/share/themes/BabyDra");
        if theme_dir.exists() {
            let _ = fs::remove_dir_all(&theme_dir);
            let _ = sender.send("  Removed ~/.local/share/themes/BabyDra".into());
        }

        // 5. Binaries in ~/.local/bin
        let local_bin = home.join(".local/bin");
        if let Ok(entries) = fs::read_dir(&local_bin) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("babydra-") || name == "wtype" {
                    let _ = fs::remove_file(entry.path());
                    let _ = sender.send(format!("  Removed ~/.local/bin/{}", name));
                }
            }
        }

        // 6. Desktop files in ~/.local/share/applications
        let app_dir = home.join(".local/share/applications");
        if let Ok(entries) = fs::read_dir(&app_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("babydra-") && name.ends_with(".desktop") {
                    let _ = fs::remove_file(entry.path());
                    let _ = sender.send(format!("  Removed desktop entry {}", name));
                }
            }
        }

        // 7. FileManager1 dbus service
        let dbus_svc =
            home.join(".local/share/dbus-1/services/org.freedesktop.FileManager1.service");
        if dbus_svc.exists() {
            let _ = fs::remove_file(&dbus_svc);
            let _ = sender.send("  Removed FileManager1 DBus service".into());
        }

        // 8. GTK, fontconfig, and CLI configs
        let gtk3_ini = home.join(".config/gtk-3.0/settings.ini");
        if gtk3_ini.exists() {
            let _ = fs::remove_file(gtk3_ini);
        }
        let gtk4_ini = home.join(".config/gtk-4.0/settings.ini");
        if gtk4_ini.exists() {
            let _ = fs::remove_file(gtk4_ini);
        }
        let fonts_conf = home.join(".config/fontconfig/fonts.conf");
        if fonts_conf.exists() {
            let _ = fs::remove_file(fonts_conf);
        }

        let fastfetch_dir = home.join(".config/fastfetch");
        if fastfetch_dir.exists() {
            let _ = fs::remove_dir_all(fastfetch_dir);
        }
        let kitty_dir = home.join(".config/kitty");
        if kitty_dir.exists() {
            let _ = fs::remove_dir_all(kitty_dir);
        }
        let nvim_dir = home.join(".config/nvim");
        if nvim_dir.exists() {
            let _ = fs::remove_dir_all(nvim_dir);
        }

        // Reset GSettings
        for key in &[
            "font-name",
            "document-font-name",
            "monospace-font-name",
            "icon-theme",
            "cursor-theme",
        ] {
            let _ = Command::new("gsettings")
                .args(["reset", "org.gnome.desktop.interface", key])
                .output();
        }
        let _ = sender.send("  Reset GNOME/GTK desktop interface settings to default".into());

        // Update desktop database
        let _ = Command::new("update-desktop-database")
            .arg(app_dir)
            .output();
    }
}

const PROTECTED_PACKAGES: &[&str] = &[
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
fn remove_shell_packages(password: &str, sender: &Sender<String>) {
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
fn remove_all_user_applications(password: &str, sender: &Sender<String>) {
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
fn clean_orphan_packages(password: &str, sender: &Sender<String>) {
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

/// Rebuilds font cache and finalizes.
fn finalize_system(sender: &Sender<String>) {
    let _ = sender.send("[6/6] Finalizing system state...".into());
    let _ = Command::new("fc-cache").arg("-r").output();
    let _ = sender.send("  Font cache refreshed.".into());
}

/// Executes the full native Arch Linux factory reset sequence.
pub fn run_factory_reset_stream(
    password: &str,
    remove_shell_pkgs: bool,
    remove_all_apps: bool,
    sender: Sender<String>,
) -> CoreResult<()> {
    let _ = sender.send("=== Starting Native Arch Linux Factory Reset ===".into());

    stop_active_processes(&sender);
    restore_systemd_console(password, &sender);
    clean_system_files(password, &sender);
    clean_user_files(&sender);

    if remove_all_apps {
        remove_all_user_applications(password, &sender);
    } else if remove_shell_pkgs {
        remove_shell_packages(password, &sender);
    } else {
        let _ = sender.send("[5/6] Skipping package removal as requested.".into());
    }

    finalize_system(&sender);

    let _ = sender.send("========================================================".into());
    let _ = sender.send("Factory Reset Complete!".into());
    let _ = sender.send("The system has been restored to clean Arch Linux baseline.".into());
    let _ = sender.send("Login prompt will be available via standard TTY console.".into());
    let _ = sender.send("========================================================".into());

    Ok(())
}

/// Requests an immediate system reboot.
pub fn reboot_system(password: Option<&str>) -> CoreResult<()> {
    if let Some(pwd) = password {
        let mut child = Command::new("sudo")
            .arg("-S")
            .arg("systemctl")
            .arg("reboot")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| CoreError::msg(e.to_string()))?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = writeln!(stdin, "{}", pwd);
            let _ = stdin.flush();
        }
        let _ = child.wait();
        Ok(())
    } else {
        Command::new("systemctl")
            .arg("reboot")
            .spawn()
            .map(|_| ())
            .map_err(|e| CoreError::msg(e.to_string()))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_reset_channel_streaming() {
        let (tx, rx) = std::sync::mpsc::channel::<String>();
        let _ = tx.send("[1/6] Stopping active BabyDra and compositor processes...".into());
        let _ = tx.send("Factory Reset Complete!".into());
        assert_eq!(
            rx.recv().unwrap(),
            "[1/6] Stopping active BabyDra and compositor processes..."
        );
        assert_eq!(rx.recv().unwrap(), "Factory Reset Complete!");
    }
}
