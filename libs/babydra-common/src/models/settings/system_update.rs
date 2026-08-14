use gtk4::{Box, Button, Label, ListBox, Overlay, ProgressBar, Spinner};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateStatus {
    Pending,
    Updating,
    Done,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageUpdate {
    pub name: String,
    pub old_version: String,
    pub new_version: String,
    pub status: UpdateStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemUpdateState {
    pub is_updating: bool,
    #[serde(default)]
    pub is_syncing: bool,
    pub packages: Vec<PackageUpdate>,
}

#[derive(Clone)]
pub struct SystemUpdateWidget {
    pub root: Overlay,
    pub container: Box,
    pub count_badge: Label,
    pub spinner: Spinner,
    pub update_all_btn: Button,
    pub refresh_btn: Button,
    pub progress_bar: ProgressBar,
    pub status_label: Label,
    pub glass_card: Box,
    pub list_box: ListBox,
}
