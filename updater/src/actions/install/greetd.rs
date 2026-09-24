use crate::actions::runner::CommandRunner;
use crate::config::GreetdConfig;

pub fn configure_greetd(runner: &CommandRunner, config: &GreetdConfig) -> Result<(), String> {
    runner.step("Configuring greetd display manager...");
    runner.run_sudo("mkdir", &["-p", "/etc/greetd"], None)?;

    for vt in &config.mask_vt_range {
        let svc = format!("getty@tty{}.service", vt);
        runner.run_sudo("systemctl", &["stop", &svc], None)?;
        runner.run_sudo("systemctl", &["mask", &svc], None)?;
    }

    let greetd_toml = format!(
        "[terminal]\nvt = {}\n\n[default_session]\ncommand = \"{}\"\nuser = \"{}\"\n",
        config.vt, config.command, config.user
    );
    runner.run_sudo_with_input(
        "tee",
        &["/etc/greetd/config.toml"],
        None,
        greetd_toml.as_bytes(),
    )?;
    runner.run_sudo("systemctl", &["enable", "greetd.service"], None)?;

    Ok(())
}
