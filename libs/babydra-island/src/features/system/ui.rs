//! Re-export shared notch capsule widget for system indicators (volume, brightness, etc.).

pub use crate::island::ui::NotchWidget;

/// Backwards-compatible type alias for [`NotchWidget`].
pub type SystemIndicatorWidget = NotchWidget;
