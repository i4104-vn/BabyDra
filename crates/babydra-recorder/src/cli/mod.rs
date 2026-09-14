//! Command-line argument parsing and invocation.

pub mod handler;

pub use handler::{handle_cli_flag, is_daemon_already_running, try_activate_running_instance};
