use std::process::{Command, Stdio};

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
