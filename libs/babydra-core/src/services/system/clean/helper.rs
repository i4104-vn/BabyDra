//! Low-level directory sizing and filesystem cleaning routines.

use std::fs;
use std::path::Path;

/// Returns the current `dir size`.
pub fn get_dir_size(path: &str) -> u64 {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    let real_path = path.replace("~", &home);
    get_dir_size_native(&real_path)
}

/// Returns the total size in bytes of a directory using `du`/`stat` native tools.
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

/// Format bytes.
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

/// Returns `true` if the given path is writable by the current user.
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

/// Recursively sums the cleanable size under `path`.
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

/// Recursively deletes files under `path` and returns the freed bytes.
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
