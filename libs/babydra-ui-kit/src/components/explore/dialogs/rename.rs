use babydra_core::i18n::trans;
use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

use super::shell::DialogShell;

/// Presents a dialog window to rename a target file or folder.
pub fn show_rename_dialog(
    path: &std::path::Path,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&impl IsA<gtk4::Window>>,
) {
    let is_dir = path.is_dir();
    let initial_name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
    let initial_icon = if is_dir {
        "folder".to_string()
    } else {
        crate::ui::icon::get_icon_name_for_file(&initial_name, false)
    };

    let shell = DialogShell::new(&trans("explore.dialog_rename_title"), 380, 185, 12, parent);
    let icon_img = shell.add_header(
        &initial_icon,
        super::shell::BadgeStyle::Primary,
        &trans("explore.dialog_rename_title"),
        Some(&trans("explore.dialog_rename_label")),
    );
    let entry = shell.add_entry(
        Some(&initial_name),
        false,
    );
    let lbl_error = shell.add_error_label();
    let bbox = shell.add_button_row();
    shell.cancel_button(&bbox);
    let btn_rename = shell.action_button(&bbox, &trans("explore.menu_rename"));

    let icon_img_c = icon_img.clone();
    entry.connect_changed(move |e| {
        let text = e.text().to_string();
        crate::ui::icon::set_file_icon(&icon_img_c, &text, is_dir);
    });

    let win = shell.window.clone();
    let path_owned = path.to_path_buf();
    DialogShell::wire_error_clear(&lbl_error, &entry);

    let entry_clicked = entry.clone();
    btn_rename.connect_clicked(move |_| {
        let new_name = entry_clicked.text().to_string();
        if new_name.is_empty() {
            return;
        }
        let target_dest = path_owned
            .parent()
            .map(|p| p.join(&new_name))
            .unwrap_or_else(|| PathBuf::from(&new_name));
        let old_name = path_owned.file_name().unwrap_or_default().to_string_lossy();

        if new_name != old_name && target_dest.exists() {
            DialogShell::show_error(
                &lbl_error,
                &entry_clicked,
                &trans("explore.error_item_exists"),
            );
        } else {
            let path_c = path_owned.clone();
            let nav_c = nav_callback.clone();
            let cp_c = current_path.clone();
            glib::spawn_future_local(async move {
                if let Err(e) = babydra_core::rename_path(path_c, new_name).await {
                    eprintln!("Rename failed: {}", e);
                }
                nav_c(cp_c);
            });
            win.close();
        }
    });

    DialogShell::finish(shell, &entry, &btn_rename);
}
