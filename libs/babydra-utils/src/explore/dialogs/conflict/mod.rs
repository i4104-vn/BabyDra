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
    let _vbox = widgets.vbox;

    let win_cancel_btn = window.clone();
    widgets.btn_cancel.connect_clicked(move |_| {
        win_cancel_btn.close();
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
}
