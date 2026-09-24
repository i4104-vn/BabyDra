use crate::actions::runner::CommandRunner;
use crate::config::PackagesConfig;

pub fn install_pacman_packages(
    runner: &CommandRunner,
    config: &PackagesConfig,
) -> Result<(), String> {
    if config.pacman.is_empty() {
        runner.log("No official packages configured; skipping pacman.");
        return Ok(());
    }

    runner.step("Installing official Arch Linux packages via pacman...");
    let mut pacman_args = vec!["-Syu", "--needed", "--noconfirm"];
    let pacman_pkgs: Vec<&str> = config.pacman.iter().map(|s| s.as_str()).collect();
    pacman_args.extend_from_slice(&pacman_pkgs);
    runner.run_sudo("pacman", &pacman_args, None)?;
    runner.success("Pacman packages installed.");
    Ok(())
}

pub fn install_yay_packages(runner: &CommandRunner, config: &PackagesConfig) -> Result<(), String> {
    if config.yay.is_empty() {
        runner.log("No AUR packages configured; skipping yay.");
        return Ok(());
    }

    runner.step("Installing AUR packages via yay...");
    let mut yay_args = vec!["-S", "--needed", "--noconfirm"];
    let yay_pkgs: Vec<&str> = config.yay.iter().map(|s| s.as_str()).collect();
    yay_args.extend_from_slice(&yay_pkgs);
    runner.run_cmd("yay", &yay_args, None)?;
    runner.success("AUR packages installed.");
    Ok(())
}
