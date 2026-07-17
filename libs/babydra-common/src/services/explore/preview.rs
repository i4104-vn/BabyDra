use gdk_pixbuf::Pixbuf;
use std::path::Path;

/// Helper to load an image, crop it to a center square, and scale it.
pub fn load_cropped_square_pixbuf(path: &Path, size: i32) -> Result<Pixbuf, glib::Error> {
    let pixbuf = Pixbuf::from_file_at_scale(path, size * 2, size * 2, true)?;
    let w = pixbuf.width();
    let h = pixbuf.height();
    let min_dim = std::cmp::min(w, h);
    
    // Crop center square
    let x = (w - min_dim) / 2;
    let y = (h - min_dim) / 2;
    let sub = pixbuf.new_subpixbuf(x, y, min_dim, min_dim);
    
    // Scale to target size
    sub.scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
        .ok_or_else(|| glib::Error::new(glib::FileError::Failed, "Failed to scale pixbuf"))
}
