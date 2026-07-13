use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align};

/// Creates the status bar widgets and returns a tuple (container_box, lbl_status).
pub fn create_status_bar() -> (Box, Label) {
    let container = Box::new(Orientation::Horizontal, 0);
    container.set_css_classes(&["status-bar"]);

    let lbl_status = Label::builder()
        .label("0 items")
        .halign(Align::Start)
        .build();

    container.append(&lbl_status);

    (container, lbl_status)
}

/// Updates the status bar label content.
pub fn update_status_bar(lbl_status: &Label, count: usize, total_size: u64) {
    let size_str = baby_utils::explore_helpers::format_size(total_size);
    lbl_status.set_text(&format!("{} items | Total size: {}", count, size_str));
}
