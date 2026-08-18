//! Application manager data models.

use crate::models::settings::app_info::{InstalledApp, InstalledPackage};

/// Payload sent from the background app/pacman scanning thread to the UI.
#[derive(Debug, Clone)]
pub struct AppsData {
    pub apps_data: Vec<InstalledApp>,
    pub pkgs: Vec<InstalledPackage>,
}

/// Type of package management action an app row can trigger.
/// Shared by both the row action items and the pending auth action.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppActionType {
    Uninstall,
    Downgrade,
}
