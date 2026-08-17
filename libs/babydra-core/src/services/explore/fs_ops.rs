use crate::error::{CoreError, CoreResult};
use crate::models::explore::file_entry::{FileEntry, FileType};
use mime_guess::from_path;
use std::ffi::CStr;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

/// Gets the owner and group names of a file from its metadata on Linux.
pub fn get_owner_group(metadata: &fs::Metadata) -> (String, String) {
    let uid = metadata.uid();
    let gid = metadata.gid();

    let owner = unsafe {
        let passwd = libc::getpwuid(uid);
        if !passwd.is_null() {
            CStr::from_ptr((*passwd).pw_name)
                .to_string_lossy()
                .into_owned()
        } else {
            uid.to_string()
        }
    };

    let group = unsafe {
        let grp = libc::getgrgid(gid);
        if !grp.is_null() {
            CStr::from_ptr((*grp).gr_name)
                .to_string_lossy()
                .into_owned()
        } else {
            gid.to_string()
        }
    };

    (owner, group)
}

/// Resolves standard icon name based on file path, type, and MIME.
pub fn get_icon_name(path: &Path, is_dir: bool, mime: &str) -> String {
    if is_dir {
        return "folder".to_string();
    }

    // Check extension fallback first
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        match ext.to_lowercase().as_str() {
            "pdf" => return "document-pdf".to_string(),
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" => {
                return "package-x-generic".to_string()
            }
            "deb" | "rpm" => return "package-x-generic".to_string(),
            "exe" | "msi" | "appimage" | "sh" | "run" => {
                return "application-x-executable".to_string()
            }
            _ => {}
        }
    }

    if mime.starts_with("image/") {
        "image-x-generic".to_string()
    } else if mime.starts_with("video/") {
        "video-x-generic".to_string()
    } else if mime.starts_with("audio/") {
        "audio-x-generic".to_string()
    } else if mime.starts_with("text/") {
        "text-x-generic".to_string()
    } else {
        "text-x-generic".to_string()
    }
}

/// Loads a directory contents asynchronously.
pub async fn load_directory(
    path: PathBuf,
    show_hidden: bool,
) -> Result<Vec<FileEntry>, std::io::Error> {
    tokio::task::spawn_blocking(move || {
        let entries = fs::read_dir(&path)?;
        let mut file_entries = Vec::new();

        for entry in entries {
            let entry = entry?;
            let entry_path = entry.path();
            let name = entry.file_name();
            let name_str = name.to_string_lossy().into_owned();

            let is_hidden = name_str.starts_with('.');
            if is_hidden && !show_hidden {
                continue;
            }

            let metadata = match entry.metadata() {
                Ok(meta) => meta,
                Err(_) => continue, // skip unreadable files
            };

            let file_type = if metadata.is_dir() {
                FileType::Directory
            } else if metadata.is_file() {
                FileType::RegularFile
            } else if metadata.is_symlink() {
                FileType::Symlink
            } else {
                FileType::Unknown
            };

            let mime_type = if metadata.is_dir() {
                "inode/directory".to_string()
            } else {
                from_path(&entry_path).first_or_octet_stream().to_string()
            };

            let mut icon_name = get_icon_name(&entry_path, metadata.is_dir(), &mime_type);
            let mut display_name = name_str.clone();

            if file_type == FileType::RegularFile
                && entry_path
                    .extension()
                    .map(|e| e == "desktop")
                    .unwrap_or(false)
            {
                if let Some(app) = crate::services::apps::parse_desktop_file(&entry_path) {
                    display_name = app.name;
                    if let Some(app_icon) = app.icon {
                        icon_name = app_icon;
                    }
                }
            }

            let (owner, group) = get_owner_group(&metadata);

            file_entries.push(FileEntry {
                path: entry_path,
                name,
                display_name,
                file_type,
                mime_type,
                size: metadata.len(),
                modified: metadata.modified().ok(),
                created: metadata.created().ok(),
                permissions: metadata.mode(),
                owner,
                group,
                is_hidden,
                icon_name,
                thumbnail_path: None,
            });
        }

        Ok(file_entries)
    })
    .await
    .unwrap_or_else(|e| {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            e.to_string(),
        ))
    })
}

/// Helper to recursively copy directories.
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

/// Asynchronously copies a file or directory.
pub async fn copy_path(src: PathBuf, dest: PathBuf) -> Result<(), std::io::Error> {
    tokio::task::spawn_blocking(move || {
        let metadata = fs::metadata(&src)?;
        if metadata.is_dir() {
            copy_dir_all(&src, &dest)
        } else {
            fs::copy(&src, &dest).map(|_| ())
        }
    })
    .await
    .unwrap()
}

/// Asynchronously moves a file or directory (handles cross-device move).
pub async fn move_path(src: PathBuf, dest: PathBuf) -> Result<(), std::io::Error> {
    tokio::task::spawn_blocking(move || {
        if fs::rename(&src, &dest).is_err() {
            let metadata = fs::metadata(&src)?;
            if metadata.is_dir() {
                copy_dir_all(&src, &dest)?;
                fs::remove_dir_all(&src)?;
            } else {
                fs::copy(&src, &dest)?;
                fs::remove_file(&src)?;
            }
        }
        Ok(())
    })
    .await
    .unwrap()
}

/// Asynchronously deletes a file or directory.
pub async fn delete_path(path: PathBuf) -> Result<(), std::io::Error> {
    tokio::task::spawn_blocking(move || {
        let metadata = fs::metadata(&path)?;
        if metadata.is_dir() {
            fs::remove_dir_all(&path)
        } else {
            fs::remove_file(&path)
        }
    })
    .await
    .unwrap()
}

/// Asynchronously renames a file or directory.
pub async fn rename_path(path: PathBuf, new_name: String) -> Result<(), std::io::Error> {
    tokio::task::spawn_blocking(move || {
        let parent = path.parent().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "No parent directory")
        })?;
        let dest = parent.join(new_name);
        fs::rename(&path, &dest)
    })
    .await
    .unwrap()
}

/// Asynchronously sends a file or directory to XDG Trash.
pub async fn send_to_trash(path: PathBuf) -> CoreResult<()> {
    tokio::task::spawn_blocking(move || {
        trash::delete(path).map_err(|e| CoreError::Message(e.to_string()))
    })
    .await
    .unwrap()
}
