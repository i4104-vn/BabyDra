use std::path::PathBuf;
use std::rc::Rc;
use gtk4::prelude::*;
use super::{CLIPBOARD, create_menu_button, create_footer_icon_button};

/// Renders the standard context menu for files/directories outside the Trash.
pub fn show_for_file_normal(
    popover: &gtk4::Popover,
    vbox: &gtk4::Box,
    target_paths: Vec<PathBuf>,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    // Create vertical menu buttons
    let btn_open = create_menu_button("Open", "folder-new");
    let btn_delete = create_menu_button("Delete Permanently", "trash");

    vbox.append(&btn_open);
    vbox.append(&btn_delete);

    // Create horizontal footer box for clipboard & file operations
    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.add_css_class("menu-sep");
    vbox.append(&sep);

    let footer_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    footer_box.add_css_class("context-menu-footer");
    footer_box.set_halign(gtk4::Align::Fill);
    footer_box.set_homogeneous(true);

    let btn_cut = create_footer_icon_button("cut", "Cut");
    let btn_copy = create_footer_icon_button("copy", "Copy");
    
    footer_box.append(&btn_cut);
    footer_box.append(&btn_copy);

    // Paste button (only if target is a single directory and clipboard is not empty)
    let is_target_dir = target_paths.len() == 1 && target_paths[0].is_dir();
    let clipboard_data = CLIPBOARD.with(|cb| cb.borrow().clone());
    
    let btn_paste = create_footer_icon_button("paste", "Paste");
    if is_target_dir {
        btn_paste.set_sensitive(clipboard_data.is_some());
        footer_box.append(&btn_paste);
    }

    // Rename button (only if 1 target is selected)
    let btn_rename = create_footer_icon_button("rename", "Rename");
    if target_paths.len() == 1 {
        footer_box.append(&btn_rename);
    }

    let btn_trash = create_footer_icon_button("trash", "Move to Trash");
    footer_box.append(&btn_trash);

    vbox.append(&footer_box);

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

    // Paste handler (if folder selected)
    if is_target_dir {
        let pop_c = popover.clone();
        let dest_dir = target_paths[0].clone();
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
                        CLIPBOARD.with(|cb| cb.replace(None));
                    }
                    nav_f(cp_f);
                });
            }
        });
    }

    // Rename dialog trigger
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
