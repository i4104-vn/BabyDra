use std::fs;
use std::process::Command;
use crate::models::{GenericOptionItem, LogLevel};
use crate::system::is_root;

pub fn execute_display_manager_task<F>(
    opt: &GenericOptionItem,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    let mut copied = 0;

    match opt.id.as_str() {
        "greetd_config" => {
            log(LogLevel::Config, "Configuring /etc/greetd/config.toml (cage + babydra-greeter)...".into());
            let greetd_toml = "[terminal]\nvt = 1\n\n[default_session]\ncommand = \"sh -c 'clear 2>/dev/null; setterm -cursor off 2>/dev/null; exec cage -s -- /usr/bin/babydra-greeter'\"\nuser = \"greeter\"\n";

            if is_root() {
                let _ = fs::create_dir_all("/etc/greetd");
                let _ = fs::write("/etc/greetd/config.toml", greetd_toml);
            } else {
                let _ = Command::new("sudo").args(["mkdir", "-p", "/etc/greetd"]).status();
                let _ = Command::new("sudo").args(["sh", "-c", "echo '[terminal]\nvt = 1\n\n[default_session]\ncommand = \"sh -c \\'clear 2>/dev/null; setterm -cursor off 2>/dev/null; exec cage -s -- /usr/bin/babydra-greeter\\'\"\nuser = \"greeter\"' > /etc/greetd/config.toml"]).status();
            }
            log(LogLevel::Success, "Configured /etc/greetd/config.toml.".into());
            copied += 1;
        }

        "mask_gettys" => {
            log(LogLevel::Config, "Masking getty on tty2-6 to eliminate terminal screen flash...".into());
            for vt in 2..=6 {
                let service = format!("getty@tty{}.service", vt);
                if is_root() {
                    let _ = Command::new("systemctl").args(["stop", &service]).status();
                    let _ = Command::new("systemctl").args(["mask", &service]).status();
                } else {
                    let _ = Command::new("sudo").args(["systemctl", "stop", &service]).status();
                    let _ = Command::new("sudo").args(["systemctl", "mask", &service]).status();
                }
            }
            log(LogLevel::Success, "Masked getty services on secondary VTs.".into());
            copied += 1;
        }

        "enable_greetd" => {
            log(LogLevel::Config, "Enabling greetd.service...".into());
            if is_root() {
                let _ = Command::new("systemctl").args(["enable", "greetd.service"]).status();
            } else {
                let _ = Command::new("sudo").args(["systemctl", "enable", "greetd.service"]).status();
            }
            log(LogLevel::Success, "Enabled greetd.service on system boot.".into());
            copied += 1;
        }

        _ => {}
    }

    (copied, 0)
}
