//! Desktop state management.

pub mod layout;

use babydra_core::config::{load_desktop_config, save_desktop_config, DesktopConfig};
use babydra_core::models::explore::FileEntry;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

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
        if let Ok(entries) = babydra_core::load_directory(dir, show_hidden).await {
            let mut list = entries;
            layout::sort_entries(&mut list, &self.config.sort_by);
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
        let margin_x = layout::DEFAULT_MARGIN_X;
        let margin_y = layout::DEFAULT_MARGIN_Y;
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

        let (snapped_x, snapped_y) = layout::snap_to_grid(
            x,
            y,
            cell_w,
            cell_h,
            layout::DEFAULT_MARGIN_X,
            layout::DEFAULT_MARGIN_Y,
        );

        self.config.icon_positions.insert(file_name, (snapped_x, snapped_y));
        self.config.auto_arrange = false;
        save_desktop_config(&self.config);
    }

    /// Updates sort order and re-sorts entries.
    pub fn set_sort_by(&mut self, sort_by: String) {
        self.config.sort_by = sort_by;
        layout::sort_entries(&mut self.entries, &self.config.sort_by);
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
