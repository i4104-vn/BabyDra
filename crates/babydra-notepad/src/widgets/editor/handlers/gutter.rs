//! Gutter line numbers drawing handler using Cairo.

use gtk4::prelude::*;
use gtk4::{DrawingArea, ScrolledWindow, TextBuffer, TextView};

/// Sets up Cairo drawing on the line numbers gutter and attaches redraw triggers.
pub fn setup_gutter_drawing(
    line_numbers: &DrawingArea,
    text_view: &TextView,
    text_buffer: &TextBuffer,
    scrolled_window: &ScrolledWindow,
) {
    let tv = text_view.clone();
    let tb = text_buffer.clone();

    line_numbers.set_draw_func(move |_area, cr, width, height| {
        let is_dark = babydra_ui_kit::ui::theme::is_dark_mode();

        // Gutter background
        if is_dark {
            cr.set_source_rgba(0.05, 0.05, 0.07, 0.95);
        } else {
            cr.set_source_rgba(0.96, 0.96, 0.97, 0.95);
        }
        let _ = cr.paint();

        // Right separator border
        if is_dark {
            cr.set_source_rgba(1.0, 1.0, 1.0, 0.08);
        } else {
            cr.set_source_rgba(0.0, 0.0, 0.0, 0.08);
        }
        cr.set_line_width(1.0);
        cr.move_to(width as f64 - 0.5, 0.0);
        cr.line_to(width as f64 - 0.5, height as f64);
        let _ = cr.stroke();

        let total_lines = tb.line_count();
        if total_lines == 0 {
            return;
        }

        let settings = babydra_core::models::notepad::load_notepad_cfg();
        // Settings font_size is in points; Cairo set_font_size uses pixels.
        // Convert: px = pt × (screen_dpi / 72).  Assume 96 dpi → factor ≈ 1.333.
        let pt_to_px = 96.0 / 72.0;
        let font_size_px = (settings.font_size as f64 * pt_to_px).clamp(10.0, 28.0);

        cr.select_font_face(
            &settings.font_family,
            cairo::FontSlant::Normal,
            cairo::FontWeight::Normal,
        );
        cr.set_font_size(font_size_px);

        let vadj = match tv.vadjustment() {
            Some(a) => a,
            None => return,
        };
        let scroll_y = vadj.value();

        let cursor_iter = tb.iter_at_mark(&tb.get_insert());
        let current_line = cursor_iter.line();

        let (mut iter, _) = tv.line_at_y(scroll_y as i32);
        let top_margin = tv.top_margin() as f64;
        // `hosts-editor-text` has 12px CSS padding.  line_yrange() is
        // relative to the TextView content, while the gutter starts at the
        // outer widget edge, so account for that padding here as well.
        let text_view_padding = 12.0;

        while iter.line() < total_lines {
            let line_idx = iter.line();
            let (line_y, line_h) = tv.line_yrange(&iter);

            // Convert buffer coordinates to visible (widget-relative) coordinates
            let draw_y = (line_y as f64) - scroll_y + top_margin + text_view_padding;
            let draw_h = line_h as f64;

            // Skip lines that are below the visible area
            if draw_y > height as f64 {
                break;
            }

            // Skip lines that are above the visible area
            if draw_y + draw_h < 0.0 {
                if !iter.forward_line() {
                    break;
                }
                continue;
            }

            let num_str = format!("{}", line_idx + 1);
            if let Ok(extents) = cr.text_extents(&num_str) {
                let x = (width as f64) - extents.width() - 14.0;
                // Center the glyph vertically within the line height:
                // baseline = line_center - glyph_visual_center_offset + nudge
                let line_center = draw_y + draw_h / 2.0;
                let y = line_center - extents.y_bearing() - extents.height() / 2.0 + 1.0;

                cr.move_to(x, y);
                if line_idx == current_line {
                    if is_dark {
                        cr.set_source_rgba(0.92, 0.94, 0.98, 0.95);
                    } else {
                        cr.set_source_rgba(0.10, 0.12, 0.16, 0.95);
                    }
                } else {
                    if is_dark {
                        cr.set_source_rgba(0.40, 0.45, 0.55, 0.70);
                    } else {
                        cr.set_source_rgba(0.55, 0.60, 0.68, 0.85);
                    }
                }
                let _ = cr.show_text(&num_str);
            }

            if !iter.forward_line() {
                break;
            }
        }
    });

    let vadj = scrolled_window.vadjustment();
    let ln = line_numbers.clone();
    vadj.connect_value_changed(move |_| {
        ln.queue_draw();
    });

    let ln = line_numbers.clone();
    let tb_clone = text_buffer.clone();
    text_buffer.connect_changed(move |_| {
        let settings = babydra_core::models::notepad::load_notepad_cfg();
        let pt_to_px = 96.0 / 72.0;
        let font_size_px = (settings.font_size as f64 * pt_to_px).clamp(10.0, 28.0);
        let digits = format!("{}", tb_clone.line_count()).len();
        let char_width = font_size_px * 0.62;
        let gutter_width = ((digits as f64 * char_width) + 28.0).max(48.0) as i32;
        ln.set_size_request(gutter_width, -1);
        ln.queue_draw();
    });
}
