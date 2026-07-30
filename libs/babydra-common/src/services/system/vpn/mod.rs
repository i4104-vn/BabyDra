//! VPN and WireGuard connection management via NetworkManager D-Bus API (zbus).

pub mod dbus;
pub mod ops;

pub use crate::models::vpn::*;
pub use ops::*;
