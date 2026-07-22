use gtk4::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

mod render;

/// Presents a conflict dialog informing the user that a target file or folder already exists.
/// Offers options to Cancel or Override (Replace).
pub fn show_conflict_dialog(
    item_name: &str,
    on_override: impl FnOnce() + 'static,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    let widgets = render::build_conflict_dialog_ui(item_name, parent);
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
            380,
            140,
            300,
            move || {
                win_cb.destroy();
            }
        );
        glib::Propagation::Stop
    });

    let win_override = window.clone();
    let override_cell = Rc::new(RefCell::new(Some(on_override)));
    widgets.btn_override.connect_clicked(move |_| {
        if let Some(cb) = override_cell.borrow_mut().take() {
            cb();
        }
        win_override.close();
    });

    window.present();
    crate::ui::animation::genie_in(vbox.upcast_ref(), 380, 140, 300);
}
