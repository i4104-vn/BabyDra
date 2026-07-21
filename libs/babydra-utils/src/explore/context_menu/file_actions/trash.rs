use std::path::PathBuf;
use std::rc::Rc;
use gtk4::prelude::*;
use crate::explore::context_menu::widgets::create_menu_button;
use crate::explore::helpers::restore_from_trash;

use babydra_common::i18n::t;

/// Renders the context menu for files/directories inside the Trash (Restore, Delete Permanently).
pub fn show_for_file_trash(
    popover: &gtk4::Popover,
    vbox: &gtk4::Box,
    target_paths: Vec<PathBuf>,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    let btn_restore = create_menu_button(&t("explore.menu_restore"), "restart");
    let btn_delete = create_menu_button(&t("explore.menu_delete_perm"), "trash");

    vbox.append(&btn_restore);
    vbox.append(&btn_delete);

    // Restore action
    let pop_c = popover.clone();
    let target_paths_restore = target_paths.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    btn_restore.connect_clicked(move |_| {
        pop_c.popdown();
        let nav_c = nav.clone();
        let cp_c = current_p.clone();
        let paths_c = target_paths_restore.clone();
        glib::spawn_future_local(async move {
            for path in paths_c {
                if let Err(err) = restore_from_trash(path).await {
                    eprintln!("Failed to restore file: {}", err);
                }
            }
            nav_c(cp_c);
        });
    });

    // Permanent delete
    let pop_c = popover.clone();
    let target_paths_del = target_paths.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    btn_delete.connect_clicked(move |_| {
        pop_c.popdown();
        let nav_c = nav.clone();
        let cp_c = current_p.clone();
        let paths_c = target_paths_del.clone();
        
        let message = if paths_c.len() == 1 {
            t("explore.dialog_confirm_delete_single").replace("{}", &paths_c[0].file_name().unwrap().to_string_lossy())
        } else {
            t("explore.dialog_confirm_delete_multi").replace("{}", &paths_c.len().to_string())
        };
        
        crate::explore::dialogs::show_delete_confirm_dialog(
            &t("explore.dialog_confirm_delete_title"),
            &message,
            move || {
                let nav_f = nav_c.clone();
                let cp_f = cp_c.clone();
                let paths_f = paths_c.clone();
                glib::spawn_future_local(async move {
                    for path in paths_f {
                        if let Err(err) = babydra_common::delete_path(path).await {
                            eprintln!("Failed to delete file: {}", err);
                        }
                    }
                    nav_f(cp_f);
                });
            }
        );
    });

    popover.popup();
}
