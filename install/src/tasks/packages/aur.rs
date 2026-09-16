use std::process::Command;

use crate::models::LogLevel;
use crate::system::{tail_lines, SudoSession};

pub fn ensure_yay_installed<F>(sudo: &SudoSession, mut log: F) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
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
        return (1, 0);
    }

    log(
        LogLevel::Info,
        "yay not found, building yay-bin from AUR (/tmp/yay-bin)...".into(),
    );
    let _ = std::fs::remove_dir_all("/tmp/yay-bin");
    let clone_res = Command::new("git")
        .args([
            "clone",
            "https://aur.archlinux.org/yay-bin.git",
            "/tmp/yay-bin",
        ])
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
                        (1, 0)
                    } else {
                        log(
                            LogLevel::Error,
                            format!("makepkg failed for yay-bin: {}", stderr.trim()),
                        );
                        (0, 1)
                    }
                }
                Err(e) => {
                    log(
                        LogLevel::Error,
                        format!("Failed to run makepkg for yay-bin: {e}"),
                    );
                    (0, 1)
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
            (0, 1)
        }
        Err(e) => {
            log(LogLevel::Error, format!("git clone error for yay-bin: {e}"));
            (0, 1)
        }
    }
}

pub fn install_aur_packages<F>(
    sudo: &SudoSession,
    packages: &[String],
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    if packages.is_empty() {
        log(
            LogLevel::Info,
            "No AUR packages declared by the source branch; skipping.".into(),
        );
        return (0, 0);
    }
    log(LogLevel::Info, "Installing AUR packages via yay...".into());
    if let Err(e) = sudo.preauth() {
        log(
            LogLevel::Error,
            format!("Sudo credential expired before AUR install: {e}"),
        );
        return (0, 1);
    }

    let mut cmd = Command::new("yay");
    cmd.args(["-S", "--noconfirm", "--needed"]);
    cmd.args(packages);
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
                (1, 0)
            } else {
                log(
                    LogLevel::Warn,
                    format!("yay completed with errors: {}", stderr.trim()),
                );
                (0, 1)
            }
        }
        Err(e) => {
            log(
                LogLevel::Warn,
                format!("yay not found or failed to execute: {e}"),
            );
            (0, 1)
        }
    }
}
