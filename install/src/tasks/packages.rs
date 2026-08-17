use crate::models::{GenericOptionItem, LogLevel};
use crate::system::is_root;
use std::fs;
use std::process::Command;

pub fn execute_packages_task<F>(opt: &GenericOptionItem, mut log: F) -> (usize, usize)
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
            let mut cmd = Command::new(if is_root() { "pacman" } else { "sudo" });
            if !is_root() {
                cmd.arg("pacman");
            }
            cmd.args(["-Syu", "--needed", "--noconfirm"]).args(&pkgs);

            if let Ok(status) = cmd.status() {
                if status.success() {
                    log(
                        LogLevel::Success,
                        "Arch Linux pacman packages installed/updated.".into(),
                    );
                    copied += 1;
                } else {
                    log(
                        LogLevel::Error,
                        "Pacman command exited with error code.".into(),
                    );
                    errors += 1;
                }
            } else {
                log(LogLevel::Error, "Failed to execute pacman.".into());
                errors += 1;
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
            let status = Command::new("yay")
                .args(["-S", "--noconfirm", "--needed"])
                .args(&aur_pkgs)
                .status();

            if let Ok(st) = status {
                if st.success() {
                    log(
                        LogLevel::Success,
                        "AUR packages installed successfully.".into(),
                    );
                    copied += 1;
                } else {
                    log(
                        LogLevel::Warn,
                        "yay command completed with warnings.".into(),
                    );
                }
            } else {
                log(LogLevel::Warn, "yay not found or failed to execute.".into());
            }
        }

        "kernel_permissions" => {
            log(
                LogLevel::Config,
                "Configuring i2c-dev and CPU performance permissions...".into(),
            );
            let _ = Command::new(if is_root() { "modprobe" } else { "sudo" })
                .args(if is_root() {
                    vec!["i2c-dev"]
                } else {
                    vec!["modprobe", "i2c-dev"]
                })
                .status();

            let tmpfiles_content = "z /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor 0666 root root -\nz /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference 0666 root root -\n";
            if is_root() {
                let _ = fs::write("/etc/tmpfiles.d/babydra-perf.conf", tmpfiles_content);
                let _ = Command::new("systemd-tmpfiles")
                    .args(["--create", "/etc/tmpfiles.d/babydra-perf.conf"])
                    .status();
            } else {
                let _ = Command::new("sudo")
                    .args(["sh", "-c", "echo 'z /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor 0666 root root -\nz /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference 0666 root root -' > /etc/tmpfiles.d/babydra-perf.conf"])
                    .status();
                let _ = Command::new("sudo")
                    .args([
                        "systemd-tmpfiles",
                        "--create",
                        "/etc/tmpfiles.d/babydra-perf.conf",
                    ])
                    .status();
            }
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
