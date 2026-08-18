//! Drive parent lookup and human-readable capacity formatting helpers.

/// Returns the current `parent drive`.
pub fn get_parent_drive(filesystem: &str) -> String {
    if filesystem.starts_with("/dev/sd") {
        if filesystem.len() >= 8 {
            return filesystem[0..8].to_string();
        }
    } else if filesystem.starts_with("/dev/nvme") {
        if let Some(p_idx) = filesystem.rfind('p') {
            if p_idx > 9 {
                return filesystem[0..p_idx].to_string();
            }
        }
    }
    filesystem.to_string()
}

/// Formats a drive capacity given in kilobytes as a human-readable string.
pub fn format_disk_size(kb: u64) -> String {
    let gb = kb as f64 / 1024.0 / 1024.0;
    if gb >= 1000.0 {
        format!("{:.1} TB", gb / 1024.0)
    } else {
        format!("{:.1} GB", gb)
    }
}
