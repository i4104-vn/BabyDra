use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaConfig {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: String,
}

impl Default for MetaConfig {
    fn default() -> Self {
        Self {
            name: "BabyDra".to_string(),
            version: "1.0.0".to_string(),
            description: "Modern Wayland Desktop Shell for Arch Linux".to_string(),
        }
    }
}
