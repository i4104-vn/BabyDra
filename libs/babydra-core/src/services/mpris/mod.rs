//! MPRIS media player controller utilities.

use std::process::Command;

/// Launches `playerctl` with the given argument slice, returning stdout as an Option string.
pub fn run_playerctl(args: &[&str]) -> Option<String> {
    let output = Command::new("playerctl").args(args).output().ok()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !stdout.is_empty() {
            return Some(stdout);
        }
    }
    None
}

/// Decodes %-encoded (URL-encoded) string characters back into standard text with full UTF-8 support.
pub fn decode_uri(uri: &str) -> String {
    let mut bytes = Vec::new();
    let mut bytes_iter = uri.as_bytes().iter();
    while let Some(&b) = bytes_iter.next() {
        if b == b'%' {
            let mut hex = Vec::with_capacity(2);
            if let Some(&h1) = bytes_iter.next() {
                hex.push(h1);
            }
            if let Some(&h2) = bytes_iter.next() {
                hex.push(h2);
            }
            if hex.len() == 2 {
                if let Ok(s) = std::str::from_utf8(&hex) {
                    if let Ok(val) = u8::from_str_radix(s, 16) {
                        bytes.push(val);
                        continue;
                    }
                }
            }
            bytes.push(b'%');
            bytes.extend(hex);
        } else {
            bytes.push(b);
        }
    }
    String::from_utf8_lossy(&bytes).to_string()
}
