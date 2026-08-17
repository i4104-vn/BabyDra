use babydra_core::FileEntry;
pub use babydra_core::{sort_entries, ContentViewHandle, ContentViewWidgets};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

mod actions;
mod gestures;
pub mod items;
mod render;
mod rendering;

pub use actions::{
    filter_content_view, set_content_view_mode, set_content_view_sort, update_content_view,
    update_content_view_silent,
};
pub use rendering::renderer::{update_content_view_ui, update_content_view_ui_silent};

/// Creates the content view area widgets and returns the scroll container and ContentViewHandle state handle.
pub fn create_content_view(
    nav_callback: impl Fn(PathBuf) + 'static,
    selection_callback: impl Fn(Vec<FileEntry>) + 'static,
) -> (gtk4::Box, ContentViewHandle) {
    let widgets = render::build_content_view_ui();

    let settings = babydra_core::load_explore_settings();
    widgets.stack.set_visible_child_name(&settings.view_mode);

    let entries: Rc<RefCell<Vec<FileEntry>>> = Rc::new(RefCell::new(Vec::new()));
    let all_entries: Rc<RefCell<Vec<FileEntry>>> = Rc::new(RefCell::new(Vec::new()));
    let current_path = Rc::new(RefCell::new(PathBuf::new()));
    let current_mode = Rc::new(RefCell::new(settings.view_mode));
    let sort_mode = Rc::new(RefCell::new("auto".to_string()));
    let nav_cb = Rc::new(nav_callback) as Rc<dyn Fn(PathBuf)>;

    let entries_clone = entries.clone();
    let selected_paths = Rc::new(RefCell::new(Vec::new()));
    let selected_paths_c = selected_paths.clone();
    let sel_cb = Rc::new(selection_callback) as Rc<dyn Fn(Vec<FileEntry>)>;

    let sc_fn = Rc::new(move |selected_paths_list: Vec<PathBuf>| {
        let mut list = Vec::new();
        let b = entries_clone.borrow();
        for path in &selected_paths_list {
            if let Some(entry) = b.iter().find(|e| e.path == *path) {
                list.push(entry.clone());
            }
        }
        *selected_paths_c.borrow_mut() = selected_paths_list;
        sel_cb(list);
    }) as Rc<dyn Fn(Vec<PathBuf>)>;

    let render_generation = Rc::new(RefCell::new(0u64));
    let history = Rc::new(RefCell::new(Vec::<PathBuf>::new()));
    let history_index = Rc::new(RefCell::new(0usize));

    // Wire pane navigation & address bar entry
    actions::wire_content_view_navigation(
        &widgets,
        nav_cb.clone(),
        current_path.clone(),
        history.clone(),
        history_index.clone(),
    );

    let handle = ContentViewHandle {
        widgets: widgets.clone(),
        entries: entries.clone(),
        all_entries: all_entries.clone(),
        current_path: current_path.clone(),
        current_mode: current_mode.clone(),
        sort_mode: sort_mode.clone(),
        nav_callback: nav_cb.clone(),
        selection_callback: sc_fn.clone(),
        selected_paths: selected_paths.clone(),
        render_generation: render_generation.clone(),
        history: history.clone(),
        history_index: history_index.clone(),
    };

    // Wire search filter change callback
    {
        let handle_c = handle.clone();
        widgets.search.connect_changed(move |entry| {
            filter_content_view(&handle_c, &entry.text());
        });
    }

    // Wire all controllers/gestures for ListBox and overlay background
    gestures::wire_listbox_controllers(
        &widgets,
        entries.clone(),
        nav_cb.clone(),
        sc_fn.clone(),
        current_path.clone(),
        selected_paths.clone(),
    );
    gestures::wire_background_controllers(&widgets, current_path.clone(), nav_cb.clone());

    (widgets.container.clone(), handle)
}

/// Dynamic FlowBox builder helper for Grid grouping/categories
pub fn create_grid_flowbox(
    entries: Rc<RefCell<Vec<FileEntry>>>,
    nav_cb: Rc<dyn Fn(PathBuf)>,
    sc_fn: Rc<dyn Fn(Vec<PathBuf>)>,
    grid_container: &gtk4::Box,
    current_path: Rc<RefCell<PathBuf>>,
    selected_paths: Rc<RefCell<Vec<PathBuf>>>,
) -> gtk4::FlowBox {
    let settings = babydra_core::load_explore_settings();
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

    gestures::wire_grid_flowbox_controllers(
        &flowbox,
        entries,
        nav_cb,
        sc_fn,
        grid_container,
        current_path,
        selected_paths,
    );

    flowbox
}
