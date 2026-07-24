//! High-performance smooth Gaussian Blur & Acrylic background generator for GTK4 windows.

use gtk4::prelude::*;
use std::sync::Arc;

/// Captures the Wayland display background using `grim`.
pub fn capture_screen_pixbuf() -> Option<gdk_pixbuf::Pixbuf> {
    let temp_path = format!("/tmp/babydra-blur-cap-{}.png", std::process::id());
    let _ = std::fs::remove_file(&temp_path);

    let status = std::process::Command::new("grim")
        .arg(&temp_path)
        .status();

    if let Ok(s) = status {
        if s.success() {
            let pb = gdk_pixbuf::Pixbuf::from_file(&temp_path).ok();
            let _ = std::fs::remove_file(&temp_path);
            return pb;
        }
    }
    let _ = std::fs::remove_file(&temp_path);
    None
}

/// Creates a smooth Gaussian blurred image from a screen Pixbuf cropped at (x, y, w, h).
/// Downscales by 12x with bilinear filtering then upscales back to produce a clean 24px Acrylic blur.
pub fn create_blurred_crop(
    screen_pb: &gdk_pixbuf::Pixbuf,
    crop_x: i32,
    crop_y: i32,
    crop_w: i32,
    crop_h: i32,
) -> Option<gdk_pixbuf::Pixbuf> {
    let sw = screen_pb.width();
    let sh = screen_pb.height();

    let rx = crop_x.clamp(0, (sw - 10).max(0));
    let ry = crop_y.clamp(0, (sh - 10).max(0));
    let rw = crop_w.min(sw - rx);
    let rh = crop_h.min(sh - ry);

    if rw <= 10 || rh <= 10 {
        return None;
    }

    let sub = screen_pb.new_subpixbuf(rx, ry, rw, rh);

    let down_w = (rw / 12).max(4);
    let down_h = (rh / 12).max(4);

    let small = sub.scale_simple(down_w, down_h, gdk_pixbuf::InterpType::Bilinear)?;
    small.scale_simple(rw, rh, gdk_pixbuf::InterpType::Bilinear)
}

/// Fallback: Loads wallpaper from ~/.config/babydra/wallpaper.png if grim fails or unavailable.
pub fn get_wallpaper_pixbuf() -> Option<gdk_pixbuf::Pixbuf> {
    let home = glib::home_dir();
    let wp_path = home.join(".config/babydra/wallpaper.png");
    if wp_path.exists() {
        gdk_pixbuf::Pixbuf::from_file(wp_path).ok()
    } else {
        None
    }
}

/// Draws rounded rectangle path into Cairo context.
fn path_rounded_rectangle(cr: &cairo::Context, x: f64, y: f64, width: f64, height: f64, radius: f64) {
    let degrees = std::f64::consts::PI / 180.0;
    cr.new_sub_path();
    cr.arc(x + width - radius, y + radius, radius, -90.0 * degrees, 0.0 * degrees);
    cr.arc(x + width - radius, y + height - radius, radius, 0.0 * degrees, 90.0 * degrees);
    cr.arc(x + radius, y + height - radius, radius, 90.0 * degrees, 180.0 * degrees);
    cr.arc(x + radius, y + radius, radius, 180.0 * degrees, 270.0 * degrees);
    cr.close_path();
}

/// Creates a DrawingArea widget that renders a smooth blurred background with tint & border.
pub fn create_blur_background_widget(
    crop_x: i32,
    crop_y: i32,
    width: i32,
    height: i32,
    radius: f64,
) -> gtk4::DrawingArea {
    let drawing_area = gtk4::DrawingArea::new();

    // Capture screen 1-shot in a background thread or synchronously
    let screen_pb = capture_screen_pixbuf().or_else(get_wallpaper_pixbuf);
    let blurred_pb = screen_pb.and_then(|pb| create_blurred_crop(&pb, crop_x, crop_y, width, height));
    let blurred_arc = Arc::new(blurred_pb);

    drawing_area.set_draw_func(move |_, cr, w_out, h_out| {
        let w = w_out as f64;
        let h = h_out as f64;
        if w <= 0.0 || h <= 0.0 {
            return;
        }

        let is_dark = super::theme::is_dark_mode();

        // 1. Clip to rounded rectangle
        path_rounded_rectangle(cr, 0.0, 0.0, w, h, radius);
        let _ = cr.clip();

        // 2. Draw blurred background if captured
        if let Some(ref pb) = *blurred_arc {
            cr.set_source_pixbuf(pb, 0.0, 0.0);
            let _ = cr.paint();
        }

        // 3. Draw translucent color tint overlay (Acrylic style)
        if is_dark {
            cr.set_source_rgba(14.0 / 255.0, 14.0 / 255.0, 18.0 / 255.0, 0.72);
        } else {
            cr.set_source_rgba(245.0 / 255.0, 245.0 / 255.0, 250.0 / 255.0, 0.75);
        }
        let _ = cr.paint();

        // 4. Subtle 1px border highlight
        path_rounded_rectangle(cr, 0.5, 0.5, w - 1.0, h - 1.0, radius);
        if is_dark {
            cr.set_source_rgba(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 0.16);
        } else {
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.12);
        }
        cr.set_line_width(1.0);
        let _ = cr.stroke();
    });

    drawing_area
}

/// Wraps a content widget inside an Overlay with a real smooth Acrylic blur background.
pub fn wrap_with_acrylic_blur<W: IsA<gtk4::Widget>>(
    content: &W,
    crop_x: i32,
    crop_y: i32,
    width: i32,
    height: i32,
    border_radius: f64,
) -> gtk4::Overlay {
    let overlay = gtk4::Overlay::new();
    let bg_widget = create_blur_background_widget(crop_x, crop_y, width, height, border_radius);

    overlay.set_child(Some(&bg_widget));
    overlay.add_overlay(content);
    overlay
}
