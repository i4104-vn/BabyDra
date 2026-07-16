//! WiFi subsystem module wrapper.

pub mod client;
pub mod connection;
pub mod discovery;
pub mod state;

pub use state::{get_wifi_state, set_wifi_enabled};
pub use discovery::{known_networks, scan_networks};
pub use connection::{connect_wifi, strip_ansi_escapes};
