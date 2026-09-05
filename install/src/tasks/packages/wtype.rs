use std::process::Command;

use crate::models::LogLevel;
use crate::system::get_user_local_bin;

pub fn build_wtype_from_source<F>(mut log: F) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let local_bin = get_user_local_bin();
    let wtype_dst = local_bin.join("wtype");

    if wtype_dst.is_file() {
        log(
            LogLevel::Success,
            format!("wtype binary already exists at {:?}", wtype_dst),
        );
        return (1, 0);
    }

    log(
        LogLevel::Info,
        "wtype not found, compiling from source with meson + ninja...".into(),
    );
    let _ = std::fs::create_dir_all(&local_bin);
    let _ = std::fs::remove_dir_all("/tmp/wtype");

    let clone_res = Command::new("git")
        .args(["clone", "https://github.com/atx/wtype.git", "/tmp/wtype"])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output();

    match clone_res {
        Ok(o) if o.status.success() => {
            let setup_res = Command::new("meson")
                .args(["setup", "build"])
                .current_dir("/tmp/wtype")
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .output();

            let ninja_res = if setup_res
                .as_ref()
                .map(|s| s.status.success())
                .unwrap_or(false)
            {
                Command::new("ninja")
                    .args(["-C", "build"])
                    .current_dir("/tmp/wtype")
                    .stdin(std::process::Stdio::null())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .output()
            } else {
                setup_res
            };

            match ninja_res {
                Ok(no) if no.status.success() => {
                    let built_file = std::path::Path::new("/tmp/wtype/build/wtype");
                    if built_file.exists() {
                        if let Err(e) = std::fs::copy(built_file, &wtype_dst) {
                            log(
                                LogLevel::Error,
                                format!("Failed to copy wtype binary: {e}"),
                            );
                            (0, 1)
                        } else {
                            use std::os::unix::fs::PermissionsExt;
                            let _ = std::fs::set_permissions(
                                &wtype_dst,
                                std::fs::Permissions::from_mode(0o755),
                            );
                            log(
                                LogLevel::Success,
                                "Compiled and installed wtype to ~/.local/bin/wtype".into(),
                            );
                            (1, 0)
                        }
                    } else {
                        log(
                            LogLevel::Error,
                            "wtype build output binary missing in /tmp/wtype/build/wtype".into(),
                        );
                        (0, 1)
                    }
                }
                Ok(no) => {
                    log(
                        LogLevel::Error,
                        format!(
                            "wtype compilation failed: {}",
                            String::from_utf8_lossy(&no.stderr).trim()
                        ),
                    );
                    (0, 1)
                }
                Err(e) => {
                    log(
                        LogLevel::Error,
                        format!("Failed to execute ninja for wtype: {e}"),
                    );
                    (0, 1)
                }
            }
        }
        Ok(o) => {
            log(
                LogLevel::Error,
                format!(
                    "Failed to clone wtype repo: {}",
                    String::from_utf8_lossy(&o.stderr).trim()
                ),
            );
            (0, 1)
        }
        Err(e) => {
            log(LogLevel::Error, format!("git clone error for wtype: {e}"));
            (0, 1)
        }
    }
}
