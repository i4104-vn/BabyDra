//! VPN and WireGuard connection management via NetworkManager CLI (nmcli).

pub mod actions;
pub mod config;
pub mod queries;

pub use crate::models::vpn::*;
pub use actions::*;
pub use config::*;
pub use queries::*;
