//! Desktop application entry and cache data models.

use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

/// Information model of a parsed desktop entry application.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopApp {
    /// Friendly user-facing name of the application.
    pub name: String,
    /// Absolute or path executable execute command.
    pub exec: String,
    /// System icon theme name or filepath.
    pub icon: Option<String>,
    /// Whether this app was installed as a dependency / system helper.
    #[serde(default)]
    pub is_dependency: bool,
    /// Unique Wayland application ID if this app is currently running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Active window title string if this app is currently running.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub window_title: Option<String>,
}

impl DesktopApp {
    /// Returns the unique window preview cache key hash of this application.
    pub fn get_screenshot_hash(&self) -> Option<String> {
        let app_id = self.app_id.as_ref()?;
        let title = self.window_title.as_deref().unwrap_or("");
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        app_id.hash(&mut hasher);
        title.hash(&mut hasher);
        Some(format!("{:x}", hasher.finish()))
    }
}

/// Cache block structure stored in local cache file.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesktopCache {
    pub system_mtime_secs: u64,
    pub local_mtime_secs: u64,
    pub apps: Vec<DesktopApp>,
}
