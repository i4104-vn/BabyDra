use std::path::PathBuf;
use std::rc::Rc;
use gtk4::prelude::*;
use super::{CLIPBOARD, create_menu_popover, create_menu_button};

pub fn show_for_file(
    parent: &gtk4::Widget,
    x: f64,
    y: f64,
    target_paths: Vec<PathBuf>,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    let (popover, vbox) = create_menu_popover(parent, x, y);

    // Create buttons
    let btn_open = create_menu_button("Open", "document-open");
    let btn_cut = create_menu_button("Cut", "edit-cut");
    let btn_copy = create_menu_button("Copy", "edit-copy");
    let btn_rename = create_menu_button("Rename", "edit-clear"); // standard rename fallback
    let btn_trash = create_menu_button("Move to Trash", "user-trash");
    let btn_delete = create_menu_button("Delete Permanently", "edit-delete");

    vbox.append(&btn_open);
    vbox.append(&btn_cut);
    vbox.append(&btn_copy);
    if target_paths.len() == 1 {
        vbox.append(&btn_rename);
    }
    vbox.append(&btn_trash);
    vbox.append(&btn_delete);

    // Event handling
    let pop_c = popover.clone();
    let target_paths_open = target_paths.clone();
    let nav = nav_callback.clone();
    btn_open.connect_clicked(move |_| {
        pop_c.popdown();
        for path in &target_paths_open {
            if path.is_dir() {
                nav(path.clone());
            } else {
                let uri = format!("file://{}", path.to_string_lossy());
                let _ = gtk4::gio::AppInfo::launch_default_for_uri(&uri, gtk4::gio::AppLaunchContext::NONE);
            }
        }
    });

    let pop_c = popover.clone();
    let target_paths_copy = target_paths.clone();
    let nav_c = nav_callback.clone();
    let current_p = current_path.clone();
    btn_copy.connect_clicked(move |_| {
        pop_c.popdown();
        CLIPBOARD.with(|cb| {
            cb.replace(Some((target_paths_copy.clone(), false)));
        });
        nav_c(current_p.clone());
    });

    let pop_c = popover.clone();
    let target_paths_cut = target_paths.clone();
    let nav_c = nav_callback.clone();
    let current_p = current_path.clone();
    btn_cut.connect_clicked(move |_| {
        pop_c.popdown();
        CLIPBOARD.with(|cb| {
            cb.replace(Some((target_paths_cut.clone(), true)));
        });
        nav_c(current_p.clone());
    });

    // Rename dialog trigger (only if 1 item selected)
    if target_paths.len() == 1 {
        let pop_c = popover.clone();
        let rename_path = target_paths[0].clone();
        let nav = nav_callback.clone();
        let current_p = current_path.clone();
        btn_rename.connect_clicked(move |_| {
            pop_c.popdown();
            crate::explore::dialogs::show_rename_dialog(&rename_path, current_p.clone(), nav.clone());
        });
    }

    // Trash action
    let pop_c = popover.clone();
    let target_paths_trash = target_paths.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    btn_trash.connect_clicked(move |_| {
        pop_c.popdown();
        let nav_c = nav.clone();
        let cp_c = current_p.clone();
        let paths_c = target_paths_trash.clone();
        glib::spawn_future_local(async move {
            for path in paths_c {
                if let Err(err) = babydra_common::send_to_trash(path).await {
                    eprintln!("Failed to trash file: {}", err);
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
        glib::spawn_future_local(async move {
            for path in paths_c {
                if let Err(err) = babydra_common::delete_path(path).await {
                    eprintln!("Failed to delete file: {}", err);
                }
            }
            nav_c(cp_c);
        });
    });

    popover.popup();
}

pub fn show_for_empty(
    parent: &gtk4::Widget,
    x: f64,
    y: f64,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    let (popover, vbox) = create_menu_popover(parent, x, y);

    let btn_new_folder = create_menu_button("New Folder", "folder-new");
    let btn_paste = create_menu_button("Paste", "edit-paste");

    vbox.append(&btn_new_folder);
    vbox.append(&btn_paste);

    // Check clipboard state for paste sensitivity
    let clipboard_data = CLIPBOARD.with(|cb| cb.borrow().clone());
    btn_paste.set_sensitive(clipboard_data.is_some());

    // Paste action implementation
    let pop_c = popover.clone();
    let dest_dir = current_path.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    btn_paste.connect_clicked(move |_| {
        pop_c.popdown();
        if let Some((sources, is_cut)) = clipboard_data.clone() {
            let nav_f = nav.clone();
            let cp_f = current_p.clone();
            let dest_dir_c = dest_dir.clone();
            glib::spawn_future_local(async move {
                let mut all_success = true;
                for src in sources {
                    if let Some(filename) = src.file_name() {
                        let dest = dest_dir_c.join(filename);
                        if is_cut {
                            if let Err(e) = babydra_common::move_path(src, dest).await {
                                eprintln!("Failed to move file: {}", e);
                                all_success = false;
                            }
                        } else {
                            if let Err(e) = babydra_common::copy_path(src, dest).await {
                                eprintln!("Failed to copy file: {}", e);
                                all_success = false;
                            }
                        }
                    }
                }
                if is_cut && all_success {
                    CLIPBOARD.with(|cb| cb.replace(None)); // Clear clipboard on cut
                }
                nav_f(cp_f);
            });
        }
    });

    // New folder action
    let pop_c = popover.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    btn_new_folder.connect_clicked(move |_| {
        pop_c.popdown();
        crate::explore::dialogs::show_new_folder_dialog(current_p.clone(), nav.clone());
    });

    popover.popup();
}
