use gtk4::prelude::*;
use gtk4::{ScrolledWindow, FlowBox, ListBox, Stack, Align};
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
    flowbox.set_row_spacing(10);
    flowbox.set_column_spacing(10);

    let flow_scroll = ScrolledWindow::new();
    flow_scroll.set_child(Some(&flowbox));
    stack.add_named(&flow_scroll, Some("icons"));

    // View Mode: List (ListBox)
    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::Multiple);

    let list_scroll = ScrolledWindow::new();
    list_scroll.set_child(Some(&listbox));
    stack.add_named(&list_scroll, Some("list"));

    ContentViewWidgets {
        container,
        flowbox,
        listbox,
        stack,
    }
}
