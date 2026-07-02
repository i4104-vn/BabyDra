//! Controller actions and local index crawling utilities for file queries.

use gtk4::prelude::*;
use std::process::Command;
use std::path::{Path, PathBuf};

mod render;

/// Iterates user directory folders recursively up to depth 2 to locate matching files.
pub fn search_files(query: &str) -> Vec<PathBuf> {
    let mut results = Vec::new();
    if query.trim().len() < 2 {
        return results;
    }
    let query_lower = query.to_lowercase();
    if let Some(home_dir) = dirs::home_dir() {
        let search_dirs = vec![
            home_dir.join("Desktop"),
            home_dir.join("Downloads"),
            home_dir.join("Documents"),
        ];
        
        for dir in search_dirs {
            if !dir.exists() {
                continue;
            }
            let mut stack = vec![(dir, 0)];
            while let Some((current_dir, depth)) = stack.pop() {
                if results.len() >= 8 {
                    break;
                }
                if let Ok(entries) = std::fs::read_dir(current_dir) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                        if file_name.starts_with('.') || 
                           file_name == "node_modules" || 
                           file_name == "target" || 
                           file_name == "build" || 
                           file_name == ".git" {
                            continue;
                        }
                        if file_name.to_lowercase().contains(&query_lower) {
                            results.push(path.clone());
                        }
                        if path.is_dir() && depth < 1 {
                            stack.push((path, depth + 1));
                        }
                    }
                }
            }
        }
    }
    results
}

/// Creates a clickable button row representing a discovered local file.
/// Binds clicking the button to launch the file using `xdg-open`.
pub fn create_file_row(path: &Path, window: &gtk4::ApplicationWindow) -> gtk4::Button {
    let (btn, _, _) = render::build_file_row_ui(path);
    let path_str = path.to_string_lossy().to_string();
    
    let win_to_close = window.clone();
    btn.connect_clicked(move |_| {
        println!("Opening file: {}", path_str);
        if let Err(e) = Command::new("xdg-open").arg(&path_str).spawn() {
            eprintln!("Failed to open file {}: {}", path_str, e);
        }
        
        win_to_close.close();
    });
    
    btn
}

