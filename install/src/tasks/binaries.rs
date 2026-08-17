use crate::models::{BinaryItem, BinaryLocation, LogLevel};
use crate::system::{format_size, get_user_local_bin, safe_copy_binary, SudoSession};
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub fn execute_binary_copy_task<F>(
    bin: &BinaryItem,
    source_binary_dir: &Path,
    sudo: &SudoSession,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let mut errors = 0;
    let user_bin_dir = get_user_local_bin();

    let src_file = source_binary_dir.join(&bin.name);
    if !src_file.exists() {
        log(
            LogLevel::Error,
            format!("Source binary '{}' not found at {:?}", bin.name, src_file),
        );
        return (0, 1);
    }

    let size_str = fs::metadata(&src_file)
        .map(|m| format_size(m.len()))
        .unwrap_or_else(|_| "unknown size".into());

    match bin.default_dest {
        BinaryLocation::UserLocalBin => {
            let dst_file = user_bin_dir.join(&bin.name);
            log(
                LogLevel::Copy,
                format!("Copying '{}' ({}) -> {:?}", bin.name, size_str, dst_file),
            );

            match safe_copy_binary(&src_file, &dst_file) {
                Ok(()) => {
                    log(
                        LogLevel::Success,
                        format!("Installed {} to ~/.local/bin", bin.name),
                    );
                    copied += 1;
                }
                Err(e) => {
                    log(
                        LogLevel::Error,
                        format!("Error installing {}: {}", bin.name, e),
                    );
                    errors += 1;
                }
            }
        }
        BinaryLocation::SystemBin => {
            let dst_file = PathBuf::from("/usr/bin").join(&bin.name);
            log(
                LogLevel::Copy,
                format!("Copying system binary '{}' -> {:?}", bin.name, dst_file),
            );

            // Use the pre-authenticated sudo session (piped password) instead
            // of a fresh `sudo cp` which would prompt on the TTY.
            let out = sudo.run_root(&[
                "cp",
                src_file.to_str().unwrap_or(""),
                dst_file.to_str().unwrap_or(""),
            ]);
            match out {
                Ok(o) if o.success => {
                    let _ = sudo.run_root_quiet(&["chmod", "755", dst_file.to_str().unwrap_or("")]);
                    log(
                        LogLevel::Success,
                        format!("Installed /usr/bin/{} via sudo.", bin.name),
                    );
                    copied += 1;
                }
                Ok(o) => {
                    log(
                        LogLevel::Warn,
                        format!(
                            "Sudo copy failed ({}). Installing fallback to {:?}",
                            o.stderr.trim(),
                            user_bin_dir.join(&bin.name)
                        ),
                    );
                    let _ = safe_copy_binary(&src_file, &user_bin_dir.join(&bin.name));
                    copied += 1;
                }
                Err(e) => {
                    log(
                        LogLevel::Warn,
                        format!("Sudo unavailable ({e}). Fallback to ~/.local/bin"),
                    );
                    let _ = safe_copy_binary(&src_file, &user_bin_dir.join(&bin.name));
                    copied += 1;
                }
            }
        }
    }

    (copied, errors)
}
