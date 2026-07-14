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

    let entries = Rc::new(RefCell::new(Vec::new()));
    let all_entries = Rc::new(RefCell::new(Vec::new()));
    let current_path = Rc::new(RefCell::new(PathBuf::new()));
    let current_mode = Rc::new(RefCell::new("icons".to_string()));
    let sort_mode = Rc::new(RefCell::new("auto".to_string()));
    let nav_cb = Rc::new(nav_callback) as Rc<dyn Fn(PathBuf)>;

    let handle = ContentViewHandle {
        widgets: widgets.clone(),
        entries: entries.clone(),
        all_entries: all_entries.clone(),
        current_path: current_path.clone(),
        current_mode: current_mode.clone(),
        sort_mode: sort_mode.clone(),
        nav_callback: nav_cb.clone(),
    };

    // Wire selection changed listeners
    let entries_clone = entries.clone();
    let sel_cb = Rc::new(selection_callback) as Rc<dyn Fn(Vec<FileEntry>)>;
    
    let sc_fn = {
        let e_ref = entries_clone.clone();
        let cb_ref = sel_cb.clone();
        move |selected_indices: Vec<usize>| {
            let mut list = Vec::new();
            let b = e_ref.borrow();
            for idx in selected_indices {
                if idx < b.len() {
                    list.push(b[idx].clone());
                }
            }
            cb_ref(list);
        }
    };

    {
        let sc = sc_fn.clone();
        widgets.flowbox.connect_selected_children_changed(move |fb| {
            let sel: Vec<usize> = fb.selected_children().iter()
                .map(|c| c.property::<String>("name").parse::<usize>().unwrap_or(usize::MAX))
                .filter(|&idx| idx != usize::MAX)
                .collect();
            sc(sel);
        });
    }
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

    // Wire double click activations
    {
        let e_ref = entries.clone();
        let nav = nav_cb.clone();
        widgets.flowbox.connect_child_activated(move |_, child| {
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

/// Helper to sort entries: directories first (sorted by name), then files (sorted by type/extension, then by name)
pub fn sort_entries(entries: &mut [FileEntry], sort_mode: &str) {
    entries.sort_by(|a, b| {
        if sort_mode == "date" {
            let weight = |e: &FileEntry| -> u32 {
                if let Some(modified) = e.modified {
                    if let Ok(duration) = std::time::SystemTime::now().duration_since(modified) {
                        let days = duration.as_secs() / 86400;
                        if days == 0 { 0 }
                        else if days == 1 { 1 }
                        else if days <= 7 { 2 }
                        else if days <= 30 { 3 }
                        else { 4 }
                    } else {
                        0
                    }
                } else {
                    5
                }
            };
            let w_a = weight(a);
            let w_b = weight(b);
            if w_a != w_b {
                return w_a.cmp(&w_b);
            }
        }

        let a_is_dir = matches!(a.file_type, babydra_common::FileType::Directory);
        let b_is_dir = matches!(b.file_type, babydra_common::FileType::Directory);
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
    });
}

/// Changes the layout layout style of content view stack.
pub fn set_content_view_mode(handle: &ContentViewHandle, mode: &str) {
    handle.current_mode.replace(mode.to_string());
    handle.widgets.stack.set_visible_child_name(mode);
    
    let e = handle.entries.borrow().clone();
    let cp = handle.current_path.borrow().clone();
    let sort = handle.sort_mode.borrow().clone();
    update::update_content_view_ui(&handle.widgets, &e, &handle.nav_callback, &cp, mode, &sort);
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

    let cp = handle.current_path.borrow().clone();
    let mode = handle.current_mode.borrow().clone();
    update::update_content_view_ui(&handle.widgets, &e, &handle.nav_callback, &cp, &mode, sort_mode);
}

/// Updates files in view area.
pub fn update_content_view(handle: &ContentViewHandle, entries: &[FileEntry], current_path: PathBuf) {
    let sort = handle.sort_mode.borrow().clone();
    let mut sorted = entries.to_vec();
    sort_entries(&mut sorted, &sort);
    handle.all_entries.replace(sorted.clone());
    handle.entries.replace(sorted.clone());
    handle.current_path.replace(current_path);

    let mode = handle.current_mode.borrow().clone();
    handle.widgets.stack.set_visible_child_name(&mode);

    let cp = handle.current_path.borrow().clone();
    update::update_content_view_ui(&handle.widgets, &sorted, &handle.nav_callback, &cp, &mode, &sort);
}

/// Filters content files list.
pub fn filter_content_view(handle: &ContentViewHandle, query: &str) {
    let sort = handle.sort_mode.borrow().clone();
    let all = handle.all_entries.borrow().clone();
    let mut filtered = babydra_common::filter_entries(&all, query);
    sort_entries(&mut filtered, &sort);
    handle.entries.replace(filtered.clone());

    let mode = handle.current_mode.borrow().clone();
    let cp = handle.current_path.borrow().clone();
    update::update_content_view_ui(&handle.widgets, &filtered, &handle.nav_callback, &cp, &mode, &sort);
}
