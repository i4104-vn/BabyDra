use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;
use gtk4::prelude::*;
use babydra_common::FileEntry;
pub use babydra_common::{ContentViewWidgets, ContentViewHandle, sort_entries};

mod render;
pub mod items;
mod gestures;
mod rendering;
mod actions;

pub use rendering::renderer::update_content_view_ui;
pub use actions::{set_content_view_mode, set_content_view_sort, update_content_view, filter_content_view};

/// Creates the content view area widgets and returns the scroll container and ContentViewHandle state handle.
pub fn create_content_view(
    nav_callback: impl Fn(PathBuf) + 'static,
    selection_callback: impl Fn(Vec<FileEntry>) + 'static,
) -> (gtk4::Box, ContentViewHandle) {
    let widgets = render::build_content_view_ui();

    let settings = babydra_common::load_explore_settings();
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
    gestures::wire_listbox_controllers(&widgets, entries.clone(), nav_cb.clone(), sc_fn.clone(), current_path.clone(), selected_paths.clone());
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
    let settings = babydra_common::load_explore_settings();
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

    gestures::wire_grid_flowbox_controllers(&flowbox, entries, nav_cb, sc_fn, grid_container, current_path, selected_paths);

    flowbox
}
