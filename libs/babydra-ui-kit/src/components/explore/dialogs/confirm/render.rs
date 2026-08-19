use gtk4::prelude::*;
use gtk4::{Align, Box, Button, Label, Orientation, Window};

pub struct ConfirmDialogWidgets {
    pub window: Window,
    pub vbox: Box,
    pub btn_cancel: Button,
    pub btn_confirm: Button,
}

/// Build confirm dialog ui.
pub fn build_confirm_dialog(
    title: &str,
    message: &str,
    parent: Option<&impl IsA<gtk4::Window>>,
) -> ConfirmDialogWidgets {
    let window = Window::builder()
        .title(title)
        .icon_name("babydra")
        .modal(true)
        .resizable(false)
        .default_width(360)
        .default_height(120)
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

    let lbl = Label::builder()
        .label(message)
        .halign(Align::Start)
        .wrap(true)
        .build();
    vbox.append(&lbl);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&babydra_core::i18n::trans("explore.settings_cancel"));
    let btn_confirm = Button::builder()
        .label(&babydra_core::i18n::trans("explore.settings_delete"))
        .css_classes(vec!["destructive-action".to_string()])
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_confirm);

    ConfirmDialogWidgets {
        window,
        vbox,
        btn_cancel,
        btn_confirm,
    }
}
