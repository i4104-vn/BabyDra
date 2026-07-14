use gtk4::prelude::*;
use gtk4::ScrolledWindow;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::FileEntry;
pub use babydra_common::{ContentViewWidgets, ContentViewHandle};

mod render;
mod update;

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
        widgets.listbox.connect_row_activated(move |_, row| {
            let idx = row.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX);
            let entry_opt = {
                let b = e_ref.borrow();
                if idx < b.len() {
                    Some(b[idx].clone())
                } else {
                    None
                }
            };
            if let Some(entry) = entry_opt {
                if matches!(entry.file_type, babydra_common::FileType::Directory) {
                    nav(entry.path);
                } else {
                    let uri = format!("file://{}", entry.path.to_string_lossy());
                    let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
                }
            }
        });
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
        flowbox.connect_child_activated(move |_, child| {
            let idx = child.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX);
            let entry_opt = {
                let b = e_ref.borrow();
                if idx < b.len() {
                    Some(b[idx].clone())
                } else {
                    None
                }
            };
            if let Some(entry) = entry_opt {
                if matches!(entry.file_type, babydra_common::FileType::Directory) {
                    nav(entry.path);
                } else {
                    let uri = format!("file://{}", entry.path.to_string_lossy());
                    let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
                }
            }
        });
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

/// Helper to sort entries: directories first (sorted by name), then files.
/// In grid view (icons mode), directories are always placed before all files.
/// In list view, folders and files can be grouped by date weight or group type.
pub fn sort_entries(entries: &mut [FileEntry], sort_mode: &str, is_grid: bool) {
    entries.sort_by(|a, b| {
        let a_is_dir = matches!(a.file_type, babydra_common::FileType::Directory);
        let b_is_dir = matches!(b.file_type, babydra_common::FileType::Directory);

        if is_grid {
            // Grid view: Directories always first!
            match (a_is_dir, b_is_dir) {
                (true, false) => return std::cmp::Ordering::Less,
                (false, true) => return std::cmp::Ordering::Greater,
                _ => {}
            }
            if a_is_dir && b_is_dir {
                return a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase());
            }
            // Both are files, sort by sort_mode
            if sort_mode == "date" {
                let get_weight = |e: &FileEntry| -> u32 {
                    if let Some(modified) = e.modified {
                        let datetime: chrono::DateTime<chrono::Local> = modified.into();
                        let now = chrono::Local::now();
                        let date_naive = datetime.date_naive();
                        let now_naive = now.date_naive();
                        if date_naive == now_naive {
                            0
                        } else if date_naive == now_naive - chrono::Duration::days(1) {
                            1
                        } else {
                            let diff = (now_naive - date_naive).num_days();
                            if diff >= 2 && diff <= 7 {
                                diff as u32
                            } else if diff > 7 {
                                8
                            } else {
                                0
                            }
                        }
                    } else {
                        9
                    }
                };
                let w_a = get_weight(a);
                let w_b = get_weight(b);
                if w_a != w_b {
                    return w_a.cmp(&w_b);
                }
            }
            let ext_a = a.path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
            let ext_b = b.path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
            let cmp_type = ext_a.cmp(&ext_b);
            if cmp_type == std::cmp::Ordering::Equal {
                a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase())
            } else {
                cmp_type
            }
        } else {
            // List view
            if sort_mode == "date" {
                let get_weight = |e: &FileEntry| -> u32 {
                    if let Some(modified) = e.modified {
                        let datetime: chrono::DateTime<chrono::Local> = modified.into();
                        let now = chrono::Local::now();
                        let date_naive = datetime.date_naive();
                        let now_naive = now.date_naive();
                        if date_naive == now_naive {
                            0
                        } else if date_naive == now_naive - chrono::Duration::days(1) {
                            1
                        } else {
                            let diff = (now_naive - date_naive).num_days();
                            if diff >= 2 && diff <= 7 {
                                diff as u32
                            } else if diff > 7 {
                                8
                            } else {
                                0
                            }
                        }
                    } else {
                        9
                    }
                };
                let w_a = get_weight(a);
                let w_b = get_weight(b);
                if w_a != w_b {
                    return w_a.cmp(&w_b);
                }
            }

            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                (true, true) => {
                    a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase())
                }
                (false, false) => {
                    let ext_a = a.path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
                    let ext_b = b.path.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
                    let cmp_type = ext_a.cmp(&ext_b);
                    if cmp_type == std::cmp::Ordering::Equal {
                        a.display_name.to_lowercase().cmp(&b.display_name.to_lowercase())
                    } else {
                        cmp_type
                    }
                }
            }
        }
    });
}

/// Changes the layout layout style of content view stack.
pub fn set_content_view_mode(handle: &ContentViewHandle, mode: &str) {
    handle.current_mode.replace(mode.to_string());
    handle.widgets.stack.set_visible_child_name(mode);
    
    let mut e = handle.entries.borrow().clone();
    let sort = handle.sort_mode.borrow().clone();
    
    // Sort with the new mode
    let is_grid = mode == "icons";
    sort_entries(&mut e, &sort, is_grid);
    handle.entries.replace(e.clone());
    
    update::update_content_view_ui(handle);
}

/// Changes the sorting mode of the content view and updates the layout.
pub fn set_content_view_sort(handle: &ContentViewHandle, sort_mode: &str) {
    handle.sort_mode.replace(sort_mode.to_string());
    let is_grid = *handle.current_mode.borrow() == "icons";
    
    // Sort current entries
    let mut e = handle.entries.borrow().clone();
    sort_entries(&mut e, sort_mode, is_grid);
    handle.entries.replace(e.clone());
    
    // Sort all entries
    let mut all = handle.all_entries.borrow().clone();
    sort_entries(&mut all, sort_mode, is_grid);
    handle.all_entries.replace(all);

    update::update_content_view_ui(handle);
}

/// Updates files in view area.
pub fn update_content_view(handle: &ContentViewHandle, entries: &[FileEntry], current_path: PathBuf) {
    let sort = handle.sort_mode.borrow().clone();
    let mode = handle.current_mode.borrow().clone();
    let is_grid = mode == "icons";
    
    let mut sorted = entries.to_vec();
    sort_entries(&mut sorted, &sort, is_grid);
    handle.all_entries.replace(sorted.clone());
    handle.entries.replace(sorted.clone());
    handle.current_path.replace(current_path);

    handle.widgets.stack.set_visible_child_name(&mode);

    update::update_content_view_ui(handle);
}

/// Filters content files list.
pub fn filter_content_view(handle: &ContentViewHandle, query: &str) {
    let sort = handle.sort_mode.borrow().clone();
    let mode = handle.current_mode.borrow().clone();
    let is_grid = mode == "icons";
    
    let all = handle.all_entries.borrow().clone();
    let mut filtered = babydra_common::filter_entries(&all, query);
    sort_entries(&mut filtered, &sort, is_grid);
    handle.entries.replace(filtered.clone());

    update::update_content_view_ui(handle);
}
