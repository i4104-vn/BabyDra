//! VPN and WireGuard connection management via NetworkManager CLI (nmcli).

pub mod ops;

pub use crate::models::vpn::*;
pub use ops::*;
