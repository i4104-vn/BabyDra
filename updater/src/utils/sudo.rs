use std::io::Write;
use std::process::{Command, Stdio};

/// Checks if sudo credentials are currently cached and valid without password prompt
pub fn check_sudo_cached() -> bool {
    Command::new("sudo")
        .arg("-n")
        .arg("true")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Validates a sudo password by running `sudo -S -v`
/// Returns true if password is valid and sudo timestamp is updated
pub fn validate_sudo_password(password: &str) -> bool {
    let mut child = match Command::new("sudo")
        .args(["-S", "-v"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "{}", password);
    }

    child.wait().map(|s| s.success()).unwrap_or(false)
}
