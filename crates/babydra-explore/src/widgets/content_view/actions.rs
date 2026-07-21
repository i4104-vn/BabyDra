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
