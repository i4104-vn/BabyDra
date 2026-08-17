use crate::explore::context_menu::{
    clipboard::execute_paste,
    custom_items::append_custom_context_items,
    widgets::{
        create_footer_container, create_footer_icon_button, create_menu_button, create_menu_popover,
    },
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
    let (popover, vbox) = create_menu_popover(parent_widget, x, y);

    let btn_refresh = create_menu_button(&t("explore.menu_refresh"), "refresh");
    let btn_copy_location = create_menu_button(&t("explore.menu_copy_location"), "copy");
    let btn_create_new = create_menu_button(&t("explore.menu_new"), "plus");

    vbox.append(&btn_refresh);
    vbox.append(&btn_copy_location);
    vbox.append(&btn_create_new);

    let pop_c = popover.clone();
    let cur_p_loc = current_path.clone();
    btn_copy_location.connect_clicked(move |_| {
        pop_c.popdown();
        if let Some(display) = gtk4::gdk::Display::default() {
            display.clipboard().set_text(&cur_p_loc.to_string_lossy());
        }
    });

    // Sub-popover containing create options
    let sub_popover = babydra_ui_kit::components::popovers::create_popover(
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

    // Check clipboard state for paste availability
    let clipboard_data = CLIPBOARD.with(|cb| cb.borrow().clone());
    let has_paste_items = clipboard_data
        .as_ref()
        .map_or(false, |(sources, _)| !sources.is_empty());

    // Only render top Paste menu button if clipboard has files to move/paste (Hidden if empty)
    if has_paste_items {
        let btn_paste = create_menu_button(&t("explore.menu_paste"), "paste");
        vbox.append(&btn_paste);

        let pop_c = popover.clone();
        let dest_dir = current_path.clone();
        let nav = nav_callback.clone();
        let current_p = current_path.clone();
        let clipboard_data_c1 = clipboard_data.clone();
        btn_paste.connect_clicked(move |_| {
            pop_c.popdown();
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

    let pop_c = popover.clone();
    let nav_refresh = nav_callback.clone();
    let current_path_refresh = current_path.clone();
    btn_refresh.connect_clicked(move |_| {
        pop_c.popdown();
        nav_refresh(current_path_refresh.clone());
    });

    // Sub-popover click actions
    let pop_c1 = popover.clone();
    let sub_pop_c1 = sub_popover.clone();
    let nav = nav_callback.clone();
    let current_p = current_path.clone();
    let parent_win_c1 = parent_window.clone();
    btn_new_folder.connect_clicked(move |_| {
        sub_pop_c1.popdown();
        pop_c1.popdown();
        crate::explore::dialogs::show_new_folder_dialog(
            current_p.clone(),
            nav.clone(),
            Some(&parent_win_c1),
        );
    });

    let pop_c2 = popover.clone();
    let sub_pop_c2 = sub_popover.clone();
    let nav2 = nav_callback.clone();
    let current_p2 = current_path.clone();
    let parent_win_c2 = parent_window.clone();
    btn_new_file.connect_clicked(move |_| {
        sub_pop_c2.popdown();
        pop_c2.popdown();
        crate::explore::dialogs::show_new_file_dialog(
            current_p2.clone(),
            nav2.clone(),
            Some(&parent_win_c2),
        );
    });

    // Custom Context Options for empty area
    append_custom_context_items(&vbox, &popover, vec![current_path.clone()], true);

    // Footer Container (Footer buttons remain visible, paste is disabled if clipboard is empty)
    let (footer_container, footer_box) = create_footer_container();

    let btn_footer_cut = create_footer_icon_button("cut", &t("explore.menu_cut"));
    let btn_footer_copy = create_footer_icon_button("copy", &t("explore.menu_copy"));
    let btn_footer_paste = create_footer_icon_button("paste", &t("explore.menu_paste"));
    let btn_footer_rename = create_footer_icon_button("rename", &t("explore.menu_rename"));
    let btn_footer_trash = create_footer_icon_button("trash", &t("explore.menu_trash"));

    btn_footer_cut.set_sensitive(false);
    btn_footer_copy.set_sensitive(false);
    btn_footer_paste.set_sensitive(has_paste_items);
    btn_footer_rename.set_sensitive(false);
    btn_footer_trash.set_sensitive(false);

    footer_box.append(&btn_footer_cut);
    footer_box.append(&btn_footer_copy);
    footer_box.append(&btn_footer_paste);
    footer_box.append(&btn_footer_rename);
    footer_box.append(&btn_footer_trash);

    if has_paste_items {
        let pop_c = popover.clone();
        let dest_dir = current_path.clone();
        let nav = nav_callback.clone();
        let current_p = current_path.clone();
        let clipboard_data_c2 = clipboard_data.clone();
        btn_footer_paste.connect_clicked(move |_| {
            pop_c.popdown();
            if let Some((sources, is_cut)) = clipboard_data_c2.clone() {
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

    vbox.append(&footer_container);

    popover.popup();
}
