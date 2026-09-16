//! Animation controller engine and transition helpers.

pub mod easing;
pub mod genie;
pub mod island;
pub mod slide;
pub mod topbar;

pub use genie::{genie_in, genie_out};
pub use island::{
    current_anim_gen, island_animate_size, island_animate_width, island_zoom_in, island_zoom_out,
    next_anim_gen,
};
pub use slide::{
    slide_in, slide_in_cancelable, slide_out, slide_out_cb, slide_out_cb_cancelable, SlideDirection,
};
pub use topbar::topbar_startup_cascade;
