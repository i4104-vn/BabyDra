use std::path::Path;

use crate::core::manifest::PermissionsConfig;
use crate::models::LogLevel;
use crate::runtime::SudoSession;

/// Configures kernel modules, user groups, and tmpfiles rules as declared by the manifest.
pub fn configure_permissions<F>(
    sudo: &SudoSession,
    config: &PermissionsConfig,
    mut log: F,
) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    log(
        LogLevel::Config,
        "Configuring system modules, permissions, and group access...".into(),
    );

    // 1. Kernel modules
    for module in &config.modules {
        let _ = sudo.run_root_quiet(&["modprobe", module]);
        let conf_content = format!("{module}\n");
        let conf_path = format!("/etc/modules-load.d/{module}.conf");
        let _ = sudo.write_root_file(Path::new(&conf_path), &conf_content);
    }

    // 2. Tmpfiles
    for (filename, content) in &config.tmpfiles {
        let dest = format!("/etc/tmpfiles.d/{filename}");
        if let Err(e) = sudo.write_root_file(Path::new(&dest), content) {
            log(
                LogLevel::Warn,
                format!("Failed to write tmpfiles config {dest}: {e}"),
            );
        } else {
            let _ = sudo.run_root_quiet(&["systemd-tmpfiles", "--create", &dest]);
        }
    }

    // Best-effort chmod for cpu scaling governors
    let _ = sudo.run_root(&[
        "sh",
        "-c",
        "chmod 666 /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor 2>/dev/null || true",
    ]);
    let _ = sudo.run_root(&[
        "sh",
        "-c",
        "chmod 666 /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference 2>/dev/null || true",
    ]);

    // 3. User groups
    if let Ok(user) = std::env::var("USER") {
        if !user.is_empty() {
            for group in &config.groups {
                let _ = sudo.run_root(&["usermod", "-aG", group, &user]);
                log(
                    LogLevel::Success,
                    format!("Added user {user} to {group} group."),
                );
            }
        }
    }

    log(
        LogLevel::Success,
        "Configured hardware modules and group permissions.".into(),
    );

    (1, 0)
}
