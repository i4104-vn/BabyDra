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
    {
        let lb_clone = widgets.listbox.clone();
        let list_fixed = widgets.list_fixed.clone();
        let list_rubberband = widgets.list_rubberband.clone();
        let drag_gesture = gtk4::GestureDrag::new();
        drag_gesture.set_button(1);
        
        let start_pos = Rc::new(RefCell::new(None::<(f64, f64)>));
        let start_pos_c = start_pos.clone();
        let drag_select_active = Rc::new(RefCell::new(false));
        
        let rb_begin = list_rubberband.clone();
        let drag_select_active_begin = drag_select_active.clone();
        let lf_parent = list_fixed.parent().map(|p| p.clone());
        drag_gesture.connect_drag_begin(move |_, x, y| {
            let mut is_item = false;
            if let Some(ref parent) = lf_parent {
                let picked = parent.pick(x, y, gtk4::PickFlags::empty());
                let mut next = picked;
                while let Some(w) = next {
                    if w.downcast_ref::<gtk4::FlowBoxChild>().is_some() || w.downcast_ref::<gtk4::ListBoxRow>().is_some() {
                        is_item = true;
                        break;
                    }
                    next = w.parent();
                }
            }

            if !is_item {
                drag_select_active_begin.replace(true);
                start_pos_c.replace(Some((x, y)));
                rb_begin.set_visible(true);
                rb_begin.set_size_request(0, 0);
            } else {
                drag_select_active_begin.replace(false);
            }
        });

        let start_pos_update = start_pos.clone();
        let drag_select_active_update = drag_select_active.clone();
        let lb_update = lb_clone.clone();
        let lf_update = list_fixed.clone();
        let lr_update = list_rubberband.clone();
        drag_gesture.connect_drag_update(move |_, offset_x, offset_y| {
            if !*drag_select_active_update.borrow() {
                return;
            }
            if let Some((start_x, start_y)) = *start_pos_update.borrow() {
                let current_x = start_x + offset_x;
                let current_y = start_y + offset_y;
                let min_x = start_x.min(current_x);
                let max_x = start_x.max(current_x);
                let min_y = start_y.min(current_y);
                let max_y = start_y.max(current_y);
                let width = max_x - min_x;
                let height = max_y - min_y;

                lf_update.move_(&lr_update, min_x, min_y);
                lr_update.set_size_request(width as i32, height as i32);

                let mut child = lb_update.first_child();
                while let Some(c) = child {
                    if let Some((cx, cy)) = c.translate_coordinates(&lb_update, 0.0, 0.0) {
                        let cw = c.width() as f64;
                        let ch = c.height() as f64;
                        
                        let intersects = !(cx > max_x || cx + cw < min_x || cy > max_y || cy + ch < min_y);
                        if let Some(row) = c.downcast_ref::<gtk4::ListBoxRow>() {
                            if intersects {
                                lb_update.select_row(Some(row));
                            } else {
                                lb_update.unselect_row(row);
                            }
                        }
                    }
                    child = c.next_sibling();
                }
            }
        });

        let rb_end = list_rubberband.clone();
        let drag_select_active_end = drag_select_active.clone();
        drag_gesture.connect_drag_end(move |_, _, _| {
            if *drag_select_active_end.borrow() {
                rb_end.set_visible(false);
            }
        });

        if let Some(list_overlay) = widgets.list_fixed.parent() {
            list_overlay.add_controller(drag_gesture);
        }
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
    {
        let grid_container = widgets.grid_container.clone();
        let grid_fixed = widgets.grid_fixed.clone();
        let grid_rubberband = widgets.grid_rubberband.clone();
        
        let drag_gesture = gtk4::GestureDrag::new();
        drag_gesture.set_button(1);
        
        let start_pos = Rc::new(RefCell::new(None::<(f64, f64)>));
        let start_pos_c = start_pos.clone();
        let drag_select_active = Rc::new(RefCell::new(false));
        
        let rb_begin = grid_rubberband.clone();
        let drag_select_active_begin = drag_select_active.clone();
        let gf_parent = grid_fixed.parent().map(|p| p.clone());
        drag_gesture.connect_drag_begin(move |_, x, y| {
            let mut is_item = false;
            if let Some(ref parent) = gf_parent {
                let picked = parent.pick(x, y, gtk4::PickFlags::empty());
                let mut next = picked;
                while let Some(w) = next {
                    if w.downcast_ref::<gtk4::FlowBoxChild>().is_some() || w.downcast_ref::<gtk4::ListBoxRow>().is_some() {
                        is_item = true;
                        break;
                    }
                    next = w.parent();
                }
            }

            if !is_item {
                drag_select_active_begin.replace(true);
                start_pos_c.replace(Some((x, y)));
                rb_begin.set_visible(true);
                rb_begin.set_size_request(0, 0);
            } else {
                drag_select_active_begin.replace(false);
            }
        });

        let start_pos_update = start_pos.clone();
        let drag_select_active_update = drag_select_active.clone();
        let gc_update = grid_container.clone();
        let gf_update = grid_fixed.clone();
        let gr_update = grid_rubberband.clone();
        drag_gesture.connect_drag_update(move |_, offset_x, offset_y| {
            if !*drag_select_active_update.borrow() {
                return;
            }
            if let Some((start_x, start_y)) = *start_pos_update.borrow() {
                let current_x = start_x + offset_x;
                let current_y = start_y + offset_y;
                let min_x = start_x.min(current_x);
                let max_x = start_x.max(current_x);
                let min_y = start_y.min(current_y);
                let max_y = start_y.max(current_y);
                let width = max_x - min_x;
                let height = max_y - min_y;

                gf_update.move_(&gr_update, min_x, min_y);
                gr_update.set_size_request(width as i32, height as i32);

                let mut sibling = gc_update.first_child();
                while let Some(child) = sibling {
                    if let Some(fb) = child.downcast_ref::<gtk4::FlowBox>() {
                        let mut item_child = fb.first_child();
                        while let Some(c) = item_child {
                            if let Some((cx, cy)) = c.translate_coordinates(&gc_update, 0.0, 0.0) {
                                let cw = c.width() as f64;
                                let ch = c.height() as f64;
                                
                                let intersects = !(cx > max_x || cx + cw < min_x || cy > max_y || cy + ch < min_y);
                                if let Some(fb_child) = c.downcast_ref::<gtk4::FlowBoxChild>() {
                                    if intersects {
                                        fb.select_child(fb_child);
                                    } else {
                                        fb.unselect_child(fb_child);
                                    }
                                }
                            }
                            item_child = c.next_sibling();
                        }
                    }
                    sibling = child.next_sibling();
                }
            }
        });

        let rb_end = grid_rubberband.clone();
        let drag_select_active_end = drag_select_active.clone();
        drag_gesture.connect_drag_end(move |_, _, _| {
            if *drag_select_active_end.borrow() {
                rb_end.set_visible(false);
            }
        });

        if let Some(grid_overlay) = widgets.grid_fixed.parent() {
            grid_overlay.add_controller(drag_gesture);
        }
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
        let cp_clone = current_path.clone();
        let drop_target = gtk4::DropTarget::new(
            gtk4::gio::File::static_type(),
            gtk4::gdk::DragAction::MOVE | gtk4::gdk::DragAction::COPY,
        );
        drop_target.connect_drop(move |_, value, _, _| {
            let dest_dir = cp_clone.borrow().clone();
            if let Ok(file) = value.get::<gtk4::gio::File>() {
                if let Some(src_path) = file.path() {
                    let dest = dest_dir.join(src_path.file_name().unwrap());
                    if src_path != dest {
                        let _ = std::fs::rename(&src_path, &dest);
                    }
                }
                return true;
            }
            false
        });
        widgets.container.add_controller(drop_target);
    }
}
