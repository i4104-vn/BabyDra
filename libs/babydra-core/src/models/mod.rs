//! Shared configuration and state data models for BabyDra.
//! Categorized into `shell`, `settings`, `explore`, and `screenshot`.

pub mod explore;
pub mod screenshot;
pub mod settings;
pub mod shell;

// Re-export submodules for backward compatibility with `models::<submodule>::...`
pub use settings::app_info;
pub use settings::bluetooth;
pub use settings::certificates;
pub use settings::display;
pub use settings::env_var;
pub use settings::hosts;
pub use settings::keybind;
pub use settings::nav;
pub use settings::startup_command;
pub use settings::system_info;
pub use settings::system_update;
pub use settings::vpn;
pub use settings::wifi;

pub use shell::app;
pub use shell::appearance;
pub use shell::battery;
pub use shell::dbusmenu;
pub use shell::exif;
pub use shell::island_state;
pub use shell::monitor;
pub use shell::network;
pub use shell::notification;
pub use shell::power;
pub use shell::shell_config;
pub use shell::storage;
pub use shell::theme_config;
pub use shell::tray_item;
pub use shell::volume;

// Direct type re-exports
pub use explore::{
    DirectoryModel, FileEntry, FileType, SessionState, SortColumn, SortOrder, TabState,
};
pub use screenshot::{Drawing, EditorState, Tool};
pub use settings::*;
pub use shell::*;
