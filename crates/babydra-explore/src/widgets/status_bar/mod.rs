use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, Button};

#[derive(Clone)]
pub struct StatusBarWidgets {
    pub container: Box,
    pub lbl_status: Label,
    pub btn_toggle_hidden: Button,
    pub btn_toggle_preview: Button,
}

/// Creates the status bar widgets and returns a StatusBarWidgets struct.
pub fn create_status_bar() -> StatusBarWidgets {
    let container = Box::new(Orientation::Horizontal, 8);
    container.set_css_classes(&["status-bar"]);

    let lbl_status = Label::builder()
        .label("0 items")
        .halign(Align::Start)
        .hexpand(true)
        .build();
    container.append(&lbl_status);

    let btn_toggle_hidden = Button::from_icon_name("view-conceal-symbolic");
    btn_toggle_hidden.set_css_classes(&["status-bar-btn"]);
    btn_toggle_hidden.set_tooltip_text(Some("Toggle Hidden Files (Ctrl+H)"));
    container.append(&btn_toggle_hidden);

    let btn_toggle_preview = Button::from_icon_name("view-sidebar-symbolic");
    btn_toggle_preview.set_css_classes(&["status-bar-btn", "status-bar-btn-active"]);
    btn_toggle_preview.set_tooltip_text(Some("Toggle Preview (F4)"));
    container.append(&btn_toggle_preview);

    StatusBarWidgets {
        container,
        lbl_status,
        btn_toggle_hidden,
        btn_toggle_preview,
    }
}

/// Updates the status bar label content.
pub fn update_status_bar(lbl_status: &Label, count: usize, total_size: u64) {
    let size_str = baby_utils::explore_helpers::format_size(total_size);
    lbl_status.set_text(&format!("{} items | Total size: {}", count, size_str));
}
