//! Image viewport and panning state models.

use gdk_pixbuf::Pixbuf;

/// Image viewport, zoom, and panning interaction state.
#[derive(Clone)]
pub struct ImageState {
    pub pixbuf: Pixbuf,
    pub scale: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub min_scale: f64,
    pub img_w: f64,
    pub img_h: f64,
    pub drag_start_x: f64,
    pub drag_start_y: f64,
}
