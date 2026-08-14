use std::path::{Path, PathBuf};
use std::fs;
use std::process::Command;
use crate::models::{BinaryItem, BinaryLocation, LogLevel};
use crate::system::{format_size, get_user_local_bin, is_root, safe_copy_binary};

pub fn execute_binary_copy_task<F>(
    bin: &BinaryItem,
    source_binary_dir: &Path,
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
        log(LogLevel::Error, format!("Source binary '{}' not found at {:?}", bin.name, src_file));
        return (0, 1);
    }

    let size_str = fs::metadata(&src_file)
        .map(|m| format_size(m.len()))
        .unwrap_or_else(|_| "unknown size".into());

    match bin.default_dest {
        BinaryLocation::UserLocalBin => {
            let dst_file = user_bin_dir.join(&bin.name);
            log(LogLevel::Copy, format!("Copying '{}' ({}) -> {:?}", bin.name, size_str, dst_file));

            match safe_copy_binary(&src_file, &dst_file) {
                Ok(()) => {
                    log(LogLevel::Success, format!("Installed {} to ~/.local/bin", bin.name));
                    copied += 1;
                }
                Err(e) => {
                    log(LogLevel::Error, format!("Error installing {}: {}", bin.name, e));
                    errors += 1;
                }
            }
        }
        BinaryLocation::SystemBin => {
            let dst_file = PathBuf::from("/usr/bin").join(&bin.name);
            log(LogLevel::Copy, format!("Copying system binary '{}' -> {:?}", bin.name, dst_file));

            if is_root() {
                match safe_copy_binary(&src_file, &dst_file) {
                    Ok(()) => {
                        log(LogLevel::Success, format!("Installed /usr/bin/{}", bin.name));
                        copied += 1;
                    }
                    Err(e) => {
                        log(LogLevel::Error, format!("Failed to install /usr/bin/{}: {}", bin.name, e));
                        errors += 1;
                    }
                }
            } else {
                log(LogLevel::Warn, format!("Root required for /usr/bin/{}. Running sudo cp...", bin.name));
                let status = Command::new("sudo")
                    .args(["cp", src_file.to_str().unwrap(), dst_file.to_str().unwrap()])
                    .status();

                if let Ok(st) = status {
                    if st.success() {
                        let _ = Command::new("sudo")
                            .args(["chmod", "755", dst_file.to_str().unwrap()])
                            .status();
                        log(LogLevel::Success, format!("Installed /usr/bin/{} via sudo.", bin.name));
                        copied += 1;
                    } else {
                        log(LogLevel::Warn, format!("Sudo copy failed. Installing fallback to {:?}", user_bin_dir.join(&bin.name)));
                        let _ = safe_copy_binary(&src_file, &user_bin_dir.join(&bin.name));
                        copied += 1;
                    }
                } else {
                    let _ = safe_copy_binary(&src_file, &user_bin_dir.join(&bin.name));
                    copied += 1;
                }
            }
        }
    }

    (copied, errors)
}
