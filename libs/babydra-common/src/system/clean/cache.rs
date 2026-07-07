//! User home cache storage cleanup routines.

use std::path::Path;
use super::helper::{get_cleanable_size_recursive, clean_path_recursive};

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
