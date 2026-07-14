use std::path::PathBuf;
use std::rc::Rc;
use gtk4::prelude::*;
use babydra_common::FileEntry;
use crate::widgets::dialogs;
use super::CLIPBOARD;
use super::helpers::{create_menu_popover, create_menu_button};

pub fn show_for_file(
    parent: &gtk4::Widget,
    x: f64,
    y: f64,
    entry: FileEntry,
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
    vbox.append(&btn_rename);
    vbox.append(&btn_trash);
    vbox.append(&btn_delete);

    // Event handling
    let pop_c = popover.clone();
    let target_path = entry.path.clone();
    let is_dir = matches!(entry.file_type, babydra_common::FileType::Directory);
    let nav = nav_callback.clone();
    btn_open.connect_clicked(move |_| {
        pop_c.popdown();
        if is_dir {
            nav(target_path.clone());
        } else {
            let uri = format!("file://{}", target_path.to_string_lossy());
            let _ = gio::AppInfo::launch_default_for_uri(&uri, gio::AppLaunchContext::NONE);
        }
    });

    let pop_c = popover.clone();
    let src_path = entry.path.clone();
    btn_copy.connect_clicked(move |_| {
        pop_c.popdown();
        CLIPBOARD.with(|cb| {
            cb.replace(Some((src_path.clone(), false)));
        });
    });

    let pop_c = popover.clone();
    let src_path = entry.path.clone();
    btn_cut.connect_clicked(move |_| {
        pop_c.popdown();
        CLIPBOARD.with(|cb| {
            cb.replace(Some((src_path.clone(), true)));
        });
    });

    // Rename dialog trigger
    let pop_c = popover.clone();
    let rename_path = entry.path.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    btn_rename.connect_clicked(move |_| {
        pop_c.popdown();
        dialogs::show_rename_dialog(&rename_path, current_p.clone(), nav.clone());
    });

    // Trash action
    let pop_c = popover.clone();
    let trash_path = entry.path.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    btn_trash.connect_clicked(move |_| {
        pop_c.popdown();
        let nav_c = nav.clone();
        let cp_c = current_p.clone();
        let path_c = trash_path.clone();
        glib::spawn_future_local(async move {
            if let Err(err) = babydra_common::send_to_trash(path_c).await {
                eprintln!("Failed to trash file: {}", err);
            }
            nav_c(cp_c);
        });
    });

    // Permanent delete
    let pop_c = popover.clone();
    let del_path = entry.path.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    btn_delete.connect_clicked(move |_| {
        pop_c.popdown();
        let nav_c = nav.clone();
        let cp_c = current_p.clone();
        let path_c = del_path.clone();
        glib::spawn_future_local(async move {
            if let Err(err) = babydra_common::delete_path(path_c).await {
                eprintln!("Failed to delete file: {}", err);
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
        if let Some((src, is_cut)) = clipboard_data.clone() {
            let dest = dest_dir.join(src.file_name().unwrap());
            let nav_f = nav.clone();
            let cp_f = current_p.clone();
            glib::spawn_future_local(async move {
                if is_cut {
                    if let Err(e) = babydra_common::move_path(src, dest).await {
                        eprintln!("Failed to move file: {}", e);
                    } else {
                        CLIPBOARD.with(|cb| cb.replace(None)); // Clear clipboard on cut
                    }
                } else {
                    if let Err(e) = babydra_common::copy_path(src, dest).await {
                        eprintln!("Failed to copy file: {}", e);
                    }
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
        dialogs::show_new_folder_dialog(current_p.clone(), nav.clone());
    });

    popover.popup();
}
