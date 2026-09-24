use crate::actions::runner::CommandRunner;
use crate::utils::fs::remove_file_if_exists;
use crate::utils::process::spawn_daemon;
use crate::utils::system::get_local_bin_dir;
use std::path::Path;

pub fn start_shell_daemons(runner: &CommandRunner, daemons: &[String]) -> Result<(), String> {
    runner.step("Starting shell services in background...");
    remove_file_if_exists(Path::new("/tmp/babydra-switcher.socket"));
    let local_bin = get_local_bin_dir();

    for daemon in daemons {
        let parts: Vec<&str> = daemon.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        let bin_name = parts[0];
        let bin_path = local_bin.join(bin_name);

        if bin_path.exists() {
            let args: Vec<&str> = if parts.len() > 1 {
                parts[1..].to_vec()
            } else {
                Vec::new()
            };
            match spawn_daemon(&bin_path, &args, bin_name) {
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
