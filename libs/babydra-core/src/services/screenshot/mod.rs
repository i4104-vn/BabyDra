//! Backend logic for screenshot capturing, regional cropping, annotations (pen, rectangles, blur),
//! and clipboard/file saving capabilities.

use crate::models::{Drawing, EditorState};
use gdk4::prelude::GdkCairoContextExt;
use std::path::PathBuf;

/// Draws a pixelated mosaic filter inside the target rectangle bounds.
pub fn draw_pixelated_rect(
    cr: &cairo::Context,
    bg_pixbuf: &gdk_pixbuf::Pixbuf,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) {
    if w <= 5.0 || h <= 5.0 {
        return;
    }

    cr.save().unwrap();
    cr.rectangle(x, y, w, h);
    cr.clip();

    let scale = 0.08;
    let sw = (w * scale).max(2.0) as i32;
    let sh = (h * scale).max(2.0) as i32;

    let sub_pb = bg_pixbuf.new_subpixbuf(x as i32, y as i32, w as i32, h as i32);
    if let Some(scaled_pb) = sub_pb.scale_simple(sw, sh, gdk_pixbuf::InterpType::Hyper) {
        cr.scale(1.0 / scale, 1.0 / scale);
        cr.set_source_pixbuf(&scaled_pb, x * scale, y * scale);
        cr.source().set_filter(cairo::Filter::Nearest);
        cr.paint().unwrap();
    }

    cr.restore().unwrap();
}

/// Resolves the default file path to save screenshots in `~/Pictures/Screenshots/`.
pub fn get_screenshot_path() -> PathBuf {
    let pictures_dir = dirs::picture_dir().unwrap_or_else(|| {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        home.join("Pictures")
    });
    let screenshots_dir = pictures_dir.join("Screenshots");
    let _ = std::fs::create_dir_all(&screenshots_dir);

    let datetime = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    screenshots_dir.join(format!("Screenshot_{}.png", datetime))
}

/// Applies an unsharp mask sharpening filter to an ImageSurface buffer in-place.
pub fn apply_unsharp_mask(surface: &mut cairo::ImageSurface, amount: f32) {
    let width = surface.width() as usize;
    let height = surface.height() as usize;
    let stride = surface.stride() as usize;

    if width < 3 || height < 3 {
        return;
    }

    surface.flush();
    if let Ok(mut data) = surface.data() {
        let src = data.to_vec();
        // 3x3 Gaussian-like blur + unsharp difference enhancement
        for y in 1..(height - 1) {
            let row_offset = y * stride;
            let prev_row = (y - 1) * stride;
            let next_row = (y + 1) * stride;

            for x in 1..(width - 1) {
                let idx = row_offset + x * 4;
                for c in 0..3 {
                    let center = src[idx + c] as f32;
                    let neighbors = (src[prev_row + (x - 1) * 4 + c] as f32
                        + src[prev_row + x * 4 + c] as f32 * 2.0
                        + src[prev_row + (x + 1) * 4 + c] as f32
                        + src[row_offset + (x - 1) * 4 + c] as f32 * 2.0
                        + src[row_offset + (x + 1) * 4 + c] as f32 * 2.0
                        + src[next_row + (x - 1) * 4 + c] as f32
                        + src[next_row + x * 4 + c] as f32 * 2.0
                        + src[next_row + (x + 1) * 4 + c] as f32)
                        / 12.0;

                    let diff = center - neighbors;
                    let enhanced = center + amount * diff;
                    data[idx + c] = enhanced.clamp(0.0, 255.0) as u8;
                }
            }
        }
    }
    surface.mark_dirty();
}

/// Invokes `grim` to capture the Wayland display screen at maximum quality and save it to a temporary file.
pub fn capture_screen() -> Option<String> {
    let temp_path = "/tmp/babydra-screenshot-temp.png";
    let _ = std::fs::remove_file(temp_path);

    let status = std::process::Command::new("grim")
        .args(["-t", "png", "-l", "1", temp_path])
        .status();

    match status {
        Ok(s) if s.success() => Some(temp_path.to_string()),
        _ => None,
    }
}

/// Saves the cropped region of the surface at high resolution (with 2x HD upscale & sharpening if enabled).
pub fn save_cropped_surface(state: &EditorState) -> Option<cairo::ImageSurface> {
    if !state.has_selection || state.crop_w <= 5.0 || state.crop_h <= 5.0 {
        return None;
    }

    let bg_w = state.bg_pixbuf.width() as f64;
    let bg_h = state.bg_pixbuf.height() as f64;

    // Coordinate mapping between GTK DrawingArea and raw Pixbuf
    let scale_x = if state.canvas_w > 0.0 {
        bg_w / state.canvas_w
    } else {
        1.0
    };
    let scale_y = if state.canvas_h > 0.0 {
        bg_h / state.canvas_h
    } else {
        1.0
    };

    let raw_x = (state.crop_x * scale_x).clamp(0.0, bg_w);
    let raw_y = (state.crop_y * scale_y).clamp(0.0, bg_h);
    let raw_w = (state.crop_w * scale_x).min(bg_w - raw_x).max(1.0);
    let raw_h = (state.crop_h * scale_y).min(bg_h - raw_y).max(1.0);

    let upscale_factor = if state.upscale { 2.0 } else { 1.0 };
    let out_w = ((raw_w * upscale_factor).round() as i32).max(1);
    let out_h = ((raw_h * upscale_factor).round() as i32).max(1);

    let mut surface = cairo::ImageSurface::create(cairo::Format::ARgb32, out_w, out_h).ok()?;
    let cr = cairo::Context::new(&surface).ok()?;

    cr.set_antialias(cairo::Antialias::Best);

    // Render high-res cropped background with Hyper (Lanczos) filtering
    let sub_x = (raw_x.floor() as i32).clamp(0, (state.bg_pixbuf.width() - 1).max(0));
    let sub_y = (raw_y.floor() as i32).clamp(0, (state.bg_pixbuf.height() - 1).max(0));
    let sub_w = (raw_w.ceil() as i32)
        .min(state.bg_pixbuf.width() - sub_x)
        .max(1);
    let sub_h = (raw_h.ceil() as i32)
        .min(state.bg_pixbuf.height() - sub_y)
        .max(1);

    let sub_pb = state.bg_pixbuf.new_subpixbuf(sub_x, sub_y, sub_w, sub_h);
    if let Some(scaled_pb) = sub_pb.scale_simple(out_w, out_h, gdk_pixbuf::InterpType::Hyper) {
        cr.set_source_pixbuf(&scaled_pb, 0.0, 0.0);
        cr.source().set_filter(cairo::Filter::Best);
        cr.paint().unwrap();
    }

    // Render annotations at target upscale resolution
    let factor_x = upscale_factor * scale_x;
    let factor_y = upscale_factor * scale_y;

    for drawing in &state.drawings {
        match drawing {
            Drawing::Blur { x, y, w, h } => {
                let bx = (x - state.crop_x) * factor_x;
                let by = (y - state.crop_y) * factor_y;
                let bw = w * factor_x;
                let bh = h * factor_y;
                if bw > 2.0 && bh > 2.0 {
                    let sub_x = (raw_x + (x - state.crop_x) * scale_x).clamp(0.0, bg_w - 1.0);
                    let sub_y = (raw_y + (y - state.crop_y) * scale_y).clamp(0.0, bg_h - 1.0);
                    let sub_w = (w * scale_x).min(bg_w - sub_x).max(1.0);
                    let sub_h = (h * scale_y).min(bg_h - sub_y).max(1.0);

                    let sub_pb = state.bg_pixbuf.new_subpixbuf(
                        sub_x as i32,
                        sub_y as i32,
                        sub_w as i32,
                        sub_h as i32,
                    );
                    let pixel_scale = 0.08;
                    let sw = (bw * pixel_scale).max(2.0) as i32;
                    let sh = (bh * pixel_scale).max(2.0) as i32;
                    if let Some(small_pb) =
                        sub_pb.scale_simple(sw, sh, gdk_pixbuf::InterpType::Hyper)
                    {
                        cr.save().unwrap();
                        cr.rectangle(bx, by, bw, bh);
                        cr.clip();
                        cr.scale(1.0 / pixel_scale, 1.0 / pixel_scale);
                        cr.set_source_pixbuf(&small_pb, bx * pixel_scale, by * pixel_scale);
                        cr.source().set_filter(cairo::Filter::Nearest);
                        cr.paint().unwrap();
                        cr.restore().unwrap();
                    }
                }
            }
            Drawing::Stroke {
                points,
                color,
                width,
            } => {
                if points.len() < 2 {
                    continue;
                }
                cr.set_source_rgb(color.0, color.1, color.2);
                cr.set_line_width(*width * ((factor_x + factor_y) / 2.0));
                cr.set_line_cap(cairo::LineCap::Round);
                cr.set_line_join(cairo::LineJoin::Round);
                let p0x = (points[0].0 - state.crop_x) * factor_x;
                let p0y = (points[0].1 - state.crop_y) * factor_y;
                cr.move_to(p0x, p0y);
                for p in &points[1..] {
                    let px = (p.0 - state.crop_x) * factor_x;
                    let py = (p.1 - state.crop_y) * factor_y;
                    cr.line_to(px, py);
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
                let rx = (x - state.crop_x) * factor_x;
                let ry = (y - state.crop_y) * factor_y;
                let rw = w * factor_x;
                let rh = h * factor_y;
                cr.set_source_rgb(color.0, color.1, color.2);
                cr.set_line_width(*width * ((factor_x + factor_y) / 2.0));
                cr.rectangle(rx, ry, rw, rh);
                cr.stroke().unwrap();
            }
        }
    }

    // Apply edge-preserving unsharp mask pass to give crystal-clear sharpness
    if state.upscale {
        apply_unsharp_mask(&mut surface, 0.40);
    }

    Some(surface)
}

/// Writes the cropped annotated screenshot to a local PNG file and displays a desktop notification.
pub fn trigger_save(state: &EditorState) -> bool {
    if let Some(surface) = save_cropped_surface(state) {
        let save_path = get_screenshot_path();
        if let Ok(mut file) = std::fs::File::create(&save_path) {
            if surface.write_to_png(&mut file).is_ok() {
                let notif_title = crate::i18n::trans("screenshot.saved_title");
                let notif_msg = crate::i18n::trans("screenshot.saved_msg")
                    .replace("{}", &format!("{:?}", save_path));

                crate::services::notification::service::send_notification(&notif_title, &notif_msg);
                return true;
            }
        }
    }
    false
}

/// Performs a fullscreen screenshot capture with HD upscaling and sharpening, saves it, and sends notification.
pub fn capture_fullscreen() -> bool {
    if let Some(temp_path) = capture_screen() {
        let save_path = get_screenshot_path();
        if let Ok(pixbuf) = gdk_pixbuf::Pixbuf::from_file(&temp_path) {
            let out_w = pixbuf.width() * 2;
            let out_h = pixbuf.height() * 2;
            if let Some(upscaled_pb) =
                pixbuf.scale_simple(out_w, out_h, gdk_pixbuf::InterpType::Hyper)
            {
                if let Ok(mut surface) =
                    cairo::ImageSurface::create(cairo::Format::ARgb32, out_w, out_h)
                {
                    if let Ok(cr) = cairo::Context::new(&surface) {
                        cr.set_antialias(cairo::Antialias::Best);
                        cr.set_source_pixbuf(&upscaled_pb, 0.0, 0.0);
                        cr.source().set_filter(cairo::Filter::Best);
                        cr.paint().unwrap();
                        apply_unsharp_mask(&mut surface, 0.35);

                        if let Ok(mut file) = std::fs::File::create(&save_path) {
                            let _ = surface.write_to_png(&mut file);
                        }
                    }
                }
            } else {
                let _ = std::fs::copy(&temp_path, &save_path);
            }
        } else {
            let _ = std::fs::copy(&temp_path, &save_path);
        }

        let notif_title = crate::i18n::trans("screenshot.full_saved_title");
        let notif_msg = crate::i18n::trans("screenshot.saved_msg")
            .replace("{}", &format!("{:?}", save_path));

        crate::services::notification::service::send_notification(&notif_title, &notif_msg);
        let _ = std::fs::remove_file(temp_path);
        return true;
    }
    false
}
