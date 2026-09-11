use std::io::Write;
use std::process::{Command, Output, Stdio};

/// Retrieves the current user's home directory path as a String.
/// Uses `dirs::home_dir()` or `$HOME` environment variable, falling back to "/tmp".
pub fn get_home_dir() -> String {
    dirs::home_dir()
        .map(|p| p.to_string_lossy().to_string())
        .or_else(|| std::env::var("HOME").ok())
        .unwrap_or_else(|| "/tmp".to_string())
}

/// Helper to run a command and return its stdout as a String if successful.
/// Trims the output and handles utf8 lossy conversion.
pub fn run_cmd(args: &[&str]) -> Option<String> {
    if args.is_empty() {
        return None;
    }
    Command::new(args[0])
        .args(&args[1..])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

/// Helper to run a command and return whether it exited successfully.
pub fn run_cmd_bool(args: &[&str]) -> bool {
    if args.is_empty() {
        return false;
    }
    Command::new(args[0])
        .args(&args[1..])
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Helper to run a command and return the raw process Output.
pub fn run_cmd_output(args: &[&str]) -> std::io::Result<Output> {
    if args.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "args cannot be empty",
        ));
    }
    Command::new(args[0]).args(&args[1..]).output()
}

/// Executes a privileged command via `sudo -S` feeding the given password to stdin.
/// Optionally writes additional input data into stdin after the password.
pub fn run_sudo(
    password: &str,
    cmd: &str,
    args: &[&str],
    input_after_pwd: Option<&[u8]>,
) -> Result<(), String> {
    let mut child = Command::new("sudo")
        .arg("-S")
        .arg(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn sudo {}: {}", cmd, e))?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "{}", password);
        if let Some(extra) = input_after_pwd {
            let _ = stdin.write_all(extra);
        }
        let _ = stdin.flush();
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for process: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let err = String::from_utf8_lossy(&output.stderr);
        Err(err.trim().to_string())
    }
}

/// Helper to terminate processes by name/pattern using `pkill`.
pub fn pkill(pattern: &str, exact: bool) -> bool {
    let flag = if exact { "-x" } else { "-f" };
    Command::new("pkill")
        .args([flag, pattern])
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Helper to send a specific signal to processes using `pkill`.
pub fn pkill_signal(signal: &str, pattern: &str, exact: bool) -> bool {
    let flag = if exact { "-x" } else { "-f" };
    Command::new("pkill")
        .args([signal, flag, pattern])
        .output()
        .map(|out| out.status.success())
        .unwrap_or(false)
}

/// Spawns a background shell command (`sh -c "<script>"`), ignoring errors.
pub fn spawn_sh(script: &str) {
    let _ = Command::new("sh").arg("-c").arg(script).spawn();
}
