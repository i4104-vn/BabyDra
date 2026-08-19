use gdk_pixbuf::Pixbuf;
use rustc_hash::FxHashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use std::time::SystemTime;

#[derive(Clone)]
struct SendPixbuf(pub Pixbuf);
unsafe impl Send for SendPixbuf {}
unsafe impl Sync for SendPixbuf {}

lazy_static::lazy_static! {
    static ref THUMBNAIL_CACHE: RwLock<FxHashMap<(PathBuf, i32), (SendPixbuf, Option<SystemTime>)>> =
        RwLock::new(FxHashMap::default());
}

/// Helper to load an image, crop it to a center square, scale it, and cache the result.
pub fn load_cropped_square(path: &Path, size: i32) -> Result<Pixbuf, glib::Error> {
    let mtime = std::fs::metadata(path).and_then(|m| m.modified()).ok();
    let cache_key = (path.to_path_buf(), size);

    // 1. Check in-memory cache
    if let Ok(cache) = THUMBNAIL_CACHE.read() {
        if let Some((send_pb, cached_mtime)) = cache.get(&cache_key) {
            if *cached_mtime == mtime {
                return Ok(send_pb.0.clone());
            }
        }
    }

    // 2. Decode and scale image efficiently
    let pixbuf = Pixbuf::from_file_at_scale(path, size * 2, size * 2, true)?;
    let w = pixbuf.width();
    let h = pixbuf.height();
    let min_dim = std::cmp::min(w, h);

    // Crop center square
    let x = (w - min_dim) / 2;
    let y = (h - min_dim) / 2;
    let sub = pixbuf.new_subpixbuf(x, y, min_dim, min_dim);

    // Scale to target size
    let result = sub
        .scale_simple(size, size, gdk_pixbuf::InterpType::Bilinear)
        .ok_or_else(|| glib::Error::new(glib::FileError::Failed, "Failed to scale pixbuf"))?;

    // 3. Save to cache
    if let Ok(mut cache) = THUMBNAIL_CACHE.write() {
        // Prevent cache from growing unboundedly (limit to 1000 items)
        if cache.len() > 1000 {
            cache.clear();
        }
        cache.insert(cache_key, (SendPixbuf(result.clone()), mtime));
    }

    Ok(result)
}
