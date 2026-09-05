//! Cairo drawing for Desktop Ethernet status icon with TX/RX activity LED indicators.

use std::f64::consts::PI;

/// Draws the desktop monitor icon with two vertical status LED dots at the bottom-right.
///
/// - Desktop monitor: frame, inner display, stand neck, and base foot.
/// - Top dot: Upload (TX) in Yellow/Amber.
/// - Bottom dot: Download (RX) in Green.
/// - Normal state: both dots are solid.
/// - Blinking state: active dot alternates between solid and dim.
pub fn draw_cairo_ethernet_desktop(
    cr: &gtk4::cairo::Context,
    width: f64,
    _height: f64,
    is_dark: bool,
    is_connected: bool,
    tx_lit: bool,
    rx_lit: bool,
) {
    let (fg_r, fg_g, fg_b) = if is_dark {
        (1.0, 1.0, 1.0)
    } else {
        (0.12, 0.12, 0.14)
    };

    let base_alpha = if is_connected { 0.85 } else { 0.35 };

    // Helper to draw a rounded rectangle
    let draw_rr = |x: f64, y: f64, rw: f64, rh: f64, r: f64| {
        let deg = PI / 180.0;
        cr.new_sub_path();
        cr.arc(x + rw - r, y + r, r, -90.0 * deg, 0.0 * deg);
        cr.arc(x + rw - r, y + rh - r, r, 0.0 * deg, 90.0 * deg);
        cr.arc(x + r, y + rh - r, r, 90.0 * deg, 180.0 * deg);
        cr.arc(x + r, y + r, r, 180.0 * deg, 270.0 * deg);
        cr.close_path();
    };

    // 1. Desktop Monitor Geometry
    // Total available area: ~17px wide x 14px high
    // Monitor screen sits on the left: x = 0.8 to 12.2 (w = 11.4, h = 8.0)
    let screen_x = 0.8;
    let screen_y = 1.0;
    let screen_w = 11.4;
    let screen_h = 8.0;
    let screen_r = 1.2;

    // Draw inner display screen subtle fill
    draw_rr(
        screen_x + 1.0,
        screen_y + 1.0,
        screen_w - 2.0,
        screen_h - 2.0,
        0.8,
    );
    let inner_alpha = if is_dark { 0.08 } else { 0.05 };
    cr.set_source_rgba(fg_r, fg_g, fg_b, inner_alpha);
    let _ = cr.fill();

    // Draw monitor outer screen border
    draw_rr(screen_x, screen_y, screen_w, screen_h, screen_r);
    cr.set_line_width(1.1);
    cr.set_source_rgba(fg_r, fg_g, fg_b, base_alpha);
    let _ = cr.stroke();

    // Stand neck
    let neck_x = screen_x + screen_w / 2.0;
    let neck_top_y = screen_y + screen_h;
    let neck_bot_y = neck_top_y + 2.2;
    cr.set_line_width(1.1);
    cr.move_to(neck_x, neck_top_y);
    cr.line_to(neck_x, neck_bot_y);
    cr.set_source_rgba(fg_r, fg_g, fg_b, base_alpha);
    let _ = cr.stroke();

    // Stand base foot
    let base_y = neck_bot_y + 0.3;
    let base_half_w = 2.6;
    cr.set_line_cap(gtk4::cairo::LineCap::Round);
    cr.move_to(neck_x - base_half_w, base_y);
    cr.line_to(neck_x + base_half_w, base_y);
    cr.set_source_rgba(fg_r, fg_g, fg_b, base_alpha);
    let _ = cr.stroke();
    cr.set_line_cap(gtk4::cairo::LineCap::Butt);

    // 2. Activity Indicator Dots (LEDs) arranged vertically at the bottom-right
    let dot_r = 1.25;
    let dot_cx = (width - 2.0).max(14.8); // ~15.0 on a 17px canvas

    // Bottom dot (RX) placed flush near the bottom, aligned with the monitor base foot
    let bot_dot_cy = base_y + 0.1; // ~11.6
    // Top dot (TX) placed directly above the bottom dot
    let top_dot_cy = bot_dot_cy - 3.4; // ~8.2

    let draw_led = |cx: f64, cy: f64, (r, g, b): (f64, f64, f64), is_lit: bool| {
        let alpha = if !is_connected {
            0.15
        } else if is_lit {
            0.95
        } else {
            0.18
        };

        // Subtle glow effect when lit
        if is_connected && is_lit {
            cr.new_sub_path();
            cr.arc(cx, cy, dot_r + 0.7, 0.0, 2.0 * PI);
            cr.set_source_rgba(r, g, b, 0.28);
            let _ = cr.fill();
        }

        // Main solid dot
        cr.new_sub_path();
        cr.arc(cx, cy, dot_r, 0.0, 2.0 * PI);
        cr.set_source_rgba(r, g, b, alpha);
        let _ = cr.fill();
    };

    // Dot 1: Upload (TX) -> Yellow/Amber (#f59e0b) - Top dot
    draw_led(dot_cx, top_dot_cy, (0.96, 0.62, 0.04), tx_lit);

    // Dot 2: Download (RX) -> Green (#22c55e) - Bottom dot
    draw_led(dot_cx, bot_dot_cy, (0.13, 0.77, 0.36), rx_lit);
}
