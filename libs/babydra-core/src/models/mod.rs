//! Shared configuration and state data models for BabyDra.
//! Categorized into topic submodules: `system`, `network`, `desktop`, `tray`, `theme`, `island`, `cli`, `settings`, `explore`, and `screenshot`.

pub mod cli;
pub mod config;
pub mod desktop;
pub mod explore;
pub mod island;
pub mod network;
pub mod screenshot;
pub mod settings;
pub mod shell;
pub mod system;
pub mod theme;
pub mod tray;

// Re-export submodules for backward compatibility with `models::<submodule>::...`
pub use settings::app_info;
pub use settings::bluetooth;
pub use settings::certificates;
pub use settings::display;
pub use settings::env_var;
pub use settings::hosts;
pub use settings::keybind;
pub use settings::nav;
pub use settings::shortcut;
pub use settings::startup_command;
pub use settings::system_info;
pub use settings::system_update;
pub use settings::vpn;
pub use settings::wifi;

pub use shell::app;
pub use shell::appearance;
pub use shell::battery;
pub use shell::dbus_menu;
pub use shell::exif;
pub use shell::island_state;
pub use shell::monitor;
pub use shell::notification;
pub use shell::power;
pub use shell::shell_config;
pub use shell::storage;
pub use shell::theme_config;
pub use shell::tray_item;
pub use shell::volume;

pub use config::*;

// Direct type re-exports
pub use cli::{CliAction, CliOptions, PageId};
pub use desktop::{calc_auto_arrange, snap_to_grid, sort_entries, DesktopApp, DesktopCache, DesktopState, ExifData, Workspace, WorkspaceReceiver, WorkspaceSnapshot, DEFAULT_CELL_HEIGHT, DEFAULT_CELL_WIDTH, DEFAULT_MARGIN_X, DEFAULT_MARGIN_Y};
pub use explore::{FileEntry, FileType, SessionState, TabState};
pub use island::{ActiveNotification, IslandState, NotificationMsg};
pub use network::{ActiveNetworkInfo, ActiveNetworkType, EthernetActivityState, NetSpeed, NetStats, NetworkReceiver, NetworkSnapshot};
pub use screenshot::{Drawing, EditorState, Tool, STROKE_WIDTHS};
pub use settings::*;
pub use system::{AppResourceUsage, AudioBackendType, AudioDevice, BatteryInfo, CpuTime, DaemonMessage, DiskInfo, MonitorReceiver, MonitorSnapshot, PerformanceProfile};
pub use theme::{CurrentAppearance, ShellConfig, ThemeConfig, ThemeSelection};
pub use tray::{LayoutItem, MenuItem, TrayItem, TraySnapshot};
