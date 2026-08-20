//! Avatar management and image cropping utilities.
//! Handles avatar storage, retrieval, and circular pixbuf masking.

use crate::error::CoreResult;
use gdk_pixbuf::prelude::*;
use std::path::{Path, PathBuf};

/// Sets the avatar image.
/// - Copies to ~/.babydra/avatar.png and shared location /var/lib/babydra/avatar.png so greetd can access it.
pub fn set_avatar(path: &Path) -> CoreResult<()> {
    if !path.exists() {
        return Err(format!("Avatar file does not exist at: {:?}", path).into());
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let babydra_dir = PathBuf::from(&home).join(".babydra");
    let _ = std::fs::create_dir_all(&babydra_dir);

    // Mirror to user directory
    let user_dest = babydra_dir.join("avatar.png");
    let _ = std::fs::remove_file(&user_dest);
    let _ = std::fs::copy(path, &user_dest);

    // Copy to shared system path accessible by greetd
    let system_dest = PathBuf::from("/var/lib/babydra/avatar.png");
    let _ = std::fs::remove_file(&system_dest);
    let _ = std::fs::copy(path, &system_dest);

    Ok(())
}

/// Retrieves the active avatar as raw bytes.
pub fn get_avatar_bytes() -> Option<Vec<u8>> {
    let mut candidates = Vec::new();

    if let Ok(home) = std::env::var("HOME") {
        candidates.push(PathBuf::from(&home).join(".babydra/avatar.png"));
    }

    candidates.push(PathBuf::from("/var/lib/babydra/avatar.png"));

    if let Ok(home) = std::env::var("HOME") {
        candidates.push(PathBuf::from(&home).join(".babydra/avatar.jpg"));
        candidates.push(PathBuf::from(&home).join(".babydra/avatar.jpeg"));
        candidates.push(PathBuf::from(&home).join(".babydra/avatar.webp"));
        candidates.push(PathBuf::from(&home).join(".face"));
        candidates.push(PathBuf::from(&home).join(".face.icon"));
    }

    for c in &candidates {
        if c.exists() && c.is_file() {
            if let Ok(bytes) = std::fs::read(c) {
                return Some(bytes);
            }
        }
    }

    None
}

/// Helper to convert raw image bytes into a square, scaled Pixbuf
pub fn crop_square(bytes: &[u8], size: i32) -> Option<gdk_pixbuf::Pixbuf> {
    let stream = gio::MemoryInputStream::from_bytes(&glib::Bytes::from(bytes));
    if let Ok(pixbuf) = gdk_pixbuf::Pixbuf::from_stream(&stream, gio::Cancellable::NONE) {
        let w = pixbuf.width();
        let h = pixbuf.height();
        let min_dim = std::cmp::min(w, h);
        let x = (w - min_dim) / 2;
        let y = (h - min_dim) / 2;

        let sub = pixbuf.new_subpixbuf(x, y, min_dim, min_dim);
        return sub.scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear);
    }
    None
}

/// Applies an anti-aliased circular alpha mask to a square pixbuf so the avatar
/// renders as a circle instead of a square. CSS `border-radius` does not
/// clip widget content, so the mask must be applied to the pixels themselves.
fn apply_circular_mask(pixbuf: &gdk_pixbuf::Pixbuf) -> gdk_pixbuf::Pixbuf {
    let w = pixbuf.width();
    let h = pixbuf.height();
    let n_channels = pixbuf.n_channels();
    let rowstride = pixbuf.rowstride();

    // Owned snapshot of the source pixels (avoids aliasing with the new buffer).
    let src: Vec<u8> = pixbuf
        .pixel_bytes()
        .map(|b| b.as_ref().to_vec())
        .unwrap_or_default();

    let Some(out) = gdk_pixbuf::Pixbuf::new(gdk_pixbuf::Colorspace::Rgb, true, 8, w, h) else {
        return pixbuf.clone();
    };

    let center_x = (w - 1) as f64 / 2.0;
    let center_y = (h - 1) as f64 / 2.0;
    let radius = (w.min(h) as f64 - 1.0) / 2.0;
    // Feather band (normalized distance) for a smooth, anti-aliased edge.
    let feather = 1.5 / radius;

    for y in 0..h {
        for x in 0..w {
            let dx = (x as f64 - center_x) / radius;
            let dy = (y as f64 - center_y) / radius;
            let dist = (dx * dx + dy * dy).sqrt();
            let alpha = (((1.0 - dist) / feather).clamp(0.0, 1.0) * 255.0).round() as u8;

            let pos = (y as usize) * (rowstride as usize) + (x as usize) * (n_channels as usize);
            let (r, g, b) = match src.get(pos..pos + 3) {
                Some(rgb) => (rgb[0], rgb[1], rgb[2]),
                None => (0, 0, 0),
            };
            out.put_pixel(x as u32, y as u32, r, g, b, alpha);
        }
    }
    out
}

/// Converts raw image bytes into a square, scaled Pixbuf masked into a circle.
/// Used for circular avatar displays (greeter, lock screen, settings preview, launcher).
pub fn crop_circle(bytes: &[u8], size: i32) -> Option<gdk_pixbuf::Pixbuf> {
    let square = crop_square(bytes, size)?;
    Some(apply_circular_mask(&square))
}
