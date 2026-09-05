use crate::models::LogLevel;
use crate::system::{tail_lines, SudoSession};

pub fn install_pacman_packages<F>(sudo: &SudoSession, mut log: F) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
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
        "chafa",
        "imagemagick",
        "gst-plugins-good",
        "gst-plugins-bad",
        "gst-plugin-va",
        "gst-libav",
        "ffmpeg",
        "fcitx5",
        "fcitx5-gtk",
        "fcitx5-qt",
        "fcitx5-configtool",
        "fcitx5-unikey",
        "fcitx5-bamboo",
    ];

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
                (1, 0)
            } else {
                log(
                    LogLevel::Error,
                    format!("Pacman exited with error: {}", o.stderr.trim()),
                );
                (0, 1)
            }
        }
        Err(e) => {
            log(LogLevel::Error, format!("Failed to run pacman: {e}"));
            (0, 1)
        }
    }
}
