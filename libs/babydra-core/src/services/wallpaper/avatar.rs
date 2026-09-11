//! Avatar management and image cropping utilities.
//! Handles avatar storage, retrieval, and circular pixbuf masking.

use crate::error::CoreResult;
use crate::services::utils::set_unix_mode;
use std::path::{Path, PathBuf};

/// Default system logo bytes bundled in babydra-core
pub const DEFAULT_LOGO_BYTES: &[u8] = include_bytes!("../logo.png");

/// Sets the avatar image.
/// - Crops and normalizes the image to 256x256 square.
/// - Saves the image as a standard PNG in `~/.babydra/avatar.png`.
/// - Persists the avatar path in `babydra.conf` under `[lockscreen] avatar`.
/// - Copies a world-readable copy to `/var/lib/babydra/avatar.png` so greetd can display it.
pub fn set_avatar(path: &Path) -> CoreResult<()> {
    if !path.exists() {
        return Err(format!("Avatar file does not exist at: {:?}", path).into());
    }

    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let babydra_dir = PathBuf::from(&home).join(".babydra");
    let _ = std::fs::create_dir_all(&babydra_dir);

    let raw_bytes = std::fs::read(path)?;
    let png_bytes = if let Some(pix) = crop_square(&raw_bytes, 256) {
        pix.save_to_bufferv("png", &[]).unwrap_or(raw_bytes)
    } else {
        raw_bytes
    };

    let user_dest = babydra_dir.join("avatar.png");
    std::fs::write(&user_dest, &png_bytes)?;

    let mut conf = crate::config::load_babydra_config();
    conf.lockscreen.avatar = user_dest.to_string_lossy().to_string();
    crate::config::save_babydra_config(&conf);

    // Save copy for greetd which runs as another user (greeter).
    let shared_dir = PathBuf::from("/var/lib/babydra");
    if std::fs::create_dir_all(&shared_dir).is_ok() {
        let _ = set_unix_mode(&shared_dir, 0o777);
    }
    let public_dest = shared_dir.join("avatar.png");
    if std::fs::write(&public_dest, &png_bytes).is_ok() {
        let _ = set_unix_mode(&public_dest, 0o666);
    }

    Ok(())
}

/// Retrieves the path to the currently active avatar file.
pub fn get_avatar_path() -> Option<PathBuf> {
    let is_readable = |p: &Path| -> bool {
        std::fs::File::open(p).is_ok()
    };

    let conf = crate::config::load_babydra_config();
    if !conf.lockscreen.avatar.is_empty() {
        let path = PathBuf::from(&conf.lockscreen.avatar);
        if is_readable(&path) {
            return Some(path);
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let user_avatar = PathBuf::from(&home).join(".babydra/avatar.png");
        if is_readable(&user_avatar) {
            return Some(user_avatar);
        }
        let user_logo = PathBuf::from(&home).join(".babydra/logo.png");
        if is_readable(&user_logo) {
            return Some(user_logo);
        }
    }

    // Shared path readable by greetd (which runs as greeter user)
    let shared_avatar = PathBuf::from("/var/lib/babydra/avatar.png");
    if is_readable(&shared_avatar) {
        return Some(shared_avatar);
    }

    let shared_logo = PathBuf::from("/var/lib/babydra/logo.png");
    if is_readable(&shared_logo) {
        return Some(shared_logo);
    }

    let sys_logo = PathBuf::from("/usr/share/babydra/logo.png");
    if is_readable(&sys_logo) {
        return Some(sys_logo);
    }

    None
}

/// Retrieves the active avatar as raw image bytes.
/// Resolves path via get_avatar_path(), falls back to embedded logo bytes.
pub fn get_avatar_bytes() -> Option<Vec<u8>> {
    if let Some(path) = get_avatar_path() {
        if let Ok(bytes) = std::fs::read(path) {
            if !bytes.is_empty() {
                return Some(bytes);
            }
        }
    }

    Some(DEFAULT_LOGO_BYTES.to_vec())
}

/// Helper to convert raw image bytes into a square, scaled Pixbuf
fn crop_square(bytes: &[u8], size: i32) -> Option<gdk_pixbuf::Pixbuf> {
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
