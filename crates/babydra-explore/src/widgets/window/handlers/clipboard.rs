//! Window-level clipboard handlers (Cut, Copy, Paste, Undo, Delete).
//!
//! The flows themselves live in [`crate::widgets::clipboard_ops`]; this module
//! only resolves the active pane's selection and refresh callbacks.

use babydra_core::{ActivePane, SessionState};
use babydra_ui_kit::components::explore::context_menu::clipboard::execute_undo;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct ClipboardCallbacks {
    pub cut: Rc<dyn Fn()>,
    pub copy: Rc<dyn Fn()>,
    pub paste: Rc<dyn Fn()>,
    pub undo: Rc<dyn Fn()>,
    pub delete: Rc<dyn Fn()>,
    pub permanent_delete: Rc<dyn Fn()>,
    pub select_all: Rc<dyn Fn()>,
}

/// Resolves the selected paths of whichever pane is currently active.
fn active_selection(
    left: &crate::widgets::state::ContentViewHandle,
    right: &Rc<RefCell<Option<Rc<crate::widgets::state::ContentViewHandle>>>>,
    pane: ActivePane,
) -> Vec<PathBuf> {
    if pane == ActivePane::Left {
        return left.selected_paths.borrow().clone();
    }
    right
        .borrow()
        .as_ref()
        .map(|r| r.selected_paths.borrow().clone())
        .unwrap_or_default()
}

type PaneHandle = Rc<crate::widgets::state::ContentViewHandle>;
type NavRef = Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>;

pub fn create_clipboard_callbacks(
    left_content_handle: PaneHandle,
    right_content_handle: Rc<RefCell<Option<PaneHandle>>>,
    active_pane: Rc<std::cell::Cell<ActivePane>>,
    session: Rc<RefCell<SessionState>>,
    navigate_pane_ref: NavRef,
) -> ClipboardCallbacks {
    // Refreshes the active pane after an operation completes.
    let make_refresh = {
        let nav = navigate_pane_ref.clone();
        let act = active_pane.clone();
        move || {
            let (nav, act) = (nav.clone(), act.clone());
            Rc::new(move |p: PathBuf| {
                if let Some(ref f) = *nav.borrow() {
                    f(act.get(), p);
                }
            }) as Rc<dyn Fn(PathBuf)>
        }
    };

    let selection = {
        let left = left_content_handle.clone();
        let right = right_content_handle.clone();
        let act = active_pane.clone();
        move || active_selection(&left, &right, act.get())
    };

    let select_all_cb = {
        let left = left_content_handle.clone();
        let right = right_content_handle.clone();
        let act = active_pane.clone();
        move || {
            if act.get() == ActivePane::Left {
                crate::widgets::content_view::select_all_items(&left);
            } else if let Some(ref r) = *right.borrow() {
                crate::widgets::content_view::select_all_items(r);
            }
        }
    };

    let cut_cb = {
        let session = session.clone();
        let selection = selection.clone();
        let nav = navigate_pane_ref.clone();
        let act = active_pane.clone();
        move || {
            let paths = selection();
            if paths.is_empty() {
                return;
            }
            crate::widgets::clipboard_ops::put_on_clipboard(paths, true, false);
            let p = session.borrow().active_tab().current_path.clone();
            if let Some(ref f) = *nav.borrow() {
                f(act.get(), p);
            }
        }
    };

    let copy_cb = {
        let session = session.clone();
        let selection = selection.clone();
        let nav = navigate_pane_ref.clone();
        let act = active_pane.clone();
        move || {
            let paths = selection();
            if paths.is_empty() {
                return;
            }
            crate::widgets::clipboard_ops::put_on_clipboard(paths, false, false);
            let p = session.borrow().active_tab().current_path.clone();
            if let Some(ref f) = *nav.borrow() {
                f(act.get(), p);
            }
        }
    };

    let paste_cb = {
        let session = session.clone();
        let refresh = make_refresh.clone();
        move || {
            let current = session.borrow().active_tab().current_path.clone();
            crate::widgets::clipboard_ops::paste(current, refresh());
        }
    };

    let undo_cb = {
        let session = session.clone();
        let refresh = make_refresh.clone();
        move || {
            let current = session.borrow().active_tab().current_path.clone();
            execute_undo(refresh(), current);
        }
    };

    let delete_cb = {
        let session = session.clone();
        let selection = selection.clone();
        let refresh = make_refresh.clone();
        move || {
            let current = session.borrow().active_tab().current_path.clone();
            crate::widgets::clipboard_ops::delete_paths(selection(), current, refresh(), false);
        }
    };

    let permanent_delete_cb = {
        let session = session.clone();
        let selection = selection.clone();
        let refresh = make_refresh.clone();
        move || {
            let current = session.borrow().active_tab().current_path.clone();
            crate::widgets::clipboard_ops::delete_paths(selection(), current, refresh(), true);
        }
    };

    ClipboardCallbacks {
        cut: Rc::new(cut_cb),
        copy: Rc::new(copy_cb),
        paste: Rc::new(paste_cb),
        undo: Rc::new(undo_cb),
        delete: Rc::new(delete_cb),
        permanent_delete: Rc::new(permanent_delete_cb),
        select_all: Rc::new(select_all_cb),
    }
}
