use std::process::Command;

/// Retrieves the current user's home directory path.
/// Defaults to "/home/i4104" if $HOME is not set.
pub fn get_home_dir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/home/i4104".to_string())
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
