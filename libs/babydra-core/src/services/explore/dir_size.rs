use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::time::{Duration, Instant};
use walkdir::WalkDir;

lazy_static::lazy_static! {
    static ref DIR_SIZE_CACHE: RwLock<FxHashMap<PathBuf, (u64, Instant)>> = RwLock::new(FxHashMap::default());
}

/// Calculates the size of a directory and all its subdirectories in parallel using Rayon.
/// Caches the results with a 60-second Time-To-Live (TTL).
pub fn calculate_dir_size_parallel(path: &Path) -> u64 {
    let path_buf = path.to_path_buf();

    // 1. Check the cache
    if let Ok(cache) = DIR_SIZE_CACHE.read() {
        if let Some(&(size, timestamp)) = cache.get(&path_buf) {
            if timestamp.elapsed() < Duration::from_secs(60) {
                return size;
            }
        }
    }

    // 2. Perform parallel directory traversal and size summation
    let total_size: u64 = WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .par_bridge() // Parallelize WalkDir iterator
        .map(|entry| {
            if entry.file_type().is_file() {
                entry.metadata().map(|m| m.len()).unwrap_or(0)
            } else {
                0
            }
        })
        .sum();

    // 3. Write to the cache
    if let Ok(mut cache) = DIR_SIZE_CACHE.write() {
        cache.insert(path_buf, (total_size, Instant::now()));
    }

    total_size
}
