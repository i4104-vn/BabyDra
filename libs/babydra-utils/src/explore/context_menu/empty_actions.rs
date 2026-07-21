use std::path::PathBuf;
use std::rc::Rc;
use gtk4::prelude::*;
use crate::explore::context_menu::{
    CLIPBOARD,
    widgets::{create_menu_popover, create_menu_button},
    clipboard::execute_paste,
    custom_items::append_custom_context_items,
};

use babydra_common::i18n::t;

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

    let btn_create_new = create_menu_button(&t("explore.menu_new"), "plus");
    let btn_paste = create_menu_button(&t("explore.menu_paste"), "paste");

    vbox.append(&btn_create_new);
    vbox.append(&btn_paste);

    // Sub-popover containing create options
    let sub_popover = crate::components::popovers::create_popover(&btn_create_new, gtk4::PositionType::Right, "explore-popover");
    let sub_vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    sub_vbox.set_css_classes(&["context-menu-box"]);
    sub_vbox.set_width_request(150);

    let btn_new_folder = create_menu_button(&t("explore.menu_new_folder"), "folder-new");
    let btn_new_file = create_menu_button(&t("explore.menu_new_file"), "text");

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
            execute_paste(sources, dest_dir.clone(), is_cut, current_p.clone(), nav.clone());
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
    append_custom_context_items(&vbox, &popover, vec![current_path], true);

    popover.popup();
}
