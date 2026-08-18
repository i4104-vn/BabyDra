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

/// Invokes `grim` to capture the Wayland display screen and save it to a temporary file.
pub fn capture_screen() -> Option<String> {
    let temp_path = "/tmp/babydra-screenshot-temp.png";
    let _ = std::fs::remove_file(temp_path);

    let status = std::process::Command::new("grim").arg(temp_path).status();

    match status {
        Ok(s) if s.success() => Some(temp_path.to_string()),
        _ => None,
    }
}

/// Saves the cropped region of the surface, applying overlay annotations.
pub fn save_cropped_surface(state: &EditorState) -> Option<cairo::ImageSurface> {
    if !state.has_selection || state.crop_w <= 5.0 || state.crop_h <= 5.0 {
        return None;
    }
    let rx = state.crop_x;
    let ry = state.crop_y;
    let rw = state.crop_w;
    let rh = state.crop_h;

    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, rw as i32, rh as i32).ok()?;
    let cr = cairo::Context::new(&surface).ok()?;

    cr.translate(-rx, -ry);

    cr.set_source_pixbuf(&state.bg_pixbuf, 0.0, 0.0);
    cr.paint().unwrap();

    for drawing in &state.drawings {
        match drawing {
            Drawing::Blur { x, y, w, h } => {
                draw_pixelated_rect(&cr, &state.bg_pixbuf, *x, *y, *w, *h);
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
                cr.set_line_width(*width);
                cr.set_line_cap(cairo::LineCap::Round);
                cr.set_line_join(cairo::LineJoin::Round);
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
                cr.set_source_rgb(color.0, color.1, color.2);
                cr.set_line_width(*width);
                cr.rectangle(*x, *y, *w, *h);
                cr.stroke().unwrap();
            }
        }
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

                crate::services::notification::service::send_notification(
                    &notif_title,
                    &notif_msg,
                );
                return true;
            }
        }
    }
    false
}

/// Performs a fullscreen screenshot capture, saves it, triggers a desktop notification, and returns success status.
pub fn capture_fullscreen() -> bool {
    if let Some(temp_path) = capture_screen() {
        let save_path = get_screenshot_path();
        if std::fs::copy(&temp_path, &save_path).is_ok() {
            let notif_title = crate::i18n::trans("screenshot.full_saved_title");
            let notif_msg =
                crate::i18n::trans("screenshot.saved_msg").replace("{}", &format!("{:?}", save_path));

            crate::services::notification::service::send_notification(&notif_title, &notif_msg);
            let _ = std::fs::remove_file(temp_path);
            return true;
        }
        let _ = std::fs::remove_file(temp_path);
    }
    false
}
