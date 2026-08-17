//! Network traffic and speed data models.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NetStats {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NetSpeed {
    pub rx_speed: f64,
    pub tx_speed: f64,
}
