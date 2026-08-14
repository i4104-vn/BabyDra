use gtk4::prelude::*;
use std::rc::Rc;

mod render;

/// Presents a delete confirmation dialog. Calls `on_confirm` if the user clicks "Delete".
pub fn show_delete_confirm_dialog(
    title: &str,
    message: &str,
    on_confirm: impl Fn() + 'static,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    let widgets = render::build_confirm_dialog_ui(title, message, parent);
    let window = widgets.window;
    let _vbox = widgets.vbox;

    let win_cancel_btn = window.clone();
    widgets.btn_cancel.connect_clicked(move |_| {
        win_cancel_btn.close();
    });

    let win_confirm = window.clone();
    let confirm_cb = Rc::new(on_confirm);
    widgets.btn_confirm.connect_clicked(move |_| {
        confirm_cb();
        win_confirm.close();
    });

    window.present();
}
