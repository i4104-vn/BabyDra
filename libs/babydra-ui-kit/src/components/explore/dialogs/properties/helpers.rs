use std::path::{Path, PathBuf};

/// Returns the current `permissions string`.
pub fn get_perm_string(mode: u32) -> String {
    let mut s = String::with_capacity(9);

    // Owner
    s.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o100 != 0 { 'x' } else { '-' });

    // Group
    s.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o010 != 0 { 'x' } else { '-' });

    // Others
    s.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    s.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    s.push(if mode & 0o001 != 0 { 'x' } else { '-' });

    s
}

/// Count dir contents recursive.
pub fn count_dir_contents(path: &Path) -> (usize, usize) {
    let mut files = 0;
    let mut folders = 0;
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.filter_map(Result::ok) {
            if let Ok(meta) = entry.metadata() {
                if meta.is_dir() {
                    folders += 1;
                    let (sub_files, sub_folders) = count_dir_contents(&entry.path());
                    files += sub_files;
                    folders += sub_folders;
                } else {
                    files += 1;
                }
            }
        }
    }
    (files, folders)
}

/// Count dialog height.
pub fn count_dialog_height(target_paths: &[PathBuf]) -> i32 {
    if target_paths.len() == 1 {
        if let Ok(meta) = std::fs::metadata(&target_paths[0]) {
            if meta.is_dir() {
                return 530;
            }
        }
        490
    } else {
        250
    }
}
