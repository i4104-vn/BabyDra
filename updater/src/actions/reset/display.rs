use crate::actions::runner::CommandRunner;

pub fn restore_tty_console(runner: &CommandRunner, is_dry_run: bool) -> Result<(), String> {
    runner.step("[Step 2/6] Restoring Arch Linux TTY login console & disabling display manager...");

    if !is_dry_run {
        runner.run_sudo("systemctl", &["stop", "greetd.service"], None)?;
        runner.run_sudo("systemctl", &["disable", "greetd.service"], None)?;
        runner.run_sudo("rm", &["-rf", "/etc/greetd"], None)?;

        for vt in 2..=6 {
            let svc = format!("getty@tty{}.service", vt);
            runner.run_sudo("systemctl", &["unmask", &svc], None)?;
        }
        runner.run_sudo("systemctl", &["enable", "getty@tty1.service"], None)?;
        runner.run_sudo("systemctl", &["set-default", "multi-user.target"], None)?;
    } else {
        runner.log("[dry-run] Would disable greetd, unmask getty@tty2..6, enable getty@tty1, set-default multi-user.target");
    }

    Ok(())
}
