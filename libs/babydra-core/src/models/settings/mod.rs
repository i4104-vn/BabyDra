//! System & Hardware Settings Models

pub mod app_info;
pub mod apps;
pub mod bluetooth;
pub mod bluetooth_state;
pub mod certificates;
pub mod display;
pub mod env_var;
pub mod hosts;
pub mod keybind;
pub mod nav;
pub mod startup_command;
pub mod system_info;
pub mod system_update;
pub mod vpn;
pub mod wifi;
pub mod wifi_state;

pub use app_info::{InstalledApp, InstalledPackage};
pub use apps::{AppActionType, AppsData};
pub use bluetooth::BtDevice;
pub use bluetooth_state::BluetoothState;
pub use certificates::CertInfo;
pub use display::MonitorConfig;
pub use env_var::EnvVar;
pub use keybind::Keybind;
pub use nav::{NavCategory, NavItem};
pub use startup_command::StartupCommand;
pub use system_info::SystemInfoData;
pub use system_update::{PackageUpdate, SystemUpdateState};
pub use vpn::{VpnConn, VpnConnDetails};
pub use wifi::{WifiConfig, WifiNetwork};
pub use wifi_state::WifiState;
