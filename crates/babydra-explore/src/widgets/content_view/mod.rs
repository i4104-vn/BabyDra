pub use crate::widgets::state::{ContentViewHandle, ContentViewWidgets};
use babydra_core::{FileEntry, TabState};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

mod actions;
mod gestures;
mod grid_item;
mod grid_renderer;
mod list_renderer;
mod render;

pub use actions::{
    filter_content_view, select_all_items, set_view_mode, set_view_sort, update_content_quiet,
    update_content_view, wire_search_filter,
};
pub use render::{render_silent, update_content_ui};

/// Creates the content view area widgets and returns the scroll container and ContentViewHandle state handle.
pub fn create_content_view(
    nav_callback: impl Fn(PathBuf) + 'static,
    selection_callback: impl Fn(Vec<FileEntry>) + 'static,
) -> (gtk4::Box, ContentViewHandle) {
    let widgets = render::build_content_view();

    let settings = babydra_core::load_explore_cfg();
    widgets.stack.set_visible_child_name(&settings.view_mode);

    let entries: Rc<RefCell<Vec<FileEntry>>> = Rc::new(RefCell::new(Vec::new()));
    let all_entries: Rc<RefCell<Vec<FileEntry>>> = Rc::new(RefCell::new(Vec::new()));
    let mut tab = TabState::new(PathBuf::new());
    tab.view_mode = settings.view_mode.clone();
    let tab = Rc::new(RefCell::new(tab));
    let sort_mode = Rc::new(RefCell::new("auto".to_string()));
    let nav_cb = Rc::new(nav_callback) as Rc<dyn Fn(PathBuf)>;

    let entries_clone = entries.clone();
    let selected_paths = Rc::new(RefCell::new(Vec::new()));
    let selected_paths_c = selected_paths.clone();
    let sel_cb = Rc::new(selection_callback) as Rc<dyn Fn(Vec<FileEntry>)>;

    let sc_fn = Rc::new(move |selected_paths_list: Vec<PathBuf>| {
        // Index entries once so select-all stays linear instead of O(selected x entries)
        let borrowed = entries_clone.borrow();
        let mut by_path = std::collections::HashMap::with_capacity(borrowed.len());
        for e in borrowed.iter() {
            by_path.insert(&e.path, e);
        }
        let list = selected_paths_list
            .iter()
            .filter_map(|path| by_path.get(path).map(|e| (*e).clone()))
            .collect();
        drop(borrowed);
        *selected_paths_c.borrow_mut() = selected_paths_list;
        sel_cb(list);
    }) as Rc<dyn Fn(Vec<PathBuf>)>;

    let render_generation = Rc::new(RefCell::new(0u64));
    let render_signature: Rc<RefCell<Option<u64>>> = Rc::new(RefCell::new(None));

    // Wire pane navigation & address bar entry
    actions::wire_content_nav(&widgets, nav_cb.clone(), tab.clone());

    let handle = ContentViewHandle {
        widgets: widgets.clone(),
        entries: entries.clone(),
        all_entries: all_entries.clone(),
        tab: tab.clone(),
        sort_mode: sort_mode.clone(),
        nav_callback: nav_cb.clone(),
        selection_callback: sc_fn.clone(),
        selected_paths: selected_paths.clone(),
        render_generation: render_generation.clone(),
        render_signature: render_signature.clone(),
    };

    // Wire search filter change callback (debounced, matched off-thread)
    actions::wire_search_filter(&widgets, &handle);

    // Wire all controllers/gestures for ListBox and overlay background
    gestures::wire_listbox_ctrls(
        &widgets,
        entries.clone(),
        nav_cb.clone(),
        sc_fn.clone(),
        tab.clone(),
        selected_paths.clone(),
    );
    gestures::wire_bg_controllers(&widgets, tab.clone(), nav_cb.clone());

    (widgets.container.clone(), handle)
}

/// Dynamic FlowBox builder helper for Grid grouping/categories
pub fn create_grid_flowbox(
    entries: Rc<RefCell<Vec<FileEntry>>>,
    nav_cb: Rc<dyn Fn(PathBuf)>,
    sc_fn: Rc<dyn Fn(Vec<PathBuf>)>,
    grid_container: &gtk4::Box,
    tab: Rc<RefCell<TabState>>,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
) -> gtk4::FlowBox {
    let settings = babydra_core::load_explore_cfg();
    let activate_on_single = !settings.double_click_to_open;

    let flowbox = gtk4::FlowBox::builder()
        .valign(gtk4::Align::Start)
        .max_children_per_line(20)
        .min_children_per_line(1)
        .selection_mode(gtk4::SelectionMode::Multiple)
        .activate_on_single_click(activate_on_single)
        .row_spacing(10)
        .column_spacing(10)
        .build();

    // Item drags must start DnD, not the FlowBox native rubberband
    babydra_ui_kit::components::explore::disable_native_rubberband(&flowbox);

    gestures::wire_grid_ctrls(
        &flowbox,
        entries,
        nav_cb,
        sc_fn,
        grid_container,
        tab,
        selected_paths,
    );

    flowbox
}
