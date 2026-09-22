use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::core::manifest::StagingConfig;
use crate::models::{BinaryItem, LogLevel};
use crate::runtime::SudoSession;

/// Copies all compiled executables to the central system staging path (e.g. `/var/lib/babydra/bin`).
pub fn stage_binaries<F>(
    config: &StagingConfig,
    source_binary_dir: &Path,
    binaries: &[BinaryItem],
    sudo: &SudoSession,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let staging_base = PathBuf::from(&config.path);
    let staging_bin = staging_base.join("bin");

    log(
        LogLevel::Bundle,
        format!("Staging all built binaries into {}/bin/ ...", config.path),
    );
    let _ = sudo.run_root_quiet(&["mkdir", "-p", staging_bin.to_str().unwrap_or("/")]);

    let mut staged_any = false;
    for binary in binaries {
        let path = source_binary_dir.join(&binary.source_name);
        if !path.is_file() {
            continue;
        }
        let fname = &binary.name;
        let dst = staging_bin.join(fname);
        let out = sudo.run_root(&[
            "cp",
            path.to_str().unwrap_or(""),
            dst.to_str().unwrap_or(""),
        ]);
        let _ = sudo.run_root_quiet(&["chmod", "755", dst.to_str().unwrap_or("")]);
        if let Ok(o) = out {
            if o.success {
                staged_any = true;
                log(
                    LogLevel::Bundle,
                    format!("Staged binary -> {}/bin/{fname}", config.path),
                );
            } else {
                log(
                    LogLevel::Warn,
                    format!("Failed to stage {fname}: {}", o.stderr.trim()),
                );
            }
        }
    }

    if staged_any {
        log(
            LogLevel::Success,
            format!("Staged binaries to {}/bin/", config.path),
        );
    }
    (1, 0)
}

/// Applies configured permissions (e.g. `chmod 777`) on the staging directory.
pub fn set_staging_permissions<F>(
    config: &StagingConfig,
    sudo: &SudoSession,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let staging_base = PathBuf::from(&config.path);
    let mode_str = &config.permissions;

    log(
        LogLevel::Config,
        format!(
            "Setting chmod {} on {} for session and service access...",
            mode_str, config.path
        ),
    );

    if SudoSession::is_root() {
        if let Ok(mode) = u32::from_str_radix(mode_str, 8) {
            let mut perms = fs::metadata(&staging_base)
                .map(|m| m.permissions())
                .unwrap_or_else(|_| fs::Permissions::from_mode(mode));
            perms.set_mode(mode);
            let _ = fs::set_permissions(&staging_base, perms);
        }
    } else {
        let _ = sudo.run_root_quiet(&["chmod", mode_str, &config.path]);
    }

    log(
        LogLevel::Success,
        format!("Set {} permissions to {}.", config.path, mode_str),
    );
    (1, 0)
}
