use crate::models::{GenericOptionItem, LogLevel};
use crate::system::{tail_lines, SudoSession};
use std::process::Command;

pub fn execute_packages_task<F>(
    opt: &GenericOptionItem,
    sudo: &SudoSession,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let mut errors = 0;

    match opt.id.as_str() {
        "pacman_packages" => {
            log(
                LogLevel::Info,
                "Running pacman -Syu for system dependencies...".into(),
            );
            let pkgs = [
                "base-devel",
                "git",
                "pkgconf",
                "gtk4",
                "gtk4-layer-shell",
                "rust",
                "labwc",
                "meson",
                "ninja",
                "playerctl",
                "grim",
                "slurp",
                "wl-clipboard",
                "libnotify",
                "gammastep",
                "wlsunset",
                "wireplumber",
                "pipewire-pulse",
                "pipewire-alsa",
                "ddcutil",
                "zip",
                "unzip",
                "p7zip",
                "unrar",
                "pacman-contrib",
                "xdg-utils",
                "polkit",
                "networkmanager",
                "networkmanager-openvpn",
                "networkmanager-vpnc",
                "networkmanager-pptp",
                "networkmanager-l2tp",
                "networkmanager-openconnect",
                "networkmanager-strongswan",
                "wireguard-tools",
                "openvpn",
                "bluez",
                "bluez-utils",
                "greetd",
                "cage",
            ];

            // Root: run pacman directly. Non-root: `sudo -S pacman` with the
            // password fed via piped stdin (no TTY prompt, no TUI breakage).
            let mut args: Vec<&str> = vec!["pacman", "-Syu", "--needed", "--noconfirm"];
            args.extend_from_slice(&pkgs);
            let out = sudo.run_root(&args);

            match out {
                Ok(o) => {
                    for line in tail_lines(&o.stdout, 5) {
                        log(LogLevel::Info, line);
                    }
                    if o.success {
                        log(
                            LogLevel::Success,
                            "Arch Linux pacman packages installed/updated.".into(),
                        );
                        copied += 1;
                    } else {
                        log(
                            LogLevel::Error,
                            format!("Pacman exited with error: {}", o.stderr.trim()),
                        );
                        errors += 1;
                    }
                }
                Err(e) => {
                    log(LogLevel::Error, format!("Failed to run pacman: {e}"));
                    errors += 1;
                }
            }
        }

        "install_yay" => {
            let yay_exists = Command::new("which")
                .arg("yay")
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false);

            if yay_exists {
                log(
                    LogLevel::Success,
                    "yay AUR helper is already installed.".into(),
                );
                copied += 1;
            } else {
                log(
                    LogLevel::Info,
                    "yay not found, building yay-bin from AUR (/tmp/yay-bin)...".into(),
                );
                let _ = std::fs::remove_dir_all("/tmp/yay-bin");
                let clone_res = Command::new("git")
                    .args(["clone", "https://aur.archlinux.org/yay-bin.git", "/tmp/yay-bin"])
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .output();

                match clone_res {
                    Ok(o) if o.status.success() => {
                        let _ = sudo.preauth();
                        let build_res = Command::new("makepkg")
                            .args(["-si", "--noconfirm"])
                            .current_dir("/tmp/yay-bin")
                            .stdin(std::process::Stdio::null())
                            .stdout(std::process::Stdio::piped())
                            .stderr(std::process::Stdio::piped())
                            .output();

                        match build_res {
                            Ok(bo) => {
                                let stdout = String::from_utf8_lossy(&bo.stdout);
                                let stderr = String::from_utf8_lossy(&bo.stderr);
                                for line in tail_lines(&stdout, 5) {
                                    log(LogLevel::Info, line);
                                }
                                if bo.status.success() {
                                    log(LogLevel::Success, "yay-bin installed successfully.".into());
                                    copied += 1;
                                } else {
                                    log(
                                        LogLevel::Error,
                                        format!("makepkg failed for yay-bin: {}", stderr.trim()),
                                    );
                                    errors += 1;
                                }
                            }
                            Err(e) => {
                                log(LogLevel::Error, format!("Failed to run makepkg for yay-bin: {e}"));
                                errors += 1;
                            }
                        }
                    }
                    Ok(o) => {
                        log(
                            LogLevel::Error,
                            format!(
                                "Failed to clone yay-bin repo: {}",
                                String::from_utf8_lossy(&o.stderr).trim()
                            ),
                        );
                        errors += 1;
                    }
                    Err(e) => {
                        log(LogLevel::Error, format!("git clone error for yay-bin: {e}"));
                        errors += 1;
                    }
                }
            }
        }

        "aur_packages" => {
            log(LogLevel::Info, "Installing AUR packages via yay...".into());
            let aur_pkgs = [
                "github-desktop",
                "fastfetch",
                "neovim",
                "awww",
                "ddcutil-service",
                "kitty",
                "ttf-segoe-ui-variable",
                "ttf-cascadia-code-nerd",
                "inter-font",
                "ttf-ubuntu-font-family",
                "ttf-jetbrains-mono-nerd",
                "ttf-nerd-fonts-symbols",
                "ttf-nerd-fonts-symbols-mono",
                "otf-font-awesome",
                "ttf-font-awesome",
                "noto-fonts",
                "noto-fonts-cjk",
                "noto-fonts-emoji",
                "noto-fonts-extra",
                "ttf-liberation",
                "papirus-icon-theme",
                "kvantum-qt5",
                "wlrctl",
            ];

            // yay internally calls sudo during makepkg. Re-validate the cached
            // credential first: long AUR builds can exceed the sudo timestamp
            // timeout, which would make yay's internal sudo fail silently.
            if let Err(e) = sudo.preauth() {
                log(
                    LogLevel::Error,
                    format!("Sudo credential expired before AUR install: {e}"),
                );
                errors += 1;
                return (copied, errors);
            }

            // Output is captured so nothing hits the TUI.
            let mut cmd = Command::new("yay");
            cmd.args(["-S", "--noconfirm", "--needed"]);
            cmd.args(aur_pkgs);
            let out = cmd
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .output();

            match out {
                Ok(o) => {
                    let stdout = String::from_utf8_lossy(&o.stdout);
                    let stderr = String::from_utf8_lossy(&o.stderr);
                    for line in tail_lines(&stdout, 5) {
                        log(LogLevel::Info, line);
                    }
                    if o.status.success() {
                        log(
                            LogLevel::Success,
                            "AUR packages installed successfully.".into(),
                        );
                        copied += 1;
                    } else {
                        log(
                            LogLevel::Warn,
                            format!("yay completed with errors: {}", stderr.trim()),
                        );
                        errors += 1;
                    }
                }
                Err(e) => {
                    log(
                        LogLevel::Warn,
                        format!("yay not found or failed to execute: {e}"),
                    );
                    errors += 1;
                }
            }
        }

        "build_wtype" => {
            let local_bin = crate::system::get_user_local_bin();
            let wtype_dst = local_bin.join("wtype");

            if wtype_dst.is_file() {
                log(
                    LogLevel::Success,
                    format!("wtype binary already exists at {:?}", wtype_dst),
                );
                copied += 1;
            } else {
                log(
                    LogLevel::Info,
                    "wtype not found, compiling from source with meson + ninja...".into(),
                );
                let _ = std::fs::create_dir_all(&local_bin);
                let _ = std::fs::remove_dir_all("/tmp/wtype");

                let clone_res = Command::new("git")
                    .args(["clone", "https://github.com/atx/wtype.git", "/tmp/wtype"])
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .output();

                match clone_res {
                    Ok(o) if o.status.success() => {
                        let setup_res = Command::new("meson")
                            .args(["setup", "build"])
                            .current_dir("/tmp/wtype")
                            .stdin(std::process::Stdio::null())
                            .stdout(std::process::Stdio::piped())
                            .stderr(std::process::Stdio::piped())
                            .output();

                        let ninja_res = if setup_res.as_ref().map(|s| s.status.success()).unwrap_or(false) {
                            Command::new("ninja")
                                .args(["-C", "build"])
                                .current_dir("/tmp/wtype")
                                .stdin(std::process::Stdio::null())
                                .stdout(std::process::Stdio::piped())
                                .stderr(std::process::Stdio::piped())
                                .output()
                        } else {
                            setup_res
                        };

                        match ninja_res {
                            Ok(no) if no.status.success() => {
                                let built_file = std::path::Path::new("/tmp/wtype/build/wtype");
                                if built_file.exists() {
                                    if let Err(e) = std::fs::copy(built_file, &wtype_dst) {
                                        log(LogLevel::Error, format!("Failed to copy wtype binary: {e}"));
                                        errors += 1;
                                    } else {
                                        use std::os::unix::fs::PermissionsExt;
                                        let _ = std::fs::set_permissions(&wtype_dst, std::fs::Permissions::from_mode(0o755));
                                        log(LogLevel::Success, "Compiled and installed wtype to ~/.local/bin/wtype".into());
                                        copied += 1;
                                    }
                                } else {
                                    log(LogLevel::Error, "wtype build output binary missing in /tmp/wtype/build/wtype".into());
                                    errors += 1;
                                }
                            }
                            Ok(no) => {
                                log(
                                    LogLevel::Error,
                                    format!("wtype compilation failed: {}", String::from_utf8_lossy(&no.stderr).trim()),
                                );
                                errors += 1;
                            }
                            Err(e) => {
                                log(LogLevel::Error, format!("Failed to execute ninja for wtype: {e}"));
                                errors += 1;
                            }
                        }
                    }
                    Ok(o) => {
                        log(
                            LogLevel::Error,
                            format!("Failed to clone wtype repo: {}", String::from_utf8_lossy(&o.stderr).trim()),
                        );
                        errors += 1;
                    }
                    Err(e) => {
                        log(LogLevel::Error, format!("git clone error for wtype: {e}"));
                        errors += 1;
                    }
                }
            }
        }

        "kernel_permissions" => {
            log(
                LogLevel::Config,
                "Configuring i2c-dev and CPU performance permissions...".into(),
            );
            let _ = sudo.run_root_quiet(&["modprobe", "i2c-dev"]);

            let _ = sudo.write_root_file(
                std::path::Path::new("/etc/modules-load.d/i2c.conf"),
                "i2c-dev\n",
            );

            let tmpfiles_content = "z /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor 0666 root root -\nz /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference 0666 root root -\n";
            if let Err(e) = sudo.write_root_file(
                std::path::Path::new("/etc/tmpfiles.d/babydra-perf.conf"),
                tmpfiles_content,
            ) {
                log(
                    LogLevel::Warn,
                    format!("Failed to write tmpfiles config: {e}"),
                );
            }
            let _ = sudo.run_root_quiet(&[
                "systemd-tmpfiles",
                "--create",
                "/etc/tmpfiles.d/babydra-perf.conf",
            ]);
            let _ = sudo.run_root(&[
                "sh",
                "-c",
                "chmod 666 /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor 2>/dev/null || true",
            ]);
            let _ = sudo.run_root(&[
                "sh",
                "-c",
                "chmod 666 /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference 2>/dev/null || true",
            ]);
            log(
                LogLevel::Success,
                "Configured CPU governor & i2c-dev permissions.".into(),
            );
            copied += 1;
        }

        _ => {}
    }

    (copied, errors)
}
