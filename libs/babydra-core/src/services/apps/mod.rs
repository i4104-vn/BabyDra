//! Parser for desktop entry specifications (`.desktop` files) and system launcher caching.

pub mod discovery;
pub mod pacman;

pub use crate::models::shell::app::{DesktopApp, DesktopCache};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

pub use discovery::{parse_desktop_file, scan_desktop_apps};

static CACHE: OnceLock<Arc<Mutex<Option<DesktopCache>>>> = OnceLock::new();

fn get_cache() -> &'static Arc<Mutex<Option<DesktopCache>>> {
    CACHE.get_or_init(|| Arc::new(Mutex::new(None)))
}

fn get_cache_file_path() -> Option<PathBuf> {
    dirs::cache_dir().map(|p| p.join("babydra").join("desktop_apps.json"))
}

fn get_dir_mtime(path: &Path) -> u64 {
    std::fs::metadata(path)
        .and_then(|m| m.modified())
        .and_then(|t| {
            t.duration_since(std::time::SystemTime::UNIX_EPOCH)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        })
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Force-scans the system applications directories and updates both the memory and disk caches.
pub fn refresh_desktop_apps() -> Vec<DesktopApp> {
    let apps = scan_desktop_apps();

    let system_mtime = get_dir_mtime(Path::new("/usr/share/applications"));
    let local_path = dirs::data_dir()
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".local/share")
        })
        .join("applications");
    let local_mtime = get_dir_mtime(&local_path);

    let cache_data = DesktopCache {
        system_mtime_secs: system_mtime,
        local_mtime_secs: local_mtime,
        apps: apps.clone(),
    };

    if let Some(cache_path) = get_cache_file_path() {
        if let Some(parent) = cache_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(file) = File::create(cache_path) {
            let _ = serde_json::to_writer(file, &cache_data);
        }
    }

    let cache = get_cache();
    if let Ok(mut lock) = cache.lock() {
        *lock = Some(cache_data);
    }

    apps
}

/// Retrieves list of system applications, querying memory or disk cache, or scanning path directories if necessary.
pub fn find_desktop_apps() -> Vec<DesktopApp> {
    let system_mtime = get_dir_mtime(Path::new("/usr/share/applications"));
    let local_path = dirs::data_dir()
        .unwrap_or_else(|| {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".local/share")
        })
        .join("applications");
    let local_mtime = get_dir_mtime(&local_path);

    let cache = get_cache();

    if let Ok(lock) = cache.lock() {
        if let Some(ref cache_data) = *lock {
            if cache_data.system_mtime_secs == system_mtime
                && cache_data.local_mtime_secs == local_mtime
            {
                return cache_data.apps.clone();
            }
        }
    }

    if let Some(cache_path) = get_cache_file_path() {
        if cache_path.exists() {
            if let Ok(file) = File::open(&cache_path) {
                if let Ok(cache_data) = serde_json::from_reader::<_, DesktopCache>(file) {
                    if cache_data.system_mtime_secs == system_mtime
                        && cache_data.local_mtime_secs == local_mtime
                    {
                        if let Ok(mut lock) = cache.lock() {
                            *lock = Some(cache_data.clone());
                        }
                        return cache_data.apps;
                    }
                }
            }
        }
    }

    refresh_desktop_apps()
}

/// Generates a unique hash string representing a specific Wayland window based on its app_id and title.
pub fn get_window_hash(app_id: &str, title: &str) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    app_id.hash(&mut hasher);
    title.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
