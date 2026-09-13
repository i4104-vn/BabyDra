//! UI topic module: Widget assembly, outside bracket indicators, and size/zoom animations.

pub mod builder;
pub mod notch;
pub mod popover;
pub mod transition;

pub use builder::IslandBuilder;
pub use notch::NotchWidget;
pub use popover::{
    popdown_animated_cb, setup_modal_popover_lifecycle, setup_popover_slide_lifecycle,
    toggle_popover_animated,
};
