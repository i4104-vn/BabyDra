use std::process::Command;

use crate::models::LogLevel;
use crate::runtime::{spawn_and_stream, SudoSession};

pub fn install_pacman_packages<F>(
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
            "No pacman packages declared by the source branch; skipping.".into(),
        );
        return (0, 0);
    }
    log(
        LogLevel::Info,
        format!("Running pacman -Syu for {} package(s)...", packages.len()),
    );
    let mut args: Vec<&str> = vec!["pacman", "-Syu", "--needed", "--noconfirm"];
    args.extend(packages.iter().map(String::as_str));

    match sudo.run_root_streaming(&args, |is_err, line| {
        if is_err && line.to_lowercase().contains("error:") {
            log(LogLevel::Error, line);
        } else {
            log(LogLevel::Info, line);
        }
    }) {
        Ok(true) => {
            log(
                LogLevel::Success,
                "Arch Linux pacman packages installed/updated successfully.".into(),
            );
            (1, 0)
        }
        Ok(false) => {
            log(LogLevel::Error, "Pacman exited with error.".into());
            (0, 1)
        }
        Err(e) => {
            log(LogLevel::Error, format!("Failed to run pacman: {e}"));
            (0, 1)
        }
    }
}

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

    let mut git_cmd = Command::new("git");
    git_cmd.args([
        "clone",
        "https://aur.archlinux.org/yay-bin.git",
        "/tmp/yay-bin",
    ]);

    let clone_ok = spawn_and_stream(git_cmd, None, |_, line| {
        log(LogLevel::Info, line);
    })
    .unwrap_or(false);

    if !clone_ok {
        log(LogLevel::Error, "Failed to clone yay-bin repo.".into());
        return (0, 1);
    }

    let _ = sudo.preauth();
    let mut makepkg_cmd = Command::new("makepkg");
    makepkg_cmd.args(["-si", "--noconfirm"]);
    makepkg_cmd.current_dir("/tmp/yay-bin");

    match spawn_and_stream(makepkg_cmd, None, |is_err, line| {
        if is_err && line.to_lowercase().contains("error:") {
            log(LogLevel::Error, line);
        } else {
            log(LogLevel::Info, line);
        }
    }) {
        Ok(true) => {
            log(LogLevel::Success, "yay-bin installed successfully.".into());
            (1, 0)
        }
        Ok(false) => {
            log(LogLevel::Error, "makepkg failed for yay-bin.".into());
            (0, 1)
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
    log(
        LogLevel::Info,
        format!("Installing {} AUR package(s) via yay...", packages.len()),
    );
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

    match spawn_and_stream(cmd, None, |is_err, line| {
        if is_err && line.to_lowercase().contains("error:") {
            log(LogLevel::Error, line);
        } else {
            log(LogLevel::Info, line);
        }
    }) {
        Ok(true) => {
            log(
                LogLevel::Success,
                "AUR packages installed successfully.".into(),
            );
            (1, 0)
        }
        Ok(false) => {
            log(
                LogLevel::Warn,
                "yay completed with warnings or errors.".into(),
            );
            (0, 1)
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
