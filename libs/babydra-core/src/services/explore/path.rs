use std::path::{Component, Path, PathBuf};

/// Sanitizes a path (resolves relative components like "." and "..")
pub fn sanitize_path(path: &Path) -> PathBuf {
    let mut components = PathBuf::new();
    for component in path.components() {
        match component {
            Component::ParentDir => {
                components.pop();
            }
            Component::CurDir => {}
            _ => {
                components.push(component);
            }
        }
    }
    components
}

/// Resolves target directory and optional focus item from a filesystem path.
/// - If `path` is an existing directory, returns `(path, None)`.
/// - If `path` is a file or pending file (e.g. download in progress) whose parent exists,
///   returns `(parent, Some(path))`.
/// - Fallback defaults to user home directory.
pub fn resolve_target_from_path(path: &Path) -> (PathBuf, Option<PathBuf>) {
    let sanitized = sanitize_path(path);
    if sanitized.is_dir() {
        (sanitized, None)
    } else if sanitized.is_file() {
        if let Some(parent) = sanitized.parent() {
            (parent.to_path_buf(), Some(sanitized))
        } else {
            (sanitized, None)
        }
    } else if let Some(parent) = sanitized.parent() {
        if parent.is_dir() {
            (parent.to_path_buf(), Some(sanitized))
        } else {
            (dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")), None)
        }
    } else {
        (dirs::home_dir().unwrap_or_else(|| PathBuf::from("/")), None)
    }
}

/// Resolves target directory and optional focus item from a URI string (e.g. `file:///home/...`).
pub fn resolve_target_from_uri(uri: &str) -> (PathBuf, Option<PathBuf>) {
    let trimmed = uri.trim();
    let raw = if let Some(stripped) = trimmed.strip_prefix("file://localhost") {
        stripped
    } else if let Some(stripped) = trimmed.strip_prefix("file://") {
        stripped
    } else if let Some(stripped) = trimmed.strip_prefix("file:") {
        stripped
    } else {
        trimmed
    };

    let decoded = crate::services::mpris::decode_uri(raw);
    let path = PathBuf::from(decoded);
    resolve_target_from_path(&path)
}
