use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box, Button, Entry, Label, Orientation, Window};

pub struct NewFileDialogWidgets {
    pub window: Window,
    pub vbox: Box,
    pub entry: Entry,
    pub lbl_error: Label,
    pub btn_cancel: Button,
    pub btn_create: Button,
}

/// Build new file dialog ui.
pub fn build_file_dialog(parent: Option<&impl IsA<gtk4::Window>>) -> NewFileDialogWidgets {
    let window = Window::builder()
        .title(&trans("explore.dialog_new_file_title"))
        .modal(true)
        .resizable(false)
        .default_width(320)
        .default_height(150)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    if let Some(p) = parent {
        window.set_transient_for(Some(p));
    }

    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.add_css_class("explore-dialog-box");
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let lbl = Label::builder()
        .label(&trans("explore.dialog_new_file_label"))
        .halign(Align::Start)
        .build();
    vbox.append(&lbl);

    let entry = Entry::new();
    entry.set_text(&trans("explore.menu_new_file"));
    entry.set_hexpand(true);
    vbox.append(&entry);

    let lbl_error = Label::builder()
        .halign(Align::Start)
        .visible(false)
        .css_classes(vec!["dialog-error-text".to_string()])
        .wrap(true)
        .max_width_chars(35)
        .build();
    vbox.append(&lbl_error);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_cancel = Button::with_label(&trans("explore.settings_cancel"));
    let btn_create = Button::builder()
        .label(&trans("explore.settings_add"))
        .css_classes(vec!["suggested-action".to_string()])
        .build();

    bbox.append(&btn_cancel);
    bbox.append(&btn_create);

    NewFileDialogWidgets {
        window,
        vbox,
        entry,
        lbl_error,
        btn_cancel,
        btn_create,
    }
}
