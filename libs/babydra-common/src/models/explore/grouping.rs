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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{set_locale, t};
    use crate::models::explore::file_entry::FileType;
    use std::ffi::OsString;
    use std::path::PathBuf;

    // i18n tests mutate a process-global locale; reuse the i18n module's test lock.
    use crate::i18n::tests::LOCALE_TEST_LOCK;

    fn entry(file_type: FileType) -> FileEntry {
        FileEntry {
            path: PathBuf::from("/tmp/test"),
            name: OsString::from("test"),
            display_name: "test".to_string(),
            file_type,
            mime_type: String::new(),
            size: 0,
            modified: None,
            created: None,
            permissions: 0o644,
            owner: String::new(),
            group: String::new(),
            is_hidden: false,
            icon_name: String::new(),
            thumbnail_path: None,
        }
    }

    #[test]
    fn group_today_uses_locale() {
        let _guard = LOCALE_TEST_LOCK.lock().unwrap();
        set_locale("en");
        let today = chrono::Local::now();
        let mut e = entry(FileType::RegularFile);
        e.modified = Some(today.into());
        let name = get_group_name(&e, "date");
        assert!(name.starts_with("Today"), "got: {name}");
    }

    #[test]
    fn group_folders_uses_locale() {
        let _guard = LOCALE_TEST_LOCK.lock().unwrap();
        set_locale("vi");
        assert_eq!(
            get_group_name(&entry(FileType::Directory), "group"),
            t("explore.group_folders")
        );
    }

    #[test]
    fn group_other_files_uses_locale() {
        let _guard = LOCALE_TEST_LOCK.lock().unwrap();
        set_locale("en");
        assert_eq!(
            get_group_name(&entry(FileType::RegularFile), "group"),
            t("explore.group_other_files")
        );
    }
}
