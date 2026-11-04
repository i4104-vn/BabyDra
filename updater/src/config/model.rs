use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::config::desktop::MetaConfig;
use crate::config::workspace::WorkspaceBinary;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdaterToml {
    #[serde(default)]
    pub meta: MetaConfig,
    #[serde(default)]
    pub binaries: UpdaterBinariesToml,
    #[serde(default)]
    pub mime_defaults: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdaterBinariesToml {
    #[serde(default)]
    pub shell_daemons: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct UpdaterConfig {
    #[allow(dead_code)]
    pub meta: MetaConfig,
    pub workspace_binaries: Vec<WorkspaceBinary>,
    pub shell_daemons: Vec<String>,
    pub gsettings: HashMap<String, String>,
    pub mime_defaults: HashMap<String, String>,
    pub pacman_count: usize,
    pub aur_count: usize,
}

impl UpdaterConfig {
    pub fn user_binaries(&self) -> Vec<&str> {
        self.workspace_binaries
            .iter()
            .filter(|b| b.scope == "user")
            .map(|b| b.name.as_str())
            .collect()
    }

    pub fn system_binaries(&self) -> Vec<&str> {
        self.workspace_binaries
            .iter()
            .filter(|b| b.scope == "system")
            .map(|b| b.name.as_str())
            .collect()
    }

    pub fn kill_processes(&self) -> Vec<String> {
        let mut procs: Vec<String> = self
            .workspace_binaries
            .iter()
            .map(|b| b.name.clone())
            .collect();
        for daemon in &self.shell_daemons {
            let bin = daemon.split_whitespace().next().unwrap_or(daemon);
            if !procs.iter().any(|p| p == bin) {
                procs.push(bin.to_string());
            }
        }
        for extra in ["fnott", "xfce4-notifyd"] {
            if !procs.iter().any(|p| p == extra) {
                procs.push(extra.to_string());
            }
        }
        procs
    }
}
