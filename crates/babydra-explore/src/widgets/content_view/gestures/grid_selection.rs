use babydra_core::FileEntry;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Wires event controllers, double click activation and keys for individual FlowBox
pub fn wire_grid_flowbox_controllers(
    flowbox: &gtk4::FlowBox,
    entries: Rc<RefCell<Vec<FileEntry>>>,
    nav_cb: Rc<dyn Fn(PathBuf)>,
    sc_fn: Rc<dyn Fn(Vec<PathBuf>)>,
    grid_container: &gtk4::Box,
    current_path: Rc<RefCell<PathBuf>>,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
) {
    // 1. Selection changed
    {
        let sc = sc_fn.clone();
        let grid_c = grid_container.clone();
        let fb_weak = flowbox.downgrade();
        flowbox.connect_selected_children_changed(move |fb| {
            if fb.selected_children().len() > 0 {
                // Deselect all items in other flowboxes
                let mut sibling = grid_c.first_child();
                while let Some(child) = sibling {
                    if let Some(other_fb) = child.downcast_ref::<gtk4::FlowBox>() {
                        if fb_weak.upgrade().as_ref() != Some(other_fb) {
                            if other_fb.selected_children().len() > 0 {
                                other_fb.unselect_all();
                            }
                        }
                    }
                    sibling = child.next_sibling();
                }
            }

            // Collect selected paths from ALL flowboxes inside grid_container
            let mut sel = Vec::new();
            let mut sibling = grid_c.first_child();
            while let Some(child) = sibling {
                if let Some(other_fb) = child.downcast_ref::<gtk4::FlowBox>() {
                    for child_item in other_fb.selected_children() {
                        let path_str = child_item.widget_name();
                        let path = PathBuf::from(path_str.to_string());
                        if path.is_absolute() {
                            sel.push(path);
                        }
                    }
                }
                sibling = child.next_sibling();
            }
            sc(sel);
        });
    }

    // 2. Double click child activation
    {
        let e_ref = entries.clone();
        let nav = nav_cb.clone();
        flowbox.connect_child_activated(move |fb, child| {
            let mut selected_indices: Vec<usize> = fb
                .selected_children()
                .iter()
                .map(|c| {
                    c.property::<String>("name")
                        .parse::<usize>()
                        .unwrap_or(usize::MAX)
                })
                .filter(|&idx| idx != usize::MAX)
                .collect();
            if selected_indices.is_empty() {
                let idx = child
                    .property::<String>("name")
                    .parse::<usize>()
                    .unwrap_or(usize::MAX);
                if idx != usize::MAX {
                    selected_indices.push(idx);
                }
            }
            let b = e_ref.borrow();
            for idx in selected_indices {
                if idx < b.len() {
                    let entry = &b[idx];
                    if matches!(entry.file_type, babydra_core::FileType::Directory) {
                        nav(entry.path.clone());
                    } else {
                        let uri = format!("file://{}", entry.path.to_string_lossy());
                        let _ = gtk4::gio::AppInfo::launch_default_for_uri(
                            &uri,
                            gtk4::gio::AppLaunchContext::NONE,
                        );
                    }
                }
            }
        });
    }

    // 3. Keyboard shortcuts (Enter, Ctrl+X, Ctrl+C, Ctrl+V)
    {
        let fb_clone = flowbox.clone();
        let e_ref = entries.clone();
        let nav = nav_cb.clone();
        let cp_ref = current_path.clone();
        let sel_paths = selected_paths.clone();
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, keyval, _, state| {
            let has_ctrl = state.contains(gtk4::gdk::ModifierType::CONTROL_MASK);
            if keyval == gtk4::gdk::Key::Return || keyval == gtk4::gdk::Key::KP_Enter {
                let selected_indices: Vec<usize> = fb_clone
                    .selected_children()
                    .iter()
                    .map(|c| {
                        c.property::<String>("name")
                            .parse::<usize>()
                            .unwrap_or(usize::MAX)
                    })
                    .filter(|&idx| idx != usize::MAX)
                    .collect();
                let b = e_ref.borrow();
                for idx in selected_indices {
                    if idx < b.len() {
                        let entry = &b[idx];
                        if matches!(entry.file_type, babydra_core::FileType::Directory) {
                            nav(entry.path.clone());
                        } else {
                            let uri = format!("file://{}", entry.path.to_string_lossy());
                            let _ = gtk4::gio::AppInfo::launch_default_for_uri(
                                &uri,
                                gtk4::gio::AppLaunchContext::NONE,
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
            } else {
                glib::Propagation::Proceed
            }
        });
        flowbox.add_controller(key_controller);
    }

    // 4. Click empty area to reset selection
    {
        let sc = sc_fn.clone();
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(1);
        gesture.connect_pressed(move |_, _, _, _| {
            sc(Vec::new());
        });
        flowbox.add_controller(gesture);
    }
}
