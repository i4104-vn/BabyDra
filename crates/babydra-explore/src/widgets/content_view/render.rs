use gtk4::prelude::*;
use gtk4::{ScrolledWindow, FlowBox, ListBox, Stack, Align, Box, Orientation, Label};
use babydra_common::ContentViewWidgets;

/// Builds the Content Area UI (FlowBox grid layout and ListBox list layout wrapped in a Stack).
pub fn build_content_view_ui() -> ContentViewWidgets {
    let container = ScrolledWindow::new();
    container.set_css_classes(&["content-view"]);

    let stack = Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
    container.set_child(Some(&stack));

    // View Mode: Icons (FlowBox)
    let flowbox = FlowBox::new();
    flowbox.set_valign(Align::Start);
    flowbox.set_max_children_per_line(20);
    flowbox.set_min_children_per_line(1);
    flowbox.set_selection_mode(gtk4::SelectionMode::Multiple);
    flowbox.set_activate_on_single_click(false);
    flowbox.set_row_spacing(10);
    flowbox.set_column_spacing(10);

    let flow_scroll = ScrolledWindow::new();
    flow_scroll.set_child(Some(&flowbox));
    stack.add_named(&flow_scroll, Some("icons"));

    // View Mode: List (ListBox) with column header
    let list_vbox = Box::new(Orientation::Vertical, 0);

    // Column header row
    let list_header = Box::new(Orientation::Horizontal, 0);
    list_header.set_css_classes(&["list-header-row"]);

    let hdr_name = Label::builder()
        .label("Name")
        .halign(Align::Start)
        .hexpand(true)
        .build();
    hdr_name.set_css_classes(&["list-header-cell"]);

    let hdr_sort_icon = Label::builder()
        .label("↑")
        .halign(Align::Start)
        .build();
    hdr_sort_icon.set_css_classes(&["list-header-sort-icon"]);

    let hdr_name_box = Box::new(Orientation::Horizontal, 4);
    hdr_name_box.set_hexpand(true);
    hdr_name_box.set_halign(Align::Fill);
    hdr_name_box.append(&hdr_name);
    hdr_name_box.append(&hdr_sort_icon);

    let hdr_date = Label::builder()
        .label("Date modified")
        .halign(Align::End)
        .build();
    hdr_date.set_css_classes(&["list-header-cell"]);
    hdr_date.set_size_request(140, -1);

    let hdr_type = Label::builder()
        .label("Type")
        .halign(Align::End)
        .build();
    hdr_type.set_css_classes(&["list-header-cell"]);
    hdr_type.set_size_request(80, -1);

    list_header.append(&hdr_name_box);
    list_header.append(&hdr_date);
    list_header.append(&hdr_type);

    list_vbox.append(&list_header);

    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::Multiple);
    listbox.set_activate_on_single_click(false);

    let list_scroll = ScrolledWindow::new();
    list_scroll.set_child(Some(&listbox));
    list_scroll.set_vexpand(true);
    list_vbox.append(&list_scroll);

    stack.add_named(&list_vbox, Some("list"));

    ContentViewWidgets {
        container,
        flowbox,
        listbox,
        list_header,
        stack,
    }
}

