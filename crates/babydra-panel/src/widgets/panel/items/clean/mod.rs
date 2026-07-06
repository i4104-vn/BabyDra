pub mod render;
pub mod cache;
pub mod logs;
pub mod temp;

use std::fs;
use std::path::Path;

/// Calculates the size of a directory recursively using native Rust filesystem APIs.
pub fn get_dir_size(path: &str) -> u64 {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let real_path = path.replace("~", &home);
    get_dir_size_native(&real_path)
}

pub fn get_dir_size_native<P: AsRef<Path>>(path: P) -> u64 {
    let mut total = 0;
    if let Ok(metadata) = fs::symlink_metadata(&path) {
        let file_type = metadata.file_type();
        if file_type.is_dir() {
            if let Ok(entries) = fs::read_dir(&path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        total += get_dir_size_native(entry.path());
                    }
                }
            }
        } else if file_type.is_file() {
            total += metadata.len();
        }
    }
    total
}

/// Retrieves the total size of orphaned packages natively by parsing the local pacman database.
pub fn get_orphans_size() -> u64 {
    let mut total_size = 0;
    let local_db = "/var/lib/pacman/local";
    if let Ok(entries) = fs::read_dir(local_db) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let desc_path = path.join("desc");
                    if desc_path.exists() {
                        if let Ok(content) = fs::read_to_string(&desc_path) {
                            let mut size = 0u64;
                            let mut is_dep = false;
                            let mut has_required_by = false;

                            let lines: Vec<&str> = content.lines().collect();
                            let mut i = 0;
                            while i < lines.len() {
                                let line = lines[i].trim();
                                if line == "%SIZE%" {
                                    if i + 1 < lines.len() {
                                        size = lines[i + 1].trim().parse::<u64>().unwrap_or(0);
                                    }
                                } else if line == "%REASON%" {
                                    if i + 1 < lines.len() {
                                        let reason = lines[i + 1].trim();
                                        if reason == "1" {
                                            is_dep = true;
                                        }
                                    }
                                } else if line == "%REQUIRED_BY%" {
                                    let mut j = i + 1;
                                    while j < lines.len() {
                                        let next_line = lines[j].trim();
                                        if next_line.starts_with('%') {
                                            break;
                                        }
                                        if !next_line.is_empty() {
                                            has_required_by = true;
                                        }
                                        j += 1;
                                    }
                                }
                                i += 1;
                            }

                            if is_dep && !has_required_by {
                                total_size += size;
                            }
                        }
                    }
                }
            }
        }
    }
    total_size
}

/// Native system cleanup using standard library file operations.
/// Returns the total number of bytes successfully freed by calling submodules.
pub fn clean_all_native() -> u64 {
    let mut freed_bytes = 0;
    freed_bytes += cache::remove_user_cache();
    freed_bytes += cache::remove_pacman_cache();
    freed_bytes += logs::remove_journal_logs();
    freed_bytes += temp::remove_trash();
    freed_bytes
}

/// Formats raw bytes into a human readable size.
pub fn format_bytes(bytes: u64) -> String {
    let kb = bytes as f64 / 1024.0;
    let mb = kb / 1024.0;
    let gb = mb / 1024.0;
    if gb >= 1.0 {
        format!("{:.2} GB", gb)
    } else if mb >= 1.0 {
        format!("{:.2} MB", mb)
    } else if kb >= 1.0 {
        format!("{:.2} KB", kb)
    } else {
        format!("{} B", bytes)
    }
}

/// Helper function to check if a directory is writable by creating/removing a temporary file.
pub fn is_dir_writable<P: AsRef<Path>>(path: P) -> bool {
    let path_ref = path.as_ref();
    if !path_ref.exists() || !path_ref.is_dir() {
        return false;
    }
    let test_file = path_ref.join(".babydra_write_test");
    if let Ok(_f) = fs::File::create(&test_file) {
        let _ = fs::remove_file(test_file);
        true
    } else {
        false
    }
}

/// Recursively calculates cleanable size, checking permissions along the traversal.
pub fn get_cleanable_size_recursive<P: AsRef<Path>>(path: P) -> u64 {
    let path_ref = path.as_ref();
    if !path_ref.exists() {
        return 0;
    }
    let mut size = 0;
    if path_ref.is_dir() {
        if is_dir_writable(path_ref) {
            if let Ok(entries) = fs::read_dir(path_ref) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let sub_path = entry.path();
                        if sub_path.is_file() {
                            if let Ok(meta) = sub_path.metadata() {
                                size += meta.len();
                            }
                        } else if sub_path.is_dir() {
                            size += get_cleanable_size_recursive(&sub_path);
                        }
                    }
                }
            }
        }
    } else if path_ref.is_file() {
        if let Some(parent) = path_ref.parent() {
            if is_dir_writable(parent) {
                if let Ok(meta) = path_ref.metadata() {
                    size += meta.len();
                }
            }
        }
    }
    size
}

/// Recursively cleans directory contents, deleting only files we have permission to remove, and sums actual freed size.
pub fn clean_path_recursive<P: AsRef<Path>>(path: P) -> u64 {
    let path_ref = path.as_ref();
    if !path_ref.exists() {
        return 0;
    }
    let mut freed = 0;
    if path_ref.is_dir() {
        if is_dir_writable(path_ref) {
            let mut all_sub_deleted = true;
            if let Ok(entries) = fs::read_dir(path_ref) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let sub_path = entry.path();
                        if sub_path.is_file() {
                            if let Ok(meta) = sub_path.metadata() {
                                let len = meta.len();
                                if fs::remove_file(&sub_path).is_ok() {
                                    freed += len;
                                } else {
                                    all_sub_deleted = false;
                                }
                            }
                        } else if sub_path.is_dir() {
                            let sub_freed = clean_path_recursive(&sub_path);
                            freed += sub_freed;
                            if sub_path.exists() {
                                all_sub_deleted = false;
                            }
                        }
                    }
                }
            }
            if all_sub_deleted {
                let _ = fs::remove_dir(path_ref);
            }
        }
    } else if path_ref.is_file() {
        if let Some(parent) = path_ref.parent() {
            if is_dir_writable(parent) {
                if let Ok(meta) = path_ref.metadata() {
                    let len = meta.len();
                    if fs::remove_file(path_ref).is_ok() {
                        freed += len;
                    }
                }
            }
        }
    }
    freed
}
