use crate::models::LogLevel;
use crate::system::SudoSession;

pub fn configure_kernel_permissions<F>(sudo: &SudoSession, mut log: F) -> (usize, usize)
where
    F: FnMut(LogLevel, String),
{
    log(
        LogLevel::Config,
        "Configuring i2c-dev and CPU performance permissions...".into(),
    );
    let _ = sudo.run_root_quiet(&["modprobe", "i2c-dev"]);

    let _ = sudo.write_root_file(
        std::path::Path::new("/etc/modules-load.d/i2c.conf"),
        "i2c-dev\n",
    );

    let tmpfiles_content = "z /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor 0666 root root -\nz /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference 0666 root root -\n";
    if let Err(e) = sudo.write_root_file(
        std::path::Path::new("/etc/tmpfiles.d/babydra-perf.conf"),
        tmpfiles_content,
    ) {
        log(
            LogLevel::Warn,
            format!("Failed to write tmpfiles config: {e}"),
        );
    }
    let _ = sudo.run_root_quiet(&[
        "systemd-tmpfiles",
        "--create",
        "/etc/tmpfiles.d/babydra-perf.conf",
    ]);
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

    // Add user to input group for babydra-keymap daemon
    if let Ok(user) = std::env::var("USER") {
        if !user.is_empty() {
            let _ = sudo.run_root(&["usermod", "-aG", "input", &user]);
            log(
                LogLevel::Success,
                format!("Added {user} to input group for keymap daemon."),
            );
        }
    }

    log(
        LogLevel::Success,
        "Configured CPU governor, i2c-dev & input group permissions.".into(),
    );

    (1, 0)
}
