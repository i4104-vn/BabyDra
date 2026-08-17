//! BabyDra TUI Installer — library entry point.
//!
//! The installer ships as a binary (`babydra-installer`) driven by
//! [`main`](crate::main); this library target exists so the install
//! pipeline (`models`, `system`, `tasks`, ...) can be exercised from
//! integration tests in `tests/` without spawning the TUI.

pub mod app;
pub mod models;
pub mod system;
pub mod tasks;
pub mod ui;

pub use app::App;
pub use models::{
    BinaryItem, BinaryLocation, GenericOptionItem, InstallState, LogLevel, LogMessage,
    PresetProfile, VariantItem, WizardStep,
};
pub use system::{
    default_binary_source_dir, find_workspace_root, initial_binaries_list,
    initial_configs_themes_options, initial_display_manager_options, initial_package_options,
    initial_variant_options, initial_varlib_options,
};
pub use tasks::{spawn_installation_worker, InstallEvent, InstallPlan};
