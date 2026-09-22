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
            Some(format!(
                "[terminal]\nvt = 1\n\n[default_session]\ncommand = \"sh -c 'clear 2>/dev/null; setterm -cursor off 2>/dev/null; exec cage -s -- {}'\"\nuser = \"greeter\"\n",
                path.display()
            ))
        })
    };

    let Some(greetd_toml) = greetd_toml else {
        log(
            LogLevel::Warn,
            "No display-manager config or system-scoped greeter declared; skipping greetd config.".into(),
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
        workspace_root.join("configs/greetd/config.toml"),
        workspace_root.join("greetd/config.toml"),
    ]
    .into_iter()
    .find_map(|path| std::fs::read_to_string(path).ok())
}
