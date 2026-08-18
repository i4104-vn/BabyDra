//! System tray item configurations model.

/// Representation of a registered system tray item.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TrayItem {
    /// DBus destination service name.
    pub service: String,
    /// DBus object path (e.g. "/StatusNotifierItem").
    pub path: String,
    /// Icon theme name, file path, or icon identifier.
    pub icon_name: String,
    /// Friendly tooltip title.
    pub title: String,
}
