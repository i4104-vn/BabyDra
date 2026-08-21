use crate::widgets::state::ContentViewWidgets;
use babydra_core::FileEntry;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Wires event controllers, drag select, click activation and keys for ListBox
pub fn wire_listbox_ctrls(
    widgets: &ContentViewWidgets,
    entries: Rc<RefCell<Vec<FileEntry>>>,
    nav_cb: Rc<dyn Fn(PathBuf)>,
    sc_fn: Rc<dyn Fn(Vec<PathBuf>)>,
    current_path: Rc<RefCell<PathBuf>>,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
) {
    // 1. Selection changed
    {
        let scroll_fn = sc_fn.clone();
        widgets.listbox.connect_selected_rows_changed(move |lb| {
            let sel: Vec<PathBuf> = lb
                .selected_rows()
                .iter()
                .map(|r| PathBuf::from(r.widget_name().to_string()))
                .filter(|p| p.is_absolute())
                .collect();
            scroll_fn(sel);
        });
    }

    // 2. Pane activation click on empty space
    {
        let lb_clone = widgets.listbox.clone();
        let scroll_fn = sc_fn.clone();
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(1);
        gesture.connect_pressed(move |_, _, _, y| {
            if lb_clone.row_at_y(y as i32).is_none() {
                scroll_fn(Vec::new());
            }
        });
        widgets.listbox.add_controller(gesture);
    }

    // 3. Double click row activation
    {
        let e_ref = entries.clone();
        let nav = nav_cb.clone();
        widgets.listbox.connect_row_activated(move |lb, row| {
            let path_str = row.widget_name();
            let path = PathBuf::from(path_str.to_string());
            let borrowed = e_ref.borrow();
            if let Some(entry) = borrowed.iter().find(|e| e.path == path) {
                if matches!(entry.file_type, babydra_core::FileType::Directory) {
                    nav(entry.path.clone());
                } else {
                    babydra_ui_kit::components::explore::prelude::launch_file_or_open_with(
                        &entry.path,
                        None::<&gtk4::Window>,
                    );
                }
            } else {
                for r in lb.selected_rows() {
                    let p = PathBuf::from(r.widget_name().to_string());
                    if let Some(entry) = borrowed.iter().find(|e| e.path == p) {
                        if matches!(entry.file_type, babydra_core::FileType::Directory) {
                            nav(entry.path.clone());
                        } else {
                            babydra_ui_kit::components::explore::prelude::launch_file_or_open_with(
                                &entry.path,
                                None::<&gtk4::Window>,
                            );
                        }
                    }
                }
            }
        });
    }

    // 4. Keyboard shortcuts (Enter, Ctrl+X, Ctrl+C, Ctrl+V)
    {
        let lb_clone = widgets.listbox.clone();
        let e_ref = entries.clone();
        let nav = nav_cb.clone();
        let cp_ref = current_path.clone();
        let sel_paths = selected_paths.clone();
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, keyval, _, state| {
            let has_ctrl = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
            if keyval == gtk4::gdk::Key::Return || keyval == gtk4::gdk::Key::KP_Enter {
                let borrowed = e_ref.borrow();
                for r in lb_clone.selected_rows() {
                    let p = PathBuf::from(r.widget_name().to_string());
                    if let Some(entry) = borrowed.iter().find(|e| e.path == p) {
                        if matches!(entry.file_type, babydra_core::FileType::Directory) {
                            nav(entry.path.clone());
                        } else {
                            babydra_ui_kit::components::explore::prelude::launch_file_or_open_with(
                                &entry.path,
                                None::<&gtk4::Window>,
                            );
                        }
                    }
                }
                glib::Propagation::Stop
            } else if has_ctrl && (keyval == gtk4::gdk::Key::x || keyval == gtk4::gdk::Key::X) {
                super::handle_cut(
                    sel_paths.borrow().clone(),
                    cp_ref.borrow().clone(),
                    nav.clone(),
                );
                glib::Propagation::Stop
            } else if has_ctrl && (keyval == gtk4::gdk::Key::c || keyval == gtk4::gdk::Key::C) {
                super::handle_copy(
                    sel_paths.borrow().clone(),
                    cp_ref.borrow().clone(),
                    nav.clone(),
                );
                glib::Propagation::Stop
            } else if has_ctrl && (keyval == gtk4::gdk::Key::v || keyval == gtk4::gdk::Key::V) {
                super::handle_paste(cp_ref.borrow().clone(), nav.clone());
                glib::Propagation::Stop
            } else if keyval == gtk4::gdk::Key::Delete || keyval == gtk4::gdk::Key::KP_Delete {
                if state.contains(gtk4::gdk::ModifierType::SHIFT_MASK) {
                    super::handle_permanent_delete(
                        sel_paths.borrow().clone(),
                        cp_ref.borrow().clone(),
                        nav.clone(),
                    );
                } else {
                    super::handle_delete(
                        sel_paths.borrow().clone(),
                        cp_ref.borrow().clone(),
                        nav.clone(),
                    );
                }
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        });
        widgets.listbox.add_controller(key_controller);
    }

    // 5. Drag-to-select with rubberband selection
    if let Some(list_overlay) = widgets.list_fixed.parent() {
        babydra_ui_kit::components::explore::wire_rubberband(
            &list_overlay,
            widgets.listbox.clone(),
            widgets.list_fixed.clone(),
            widgets.list_rubberband.clone(),
            selected_paths,
        );
    }
}
