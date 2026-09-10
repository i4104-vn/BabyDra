//! UI components for the clipboard island feature.

pub mod notch;
pub mod popover;
pub mod render;
pub mod row;

pub use notch::ClipboardNotchWidgets;
pub use popover::{ClipboardPopover, MAX_VISIBLE_ITEMS};
pub use render::render_popover;
pub use row::ClipboardItemRow;
