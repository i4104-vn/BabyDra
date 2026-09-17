//! Controllers for the recording dynamic island feature.

pub mod actions;
pub mod keyboard;
pub mod wiring;

pub use crate::features::recording::models::PopoverActionsContext;
pub use actions::*;
pub use keyboard::create_keyboard_controller;
pub use wiring::connect_popover_actions;
