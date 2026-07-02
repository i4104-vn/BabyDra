//! Persistent Dynamic Island overlay state model.

use serde::{Serialize, Deserialize};

/// Represents the current active state of the Dynamic Island overlay notch.
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct IslandState {
    /// Whether the island overlay is currently visible.
    pub active: bool,
    /// Primary heading text shown inside the island.
    pub title: String,
    /// Secondary description text shown inside the island.
    pub subtitle: String,
    /// Icon name or path used to decorate the island.
    pub icon: String,
}

