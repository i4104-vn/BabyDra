use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::PathBuf;
use gtk4::prelude::*;
use babydra_common::{SessionState, ActivePane, HeaderBarWidgets};
use crate::widgets::content_view::ContentViewHandle;

/// Wires up toolbar interaction handlers such as "New Folder" / "Empty Trash", "Cut", "Copy", "Paste", "Rename", "Delete".
pub fn wire_toolbar_buttons(
    header_widgets: &HeaderBarWidgets,
    session: Rc<RefCell<SessionState>>,
    navigate_pane_ref: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
    active_pane: Rc<Cell<ActivePane>>,
    left_content_handle: Rc<ContentViewHandle>,
    right_content_handle: Rc<RefCell<Option<Rc<ContentViewHandle>>>>,
) {
    let nav = navigate_pane_ref.clone();
    let active = active_pane.clone();

    // Helper to get active selected paths
    let get_selected_paths = {
        let left = left_content_handle.clone();
        let right = right_content_handle.clone();
        let act = active.clone();
        move || {
            if act.get() == ActivePane::Left {
                left.selected_paths.borrow().clone()
            } else {
                right.borrow().as_ref()
                    .map(|r| r.selected_paths.borrow().clone())
                    .unwrap_or_default()
            }
        }
    };

    // Helper for navigation callback
    let get_nav_cb = {
        let nav = nav.clone();
        let act = active.clone();
        move || {
            let nav = nav.clone();
            let act = act.clone();
            Rc::new(move |p| {
                if let Some(ref f) = *nav.borrow() {
                    f(act.get(), p);
                }
            }) as Rc<dyn Fn(PathBuf)>
        }
    };

    // 1. New Folder
    let btn_new_folder_c = header_widgets.btn_new_folder.clone();
    let session_new_folder = session.clone();
    let nav_new_folder = nav.clone();
    let active_new_folder = active.clone();
    let get_nav_cb_new_folder = get_nav_cb.clone();
    header_widgets.btn_new_folder.connect_clicked(move |_| {
        let path = session_new_folder.borrow().active_tab().current_path.clone();
        let is_in_trash = path.to_string_lossy().ends_with("Trash/files");
        if is_in_trash {
            babydra_common::helper::clean::remove_trash();
            if let Some(ref f) = *nav_new_folder.borrow() {
                f(active_new_folder.get(), path);
            }
        } else {
            let nav_cb = get_nav_cb_new_folder();
            let win = btn_new_folder_c.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
            babydra_utils::explore::dialogs::show_new_folder_dialog(path, nav_cb, win.as_ref());
        }
    });

    // 2. Cut
    let get_sel_cut = get_selected_paths.clone();
    let get_nav_cut = get_nav_cb.clone();
    let session_cut = session.clone();
    header_widgets.btn_cut.connect_clicked(move |_| {
        let paths = get_sel_cut();
        if !paths.is_empty() {
            babydra_utils::explore::CLIPBOARD.with(|cb| cb.replace(Some((paths, true))));
            // Refresh to show dim effect
            let path = session_cut.borrow().active_tab().current_path.clone();
            get_nav_cut()(path);
        }
    });

    // 3. Copy
    let get_sel_copy = get_selected_paths.clone();
    let get_nav_copy = get_nav_cb.clone();
    let session_copy = session.clone();
    header_widgets.btn_copy.connect_clicked(move |_| {
        let paths = get_sel_copy();
        if !paths.is_empty() {
            babydra_utils::explore::CLIPBOARD.with(|cb| cb.replace(Some((paths, false))));
            let path = session_copy.borrow().active_tab().current_path.clone();
            get_nav_copy()(path);
        }
    });

    // 4. Paste
    let session_paste = session.clone();
    let get_nav_paste = get_nav_cb.clone();
    header_widgets.btn_paste.connect_clicked(move |_| {
        let clipboard_data = babydra_utils::explore::CLIPBOARD.with(|cb| cb.borrow().clone());
        if let Some((sources, is_cut)) = clipboard_data {
            let dest_dir = session_paste.borrow().active_tab().current_path.clone();
            babydra_utils::explore::context_menu::clipboard::execute_paste(
                sources,
                dest_dir.clone(),
                is_cut,
                dest_dir.clone(),
                get_nav_paste(),
            );
        }
    });

    // 5. Rename
    let btn_rename_c = header_widgets.btn_rename.clone();
    let get_sel_rename = get_selected_paths.clone();
    let get_nav_rename = get_nav_cb.clone();
    let session_rename = session.clone();
    header_widgets.btn_rename.connect_clicked(move |_| {
        let paths = get_sel_rename();
        if paths.len() == 1 {
            let current_path = session_rename.borrow().active_tab().current_path.clone();
            let win = btn_rename_c.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
            babydra_utils::explore::dialogs::show_rename_dialog(&paths[0], current_path, get_nav_rename(), win.as_ref());
        }
    });

    // 6. Delete
    let btn_delete_c = header_widgets.btn_delete.clone();
    let get_sel_delete = get_selected_paths.clone();
    let get_nav_delete = get_nav_cb.clone();
    let session_delete = session.clone();
    header_widgets.btn_delete.connect_clicked(move |_| {
        let paths = get_sel_delete();
        if !paths.is_empty() {
            let current_path = session_delete.borrow().active_tab().current_path.clone();
            let is_in_trash = current_path.to_string_lossy().contains("Trash/files");
            let win = btn_delete_c.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
            
            if is_in_trash {
                let message = if paths.len() == 1 {
                    babydra_common::i18n::t("explore.dialog_confirm_delete_single")
                        .replace("{}", &paths[0].file_name().unwrap().to_string_lossy())
                } else {
                    babydra_common::i18n::t("explore.dialog_confirm_delete_multi")
                        .replace("{}", &paths.len().to_string())
                };
                
                let nav_cb = get_nav_delete();
                let cp_c = current_path.clone();
                let paths_c = paths.clone();
                
                babydra_utils::explore::dialogs::show_delete_confirm_dialog(
                    &babydra_common::i18n::t("explore.dialog_confirm_delete_title"),
                    &message,
                    move || {
                        let nav_f = nav_cb.clone();
                        let cp_f = cp_c.clone();
                        let paths_f = paths_c.clone();
                        glib::spawn_future_local(async move {
                            for path in paths_f {
                                if let Err(err) = babydra_common::delete_path(path).await {
                                    eprintln!("Failed to delete file: {}", err);
                                }
                            }
                            nav_f(cp_f);
                        });
                    },
                    win.as_ref(),
                );
            } else {
                let nav_cb = get_nav_delete();
                let cp_c = current_path.clone();
                glib::spawn_future_local(async move {
                    for path in paths {
                        if let Err(err) = babydra_common::send_to_trash(path).await {
                            eprintln!("Failed to trash file: {}", err);
                        }
                    }
                    nav_cb(cp_c);
                });
            }
        }
    });

    // 7. Sensitivity periodic sync (runs every 100ms on UI thread)
    let left = left_content_handle;
    let right = right_content_handle;
    let act = active;
    let hw = header_widgets.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        let selected_count = if act.get() == ActivePane::Left {
            left.selected_paths.borrow().len()
        } else {
            right.borrow().as_ref()
                .map(|r| r.selected_paths.borrow().len())
                .unwrap_or(0)
        };

        hw.btn_cut.set_sensitive(selected_count > 0);
        hw.btn_copy.set_sensitive(selected_count > 0);
        hw.btn_rename.set_sensitive(selected_count == 1);
        hw.btn_delete.set_sensitive(selected_count > 0);

        let has_clipboard = babydra_utils::explore::CLIPBOARD.with(|cb| cb.borrow().is_some());
        hw.btn_paste.set_sensitive(has_clipboard);

        glib::ControlFlow::Continue
    });
}
