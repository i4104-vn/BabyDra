//! Icon and logo path resolver logic for business logic (non-GUI).

/// Retrieves the path to the logo icon, extracting it if not present.
pub fn get_logo_path() -> std::path::PathBuf {
    let logo_dir = crate::config::get_babydra_config_dir();
    let logo_path = logo_dir.join("logo.png");
    if !logo_path.exists() {
        let _ = std::fs::create_dir_all(&logo_dir);
        const PNG_BYTES: &[u8] = include_bytes!("logo.png");
        let _ = std::fs::write(&logo_path, PNG_BYTES);
    }
    logo_path
}
