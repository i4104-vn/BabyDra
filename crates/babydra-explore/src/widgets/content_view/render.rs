use crate::widgets::state::ContentViewWidgets;
use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box, Entry, FlowBox, ListBox, Orientation, ScrolledWindow, Stack};

/// Builds the content view widgets: icon grid, list view, navigation bar, and search entry.
pub fn build_content_view() -> ContentViewWidgets {
    let settings = babydra_core::load_explore_cfg();
    let activate_on_single = !settings.double_click_to_open;

    let scroll_win = ScrolledWindow::new();

    let stack = Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
    scroll_win.set_child(Some(&stack));

    // View Mode: Icons (FlowBox)
    let flowbox = FlowBox::new();
    flowbox.set_valign(Align::Start);
    flowbox.set_max_children_per_line(20);
    flowbox.set_min_children_per_line(1);
    flowbox.set_selection_mode(gtk4::SelectionMode::Multiple);
    flowbox.set_activate_on_single_click(activate_on_single);
    flowbox.set_row_spacing(10);
    flowbox.set_column_spacing(10);

    let grid_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    grid_container.set_valign(Align::Start);

    let grid_overlay = gtk4::Overlay::new();
    grid_overlay.set_child(Some(&grid_container));

    let grid_fixed = gtk4::Fixed::new();
    grid_fixed.set_can_target(false);
    grid_overlay.add_overlay(&grid_fixed);

    let grid_rubberband = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    grid_rubberband.add_css_class("rubberband");
    grid_rubberband.set_visible(false);
    grid_fixed.put(&grid_rubberband, 0.0, 0.0);

    let flow_scroll = ScrolledWindow::new();
    flow_scroll.set_child(Some(&grid_overlay));
    stack.add_named(&flow_scroll, Some("icons"));

    // View Mode: List (ListBox)
    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::Multiple);
    listbox.set_activate_on_single_click(activate_on_single);

    let list_overlay = gtk4::Overlay::new();
    list_overlay.set_child(Some(&listbox));

    let list_fixed = gtk4::Fixed::new();
    list_fixed.set_can_target(false);
    list_overlay.add_overlay(&list_fixed);

    let list_rubberband = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    list_rubberband.add_css_class("rubberband");
    list_rubberband.set_visible(false);
    list_fixed.put(&list_rubberband, 0.0, 0.0);

    let list_scroll = ScrolledWindow::new();
    list_scroll.set_child(Some(&list_overlay));
    stack.add_named(&list_scroll, Some("list"));

    let pane_nav_row = Box::new(Orientation::Horizontal, 4);
    pane_nav_row.set_css_classes(&["nav-bar"]);
    pane_nav_row.set_margin_start(6);
    pane_nav_row.set_margin_end(6);
    pane_nav_row.set_margin_bottom(4);
    pane_nav_row.set_margin_top(4);

    let btn_back = babydra_ui_kit::components::create_icon_button(
        "back",
        16,
        &["nav-btn"],
        Some(&trans("explore.back")),
        || {},
    );
    btn_back.set_size_request(28, 28);
    btn_back.set_hexpand(false);
    btn_back.set_vexpand(false);

    let btn_forward = babydra_ui_kit::components::create_icon_button(
        "forward",
        16,
        &["nav-btn"],
        Some(&trans("explore.forward")),
        || {},
    );
    btn_forward.set_size_request(28, 28);
    btn_forward.set_hexpand(false);
    btn_forward.set_vexpand(false);

    let btn_up = babydra_ui_kit::components::create_icon_button(
        "up",
        16,
        &["nav-btn"],
        Some(&trans("explore.up")),
        || {},
    );
    btn_up.set_size_request(28, 28);
    btn_up.set_hexpand(false);
    btn_up.set_vexpand(false);

    let btn_refresh = babydra_ui_kit::components::create_icon_button(
        "refresh",
        16,
        &["nav-btn"],
        Some(&trans("explore.refresh")),
        || {},
    );
    btn_refresh.set_size_request(28, 28);
    btn_refresh.set_hexpand(false);
    btn_refresh.set_vexpand(false);

    pane_nav_row.append(&btn_back);
    pane_nav_row.append(&btn_forward);
    pane_nav_row.append(&btn_up);
    pane_nav_row.append(&btn_refresh);

    let address_wrap = Box::new(Orientation::Horizontal, 0);
    address_wrap.set_css_classes(&["address-bar-wrap"]);
    address_wrap.set_hexpand(true);
    address_wrap.set_valign(Align::Center);

    let address_stack = Stack::new();
    address_stack.set_hexpand(true);

    let breadcrumb_box = Box::new(Orientation::Horizontal, 2);
    breadcrumb_box.set_valign(Align::Center);
    address_stack.add_named(&breadcrumb_box, Some("breadcrumbs"));

    let entry_address = Entry::new();
    entry_address.set_hexpand(true);
    entry_address.set_css_classes(&["address-entry"]);
    address_stack.add_named(&entry_address, Some("address"));

    address_wrap.append(&address_stack);
    pane_nav_row.append(&address_wrap);

    let search = Entry::builder()
        .placeholder_text(&trans("explore.search_placeholder"))
        .primary_icon_name("system-search-symbolic")
        .css_classes(vec!["search-entry".to_string()])
        .build();
    search.set_size_request(80, -1);
    search.set_hexpand(false);
    pane_nav_row.append(&search);

    // Bottom progress bar for loading
    let progress_bar = gtk4::ProgressBar::builder()
        .visible(false)
        .css_classes(vec!["content-loading-progress".to_string()])
        .build();

    let container = Box::new(Orientation::Vertical, 0);
    container.set_css_classes(&["content-view"]);
    scroll_win.set_vexpand(true);
    container.append(&pane_nav_row);
    container.append(&scroll_win);
    container.append(&progress_bar);

    ContentViewWidgets {
        container,
        flowbox,
        listbox,
        grid_container,
        stack,
        grid_fixed,
        grid_rubberband,
        list_fixed,
        list_rubberband,
        progress_bar,
        btn_back,
        btn_forward,
        btn_up,
        btn_refresh,
        breadcrumb_box,
        entry_address,
        address_stack,
        address_wrap,
        search,
    }
}
use crate::widgets::state::ContentViewHandle;

/// Update content view ui silent.
pub fn render_silent(handle: &ContentViewHandle) {
    update_content_internal(handle, true);
}

/// Update content view ui.
pub fn update_content_ui(handle: &ContentViewHandle) {
    update_content_internal(handle, false);
}

/// Update content view ui internal.
fn update_content_internal(handle: &ContentViewHandle, silent: bool) {
    let widgets = handle.widgets.clone();
    let entries = handle.entries.borrow().clone();
    let nav_callback = handle.nav_callback.clone();
    let current_path = handle.current_path.borrow().clone();
    let start_path = current_path.clone();
    let current_mode = handle.current_mode.borrow().clone();
    let sort_mode = handle.sort_mode.borrow().clone();
    let selected_paths = handle.selected_paths.clone();
    let handle_c = handle.clone();

    // Increment and capture the render generation
    let gen = {
        let mut g = handle.render_generation.borrow_mut();
        *g += 1;
        *g
    };

    if !silent {
        widgets.progress_bar.set_visible(true);
        widgets.progress_bar.set_fraction(0.0);
    }

    while let Some(child) = widgets.grid_container.first_child() {
        widgets.grid_container.remove(&child);
    }

    while let Some(child) = widgets.listbox.first_child() {
        widgets.listbox.remove(&child);
    }

    glib::spawn_future_local(async move {
        if current_mode == "icons" {
            if sort_mode == "auto" {
                super::grid_renderer::render_flat_grid(
                    &handle_c,
                    &widgets,
                    &entries,
                    &current_path,
                    &start_path,
                    gen,
                    &nav_callback,
                    selected_paths,
                )
                .await;
            } else {
                super::grid_renderer::render_grouped_grid(
                    &handle_c,
                    &widgets,
                    &entries,
                    &current_path,
                    &start_path,
                    gen,
                    &sort_mode,
                    &nav_callback,
                    selected_paths,
                )
                .await;
            }
        } else {
            super::list_renderer::render_list_view(
                &handle_c,
                &widgets,
                &entries,
                &current_path,
                &start_path,
                gen,
                &sort_mode,
                &nav_callback,
                selected_paths,
            )
            .await;
        }

        // Hide progress bar when layout completes successfully
        if *handle_c.current_path.borrow() == start_path
            && *handle_c.render_generation.borrow() == gen
        {
            handle_c.widgets.progress_bar.set_visible(false);
        }
    });
}
