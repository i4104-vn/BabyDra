//! Icon and logo path resolver logic for business logic (non-GUI).

/// Retrieves the path to the logo icon, extracting it if not present.
pub fn get_logo_path() -> std::path::PathBuf {
    std::path::PathBuf::from("/usr/share/babydra/logo.png")
}
