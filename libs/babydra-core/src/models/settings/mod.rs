//! System & Hardware Settings Models

pub mod app_info;
pub mod certificates;
pub mod display;
pub mod env_var;
pub mod hosts;
pub mod keybind;
pub mod startup_command;
pub mod system_info;
pub mod system_update;
pub mod vpn;
pub mod wifi;

pub use app_info::{AppsWidget, InstalledApp, InstalledPackage};
pub use certificates::{CertInfo, CertificatesWidget};
pub use display::{DisplayCardRow, DisplaysWidget, MonitorConfig};
pub use env_var::{EnvVar, EnvWidget};
pub use hosts::HostsWidget;
pub use keybind::{Keybind, KeybindsWidget};
pub use startup_command::{StartupCommand, StartupWidget};
pub use system_info::SystemInfoData;
pub use system_update::{PackageUpdate, SystemUpdateState, SystemUpdateWidget};
pub use vpn::{VpnConn, VpnConnDetails};
pub use wifi::{WifiConfig, WifiNetwork};
