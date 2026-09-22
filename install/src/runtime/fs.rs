use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn safe_copy_binary(src: &Path, dst: &Path) -> Result<()> {
    if let Some(parent) = dst.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {:?}", parent))?;
    }

    // Atomic replace via temp file to avoid ETXTBSY on running binaries
    let temp_dst = dst.with_extension(format!("tmp.{}", std::process::id()));

    fs::copy(src, &temp_dst)
        .with_context(|| format!("Failed to copy {:?} to {:?}", src, temp_dst))?;

    let mut perms = fs::metadata(&temp_dst)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&temp_dst, perms)
        .with_context(|| format!("Failed to set permissions 0755 on {:?}", temp_dst))?;

    fs::rename(&temp_dst, dst)
        .with_context(|| format!("Failed to replace {:?} with {:?}", dst, temp_dst))?;

    Ok(())
}

pub fn copy_recursive(src: &Path, dst: &Path) -> Result<()> {
    if !src.exists() {
        return Ok(());
    }
    if src.is_file() {
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(src, dst)?;
        return Ok(());
    }

    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let target = dst.join(entry.file_name());
        if path.is_dir() {
            copy_recursive(&path, &target)?;
        } else {
            fs::copy(&path, &target)?;
        }
    }
    Ok(())
}
