//! Screenshot-backed blur surfaces for shell components (Disabled for solid opaque interface).

use gtk4::prelude::*;

/// Runs `grim` to capture a specific screen region (disabled).
pub fn capture_screen_bytes(
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _scale: i32,
) -> Option<Vec<u8>> {
    None
}

/// Decodes PNG bytes and applies a multi-pass bilinear frosted blur (disabled).
pub fn blur_pixbuf_from_bytes(_bytes: &[u8]) -> Option<gdk_pixbuf::Pixbuf> {
    None
}

/// Direct helper to capture a specific screen region via `grim` and return
/// a high-quality blurred Pixbuf (disabled).
pub fn capture_screen_region(
    _x: i32,
    _y: i32,
    _width: i32,
    _height: i32,
    _scale: i32,
) -> Option<gdk_pixbuf::Pixbuf> {
    None
}

/// Computes the monitor-relative global coordinates and dimensions for capturing
/// the screen region behind `widget` (disabled).
pub fn compute_widget_region(_widget: &gtk4::Widget) -> Option<(i32, i32, i32, i32, i32)> {
    None
}

/// Captures a blurred screenshot backdrop behind the specified widget (disabled).
pub fn capture_blurred_widget(_widget: &gtk4::Widget) -> Option<gdk_pixbuf::Pixbuf> {
    None
}

/// Wraps a component in an overlay container. Blur is disabled; components have
/// solid, smooth, opaque backgrounds.
pub fn with_blurred_backdrop<W: IsA<gtk4::Widget>>(content: &W) -> gtk4::Overlay {
    let overlay = gtk4::Overlay::new();
    overlay.add_css_class("blur-backdrop-host");
    let content_widget = content.clone().upcast::<gtk4::Widget>();
    overlay.set_child(Some(&content_widget));
    overlay
}

/// Manually refreshes the blurred backdrop image inside an overlay host (no-op).
pub fn refresh_blurred_backdrop(_overlay: &gtk4::Overlay) {}

/// Adds screenshot-backed blur to popovers created by the shared UI kit (no-op).
pub fn enable_popover_blur(_popover: &gtk4::Popover) {}


