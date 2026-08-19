use std::path::{Path, PathBuf};

/// Sanitizes a path (resolves relative components like "." and "..")
pub fn sanitize_path(path: &Path) -> PathBuf {
    let mut components = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                components.pop();
            }
            std::path::Component::CurDir => {}
            _ => {
                components.push(component);
            }
        }
    }
    components
}

/// Parses command line arguments to determine target directory, decoding URI if necessary.
pub fn parse_target_dir() -> (PathBuf, Option<PathBuf>) {
    let mut target_dir = glib::home_dir();
    let mut focus_item = None;

    if let Some(arg) = std::env::args().nth(1) {
        let path_str = if arg.starts_with("file://") {
            babydra_core::mpris::decode_uri(&arg.replacen("file://", "", 1))
        } else {
            arg
        };
        let path = PathBuf::from(path_str);
        if path.is_dir() {
            target_dir = path;
        } else if path.is_file() {
            if let Some(parent) = path.parent() {
                target_dir = parent.to_path_buf();
                focus_item = Some(path);
            }
        }
    }
    (target_dir, focus_item)
}
