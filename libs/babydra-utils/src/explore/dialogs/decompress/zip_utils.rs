use std::path::PathBuf;
use crate::explore::dialogs::shared::shell_quote;

pub async fn is_zip_encrypted(archive_path: &PathBuf) -> bool {
    let parent_dir = match archive_path.parent() {
        Some(p) => p.to_path_buf(),
        None => return false,
    };
    
    let filename = archive_path.file_name().unwrap().to_string_lossy().to_string();
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

pub async fn check_password_correct(archive_path: &PathBuf, password: &str) -> bool {
    let parent_dir = match archive_path.parent() {
        Some(p) => p.to_path_buf(),
        None => return false,
    };
    let filename = archive_path.file_name().unwrap().to_string_lossy().to_string();
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
