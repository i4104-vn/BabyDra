use crate::models::explore::FileEntry;

pub fn sort_entries(entries: &mut [FileEntry], sort_mode: &str) {
    entries.sort_by(|a, b| {
        let a_is_dir = matches!(a.file_type, crate::models::explore::FileType::Directory);
        let b_is_dir = matches!(b.file_type, crate::models::explore::FileType::Directory);

        if sort_mode == "date" {
            match (a.modified, b.modified) {
                (Some(ma), Some(mb)) => {
                    if ma != mb {
                        return mb.cmp(&ma); // reverse to get newest first
                    }
                }
                (Some(_), None) => return std::cmp::Ordering::Less,
                (None, Some(_)) => return std::cmp::Ordering::Greater,
                (None, None) => {}
            }
        }

        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (true, true) => {
                a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase())
            }
            (false, false) => {
                let ext_a = a.path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
                let ext_b = b.path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
                let cmp_type = ext_a.cmp(&ext_b);
                if cmp_type == std::cmp::Ordering::Equal {
                    a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase())
                } else {
                    cmp_type
                }
            }
        }
    });
}
