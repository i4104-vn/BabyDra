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

        cr.select_font_face(
            "JetBrains Mono",
            cairo::FontSlant::Normal,
            cairo::FontWeight::Normal,
        );
        cr.set_font_size(11.0);

        let vadj = match tv.vadjustment() {
            Some(a) => a,
            None => return,
        };
        let scroll_y = vadj.value();

        let cursor_iter = tb.iter_at_mark(&tb.get_insert());
        let current_line = cursor_iter.line();

        let (mut iter, _) = tv.line_at_y(scroll_y as i32);
        let top_margin = tv.top_margin() as f64;

        while iter.line() < total_lines {
            let line_idx = iter.line();
            let (line_y, line_h) = tv.line_yrange(&iter);
            let draw_y = (line_y as f64) - scroll_y + top_margin;

            if draw_y > height as f64 {
                break;
            }

            let num_str = format!("{}", line_idx + 1);
            if let Ok(extents) = cr.text_extents(&num_str) {
                let x = (width as f64) - extents.width() - 10.0;
                let y = draw_y + (line_h as f64 + extents.height()) / 2.0;

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
        let digits = format!("{}", tb_clone.line_count()).len();
        let gutter_width = (digits as i32 * 8 + 24).max(44);
        ln.set_size_request(gutter_width, -1);
        ln.queue_draw();
    });
}
