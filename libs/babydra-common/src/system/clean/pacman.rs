//! Pacman package manager system cache and orphans logic.

use std::fs;
use std::path::Path;
use super::helper::is_dir_writable;

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

pub fn get_pacman_cache_size() -> u64 {
    let pacman_pkg_dir = Path::new("/var/cache/pacman/pkg");
    let mut size = 0;
    if pacman_pkg_dir.exists() && is_dir_writable(pacman_pkg_dir) {
        if let Ok(entries) = fs::read_dir(pacman_pkg_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Ok(meta) = entry.metadata() {
                            size += meta.len();
                        }
                    }
                }
            }
        }
    }
    size
}

pub fn remove_pacman_cache() -> u64 {
    let pacman_pkg_dir = Path::new("/var/cache/pacman/pkg");
    let mut freed = 0;
    if pacman_pkg_dir.exists() && is_dir_writable(pacman_pkg_dir) {
        if let Ok(entries) = fs::read_dir(pacman_pkg_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Ok(meta) = entry.metadata() {
                            let size = meta.len();
                            if fs::remove_file(path).is_ok() {
                                freed += size;
                            }
                        }
                    }
                }
            }
        }
    }
    freed
}
