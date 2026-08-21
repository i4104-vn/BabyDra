//! Avatar management and image cropping utilities.
//! Handles avatar storage, retrieval, and circular pixbuf masking.

use crate::error::CoreResult;
use base64::prelude::*;
use std::path::{Path, PathBuf};

/// Default system logo bytes bundled in babydra-core
pub const DEFAULT_LOGO_BYTES: &[u8] = include_bytes!("../logo.png");

/// Sets the avatar image.
/// - Crops and normalizes the image to 256x256 square.
/// - Encodes the image bytes to Base64 and persists in `~/.babydra/avatar.bb`.
/// - Persists the avatar path in `babydra.conf` under `[lockscreen] avatar`.
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

    let encoded = BASE64_STANDARD.encode(&png_bytes);
    let user_dest = babydra_dir.join("avatar.bb");
    std::fs::write(&user_dest, &encoded)?;

    // Clean up legacy avatar.png if present
    let _ = std::fs::remove_file(babydra_dir.join("avatar.png"));

    let mut conf = crate::config::load_babydra_config();
    conf.lockscreen.avatar = user_dest.to_str().unwrap_or_default().to_string();
    crate::config::save_babydra_config(&conf);

    // Save fallback for greetd which runs as another user.
    use std::os::unix::fs::PermissionsExt;
    let shared_dir = PathBuf::from("/var/lib/babydra");
    if std::fs::create_dir_all(&shared_dir).is_ok() {
        let _ = std::fs::set_permissions(&shared_dir, std::fs::Permissions::from_mode(0o777));
    }
    let public_dest = shared_dir.join("avatar_fallback.bb");
    if std::fs::write(&public_dest, &encoded).is_ok() {
        let _ = std::fs::set_permissions(&public_dest, std::fs::Permissions::from_mode(0o666));
    }

    Ok(())
}

/// Retrieves the path to the currently active avatar file (.bb or logo).
pub fn get_avatar_path() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        let user_bb = PathBuf::from(&home).join(".babydra/avatar.bb");
        if user_bb.exists() && user_bb.is_file() {
            return Some(user_bb);
        }
        let user_logo = PathBuf::from(&home).join(".babydra/logo.png");
        if user_logo.exists() && user_logo.is_file() {
            return Some(user_logo);
        }
    }

    let fallback = PathBuf::from("/var/lib/babydra/avatar_fallback.bb");
    if fallback.exists() && fallback.is_file() {
        return Some(fallback);
    }

    None
}

/// Retrieves the active avatar as raw image bytes decoded from Base64 `.bb`.
/// If no `.bb` avatar file exists, falls back directly to the user's logo.
pub fn get_avatar_bytes() -> Option<Vec<u8>> {
    crate::config::invalidate_cache();
    let conf = crate::config::load_babydra_config();

    let candidate_paths = [
        if !conf.lockscreen.avatar.is_empty() {
            Some(PathBuf::from(&conf.lockscreen.avatar))
        } else {
            None
        },
        dirs::home_dir().map(|h| h.join(".babydra").join("avatar.bb")),
        Some(PathBuf::from("/var/lib/babydra/avatar_fallback.bb")),
    ];

    for candidate in candidate_paths.into_iter().flatten() {
        if candidate.exists() && candidate.is_file() {
            if let Ok(content) = std::fs::read_to_string(&candidate) {
                let trimmed = content.trim();
                if let Ok(bytes) = BASE64_STANDARD.decode(trimmed.as_bytes()) {
                    if !bytes.is_empty() {
                        return Some(bytes);
                    }
                }
            }
        }
    }

    // Fallback directly to logo if no avatar.bb is configured or found
    if let Some(home) = dirs::home_dir() {
        let logo_path = home.join(".babydra/logo.png");
        if logo_path.exists() && logo_path.is_file() {
            if let Ok(bytes) = std::fs::read(&logo_path) {
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
