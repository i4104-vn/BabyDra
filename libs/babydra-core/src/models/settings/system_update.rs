use serde::{Deserialize, Serialize};

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
