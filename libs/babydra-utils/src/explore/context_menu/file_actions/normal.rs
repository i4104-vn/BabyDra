use std::path::PathBuf;
use std::rc::Rc;
use gtk4::prelude::*;
use crate::explore::context_menu::{
    CLIPBOARD,
    widgets::{create_menu_button, create_footer_icon_button},
    clipboard::execute_paste,
    custom_items::append_custom_context_items,
};
use crate::explore::helpers::is_archive_file;

use babydra_common::i18n::t;

/// Renders the standard context menu for files/directories outside the Trash.
pub fn show_for_file_normal(
    popover: &gtk4::Popover,
    vbox: &gtk4::Box,
    target_paths: Vec<PathBuf>,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: &gtk4::Window,
) {
    // Create vertical menu buttons
    let btn_open = create_menu_button(&t("explore.menu_open"), "folder-new");
    let btn_compress = create_menu_button(&t("explore.menu_compress"), "folder");
    let has_archive = target_paths.iter().any(|path| is_archive_file(path));
    let btn_decompress = if has_archive {
        Some(create_menu_button(&t("explore.menu_decompress"), "download"))
    } else {
        None
    };
    let btn_refresh = create_menu_button(&t("explore.menu_refresh"), "refresh");
    vbox.append(&btn_refresh);

    vbox.append(&btn_open);
    vbox.append(&btn_compress);
    if let Some(ref btn) = btn_decompress {
        vbox.append(btn);
    }

    // Create horizontal footer container & box for clipboard & file operations
    let footer_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    footer_container.add_css_class("context-menu-footer");
    footer_container.set_halign(gtk4::Align::Fill);

    let footer_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    footer_box.set_halign(gtk4::Align::Start);
    footer_box.set_homogeneous(false);

    let btn_cut = create_footer_icon_button("cut", &t("explore.menu_cut"));
    let btn_copy = create_footer_icon_button("copy", &t("explore.menu_copy"));
    let btn_paste = create_footer_icon_button("paste", &t("explore.menu_paste"));
    let btn_rename = create_footer_icon_button("rename", &t("explore.menu_rename"));
    let btn_trash = create_footer_icon_button("trash", &t("explore.menu_trash"));

    btn_cut.set_sensitive(!target_paths.is_empty());
    btn_copy.set_sensitive(!target_paths.is_empty());
    btn_trash.set_sensitive(!target_paths.is_empty());

    footer_box.append(&btn_cut);
    footer_box.append(&btn_copy);
    footer_box.append(&btn_paste);
    footer_box.append(&btn_rename);
    footer_box.append(&btn_trash);

    // Check clipboard state for paste sensitivity
    let clipboard_data = CLIPBOARD.with(|cb| cb.borrow().clone());
    let has_paste_items = clipboard_data.as_ref().map_or(false, |(sources, _)| !sources.is_empty());
    btn_paste.set_sensitive(has_paste_items);

    if has_paste_items {
        let pop_c = popover.clone();
        let is_target_dir = target_paths.len() == 1 && target_paths[0].is_dir();
        let dest_dir = if is_target_dir {
            target_paths[0].clone()
        } else {
            current_path.clone()
        };
        let nav = nav_callback.clone();
        let current_p = current_path.clone();
        let clipboard_data_c = clipboard_data.clone();
        btn_paste.connect_clicked(move |_| {
            pop_c.popdown();
            if let Some((sources, is_cut)) = clipboard_data_c.clone() {
                execute_paste(sources, dest_dir.clone(), is_cut, current_p.clone(), nav.clone());
            }
        });
    }

    // Rename button sensitivity (only enabled if exactly 1 target is selected)
    let is_single = target_paths.len() == 1;
    btn_rename.set_sensitive(is_single);

    footer_container.append(&footer_box);

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
    btn_copy.connect_clicked(move |_| {
        pop_c.popdown();
        CLIPBOARD.with(|cb| {
            cb.replace(Some((target_paths_copy.clone(), false)));
        });
    });

    let pop_c = popover.clone();
    let target_paths_cut = target_paths.clone();
    btn_cut.connect_clicked(move |_| {
        pop_c.popdown();
        CLIPBOARD.with(|cb| {
            cb.replace(Some((target_paths_cut.clone(), true)));
        });
    });

    // Rename dialog trigger
    if is_single {
        let pop_c = popover.clone();
        let rename_path = target_paths[0].clone();
        let nav = nav_callback.clone();
        let current_p = current_path.clone();
        let parent_c = parent.clone();
        btn_rename.connect_clicked(move |_| {
            pop_c.popdown();
            crate::explore::dialogs::show_rename_dialog(&rename_path, current_p.clone(), nav.clone(), Some(&parent_c));
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

    // Compress action
    let target_paths_compress = target_paths.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    let pop_c = popover.clone();
    let parent_c = parent.clone();
    btn_compress.connect_clicked(move |_| {
        pop_c.popdown();
        crate::explore::dialogs::show_compress_dialog(target_paths_compress.clone(), current_p.clone(), nav.clone(), Some(&parent_c));
    });

    // Decompress action
    if let Some(ref btn) = btn_decompress {
        let target_paths_decompress = target_paths.clone();
        let nav = nav_callback.clone();
        let current_p = current_path.clone();
        let pop_c = popover.clone();
        let parent_c = parent.clone();
        btn.connect_clicked(move |_| {
            pop_c.popdown();
            for path in &target_paths_decompress {
                if is_archive_file(path) {
                    crate::explore::dialogs::perform_decompress_async(path.clone(), current_p.clone(), nav.clone(), Some(&parent_c));
                }
            }
        });
    }

    // Refresh action
    let pop_c = popover.clone();
    let nav_refresh = nav_callback.clone();
    let current_p = current_path.clone();
    btn_refresh.connect_clicked(move |_| {
        pop_c.popdown();
        nav_refresh(current_p.clone());
    });

    // Custom Context Options
    append_custom_context_items(vbox, popover, target_paths.clone(), false);

    // Separator and Properties button
    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.add_css_class("menu-sep");
    vbox.append(&sep);

    let btn_properties = create_menu_button(&t("explore.menu_properties"), "info");
    vbox.append(&btn_properties);

    let pop_c = popover.clone();
    let target_paths_props = target_paths.clone();
    let parent_c = parent.clone();
    btn_properties.connect_clicked(move |_| {
        pop_c.popdown();
        crate::explore::dialogs::show_properties_dialog(target_paths_props.clone(), Some(&parent_c));
    });

    vbox.append(&footer_container);
    popover.popup();
}
