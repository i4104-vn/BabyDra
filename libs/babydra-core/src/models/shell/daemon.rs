//! Daemon lifecycle message models shared by shell components.

/// Messages sent from the background socket listener thread to the GTK main
/// thread to control window visibility (used by alt-tab style switchers).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DaemonMessage {
    /// Show the switcher window (cycle to next if already visible)
    ShowOrNext,
    /// Hide/close the switcher window (Alt was released)
    Hide,
}
