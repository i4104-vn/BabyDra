use std::path::PathBuf;
use std::rc::Rc;
use gtk4::prelude::*;
use super::{CLIPBOARD, create_menu_popover, create_menu_button};

/// Renders the standard context menu for files/directories outside the Trash.
pub fn show_for_file_normal(
    popover: &gtk4::Popover,
    vbox: &gtk4::Box,
    target_paths: Vec<PathBuf>,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    // Create buttons
    let btn_open = create_menu_button("Open", "folder-new");
    let btn_cut = create_menu_button("Cut", "cut");
    let btn_copy = create_menu_button("Copy", "copy");
    let btn_rename = create_menu_button("Rename", "rename");
    let btn_trash = create_menu_button("Move to Trash", "trash");
    let btn_delete = create_menu_button("Delete Permanently", "trash");

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

    // Custom Context Options
    let settings = babydra_common::load_explore_settings();
    if !settings.custom_context_items.is_empty() {
        let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
        sep.add_css_class("menu-sep");
        vbox.append(&sep);

        for item in settings.custom_context_items {
            let icon_key = if item.name.to_lowercase().contains("terminal") {
                "terminal"
            } else {
                "settings"
            };
            let btn_custom = create_menu_button(&item.name, icon_key);
            vbox.append(&btn_custom);

            let pop_c = popover.clone();
            let command_tmpl = item.command.clone();
            let target_paths_c = target_paths.clone();
            btn_custom.connect_clicked(move |_| {
                pop_c.popdown();
                let command_tmpl_c = command_tmpl.clone();
                let paths = target_paths_c.clone();
                for path in paths {
                    let path_str = path.to_string_lossy().to_string();
                    let parent_str = path.parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_else(|| "".to_string());
                    let name_str = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                    let stem_str = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                    let ext_str = path.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();

                    let cmd_str = command_tmpl_c
                        .replace("{path}", &path_str)
                        .replace("{dir}", &parent_str)
                        .replace("{name}", &name_str)
                        .replace("{stem}", &stem_str)
                        .replace("{ext}", &ext_str);
                    
                    let _ = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(&cmd_str)
                        .spawn();
                }
            });
        }
    }

    popover.popup();
}
