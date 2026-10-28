use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupCommand {
    pub id: u32,
    #[serde(default)]
    pub name: Option<String>,
    pub command: String,
}
