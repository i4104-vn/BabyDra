use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use crate::core::manifest::BuildDepConfig;
use crate::models::LogLevel;
use crate::runtime::{expand_path, tail_lines};

/// Builds and installs an external dependency from source (e.g. wtype, wlrctl).
pub fn build_dependency<F>(dep: &BuildDepConfig, mut log: F) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let target_dest = expand_path(&dep.install_to);

    // If check executable exists in target or PATH, skip build
    let already_installed = if target_dest.is_file() {
        true
    } else {
        Command::new("which")
            .arg(&dep.check)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    };

    if already_installed {
        log(
            LogLevel::Success,
            format!("{} is already installed; skipped build.", dep.name),
        );
        return (1, 0);
    }

    log(
        LogLevel::Info,
        format!("{} not found, building from source ({}) ...", dep.name, dep.git),
    );

    let build_dir = std::env::temp_dir().join(format!("build_{}", dep.name));
    let _ = fs::remove_dir_all(&build_dir);

    // 1. Clone
    let clone_res = Command::new("git")
        .args(["clone", &dep.git, build_dir.to_str().unwrap_or("")])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output();

    match clone_res {
        Ok(o) if o.status.success() => {
            // 2. Run build steps
            for cmd_line in &dep.build {
                let parts: Vec<&str> = cmd_line.split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }
                let mut cmd = Command::new(parts[0]);
                cmd.args(&parts[1..]);
                cmd.current_dir(&build_dir);
                cmd.stdin(std::process::Stdio::null());
                cmd.stdout(std::process::Stdio::piped());
                cmd.stderr(std::process::Stdio::piped());

                let step_res = cmd.output();
                match step_res {
                    Ok(out) => {
                        let combined = format!(
                            "{}\n{}",
                            String::from_utf8_lossy(&out.stdout),
                            String::from_utf8_lossy(&out.stderr)
                        );
                        for line in tail_lines(&combined, 4) {
                            log(LogLevel::Info, line);
                        }
                        if !out.status.success() {
                            log(
                                LogLevel::Error,
                                format!("Build command failed: '{cmd_line}'"),
                            );
                            return (0, 1);
                        }
                    }
                    Err(e) => {
                        log(
                            LogLevel::Error,
                            format!("Failed to execute build step '{cmd_line}': {e}"),
                        );
                        return (0, 1);
                    }
                }
            }

            // 3. Install artifact
            let artifact_path = build_dir.join(&dep.artifact);
            if !artifact_path.exists() {
                log(
                    LogLevel::Error,
                    format!("Expected build artifact missing at {:?}", artifact_path),
                );
                return (0, 1);
            }

            let install_target = if target_dest.is_dir() || dep.install_to.ends_with('/') {
                target_dest.join(Path::new(&dep.artifact).file_name().unwrap_or_default())
            } else {
                target_dest
            };

            if let Some(parent) = install_target.parent() {
                let _ = fs::create_dir_all(parent);
            }

            match fs::copy(&artifact_path, &install_target) {
                Ok(_) => {
                    let _ = fs::set_permissions(&install_target, fs::Permissions::from_mode(0o755));
                    log(
                        LogLevel::Success,
                        format!("Installed {} to {}", dep.name, install_target.display()),
                    );
                    (1, 0)
                }
                Err(e) => {
                    log(
                        LogLevel::Error,
                        format!("Failed to install build artifact: {e}"),
                    );
                    (0, 1)
                }
            }
        }
        Ok(o) => {
            log(
                LogLevel::Error,
                format!(
                    "git clone failed for {}: {}",
                    dep.name,
                    String::from_utf8_lossy(&o.stderr).trim()
                ),
            );
            (0, 1)
        }
        Err(e) => {
            log(LogLevel::Error, format!("Failed to clone {}: {e}", dep.name));
            (0, 1)
        }
    }
}
