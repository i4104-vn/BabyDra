//! Shared configuration and state data models for BabyDra.

pub mod theme_config;
pub mod shell_config;
pub mod island_state;
pub mod volume;
pub mod storage;
pub mod notification;
pub mod tray_item;
pub mod screenshot;
pub mod explore;
pub mod network;
pub mod battery;
pub mod power;

pub mod display;
pub mod app_info;
pub mod system_update;
pub mod startup_command;
pub mod wifi;
pub mod hosts;
pub mod vpn;

pub use theme_config::ThemeConfig;
pub use shell_config::ShellConfig;
pub use island_state::IslandState;
pub use volume::AudioDevice;
pub use storage::DiskInfo;
pub use notification::{ActiveNotification, NotificationMsg};
pub use tray_item::TrayItem;
pub use screenshot::{Drawing, Tool, EditorState};
pub use explore::{FileEntry, FileType, DirectoryModel, SortColumn, SortOrder, TabState, SessionState};
pub use network::{NetStats, NetSpeed};
pub use battery::BatteryInfo;
pub use power::PerformanceProfile;
pub use display::MonitorConfig;
pub use app_info::{InstalledApp, InstalledPackage, AppsWidget};
pub use system_update::{PackageUpdate, SystemUpdateWidget};
pub use startup_command::StartupCommand;
pub use wifi::WifiNetwork;
pub use hosts::HostsWidget;
pub use vpn::{VpnConn, VpnConnDetails};


