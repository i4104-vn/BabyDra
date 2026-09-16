use crate::models::LogLevel;
use crate::system::{tail_lines, SudoSession};

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
        "Running pacman -Syu for system dependencies...".into(),
    );
    let mut args: Vec<&str> = vec!["pacman", "-Syu", "--needed", "--noconfirm"];
    args.extend(packages.iter().map(String::as_str));
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
