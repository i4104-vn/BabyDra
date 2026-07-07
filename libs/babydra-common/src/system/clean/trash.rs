//! Trash folder size checking and emptying.

use std::fs;
use std::path::Path;
use super::helper::{get_cleanable_size_recursive, clean_path_recursive};

pub fn get_trash_size() -> u64 {
    let home = std::env::var("HOME").unwrap_or_default();
    if home.is_empty() {
        return 0;
    }
    let trash_dir = format!("{}/.local/share/Trash", home);
    let files_path = format!("{}/files", trash_dir);
    let info_path = format!("{}/info", trash_dir);
    let p_files = Path::new(&files_path);
    let p_info = Path::new(&info_path);
    let mut size = 0;
    if p_files.exists() {
        size += get_cleanable_size_recursive(p_files);
    }
    if p_info.exists() {
        size += get_cleanable_size_recursive(p_info);
    }
    size
}

pub fn remove_trash() -> u64 {
    let home = std::env::var("HOME").unwrap_or_default();
    if home.is_empty() {
        return 0;
    }
    let trash_dir = format!("{}/.local/share/Trash", home);
    let files_path = format!("{}/files", trash_dir);
    let info_path = format!("{}/info", trash_dir);
    let p_files = Path::new(&files_path);
    let p_info = Path::new(&info_path);
    let mut freed = 0;
    if p_files.exists() {
        freed += clean_path_recursive(p_files);
        let _ = fs::create_dir_all(p_files);
    }
    if p_info.exists() {
        freed += clean_path_recursive(p_info);
        let _ = fs::create_dir_all(p_info);
    }
    freed
}
