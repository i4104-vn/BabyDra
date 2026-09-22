//! BabyDra TUI Installer — library entry point.
//!
//! The installer ships as a binary (`babydra-installer`) driven by `main`;
//! the library target keeps the pipeline modules (`models`, `core`, `runtime`,
//! `discovery`, `tasks`, `ui`) importable and unit-testable without spawning the TUI.

pub mod app;
pub mod core;
pub mod discovery;
pub mod models;
pub mod runtime;
pub mod system;
pub mod tasks;
pub mod ui;
