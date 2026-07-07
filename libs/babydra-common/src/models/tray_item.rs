//! System tray item configurations model.

/// Representation of a registered system tray item.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TrayItem {
    /// DBus destination service name.
    pub service: String,
    /// Icon theme name or path string.
    pub icon_name: String,
    /// Friendly tooltip title.
    pub title: String,
}
