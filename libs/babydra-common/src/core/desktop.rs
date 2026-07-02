//! Parser for desktop entry specifications (`.desktop` files) and system launcher caching.

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

/// Information model of a parsed desktop entry application.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DesktopApp {
    /// Friendly user-facing name of the application.
    pub name: String,
    /// Absolute or path executable execute command.
    pub exec: String,
    /// System icon theme name or filepath.
    pub icon: Option<String>,
    /// Unique Wayland application ID if this app is currently running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Active window title string if this app is currently running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_title: Option<String>,
}

/// Cache block structure stored in local cache file.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DesktopCache {
    pub system_mtime_secs: u64,
    pub local_mtime_secs: u64,
    pub apps: Vec<DesktopApp>,
}

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
        .and_then(|t| t.duration_since(std::time::SystemTime::UNIX_EPOCH).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)))
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Force-scans the system applications directories and updates both the memory and disk caches.
pub fn refresh_desktop_apps_cache() -> Vec<DesktopApp> {
    let apps = scan_desktop_apps_from_filesystem();
    
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

fn scan_desktop_apps_from_filesystem() -> Vec<DesktopApp> {
    let mut apps = Vec::new();
    let paths = vec![
        PathBuf::from("/usr/share/applications"),
        dirs::data_dir()
            .unwrap_or_else(|| {
                let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
                PathBuf::from(home).join(".local/share")
            })
            .join("applications"),
    ];

    for path in paths {
        if !path.exists() {
            continue;
        }
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.extension().map(|e| e == "desktop").unwrap_or(false) {
                    if let Some(app) = parse_desktop_file(&entry_path) {
                        apps.push(app);
                    }
                }
            }
        }
    }

    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps.dedup_by(|a, b| a.name.to_lowercase() == b.name.to_lowercase());

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
            if cache_data.system_mtime_secs == system_mtime && cache_data.local_mtime_secs == local_mtime {
                return cache_data.apps.clone();
            }
        }
    }
    
    if let Some(cache_path) = get_cache_file_path() {
        if cache_path.exists() {
            if let Ok(file) = File::open(&cache_path) {
                if let Ok(cache_data) = serde_json::from_reader::<_, DesktopCache>(file) {
                    if cache_data.system_mtime_secs == system_mtime && cache_data.local_mtime_secs == local_mtime {
                        if let Ok(mut lock) = cache.lock() {
                            *lock = Some(cache_data.clone());
                        }
                        return cache_data.apps;
                    }
                }
            }
        }
    }
    
    refresh_desktop_apps_cache()
}

fn parse_desktop_file(path: &Path) -> Option<DesktopApp> {
    let file = File::open(path).ok()?;
    let reader = BufReader::new(file);

    let mut name = None;
    let mut exec = None;
    let mut icon = None;
    let mut no_display = false;
    let mut in_desktop_entry = false;

    for line in reader.lines().flatten() {
        let line = line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            if line == "[Desktop Entry]" {
                in_desktop_entry = true;
            } else {
                in_desktop_entry = false;
            }
            continue;
        }

        if !in_desktop_entry {
            continue;
        }

        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim();
            let value = line[pos + 1..].trim();

            match key {
                "Name" if name.is_none() => name = Some(value.to_string()),
                "Exec" if exec.is_none() => {
                    let clean_exec = value
                        .split_whitespace()
                        .filter(|word| !word.starts_with('%'))
                        .collect::<Vec<&str>>()
                        .join(" ");
                    exec = Some(clean_exec);
                }
                "Icon" if icon.is_none() => icon = Some(value.to_string()),
                "NoDisplay" => {
                    if value.to_lowercase() == "true" {
                        no_display = true;
                    }
                }
                _ => {}
            }
        }
    }

    if no_display {
        return None;
    }

    match (name, exec) {
        (Some(n), Some(e)) => Some(DesktopApp { name: n, exec: e, icon, app_id: None, window_title: None }),
        _ => None,
    }
}

/// Generates a unique hash string representing a specific Wayland window based on its app_id and title.
pub fn get_window_hash(app_id: &str, title: &str) -> String {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    app_id.hash(&mut hasher);
    title.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

impl DesktopApp {
    /// Returns the unique window preview cache key hash of this application.
    pub fn get_screenshot_hash(&self) -> Option<String> {
        let app_id = self.app_id.as_ref()?;
        let title = self.window_title.as_deref().unwrap_or("");
        Some(get_window_hash(app_id, title))
    }
}


