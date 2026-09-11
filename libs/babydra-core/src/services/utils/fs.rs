use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Returns the user's home directory as a `PathBuf`.
pub fn get_home_path() -> PathBuf {
    dirs::home_dir()
        .or_else(|| std::env::var("HOME").ok().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("/tmp"))
}

/// Returns the `~/.config/babydra` directory path.
pub fn get_babydra_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| get_home_path().join(".config"))
        .join("babydra")
}

/// Creates a directory and all parent components if they do not exist.
pub fn ensure_dir_exists(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Sets Unix file permissions using an octal mode (e.g. 0o755, 0o644).
/// If metadata is available, preserves special bits outside the 0o777 mask.
#[cfg(unix)]
pub fn set_unix_mode(path: &Path, mode: u32) -> std::io::Result<()> {
    if let Ok(metadata) = fs::metadata(path) {
        use std::os::unix::fs::MetadataExt;
        let final_mode = (metadata.mode() & !0o777) | mode;
        let mut perms = metadata.permissions();
        perms.set_mode(final_mode);
        fs::set_permissions(path, perms)
    } else {
        let perms = fs::Permissions::from_mode(mode);
        fs::set_permissions(path, perms)
    }
}

#[cfg(not(unix))]
pub fn set_unix_mode(_path: &Path, _mode: u32) -> std::io::Result<()> {
    Ok(())
}

/// Formats a byte count into a human-readable size string (B, KB, MB, GB).
pub fn format_bytes(bytes: u64) -> String {
    let kb = bytes as f64 / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;
    if gb >= 1.0 {
        format!("{:.2} GB", gb)
    } else if mb >= 1.0 {
        format!("{:.2} MB", mb)
    } else if kb >= 1.0 {
        format!("{:.2} KB", kb)
    } else {
        format!("{} B", bytes)
    }
}

/// Reads a file into a trimmed String, returning None if read failed or file is empty.
pub fn read_trimmed_string(path: impl AsRef<Path>) -> Option<String> {
    fs::read_to_string(path.as_ref())
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Reads a sysfs / procfs numerical file into a parsed `u32`.
pub fn read_sysfs_u32(path: impl AsRef<Path>) -> Option<u32> {
    read_trimmed_string(path).and_then(|s| s.parse::<u32>().ok())
}

/// Reads a sysfs / procfs numerical file into a parsed `f64`.
pub fn read_sysfs_f64(path: impl AsRef<Path>) -> Option<f64> {
    read_trimmed_string(path).and_then(|s| s.parse::<f64>().ok())
}

/// Appends a configuration line to a file if not already present.
pub fn append_line_if_missing(file: &Path, line: &str) -> std::io::Result<()> {
    if file.exists() {
        if let Ok(content) = fs::read_to_string(file) {
            if content.contains(line) {
                return Ok(());
            }
            let prefix = if content.ends_with('\n') { "" } else { "\n" };
            fs::write(file, format!("{content}{prefix}{line}\n"))?;
        }
    } else {
        fs::write(file, format!("{line}\n"))?;
    }
    Ok(())
}
