//! Wi-Fi network model.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiNetwork {
    pub ssid: String,
    pub security: String,
    pub strength: String,
    pub is_connected: bool,
}
