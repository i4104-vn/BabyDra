use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Button, Window, Align};
use babydra_common::i18n::t;

pub fn show_settings_dialog(parent: &gtk4::Window, on_change_callback: impl Fn() + 'static) {
    let window = Window::builder()
        .title(&t("explore.settings"))
        .transient_for(parent)
        .modal(true)
        .resizable(true)
        .default_width(500)
        .default_height(400)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let lbl_title = Label::builder()
        .label(&t("explore.settings"))
        .halign(Align::Start)
        .build();
    lbl_title.add_css_class("settings-title-label");
    vbox.append(&lbl_title);

    let lbl_content = Label::builder()
        .label("Settings content goes here (Phase 3 & 4)")
        .halign(Align::Center)
        .valign(Align::Center)
        .vexpand(true)
        .build();
    vbox.append(&lbl_content);

    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let close_text = if babydra_common::i18n::get_locale() == "vi" { "Đóng" } else { "Close" };
    let btn_close = Button::builder()
        .label(close_text)
        .css_classes(vec!["suggested-action".to_string()])
        .build();
    bbox.append(&btn_close);

    let win_c = window.clone();
    btn_close.connect_clicked(move |_| {
        win_c.close();
    });

    // Dummy call to on_change_callback to make Rust compile it for now
    let on_change = std::rc::Rc::new(on_change_callback);
    let _ = on_change.clone();

    window.present();
}
