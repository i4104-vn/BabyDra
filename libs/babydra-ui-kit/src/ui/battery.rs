//! Shared Cairo Battery Drawing Component used by panel topbar & settings.

use gtk4::prelude::*;
use gtk4::DrawingArea;

/// Returns the hex color string matching the battery state.
pub fn get_battery_hex(percentage: u32, is_charging: bool) -> &'static str {
    if is_charging || percentage > 30 {
        "#22c55e" // Green
    } else if percentage > 15 {
        "#f59e0b" // Amber
    } else {
        "#ef4444" // Red
    }
}

/// Returns Cairo RGB tuples (0.0 .. 1.0) matching the battery state.
pub fn get_battery_rgb(percentage: u32, is_charging: bool) -> (f64, f64, f64) {
    if is_charging || percentage > 30 {
        (0.13, 0.77, 0.36) // Green (#22c55e)
    } else if percentage > 15 {
        (0.96, 0.62, 0.04) // Amber (#f59e0b)
    } else {
        (0.94, 0.27, 0.27) // Red (#ef4444)
    }
}

/// Draw cairo battery.
pub fn draw_cairo_battery(
    cr: &gtk4::cairo::Context,
    width: f64,
    height: f64,
    percentage: u32,
    is_charging: bool,
    is_dark: bool,
) {
    let (fg_r, fg_g, fg_b) = if is_dark {
        (1.0, 1.0, 1.0)
    } else {
        (0.15, 0.15, 0.15)
    };

    let (fill_r, fill_g, fill_b) = get_battery_rgb(percentage, is_charging);

    let shell_x = 1.5;
    let shell_y = 1.5;
    let tip_w = (width * 0.10).clamp(2.0, 6.0);
    let shell_w = width - tip_w - 3.0;
    let shell_h = height - 3.0;

    let corner_r = (shell_h * 0.2).clamp(1.5, 6.0);

    let draw_rr = |x: f64, y: f64, rw: f64, rh: f64, r: f64| {
        let deg = std::f64::consts::PI / 180.0;
        cr.new_sub_path();
        cr.arc(x + rw - r, y + r, r, -90.0 * deg, 0.0 * deg);
        cr.arc(x + rw - r, y + rh - r, r, 0.0 * deg, 90.0 * deg);
        cr.arc(x + r, y + rh - r, r, 90.0 * deg, 180.0 * deg);
        cr.arc(x + r, y + r, r, 180.0 * deg, 270.0 * deg);
        cr.close_path();
    };

    // 1. Draw outer shell border
    draw_rr(shell_x, shell_y, shell_w, shell_h, corner_r);
    cr.set_line_width((height * 0.08).clamp(1.5, 3.0));
    cr.set_source_rgba(fg_r, fg_g, fg_b, 0.85);
    let _ = cr.stroke();

    // 2. Draw terminal tip on right
    let tip_x = shell_x + shell_w + 1.5;
    let tip_h = shell_h * 0.45;
    let tip_y = shell_y + (shell_h - tip_h) / 2.0;
    draw_rr(tip_x, tip_y, tip_w, tip_h, 1.5);
    cr.set_source_rgba(fg_r, fg_g, fg_b, 0.85);
    let _ = cr.fill();

    // 3. Draw inner fill bar
    let pad = (height * 0.10).clamp(2.0, 5.0);
    let fill_max_w = shell_w - (pad * 2.0);
    let fill_h = shell_h - (pad * 2.0);
    let fill_w = (fill_max_w * (percentage as f64 / 100.0)).clamp(2.0, fill_max_w);

    draw_rr(
        shell_x + pad,
        shell_y + pad,
        fill_w,
        fill_h,
        (corner_r - 1.0).max(1.0),
    );
    cr.set_source_rgba(fill_r, fill_g, fill_b, 0.95);
    let _ = cr.fill();

    // 4. Draw lightning bolt if charging
    if is_charging {
        let cx = shell_x + shell_w / 2.0;
        let cy = shell_y + shell_h / 2.0;
        let scale = (height / 24.0).clamp(0.5, 2.5);
        cr.new_path();
        cr.move_to(cx + 1.0 * scale, cy - 5.0 * scale);
        cr.line_to(cx - 3.0 * scale, cy + 1.0 * scale);
        cr.line_to(cx - 0.3 * scale, cy + 1.0 * scale);
        cr.line_to(cx - 1.0 * scale, cy + 5.0 * scale);
        cr.line_to(cx + 3.0 * scale, cy - 1.0 * scale);
        cr.line_to(cx + 0.3 * scale, cy - 1.0 * scale);
        cr.close_path();

        cr.set_source_rgba(1.0, 1.0, 1.0, 0.95);
        let _ = cr.fill();
    }
}

/// Creates a new `battery drawing area`.
pub fn create_battery_area(
    percentage: u32,
    is_charging: bool,
    width: i32,
    height: i32,
) -> DrawingArea {
    let da = DrawingArea::new();
    da.set_content_width(width);
    da.set_content_height(height);
    da.set_valign(gtk4::Align::Center);
    da.set_halign(gtk4::Align::Center);

    da.set_draw_func(move |_area, cr, w, h| {
        let is_dark = crate::ui::theme::is_dark_mode();
        draw_cairo_battery(cr, w as f64, h as f64, percentage, is_charging, is_dark);
    });

    da
}
