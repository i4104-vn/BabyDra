use crate::components::explore::context_menu::{
    clipboard::execute_paste,
    custom_items::append_custom_context_items,
    widgets::{create_menu_button, ContextMenuBuilder},
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

    // 3. Sub-popover containing create options
    let btn_create_new = create_menu_button(&t("explore.menu_new"), "plus");
    let sub_popover = crate::components::popovers::create_popover(
        &btn_create_new,
        gtk4::PositionType::Right,
        "explore-popover",
    );
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
    let sub_popover_c2 = sub_popover.clone();
    let motion = gtk4::EventControllerMotion::new();
    motion.connect_enter(move |_, _, _| {
        sub_popover_c2.popup();
    });
    btn_create_new.add_controller(motion);
    builder = builder.raw_item(&btn_create_new);

    // 4. Check clipboard state for paste availability
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

    // Sub-popover click actions
    let pop_c1 = builder.popover().clone();
    let sub_pop_c1 = sub_popover.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    let parent_win_c1 = parent_window.clone();
    btn_new_folder.connect_clicked(move |_| {
        sub_pop_c1.popdown();
        pop_c1.popdown();
        crate::components::explore::dialogs::show_new_folder_dialog(
            current_p.clone(),
            nav.clone(),
            Some(&parent_win_c1),
        );
    });

    let pop_c2 = builder.popover().clone();
    let sub_pop_c2 = sub_popover.clone();
    let nav2 = nav_callback.clone();
    let current_p2 = current_path.clone();
    let parent_win_c2 = parent_window.clone();
    btn_new_file.connect_clicked(move |_| {
        sub_pop_c2.popdown();
        pop_c2.popdown();
        crate::components::explore::dialogs::show_new_file_dialog(
            current_p2.clone(),
            nav2.clone(),
            Some(&parent_win_c2),
        );
    });

    // Custom Context Options for empty area
    append_custom_context_items(builder.container(), builder.popover(), vec![current_path.clone()], true);

    // Footer actions (Cut, Copy, Paste, Rename, Trash)
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
