use gtk4::prelude::*;
use gtk4::FlowBoxChild;
use std::path::PathBuf;
use std::rc::Rc;
use babydra_common::FileEntry;
use std::cell::RefCell;

pub fn create_flow_child(
    idx: usize,
    entry: &FileEntry,
    current_path: &PathBuf,
    nav_callback: &Rc<dyn Fn(PathBuf)>,
    entries: Rc<RefCell<Vec<FileEntry>>>,
) -> FlowBoxChild {
    let target_entry = entry.clone();
    let cp = current_path.clone();
    let nav = nav_callback.clone();
    let entries_c = entries.clone();

    let flow_child = baby_utils::components::create_grid_file_item(
        idx,
        entry,
        move |widget, x, y| {
            let mut target_paths = Vec::new();
            if let Some(child) = widget.parent().and_then(|p| p.downcast::<gtk4::FlowBoxChild>().ok()) {
                if let Some(flowbox) = child.parent().and_then(|p| p.downcast::<gtk4::FlowBox>().ok()) {
                    let selected = flowbox.selected_children();
                    let is_clicked_selected = selected.contains(&child);
                    
                    let mut target_indices = Vec::new();
                    if is_clicked_selected {
                        if let Some(grid_container) = flowbox.parent().and_then(|p| p.downcast::<gtk4::Box>().ok()) {
                            let mut sibling = grid_container.first_child();
                            while let Some(c) = sibling {
                                if let Some(fb) = c.downcast_ref::<gtk4::FlowBox>() {
                                    for item in fb.selected_children() {
                                        if let Ok(idx_val) = item.property::<String>("name").parse::<usize>() {
                                            target_indices.push(idx_val);
                                        }
                                    }
                                }
                                sibling = c.next_sibling();
                            }
                        }
                    } else {
                        flowbox.select_child(&child);
                        if let Ok(idx_val) = child.property::<String>("name").parse::<usize>() {
                            target_indices.push(idx_val);
                        }
                    }
                    
                    let b = entries_c.borrow();
                    for idx_val in target_indices {
                        if idx_val < b.len() {
                            target_paths.push(b[idx_val].path.clone());
                        }
                    }
                }
            }
            if target_paths.is_empty() {
                target_paths.push(target_entry.path.clone());
            }

            crate::widgets::context_menu::show_for_file(
                widget,
                x,
                y,
                target_paths,
                cp.clone(),
                nav.clone(),
            );
        },
    );

    // Dim item if it's currently in the cut clipboard
    let is_cut = baby_utils::explore::CLIPBOARD.with(|cb| {
        cb.borrow().as_ref().map(|(paths, cut)| *cut && paths.contains(&entry.path)).unwrap_or(false)
    });
    if is_cut {
        if let Some(child_widget) = flow_child.child() {
            child_widget.add_css_class("cut-item");
        }
    }

    let double_click_gesture = gtk4::GestureClick::new();
    double_click_gesture.set_button(1);
    let target_path = entry.path.clone();
    let is_dir = matches!(entry.file_type, babydra_common::FileType::Directory);
    let nav_c = nav_callback.clone();
    double_click_gesture.connect_pressed(move |_, n_press, _, _| {
        if n_press == 2 {
            if is_dir {
                nav_c(target_path.clone());
            } else {
                let uri = format!("file://{}", target_path.to_string_lossy());
                let _ = gtk4::gio::AppInfo::launch_default_for_uri(&uri, gtk4::gio::AppLaunchContext::NONE);
            }
        }
    });
    flow_child.add_controller(double_click_gesture);

    flow_child
}
