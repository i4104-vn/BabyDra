use std::path::PathBuf;
use std::rc::Rc;
use gtk4::prelude::*;
use babydra_common::FileEntry;

mod helpers;
mod show;

pub struct ContextMenu;

// Copy / Cut (using a simple static thread-local clipboard buffer for simplicity & cross-pane support)
thread_local! {
    pub static CLIPBOARD: std::cell::RefCell<Option<(PathBuf, bool)>> = std::cell::RefCell::new(None); // (path, is_cut)
}
