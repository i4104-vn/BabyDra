use crate::components::context_menu::ContextMenuBuilder;
use crate::components::explore::context_menu::{
    clipboard::execute_paste,
    custom_items::append_custom_context_items,
    CLIPBOARD,
};
use gtk4::prelude::*;
use std::path::PathBuf;
use std::rc::Rc;

use babydra_core::i18n::t;

/// Renders the context menu when right-clicking on an empty space inside a folder directory.
pub fn show_for_empty(
    parent_widget: &gtk4::Widget,
    x: f64,
    y: f64,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent_window: &gtk4::Window,
) {
    if current_path.to_string_lossy().contains("Trash/files") {
        return;
    }

    let mut builder = ContextMenuBuilder::new(parent_widget).at_coords(x, y);

    // 1. Refresh
    let nav_refresh = nav_callback.clone();
    let current_path_refresh = current_path.clone();
    builder = builder.item(&t("explore.menu_refresh"), "refresh", move || {
        nav_refresh(current_path_refresh.clone());
    });

    // 2. Copy Location
    let cur_p_loc = current_path.clone();
    builder = builder.item(&t("explore.menu_copy_location"), "copy", move || {
        if let Some(display) = gtk4::gdk::Display::default() {
            display.clipboard().set_text(&cur_p_loc.to_string_lossy());
        }
    });

    // 3. Submenu: New (Folder, Document)
    let nav_new_folder = nav_callback.clone();
    let current_p_folder = current_path.clone();
    let parent_win_folder = parent_window.clone();

    let nav_new_file = nav_callback.clone();
    let current_p_file = current_path.clone();
    let parent_win_file = parent_window.clone();

    builder = builder.submenu(&t("explore.menu_new"), Some("plus"), move |sub| {
        sub.item(&t("explore.menu_new_folder"), "folder-new", move || {
            crate::components::explore::dialogs::show_new_folder_dialog(
                current_p_folder.clone(),
                nav_new_folder.clone(),
                Some(&parent_win_folder),
            );
        })
        .item(&t("explore.menu_new_file"), "text", move || {
            crate::components::explore::dialogs::show_new_file_dialog(
                current_p_file.clone(),
                nav_new_file.clone(),
                Some(&parent_win_file),
            );
        })
    });

    // 4. Paste
    let clipboard_data = CLIPBOARD.with(|cb| cb.borrow().clone());
    let has_paste_items = clipboard_data
        .as_ref()
        .map_or(false, |(sources, _)| !sources.is_empty());

    if has_paste_items {
        let dest_dir = current_path.clone();
        let nav = nav_callback.clone();
        let current_p = current_path.clone();
        let clipboard_data_c1 = clipboard_data.clone();
        builder = builder.item(&t("explore.menu_paste"), "paste", move || {
            if let Some((sources, is_cut)) = clipboard_data_c1.clone() {
                execute_paste(
                    sources,
                    dest_dir.clone(),
                    is_cut,
                    current_p.clone(),
                    nav.clone(),
                );
            }
        });
    }

    // 5. Custom Context Options for empty area
    let current_p_custom = current_path.clone();
    builder = builder.custom_items(move |vbox, popover| {
        append_custom_context_items(vbox, popover, vec![current_p_custom], true);
    });

    // 6. Footer actions (Cut, Copy, Paste, Rename, Trash)
    let dest_dir = current_path.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    let clipboard_data_c2 = clipboard_data.clone();

    builder = builder
        .footer_button_sensitive("cut", &t("explore.menu_cut"), false, || {})
        .footer_button_sensitive("copy", &t("explore.menu_copy"), false, || {})
        .footer_button_sensitive("paste", &t("explore.menu_paste"), has_paste_items, move || {
            if let Some((sources, is_cut)) = clipboard_data_c2.clone() {
                execute_paste(
                    sources,
                    dest_dir.clone(),
                    is_cut,
                    current_p.clone(),
                    nav.clone(),
                );
            }
        })
        .footer_button_sensitive("rename", &t("explore.menu_rename"), false, || {})
        .footer_button_sensitive("trash", &t("explore.menu_trash"), false, || {});

    builder.popup();
}

