//! MPRIS media player controller utilities.

use std::process::Command;

/// Launches `playerctl` with the given argument slice, returning stdout as an Option string.
pub fn run_playerctl(args: &[&str]) -> Option<String> {
    let output = Command::new("playerctl")
        .args(args)
        .output()
        .ok()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !stdout.is_empty() {
            return Some(stdout);
        }
    }
    None
}

/// Decodes %-encoded (URL-encoded) string characters back into standard text.
pub fn decode_uri(uri: &str) -> String {
    let mut decoded = String::new();
    let mut chars = uri.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            if let (Some(h1), Some(h2)) = (chars.next(), chars.next()) {
                if let Some(hex) = u8::from_str_radix(&format!("{}{}", h1, h2), 16).ok() {
                    decoded.push(hex as char);
                    continue;
                }
            }
        }
        decoded.push(c);
    }
    decoded
}
