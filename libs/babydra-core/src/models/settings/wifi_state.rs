//! Wi-Fi settings panel state model.

use crate::models::settings::wifi::WifiNetwork;

/// Runtime state for the Wi-Fi settings panel.
#[derive(Debug, Clone, Default)]
pub struct WifiState {
    pub enabled: bool,
    pub networks: Vec<WifiNetwork>,
    pub is_loading: bool,
    pub connecting_ssid: Option<String>,
}
