pub mod clipboard;
pub mod custom_items;
pub mod dimming;
pub mod empty_actions;
pub mod file_actions;
pub mod more;

pub use dimming::{apply_cut_dimming, apply_cut_everywhere};
pub use empty_actions::show_for_empty;
pub use more::{append_more_submenu, get_apps_for_path, launch_app};

use std::path::PathBuf;
use std::rc::Rc;

thread_local! {
    pub static CLIPBOARD: std::cell::RefCell<Option<(Vec<PathBuf>, bool)>> = std::cell::RefCell::new(None); // (paths, is_cut)
}

/// Routes context menu presentation for file entries, delegating to trash-specific or standard file menus.
pub fn show_for_file(
    parent: &gtk4::Widget,
    x: f64,
    y: f64,
    target_paths: Vec<PathBuf>,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent_window: &gtk4::Window,
) {
    let is_in_trash = current_path.to_string_lossy().contains("Trash/files");
    if is_in_trash {
        file_actions::show_for_file_trash(
            parent,
            x,
            y,
            target_paths,
            current_path,
            nav_callback,
            parent_window,
        );
    } else {
        file_actions::show_for_file_normal(
            parent,
            x,
            y,
            target_paths,
            current_path,
            nav_callback,
            parent_window,
        );
    }
}
