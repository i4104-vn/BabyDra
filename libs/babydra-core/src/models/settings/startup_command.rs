use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupCommand {
    pub id: u32,
    pub command: String,
}
