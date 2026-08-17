use crate::models::explore::FileEntry;

/// Sort entries.
pub fn sort_entries(entries: &mut [FileEntry], sort_mode: &str) {
    entries.sort_by(|a, b| {
        let a_is_dir = matches!(a.file_type, crate::models::explore::FileType::Directory);
        let b_is_dir = matches!(b.file_type, crate::models::explore::FileType::Directory);

        if sort_mode == "date" {
            let get_weight = |e: &FileEntry| -> u32 {
                if let Some(modified) = e.modified {
                    let datetime: chrono::DateTime<chrono::Local> = modified.into();
                    let now = chrono::Local::now();
                    let date_naive = datetime.date_naive();
                    let now_naive = now.date_naive();
                    if date_naive == now_naive {
                        0
                    } else if date_naive == now_naive - chrono::Duration::days(1) {
                        1
                    } else {
                        let diff = (now_naive - date_naive).num_days();
                        if diff >= 2 && diff <= 7 {
                            diff as u32
                        } else if diff > 7 {
                            8
                        } else {
                            0
                        }
                    }
                } else {
                    9
                }
            };

            let w_a = get_weight(a);
            let w_b = get_weight(b);

            if w_a != w_b {
                return w_a.cmp(&w_b);
            }

            // Within the same group/category, directories always go first!
            match (a_is_dir, b_is_dir) {
                (true, false) => return std::cmp::Ordering::Less,
                (false, true) => return std::cmp::Ordering::Greater,
                _ => {}
            }

            // Within the same type and category, sort by exact modified time descending (newest first)
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
        } else {
            // Non-date sort modes (e.g. name or auto):
            // Always put directories at the top of the entire list!
            match (a_is_dir, b_is_dir) {
                (true, false) => return std::cmp::Ordering::Less,
                (false, true) => return std::cmp::Ordering::Greater,
                _ => {}
            }
        }

        // Fallback sorting (alphabetical / extension)
        match (a_is_dir, b_is_dir) {
            (true, true) => a
                .display_name
                .to_lowercase()
                .cmp(&b.display_name.to_lowercase()),
            (false, false) => {
                let ext_a = a
                    .path
                    .extension()
                    .map(|e| e.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                let ext_b = b
                    .path
                    .extension()
                    .map(|e| e.to_string_lossy().to_lowercase())
                    .unwrap_or_default();
                let cmp_type = ext_a.cmp(&ext_b);
                if cmp_type == std::cmp::Ordering::Equal {
                    a.display_name
                        .to_lowercase()
                        .cmp(&b.display_name.to_lowercase())
                } else {
                    cmp_type
                }
            }
            _ => std::cmp::Ordering::Equal,
        }
    });
}
