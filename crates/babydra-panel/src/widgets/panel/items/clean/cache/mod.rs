use std::fs;
use std::path::Path;
use super::{is_dir_writable, get_cleanable_size_recursive, clean_path_recursive};

pub fn get_user_cache_size() -> u64 {
    let home = std::env::var("HOME").unwrap_or_default();
    if home.is_empty() {
        return 0;
    }
    let safe_paths = vec![
        format!("{}/.cache/thumbnails", home),
        format!("{}/.cache/fontconfig", home),
        format!("{}/.cache/gstreamer-1.0", home),
        format!("{}/.cache/mesa_shader_cache", home),
    ];
    let mut size = 0;
    for path in safe_paths {
        let p = Path::new(&path);
        if p.exists() {
            size += get_cleanable_size_recursive(p);
        }
    }
    size
}

pub fn remove_user_cache() -> u64 {
    let home = std::env::var("HOME").unwrap_or_default();
    if home.is_empty() {
        return 0;
    }
    let safe_paths = vec![
        format!("{}/.cache/thumbnails", home),
        format!("{}/.cache/fontconfig", home),
        format!("{}/.cache/gstreamer-1.0", home),
        format!("{}/.cache/mesa_shader_cache", home),
    ];
    let mut freed = 0;
    for path in safe_paths {
        let p = Path::new(&path);
        if p.exists() {
            freed += clean_path_recursive(p);
        }
    }
    freed
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
