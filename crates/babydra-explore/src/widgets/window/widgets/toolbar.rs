use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::path::PathBuf;
use gtk4::prelude::*;
use babydra_common::{SessionState, ActivePane, HeaderBarWidgets};

/// Wires up toolbar interaction handlers such as "New Folder" / "Empty Trash".
pub fn wire_toolbar_buttons(
    header_widgets: &HeaderBarWidgets,
    session: Rc<RefCell<SessionState>>,
    navigate_pane_ref: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
    active_pane: Rc<Cell<ActivePane>>,
) {
    let session_c = session;
    let nav = navigate_pane_ref;
    let active = active_pane;

    let btn_new_folder_c = header_widgets.btn_new_folder.clone();
    header_widgets.btn_new_folder.connect_clicked(move |_| {
        let path = session_c.borrow().active_tab().current_path.clone();
        let is_in_trash = path.to_string_lossy().ends_with("Trash/files");
        if is_in_trash {
            babydra_common::helper::clean::remove_trash();
            if let Some(ref f) = *nav.borrow() {
                f(active.get(), path);
            }
        } else {
            let nav_cb = {
                let nav = nav.clone();
                let act = active.clone();
                Rc::new(move |p| {
                    if let Some(ref f) = *nav.borrow() {
                        f(act.get(), p);
                    }
                })
            };
            let win = btn_new_folder_c.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
            babydra_utils::explore::dialogs::show_new_folder_dialog(path, nav_cb, win.as_ref());
        }
    });
}
