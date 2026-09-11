//! System appearance and desktop theme data models.

use serde::{Deserialize, Serialize};

/// Represents the active GTK and icon appearance settings.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CurrentAppearance {
    pub gtk_theme: String,
    pub icon_theme: String,
    pub cursor_theme: String,
    pub cursor_size: u32,
}
