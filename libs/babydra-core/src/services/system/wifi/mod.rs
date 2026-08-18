//! WiFi subsystem module wrapper.

pub mod client;
pub mod connection;
pub mod discovery;
pub mod state;

pub use connection::{
    connect_wifi, forget_wifi, get_wifi_config, set_wifi_config, strip_ansi_escapes,
};
pub use discovery::{known_networks, scan_networks};
pub use state::{get_wifi_signal, get_wifi_state, set_wifi_enabled};
