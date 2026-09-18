//! Clipboard and delete operations shared by the header-bar buttons, keyboard
//! shortcuts, and item key controllers. This is the only place these flows are
//! implemented; callers supply selections and refresh callbacks.

use babydra_ui_kit::components::explore::context_menu::clipboard::{
    paste_from_clipboard, set_clipboard_files,
};
use babydra_ui_kit::components::explore::{apply_cut_everywhere, is_in_trash, CLIPBOARD};
use std::path::PathBuf;
use std::rc::Rc;

/// Places paths on both the GDK clipboard and the in-app clipboard state,
/// then mirrors the operation onto the visible icons: cut items get dimmed,
/// a copy clears any previous dimming. This makes the change visible
/// immediately without waiting for a directory reload.
pub fn put_on_clipboard(paths: Vec<PathBuf>, is_cut: bool) {
    if paths.is_empty() {
        return;
    }
    set_clipboard_files(&paths, is_cut);
    CLIPBOARD.with(|cb| cb.replace(Some((paths.clone(), is_cut))));
    if is_cut {
        apply_cut_everywhere(&paths);
    } else {
        apply_cut_everywhere(&[]);
    }

    let title = if is_cut {
        babydra_core::i18n::trans("explore.cut")
    } else {
        babydra_core::i18n::trans("explore.copy")
    };
    let body = format!(
        "{} {}",
        paths.len(),
        babydra_core::i18n::trans("explore.items")
    );
    babydra_core::send_notification(&title, &body);
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
    if hard_delete {
        let title = babydra_core::i18n::trans("explore.dialog_confirm_delete_title");
        let message = if paths.len() == 1 {
            let name = paths[0]
                .file_name()
                .map(|value| value.to_string_lossy().to_string())
                .unwrap_or_else(|| paths[0].to_string_lossy().to_string());
            babydra_core::i18n::trans("explore.dialog_confirm_delete_single").replace("{}", &name)
        } else {
            babydra_core::i18n::trans("explore.dialog_confirm_delete_multi")
                .replace("{}", &paths.len().to_string())
        };
        let paths_c = paths.clone();
        let current_path_c = current_path.clone();
        let nav_cb_c = nav_cb.clone();
        babydra_ui_kit::components::explore::show_delete_confirm(
            &title,
            &message,
            move || {
                delete_paths_now(
                    paths_c.clone(),
                    current_path_c.clone(),
                    nav_cb_c.clone(),
                    true,
                )
            },
            None::<&gtk4::Window>,
        );
        return;
    }

    delete_paths_now(paths, current_path, nav_cb, false);
}

fn delete_paths_now(
    paths: Vec<PathBuf>,
    current_path: PathBuf,
    nav_cb: Rc<dyn Fn(PathBuf)>,
    hard_delete: bool,
) {
    glib::spawn_future_local(async move {
        let mut completed = 0;
        for p in paths {
            let result = if hard_delete {
                babydra_core::delete_path(p).await
            } else {
                babydra_core::send_to_trash(p)
                    .await
                    .map_err(|err| std::io::Error::other(err.to_string()))
            };
            if result.is_ok() {
                completed += 1;
            }
        }
        if completed > 0 {
            let title = if hard_delete {
                babydra_core::i18n::trans("explore.menu_delete_perm")
            } else {
                babydra_core::i18n::trans("explore.menu_trash")
            };
            let body = format!(
                "{} {}",
                completed,
                babydra_core::i18n::trans("explore.items")
            );
            babydra_core::send_notification(&title, &body);
        }
        nav_cb(current_path);
    });
}
