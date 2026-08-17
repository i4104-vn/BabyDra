//! User home cache storage cleanup routines.

use super::helper::{clean_path_recursive, get_cleanable_size_recursive};
use std::path::Path;

/// Returns the current `user cache size`.
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

/// Removes `user cache` and returns the result.
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
