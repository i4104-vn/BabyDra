use crate::models::{GenericOptionItem, LogLevel};
use crate::system::SudoSession;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn execute_varlib_task<F>(
    opt: &GenericOptionItem,
    _workspace_root: &Path,
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

            let all_binary_names = [
                "babydra-panel",
                "babydra-desktop",
                "babydra-switcher",
                "babydra-screenshot",
                "babydra-lock",
                "babydra-launcher",
                "babydra-preview",
                "babydra-settings",
                "babydra-explore",
                "babydra-greeter",
            ];

            for bname in all_binary_names {
                let src = source_binary_dir.join(bname);
                let dst = var_lib_bin.join(bname);
                if src.exists() {
                    let out = sudo.run_root(&[
                        "cp",
                        src.to_str().unwrap_or(""),
                        dst.to_str().unwrap_or(""),
                    ]);
                    let _ = sudo.run_root_quiet(&["chmod", "755", dst.to_str().unwrap_or("")]);
                    if let Ok(o) = out {
                        if o.success {
                            log(
                                LogLevel::Bundle,
                                format!("Staged binary -> /var/lib/babydra/bin/{bname}"),
                            );
                        } else {
                            log(
                                LogLevel::Warn,
                                format!("Failed to stage {bname}: {}", o.stderr.trim()),
                            );
                        }
                    }
                }
            }
            log(
                LogLevel::Success,
                "Staged binaries to /var/lib/babydra/bin/".into(),
            );
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
