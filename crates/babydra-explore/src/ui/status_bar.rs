use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align};

pub struct StatusBar {
    container: Box,
    lbl_status: Label,
}

impl StatusBar {
    pub fn new() -> Self {
        let container = Box::new(Orientation::Horizontal, 0);
        container.set_css_classes(&["status-bar"]);
        container.set_margin_top(4);
        container.set_margin_bottom(4);
        container.set_margin_start(10);
        container.set_margin_end(10);

        let lbl_status = Label::builder()
            .label("0 items")
            .halign(Align::Start)
            .build();

        container.append(&lbl_status);

        Self {
            container,
            lbl_status,
        }
    }

    pub fn widget(&self) -> &Box {
        &self.container
    }

    pub fn update(&self, count: usize, total_size: u64) {
        let size_str = baby_utils::explore_helpers::format_size(total_size);
        self.lbl_status.set_text(&format!("{} items | Total size: {}", count, size_str));
    }
}
