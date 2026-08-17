use std::path::Path;

/// Check if a path is a supported archive file.
pub fn is_archive_file(path: &Path) -> bool {
    let name = path.to_string_lossy().to_lowercase();
    name.ends_with(".zip")
        || name.ends_with(".tar")
        || name.ends_with(".tar.gz")
        || name.ends_with(".tgz")
        || name.ends_with(".tar.xz")
        || name.ends_with(".txz")
        || name.ends_with(".tar.bz2")
        || name.ends_with(".tbz2")
        || name.ends_with(".tar.zst")
        || name.ends_with(".tar.lz4")
        || name.ends_with(".rar")
        || name.ends_with(".7z")
}
