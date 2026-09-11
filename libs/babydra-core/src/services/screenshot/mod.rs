//! Backend logic for screenshot capturing, regional cropping, annotations (pen, shapes, blur),
//! and clipboard/file saving capabilities.

pub mod capture;
pub mod render;

pub use capture::{capture_fullscreen, capture_screen, get_screenshot_path, trigger_save};
pub use render::{
    apply_stroke_style, draw_pixelated_rect, render_shape, save_cropped_surface, scale_drawing,
};
