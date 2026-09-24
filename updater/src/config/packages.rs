use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PackagesConfig {
    #[serde(default)]
    pub pacman: Vec<String>,
    #[serde(default)]
    pub yay: Vec<String>,
}
