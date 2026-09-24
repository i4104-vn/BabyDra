use crate::utils::system::get_home_dir;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

/// Spawns a background daemon detached from the terminal session and process group.
/// Standard input is closed (/dev/null).
/// Standard output and error are redirected to ~/.cache/babydra/<log_name>.log
/// so that the terminal is never spammed and the process will continue running
/// without interruption after the updater exits.
pub fn spawn_daemon(
    binary_path: &Path,
    args: &[&str],
    log_name: &str,
) -> Result<(), String> {
    if !binary_path.is_file() {
        return Err(format!("Daemon binary not found at: {}", binary_path.display()));
    }

    let log_dir = get_home_dir().join(".cache/babydra");
    let _ = std::fs::create_dir_all(&log_dir);
    let log_file_path = log_dir.join(format!("{}.log", log_name));

    let (stdout_target, stderr_target) = match std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
    {
        Ok(file) => {
            let file_err = file.try_clone().map(Stdio::from).unwrap_or_else(|_| Stdio::null());
            (Stdio::from(file), file_err)
        }
        Err(_) => (Stdio::null(), Stdio::null()),
    };

    let mut cmd = Command::new(binary_path);
    cmd.args(args)
        .stdin(Stdio::null())
        .stdout(stdout_target)
        .stderr(stderr_target);

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            cmd.pre_exec(|| {
                // Detach from controlling terminal into its own session and process group
                if libc::setsid() == -1 {
                    libc::setpgid(0, 0);
                }
                Ok(())
            });
        }
    }

    cmd.spawn()
        .map_err(|e| format!("Failed to spawn {}: {}", binary_path.display(), e))?;

    Ok(())
}

/// Kills a process by exact name using killall
pub fn kill_process(name: &str) {
    let _ = Command::new("killall")
        .arg(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

/// Kills a list of processes by name
pub fn kill_processes(names: &[String]) {
    for name in names {
        kill_process(name);
    }
    std::thread::sleep(Duration::from_millis(150));
}

/// Kills processes matching a pattern, optionally restricted to a specific user
pub fn pkill(pattern: &str, user: Option<&str>) {
    let mut cmd = Command::new("pkill");
    if let Some(u) = user {
        cmd.args(["-u", u]);
    }
    cmd.arg("-f").arg(pattern);
    let _ = cmd.stdout(Stdio::null()).stderr(Stdio::null()).status();
}

/// Checks if a process with the specified name is currently running
pub fn is_process_running(name: &str) -> bool {
    Command::new("pgrep")
        .arg("-x")
        .arg(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Checks if a command is available on the system PATH
pub fn command_exists(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
