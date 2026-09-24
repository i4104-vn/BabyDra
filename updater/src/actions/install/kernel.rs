use crate::actions::runner::CommandRunner;
use crate::utils::system::get_current_user;

pub fn configure_kernel_and_permissions(runner: &CommandRunner) -> Result<(), String> {
    let user = get_current_user();

    // 1. i2c-dev module
    runner.step("Configuring i2c-dev kernel module for monitor brightness...");
    runner.run_sudo("modprobe", &["i2c-dev"], None)?;
    runner.run_sudo_with_input("tee", &["/etc/modules-load.d/i2c.conf"], None, b"i2c-dev\n")?;

    // 2. CPU governor & EPP permissions
    runner.step("Configuring CPU performance permissions...");
    let perf_conf = "z /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor 0666 root root -\nz /sys/devices/system/cpu/cpu*/cpufreq/energy_performance_preference 0666 root root -\n";
    runner.run_sudo_with_input(
        "tee",
        &["/etc/tmpfiles.d/babydra-perf.conf"],
        None,
        perf_conf.as_bytes(),
    )?;
    runner.run_sudo(
        "systemd-tmpfiles",
        &["--create", "/etc/tmpfiles.d/babydra-perf.conf"],
        None,
    )?;

    // 3. User input group and udev rules
    runner.step("Configuring input group and udev rules for keymap daemon...");
    runner.run_sudo("usermod", &["-aG", "input", &user], None)?;
    runner.run_sudo("modprobe", &["uinput"], None)?;
    runner.run_sudo_with_input(
        "tee",
        &["/etc/modules-load.d/babydra-keymap.conf"],
        None,
        b"uinput\n",
    )?;

    let udev_rules = "KERNEL==\"uinput\", GROUP=\"input\", MODE=\"0660\"\nSUBSYSTEM==\"input\", KERNEL==\"event*\", GROUP=\"input\", MODE=\"0660\"\n";
    runner.run_sudo_with_input(
        "tee",
        &["/etc/udev/rules.d/99-babydra-keymap.rules"],
        None,
        udev_rules.as_bytes(),
    )?;
    runner.run_sudo("udevadm", &["control", "--reload-rules"], None)?;
    runner.run_sudo(
        "udevadm",
        &[
            "trigger",
            "--subsystem-match=misc",
            "--sysname-match=uinput",
        ],
        None,
    )?;
    runner.run_sudo("udevadm", &["trigger", "--subsystem-match=input"], None)?;
    runner.run_sudo("udevadm", &["settle"], None)?;

    Ok(())
}
