use std::path::PathBuf;
use babydra_common::{FileEntry, ContentViewHandle, sort_entries};

/// Changes the layout style of content view stack.
pub fn set_content_view_mode(handle: &ContentViewHandle, mode: &str) {
    handle.current_mode.replace(mode.to_string());
    handle.widgets.stack.set_visible_child_name(mode);
    
    let mut e = handle.entries.borrow().clone();
    let sort = handle.sort_mode.borrow().clone();
    
    // Sort with the new mode
    sort_entries(&mut e, &sort);
    handle.entries.replace(e.clone());
    
    super::rendering::renderer::update_content_view_ui(handle);
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

    super::rendering::renderer::update_content_view_ui(handle);
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

    super::rendering::renderer::update_content_view_ui(handle);
}

/// Filters content files list.
pub fn filter_content_view(handle: &ContentViewHandle, query: &str) {
    let sort = handle.sort_mode.borrow().clone();
    
    let all = handle.all_entries.borrow().clone();
    let mut filtered = babydra_common::filter_entries(&all, query);
    sort_entries(&mut filtered, &sort);
    handle.entries.replace(filtered.clone());

    super::rendering::renderer::update_content_view_ui(handle);
}

/// Wires navigation buttons (back, forward, up, refresh) and address bar entry handlers.
pub fn wire_content_view_navigation(
    widgets: &babydra_common::ContentViewWidgets,
    nav_cb: std::rc::Rc<dyn Fn(PathBuf)>,
    current_path: std::rc::Rc<std::cell::RefCell<PathBuf>>,
    history: std::rc::Rc<std::cell::RefCell<Vec<PathBuf>>>,
    history_index: std::rc::Rc<std::cell::RefCell<usize>>,
) {
    use gtk4::prelude::*;

    // Wire pane navigation button clicks
    {
        let history_c = history.clone();
        let history_index_c = history_index.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_back.connect_clicked(move |_| {
            let path_opt = {
                let hist = history_c.borrow();
                let mut idx = history_index_c.borrow_mut();
                if *idx > 0 {
                    *idx -= 1;
                    Some(hist[*idx].clone())
                } else {
                    None
                }
            };
            if let Some(path) = path_opt {
                nav_c(path);
            }
        });
    }
    {
        let history_c = history.clone();
        let history_index_c = history_index.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_forward.connect_clicked(move |_| {
            let path_opt = {
                let hist = history_c.borrow();
                let mut idx = history_index_c.borrow_mut();
                if *idx + 1 < hist.len() {
                    *idx += 1;
                    Some(hist[*idx].clone())
                } else {
                    None
                }
            };
            if let Some(path) = path_opt {
                nav_c(path);
            }
        });
    }
    {
        let current_path_c = current_path.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_up.connect_clicked(move |_| {
            let current = current_path_c.borrow().clone();
            if let Some(parent) = current.parent() {
                nav_c(parent.to_path_buf());
            }
        });
    }
    {
        let current_path_c = current_path.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_refresh.connect_clicked(move |_| {
            let current = current_path_c.borrow().clone();
            nav_c(current);
        });
    }

    // Address bar toggle on click
    {
        let current_path_c = current_path.clone();
        let address_stack_c = widgets.address_stack.clone();
        let entry_address_c = widgets.entry_address.clone();
        let address_wrap_c = widgets.address_wrap.clone();
        let gesture = gtk4::GestureClick::new();
        gesture.connect_pressed(move |_, _, _, _| {
            if address_stack_c.visible_child_name().as_deref() == Some("breadcrumbs") {
                let path = current_path_c.borrow().clone();
                entry_address_c.set_text(&path.to_string_lossy());
                address_stack_c.set_visible_child_name("address");
                entry_address_c.grab_focus();
            }
        });
        address_wrap_c.add_controller(gesture);
    }

    // Address Entry activated (Enter key pressed)
    {
        let nav_c = nav_cb.clone();
        let address_stack_c = widgets.address_stack.clone();
        widgets.entry_address.connect_activate(move |entry| {
            let text = entry.text().to_string();
            let p = PathBuf::from(text);
            if p.exists() {
                nav_c(p);
            }
            address_stack_c.set_visible_child_name("breadcrumbs");
        });
    }
}
