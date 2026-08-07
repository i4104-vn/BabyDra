//! Shared configuration and state data models for BabyDra.
//! Categorized into `shell`, `settings`, `explore`, and `screenshot`.

pub mod shell;
pub mod settings;
pub mod explore;
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
