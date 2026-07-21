use std::path::{Path, PathBuf};
use std::process::Stdio;

/// Properly quote a string for use in a shell single-quoted argument.
pub fn shell_quote(s: &str) -> String {
    format!("'{}'", s.replace('\'', "'\\''"))
}

/// Executes a custom context menu shell command string.
pub fn execute_custom_command(command_str: &str) -> std::io::Result<std::process::Child> {
    std::process::Command::new("sh")
        .arg("-c")
        .arg(command_str)
        .spawn()
}

/// Spawns a Tokio process command for compression (ZIP or TAR).
pub fn spawn_compress_command(
    parent_dir: &Path,
    archive_filename: &str,
    target_files: &[String],
    is_zip: bool,
) -> std::io::Result<tokio::process::Child> {
    let cmd_str = if is_zip {
        format!(
            "zip -r {} {}",
            shell_quote(archive_filename),
            target_files.iter().map(|f| shell_quote(f)).collect::<Vec<_>>().join(" ")
        )
    } else {
        format!(
            "tar -cvf {} {}",
            shell_quote(archive_filename),
            target_files.iter().map(|f| shell_quote(f)).collect::<Vec<_>>().join(" ")
        )
    };

    let mut cmd = tokio::process::Command::new("sh");
    cmd.current_dir(parent_dir);
    cmd.arg("-c").arg(&cmd_str);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.spawn()
}

/// Spawns a Tokio process command for decompression based on file extension.
pub fn spawn_decompress_command(
    parent_dir: &Path,
    filename: &str,
    password: Option<&str>,
) -> std::io::Result<tokio::process::Child> {
    let name_lower = filename.to_lowercase();
    
    let cmd_str = if name_lower.ends_with(".zip") {
        match password {
            Some(pass) => format!("unzip -o -P {} {}", shell_quote(pass), shell_quote(filename)),
            None => format!("unzip -o {}", shell_quote(filename)),
        }
    } else if name_lower.ends_with(".tar.gz") || name_lower.ends_with(".tgz") {
        format!("tar -xzvf {}", shell_quote(filename))
    } else if name_lower.ends_with(".tar.bz2") || name_lower.ends_with(".tbz2") {
        format!("tar -xjvf {}", shell_quote(filename))
    } else if name_lower.ends_with(".tar.xz") || name_lower.ends_with(".txz") {
        format!("tar -xJvf {}", shell_quote(filename))
    } else if name_lower.ends_with(".tar.zst") {
        format!("tar -xavf {}", shell_quote(filename))
    } else if name_lower.ends_with(".rar") {
        format!("unrar x {}", shell_quote(filename))
    } else if name_lower.ends_with(".7z") {
        format!("7z x {}", shell_quote(filename))
    } else {
        format!("tar -xvf {}", shell_quote(filename))
    };

    let mut cmd = tokio::process::Command::new("sh");
    cmd.current_dir(parent_dir);
    cmd.arg("-c").arg(&cmd_str);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    cmd.spawn()
}

/// Asynchronously checks if a ZIP file is encrypted using system unzip command.
pub async fn is_zip_encrypted(archive_path: &PathBuf) -> bool {
    let parent_dir = match archive_path.parent() {
        Some(p) => p.to_path_buf(),
        None => return false,
    };
    
    let filename = archive_path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let quoted = shell_quote(&filename);
    let cmd_str = format!("unzip -t -P '' {}", quoted);
    
    let output = tokio::task::spawn_blocking(move || {
        std::process::Command::new("sh")
            .current_dir(parent_dir)
            .arg("-c")
            .arg(&cmd_str)
            .output()
    }).await;
    
    if let Ok(Ok(out)) = output {
        let stderr = String::from_utf8_lossy(&out.stderr).to_lowercase();
        let stdout = String::from_utf8_lossy(&out.stdout).to_lowercase();
        
        stdout.contains("password") || stderr.contains("password") ||
        stdout.contains("incorrect password") || stderr.contains("incorrect password") ||
        stdout.contains("encrypted") || stderr.contains("encrypted") ||
        out.status.code() == Some(82) || out.status.code() == Some(81)
    } else {
        false
    }
}

/// Asynchronously checks if the password provided for a ZIP archive is correct.
pub async fn check_zip_password(archive_path: &PathBuf, password: &str) -> bool {
    let parent_dir = match archive_path.parent() {
        Some(p) => p.to_path_buf(),
        None => return false,
    };
    let filename = archive_path.file_name().unwrap_or_default().to_string_lossy().to_string();
    let cmd_str = format!("unzip -t -P {} {}", shell_quote(password), shell_quote(&filename));
    
    let output = tokio::task::spawn_blocking(move || {
        std::process::Command::new("sh")
            .current_dir(parent_dir)
            .arg("-c")
            .arg(&cmd_str)
            .output()
    }).await;
    
    if let Ok(Ok(out)) = output {
        out.status.success()
    } else {
        false
    }
}
