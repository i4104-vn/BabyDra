//! VPN and WireGuard connection management via NetworkManager D-Bus API (zbus).

pub mod dbus;
pub mod ops;
pub mod types;

pub use ops::*;
pub use types::*;
