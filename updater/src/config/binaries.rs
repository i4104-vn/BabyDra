use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BinariesConfig {
    #[serde(default)]
    pub user_binaries: Vec<String>,
    #[serde(default)]
    pub system_binaries: Vec<String>,
    #[serde(default)]
    pub shell_daemons: Vec<String>,
    #[serde(default)]
    pub kill_processes: Vec<String>,
}
