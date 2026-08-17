use crate::models::{GenericOptionItem, LogLevel};
use crate::system::{get_user_home, is_root, safe_copy_binary};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn execute_varlib_task<F>(
    opt: &GenericOptionItem,
    workspace_root: &Path,
    source_binary_dir: &Path,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;
    let var_lib_babydra = PathBuf::from("/var/lib/babydra");
    let var_lib_bin = var_lib_babydra.join("bin");
    let home = get_user_home();

    match opt.id.as_str() {
        "stage_binaries" => {
            log(
                LogLevel::Bundle,
                "Staging all built binaries into /var/lib/babydra/bin/...".into(),
            );
            if is_root() {
                let _ = fs::create_dir_all(&var_lib_bin);
            } else {
                let _ = Command::new("sudo")
                    .args(["mkdir", "-p", var_lib_bin.to_str().unwrap()])
                    .status();
            }

            let all_binary_names = [
                "babydra-panel",
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
                    if is_root() {
                        let _ = safe_copy_binary(&src, &dst);
                    } else {
                        let _ = Command::new("sudo")
                            .args(["cp", src.to_str().unwrap(), dst.to_str().unwrap()])
                            .status();
                        let _ = Command::new("sudo")
                            .args(["chmod", "755", dst.to_str().unwrap()])
                            .status();
                    }
                    log(
                        LogLevel::Bundle,
                        format!("Staged binary -> /var/lib/babydra/bin/{}", bname),
                    );
                }
            }
            log(
                LogLevel::Success,
                "Staged binaries to /var/lib/babydra/bin/".into(),
            );
            copied += 1;
        }

        "stage_wallpapers" => {
            log(
                LogLevel::Bundle,
                "Staging wallpapers to /var/lib/babydra and /usr/share/babydra...".into(),
            );
            let wp = workspace_root.join("wallpaper.png");
            let user_babydra = home.join(".babydra");
            let _ = fs::create_dir_all(&user_babydra);

            if wp.exists() {
                let _ = fs::copy(&wp, user_babydra.join("wallpaper.png"));

                if is_root() {
                    let _ = fs::create_dir_all(&var_lib_babydra);
                    let _ = fs::create_dir_all("/usr/share/babydra");
                    let _ = fs::copy(&wp, var_lib_babydra.join("greeter_wallpaper.png"));
                    let _ = fs::copy(&wp, "/usr/share/babydra/wallpaper.png");
                } else {
                    let _ = Command::new("sudo")
                        .args(["mkdir", "-p", "/usr/share/babydra", "/var/lib/babydra"])
                        .status();
                    let _ = Command::new("sudo")
                        .args([
                            "cp",
                            wp.to_str().unwrap(),
                            "/var/lib/babydra/greeter_wallpaper.png",
                        ])
                        .status();
                    let _ = Command::new("sudo")
                        .args([
                            "cp",
                            wp.to_str().unwrap(),
                            "/usr/share/babydra/wallpaper.png",
                        ])
                        .status();
                }
                log(LogLevel::Success, "Deployed system wallpapers.".into());
            }
            copied += 1;
        }

        "stage_logos" => {
            log(
                LogLevel::Bundle,
                "Deploying logos & icons to /var/lib/babydra & /usr/share/babydra...".into(),
            );
            let logo = workspace_root.join("libs/babydra-core/src/services/logo.png");
            let user_babydra = home.join(".babydra");
            let _ = fs::create_dir_all(&user_babydra);

            if logo.exists() {
                let _ = fs::copy(&logo, user_babydra.join("logo.png"));

                if is_root() {
                    let _ = fs::create_dir_all(&var_lib_babydra);
                    let _ = fs::create_dir_all("/usr/share/babydra");
                    let _ = fs::copy(&logo, var_lib_babydra.join("logo.png"));
                    let _ = fs::copy(&logo, "/usr/share/babydra/logo.png");
                    let _ = fs::copy(&logo, "/usr/share/babydra/babydra-preview.png");
                    let _ = fs::copy(&logo, "/usr/share/babydra/babydra-settings.png");
                } else {
                    let _ = Command::new("sudo")
                        .args(["mkdir", "-p", "/usr/share/babydra", "/var/lib/babydra"])
                        .status();
                    let _ = Command::new("sudo")
                        .args(["cp", logo.to_str().unwrap(), "/var/lib/babydra/logo.png"])
                        .status();
                    let _ = Command::new("sudo")
                        .args(["cp", logo.to_str().unwrap(), "/usr/share/babydra/logo.png"])
                        .status();
                    let _ = Command::new("sudo")
                        .args([
                            "cp",
                            logo.to_str().unwrap(),
                            "/usr/share/babydra/babydra-preview.png",
                        ])
                        .status();
                    let _ = Command::new("sudo")
                        .args([
                            "cp",
                            logo.to_str().unwrap(),
                            "/usr/share/babydra/babydra-settings.png",
                        ])
                        .status();
                }
                log(
                    LogLevel::Success,
                    "Deployed brand logos & preview icons.".into(),
                );
            }
            copied += 1;
        }

        "set_varlib_permissions" => {
            log(
                LogLevel::Config,
                "Setting chmod 777 on /var/lib/babydra for greeter/user access...".into(),
            );
            if is_root() {
                let mut perms = fs::metadata(&var_lib_babydra)
                    .map(|m| m.permissions())
                    .unwrap_or_else(|_| fs::Permissions::from_mode(0o777));
                perms.set_mode(0o777);
                let _ = fs::set_permissions(&var_lib_babydra, perms);
            } else {
                let _ = Command::new("sudo")
                    .args(["chmod", "777", "/var/lib/babydra"])
                    .status();
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
