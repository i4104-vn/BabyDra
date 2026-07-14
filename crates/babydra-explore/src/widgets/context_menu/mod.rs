use std::path::PathBuf;

mod helpers;
mod show;

pub use show::{show_for_file, show_for_empty};

// Copy / Cut (using a simple static thread-local clipboard buffer for simplicity & cross-pane support)
thread_local! {
    pub static CLIPBOARD: std::cell::RefCell<Option<(PathBuf, bool)>> = std::cell::RefCell::new(None); // (path, is_cut)
}
