use std::path::PathBuf;
use std::rc::Rc;
use gtk4::prelude::*;
use crate::explore::context_menu::{create_menu_button};

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

pub async fn restore_from_trash(trash_file_path: PathBuf) -> Result<(), String> {
    let file_name = trash_file_path.file_name().ok_or("Invalid file name")?;
    let trash_dir = trash_file_path.parent().ok_or("Invalid parent directory")?;
    let trash_root = trash_dir.parent().ok_or("Invalid trash root")?;
    let info_dir = trash_root.join("info");
    
    let info_file_name = format!("{}.trashinfo", file_name.to_string_lossy());
    let info_path = info_dir.join(info_file_name);
    
    if !info_path.exists() {
        return Err("Trash info file does not exist".to_string());
    }
    
    let content = std::fs::read_to_string(&info_path).map_err(|e| e.to_string())?;
    let mut original_path_str = None;
    for line in content.lines() {
        if line.starts_with("Path=") {
            let path_part = &line["Path=".len()..];
            original_path_str = Some(path_part.to_string());
            break;
        }
    }
    
    let original_path_str = original_path_str.ok_or("Path field not found in trashinfo")?;
    let decoded_path_str = percent_decode(&original_path_str);
    let dest_path = PathBuf::from(decoded_path_str);
    
    if let Some(parent) = dest_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    
    babydra_common::move_path(trash_file_path.clone(), dest_path).await.map_err(|e| e.to_string())?;
    let _ = std::fs::remove_file(info_path);
    
    Ok(())
}

fn percent_decode(s: &str) -> String {
    let mut bytes = Vec::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next().unwrap_or('0');
            let h2 = chars.next().unwrap_or('0');
            let hex_str = format!("{}{}", h1, h2);
            if let Ok(b) = u8::from_str_radix(&hex_str, 16) {
                bytes.push(b);
            }
        } else {
            bytes.push(c as u8);
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}
