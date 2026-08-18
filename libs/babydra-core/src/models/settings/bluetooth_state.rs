//! Bluetooth settings panel state model.

use crate::models::settings::bluetooth::BtDevice;

/// Runtime state for the Bluetooth settings panel.
#[derive(Debug, Clone, Default)]
pub struct BluetoothState {
    pub enabled: bool,
    pub devices: Vec<BtDevice>,
    pub is_loading: bool,
}
