use crate::models::explore::file_entry::FileEntry;
use chrono::{DateTime, Local, Duration};

/// Helper to determine the group/category name of a file entry.
pub fn get_group_name(entry: &FileEntry, sort_mode: &str) -> String {
    if sort_mode == "date" {
        if let Some(modified) = entry.modified {
            let datetime: DateTime<Local> = modified.into();
            let now = Local::now();
            let date_naive = datetime.date_naive();
            let now_naive = now.date_naive();
            let date_str = datetime.format(" (%d/%m)").to_string();
            if date_naive == now_naive {
                format!("Today{}", date_str)
            } else if date_naive == now_naive - Duration::days(1) {
                format!("Yesterday{}", date_str)
            } else {
                let diff = (now_naive - date_naive).num_days();
                if diff >= 2 && diff <= 7 {
                    format!("{}{}", datetime.format("%A"), date_str)
                } else if diff > 7 {
                    "Older than a week".to_string()
                } else {
                    format!("Today{}", date_str)
                }
            }
        } else {
            "Unknown Date".to_string()
        }
    } else { // "group"
        if matches!(entry.file_type, crate::models::explore::file_entry::FileType::Directory) {
            "Folders".to_string()
        } else {
            match entry.path.extension() {
                Some(ext) => format!("{} Files", ext.to_string_lossy().to_uppercase()),
                None => "Other Files".to_string(),
            }
        }
    }
}
