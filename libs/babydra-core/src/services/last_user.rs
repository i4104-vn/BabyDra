//! Durable last-logged-in user persistence.
//!
//! The greeter runs as the `greeter` user while the session runs as the
//! logged-in user, so the record must live in the shared, world-writable
//! `/var/lib/babydra` store. `/tmp` is volatile and wiped on every reboot,
//! which used to reset the preselected greeter user after each restart.

use crate::services::utils::set_unix_mode;
use std::io::Write;
use std::path::PathBuf;

/// Durable last-user file shared between the greeter and the session.
pub fn get_last_user_path() -> PathBuf {
    PathBuf::from("/var/lib/babydra/last_user")
}

/// Legacy volatile location kept only as a read fallback for one release.
const LEGACY_TMP_PATH: &str = "/tmp/babydra-last-user";

/// Returns the last successfully logged-in username, if recorded.
pub fn get_last_user() -> Option<String> {
    for path in [get_last_user_path(), PathBuf::from(LEGACY_TMP_PATH)] {
        if let Ok(content) = std::fs::read_to_string(&path) {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

/// Records the last successfully logged-in username to the shared store.
pub fn save_last_user(user: &str) {
    let path = get_last_user_path();

    // Best effort: make sure the shared dir and file are writable by both the
    // greeter user and regular users (same convention as the wallpaper store).
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
        let _ = set_unix_mode(parent, 0o777);
    }

    if let Ok(mut file) = std::fs::File::create(&path) {
        let _ = writeln!(file, "{}", user);
    }
    let _ = set_unix_mode(&path, 0o666);
}
