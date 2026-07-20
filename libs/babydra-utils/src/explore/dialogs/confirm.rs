use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Button, Align, Window};
use std::rc::Rc;

/// Presents a delete confirmation dialog. Calls `on_confirm` if the user clicks "Delete".
pub fn show_delete_confirm_dialog(
    title: &str,
    message: &str,
    on_confirm: impl Fn() + 'static,
) {
    let window = Window::builder()
        .title(title)
        .modal(true)
        .resizable(false)
        .default_width(360)
        .default_height(120)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);
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

    let btn_cancel = Button::with_label("Cancel");
    let btn_confirm = Button::builder()
        .label("Delete")
        .css_classes(vec!["destructive-action".to_string()])
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_confirm);

    let win_cancel = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel.close();
    });

    let win_confirm = window.clone();
    let confirm_cb = Rc::new(on_confirm);
    btn_confirm.connect_clicked(move |_| {
        confirm_cb();
        win_confirm.close();
    });

    window.present();
}
