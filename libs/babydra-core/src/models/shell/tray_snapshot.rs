//! System tray snapshot model for change detection.

/// A lightweight snapshot of a tray icon used by the panel to detect when the
/// tray contents changed and trigger a rebuild.
#[derive(Clone, PartialEq, Debug)]
pub struct TraySnapshot {
    pub service: String,
    pub icon_name: String,
}
