//! Image processing, scaling, and rounded/circular masking utilities.
//! Centralized UI helper for avatars, album art, and rounded icons across BabyDra.

use gdk_pixbuf::prelude::*;
use gtk4::prelude::*;

/// Converts raw image bytes into a square, centered, scaled `Pixbuf`.
pub fn crop_square(bytes: &[u8], size: i32) -> Option<gdk_pixbuf::Pixbuf> {
    let stream = gio::MemoryInputStream::from_bytes(&glib::Bytes::from(bytes));
    let pixbuf = gdk_pixbuf::Pixbuf::from_stream(&stream, gio::Cancellable::NONE).ok()?;
    crop_square_pixbuf(&pixbuf, size)
}

/// Center-crops an existing `Pixbuf` to a 1:1 square and scales it to `size x size`.
pub fn crop_square_pixbuf(pixbuf: &gdk_pixbuf::Pixbuf, size: i32) -> Option<gdk_pixbuf::Pixbuf> {
    let w = pixbuf.width();
    let h = pixbuf.height();
    if w <= 0 || h <= 0 {
        return None;
    }
    let min_dim = std::cmp::min(w, h);
    let x = (w - min_dim) / 2;
    let y = (h - min_dim) / 2;

    let sub = pixbuf.new_subpixbuf(x, y, min_dim, min_dim);
    sub.scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
}

/// Applies an anti-aliased circular alpha mask to a square pixbuf.
pub fn apply_circular_mask(pixbuf: &gdk_pixbuf::Pixbuf) -> gdk_pixbuf::Pixbuf {
    let w = pixbuf.width();
    let h = pixbuf.height();
    let n_channels = pixbuf.n_channels();
    let rowstride = pixbuf.rowstride();

    let src_bytes = pixbuf.pixel_bytes();
    let src = src_bytes.as_ref().map(|b| b.as_ref()).unwrap_or(&[]);

    let Some(out) = gdk_pixbuf::Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, true, 8, w, h) else {
        return pixbuf.clone();
    };

    let out_stride = out.rowstride() as usize;
    let out_n_channels = out.n_channels() as usize;
    let out_pixels = unsafe { out.pixels() };

    let center_x = (w - 1) as f64 / 2.0;
    let center_y = (h - 1) as f64 / 2.0;
    let radius = (w.min(h) as f64 - 1.0) / 2.0;
    let feather = 1.5 / radius.max(1.0);

    let src_stride = rowstride as usize;
    let src_channels = n_channels as usize;

    for y in 0..h {
        let y_f = y as f64;
        let dy = (y_f - center_y) / radius.max(1.0);
        let dy2 = dy * dy;
        let src_row_pos = (y as usize) * src_stride;
        let out_row_pos = (y as usize) * out_stride;

        for x in 0..w {
            let x_f = x as f64;
            let dx = (x_f - center_x) / radius.max(1.0);
            let dist = (dx * dx + dy2).sqrt();
            let alpha = (((1.0 - dist) / feather).clamp(0.0, 1.0) * 255.0).round() as u8;

            let pos = src_row_pos + (x as usize) * src_channels;
            let (r, g, b) = match src.get(pos..pos + 3) {
                Some(rgb) => (rgb[0], rgb[1], rgb[2]),
                None => (0, 0, 0),
            };

            let out_pos = out_row_pos + (x as usize) * out_n_channels;
            if out_pos + 4 <= out_pixels.len() {
                out_pixels[out_pos] = r;
                out_pixels[out_pos + 1] = g;
                out_pixels[out_pos + 2] = b;
                out_pixels[out_pos + 3] = alpha;
            }
        }
    }
    out
}

/// Applies an anti-aliased rounded rectangle corner mask with `radius` to a pixbuf.
pub fn apply_rounded_mask(pixbuf: &gdk_pixbuf::Pixbuf, radius: f64) -> gdk_pixbuf::Pixbuf {
    let w = pixbuf.width();
    let h = pixbuf.height();
    let n_channels = pixbuf.n_channels();
    let rowstride = pixbuf.rowstride();

    let src_bytes = pixbuf.pixel_bytes();
    let src = src_bytes.as_ref().map(|b| b.as_ref()).unwrap_or(&[]);

    let Some(out) = gdk_pixbuf::Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, true, 8, w, h) else {
        return pixbuf.clone();
    };

    let out_stride = out.rowstride() as usize;
    let out_n_channels = out.n_channels() as usize;
    let out_pixels = unsafe { out.pixels() };

    let r = radius.min(w as f64 / 2.0).min(h as f64 / 2.0);
    let feather = 1.0;

    let src_stride = rowstride as usize;
    let src_channels = n_channels as usize;

    for y in 0..h {
        let py = y as f64 + 0.5;
        let is_top = py < r;
        let is_bottom = py > h as f64 - r;

        let src_row_pos = (y as usize) * src_stride;
        let out_row_pos = (y as usize) * out_stride;

        for x in 0..w {
            let px = x as f64 + 0.5;

            // Determine if pixel is in one of the 4 corner boxes
            let (cx, cy) = if px < r && is_top {
                (r, r)
            } else if px > w as f64 - r && is_top {
                (w as f64 - r, r)
            } else if px < r && is_bottom {
                (r, h as f64 - r)
            } else if px > w as f64 - r && is_bottom {
                (w as f64 - r, h as f64 - r)
            } else {
                (-1.0, -1.0)
            };

            let corner_alpha = if cx >= 0.0 {
                let dx = px - cx;
                let dy = py - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= r - feather {
                    1.0
                } else if dist >= r {
                    0.0
                } else {
                    ((r - dist) / feather).clamp(0.0, 1.0)
                }
            } else {
                1.0
            };

            let pos = src_row_pos + (x as usize) * src_channels;
            let (cr, cg, cb, orig_a) = match src.get(pos..pos + src_channels) {
                Some(slice) if slice.len() >= 4 => (slice[0], slice[1], slice[2], slice[3]),
                Some(slice) if slice.len() >= 3 => (slice[0], slice[1], slice[2], 255),
                _ => (0, 0, 0, 0),
            };

            let final_a = ((orig_a as f64) * corner_alpha).round().clamp(0.0, 255.0) as u8;
            let out_pos = out_row_pos + (x as usize) * out_n_channels;
            if out_pos + 4 <= out_pixels.len() {
                out_pixels[out_pos] = cr;
                out_pixels[out_pos + 1] = cg;
                out_pixels[out_pos + 2] = cb;
                out_pixels[out_pos + 3] = final_a;
            }
        }
    }
    out
}

/// Converts raw image bytes into a square, scaled, circularly masked `Pixbuf`.
pub fn crop_circle(bytes: &[u8], size: i32) -> Option<gdk_pixbuf::Pixbuf> {
    let square = crop_square(bytes, size)?;
    Some(apply_circular_mask(&square))
}

/// Converts raw image bytes into a square, scaled, rounded rectangle masked `Pixbuf`.
pub fn crop_rounded(bytes: &[u8], size: i32, radius: f64) -> Option<gdk_pixbuf::Pixbuf> {
    let square = crop_square(bytes, size)?;
    Some(apply_rounded_mask(&square, radius))
}

/// Creates a circular avatar GTK `Image` widget at a fixed pixel size.
pub fn create_circle_avatar(
    bytes: &[u8],
    size: i32,
    css_class: Option<&str>,
) -> Option<gtk4::Widget> {
    let pixbuf = crop_circle(bytes, size)?;
    let texture = gdk4::Texture::for_pixbuf(&pixbuf);
    let img = gtk4::Image::from_paintable(Some(&texture));
    img.set_pixel_size(size);
    if let Some(cls) = css_class {
        img.add_css_class(cls);
    }
    img.set_halign(gtk4::Align::Center);
    img.set_valign(gtk4::Align::Center);
    Some(img.upcast())
}

/// Creates a rounded square cover-art / thumbnail GTK `Picture` widget at a fixed size.
pub fn create_rounded_picture(
    bytes: &[u8],
    size: i32,
    radius: f64,
    css_class: Option<&str>,
) -> Option<gtk4::Widget> {
    let pixbuf = crop_rounded(bytes, size, radius)?;
    let texture = gdk4::Texture::for_pixbuf(&pixbuf);
    let picture = gtk4::Picture::for_paintable(&texture);
    picture.set_size_request(size, size);
    picture.set_content_fit(gtk4::ContentFit::Cover);
    if let Some(cls) = css_class {
        picture.add_css_class(cls);
    }
    picture.set_halign(gtk4::Align::Center);
    picture.set_valign(gtk4::Align::Center);
    Some(picture.upcast())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_circular_mask() {
        let pb = gdk_pixbuf::Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, false, 8, 20, 20).unwrap();
        pb.fill(0xffffffff);
        let masked = apply_circular_mask(&pb);
        assert_eq!(masked.width(), 20);
        assert_eq!(masked.height(), 20);
        assert_eq!(masked.n_channels(), 4);

        let pixels = unsafe { masked.pixels() };
        let stride = masked.rowstride() as usize;

        // Center pixel (10, 10) should have high alpha
        let center_idx = 10 * stride + 10 * 4;
        assert!(pixels[center_idx + 3] > 200);

        // Corner pixel (0, 0) should be fully or mostly transparent
        assert_eq!(pixels[3], 0);
    }

    #[test]
    fn test_apply_rounded_mask() {
        let pb = gdk_pixbuf::Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, false, 8, 20, 20).unwrap();
        pb.fill(0xffffffff);
        let masked = apply_rounded_mask(&pb, 5.0);
        assert_eq!(masked.width(), 20);
        assert_eq!(masked.height(), 20);
        assert_eq!(masked.n_channels(), 4);

        let pixels = unsafe { masked.pixels() };
        let stride = masked.rowstride() as usize;

        // Center pixel should have alpha 255
        let center_idx = 10 * stride + 10 * 4;
        assert_eq!(pixels[center_idx + 3], 255);

        // Corner pixel (0, 0) should have 0 alpha
        assert_eq!(pixels[3], 0);
    }
}
