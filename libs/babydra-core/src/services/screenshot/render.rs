//! Cairo rendering, shape drawing, and pixel mosaic blur utilities for Screenshot editor.

use crate::models::{Drawing, EditorState};
use gdk4::prelude::GdkCairoContextExt;

/// Applies stroke styling shared by every outlined shape.
pub fn apply_stroke_style(cr: &cairo::Context, color: (f64, f64, f64), width: f64) {
    cr.set_source_rgb(color.0, color.1, color.2);
    cr.set_line_width(width);
    cr.set_line_cap(cairo::LineCap::Round);
    cr.set_line_join(cairo::LineJoin::Round);
}

/// Renders a single non-blur drawing with its geometry already mapped into the
/// current context space. Used by both the live canvas and the final export so
/// they can never diverge.
pub fn render_shape(cr: &cairo::Context, drawing: &Drawing) {
    match drawing {
        Drawing::Stroke {
            points,
            color,
            width,
        } => {
            if points.len() < 2 {
                return;
            }
            apply_stroke_style(cr, *color, *width);
            cr.move_to(points[0].0, points[0].1);
            for p in &points[1..] {
                cr.line_to(p.0, p.1);
            }
            cr.stroke().unwrap();
        }
        Drawing::Rect {
            x,
            y,
            w,
            h,
            color,
            width,
        } => {
            apply_stroke_style(cr, *color, *width);
            cr.rectangle(*x, *y, *w, *h);
            cr.stroke().unwrap();
        }
        Drawing::Ellipse {
            x,
            y,
            w,
            h,
            color,
            width,
        } => {
            if *w <= 0.0 || *h <= 0.0 {
                return;
            }
            apply_stroke_style(cr, *color, *width);
            const SEGMENTS: usize = 72;
            let cx = x + w / 2.0;
            let cy = y + h / 2.0;
            let rx = w / 2.0;
            let ry = h / 2.0;
            for i in 0..=SEGMENTS {
                let a = (i as f64) * 2.0 * std::f64::consts::PI / SEGMENTS as f64;
                let px = cx + rx * a.cos();
                let py = cy + ry * a.sin();
                if i == 0 {
                    cr.move_to(px, py);
                } else {
                    cr.line_to(px, py);
                }
            }
            cr.stroke().unwrap();
        }
        Drawing::Line {
            x1,
            y1,
            x2,
            y2,
            color,
            width,
        } => {
            apply_stroke_style(cr, *color, *width);
            cr.move_to(*x1, *y1);
            cr.line_to(*x2, *y2);
            cr.stroke().unwrap();
        }
        Drawing::Arrow {
            x1,
            y1,
            x2,
            y2,
            color,
            width,
        } => {
            apply_stroke_style(cr, *color, *width);
            let angle = (y2 - y1).atan2(x2 - x1);
            let head = (width * 4.5).clamp(12.0, 40.0);
            let spread = std::f64::consts::PI / 7.0;
            cr.move_to(*x1, *y1);
            cr.line_to(x2 - head * angle.cos(), y2 - head * angle.sin());
            cr.stroke().unwrap();
            for dir in [-1.0, 1.0] {
                let a = angle + std::f64::consts::PI + dir * spread;
                cr.move_to(*x2, *y2);
                cr.line_to(x2 + head * a.cos(), y2 + head * a.sin());
                cr.stroke().unwrap();
            }
        }
        Drawing::Blur { .. } => unreachable!("blur is rendered by the caller"),
    }
}

/// Draws a pixelated mosaic filter inside the target rectangle bounds.
pub fn draw_pixelated_rect(
    cr: &cairo::Context,
    bg_pixbuf: &gdk_pixbuf::Pixbuf,
    src: (i32, i32, i32, i32),
    dest: (f64, f64, f64, f64),
) {
    let (src_x, src_y, src_w, src_h) = src;
    let (dest_x, dest_y, dest_w, dest_h) = dest;
    if src_w < 2 || src_h < 2 || dest_w <= 5.0 || dest_h <= 5.0 {
        return;
    }

    cr.save().unwrap();
    cr.rectangle(dest_x, dest_y, dest_w, dest_h);
    cr.clip();

    const MOSAIC_TARGET: f64 = 10.0;
    let sw = ((dest_w / MOSAIC_TARGET).round() as i32).clamp(2, src_w);
    let sh = ((dest_h / MOSAIC_TARGET).round() as i32).clamp(2, src_h);

    let sub_pb = bg_pixbuf.new_subpixbuf(src_x, src_y, src_w, src_h);
    if let Some(scaled_pb) = sub_pb.scale_simple(sw, sh, gdk_pixbuf::InterpType::Hyper) {
        let sx = dest_w / sw as f64;
        let sy = dest_h / sh as f64;
        cr.scale(sx, sy);
        cr.set_source_pixbuf(&scaled_pb, dest_x / sx, dest_y / sy);
        cr.source().set_filter(cairo::Filter::Nearest);
        cr.paint().unwrap();
    }

    cr.restore().unwrap();
}

/// Saves the cropped region of the surface, preserving 100% pixel-perfect native resolution.
pub fn save_cropped_surface(state: &EditorState) -> Option<cairo::ImageSurface> {
    if !state.has_selection || state.crop_w <= 5.0 || state.crop_h <= 5.0 {
        return None;
    }

    let bg_w = state.bg_pixbuf.width();
    let bg_h = state.bg_pixbuf.height();

    let scale_x = if state.canvas_w > 0.0 {
        bg_w as f64 / state.canvas_w
    } else {
        1.0
    };
    let scale_y = if state.canvas_h > 0.0 {
        bg_h as f64 / state.canvas_h
    } else {
        1.0
    };

    let raw_x = ((state.crop_x * scale_x).floor() as i32).clamp(0, bg_w - 1);
    let raw_y = ((state.crop_y * scale_y).floor() as i32).clamp(0, bg_h - 1);
    let raw_w =
        (((state.crop_x + state.crop_w) * scale_x).ceil() as i32).clamp(raw_x + 1, bg_w) - raw_x;
    let raw_h =
        (((state.crop_y + state.crop_h) * scale_y).ceil() as i32).clamp(raw_y + 1, bg_h) - raw_y;

    let sub_pb = state.bg_pixbuf.new_subpixbuf(raw_x, raw_y, raw_w, raw_h);

    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, raw_w, raw_h).ok()?;
    let cr = cairo::Context::new(&surface).ok()?;

    cr.set_source_pixbuf(&sub_pb, 0.0, 0.0);
    cr.source().set_filter(cairo::Filter::Nearest);
    cr.paint().unwrap();

    cr.set_antialias(cairo::Antialias::Best);

    let tx = state.crop_x * scale_x - raw_x as f64;
    let ty = state.crop_y * scale_y - raw_y as f64;
    let line_scale = (scale_x + scale_y) / 2.0;

    for drawing in &state.drawings {
        match drawing {
            Drawing::Blur { x, y, w, h } => {
                let bx = ((x - state.crop_x) * scale_x).floor();
                let by = ((y - state.crop_y) * scale_y).floor();
                let bw = (w * scale_x).ceil();
                let bh = (h * scale_y).ceil();
                let sx = (bx as i32).clamp(0, raw_w - 2);
                let sy = (by as i32).clamp(0, raw_h - 2);
                let sw = (bw as i32).clamp(2, raw_w - sx);
                let sh = (bh as i32).clamp(2, raw_h - sy);
                draw_pixelated_rect(&cr, &sub_pb, (sx, sy, sw, sh), (bx, by, bw, bh));
            }
            other => {
                cr.save().unwrap();
                cr.translate(-tx, -ty);
                cr.scale(scale_x, scale_y);
                render_shape(&cr, &scale_drawing(other, line_scale));
                cr.restore().unwrap();
            }
        }
    }

    Some(surface)
}

/// Produces a copy of the drawing with stroke widths multiplied by `line_scale`.
pub fn scale_drawing(drawing: &Drawing, line_scale: f64) -> Drawing {
    match drawing {
        Drawing::Stroke {
            points,
            color,
            width,
        } => Drawing::Stroke {
            points: points.clone(),
            color: *color,
            width: width * line_scale,
        },
        Drawing::Rect {
            x,
            y,
            w,
            h,
            color,
            width,
        } => Drawing::Rect {
            x: *x,
            y: *y,
            w: *w,
            h: *h,
            color: *color,
            width: width * line_scale,
        },
        Drawing::Ellipse {
            x,
            y,
            w,
            h,
            color,
            width,
        } => Drawing::Ellipse {
            x: *x,
            y: *y,
            w: *w,
            h: *h,
            color: *color,
            width: width * line_scale,
        },
        Drawing::Line {
            x1,
            y1,
            x2,
            y2,
            color,
            width,
        } => Drawing::Line {
            x1: *x1,
            y1: *y1,
            x2: *x2,
            y2: *y2,
            color: *color,
            width: width * line_scale,
        },
        Drawing::Arrow {
            x1,
            y1,
            x2,
            y2,
            color,
            width,
        } => Drawing::Arrow {
            x1: *x1,
            y1: *y1,
            x2: *x2,
            y2: *y2,
            color: *color,
            width: width * line_scale,
        },
        Drawing::Blur { .. } => drawing.clone(),
    }
}
