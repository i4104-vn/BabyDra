use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box, Button, Label, Orientation, Window};

pub struct ConflictDialogWidgets {
    pub window: Window,
    pub vbox: Box,
    pub btn_cancel: Button,
    pub btn_override: Button,
}

/// Build conflict dialog ui.
pub fn build_conflict(
    item_name: &str,
    parent: Option<&impl IsA<gtk4::Window>>,
) -> ConflictDialogWidgets {
    let title = trans("explore.dialog_conflict_title");
    let window = Window::builder()
        .title(&title)
        .icon_name("babydra")
        .modal(true)
        .resizable(false)
        .default_width(380)
        .default_height(140)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    if let Some(p) = parent {
        window.set_transient_for(Some(p));
    }

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.add_css_class("explore-dialog-box");
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let msg_template = trans("explore.dialog_conflict_msg");
    let msg = msg_template.replace("{}", item_name);
    let lbl = Label::builder()
        .label(&msg)
        .halign(Align::Start)
        .wrap(true)
        .max_width_chars(45)
        .build();
    vbox.append(&lbl);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&trans("explore.settings_cancel"));
    let btn_override = Button::builder()
        .label(&trans("explore.dialog_override"))
        .css_classes(vec![
            "suggested-action".to_string(),
            "destructive-action".to_string(),
        ])
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_override);

    ConflictDialogWidgets {
        window,
        vbox,
        btn_cancel,
        btn_override,
    }
}
