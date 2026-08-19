use std::path::PathBuf;
use std::rc::Rc;

/// Check if a path is in Trash
pub fn is_in_trash(path: &std::path::Path) -> bool {
    path.to_string_lossy().contains("Trash/files")
}

/// Shared handler for Ctrl+X (cut) operation.
pub fn handle_cut(paths: Vec<PathBuf>, current_path: PathBuf, _nav_cb: Rc<dyn Fn(PathBuf)>) {
    if !is_in_trash(&current_path) && !paths.is_empty() {
        babydra_ui_kit::components::explore::CLIPBOARD.with(|cb| {
            cb.replace(Some((paths.clone(), true)));
        });
        babydra_ui_kit::components::explore::context_menu::clipboard::set_clipboard_files(
            &paths, true,
        );
        babydra_ui_kit::components::explore::context_menu::clipboard::apply_cut_everywhere(&paths);
    }
}

/// Shared handler for Ctrl+C (copy) operation.
pub fn handle_copy(paths: Vec<PathBuf>, current_path: PathBuf, _nav_cb: Rc<dyn Fn(PathBuf)>) {
    if !is_in_trash(&current_path) && !paths.is_empty() {
        babydra_ui_kit::components::explore::CLIPBOARD.with(|cb| {
            cb.replace(Some((paths.clone(), false)));
        });
        babydra_ui_kit::components::explore::context_menu::clipboard::set_clipboard_files(
            &paths, false,
        );
        babydra_ui_kit::components::explore::context_menu::clipboard::apply_cut_everywhere(&[]);
    }
}

/// Shared handler for Ctrl+V (paste) operation.
pub fn handle_paste(current_path: PathBuf, nav_cb: Rc<dyn Fn(PathBuf)>) {
    if is_in_trash(&current_path) {
        return;
    }
    babydra_ui_kit::components::explore::context_menu::clipboard::paste_from_clipboard(
        current_path.clone(),
        current_path,
        nav_cb,
    );
}
