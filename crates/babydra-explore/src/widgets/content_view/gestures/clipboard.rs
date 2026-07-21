use std::path::PathBuf;
use std::rc::Rc;

/// Check if a path is in Trash
pub fn is_in_trash(path: &std::path::Path) -> bool {
    path.to_string_lossy().contains("Trash/files")
}

/// Performs the cut/copy paste operation asynchronously and returns if it was completely successful.
pub async fn perform_paste(sources: Vec<PathBuf>, is_cut: bool, dest_dir: PathBuf) -> bool {
    let mut all_success = true;
    for src in sources {
        if let Some(filename) = src.file_name() {
            let dest = dest_dir.join(filename);
            if is_cut {
                if let Err(e) = babydra_common::move_path(src, dest).await {
                    eprintln!("Failed to move file: {}", e);
                    all_success = false;
                }
            } else {
                if let Err(e) = babydra_common::copy_path(src, dest).await {
                    eprintln!("Failed to copy file: {}", e);
                    all_success = false;
                }
            }
        }
    }
    all_success
}

/// Shared handler for Ctrl+X (cut) operation.
pub fn handle_cut(paths: Vec<PathBuf>, current_path: PathBuf, nav_cb: Rc<dyn Fn(PathBuf)>) {
    if !is_in_trash(&current_path) && !paths.is_empty() {
        babydra_utils::explore::CLIPBOARD.with(|cb| {
            cb.replace(Some((paths, true)));
        });
        nav_cb(current_path);
    }
}

/// Shared handler for Ctrl+C (copy) operation.
pub fn handle_copy(paths: Vec<PathBuf>, current_path: PathBuf, nav_cb: Rc<dyn Fn(PathBuf)>) {
    if !is_in_trash(&current_path) && !paths.is_empty() {
        babydra_utils::explore::CLIPBOARD.with(|cb| {
            cb.replace(Some((paths, false)));
        });
        nav_cb(current_path);
    }
}

/// Shared handler for Ctrl+V (paste) operation.
pub fn handle_paste(current_path: PathBuf, nav_cb: Rc<dyn Fn(PathBuf)>) {
    if is_in_trash(&current_path) {
        return;
    }
    let clipboard_data = babydra_utils::explore::CLIPBOARD.with(|cb| cb.borrow().clone());
    if let Some((sources, is_cut)) = clipboard_data {
        let dest_dir_c = current_path.clone();
        let nav_c = nav_cb.clone();
        glib::spawn_future_local(async move {
            let success = perform_paste(sources, is_cut, dest_dir_c.clone()).await;
            if is_cut && success {
                babydra_utils::explore::CLIPBOARD.with(|cb| cb.replace(None));
            }
            nav_c(dest_dir_c);
        });
    }
}
