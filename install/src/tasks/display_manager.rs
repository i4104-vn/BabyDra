use crate::models::{BinaryItem, GenericOptionItem, LogLevel};
use crate::system::{binary_target_path, SudoSession};
use std::path::Path;

pub fn execute_display_manager_task<F>(
    opt: &GenericOptionItem,
    workspace_root: &Path,
    binaries: &[BinaryItem],
    sudo: &SudoSession,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;

    match opt.id.as_str() {
        "greetd_config" => {
            log(
                LogLevel::Config,
                "Configuring /etc/greetd/config.toml from source metadata...".into(),
            );
            let greetd_toml = discover_greetd_config(workspace_root).or_else(|| {
                let greeter = binaries
                    .iter()
                    .find(|binary| matches!(binary.default_dest, crate::models::BinaryLocation::SystemBin))?;
                let path = binary_target_path(&greeter.name, &greeter.default_dest);
                Some(format!(
                    "[terminal]\nvt = 1\n\n[default_session]\ncommand = \"sh -c 'clear 2>/dev/null; setterm -cursor off 2>/dev/null; exec cage -s -- {}'\"\nuser = \"greeter\"\n",
                    path.display()
                ))
            });

            let Some(greetd_toml) = greetd_toml else {
                log(
                    LogLevel::Warn,
                    "No display-manager config or system-scoped greeter declared; skipping greetd config.".into(),
                );
                return (0, 0);
            };

            // Write via temp file + sudo cp (avoids fragile `sudo sh -c echo`).
            match sudo.write_root_file(Path::new("/etc/greetd/config.toml"), &greetd_toml) {
                Ok(()) => {
                    log(
                        LogLevel::Success,
                        "Configured /etc/greetd/config.toml.".into(),
                    );
                    copied += 1;
                }
                Err(e) => log(
                    LogLevel::Error,
                    format!("Failed to write /etc/greetd/config.toml: {e}"),
                ),
            }
        }

        "mask_gettys" => {
            log(
                LogLevel::Config,
                "Masking getty on tty2-6 to eliminate terminal screen flash...".into(),
            );
            for vt in 2..=6 {
                let service = format!("getty@tty{vt}.service");
                let _ = sudo.run_root_quiet(&["systemctl", "stop", &service]);
                let _ = sudo.run_root_quiet(&["systemctl", "mask", &service]);
            }
            log(
                LogLevel::Success,
                "Masked getty services on secondary VTs.".into(),
            );
            copied += 1;
        }

        "enable_greetd" => {
            log(LogLevel::Config, "Enabling greetd.service...".into());
            let _ = sudo.run_root_quiet(&["systemctl", "enable", "greetd.service"]);
            log(
                LogLevel::Success,
                "Enabled greetd.service on system boot.".into(),
            );
            copied += 1;
        }

        _ => {}
    }

    (copied, 0)
}

fn discover_greetd_config(workspace_root: &Path) -> Option<String> {
    [
        workspace_root.join("configs/greetd/config.toml"),
        workspace_root.join("greetd/config.toml"),
    ]
    .into_iter()
    .find_map(|path| std::fs::read_to_string(path).ok())
}
