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
pub fn parse_target_dir() -> PathBuf {
    let mut target_dir = glib::home_dir();
    if let Some(arg) = std::env::args().nth(1) {
        let path_str = if arg.starts_with("file://") {
            babydra_common::desktop::mpris::decode_uri(&arg.replacen("file://", "", 1))
        } else {
            arg
        };
        let path = PathBuf::from(path_str);
        if path.exists() {
            target_dir = path;
        }
    }
    target_dir
}
