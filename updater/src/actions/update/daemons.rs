use crate::actions::runner::CommandRunner;
use crate::utils::fs::remove_file_if_exists;
use crate::utils::system::get_local_bin_dir;
use std::process::Command;

pub fn start_shell_daemons(runner: &CommandRunner, daemons: &[String]) -> Result<(), String> {
    runner.step("Starting shell services...");
    remove_file_if_exists(std::path::Path::new("/tmp/babydra-switcher.socket"));
    let local_bin = get_local_bin_dir();

    for daemon in daemons {
        let parts: Vec<&str> = daemon.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let bin_name = parts[0];
        let bin_path = local_bin.join(bin_name);

        if bin_path.exists() {
            let mut cmd = Command::new(&bin_path);
            if parts.len() > 1 {
                cmd.args(&parts[1..]);
            }
            match cmd.spawn() {
                Ok(_) => runner.log(format!("Started background daemon: {}", daemon)),
                Err(e) => return Err(format!("Failed to start {}: {}", daemon, e)),
            }
        } else {
            return Err(format!(
                "Configured daemon binary '{}' was not found at {}.",
                bin_name,
                bin_path.display()
            ));
        }
    }

    Ok(())
}
