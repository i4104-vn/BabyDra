//! Wi-Fi background workers, polling, and signal orchestration.

pub mod connection;
pub mod dialogs;
pub mod scanner;
pub mod status;

pub use connection::setup_connection_manager;
pub use dialogs::wire_dialog_signals;
pub use scanner::setup_scanner;
pub use status::setup_status_sync;
