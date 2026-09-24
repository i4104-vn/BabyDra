use crate::actions::runner::CommandRunner;
use crate::core::state::ResetMode;
use std::process::Command;

pub fn clean_packages(
    runner: &CommandRunner,
    mode: ResetMode,
    is_dry_run: bool,
) -> Result<(), String> {
    runner.step("[Step 5/6] Cleaning packages...");

    if is_dry_run {
        runner.log("[dry-run] Would uninstall packages according to reset mode");
        return Ok(());
    }

    match mode {
        ResetMode::RemoveShellPackages => {
            let shell_pkgs = [
                "labwc",
                "greetd",
                "cage",
                "gtk4-layer-shell",
                "wlrctl",
                "ddcutil-service",
                "gammastep",
                "wlsunset",
                "kvantum-qt5",
            ];
            let mut pkgs_to_remove = Vec::new();
            for pkg in &shell_pkgs {
                if runner.run_cmd("pacman", &["-Q", pkg], None).is_ok() {
                    pkgs_to_remove.push(*pkg);
                }
            }
            if !pkgs_to_remove.is_empty() {
                let mut args = vec!["-Rns", "--noconfirm"];
                args.extend_from_slice(&pkgs_to_remove);
                runner.run_sudo("pacman", &args, None)?;
            }
        }
        ResetMode::RemoveAllApps => {
            runner.log("Scanning non-base packages to revert to pure Arch baseline...");
            let output = Command::new("pacman")
                .args(["-Qdtq"])
                .output()
                .map_err(|e| format!("Failed to query orphan packages: {}", e));

            match output {
                Ok(output) if output.status.success() => {
                    let package_list = String::from_utf8_lossy(&output.stdout);
                    let packages: Vec<&str> = package_list
                        .lines()
                        .map(str::trim)
                        .filter(|package| !package.is_empty())
                        .collect();

                    if packages.is_empty() {
                        runner.log("No orphan packages found.");
                    } else {
                        let mut args = vec!["-Rns", "--noconfirm"];
                        args.extend(packages);
                        runner.run_sudo("pacman", &args, None)?;
                    }
                }
                Ok(output) => {
                    return Err(format!(
                        "Failed to query orphan packages (exit code {}).",
                        output.status.code().unwrap_or(-1)
                    ));
                }
                Err(error) => return Err(error),
            }
        }
        _ => {
            runner.log("Skipping package uninstallation as requested.");
        }
    }

    Ok(())
}
