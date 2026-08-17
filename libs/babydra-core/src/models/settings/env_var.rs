use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub id: usize,
    pub key: String,
    pub value: String,
}
