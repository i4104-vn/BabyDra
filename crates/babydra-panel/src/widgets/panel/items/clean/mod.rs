pub mod render;

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

/// Retrieves the size of journal logs natively.
pub fn get_journal_size() -> u64 {
    get_dir_size_native("/var/log/journal")
}

/// Retrieves the size of the user's trash bin.
pub fn get_trash_size() -> u64 {
    get_dir_size("~/.local/share/Trash")
}

/// Native system cleanup using standard library file operations.
/// Returns the total number of bytes successfully freed.
pub fn clean_all_native() -> u64 {
    let mut freed_bytes = 0;

    // 1. Clean User Cache
    let home = std::env::var("HOME").unwrap_or_default();
    if !home.is_empty() {
        let safe_paths = vec![
            format!("{}/.cache/thumbnails", home),
            format!("{}/.cache/pip", home),
            format!("{}/.cache/cargo/registry/cache", home),
            format!("{}/.cache/go-build", home),
            format!("{}/.cache/yarn", home),
            format!("{}/.cache/fontconfig", home),
            format!("{}/.cache/gstreamer-1.0", home),
            format!("{}/.cache/mesa_shader_cache", home),
        ];

        for path in safe_paths {
            let p = Path::new(&path);
            if p.exists() {
                let size = get_dir_size_native(p);
                if fs::remove_dir_all(p).is_ok() {
                    freed_bytes += size;
                }
            }
        }
    }

    // 2. Clean Pacman Package Cache (ignoring files if permission denied)
    let pacman_pkg_dir = Path::new("/var/cache/pacman/pkg");
    if pacman_pkg_dir.exists() {
        if let Ok(entries) = fs::read_dir(pacman_pkg_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Ok(meta) = entry.metadata() {
                            let size = meta.len();
                            if fs::remove_file(path).is_ok() {
                                freed_bytes += size;
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Clean Journal Logs (archived files containing '@')
    let journal_dir = Path::new("/var/log/journal");
    if journal_dir.exists() {
        if let Ok(entries) = fs::read_dir(journal_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Ok(sub_entries) = fs::read_dir(&path) {
                            for sub_entry in sub_entries {
                                if let Ok(sub_entry) = sub_entry {
                                    let sub_path = sub_entry.path();
                                    if sub_path.is_file() {
                                        if let Ok(meta) = sub_entry.metadata() {
                                            let size = meta.len();
                                            let file_name = sub_path.file_name().unwrap_or_default().to_string_lossy();
                                            if file_name.contains('@') {
                                                if fs::remove_file(sub_path).is_ok() {
                                                    freed_bytes += size;
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

    // 4. Clean Trash Bin
    if !home.is_empty() {
        let trash_dir = format!("{}/.local/share/Trash", home);
        let files_path = format!("{}/files", trash_dir);
        let info_path = format!("{}/info", trash_dir);
        let p_files = Path::new(&files_path);
        let p_info = Path::new(&info_path);
        
        if p_files.exists() {
            let size = get_dir_size_native(p_files);
            if fs::remove_dir_all(p_files).is_ok() {
                freed_bytes += size;
                let _ = fs::create_dir_all(p_files);
            }
        }
        if p_info.exists() {
            let size = get_dir_size_native(p_info);
            if fs::remove_dir_all(p_info).is_ok() {
                freed_bytes += size;
                let _ = fs::create_dir_all(p_info);
            }
        }
    }

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
