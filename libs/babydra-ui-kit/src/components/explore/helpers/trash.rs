use std::path::Path;

/// Check if a path is in Trash.
pub fn is_in_trash(path: &Path) -> bool {
    path.to_string_lossy().contains("Trash/files")
}
