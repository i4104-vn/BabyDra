//! Cairo image surface utilities, texture downloads, and pre-scaling.

use gdk4::prelude::*;
use gdk_pixbuf::Pixbuf;
use gtk4::cairo;
use gtk4::prelude::*;
use std::path::Path;

/// Dynamically queries the current display/monitor geometry and scale factor.
pub fn get_monitor_res(drawing_area: &gtk4::DrawingArea) -> (i32, i32) {
    let w = drawing_area.width();
    let h = drawing_area.height();
    if w > 100 && h > 100 {
        return (w, h);
    }

    if let Some(display) = gtk4::gdk::Display::default() {
        let monitors = display.monitors();
        if let Some(item) = monitors.item(0) {
            if let Ok(mon) = item.downcast::<gtk4::gdk::Monitor>() {
                let geom = mon.geometry();
                let scale = mon.scale_factor().max(1);
                let mw = geom.width() * scale;
                let mh = geom.height() * scale;
                if mw > 100 && mh > 100 {
                    return (mw, mh);
                }
            }
        }
    }

    (2560, 1440)
}

/// Converts a Pixbuf to an optimized Cairo ImageSurface for ultra-fast memory blitting.
pub fn pixbuf_to_surface(pixbuf: &Pixbuf) -> Option<cairo::ImageSurface> {
    let width = pixbuf.width();
    let height = pixbuf.height();
    let format = if pixbuf.has_alpha() {
        cairo::Format::ARgb32
    } else {
        cairo::Format::Rgb24
    };

    let surface = cairo::ImageSurface::create(format, width, height).ok()?;
    let cr = cairo::Context::new(&surface).ok()?;
    cr.set_source_pixbuf(pixbuf, 0.0, 0.0);
    let _ = cr.paint();
    Some(surface)
}

/// Converts a GDK Texture to a Cairo ImageSurface via fast and memory-safe copy.
pub fn texture_to_surface(texture: &gdk4::Texture) -> Option<cairo::ImageSurface> {
    let width = texture.width();
    let height = texture.height();
    if width <= 0 || height <= 0 {
        return None;
    }

    let mut downloader = gdk4::TextureDownloader::new(texture);
    downloader.set_format(gdk4::MemoryFormat::B8g8r8a8Premultiplied);
    let (bytes, src_stride) = downloader.download_bytes();
    if bytes.is_empty() || src_stride == 0 {
        return None;
    }

    let mut surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width, height).ok()?;
    let dst_stride = surface.stride() as usize;
    let row_len = ((width as usize) * 4).min(src_stride).min(dst_stride);

    {
        let mut surf_data = surface.data().ok()?;
        let src_bytes = &bytes[..];
        for y in 0..height as usize {
            let src_start = y * src_stride;
            let dst_start = y * dst_stride;
            if src_start + row_len <= src_bytes.len() && dst_start + row_len <= surf_data.len() {
                surf_data[dst_start..dst_start + row_len]
                    .copy_from_slice(&src_bytes[src_start..src_start + row_len]);
            }
        }
    }

    surface.mark_dirty();
    Some(surface)
}

/// Captures the current visible frame from the live wallpaper Picture widget.
pub fn capture_live_picture_surface(live_picture: &gtk4::Picture) -> Option<cairo::ImageSurface> {
    if !live_picture.is_visible() {
        return None;
    }
    if let Some(paintable) = live_picture.paintable() {
        let current_image = paintable.current_image();
        if let Ok(texture) = current_image.downcast::<gdk4::Texture>() {
            return texture_to_surface(&texture);
        }
    }
    None
}

/// Loads and pre-scales an image to the exact monitor dimensions (with 3% margin for subtle zoom)
/// so that Cairo rendering during animation does zero heavy resampling and renders in < 0.2ms.
pub fn load_and_prescale(path: &Path, target_w: i32, target_h: i32) -> Option<cairo::ImageSurface> {
    if !path.exists() {
        return None;
    }
    let pixbuf = Pixbuf::from_file(path).ok()?;
    let orig_w = pixbuf.width() as f64;
    let orig_h = pixbuf.height() as f64;
    if orig_w <= 0.0 || orig_h <= 0.0 || target_w <= 0 || target_h <= 0 {
        return None;
    }

    let req_w = ((target_w as f64) * 1.03).round() as i32;
    let req_h = ((target_h as f64) * 1.03).round() as i32;

    let scale_x = req_w as f64 / orig_w;
    let scale_y = req_h as f64 / orig_h;
    let scale = scale_x.max(scale_y);

    let scaled_w = (orig_w * scale).round() as i32;
    let scaled_h = (orig_h * scale).round() as i32;

    let scaled_pixbuf =
        if (scaled_w - orig_w as i32).abs() < 50 && (scaled_h - orig_h as i32).abs() < 50 {
            pixbuf
        } else {
            pixbuf.scale_simple(scaled_w, scaled_h, gdk_pixbuf::InterpType::Bilinear)?
        };

    pixbuf_to_surface(&scaled_pixbuf)
}

/// Loads and pre-scales the first frame of a wallpaper (video or image) to monitor dimensions.
pub fn load_first_frame_surface(
    path: &Path,
    target_w: i32,
    target_h: i32,
) -> Option<cairo::ImageSurface> {
    if babydra_core::wallpaper::is_video_file(path) {
        let frame_path = babydra_core::wallpaper::get_or_create_first_frame(path);
        load_and_prescale(&frame_path, target_w, target_h)
    } else {
        load_and_prescale(path, target_w, target_h)
    }
}

/// Blits a pre-scaled ImageSurface covering the screen area with aspect-fill, alpha, and zoom.
pub fn draw_surface_aspect_fill(
    cr: &cairo::Context,
    surface: &cairo::ImageSurface,
    screen_w: f64,
    screen_h: f64,
    alpha: f64,
    zoom: f64,
) {
    if alpha <= 0.001 || surface.status().is_err() {
        return;
    }

    let surf_w = surface.width() as f64;
    let surf_h = surface.height() as f64;
    if surf_w <= 0.0 || surf_h <= 0.0 {
        return;
    }

    let scale_x = screen_w / surf_w;
    let scale_y = screen_h / surf_h;
    let base_scale = scale_x.max(scale_y);
    let scale = base_scale * zoom;

    let scaled_w = surf_w * scale;
    let scaled_h = surf_h * scale;
    let offset_x = (screen_w - scaled_w) / 2.0;
    let offset_y = (screen_h - scaled_h) / 2.0;

    let _ = cr.save();
    cr.translate(offset_x, offset_y);
    cr.scale(scale, scale);
    let _ = cr.set_source_surface(surface, 0.0, 0.0);
    if alpha >= 0.999 {
        let _ = cr.paint();
    } else {
        let _ = cr.paint_with_alpha(alpha);
    }
    let _ = cr.restore();
}
