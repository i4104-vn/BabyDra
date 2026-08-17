use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keybind {
    pub id: usize,
    pub bind_type: String,
    pub modifiers: String,
    pub key: String,
    pub dispatcher: String,
    pub args: String,
}
