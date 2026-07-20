use gtk4::prelude::*;
use gtk4::{ScrolledWindow, FlowBox, ListBox, Stack, Align};
use babydra_common::ContentViewWidgets;

pub fn build_content_view_ui() -> ContentViewWidgets {
    let settings = babydra_common::load_explore_settings();
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

    // Bottom progress bar for loading
    let progress_bar = gtk4::ProgressBar::builder()
        .visible(false)
        .css_classes(vec!["content-loading-progress".to_string()])
        .build();

    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    container.set_css_classes(&["content-view"]);
    scroll_win.set_vexpand(true);
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
    }
}
