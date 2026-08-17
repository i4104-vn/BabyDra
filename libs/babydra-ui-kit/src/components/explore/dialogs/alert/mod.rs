use gtk4::prelude::*;
use std::rc::Rc;

mod render;

/// Presents a simple error/alert modal dialog with a Close/OK button.
pub fn show_alert_dialog(title: &str, message: &str, parent: Option<&impl IsA<gtk4::Window>>) {
    let widgets = render::build_alert_dialog_ui(title, message, parent);
    let window = widgets.window;
    let vbox = widgets.vbox;

    let win_ok = window.clone();
    widgets.btn_ok.connect_clicked(move |_| {
        win_ok.close();
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
        crate::ui::animation::genie_out(vbox_cancel.upcast_ref(), 340, 130, 300, move || {
            win_cb.destroy();
        });
        glib::Propagation::Stop
    });

    window.present();
    crate::ui::animation::genie_in(vbox.upcast_ref(), 340, 130, 300);
}
