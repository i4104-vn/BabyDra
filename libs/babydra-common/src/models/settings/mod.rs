//! System & Hardware Settings Models

pub mod app_info;
pub mod display;
pub mod hosts;
pub mod wifi;
pub mod vpn;
pub mod system_update;
pub mod system_info;
pub mod startup_command;
pub mod certificates;
pub mod env_var;
pub mod keybind;

pub use app_info::{InstalledApp, InstalledPackage, AppsWidget};
pub use display::{MonitorConfig, DisplayCardRow, DisplaysWidget};
pub use hosts::HostsWidget;
pub use wifi::{WifiNetwork, WifiConfig};
pub use vpn::{VpnConn, VpnConnDetails};
pub use system_update::{PackageUpdate, SystemUpdateWidget, SystemUpdateState};
pub use system_info::SystemInfoData;
pub use startup_command::{StartupCommand, StartupWidget};
pub use certificates::{CertInfo, CertificatesWidget};
pub use env_var::{EnvVar, EnvWidget};
pub use keybind::{Keybind, KeybindsWidget};
