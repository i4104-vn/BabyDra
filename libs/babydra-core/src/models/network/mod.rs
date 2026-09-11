//! Network traffic, active connection and speed models.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct NetStats {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

/// Unified network activity snapshot broadcast by NetworkMonitor.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkSnapshot {
    pub active_info: ActiveNetworkInfo,
    pub rx_speed: f64,
    pub tx_speed: f64,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EthernetActivityState {
    pub is_connected: bool,
    pub tx_lit: bool,
    pub rx_lit: bool,
}

pub struct NetworkReceiver {
    rx: std::sync::mpsc::Receiver<NetworkSnapshot>,
}

impl NetworkReceiver {
    pub fn new(rx: std::sync::mpsc::Receiver<NetworkSnapshot>) -> Self {
        Self { rx }
    }

    pub fn attach<F: FnMut(NetworkSnapshot) -> glib::ControlFlow + 'static>(self, _context: Option<&()>, mut func: F) {
        glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
            let mut latest = None;
            while let Ok(snap) = self.rx.try_recv() {
                latest = Some(snap);
            }
            if let Some(snap) = latest {
                func(snap)
            } else {
                glib::ControlFlow::Continue
            }
        });
    }
}


