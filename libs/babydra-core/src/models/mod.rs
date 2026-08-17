//! Shared configuration and state data models for BabyDra.
//! Categorized into `shell`, `settings`, `explore`, and `screenshot`.

pub mod explore;
pub mod screenshot;
pub mod settings;
pub mod shell;

// Re-export submodules for backward compatibility with `models::<submodule>::...`
pub use settings::app_info;
pub use settings::certificates;
pub use settings::display;
pub use settings::env_var;
pub use settings::hosts;
pub use settings::keybind;
pub use settings::startup_command;
pub use settings::system_info;
pub use settings::system_update;
pub use settings::vpn;
pub use settings::wifi;

// Direct type re-exports
pub use explore::{
    DirectoryModel, FileEntry, FileType, SessionState, SortColumn, SortOrder, TabState,
};
pub use screenshot::{Drawing, EditorState, Tool};
pub use settings::*;
pub use shell::*;
