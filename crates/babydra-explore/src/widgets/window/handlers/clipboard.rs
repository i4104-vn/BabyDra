//! Window-level clipboard handlers (Cut, Copy, Paste, Undo).

use babydra_core::{ActivePane, SessionState};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct ClipboardCallbacks {
    pub cut: Rc<dyn Fn()>,
    pub copy: Rc<dyn Fn()>,
    pub paste: Rc<dyn Fn()>,
    pub undo: Rc<dyn Fn()>,
}

pub fn create_clipboard_callbacks(
    left_content_handle: Rc<crate::widgets::state::ContentViewHandle>,
    right_content_handle: Rc<RefCell<Option<Rc<crate::widgets::state::ContentViewHandle>>>>,
    active_pane: Rc<std::cell::Cell<ActivePane>>,
    session: Rc<RefCell<SessionState>>,
    navigate_pane_ref: Rc<RefCell<Option<Rc<dyn Fn(ActivePane, PathBuf)>>>>,
) -> ClipboardCallbacks {
    let cut_cb = {
        let left = left_content_handle.clone();
        let right = right_content_handle.clone();
        let act = active_pane.clone();
        let session = session.clone();
        let nav = navigate_pane_ref.clone();
        move || {
            let paths = if act.get() == ActivePane::Left {
                left.selected_paths.borrow().clone()
            } else {
                right
                    .borrow()
                    .as_ref()
                    .map(|r| r.selected_paths.borrow().clone())
                    .unwrap_or_default()
            };
            if !paths.is_empty() {
                let current_path = session.borrow().active_tab().current_path.clone();
                babydra_ui_kit::components::explore::context_menu::clipboard::set_clipboard_files(
                    &paths, true,
                );
                babydra_ui_kit::components::explore::CLIPBOARD
                    .with(|cb| cb.replace(Some((paths, true))));
                if let Some(ref f) = *nav.borrow() {
                    f(act.get(), current_path);
                }
            }
        }
    };

    let copy_cb = {
        let left = left_content_handle.clone();
        let right = right_content_handle.clone();
        let act = active_pane.clone();
        let session = session.clone();
        let nav = navigate_pane_ref.clone();
        move || {
            let paths = if act.get() == ActivePane::Left {
                left.selected_paths.borrow().clone()
            } else {
                right
                    .borrow()
                    .as_ref()
                    .map(|r| r.selected_paths.borrow().clone())
                    .unwrap_or_default()
            };
            if !paths.is_empty() {
                let current_path = session.borrow().active_tab().current_path.clone();
                babydra_ui_kit::components::explore::context_menu::clipboard::set_clipboard_files(
                    &paths, false,
                );
                babydra_ui_kit::components::explore::CLIPBOARD
                    .with(|cb| cb.replace(Some((paths, false))));
                if let Some(ref f) = *nav.borrow() {
                    f(act.get(), current_path);
                }
            }
        }
    };

    let paste_cb = {
        let session = session.clone();
        let act = active_pane.clone();
        let nav = navigate_pane_ref.clone();
        move || {
            let current_path = session.borrow().active_tab().current_path.clone();
            let nav_cb = {
                let nav = nav.clone();
                let act = act.clone();
                Rc::new(move |p| {
                    if let Some(ref f) = *nav.borrow() {
                        f(act.get(), p);
                    }
                }) as Rc<dyn Fn(PathBuf)>
            };
            babydra_ui_kit::components::explore::context_menu::clipboard::paste_from_clipboard(
                current_path.clone(),
                current_path,
                nav_cb,
            );
        }
    };

    let undo_cb = {
        let session = session.clone();
        let act = active_pane.clone();
        let nav = navigate_pane_ref.clone();
        move || {
            let current_path = session.borrow().active_tab().current_path.clone();
            let nav_cb = {
                let nav = nav.clone();
                let act = act.clone();
                Rc::new(move |p| {
                    if let Some(ref f) = *nav.borrow() {
                        f(act.get(), p);
                    }
                }) as Rc<dyn Fn(PathBuf)>
            };
            babydra_ui_kit::components::explore::context_menu::clipboard::execute_undo(
                nav_cb,
                current_path,
            );
        }
    };

    ClipboardCallbacks {
        cut: Rc::new(cut_cb),
        copy: Rc::new(copy_cb),
        paste: Rc::new(paste_cb),
        undo: Rc::new(undo_cb),
    }
}
