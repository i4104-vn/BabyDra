use crate::widgets::state::ContentViewHandle;
use babydra_core::{sort_entries, FileEntry, TabState};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

/// Changes the layout style of content view stack.
pub fn set_view_mode(handle: &ContentViewHandle, mode: &str) {
    handle.tab.borrow_mut().view_mode = mode.to_string();
    handle.widgets.stack.set_visible_child_name(mode);

    // Sort in place to avoid deep-cloning every FileEntry
    {
        let sort = handle.sort_mode.borrow().clone();
        let mut e = handle.entries.borrow_mut();
        sort_entries(&mut e, &sort);
    }

    super::render::update_content_ui(handle);
}

/// Changes the sorting mode of the content view and updates the layout.
pub fn set_view_sort(handle: &ContentViewHandle, sort_mode: &str) {
    handle.sort_mode.replace(sort_mode.to_string());

    // Sort in place to avoid deep-cloning every FileEntry
    {
        let mut e = handle.entries.borrow_mut();
        sort_entries(&mut e, sort_mode);
    }
    {
        let mut all = handle.all_entries.borrow_mut();
        sort_entries(&mut all, sort_mode);
    }

    super::render::update_content_ui(handle);
}

/// Updates files in view area.
pub fn update_content_view(
    handle: &ContentViewHandle,
    entries: &[FileEntry],
    current_path: PathBuf,
) {
    let (sort, mode) = {
        let tab = handle.tab.borrow();
        (handle.sort_mode.borrow().clone(), tab.view_mode.clone())
    };

    let mut sorted = entries.to_vec();
    sort_entries(&mut sorted, &sort);
    handle.all_entries.replace(sorted.clone());
    handle.entries.replace(sorted);
    handle.tab.borrow_mut().current_path = current_path;

    handle.widgets.stack.set_visible_child_name(&mode);

    super::render::update_content_ui(handle);
}

/// Updates files in view area silently without resetting progress bar or layout flash.
pub fn update_content_quiet(
    handle: &ContentViewHandle,
    entries: &[FileEntry],
    current_path: PathBuf,
) {
    let (sort, mode) = {
        let tab = handle.tab.borrow();
        (handle.sort_mode.borrow().clone(), tab.view_mode.clone())
    };

    let mut sorted = entries.to_vec();
    sort_entries(&mut sorted, &sort);
    handle.all_entries.replace(sorted.clone());
    handle.entries.replace(sorted);
    handle.tab.borrow_mut().current_path = current_path;

    handle.widgets.stack.set_visible_child_name(&mode);

    super::render::render_silent(handle);
}

/// Filters content files list.
pub fn filter_content_view(handle: &ContentViewHandle, query: &str) {
    let sort = handle.sort_mode.borrow().clone();

    let all = handle.all_entries.borrow().clone();
    let mut filtered = babydra_core::filter_entries(&all, query);
    sort_entries(&mut filtered, &sort);
    handle.entries.replace(filtered);

    super::render::update_content_ui(handle);
}

const SEARCH_DEBOUNCE_MS: u64 = 200;

/// Wires the search entry with a debounce and off-thread fuzzy matching so fast
/// typing does not run rayon matching plus a full widget rebuild per keystroke.
pub fn wire_search_filter(
    widgets: &crate::widgets::state::ContentViewWidgets,
    handle: &ContentViewHandle,
) {
    let handle_c = handle.clone();
    let pending = Rc::new(RefCell::new(None::<glib::SourceId>));

    widgets.search.connect_changed(move |entry| {
        if let Some(source_id) = pending.borrow_mut().take() {
            source_id.remove();
        }

        let query = entry.text().to_string();
        if query.is_empty() {
            filter_content_view(&handle_c, "");
            return;
        }

        let handle_t = handle_c.clone();
        let pending_t = pending.clone();
        let source_id = glib::timeout_add_local_once(
            std::time::Duration::from_millis(SEARCH_DEBOUNCE_MS),
            move || {
                pending_t.borrow_mut().take();
                glib::spawn_future_local(async move {
                    // Snapshot inputs and match on a worker thread
                    let all = handle_t.all_entries.borrow().clone();
                    let sort = handle_t.sort_mode.borrow().clone();
                    let q = query.clone();
                    let filtered = tokio::task::spawn_blocking(move || {
                        let mut filtered = babydra_core::filter_entries(&all, &q);
                        babydra_core::sort_entries(&mut filtered, &sort);
                        filtered
                    })
                    .await
                    .unwrap_or_default();

                    // Ignore stale results when the query changed meanwhile
                    if handle_t.widgets.search.text() == query {
                        handle_t.entries.replace(filtered);
                        super::render::update_content_ui(&handle_t);
                    }
                });
            },
        );
        *pending.borrow_mut() = Some(source_id);
    });
}

/// Wires navigation buttons (back, forward, up, refresh) and address bar entry
/// handlers, all driven by the pane's `TabState` history.
pub fn wire_content_nav(
    widgets: &crate::widgets::state::ContentViewWidgets,
    nav_cb: std::rc::Rc<dyn Fn(PathBuf)>,
    tab: Rc<RefCell<TabState>>,
) {
    // Wire pane navigation button clicks
    {
        let tab_c = tab.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_back.connect_clicked(move |_| {
            let path_opt = {
                let mut t = tab_c.borrow_mut();
                if t.go_back() {
                    Some(t.current_path.clone())
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
        let tab_c = tab.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_forward.connect_clicked(move |_| {
            let path_opt = {
                let mut t = tab_c.borrow_mut();
                if t.go_forward() {
                    Some(t.current_path.clone())
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
        let tab_c = tab.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_up.connect_clicked(move |_| {
            let parent = tab_c.borrow().current_path.parent().map(|p| p.to_path_buf());
            if let Some(parent) = parent {
                nav_c(parent);
            }
        });
    }
    {
        let tab_c = tab.clone();
        let nav_c = nav_cb.clone();
        widgets.btn_refresh.connect_clicked(move |_| {
            let current = tab_c.borrow().current_path.clone();
            nav_c(current);
        });
    }

    // Address bar toggle on click
    {
        let tab_c = tab.clone();
        let address_stack_c = widgets.address_stack.clone();
        let entry_address_c = widgets.entry_address.clone();
        let address_wrap_c = widgets.address_wrap.clone();
        let gesture = gtk4::GestureClick::new();
        gesture.connect_pressed(move |_, _, _, _| {
            if address_stack_c.visible_child_name().as_deref() == Some("breadcrumbs") {
                let path = tab_c.borrow().current_path.clone();
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

/// Selects all items in the active content view (grid or list).
pub fn select_all_items(handle: &ContentViewHandle) {
    let mode = handle.tab.borrow().view_mode.clone();
    let entries = handle.entries.borrow().clone();
    let all_paths: Vec<PathBuf> = entries.iter().map(|e| e.path.clone()).collect();

    if mode == "list" {
        handle.widgets.listbox.select_all();
    } else {
        let mut sibling = handle.widgets.grid_container.first_child();
        while let Some(child) = sibling {
            if let Some(fb) = child.downcast_ref::<gtk4::FlowBox>() {
                fb.select_all();
            }
            sibling = child.next_sibling();
        }
    }

    *handle.selected_paths.borrow_mut() = all_paths.clone();
    (handle.selection_callback)(all_paths);
}
