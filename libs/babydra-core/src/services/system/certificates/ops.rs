use crate::error::{CoreError, CoreResult};
use crate::models::CertInfo;
use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

pub const ANCHORS_DIR: &str = "/etc/ca-certificates/trust-source/anchors";

/// Lists `ca certificates`.
pub fn list_ca_certificates() -> Vec<CertInfo> {
    let mut certs = Vec::new();
    if let Ok(dir) = fs::read_dir(ANCHORS_DIR) {
        for entry in dir.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    certs.push(CertInfo {
                        filename: filename.to_string(),
                        path: path.to_string_lossy().to_string(),
                    });
                }
            }
        }
    }
    certs.sort_by(|a, b| a.filename.cmp(&b.filename));
    certs
}

/// Add ca certificate.
pub fn add_ca_certificate(src_path: &str, filename: &str, sudo_password: &str) -> CoreResult<()> {
    let cmd_str = format!(
        "mkdir -p /etc/ca-certificates/trust-source/anchors && cp '{}' '/etc/ca-certificates/trust-source/anchors/{}' && update-ca-trust",
        src_path, filename
    );
    execute_sudo_command(&cmd_str, sudo_password)
}

/// Delete ca certificate.
pub fn delete_ca_cert(filename: &str, sudo_password: &str) -> CoreResult<()> {
    let cmd_str = format!(
        "rm -f '/etc/ca-certificates/trust-source/anchors/{}' && update-ca-trust",
        filename
    );
    execute_sudo_command(&cmd_str, sudo_password)
}

fn execute_sudo_command(cmd_str: &str, password: &str) -> CoreResult<()> {
    let mut child = Command::new("sudo")
        .arg("-S")
        .arg("sh")
        .arg("-c")
        .arg(cmd_str)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to execute sudo: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = writeln!(stdin, "{}", password);
        let _ = stdin.flush();
    }

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Failed to wait for process: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(if err_msg.trim().is_empty() {
            CoreError::msg("Permission denied or incorrect password")
        } else {
            CoreError::msg(err_msg.trim().to_string())
        })
    }
}
