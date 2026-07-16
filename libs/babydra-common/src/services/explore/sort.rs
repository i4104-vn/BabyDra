use crate::models::explore::FileEntry;

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
