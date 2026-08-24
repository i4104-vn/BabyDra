use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::time::{Duration, Instant};
use walkdir::WalkDir;

lazy_static::lazy_static! {
    static ref DIR_SIZE_CACHE: RwLock<FxHashMap<PathBuf, (u64, Instant)>> = RwLock::new(FxHashMap::default());
}

/// Maximum number of cached directory sizes before stale entries are evicted.
const DIR_SIZE_CACHE_CAP: usize = 512;

/// Calculates the size of a directory and all its subdirectories in parallel using Rayon.
/// Caches the results with a 60-second Time-To-Live (TTL).
pub fn calc_dir_size(path: &Path) -> u64 {
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
        .max_depth(6)
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

    // 3. Write to the cache, evicting expired entries once the cap is reached
    if let Ok(mut cache) = DIR_SIZE_CACHE.write() {
        if cache.len() >= DIR_SIZE_CACHE_CAP {
            cache.retain(|_, (_, timestamp)| timestamp.elapsed() < Duration::from_secs(60));
        }
        if cache.len() >= DIR_SIZE_CACHE_CAP {
            cache.clear();
        }
        cache.insert(path_buf, (total_size, Instant::now()));
    }

    total_size
}
