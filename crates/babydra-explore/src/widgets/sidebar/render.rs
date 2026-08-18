use gtk4::prelude::*;
use gtk4::{Align, Box, Label, Orientation, ScrolledWindow, Separator};

/// Builds the scroll container and layout box for the sidebar.
pub fn build_sidebar_ui() -> (ScrolledWindow, Box) {
    let container = ScrolledWindow::new();
    container.set_hscrollbar_policy(gtk4::PolicyType::Never);
    container.set_css_classes(&["sidebar"]);
    container.set_width_request(200);
    container.set_hexpand(false);
    container.set_vexpand(true);

    let vbox = Box::new(Orientation::Vertical, 0);
    container.set_child(Some(&vbox));

    (container, vbox)
}

/// Helper function to create standard section titles.
pub fn create_section_title(label: &str) -> Label {
    let title = Label::new(Some(label));
    title.set_css_classes(&["sidebar-section-label"]);
    title.set_halign(Align::Start);
    title
}

/// Helper to build a horizontal separator.
pub fn create_sidebar_sep() -> Separator {
    let sep = Separator::new(Orientation::Horizontal);
    sep.set_margin_top(8);
    sep.set_margin_bottom(4);
    sep.set_margin_start(12);
    sep.set_margin_end(12);
    sep
}
