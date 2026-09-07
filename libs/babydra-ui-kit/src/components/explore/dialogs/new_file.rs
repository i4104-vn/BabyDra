use babydra_core::i18n::trans;
use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

use super::shell::DialogShell;

/// Presents a dialog window to create a new empty file under a directory.
pub fn show_new_file_dialog(
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    let shell = DialogShell::new(
        &trans("explore.dialog_new_file_title"),
        380,
        185,
        12,
        parent,
    );
    let icon_img = shell.add_header(
        "text-x-generic",
        super::shell::BadgeStyle::Primary,
        &trans("explore.dialog_new_file_title"),
        Some(&trans("explore.dialog_new_file_label")),
    );
    let entry = shell.add_entry(Some(&trans("explore.menu_new_file")), false);
    let lbl_error = shell.add_error_label();
    let bbox = shell.add_button_row();
    shell.cancel_button(&bbox);
    let btn_create = shell.action_button(&bbox, &trans("explore.settings_add"));

    let icon_img_c = icon_img.clone();
    entry.connect_changed(move |e| {
        let text = e.text().to_string();
        crate::ui::icon::set_file_icon(&icon_img_c, &text, false);
    });

    let win = shell.window.clone();
    DialogShell::wire_error_clear(&lbl_error, &entry);

    let entry_clicked = entry.clone();
    btn_create.connect_clicked(move |_| {
        let name = entry_clicked.text().to_string();
        if name.is_empty() {
            return;
        }
        let file_path = current_path.join(&name);
        if file_path.exists() {
            DialogShell::show_error(
                &lbl_error,
                &entry_clicked,
                &trans("explore.error_file_exists"),
            );
        } else {
            let nav_c = nav_callback.clone();
            let cp_c = current_path.clone();
            glib::spawn_future_local(async move {
                let _ = tokio::fs::write(&file_path, "").await;
                nav_c(cp_c);
            });
            win.close();
        }
    });

    DialogShell::finish(shell, &entry, &btn_create);
}
