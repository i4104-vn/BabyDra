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
    let nav_cb = Rc::new(nav_callback) as Rc<dyn Fn(PathBuf)>;

    let handle = ContentViewHandle {
        widgets: widgets.clone(),
        entries: entries.clone(),
        all_entries: all_entries.clone(),
        current_path: current_path.clone(),
        current_mode: current_mode.clone(),
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
            let sel: Vec<usize> = fb.selected_children().iter().map(|c| c.index() as usize).collect();
            sc(sel);
        });
    }
    {
        let sc = sc_fn.clone();
        widgets.listbox.connect_selected_rows_changed(move |lb| {
            let sel: Vec<usize> = lb.selected_rows().iter().map(|r| r.index() as usize).collect();
            sc(sel);
        });
    }

    // Wire double click activations
    {
        let e_ref = entries.clone();
        let nav = nav_cb.clone();
        widgets.flowbox.connect_child_activated(move |_, child| {
            let idx = child.index() as usize;
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
            let idx = row.index() as usize;
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

/// Changes the layout layout style of content view stack.
pub fn set_content_view_mode(handle: &ContentViewHandle, mode: &str) {
    handle.current_mode.replace(mode.to_string());
    handle.widgets.stack.set_visible_child_name(mode);
    
    let e = handle.entries.borrow().clone();
    let cp = handle.current_path.borrow().clone();
    update::update_content_view_ui(&handle.widgets, &e, &handle.nav_callback, &cp, mode);
}

/// Updates files in view area.
pub fn update_content_view(handle: &ContentViewHandle, entries: &[FileEntry], current_path: PathBuf) {
    handle.all_entries.replace(entries.to_vec());
    handle.entries.replace(entries.to_vec());
    handle.current_path.replace(current_path);

    let mode = handle.current_mode.borrow().clone();
    handle.widgets.stack.set_visible_child_name(&mode);

    let cp = handle.current_path.borrow().clone();
    update::update_content_view_ui(&handle.widgets, entries, &handle.nav_callback, &cp, &mode);
}

/// Filters content files list.
pub fn filter_content_view(handle: &ContentViewHandle, query: &str) {
    let all = handle.all_entries.borrow().clone();
    let filtered = babydra_common::filter_entries(&all, query);
    handle.entries.replace(filtered);

    let mode = handle.current_mode.borrow().clone();
    let cp = handle.current_path.borrow().clone();
    update::update_content_view_ui(&handle.widgets, &handle.entries.borrow(), &handle.nav_callback, &cp, &mode);
}
