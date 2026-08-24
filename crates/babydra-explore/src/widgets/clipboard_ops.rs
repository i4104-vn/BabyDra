//! Clipboard and delete operations shared by the header-bar buttons, keyboard
//! shortcuts, and item key controllers. This is the only place these flows are
//! implemented; callers supply selections and refresh callbacks.

use babydra_ui_kit::components::explore::context_menu::clipboard::{
    paste_from_clipboard, set_clipboard_files,
};
use babydra_ui_kit::components::explore::{apply_cut_everywhere, is_in_trash, CLIPBOARD};
use std::path::PathBuf;
use std::rc::Rc;

/// Places paths on both the GDK clipboard and the in-app clipboard state.
///
/// `dim` mirrors the operation onto file icons (dimmed cut icons / cleared
/// dimming on copy), matching the Ctrl+X/Ctrl+C presentation.
pub fn put_on_clipboard(paths: Vec<PathBuf>, is_cut: bool, dim: bool) {
    if paths.is_empty() {
        return;
    }
    set_clipboard_files(&paths, is_cut);
    CLIPBOARD.with(|cb| cb.replace(Some((paths.clone(), is_cut))));
    if dim {
        if is_cut {
            apply_cut_everywhere(&paths);
        } else {
            apply_cut_everywhere(&[]);
        }
    }
}

/// Pastes the clipboard contents into `dest_dir` unless it is inside the Trash.
pub fn paste(dest_dir: PathBuf, nav_cb: Rc<dyn Fn(PathBuf)>) {
    if is_in_trash(&dest_dir) {
        return;
    }
    paste_from_clipboard(dest_dir.clone(), dest_dir, nav_cb);
}

/// Deletes paths asynchronously (to Trash, or permanently when requested or
/// when operating inside the Trash), then refreshes via `nav_cb`.
pub fn delete_paths(
    paths: Vec<PathBuf>,
    current_path: PathBuf,
    nav_cb: Rc<dyn Fn(PathBuf)>,
    permanent: bool,
) {
    if paths.is_empty() {
        return;
    }
    let hard_delete = permanent || is_in_trash(&current_path);
    glib::spawn_future_local(async move {
        for p in paths {
            if hard_delete {
                let _ = babydra_core::delete_path(p).await;
            } else {
                let _ = babydra_core::send_to_trash(p).await;
            }
        }
        nav_cb(current_path);
    });
}
