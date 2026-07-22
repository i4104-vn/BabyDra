use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

mod render;

/// Presents a dialog window to rename a target file or folder.
pub fn show_rename_dialog(
    path: &PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    let widgets = render::build_rename_dialog_ui(path, parent);
    let window = widgets.window;
    let vbox = widgets.vbox;
    let entry = widgets.entry;
    let btn_rename = widgets.btn_rename;

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

    let win_rename = window.clone();
    let path = path.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    let entry_c = entry.clone();
    btn_rename.connect_clicked(move |_| {
        let new_name = entry_c.text().to_string();
        if !new_name.is_empty() {
            let target_dest = path.parent().map(|p| p.join(&new_name)).unwrap_or_else(|| PathBuf::from(&new_name));
            let path_c = path.clone();
            let new_name_c = new_name.clone();
            let nav_c = nav.clone();
            let cp_c = current_p.clone();

            let do_rename = move || {
                let path_inner = path_c.clone();
                let new_name_inner = new_name_c.clone();
                let nav_inner = nav_c.clone();
                let cp_inner = cp_c.clone();
                glib::spawn_future_local(async move {
                    if let Err(e) = babydra_common::rename_path(path_inner, new_name_inner).await {
                        eprintln!("Rename failed: {}", e);
                    }
                    nav_inner(cp_inner);
                });
            };

            let old_name = path.file_name().unwrap_or_default().to_string_lossy();
            if new_name != old_name && target_dest.exists() {
                let msg = format!("Mục \"{}\" đã tồn tại tại vị trí này. Vui lòng chọn tên khác.", new_name);
                super::alert::show_alert_dialog("Mục đã tồn tại", &msg, Some(&win_rename));
            } else {
                do_rename();
                win_rename.close();
            }
        }
    });

    let entry_trigger = entry.clone();
    entry.connect_activate(move |_| {
        btn_rename.emit_clicked();
    });

    window.present();
    crate::ui::animation::genie_in(vbox.upcast_ref(), 320, 140, 300);
    entry_trigger.grab_focus();
}
