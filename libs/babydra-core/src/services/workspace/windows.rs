use crate::models::DesktopApp;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

type WindowsMapCache = Option<(PathBuf, HashMap<String, u32>)>;

static WINDOWS_MAP_CACHE: OnceLock<Mutex<WindowsMapCache>> = OnceLock::new();

fn windows_map_cache() -> &'static Mutex<WindowsMapCache> {
    WINDOWS_MAP_CACHE.get_or_init(|| Mutex::new(None))
}

pub fn get_windows_map_cache_path() -> PathBuf {
    super::cache::get_workspace_cache_dir().join("workspace_windows.json")
}

pub fn read_windows_map() -> HashMap<String, u32> {
    let path = get_windows_map_cache_path();
    let mut cache = windows_map_cache().lock().unwrap();
    if let Some((cached_path, map)) = cache.as_ref() {
        if cached_path == &path {
            return map.clone();
        }
    }

    let map = read_windows_map_from_disk(&path);
    *cache = Some((path, map.clone()));
    map
}

fn read_windows_map_from_disk(path: &std::path::Path) -> HashMap<String, u32> {
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(map) = serde_json::from_str::<HashMap<String, u32>>(&content) {
            return map;
        }
    }
    HashMap::new()
}

pub fn write_windows_map(map: &HashMap<String, u32>) {
    let path = get_windows_map_cache_path();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string(map) {
        let _ = fs::write(path, json);
    }
    *windows_map_cache().lock().unwrap() = Some((get_windows_map_cache_path(), map.clone()));
}

pub(crate) fn clear_windows_map_cache() {
    *windows_map_cache().lock().unwrap() = None;
}

pub fn make_app_key(app: &DesktopApp) -> String {
    let app_id = app.app_id.as_deref().unwrap_or(&app.name);
    let title = app.window_title.as_deref().unwrap_or("");
    format!("{}::{}", app_id, title)
}

pub fn sync_workspace_apps(current_ws: u32, apps: &[DesktopApp]) -> HashMap<String, u32> {
    let mut map = read_windows_map();
    if apps.is_empty() {
        return map;
    }

    let mut current_keys = std::collections::HashSet::new();
    let mut new_apps = Vec::new();

    for app in apps {
        let key = make_app_key(app);
        current_keys.insert(key.clone());
        if !map.contains_key(&key) {
            new_apps.push((key, app));
        }
    }

    let mut disappeared_by_app: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    for (old_key, &ws) in &map {
        if !current_keys.contains(old_key) {
            let app_id = old_key
                .split_once("::")
                .map(|(id, _)| id)
                .unwrap_or(old_key)
                .to_string();
            disappeared_by_app
                .entry(app_id)
                .or_default()
                .push((old_key.clone(), ws));
        }
    }

    for (new_key, app) in new_apps {
        let app_id = app.app_id.as_deref().unwrap_or(&app.name);
        if let Some(disappeared_list) = disappeared_by_app.get_mut(app_id) {
            if let Some((old_key, old_ws)) = disappeared_list.pop() {
                map.remove(&old_key);
                map.insert(new_key, old_ws);
                continue;
            }
        }

        map.insert(new_key, current_ws);
    }

    map.retain(|k, _| current_keys.contains(k));
    if map != read_windows_map() {
        write_windows_map(&map);
    }
    map
}

pub fn get_app_workspace(app: &DesktopApp, map: &HashMap<String, u32>, fallback_ws: u32) -> u32 {
    let key = make_app_key(app);
    if let Some(&ws) = map.get(&key) {
        return ws;
    }
    fallback_ws
}

pub fn filter_apps_for_workspace(
    ws_id: u32,
    apps: &[DesktopApp],
    current_ws: u32,
) -> Vec<DesktopApp> {
    let map = sync_workspace_apps(current_ws, apps);
    apps.iter()
        .filter(|app| get_app_workspace(app, &map, current_ws) == ws_id)
        .cloned()
        .collect()
}
