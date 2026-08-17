//! Island assembly helpers.
//!
//! `create_system_island` keeps the historical entry point (returns just the
//! notch capsule widget). `build_default_island` additionally returns the
//! [`Island`] manager so callers can register extra views/features or reach
//! the handles via [`crate::island::default_island`].

use crate::features;
use crate::island::{Island, IslandBuilder};

/// Creates the default Dynamic Island (media player + notifications) and
/// returns the notch capsule widget, ready to be appended to the panel.
pub fn create_system_island() -> gtk4::Box {
    build_default_island().capsule()
}

/// Builds the default Dynamic Island (media player + notifications) and
/// returns the manager.
///
/// The same manager is also registered as the process-wide default island
/// (see [`crate::island::default_island`]), so other parts of the shell can
/// register their own features later.
pub fn build_default_island() -> Island {
    Island::builder()
        .feature(Box::new(features::notification::NotificationFeature::new()))
        .feature(Box::new(features::media_player::MediaPlayerFeature::new()))
        .idle(features::default::idle_logo_view())
        .build()
}

/// Returns a bare [`IslandBuilder`] for fully custom island assemblies.
pub fn island_builder() -> IslandBuilder {
    Island::builder()
}
