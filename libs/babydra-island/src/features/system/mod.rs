//! System Features for the Dynamic Island.
//!
//! This module groups all hardware and desktop status indicator features:
//! - [`volume::VolumeFeature`]: Audio output volume & mute status changes.
//! - [`brightness::BrightnessFeature`]: Display screen backlight & DDC brightness changes.
//!
//! ## Extending with new system features
//!
//! To add a new system feature (e.g. `battery`, `network`, `bluetooth`, `caps_lock`):
//! 1. Create a subdirectory under `features/system/<name>/` with:
//!    - `mod.rs`: Struct implementing [`crate::island::IslandFeature`] (priority: 90-95).
//!    - `service.rs` (optional): Background listener or event monitoring thread.
//! 2. Use [`ui::SystemIndicatorWidget`] for consistent capsule presentation:
//!    icon on the left, feature name in the center, and value/% on the right.
//! 3. Expose the new feature in this file (`pub mod <name>; pub use <name>::...`).
//! 4. Register it in [`crate::render::build_default_island`].

pub mod brightness;
pub mod ui;
pub mod volume;

pub use brightness::BrightnessFeature;
pub use ui::SystemIndicatorWidget;
pub use volume::VolumeFeature;

/// Registers all default system indicator features onto the given [`crate::island::IslandBuilder`].
pub fn register_system_features(
    builder: crate::island::IslandBuilder,
) -> crate::island::IslandBuilder {
    builder
        .feature(Box::new(VolumeFeature::new()))
        .feature(Box::new(BrightnessFeature::new()))
}
