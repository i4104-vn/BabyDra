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
    let vbox = widgets.vbox;

    let win_cancel_btn = window.clone();
    widgets.btn_cancel.connect_clicked(move |_| {
        win_cancel_btn.close();
    });

    let win_cancel = window.clone();
    let vbox_cancel = vbox.clone();
    let is_animating = Rc::new(std::cell::Cell::new(false));
    let is_animating_cancel = is_animating.clone();
    window.connect_close_request(move |_| {
        if is_animating_cancel.get() {
            return glib::Propagation::Stop;
        }
        is_animating_cancel.set(true);
        let win_cb = win_cancel.clone();
        crate::ui::animation::genie_out(
            vbox_cancel.upcast_ref(),
            360,
            120,
            300,
            move || {
                win_cb.destroy();
            }
        );
        glib::Propagation::Stop
    });

    let win_confirm = window.clone();
    let confirm_cb = Rc::new(on_confirm);
    widgets.btn_confirm.connect_clicked(move |_| {
        confirm_cb();
        win_confirm.close();
    });

    window.present();
    crate::ui::animation::genie_in(vbox.upcast_ref(), 360, 120, 300);
}
