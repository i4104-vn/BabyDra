//! Desktop icon grid state and layout models.
//! `DesktopState` holds runtime desktop state; `layout` provides positioning
//! and sorting algorithms. Both were moved here from `babydra-desktop`.

use crate::config::{load_desktop_config, save_desktop_config, DesktopConfig};
use crate::models::explore::FileEntry;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

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
pub fn calc_auto_arrange(
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
                let a_is_dir = a.file_type == crate::models::explore::FileType::Directory;
                let b_is_dir = b.file_type == crate::models::explore::FileType::Directory;
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
                let a_is_dir = a.file_type == crate::models::explore::FileType::Directory;
                let b_is_dir = b.file_type == crate::models::explore::FileType::Directory;
                match (a_is_dir, b_is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase()),
                }
            });
        }
    }
}

/// Primary desktop runtime state.
#[derive(Clone, Debug)]
pub struct DesktopState {
    /// File entries found in ~/Desktop
    pub entries: Vec<FileEntry>,
    /// Set of selected canonical path strings
    pub selected_paths: HashSet<PathBuf>,
    /// Desktop configuration loaded from babydra.conf
    pub config: DesktopConfig,
    /// Desktop screen dimensions (width, height)
    pub screen_size: (i32, i32),
}

impl Default for DesktopState {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopState {
    /// Initializes a new DesktopState instance.
    pub fn new() -> Self {
        let config = load_desktop_config();
        Self {
            entries: Vec::new(),
            selected_paths: HashSet::new(),
            config,
            screen_size: (1920, 1080),
        }
    }

    /// Resolves the absolute path to the desktop folder: `~/Desktop`.
    pub fn desktop_dir() -> PathBuf {
        dirs::desktop_dir().unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("Desktop")
        })
    }

    /// Reloads files from `~/Desktop` directory.
    pub async fn reload_entries(&mut self) {
        let dir = Self::desktop_dir();
        let _ = std::fs::create_dir_all(&dir);

        let show_hidden = false;
        if let Ok(entries) = crate::services::explore::load_directory(dir, show_hidden).await {
            let mut list = entries;
            sort_entries(&mut list, &self.config.sort_by);
            self.entries = list;
        }

        // Clean up selected paths that no longer exist
        let existing_paths: HashSet<PathBuf> = self.entries.iter().map(|e| e.path.clone()).collect();
        self.selected_paths.retain(|p| existing_paths.contains(p));
    }

    /// Checks if a file path is currently selected.
    pub fn is_selected(&self, path: &Path) -> bool {
        self.selected_paths.contains(path)
    }

    /// Selects or toggles a file item.
    pub fn select(&mut self, path: PathBuf, multi: bool, toggle: bool) {
        if multi {
            if toggle {
                if self.selected_paths.contains(&path) {
                    self.selected_paths.remove(&path);
                } else {
                    self.selected_paths.insert(path);
                }
            } else {
                self.selected_paths.insert(path);
            }
        } else {
            self.selected_paths.clear();
            self.selected_paths.insert(path);
        }
    }

    /// Selects all files on desktop.
    pub fn select_all(&mut self) {
        self.selected_paths = self.entries.iter().map(|e| e.path.clone()).collect();
    }

    /// Clears selection.
    pub fn clear_selection(&mut self) {
        self.selected_paths.clear();
    }

    /// Gets position for an entry (either manual or auto-arranged).
    pub fn get_entry_position(&self, file_name: &str, index: usize) -> (i32, i32) {
        let cell_w = self.config.grid_spacing as i32;
        let cell_h = cell_w + 14;

        if !self.config.auto_arrange {
            if let Some(&(x, y)) = self.config.icon_positions.get(file_name) {
                return (x, y);
            }
        }

        // Fallback auto-arrange column layout
        let margin_x = DEFAULT_MARGIN_X;
        let margin_y = DEFAULT_MARGIN_Y;
        let usable_height = (self.screen_size.1 - margin_y - 40).max(cell_h);
        let max_rows = (usable_height / cell_h).max(1);

        let row = (index as i32) % max_rows;
        let col = (index as i32) / max_rows;

        let x = margin_x + col * cell_w;
        let y = margin_y + row * cell_h;

        (x, y)
    }

    /// Saves a manual icon position and persists configuration.
    pub fn set_icon_position(&mut self, file_name: String, x: i32, y: i32) {
        let cell_w = self.config.grid_spacing as i32;
        let cell_h = cell_w + 14;

        let (snapped_x, snapped_y) = snap_to_grid(
            x,
            y,
            cell_w,
            cell_h,
            DEFAULT_MARGIN_X,
            DEFAULT_MARGIN_Y,
        );

        self.config.icon_positions.insert(file_name, (snapped_x, snapped_y));
        self.config.auto_arrange = false;
        save_desktop_config(&self.config);
    }

    /// Updates sort order and re-sorts entries.
    pub fn set_sort_by(&mut self, sort_by: String) {
        self.config.sort_by = sort_by;
        sort_entries(&mut self.entries, &self.config.sort_by);
        save_desktop_config(&self.config);
    }

    /// Toggles auto-arrange mode.
    pub fn set_auto_arrange(&mut self, auto: bool) {
        self.config.auto_arrange = auto;
        if auto {
            self.config.icon_positions.clear();
        }
        save_desktop_config(&self.config);
    }

    /// Updates icon size preference.
    pub fn set_icon_size(&mut self, size: u32) {
        self.config.icon_size = size;
        self.config.grid_spacing = (size * 2).max(80);
        save_desktop_config(&self.config);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::explore::FileType;
    use std::ffi::OsString;

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
    fn test_calc_auto_arrange() {
        let entries = vec![
            mock_entry("A", true, 0),
            mock_entry("B", false, 100),
            mock_entry("C", false, 200),
        ];

        let positions = calc_auto_arrange(&entries, 96, 104, 1080, 16, 48);
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
