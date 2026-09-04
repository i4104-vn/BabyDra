//! User account, display name, hostname, and password management services.

use std::io::Write;
use std::process::{Command, Stdio};

/// Information about a user account.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct UserAccountInfo {
    pub username: String,
    pub display_name: String,
    pub uid: u32,
    pub gid: u32,
    pub home_dir: String,
    pub shell: String,
}

/// Parses the user's full name / display name from the GECOS field in `/etc/passwd`.
pub fn parse_gecos_name(gecos: &str) -> String {
    gecos.split(',').next().unwrap_or("").trim().to_string()
}

/// Returns the current logged-in user account information.
pub fn get_user_account_info() -> UserAccountInfo {
    let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());

    if let Ok(content) = std::fs::read_to_string("/etc/passwd") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 7 && parts[0] == username {
                let uid = parts[2].parse::<u32>().unwrap_or(1000);
                let gid = parts[3].parse::<u32>().unwrap_or(1000);
                let gecos_name = parse_gecos_name(parts[4]);
                let display_name = if gecos_name.is_empty() {
                    username.clone()
                } else {
                    gecos_name
                };
                let home_dir = parts[5].to_string();
                let shell = parts[6].to_string();

                return UserAccountInfo {
                    username,
                    display_name,
                    uid,
                    gid,
                    home_dir,
                    shell,
                };
            }
        }
    }

    // Fallback if /etc/passwd parsing fails
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".to_string());
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    UserAccountInfo {
        display_name: username.clone(),
        username,
        uid: 1000,
        gid: 1000,
        home_dir: home,
        shell,
    }
}

/// Executes a command with elevated privileges using `sudo -S`.
fn run_sudo_command(
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
        let err_msg = String::from_utf8_lossy(&output.stderr);
        let trimmed = err_msg.trim();
        if trimmed.is_empty() {
            Err("Authentication failed or permission denied".to_string())
        } else {
            Err(trimmed.to_string())
        }
    }
}

/// Updates the user's real / display name (GECOS field) via `usermod -c`.
pub fn update_display_name(
    username: &str,
    new_name: &str,
    sudo_password: &str,
) -> Result<(), String> {
    let trimmed_name = new_name.trim();
    if trimmed_name.is_empty() {
        return Err("Name cannot be empty".to_string());
    }
    if trimmed_name.contains(':') || trimmed_name.contains('\n') || trimmed_name.contains('\r') {
        return Err("Name contains invalid characters".to_string());
    }

    run_sudo_command(
        sudo_password,
        "usermod",
        &["-c", trimmed_name, username],
        None,
    )
}

/// Changes the user's password using PAM verification and `chpasswd`.
pub fn change_user_password(
    username: &str,
    current_pwd: &str,
    new_pwd: &str,
) -> Result<(), String> {
    if new_pwd.is_empty() {
        return Err("New password cannot be empty".to_string());
    }

    // Step 1: Verify current password via PAM
    if !crate::services::system::auth::verify_password(username, current_pwd) {
        return Err("Current password is incorrect".to_string());
    }

    // Step 2: Update password via `sudo chpasswd`
    let chpasswd_line = format!("{}:{}\n", username, new_pwd);
    run_sudo_command(
        current_pwd,
        "chpasswd",
        &[],
        Some(chpasswd_line.as_bytes()),
    )
}

/// Validates that a hostname conforms to RFC 1123 standards.
pub fn validate_hostname(hostname: &str) -> Result<(), String> {
    let trimmed = hostname.trim();
    if trimmed.is_empty() {
        return Err("Hostname cannot be empty".to_string());
    }
    if trimmed.len() > 63 {
        return Err("Hostname must be 63 characters or fewer".to_string());
    }
    if trimmed.starts_with('-') || trimmed.ends_with('-') {
        return Err("Hostname cannot start or end with a hyphen".to_string());
    }
    for ch in trimmed.chars() {
        if !ch.is_ascii_alphanumeric() && ch != '-' {
            return Err("Hostname can only contain letters, numbers, and hyphens".to_string());
        }
    }
    Ok(())
}

/// Returns the current system hostname.
pub fn get_system_hostname() -> String {
    if let Ok(output) = Command::new("hostnamectl").arg("hostname").output() {
        if output.status.success() {
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !name.is_empty() {
                return name;
            }
        }
    }

    if let Ok(content) = std::fs::read_to_string("/etc/hostname") {
        let name = content.trim().to_string();
        if !name.is_empty() {
            return name;
        }
    }

    if let Ok(content) = std::fs::read_to_string("/proc/sys/kernel/hostname") {
        let name = content.trim().to_string();
        if !name.is_empty() {
            return name;
        }
    }

    "localhost".to_string()
}

/// Updates the system hostname using `hostnamectl hostname <new_hostname>`.
pub fn update_system_hostname(new_hostname: &str, sudo_password: &str) -> Result<(), String> {
    let trimmed = new_hostname.trim();
    validate_hostname(trimmed)?;

    run_sudo_command(
        sudo_password,
        "hostnamectl",
        &["hostname", trimmed],
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gecos_name() {
        assert_eq!(parse_gecos_name("Khoa Tran,,,,"), "Khoa Tran");
        assert_eq!(parse_gecos_name("Alice Smith"), "Alice Smith");
        assert_eq!(parse_gecos_name(""), "");
        assert_eq!(parse_gecos_name("  Bob  ,room,phone"), "Bob");
    }

    #[test]
    fn test_validate_hostname_valid() {
        assert!(validate_hostname("tdkhoa-01").is_ok());
        assert!(validate_hostname("babydra").is_ok());
        assert!(validate_hostname("my-pc-2026").is_ok());
    }

    #[test]
    fn test_validate_hostname_invalid() {
        assert!(validate_hostname("").is_err());
        assert!(validate_hostname("-invalid").is_err());
        assert!(validate_hostname("invalid-").is_err());
        assert!(validate_hostname("has space").is_err());
        assert!(validate_hostname("has.dot").is_err());
        assert!(validate_hostname("has_underscore").is_err());
        assert!(validate_hostname(&"a".repeat(64)).is_err());
    }

    #[test]
    fn test_get_user_account_info_not_empty() {
        let info = get_user_account_info();
        assert!(!info.username.is_empty());
        assert!(!info.display_name.is_empty());
    }
}
