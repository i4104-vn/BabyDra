//! Wi-Fi network model.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiNetwork {
    pub ssid: String,
    pub security: String,
    pub strength: String,
    pub is_connected: bool,
    pub signal: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WifiConfig {
    pub method: String,
    pub ip_address: String,
    pub prefix: u32,
    pub gateway: String,
    pub dns: String,
    pub bssid: Option<String>,
    pub frequency: Option<String>,
    pub speed: Option<String>,
    pub interface: Option<String>,
    pub mac_address: Option<String>,
}
