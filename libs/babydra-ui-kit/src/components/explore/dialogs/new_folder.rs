use babydra_core::i18n::trans;
use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

use super::shell::DialogShell;

/// Presents a dialog window to create a new folder under a directory.
pub fn show_folder_dialog(
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    let shell = DialogShell::new(
        &trans("explore.dialog_new_folder_title"),
        320,
        150,
        10,
        parent,
    );
    shell.add_label(&trans("explore.dialog_new_folder_label"));
    let entry = shell.add_entry(Some(&trans("explore.menu_new_folder")), false);
    let lbl_error = shell.add_error_label();
    let bbox = shell.add_button_row();
    shell.cancel_button(&bbox);
    let btn_create = shell.action_button(&bbox, &trans("explore.settings_add"));

    let win = shell.window.clone();
    DialogShell::wire_error_clear(&lbl_error, &entry);

    let entry_clicked = entry.clone();
    btn_create.connect_clicked(move |_| {
        let name = entry_clicked.text().to_string();
        if name.is_empty() {
            return;
        }
        let folder_path = current_path.join(&name);
        if folder_path.exists() {
            DialogShell::show_error(
                &lbl_error,
                &entry_clicked,
                &trans("explore.error_folder_exists"),
            );
        } else {
            let nav_c = nav_callback.clone();
            let cp_c = current_path.clone();
            glib::spawn_future_local(async move {
                let _ = tokio::fs::create_dir_all(folder_path).await;
                nav_c(cp_c);
            });
            win.close();
        }
    });

    DialogShell::finish(shell, &entry, &btn_create);
}
