use crate::widgets::state::PreviewPanelWidgets;
use gtk4::prelude::*;
use gtk4::{Align, Box, Label, Orientation, ScrolledWindow};
use std::cell::RefCell;
use std::rc::Rc;

/// Creates a new `preview panel`.
pub fn create_preview_panel() -> (ScrolledWindow, PreviewPanelWidgets) {
    let container = ScrolledWindow::new();
    container.set_css_classes(&["preview-panel"]);
    container.set_hexpand(true);
    container.set_vexpand(true);

    let vbox = Box::new(Orientation::Vertical, 6);
    vbox.set_hexpand(true);
    vbox.set_vexpand(true);
    container.set_child(Some(&vbox));

    let lbl_status = Label::new(Some(&babydra_core::i18n::t("explore.preview_no_selection")));
    lbl_status.set_css_classes(&["dim-label"]);
    lbl_status.set_margin_bottom(6);
    vbox.append(&lbl_status);

    // Scrolled inner container for the label text content
    let scroll_content = ScrolledWindow::new();
    scroll_content.set_hexpand(true);
    scroll_content.set_vexpand(true);
    vbox.append(&scroll_content);

    let lbl_content = Label::builder()
        .use_markup(true)
        .wrap(true)
        .halign(Align::Start)
        .valign(Align::Start)
        .selectable(true)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(6)
        .margin_end(6)
        .build();
    scroll_content.set_child(Some(&lbl_content));

    let widgets = PreviewPanelWidgets {
        container: container.clone(),
        lbl_content,
        lbl_status,
        current_file: Rc::new(RefCell::new(None)),
        watcher: Rc::new(RefCell::new(None)),
    };

    (container, widgets)
}
