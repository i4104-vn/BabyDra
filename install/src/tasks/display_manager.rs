use crate::models::{GenericOptionItem, LogLevel};
use crate::system::SudoSession;
use std::path::Path;

pub fn execute_display_manager_task<F>(
    opt: &GenericOptionItem,
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
                "Configuring /etc/greetd/config.toml (cage + babydra-greeter)...".into(),
            );
            let greetd_toml = "[terminal]\nvt = 1\n\n[default_session]\ncommand = \"sh -c 'clear 2>/dev/null; setterm -cursor off 2>/dev/null; exec cage -s -- /usr/bin/babydra-greeter'\"\nuser = \"greeter\"\n";

            // Write via temp file + sudo cp (avoids fragile `sudo sh -c echo`).
            match sudo.write_root_file(Path::new("/etc/greetd/config.toml"), greetd_toml) {
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
