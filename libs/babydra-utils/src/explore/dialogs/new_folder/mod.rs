use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

mod render;

/// Presents a dialog window to create a new folder under a directory.
pub fn show_new_folder_dialog(
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    let widgets = render::build_new_folder_dialog_ui(parent);
    let window = widgets.window;
    let vbox = widgets.vbox;
    let entry = widgets.entry;
    let btn_create = widgets.btn_create;

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
            320,
            140,
            300,
            move || {
                win_cb.destroy();
            }
        );
        glib::Propagation::Stop
    });

    let win_create = window.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    let entry_c = entry.clone();
    btn_create.connect_clicked(move |_| {
        let name = entry_c.text().to_string();
        if !name.is_empty() {
            let folder_path = current_p.join(name);
            let nav_c = nav.clone();
            let cp_c = current_p.clone();
            glib::spawn_future_local(async move {
                let _ = tokio::fs::create_dir_all(folder_path).await;
                nav_c(cp_c);
            });
        }
        win_create.close();
    });

    let entry_trigger = entry.clone();
    entry.connect_activate(move |_| {
        btn_create.emit_clicked();
    });

    window.present();
    crate::ui::animation::genie_in(vbox.upcast_ref(), 320, 140, 300);
    entry_trigger.grab_focus();
}
