//! Desktop icon grid layout and positioning algorithms.

use babydra_core::models::explore::FileEntry;
use std::collections::HashMap;

/// Standard margin from top/left screen edge in pixels.
pub const DEFAULT_MARGIN_X: i32 = 16;
pub const DEFAULT_MARGIN_Y: i32 = 48; // Leaves space below top panel if needed

/// Default cell dimension on desktop grid (width, height).
pub const DEFAULT_CELL_WIDTH: i32 = 96;
pub const DEFAULT_CELL_HEIGHT: i32 = 104;

/// Snaps coordinate (x, y) to the nearest grid cell.
pub fn snap_to_grid(
    x: i32,
    y: i32,
    cell_w: i32,
    cell_h: i32,
    margin_x: i32,
    margin_y: i32,
) -> (i32, i32) {
    let cell_w = cell_w.max(64);
    let cell_h = cell_h.max(64);

    let col = ((x - margin_x + cell_w / 2) / cell_w).max(0);
    let row = ((y - margin_y + cell_h / 2) / cell_h).max(0);

    let snapped_x = margin_x + col * cell_w;
    let snapped_y = margin_y + row * cell_h;

    (snapped_x, snapped_y)
}

/// Computes auto-arrange layout: fills columns top-to-bottom, then left-to-right.
pub fn calculate_auto_arrange(
    entries: &[FileEntry],
    cell_w: i32,
    cell_h: i32,
    screen_height: i32,
    margin_x: i32,
    margin_y: i32,
) -> HashMap<String, (i32, i32)> {
    let mut positions = HashMap::new();
    let cell_w = cell_w.max(64);
    let cell_h = cell_h.max(64);

    let usable_height = (screen_height - margin_y - 40).max(cell_h);
    let max_rows = (usable_height / cell_h).max(1);

    for (index, entry) in entries.iter().enumerate() {
        let index = index as i32;
        let row = index % max_rows;
        let col = index / max_rows;

        let x = margin_x + col * cell_w;
        let y = margin_y + row * cell_h;

        let file_name = entry.name.to_string_lossy().to_string();
        positions.insert(file_name, (x, y));
    }

    positions
}

/// Sorts file entries in-place based on the given sort criterion.
pub fn sort_entries(entries: &mut [FileEntry], sort_by: &str) {
    match sort_by {
        "type" => {
            entries.sort_by(|a, b| {
                let a_is_dir = a.file_type == babydra_core::FileType::Directory;
                let b_is_dir = b.file_type == babydra_core::FileType::Directory;
                match (a_is_dir, b_is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()),
                }
            });
        }
        "modified" => {
            entries.sort_by(|a, b| b.modified.cmp(&a.modified));
        }
        "size" => {
            entries.sort_by(|a, b| b.size.cmp(&a.size));
        }
        _ => {
            // Default "name": folders first, then alphabetical case-insensitive
            entries.sort_by(|a, b| {
                let a_is_dir = a.file_type == babydra_core::FileType::Directory;
                let b_is_dir = b.file_type == babydra_core::FileType::Directory;
                match (a_is_dir, b_is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()),
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use babydra_core::models::explore::FileType;
    use std::ffi::OsString;
    use std::path::PathBuf;

    fn mock_entry(name: &str, is_dir: bool, size: u64) -> FileEntry {
        FileEntry {
            path: PathBuf::from(format!("/home/user/Desktop/{}", name)),
            name: OsString::from(name),
            display_name: name.to_string(),
            file_type: if is_dir {
                FileType::Directory
            } else {
                FileType::RegularFile
            },
            mime_type: if is_dir {
                "inode/directory".to_string()
            } else {
                "text/plain".to_string()
            },
            size,
            modified: None,
            created: None,
            permissions: 0o755,
            owner: "user".to_string(),
            group: "user".to_string(),
            is_hidden: false,
            icon_name: "folder".to_string(),
            thumbnail_path: None,
        }
    }

    #[test]
    fn test_snap_to_grid() {
        let (x, y) = snap_to_grid(20, 50, 96, 104, 16, 48);
        assert_eq!((x, y), (16, 48));

        let (x2, y2) = snap_to_grid(120, 160, 96, 104, 16, 48);
        assert_eq!((x2, y2), (112, 152));
    }

    #[test]
    fn test_calculate_auto_arrange() {
        let entries = vec![
            mock_entry("A", true, 0),
            mock_entry("B", false, 100),
            mock_entry("C", false, 200),
        ];

        let positions = calculate_auto_arrange(&entries, 96, 104, 1080, 16, 48);
        assert_eq!(positions.len(), 3);
        assert_eq!(positions.get("A"), Some(&(16, 48)));
        assert_eq!(positions.get("B"), Some(&(16, 152)));
        assert_eq!(positions.get("C"), Some(&(16, 256)));
    }

    #[test]
    fn test_sort_entries() {
        let mut entries = vec![
            mock_entry("zebra.txt", false, 10),
            mock_entry("alpha_folder", true, 0),
            mock_entry("beta.txt", false, 500),
        ];

        // Sort by name (folders first)
        sort_entries(&mut entries, "name");
        assert_eq!(entries[0].display_name, "alpha_folder");
        assert_eq!(entries[1].display_name, "beta.txt");
        assert_eq!(entries[2].display_name, "zebra.txt");

        // Sort by size
        sort_entries(&mut entries, "size");
        assert_eq!(entries[0].display_name, "beta.txt");
        assert_eq!(entries[1].display_name, "zebra.txt");
        assert_eq!(entries[2].display_name, "alpha_folder");
    }
}

