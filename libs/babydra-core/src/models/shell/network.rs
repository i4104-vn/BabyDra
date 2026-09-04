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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActiveNetworkType {
    Ethernet,
    Wifi,
    Disconnected,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveNetworkInfo {
    pub network_type: ActiveNetworkType,
    pub is_connected: bool,
    pub name: String,
    pub ip_address: String,
    pub icon_name: String,
    pub interface: String,
}

impl Default for ActiveNetworkInfo {
    fn default() -> Self {
        Self {
            network_type: ActiveNetworkType::Disconnected,
            is_connected: false,
            name: "Disconnected".to_string(),
            ip_address: "127.0.0.1".to_string(),
            icon_name: "wifi".to_string(),
            interface: String::new(),
        }
    }
}
