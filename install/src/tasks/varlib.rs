use crate::models::{GenericOptionItem, LogLevel};
use crate::system::SudoSession;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn execute_varlib_task<F>(
    opt: &GenericOptionItem,
    source_binary_dir: &Path,
    sudo: &SudoSession,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let var_lib_babydra = PathBuf::from("/var/lib/babydra");
    let var_lib_bin = var_lib_babydra.join("bin");

    match opt.id.as_str() {
        "stage_binaries" => {
            log(
                LogLevel::Bundle,
                "Staging all built binaries into /var/lib/babydra/bin/...".into(),
            );
            let _ = sudo.run_root_quiet(&["mkdir", "-p", var_lib_bin.to_str().unwrap_or("/")]);

            let mut staged_any = false;
            if let Ok(entries) = fs::read_dir(source_binary_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let fname = path.file_name().unwrap_or_default().to_string_lossy();
                        if fname.starts_with("babydra-") && !fname.contains('.') {
                            let dst = var_lib_bin.join(&*fname);
                            let out = sudo.run_root(&[
                                "cp",
                                path.to_str().unwrap_or(""),
                                dst.to_str().unwrap_or(""),
                            ]);
                            let _ =
                                sudo.run_root_quiet(&["chmod", "755", dst.to_str().unwrap_or("")]);
                            if let Ok(o) = out {
                                if o.success {
                                    staged_any = true;
                                    log(
                                        LogLevel::Bundle,
                                        format!("Staged binary -> /var/lib/babydra/bin/{fname}"),
                                    );
                                } else {
                                    log(
                                        LogLevel::Warn,
                                        format!("Failed to stage {fname}: {}", o.stderr.trim()),
                                    );
                                }
                            }
                        }
                    }
                }
            }

            if staged_any {
                log(
                    LogLevel::Success,
                    "Staged binaries to /var/lib/babydra/bin/".into(),
                );
            }
            copied += 1;
        }

        "set_varlib_permissions" => {
            log(
                LogLevel::Config,
                "Setting chmod 777 on /var/lib/babydra for greeter/user access...".into(),
            );
            if SudoSession::is_root() {
                let mut perms = fs::metadata(&var_lib_babydra)
                    .map(|m| m.permissions())
                    .unwrap_or_else(|_| fs::Permissions::from_mode(0o777));
                perms.set_mode(0o777);
                let _ = fs::set_permissions(&var_lib_babydra, perms);
            } else {
                let _ = sudo.run_root_quiet(&["chmod", "777", "/var/lib/babydra"]);
            }
            log(
                LogLevel::Success,
                "Set /var/lib/babydra permissions to 0777.".into(),
            );
            copied += 1;
        }

        _ => {}
    }

    (copied, 0)
}
