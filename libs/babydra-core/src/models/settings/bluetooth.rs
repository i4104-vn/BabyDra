//! Bluetooth data models.

use serde::{Deserialize, Serialize};

/// Represents a detected Bluetooth remote device.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BtDevice {
    pub mac: String,
    pub name: String,
    pub connected: bool,
}
