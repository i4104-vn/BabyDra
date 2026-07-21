use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, ListBox, ListBoxRow, Entry, Grid};
use babydra_common::i18n::t;
use babydra_common::config::settings::CustomContextItem;

const AVAILABLE_ICONS: &[&str] = &[
    "settings", "terminal", "folder", "text", "camera", "music", "user",
    "activity", "lock", "wifi", "refresh", "power", "search", "logo"
];

/// Renders a custom context menu item list row with inline editing and delete capabilities.
pub fn render_option_row(listbox: &ListBox, item: CustomContextItem) {
    let row = ListBoxRow::new();
    row.add_css_class("settings-custom-item-row");

    let saved_item = std::cell::RefCell::new(item.clone());

    // 1. VIEW MODE LAYOUT
    let hbox_view = Box::new(Orientation::Horizontal, 10);
    hbox_view.set_margin_top(2);
    hbox_view.set_margin_bottom(2);
    hbox_view.set_margin_start(10);
    hbox_view.set_margin_end(10);

    let vbox_text = Box::new(Orientation::Vertical, 2);
    vbox_text.set_valign(Align::Center);
    let lbl_name = Label::builder()
        .halign(Align::Start)
        .build();
    lbl_name.add_css_class("settings-item-name");
    
    let lbl_cmd = Label::builder()
        .halign(Align::Start)
        .build();
    lbl_cmd.add_css_class("settings-item-command");

    let icon_name = item.icon.as_deref().unwrap_or("settings");
    let img_icon = babydra_utils::ui::icon::get_icon(icon_name, 16);
    img_icon.set_pixel_size(16);
    img_icon.set_valign(Align::Center);

    vbox_text.append(&lbl_name);
    vbox_text.append(&lbl_cmd);
    hbox_view.append(&img_icon);
    hbox_view.append(&vbox_text);

    let spacer1 = Box::new(Orientation::Horizontal, 0);
    spacer1.set_hexpand(true);
    hbox_view.append(&spacer1);

    let btn_edit = babydra_utils::components::create_icon_button("rename", 16, &["flat", "circular", "row-action-btn", "edit-btn"], Some(&t("explore.settings_edit")), || {});
    btn_edit.set_valign(Align::Center);
    btn_edit.set_cursor_from_name(Some("pointer"));
    hbox_view.append(&btn_edit);

    let btn_del = babydra_utils::components::create_icon_button("trash", 16, &["flat", "circular", "row-action-btn", "delete-btn"], Some(&t("explore.settings_delete")), || {});
    btn_del.set_valign(Align::Center);
    btn_del.set_cursor_from_name(Some("pointer"));
    hbox_view.append(&btn_del);

    // 2. EDIT MODE LAYOUT
    let hbox_edit = Box::new(Orientation::Horizontal, 10);
    hbox_edit.set_margin_top(2);
    hbox_edit.set_margin_bottom(2);
    hbox_edit.set_margin_start(10);
    hbox_edit.set_margin_end(10);

    let grid_edit = Grid::new();
    grid_edit.set_row_spacing(6);
    grid_edit.set_column_spacing(8);
    grid_edit.set_hexpand(true);
    hbox_edit.append(&grid_edit);

    let name_edit_hbox = Box::new(Orientation::Horizontal, 8);
    let entry_edit_name = Entry::builder()
        .hexpand(true)
        .css_classes(vec!["inline-entry".to_string(), "small-entry".to_string()])
        .build();
    
    // State for edited icon selection
    let edit_selected_icon = std::rc::Rc::new(std::cell::RefCell::new(item.icon.clone().unwrap_or_else(|| "settings".to_string())));

    let initial_icon = item.icon.clone().unwrap_or_else(|| "settings".to_string());
    let popover_edit_icon = gtk4::Popover::builder()
        .has_arrow(true)
        .autohide(true)
        .build();

    let popover_edit_icon_c = popover_edit_icon.clone();
    let btn_edit_icon = babydra_utils::components::create_icon_button(&initial_icon, 16, &["circular", "icon-select-btn"], None, move || popover_edit_icon_c.popup());
    btn_edit_icon.set_valign(Align::Center);
    btn_edit_icon.set_cursor_from_name(Some("pointer"));
    popover_edit_icon.set_parent(&btn_edit_icon);

    let edit_icon_grid = Grid::new();
    edit_icon_grid.set_row_spacing(6);
    edit_icon_grid.set_column_spacing(6);
    edit_icon_grid.set_margin_top(8);
    edit_icon_grid.set_margin_bottom(8);
    edit_icon_grid.set_margin_start(8);
    edit_icon_grid.set_margin_end(8);

    let cols = 4;
    for (idx, icon_name) in AVAILABLE_ICONS.iter().enumerate() {
        let r = (idx / cols) as i32;
        let c = (idx % cols) as i32;
        
        let icon_name_str = icon_name.to_string();
        let edit_selected_icon_c = edit_selected_icon.clone();
        let btn_edit_icon_c = btn_edit_icon.clone();
        let popover_edit_icon_c = popover_edit_icon.clone();
        
        let btn_item = babydra_utils::components::create_icon_button(icon_name, 20, &["flat", "icon-grid-item"], Some(*icon_name), move || { edit_selected_icon_c.replace(icon_name_str.clone()); let new_img = babydra_utils::ui::icon::get_icon(&icon_name_str, 16); new_img.set_pixel_size(16); btn_edit_icon_c.set_child(Some(&new_img)); popover_edit_icon_c.popdown(); });
        btn_item.set_cursor_from_name(Some("pointer"));
        
        edit_icon_grid.attach(&btn_item, c, r, 1, 1);
    }
    popover_edit_icon.set_child(Some(&edit_icon_grid));

    name_edit_hbox.append(&entry_edit_name);
    name_edit_hbox.append(&btn_edit_icon);

    let entry_edit_cmd = Entry::builder()
        .hexpand(true)
        .css_classes(vec!["inline-entry".to_string(), "small-entry".to_string()])
        .build();

    grid_edit.attach(&name_edit_hbox, 0, 0, 1, 1);
    grid_edit.attach(&entry_edit_cmd, 0, 1, 1, 1);

    // Helper placeholders for inline edit command entry
    let inline_placeholders = Box::new(Orientation::Horizontal, 4);
    inline_placeholders.set_margin_top(2);
    let ph_list = [
        ("{path}", "explore.placeholder_path_desc"),
        ("{dir}",  "explore.placeholder_dir_desc"),
        ("{name}", "explore.placeholder_name_desc"),
        ("{stem}", "explore.placeholder_stem_desc"),
        ("{ext}",  "explore.placeholder_ext_desc"),
    ];
    for (ph, desc_key) in ph_list {
        let btn_ph = babydra_utils::components::create_button(ph);
        btn_ph.remove_css_class("baby-button");
        btn_ph.add_css_class("flat");
        btn_ph.add_css_class("placeholder-btn-small");
        btn_ph.set_tooltip_text(Some(&t(desc_key)));
        btn_ph.set_cursor_from_name(Some("pointer"));
        let entry_edit_cmd_c = entry_edit_cmd.clone();
        btn_ph.connect_clicked(move |_| {
            let mut pos = entry_edit_cmd_c.position();
            entry_edit_cmd_c.insert_text(ph, &mut pos);
            entry_edit_cmd_c.grab_focus();
        });
        inline_placeholders.append(&btn_ph);
    }
    grid_edit.attach(&inline_placeholders, 0, 2, 1, 1);

    let vbox_buttons = Box::new(Orientation::Vertical, 6);
    vbox_buttons.set_valign(Align::Center);
    hbox_edit.append(&vbox_buttons);

    let btn_save = babydra_utils::components::create_accent_button(&t("explore.settings_save"));
    btn_save.add_css_class("small-btn");
    btn_save.set_cursor_from_name(Some("pointer"));
    let btn_cancel = babydra_utils::components::create_button(&t("explore.settings_cancel"));
    btn_cancel.remove_css_class("baby-button");
    btn_cancel.add_css_class("flat");
    btn_cancel.add_css_class("small-btn");
    btn_cancel.set_cursor_from_name(Some("pointer"));

    vbox_buttons.append(&btn_save);
    vbox_buttons.append(&btn_cancel);

    // Sync view content helper
    let update_view_labels = {
        let lbl_n = lbl_name.clone();
        let lbl_c = lbl_cmd.clone();
        let saved = saved_item.clone();
        let img_icon_c = img_icon.clone();
        move || {
            let item = saved.borrow();
            lbl_n.set_label(&item.name);
            lbl_c.set_label(&item.command);
            
            let icon_name = item.icon.as_deref().unwrap_or("settings");
            let temp_img = babydra_utils::ui::icon::get_icon(icon_name, 16);
            if let Some(paintable) = temp_img.paintable() {
                img_icon_c.set_paintable(Some(&paintable));
            }
        }
    };
    update_view_labels();

    // Wire View Mode Edit click
    let row_c = row.clone();
    let hb_edit = hbox_edit.clone();
    let ent_name = entry_edit_name.clone();
    let ent_cmd = entry_edit_cmd.clone();
    let btn_edit_icon_c = btn_edit_icon.clone();
    let edit_selected_icon_c = edit_selected_icon.clone();
    let saved = saved_item.clone();
    btn_edit.connect_clicked(move |_| {
        let current = saved.borrow();
        ent_name.set_text(&current.name);
        ent_cmd.set_text(&current.command);
        
        let cur_icon = current.icon.clone().unwrap_or_else(|| "settings".to_string());
        edit_selected_icon_c.replace(cur_icon.clone());
        let cur_icon_img = babydra_utils::ui::icon::get_icon(&cur_icon, 16);
        cur_icon_img.set_pixel_size(16);
        btn_edit_icon_c.set_child(Some(&cur_icon_img));

        row_c.set_child(Some(&hb_edit));
        ent_name.grab_focus();
    });

    // Wire Edit Mode Cancel click
    let row_c2 = row.clone();
    let hb_view = hbox_view.clone();
    btn_cancel.connect_clicked(move |_| {
        row_c2.set_child(Some(&hb_view));
    });

    // Wire Edit Mode Save click
    let row_c3 = row.clone();
    let hb_view2 = hbox_view.clone();
    let ent_name2 = entry_edit_name.clone();
    let ent_cmd2 = entry_edit_cmd.clone();
    let edit_selected_icon2 = edit_selected_icon.clone();
    let saved2 = saved_item.clone();
    let update_lbls = update_view_labels.clone();
    btn_save.connect_clicked(move |_| {
        let new_name = ent_name2.text().to_string();
        let new_cmd = ent_cmd2.text().to_string();
        let new_icon = edit_selected_icon2.borrow().clone();
        if !new_name.is_empty() && !new_cmd.is_empty() {
            let old_name = saved2.borrow().name.clone();
            let old_cmd = saved2.borrow().command.clone();

            {
                let mut s = saved2.borrow_mut();
                s.name = new_name.clone();
                s.command = new_cmd.clone();
                s.icon = Some(new_icon.clone());
            }

            // Update settings file
            let mut s = babydra_common::load_explore_settings();
            if let Some(idx) = s.custom_context_items.iter().position(|i| i.name == old_name && i.command == old_cmd) {
                s.custom_context_items[idx].name = new_name;
                s.custom_context_items[idx].command = new_cmd;
                s.custom_context_items[idx].icon = Some(new_icon);
                babydra_common::save_explore_settings(&s);
            }

            update_lbls();
            row_c3.set_child(Some(&hb_view2));
        }
    });

    // Wire view mode Delete click
    let saved3 = saved_item.clone();
    let listbox_c2 = listbox.clone();
    let row_c4 = row.clone();
    btn_del.connect_clicked(move |_| {
        listbox_c2.remove(&row_c4);
        let item = saved3.borrow();
        let mut s = babydra_common::load_explore_settings();
        s.custom_context_items.retain(|i| i.name != item.name || i.command != item.command);
        babydra_common::save_explore_settings(&s);
    });

    row.set_child(Some(&hbox_view));
    listbox.append(&row);
}
