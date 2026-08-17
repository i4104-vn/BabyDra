use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Button, Align, Window};
use babydra_common::i18n::t;

pub struct AlertDialogWidgets {
    pub window: Window,
    pub vbox: Box,
    pub btn_ok: Button,
}

pub fn build_alert_dialog_ui(
    title: &str,
    message: &str,
    parent: Option<&impl IsA<gtk4::Window>>,
) -> AlertDialogWidgets {
    let window = Window::builder()
        .title(title)
        .modal(true)
        .resizable(false)
        .default_width(340)
        .default_height(130)
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
        .max_width_chars(40)
        .build();
    vbox.append(&lbl);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_ok = Button::builder()
        .label(&t("explore.settings_close"))
        .css_classes(vec!["suggested-action".to_string()])
        .build();

    bbox.append(&btn_ok);

    AlertDialogWidgets {
        window,
        vbox,
        btn_ok,
    }
}
