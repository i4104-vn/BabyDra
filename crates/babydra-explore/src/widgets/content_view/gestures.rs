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
    sc_fn: Rc<dyn Fn(Vec<usize>)>,
) {
    // 1. Selection changed
    {
        let sc = sc_fn.clone();
        widgets.listbox.connect_selected_rows_changed(move |lb| {
            let sel: Vec<usize> = lb.selected_rows().iter()
                .map(|r| r.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX))
                .filter(|&idx| idx != usize::MAX)
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

    // 4. Enter key activation
    {
        let lb_clone = widgets.listbox.clone();
        let e_ref = entries.clone();
        let nav = nav_cb.clone();
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, keyval, _, _| {
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

/// Wires event controllers, double click activation and keys for individual FlowBox
pub fn wire_grid_flowbox_controllers(
    flowbox: &gtk4::FlowBox,
    entries: Rc<RefCell<Vec<FileEntry>>>,
    nav_cb: Rc<dyn Fn(PathBuf)>,
    sc_fn: Rc<dyn Fn(Vec<usize>)>,
    grid_container: &gtk4::Box,
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
            
            // Collect selected indices from ALL flowboxes inside grid_container
            let mut sel = Vec::new();
            let mut sibling = grid_c.first_child();
            while let Some(child) = sibling {
                if let Some(other_fb) = child.downcast_ref::<gtk4::FlowBox>() {
                    for child_item in other_fb.selected_children() {
                        let idx_str = child_item.property::<String>("name");
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            sel.push(idx);
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
            let mut selected_indices: Vec<usize> = fb.selected_children().iter()
                .map(|c| c.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX))
                .filter(|&idx| idx != usize::MAX)
                .collect();
            if selected_indices.is_empty() {
                let idx = child.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX);
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

    // 3. Enter key activation
    {
        let fb_clone = flowbox.clone();
        let e_ref = entries.clone();
        let nav = nav_cb.clone();
        let key_controller = gtk4::EventControllerKey::new();
        key_controller.connect_key_pressed(move |_, keyval, _, _| {
            if keyval == gtk4::gdk::Key::Return || keyval == gtk4::gdk::Key::KP_Enter {
                let selected_indices: Vec<usize> = fb_clone.selected_children().iter()
                    .map(|c| c.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX))
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

/// Wires background click gestures, context menus, drag select, and drag drop to the view container
pub fn wire_background_controllers(
    widgets: &ContentViewWidgets,
    current_path: Rc<RefCell<PathBuf>>,
    nav_cb: Rc<dyn Fn(PathBuf)>,
) {
    // 1. Drag-to-select for Grid overlay
    if let Some(grid_overlay) = widgets.grid_fixed.parent() {
        baby_utils::explore::wire_rubberband_grid(
            &grid_overlay,
            widgets.grid_container.clone(),
            widgets.grid_fixed.clone(),
            widgets.grid_rubberband.clone(),
        );
    }

    // 2. Right click context menu on empty space
    {
        let cp = current_path.clone();
        let nav = nav_cb.clone();
        let container_widget = widgets.container.clone();
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(3);
        gesture.connect_pressed(move |gesture, _, x, y| {
            gesture.set_state(gtk4::EventSequenceState::Claimed);
            let path = cp.borrow().clone();
            crate::widgets::context_menu::show_for_empty(
                container_widget.upcast_ref(),
                x,
                y,
                path,
                nav.clone(),
            );
        });
        widgets.container.add_controller(gesture);
    }

    // 3. Drop target to background
    {
        let drop_target = baby_utils::explore::create_background_drop_target(current_path.clone());
        widgets.container.add_controller(drop_target);
    }
}
