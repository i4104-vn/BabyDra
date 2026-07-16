use std::path::PathBuf;

thread_local! {
    pub static CLIPBOARD: std::cell::RefCell<Option<(Vec<PathBuf>, bool)>> = std::cell::RefCell::new(None); // (paths, is_cut)
}

mod render;
mod actions;

pub use render::{create_menu_popover, create_menu_button};
pub use actions::{show_for_file, show_for_empty};
