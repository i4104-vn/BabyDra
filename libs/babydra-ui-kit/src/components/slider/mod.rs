//! Interactive slider components.
//!
//! Provides `PillSlider` (for audio volume / mic controls) and `CustomSlider`
//! (for stepped thresholds like battery charging).

pub mod custom;
pub mod debounced;
pub mod helpers;
pub mod pill;

pub use custom::CustomSlider;
pub use debounced::bind_debounced_slider;
pub use helpers::{draw_rounded_rect, snap_to_step};
pub use pill::PillSlider;
