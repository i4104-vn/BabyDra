//! Island configuration and display states.

/// Size of the idle logo pill when `IslandConfig::idle_visible` is enabled.
pub const IDLE_SIZE: (i32, i32) = (28, 16);

/// Configuration for the island manager.
#[derive(Clone, Debug)]
pub struct IslandConfig {
    /// Show the idle logo pill when no view is active (default: hidden).
    pub idle_visible: bool,
    /// Controller loop interval in milliseconds.
    pub poll_interval_ms: u64,
    /// Expand animation duration in milliseconds.
    pub expand_ms: u64,
    /// Collapse animation duration in milliseconds.
    pub collapse_ms: u64,
}

impl Default for IslandConfig {
    fn default() -> Self {
        Self {
            idle_visible: false,
            poll_interval_ms: 150,
            expand_ms: 350,
            collapse_ms: 500,
        }
    }
}

/// What the island currently displays.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IslandDisplay {
    Hidden,
    Idle,
    View(usize),
}
