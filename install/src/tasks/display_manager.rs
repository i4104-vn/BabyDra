use std::path::Path;

use crate::core::manifest::GreetdConfig;
use crate::discovery::binary_target_path;
use crate::models::{BinaryItem, LogLevel};
use crate::runtime::SudoSession;

pub fn configure_greetd_session<F>(
    workspace_root: &Path,
    config: &GreetdConfig,
    binaries: &[BinaryItem],
    sudo: &SudoSession,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    log(
        LogLevel::Config,
        "Configuring /etc/greetd/config.toml from source metadata...".into(),
    );

    let greetd_toml = if let Some(custom_conf) = &config.config {
        let path = workspace_root.join(custom_conf);
        std::fs::read_to_string(path).ok()
    } else {
        discover_greetd_config(workspace_root).or_else(|| {
            let greeter = binaries
                .iter()
                .find(|binary| matches!(binary.default_dest, crate::models::BinaryLocation::SystemBin))?;
            let path = binary_target_path(&greeter.name, &greeter.default_dest);

            // Verify binary existence: check /usr/bin/<name> or try to copy from staging / release dir
            if !path.exists() {
                let candidates = [
                    workspace_root.join("target/release").join(&greeter.name),
                    workspace_root.join("branches").join("release").join("target/release").join(&greeter.name),
                    workspace_root.join("branches").join("develop").join("target/release").join(&greeter.name),
                    std::path::PathBuf::from("/var/lib/babydra/bin").join(&greeter.name),
                    crate::runtime::get_user_local_bin().join(&greeter.name),
                ];
                let mut recovered = false;
                for candidate in &candidates {
                    if candidate.is_file() {
                        let _ = sudo.run_root(&[
                            "cp",
                            candidate.to_str().unwrap_or(""),
                            path.to_str().unwrap_or(""),
                        ]);
                        let _ = sudo.run_root_quiet(&["chmod", "755", path.to_str().unwrap_or("")]);
                        if path.exists() {
                            log(
                                LogLevel::Info,
                                format!("Recovered missing system greeter from {:?}", candidate),
                            );
                            recovered = true;
                            break;
                        }
                    }
                }

                if !recovered {
                    log(
                        LogLevel::Error,
                        format!(
                            "Greeter binary '{}' does not exist at {:?}. Skipping greetd config to avoid boot crash.",
                            greeter.name, path
                        ),
                    );
                    return None;
                }
            }

            Some(format!(
                "[terminal]\nvt = 1\n\n[default_session]\ncommand = \"sh -c 'clear 2>/dev/null; setterm -cursor off 2>/dev/null; exec cage -s -- {}'\"\nuser = \"greeter\"\n",
                path.display()
            ))
        })
    };

    let Some(greetd_toml) = greetd_toml else {
        log(
            LogLevel::Warn,
            "No valid display-manager config or verified greeter executable; skipping greetd config.".into(),
        );
        return (0, 0);
    };

    match sudo.write_root_file(Path::new("/etc/greetd/config.toml"), &greetd_toml) {
        Ok(()) => {
            log(
                LogLevel::Success,
                "Configured /etc/greetd/config.toml.".into(),
            );
            (1, 0)
        }
        Err(e) => {
            log(
                LogLevel::Error,
                format!("Failed to write /etc/greetd/config.toml: {e}"),
            );
            (0, 1)
        }
    }
}

pub fn mask_secondary_gettys<F>(
    config: &GreetdConfig,
    sudo: &SudoSession,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    log(
        LogLevel::Config,
        format!(
            "Masking secondary gettys on vt{:?} to eliminate screen flash...",
            config.mask_gettys
        ),
    );
    for &vt in &config.mask_gettys {
        let service = format!("getty@tty{vt}.service");
        let _ = sudo.run_root_quiet(&["systemctl", "stop", &service]);
        let _ = sudo.run_root_quiet(&["systemctl", "mask", &service]);
    }
    log(
        LogLevel::Success,
        "Masked getty services on secondary VTs.".into(),
    );
    (1, 0)
}

pub fn enable_greetd_service<F>(
    config: &GreetdConfig,
    sudo: &SudoSession,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    // Ensure both config and greeter executable exist before enabling greetd.service
    if !Path::new("/etc/greetd/config.toml").exists() {
        log(
            LogLevel::Error,
            "Cannot enable greetd.service: /etc/greetd/config.toml does not exist.".into(),
        );
        return (0, 1);
    }

    if !Path::new("/usr/bin/babydra-greeter").exists() {
        log(
            LogLevel::Error,
            "Cannot enable greetd.service: /usr/bin/babydra-greeter does not exist. Skipping to prevent boot crash.".into(),
        );
        return (0, 1);
    }

    log(
        LogLevel::Config,
        format!("Enabling {} on boot...", config.enable_service),
    );
    let _ = sudo.run_root_quiet(&["systemctl", "enable", &config.enable_service]);
    log(
        LogLevel::Success,
        format!("Enabled {} on system boot.", config.enable_service),
    );
    (1, 0)
}

fn discover_greetd_config(workspace_root: &Path) -> Option<String> {
    [
        workspace_root.join("assets/configs/greetd/config.toml"),
        workspace_root.join("configs/greetd/config.toml"),
        workspace_root.join("greetd/config.toml"),
    ]
    .into_iter()
    .find_map(|path| std::fs::read_to_string(path).ok())
}
