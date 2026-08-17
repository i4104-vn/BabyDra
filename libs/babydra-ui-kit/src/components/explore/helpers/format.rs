use chrono::{DateTime, Local};
use std::time::SystemTime;

/// Formats a byte size into a human-readable string (B, KB, MB, GB).
pub fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

/// Formats a SystemTime into a human-readable local date-time string.
pub fn format_date(mtime: SystemTime) -> String {
    let datetime: DateTime<Local> = mtime.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
