//! Wallpaper management utilities.
//! Rendered directly by babydra-desktop shell on Wayland Background layer.

use crate::error::CoreResult;
use std::path::{Path, PathBuf};

/// Sets the desktop wallpaper and persists the path in babydra.conf.
pub fn set_wallpaper(path: &Path) -> CoreResult<()> {
    if !path.exists() {
        return Err(format!("Wallpaper file does not exist at: {:?}", path).into());
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let target_dir = PathBuf::from(&home).join(".babydra").join("wallpaper");
    let _ = std::fs::create_dir_all(&target_dir);

    // Save image to ~/.babydra/wallpaper if not already there
    let target_path = if path.parent() != Some(&target_dir) {
        if let Some(file_name) = path.file_name() {
            let dest = target_dir.join(file_name);
            if path != dest {
                let _ = std::fs::copy(path, &dest);
            }
            dest
        } else {
            path.to_path_buf()
        }
    } else {
        path.to_path_buf()
    };

    let path_str = target_path.to_str().ok_or("Invalid path encoding")?;

    let mut conf = crate::config::load_babydra_config();
    conf.wallpaper.current = path_str.to_string();
    crate::config::save_babydra_config(&conf);

    Ok(())
}

/// Applies the currently saved wallpaper from babydra.conf.
pub fn apply_wallpaper() {
    if let Some(path) = get_wallpaper() {
        let _ = set_wallpaper(&path);
    }
}

/// Retrieves the path to the currently active wallpaper from user configuration or wallpaper directory.
pub fn get_wallpaper() -> Option<PathBuf> {
    crate::config::invalidate_cache();
    let conf = crate::config::load_babydra_config();
    if !conf.wallpaper.current.is_empty() {
        let path = PathBuf::from(&conf.wallpaper.current);
        if path.exists() {
            return Some(path);
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let wp_dir = PathBuf::from(&home).join(".babydra/wallpaper");
        if let Ok(entries) = std::fs::read_dir(wp_dir) {
            let mut files: Vec<PathBuf> = entries
                .filter_map(Result::ok)
                .map(|e| e.path())
                .filter(|p| {
                    p.is_file()
                        && p.extension()
                            .and_then(|ext| ext.to_str())
                            .map(|ext| {
                                matches!(
                                    ext.to_lowercase().as_str(),
                                    "png" | "jpg" | "jpeg" | "webp"
                                )
                            })
                            .unwrap_or(false)
                })
                .collect();
            files.sort();
            if let Some(first) = files.first() {
                return Some(first.clone());
            }
        }
    }

    None
}

/// Returns the path to the user's wallpaper directory (~/.babydra/wallpaper).
pub fn get_wallpaper_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let dir = PathBuf::from(home).join(".babydra").join("wallpaper");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Retrieves all local wallpaper image files from ~/.babydra/wallpaper.
pub fn get_local_wallpapers() -> Vec<PathBuf> {
    let dir = get_wallpaper_dir();
    let mut files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                    let ext_lower = ext.to_lowercase();
                    if matches!(ext_lower.as_str(), "png" | "jpg" | "jpeg" | "webp") {
                        files.push(path);
                    }
                }
            }
        }
    }
    files.sort();
    files
}

/// Sets the greeter / lock background image.
/// - Saves the image to ~/.babydra/wallpaper/<filename> for reuse in user's wallpaper library.
/// - Copies to ~/.babydra/greeter_wallpaper.png and shared /var/lib/babydra/greeter_wallpaper.png for greetd.
pub fn set_greeter_wp(path: &Path) -> CoreResult<()> {
    if !path.exists() {
        return Err(format!("Greeter background file does not exist at: {:?}", path).into());
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let babydra_dir = PathBuf::from(&home).join(".babydra");
    let target_dir = babydra_dir.join("wallpaper");
    let _ = std::fs::create_dir_all(&target_dir);

    // Save to ~/.babydra/wallpaper for future reuse
    if path.parent() != Some(&target_dir) {
        if let Some(file_name) = path.file_name() {
            let dest = target_dir.join(file_name);
            if path != dest {
                let _ = std::fs::copy(path, &dest);
            }
        }
    }

    // Mirror to user directory
    let user_dest = babydra_dir.join("greeter_wallpaper.png");
    let _ = std::fs::remove_file(&user_dest);
    let _ = std::fs::copy(path, &user_dest);

    // Copy to shared system path accessible by greetd
    let system_dest = PathBuf::from("/var/lib/babydra/greeter_wallpaper.png");
    let _ = std::fs::remove_file(&system_dest);
    let _ = std::fs::copy(path, &system_dest);

    Ok(())
}

/// No longer mirrors to system path, just a no-op placeholder for compatibility
pub fn apply_greeter_wp() {
    // No-op
}

/// Retrieves the active greeter background as raw bytes.
pub fn get_greeter_wp_bytes() -> Option<Vec<u8>> {
    let mut candidates = Vec::new();

    if let Ok(home) = std::env::var("HOME") {
        candidates.push(PathBuf::from(&home).join(".babydra/greeter_wallpaper.png"));
    }

    candidates.push(PathBuf::from("/var/lib/babydra/greeter_wallpaper.png"));

    if let Ok(home) = std::env::var("HOME") {
        candidates.push(PathBuf::from(&home).join(".babydra/wallpaper.png"));
    }

    candidates.push(PathBuf::from("/usr/share/babydra/greeter_wallpaper.png"));
    candidates.push(PathBuf::from("/usr/share/babydra/wallpaper.png"));

    for c in &candidates {
        if c.exists() && c.is_file() {
            if let Ok(bytes) = std::fs::read(c) {
                return Some(bytes);
            }
        }
    }

    None
}

/// Retrieves the active greeter background as a CSS URL string.
pub fn get_greeter_wp_css() -> String {
    if let Ok(home) = std::env::var("HOME") {
        let user_wp = PathBuf::from(&home).join(".babydra/greeter_wallpaper.png");
        if user_wp.exists() {
            return format!("url('file://{}')", user_wp.display());
        }
    }

    let shared = PathBuf::from("/var/lib/babydra/greeter_wallpaper.png");
    if shared.exists() {
        return format!("url('file://{}')", shared.display());
    }

    if let Ok(home) = std::env::var("HOME") {
        let default_wp = PathBuf::from(&home).join(".babydra/wallpaper.png");
        if default_wp.exists() {
            return format!("url('file://{}')", default_wp.display());
        }
    }

    "url('file:///usr/share/babydra/wallpaper.png')".to_string()
}

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
/// renders as a circle instead of a square. GTK4 CSS `border-radius` does not
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
/// Used for circular avatar displays (greeter, lock screen, settings preview).
pub fn crop_circle(bytes: &[u8], size: i32) -> Option<gdk_pixbuf::Pixbuf> {
    let square = crop_square(bytes, size)?;
    Some(apply_circular_mask(&square))
}
