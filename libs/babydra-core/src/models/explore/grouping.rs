use crate::i18n::t;
use crate::models::explore::file_entry::FileEntry;
use chrono::{DateTime, Duration, Local};

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
                format!("{}{}", t("explore.group_today").replace("{}", ""), date_str)
            } else if date_naive == now_naive - Duration::days(1) {
                format!(
                    "{}{}",
                    t("explore.group_yesterday").replace("{}", ""),
                    date_str
                )
            } else {
                let diff = (now_naive - date_naive).num_days();
                if diff >= 2 && diff <= 7 {
                    let weekday = match datetime.format("%A").to_string().as_str() {
                        "Monday" => t("weekday.mon"),
                        "Tuesday" => t("weekday.tue"),
                        "Wednesday" => t("weekday.wed"),
                        "Thursday" => t("weekday.thu"),
                        "Friday" => t("weekday.fri"),
                        "Saturday" => t("weekday.sat"),
                        _ => t("weekday.sun"),
                    };
                    format!("{}{}", weekday, date_str)
                } else if diff > 7 {
                    t("explore.group_older_week")
                } else {
                    format!("{}{}", t("explore.group_today").replace("{}", ""), date_str)
                }
            }
        } else {
            t("explore.group_unknown_date")
        }
    } else {
        // "group"
        if matches!(
            entry.file_type,
            crate::models::explore::file_entry::FileType::Directory
        ) {
            t("explore.group_folders")
        } else {
            match entry.path.extension() {
                Some(ext) => {
                    t("explore.group_files").replace("{}", &ext.to_string_lossy().to_uppercase())
                }
                None => t("explore.group_other_files"),
            }
        }
    }
}
