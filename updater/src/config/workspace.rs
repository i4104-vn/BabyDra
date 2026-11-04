use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceToml {
    #[serde(default)]
    pub binaries: Vec<WorkspaceBinary>,
    #[serde(default)]
    pub packages: WorkspacePackages,
    #[serde(default)]
    pub gsettings: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceBinary {
    pub name: String,
    #[serde(default = "default_scope")]
    pub scope: String,
    #[allow(dead_code)]
    pub description: Option<String>,
}

fn default_scope() -> String {
    "user".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspacePackages {
    #[serde(default)]
    pub pacman: Vec<String>,
    #[serde(default)]
    pub aur: Vec<String>,
}
