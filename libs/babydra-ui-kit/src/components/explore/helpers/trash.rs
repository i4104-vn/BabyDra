use babydra_core::CoreError;
use std::path::{Path, PathBuf};

/// Check if a path is in Trash.
pub fn is_in_trash(path: &Path) -> bool {
    path.to_string_lossy().contains("Trash/files")
}

/// Restore a file or folder from Trash to its original path based on .trashinfo.
pub async fn restore_from_trash(trash_file_path: PathBuf) -> Result<(), CoreError> {
    let file_name = trash_file_path
        .file_name()
        .ok_or_else(|| CoreError::Invalid("Invalid file name".into()))?;
    let trash_dir = trash_file_path
        .parent()
        .ok_or_else(|| CoreError::Invalid("Invalid parent directory".into()))?;
    let trash_root = trash_dir
        .parent()
        .ok_or_else(|| CoreError::Invalid("Invalid trash root".into()))?;
    let info_dir = trash_root.join("info");

    let info_file_name = format!("{}.trashinfo", file_name.to_string_lossy());
    let info_path = info_dir.join(info_file_name);

    if !info_path.exists() {
        return Err(CoreError::NotFound("Trash info file does not exist".into()));
    }

    let content = std::fs::read_to_string(&info_path)?;
    let mut original_path_str = None;
    for line in content.lines() {
        if line.starts_with("Path=") {
            let path_part = &line["Path=".len()..];
            original_path_str = Some(path_part.to_string());
            break;
        }
    }

    let original_path_str = original_path_str
        .ok_or_else(|| CoreError::Invalid("Path field not found in trashinfo".into()))?;
    let decoded_path_str = percent_decode(&original_path_str);
    let dest_path = PathBuf::from(decoded_path_str);

    if let Some(parent) = dest_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    babydra_core::move_path(trash_file_path.clone(), dest_path)
        .await
        .map_err(|e| CoreError::Message(e.to_string()))?;
    let _ = std::fs::remove_file(info_path);

    Ok(())
}

fn percent_decode(s: &str) -> String {
    let mut bytes = Vec::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next().unwrap_or('0');
            let h2 = chars.next().unwrap_or('0');
            let hex_str = format!("{}{}", h1, h2);
            if let Ok(b) = u8::from_str_radix(&hex_str, 16) {
                bytes.push(b);
            }
        } else {
            bytes.push(c as u8);
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}
