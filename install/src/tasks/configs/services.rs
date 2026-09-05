use std::fs;
use std::process::Command;

use crate::models::LogLevel;
use crate::system::{get_user_home, get_user_local_bin, SudoSession};

pub fn restart_services<F>(sudo: &SudoSession, mut log: F) -> usize
where
    F: FnMut(LogLevel, String),
{
    let home = get_user_home();
    let user_bin_dir = get_user_local_bin();

    let labwc_running = sudo
        .run("pgrep", &["-x", "labwc"])
        .map(|o| o.success)
        .unwrap_or(false);
    if labwc_running {
        let _ = sudo.run("labwc", &["--reconfigure"]);
        log(
            LogLevel::Success,
            "Reloaded labwc compositor configuration.".into(),
        );
    }

    // Remove stale socket and restart switcher daemon
    let switcher_bin = user_bin_dir.join("babydra-switcher");
    if switcher_bin.exists() {
        let _ = fs::remove_file("/tmp/babydra-switcher.socket");
        let _ = Command::new(&switcher_bin)
            .arg("--daemon")
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
        log(
            LogLevel::Success,
            "babydra-switcher --daemon started in background.".into(),
        );
    }

    let panel_bin = user_bin_dir.join("babydra-panel");
    if panel_bin.exists() {
        log(
            LogLevel::Info,
            "Starting babydra-panel background service...".into(),
        );
        let log_dir = home.join(".cache/babydra");
        let _ = fs::create_dir_all(&log_dir);
        let log_file = log_dir.join("panel.log");
        let opened = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file);
        if let Ok(f) = opened {
            let out = f.try_clone().unwrap_or_else(|_| {
                fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&log_file)
                    .expect("reopen panel log")
            });
            let _ = Command::new(&panel_bin)
                .stdout(std::process::Stdio::from(out))
                .stderr(std::process::Stdio::from(f))
                .spawn();
        }
        log(
            LogLevel::Success,
            format!("babydra-panel started (logs: {}).", log_file.display()),
        );
    }

    let desktop_bin = user_bin_dir.join("babydra-desktop");
    if desktop_bin.exists() {
        let _ = Command::new(&desktop_bin)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
        log(
            LogLevel::Success,
            "babydra-desktop started in background.".into(),
        );
    }

    let keymap_bin = user_bin_dir.join("babydra-keymap");
    if keymap_bin.exists() {
        let _ = Command::new(&keymap_bin)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn();
        log(
            LogLevel::Success,
            "babydra-keymap daemon started in background.".into(),
        );
    }

    1
}
