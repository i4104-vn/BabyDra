//! Cairo rendering implementation for drawing the wallpaper and water-drop ripple animation.

use super::animation::ease_out_quart;
use super::surface::draw_surface_aspect_fill;
use gtk4::cairo;

/// Renders the wallpaper with expanding water ripple circle and wave rings.
pub fn render_wallpaper(
    cr: &cairo::Context,
    w: f64,
    h: f64,
    p: f64,
    (rx, ry): (f64, f64),
    old_surf: Option<&cairo::ImageSurface>,
    cur_surf: Option<&cairo::ImageSurface>,
) {
    // 1. Fallback dark background
    cr.set_source_rgb(0.08, 0.09, 0.11);
    let _ = cr.paint();

    // 2. Base layer: previous wallpaper
    if p < 1.0 {
        if let Some(old) = old_surf {
            draw_surface_aspect_fill(cr, old, w, h, 1.0, 1.0);
        }
    }

    // 3. New wallpaper: Expanding circular water drop ripple from random origin
    if let Some(cur) = cur_surf {
        if p < 1.0 {
            let origin_x = w * rx;
            let origin_y = h * ry;

            // Dynamically compute maximum radius to the furthest corner of the screen
            let d1 = (origin_x * origin_x + origin_y * origin_y).sqrt();
            let d2 = ((w - origin_x).powi(2) + origin_y * origin_y).sqrt();
            let d3 = (origin_x * origin_x + (h - origin_y).powi(2)).sqrt();
            let d4 = ((w - origin_x).powi(2) + (h - origin_y).powi(2)).sqrt();
            let max_radius = d1.max(d2).max(d3).max(d4);

            let eased = ease_out_quart(p);
            let current_radius = max_radius * eased;

            // Circular mask clip for new wallpaper
            let _ = cr.save();
            cr.arc(
                origin_x,
                origin_y,
                current_radius,
                0.0,
                2.0 * std::f64::consts::PI,
            );
            cr.clip();
            let zoom = 1.02 - eased * 0.02;
            draw_surface_aspect_fill(cr, cur, w, h, 1.0, zoom);
            let _ = cr.restore();

            // Luminous water ripple wave rings along the circumference
            let wave_alpha = (1.0 - p) * 0.45;
            if wave_alpha > 0.01 && current_radius > 8.0 {
                let _ = cr.save();
                // Primary ripple wavefront
                cr.set_source_rgba(1.0, 1.0, 1.0, wave_alpha);
                cr.set_line_width(3.5 * (1.0 - p * 0.4));
                cr.arc(
                    origin_x,
                    origin_y,
                    current_radius,
                    0.0,
                    2.0 * std::f64::consts::PI,
                );
                let _ = cr.stroke();

                // Secondary subtle trailing ripple
                if current_radius > 25.0 {
                    cr.set_source_rgba(1.0, 1.0, 1.0, wave_alpha * 0.35);
                    cr.set_line_width(1.5);
                    cr.arc(
                        origin_x,
                        origin_y,
                        (current_radius - 12.0).max(0.0),
                        0.0,
                        2.0 * std::f64::consts::PI,
                    );
                    let _ = cr.stroke();
                }
                let _ = cr.restore();
            }
        } else {
            // Fully completed: Render full screen
            draw_surface_aspect_fill(cr, cur, w, h, 1.0, 1.0);
        }
    }
}
