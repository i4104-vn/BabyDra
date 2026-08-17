use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertInfo {
    pub filename: String,
    pub path: String,
}
