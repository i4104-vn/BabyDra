//! Shared configuration and state data models for BabyDra.
//! Categorized into `shell`, `settings`, `explore`, and `screenshot`.

pub mod shell;
pub mod settings;
pub mod explore;
<<<<<<< HEAD
pub mod screenshot;

// Re-export submodules for backward compatibility with `models::<submodule>::...`
pub use settings::app_info;
pub use settings::display;
pub use settings::hosts;
pub use settings::wifi;
pub use settings::vpn;
pub use settings::system_update;
pub use settings::system_info;
pub use settings::startup_command;
pub use settings::certificates;
pub use settings::env_var;
pub use settings::keybind;

// Direct type re-exports
pub use shell::*;
pub use settings::*;
pub use explore::{DirectoryModel, FileEntry, FileType, SessionState, SortColumn, SortOrder, TabState};
pub use screenshot::{Drawing, EditorState, Tool};
=======
pub mod network;
pub mod battery;
pub mod power;

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
>>>>>>> hard-develop
