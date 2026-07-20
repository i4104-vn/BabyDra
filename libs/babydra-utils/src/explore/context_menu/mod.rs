pub mod file_actions;
pub mod empty_actions;
pub mod render;

pub use empty_actions::show_for_empty;
pub use render::{create_menu_popover, create_menu_button, create_footer_icon_button};

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
) {
    let (popover, vbox) = create_menu_popover(parent, x, y);
    let is_in_trash = current_path.to_string_lossy().contains("Trash/files");
    if is_in_trash {
        file_actions::show_for_file_trash(&popover, &vbox, target_paths, current_path, nav_callback);
    } else {
        file_actions::show_for_file_normal(&popover, &vbox, target_paths, current_path, nav_callback);
    }
}
