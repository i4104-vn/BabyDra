use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::{FileEntry, ContentViewWidgets};

/// Wires event controllers, drag select, click activation and keys for ListBox
pub fn wire_listbox_controllers(
    widgets: &ContentViewWidgets,
    entries: Rc<RefCell<Vec<FileEntry>>>,
    nav_cb: Rc<dyn Fn(PathBuf)>,
    sc_fn: Rc<dyn Fn(Vec<PathBuf>)>,
    current_path: Rc<RefCell<PathBuf>>,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
) {
    // 1. Selection changed
    {
        let sc = sc_fn.clone();
        widgets.listbox.connect_selected_rows_changed(move |lb| {
            let sel: Vec<PathBuf> = lb.selected_rows().iter()
                .map(|r| PathBuf::from(r.widget_name().to_string()))
                .filter(|p| p.is_absolute())
                .collect();
            sc(sel);
        });
    }

    // 2. Pane activation click on empty space
    {
        let sc = sc_fn.clone();
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(1);
        gesture.connect_pressed(move |_, _, _, _| {
            sc(Vec::new());
        });
        widgets.listbox.add_controller(gesture);
    }

    // 3. Double click row activation
    {
        let e_ref = entries.clone();
        let nav = nav_cb.clone();
        widgets.listbox.connect_row_activated(move |lb, row| {
            let mut selected_indices: Vec<usize> = lb.selected_rows().iter()
                .map(|r| r.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX))
                .filter(|&idx| idx != usize::MAX)
                .collect();
            if selected_indices.is_empty() {
                let idx = row.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX);
                if idx != usize::MAX {
                    selected_indices.push(idx);
                }
            }
            let b = e_ref.borrow();
            for idx in selected_indices {
                if idx < b.len() {
                    let entry = &b[idx];
                    if matches!(entry.file_type, babydra_common::FileType::Directory) {
                        nav(entry.path.clone());
                    } else {
                        let uri = format!("file://{}", entry.path.to_string_lossy());
                        let _ = gtk4::gio::AppInfo::launch_default_for_uri(&uri, gtk4::gio::AppLaunchContext::NONE);
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
                let selected_indices: Vec<usize> = lb_clone.selected_rows().iter()
                    .map(|r| r.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX))
                    .filter(|&idx| idx != usize::MAX)
                    .collect();
                let b = e_ref.borrow();
                for idx in selected_indices {
                    if idx < b.len() {
                        let entry = &b[idx];
                        if matches!(entry.file_type, babydra_common::FileType::Directory) {
                            nav(entry.path.clone());
                        } else {
                            let uri = format!("file://{}", entry.path.to_string_lossy());
                            let _ = gtk4::gio::AppInfo::launch_default_for_uri(&uri, gtk4::gio::AppLaunchContext::NONE);
                        }
                    }
                }
                glib::Propagation::Stop
            } else if has_ctrl && (keyval == gtk4::gdk::Key::x || keyval == gtk4::gdk::Key::X) {
                let paths = sel_paths.borrow().clone();
                if !paths.is_empty() {
                    baby_utils::explore::CLIPBOARD.with(|cb| {
                        cb.replace(Some((paths, true)));
                    });
                    nav(cp_ref.borrow().clone());
                }
                glib::Propagation::Stop
            } else if has_ctrl && (keyval == gtk4::gdk::Key::c || keyval == gtk4::gdk::Key::C) {
                let paths = sel_paths.borrow().clone();
                if !paths.is_empty() {
                    baby_utils::explore::CLIPBOARD.with(|cb| {
                        cb.replace(Some((paths, false)));
                    });
                    nav(cp_ref.borrow().clone());
                }
                glib::Propagation::Stop
            } else if has_ctrl && (keyval == gtk4::gdk::Key::v || keyval == gtk4::gdk::Key::V) {
                let clipboard_data = baby_utils::explore::CLIPBOARD.with(|cb| cb.borrow().clone());
                if let Some((sources, is_cut)) = clipboard_data {
                    let dest_dir_c = cp_ref.borrow().clone();
                    let nav_c = nav.clone();
                    glib::spawn_future_local(async move {
                        let success = super::perform_paste(sources, is_cut, dest_dir_c.clone()).await;
                        if is_cut && success {
                            baby_utils::explore::CLIPBOARD.with(|cb| cb.replace(None));
                        }
                        nav_c(dest_dir_c);
                    });
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
        baby_utils::explore::wire_rubberband_listbox(
            &list_overlay,
            widgets.listbox.clone(),
            widgets.list_fixed.clone(),
            widgets.list_rubberband.clone(),
        );
    }
}
