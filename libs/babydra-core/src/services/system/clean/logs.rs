//! System journal logs sizing and cleanup.

use super::helper::is_dir_writable;
use std::fs;
use std::path::Path;

/// Returns the current `journal logs size`.
pub fn get_journal_logs_size() -> u64 {
    let journal_dir = Path::new("/var/log/journal");
    let mut size = 0;
    if journal_dir.exists() {
        if let Ok(entries) = fs::read_dir(journal_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() && is_dir_writable(&path) {
                        if let Ok(sub_entries) = fs::read_dir(&path) {
                            for sub_entry in sub_entries {
                                if let Ok(sub_entry) = sub_entry {
                                    let sub_path = sub_entry.path();
                                    if sub_path.is_file() {
                                        if let Ok(meta) = sub_entry.metadata() {
                                            let file_name = sub_path
                                                .file_name()
                                                .unwrap_or_default()
                                                .to_string_lossy();
                                            if file_name.contains('@') {
                                                size += meta.len();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    size
}

/// Removes `journal logs` and returns the result.
pub fn remove_journal_logs() -> u64 {
    let journal_dir = Path::new("/var/log/journal");
    let mut freed = 0;
    if journal_dir.exists() {
        if let Ok(entries) = fs::read_dir(journal_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() && is_dir_writable(&path) {
                        if let Ok(sub_entries) = fs::read_dir(&path) {
                            for sub_entry in sub_entries {
                                if let Ok(sub_entry) = sub_entry {
                                    let sub_path = sub_entry.path();
                                    if sub_path.is_file() {
                                        if let Ok(meta) = sub_entry.metadata() {
                                            let size = meta.len();
                                            let file_name = sub_path
                                                .file_name()
                                                .unwrap_or_default()
                                                .to_string_lossy();
                                            if file_name.contains('@') {
                                                if fs::remove_file(sub_path).is_ok() {
                                                    freed += size;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    freed
}
