//! Desktop file search and query utilities.

use std::path::PathBuf;

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
                        if file_name.starts_with('.')
                            || file_name == "node_modules"
                            || file_name == "target"
                            || file_name == "build"
                            || file_name == ".git"
                        {
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
