use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align};
use babydra_common::i18n::t;

#[derive(Clone)]
pub struct StatusBarWidgets {
    pub container: Box,
    pub lbl_status: Label,
    pub btn_toggle_hidden: gtk4::Button,
    pub btn_toggle_preview: gtk4::Button,
}

/// Creates the status bar widgets and returns a StatusBarWidgets struct.
pub fn create_status_bar() -> StatusBarWidgets {
    let container = Box::new(Orientation::Horizontal, 8);
    container.set_css_classes(&["status-bar"]);

    let lbl_status = Label::builder()
        .label(&format!("0 {}", t("explore.items")))
        .halign(Align::Start)
        .hexpand(true)
        .build();
    container.append(&lbl_status);

    let btn_toggle_hidden = babydra_utils::components::create_icon_button("eye-off", 16, &["status-bar-btn"], Some(&t("explore.toggle_hidden")), || {});
    container.append(&btn_toggle_hidden);

    let btn_toggle_preview = babydra_utils::components::create_icon_button("sidebar", 16, &["status-bar-btn", "status-bar-btn-active"], Some(&t("explore.toggle_preview")), || {});
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
    let size_str = babydra_utils::explore::format_size(total_size);
    lbl_status.set_text(&format!(
        "{} {} | {}: {}",
        count,
        t("explore.items"),
        t("explore.total_size"),
        size_str
    ));
}
