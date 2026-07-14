use std::path::{Path, PathBuf};
use std::time::SystemTime;
use chrono::{DateTime, Local};

/// Formats a file size in bytes to a human-readable string (e.g. 1.2 MB).
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Formats a SystemTime into a human-readable date and time string.
pub fn format_date(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

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

/// Dynamically morphs the button layout/icons between "New Folder" and "Empty Trash".
pub fn update_new_folder_button(btn: &gtk4::Button, is_in_trash: bool) {
    use gtk4::prelude::*;
    if is_in_trash {
        if let Some(box_child) = btn.child().and_downcast::<gtk4::Box>() {
            if let Some(img) = box_child.first_child().and_downcast::<gtk4::Image>() {
                img.set_icon_name(Some("user-trash-full-symbolic"));
            }
            if let Some(lbl) = box_child.first_child().and_then(|w| w.next_sibling()).and_downcast::<gtk4::Label>() {
                lbl.set_text("Empty Trash");
            }
        }
        btn.add_css_class("empty-trash-btn");
    } else {
        if let Some(box_child) = btn.child().and_downcast::<gtk4::Box>() {
            if let Some(img) = box_child.first_child().and_downcast::<gtk4::Image>() {
                img.set_icon_name(Some("folder-new-symbolic"));
            }
            if let Some(lbl) = box_child.first_child().and_then(|w| w.next_sibling()).and_downcast::<gtk4::Label>() {
                lbl.set_text("New Folder");
            }
        }
        btn.remove_css_class("empty-trash-btn");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1536 * 1024), "1.50 MB");
    }

    #[test]
    fn test_format_date() {
        let system_time = SystemTime::UNIX_EPOCH + Duration::from_secs(1609459200); // 2021-01-01
        let formatted = format_date(system_time);
        assert!(formatted.starts_with("2021-01-01") || formatted.starts_with("2020-12-31") || formatted.starts_with("2021-01-02"));
    }
}
