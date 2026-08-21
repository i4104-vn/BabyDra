use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SidebarItem {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub path: PathBuf,
    pub is_bookmark: bool,
}

pub fn get_sidebar_layout_path() -> PathBuf {
    crate::config::get_config_dir().join("explore.json")
}

pub fn load_sidebar_layout() -> Vec<SidebarItem> {
    let path = get_sidebar_layout_path();
    if let Ok(data) = std::fs::read_to_string(&path) {
        if let Ok(items) = serde_json::from_str(&data) {
            return items;
        }
    }

    // Default layout
    let mut default_items = Vec::new();

    // Places
    default_items.push(SidebarItem {
        id: "home".to_string(),
        name: "explore.home".to_string(),
        icon: "user-home".to_string(),
        path: glib::home_dir(),
        is_bookmark: false,
    });

    // Add default folders
    let folders = [
        (
            "downloads",
            "explore.downloads",
            "folder-download",
            glib::UserDirectory::Downloads,
            "Downloads",
        ),
        (
            "documents",
            "explore.documents",
            "folder-documents",
            glib::UserDirectory::Documents,
            "Documents",
        ),
        (
            "pictures",
            "explore.pictures",
            "folder-pictures",
            glib::UserDirectory::Pictures,
            "Pictures",
        ),
        (
            "music",
            "explore.music",
            "folder-music",
            glib::UserDirectory::Music,
            "Music",
        ),
        (
            "desktop",
            "explore.desktop",
            "folder-desktop",
            glib::UserDirectory::Desktop,
            "Desktop",
        ),
        (
            "videos",
            "explore.videos",
            "folder-videos",
            glib::UserDirectory::Videos,
            "Videos",
        ),
    ];
    for (id, name, icon, ud, fb) in folders {
        let p = glib::user_special_dir(ud).unwrap_or_else(|| glib::home_dir().join(fb));
        default_items.push(SidebarItem {
            id: id.to_string(),
            name: name.to_string(),
            icon: icon.to_string(),
            path: p,
            is_bookmark: false,
        });
    }

    let trash_path = glib::user_data_dir().join("Trash/files");
    default_items.push(SidebarItem {
        id: "trash".to_string(),
        name: "explore.trash".to_string(),
        icon: "user-trash".to_string(),
        path: trash_path,
        is_bookmark: false,
    });

    // This PC
    default_items.push(SidebarItem {
        id: "this_pc".to_string(),
        name: "explore.local_disk".to_string(),
        icon: "drive-harddisk".to_string(),
        path: PathBuf::from("/"),
        is_bookmark: false,
    });

    default_items
}

pub fn save_sidebar_layout(items: &[SidebarItem]) {
    let path = get_sidebar_layout_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(items) {
        let _ = std::fs::write(path, json);
    }
}
