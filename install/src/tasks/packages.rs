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

        "kernel_permissions" => {
            log(
                LogLevel::Config,
                "Configuring i2c-dev and CPU performance permissions...".into(),
            );
            let _ = sudo.run_root_quiet(&["modprobe", "i2c-dev"]);

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
