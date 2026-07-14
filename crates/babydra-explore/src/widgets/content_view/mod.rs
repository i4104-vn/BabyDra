use gtk4::prelude::*;
use gtk4::ScrolledWindow;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::FileEntry;
pub use babydra_common::{ContentViewWidgets, ContentViewHandle, sort_entries};

mod render;
mod update;
pub mod helpers;

/// Creates the content view area widgets and returns the scroll container and ContentViewHandle state handle.
pub fn create_content_view(
    nav_callback: impl Fn(PathBuf) + 'static,
    selection_callback: impl Fn(Vec<FileEntry>) + 'static,
) -> (ScrolledWindow, ContentViewHandle) {
    let widgets = render::build_content_view_ui();

    let entries: Rc<RefCell<Vec<FileEntry>>> = Rc::new(RefCell::new(Vec::new()));
    let all_entries: Rc<RefCell<Vec<FileEntry>>> = Rc::new(RefCell::new(Vec::new()));
    let current_path = Rc::new(RefCell::new(PathBuf::new()));
    let current_mode = Rc::new(RefCell::new("icons".to_string()));
    let sort_mode = Rc::new(RefCell::new("auto".to_string()));
    let nav_cb = Rc::new(nav_callback) as Rc<dyn Fn(PathBuf)>;

    let entries_clone = entries.clone();
    let sel_cb = Rc::new(selection_callback) as Rc<dyn Fn(Vec<FileEntry>)>;
    
    let sc_fn = Rc::new(move |selected_indices: Vec<usize>| {
        let mut list = Vec::new();
        let b = entries_clone.borrow();
        for idx in selected_indices {
            if idx < b.len() {
                list.push(b[idx].clone());
            }
        }
        sel_cb(list);
    }) as Rc<dyn Fn(Vec<usize>)>;

    let handle = ContentViewHandle {
        widgets: widgets.clone(),
        entries: entries.clone(),
        all_entries: all_entries.clone(),
        current_path: current_path.clone(),
        current_mode: current_mode.clone(),
        sort_mode: sort_mode.clone(),
        nav_callback: nav_cb.clone(),
        selection_callback: sc_fn.clone(),
    };

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

    // Wire pane activation gestures on empty space clicks
    {
        let sc = sc_fn.clone();
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(1);
        gesture.connect_pressed(move |_, _, _, _| {
            sc(Vec::new());
        });
        widgets.listbox.add_controller(gesture);
    }

    // Wire double click activations
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
                        let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
                    }
                }
            }
        });
    }

    // Wire Enter key activation for ListBox
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
                            let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
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

    // Wire drag-to-select for ListBox
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

    // Wire drag-to-select for FlowBox (grid mode)
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
    // Wire right click empty area context menu gesture to the main scrolled window
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

    // Wire drop-to-move for empty space in the view area
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

    (widgets.container.clone(), handle)
}

/// Dynamic FlowBox builder helper for Grid grouping/categories
pub fn create_grid_flowbox(
    entries: Rc<RefCell<Vec<FileEntry>>>,
    nav_cb: Rc<dyn Fn(PathBuf)>,
    sc_fn: Rc<dyn Fn(Vec<usize>)>,
    grid_container: &gtk4::Box,
) -> gtk4::FlowBox {
    let flowbox = gtk4::FlowBox::builder()
        .valign(gtk4::Align::Start)
        .max_children_per_line(20)
        .min_children_per_line(1)
        .selection_mode(gtk4::SelectionMode::Multiple)
        .activate_on_single_click(false)
        .row_spacing(10)
        .column_spacing(10)
        .build();

    // Wire selection changed
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

    // Wire double click activation
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
                        let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
                    }
                }
            }
        });
    }

    // Wire Enter key activation for FlowBox
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
                            let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
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



    // Wire click to update active pane
    {
        let sc = sc_fn.clone();
        let gesture = gtk4::GestureClick::new();
        gesture.set_button(1);
        gesture.connect_pressed(move |_, _, _, _| {
            sc(Vec::new());
        });
        flowbox.add_controller(gesture);
    }

    flowbox
}


/// Changes the layout layout style of content view stack.
pub fn set_content_view_mode(handle: &ContentViewHandle, mode: &str) {
    handle.current_mode.replace(mode.to_string());
    handle.widgets.stack.set_visible_child_name(mode);
    
    let mut e = handle.entries.borrow().clone();
    let sort = handle.sort_mode.borrow().clone();
    
    // Sort with the new mode
    sort_entries(&mut e, &sort);
    handle.entries.replace(e.clone());
    
    update::update_content_view_ui(handle);
}

/// Changes the sorting mode of the content view and updates the layout.
pub fn set_content_view_sort(handle: &ContentViewHandle, sort_mode: &str) {
    handle.sort_mode.replace(sort_mode.to_string());
    
    // Sort current entries
    let mut e = handle.entries.borrow().clone();
    sort_entries(&mut e, sort_mode);
    handle.entries.replace(e.clone());
    
    // Sort all entries
    let mut all = handle.all_entries.borrow().clone();
    sort_entries(&mut all, sort_mode);
    handle.all_entries.replace(all);

    update::update_content_view_ui(handle);
}

/// Updates files in view area.
pub fn update_content_view(handle: &ContentViewHandle, entries: &[FileEntry], current_path: PathBuf) {
    let sort = handle.sort_mode.borrow().clone();
    let mode = handle.current_mode.borrow().clone();
    
    let mut sorted = entries.to_vec();
    sort_entries(&mut sorted, &sort);
    handle.all_entries.replace(sorted.clone());
    handle.entries.replace(sorted.clone());
    handle.current_path.replace(current_path);

    handle.widgets.stack.set_visible_child_name(&mode);

    update::update_content_view_ui(handle);
}

/// Filters content files list.
pub fn filter_content_view(handle: &ContentViewHandle, query: &str) {
    let sort = handle.sort_mode.borrow().clone();
    
    let all = handle.all_entries.borrow().clone();
    let mut filtered = babydra_common::filter_entries(&all, query);
    sort_entries(&mut filtered, &sort);
    handle.entries.replace(filtered.clone());

    update::update_content_view_ui(handle);
}
