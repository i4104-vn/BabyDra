use std::path::PathBuf;
use std::rc::Rc;
use gtk4::prelude::*;
use crate::explore::context_menu::{CLIPBOARD, create_menu_popover, create_menu_button};

/// Renders the context menu when right-clicking on an empty space inside a folder directory.
pub fn show_for_empty(
    parent: &gtk4::Widget,
    x: f64,
    y: f64,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
) {
    if current_path.to_string_lossy().contains("Trash/files") {
        return;
    }
    let (popover, vbox) = create_menu_popover(parent, x, y);

    let btn_create_new = create_menu_button("New", "plus");
    let btn_paste = create_menu_button("Paste", "paste");

    vbox.append(&btn_create_new);
    vbox.append(&btn_paste);

    // Sub-popover containing create options
    let sub_popover = crate::components::popovers::create_popover(&btn_create_new, gtk4::PositionType::Right, "explore-popover");
    let sub_vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    sub_vbox.set_css_classes(&["context-menu-box"]);
    sub_vbox.set_width_request(150);

    let btn_new_folder = create_menu_button("Folder", "folder-new");
    let btn_new_file = create_menu_button("File", "text");

    sub_vbox.append(&btn_new_folder);
    sub_vbox.append(&btn_new_file);
    sub_popover.set_child(Some(&sub_vbox));

    let sub_popover_c = sub_popover.clone();
    btn_create_new.connect_clicked(move |_| {
        sub_popover_c.popup();
    });

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

    // Sub-popover click actions
    let pop_c1 = popover.clone();
    let sub_pop_c1 = sub_popover.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    btn_new_folder.connect_clicked(move |_| {
        sub_pop_c1.popdown();
        pop_c1.popdown();
        crate::explore::dialogs::show_new_folder_dialog(current_p.clone(), nav.clone());
    });

    let pop_c2 = popover.clone();
    let sub_pop_c2 = sub_popover.clone();
    let nav2 = nav_callback.clone();
    let current_p2 = current_path.clone();
    btn_new_file.connect_clicked(move |_| {
        sub_pop_c2.popdown();
        pop_c2.popdown();
        crate::explore::dialogs::show_new_file_dialog(current_p2.clone(), nav2.clone());
    });

    // Custom Context Options for empty area
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
            let current_path_c = current_path.clone();
            btn_custom.connect_clicked(move |_| {
                pop_c.popdown();
                let path_str = current_path_c.to_string_lossy().to_string();
                let name_str = current_path_c.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                let stem_str = current_path_c.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                let ext_str = current_path_c.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();

                let cmd_str = command_tmpl
                    .replace("{path}", &path_str)
                    .replace("{dir}", &path_str)
                    .replace("{name}", &name_str)
                    .replace("{stem}", &stem_str)
                    .replace("{ext}", &ext_str);
                
                let _ = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&cmd_str)
                    .spawn();
            });
        }
    }

    popover.popup();
}
