use crate::i18n::trans;
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
                format!("{}{}", trans("explore.group_today").replace("{}", ""), date_str)
            } else if date_naive == now_naive - Duration::days(1) {
                format!(
                    "{}{}",
                    trans("explore.group_yesterday").replace("{}", ""),
                    date_str
                )
            } else {
                let diff = (now_naive - date_naive).num_days();
                if diff >= 2 && diff <= 7 {
                    let weekday = match datetime.format("%A").to_string().as_str() {
                        "Monday" => trans("weekday.mon"),
                        "Tuesday" => trans("weekday.tue"),
                        "Wednesday" => trans("weekday.wed"),
                        "Thursday" => trans("weekday.thu"),
                        "Friday" => trans("weekday.fri"),
                        "Saturday" => trans("weekday.sat"),
                        _ => trans("weekday.sun"),
                    };
                    format!("{}{}", weekday, date_str)
                } else if diff > 7 {
                    trans("explore.group_older_week")
                } else {
                    format!("{}{}", trans("explore.group_today").replace("{}", ""), date_str)
                }
            }
        } else {
            trans("explore.group_unknown_date")
        }
    } else {
        // "group"
        if matches!(
            entry.file_type,
            crate::models::explore::file_entry::FileType::Directory
        ) {
            trans("explore.group_folders")
        } else {
            match entry.path.extension() {
                Some(ext) => {
                    trans("explore.group_files").replace("{}", &ext.to_string_lossy().to_uppercase())
                }
                None => trans("explore.group_other_files"),
            }
        }
    }
}
