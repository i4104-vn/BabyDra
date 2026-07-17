use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Button, Align, ListBox, ListBoxRow, Entry, Grid, Separator};
use babydra_common::i18n::t;

pub fn build_context_menu_page() -> Box {
    let settings = babydra_common::load_explore_settings();
    let tab_context = Box::new(Orientation::Vertical, 10);

    let lbl_context_title = Label::builder()
        .label(&t("explore.settings_context_menu"))
        .halign(Align::Start)
        .build();
    lbl_context_title.add_css_class("settings-title-label");
    tab_context.append(&lbl_context_title);

    let lbl_section_title = Label::builder()
        .label(&t("explore.settings_custom_options"))
        .halign(Align::Start)
        .build();
    lbl_section_title.add_css_class("settings-section-title");
    tab_context.append(&lbl_section_title);

    let scroll = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .min_content_height(140)
        .build();
    tab_context.append(&scroll);

    let context_listbox = ListBox::new();
    context_listbox.set_selection_mode(gtk4::SelectionMode::None);
    context_listbox.add_css_class("settings-listbox");
    scroll.set_child(Some(&context_listbox));

    // Helper to render custom option row with inline edit support
    let listbox_c = context_listbox.clone();
    let render_option_row = move |listbox: &ListBox, item: babydra_common::config::settings::CustomContextItem| {
        let row = ListBoxRow::new();
        row.add_css_class("settings-custom-item-row");

        let saved_item = std::cell::RefCell::new(item);

        // 1. VIEW MODE LAYOUT
        let hbox_view = Box::new(Orientation::Horizontal, 10);
        hbox_view.set_margin_top(6);
        hbox_view.set_margin_bottom(6);
        hbox_view.set_margin_start(10);
        hbox_view.set_margin_end(10);

        let vbox_text = Box::new(Orientation::Vertical, 2);
        let lbl_name = Label::builder()
            .halign(Align::Start)
            .build();
        lbl_name.add_css_class("settings-item-name");
        
        let lbl_cmd = Label::builder()
            .halign(Align::Start)
            .build();
        lbl_cmd.add_css_class("settings-item-command");
        
        vbox_text.append(&lbl_name);
        vbox_text.append(&lbl_cmd);
        hbox_view.append(&vbox_text);

        let spacer1 = Box::new(Orientation::Horizontal, 0);
        spacer1.set_hexpand(true);
        hbox_view.append(&spacer1);

        let btn_edit = Button::from_icon_name("document-edit-symbolic");
        btn_edit.set_tooltip_text(Some(&t("explore.settings_edit")));
        btn_edit.add_css_class("flat");
        btn_edit.add_css_class("edit-btn");
        btn_edit.set_cursor_from_name(Some("pointer"));
        hbox_view.append(&btn_edit);

        let btn_del = Button::from_icon_name("user-trash-symbolic");
        btn_del.set_tooltip_text(Some(&t("explore.settings_delete")));
        btn_del.add_css_class("flat");
        btn_del.add_css_class("destructive-action");
        btn_del.set_cursor_from_name(Some("pointer"));
        hbox_view.append(&btn_del);

        // 2. EDIT MODE LAYOUT
        let hbox_edit = Box::new(Orientation::Horizontal, 10);
        hbox_edit.set_margin_top(6);
        hbox_edit.set_margin_bottom(6);
        hbox_edit.set_margin_start(10);
        hbox_edit.set_margin_end(10);

        let grid_edit = Grid::new();
        grid_edit.set_row_spacing(6);
        grid_edit.set_column_spacing(8);
        grid_edit.set_hexpand(true);
        hbox_edit.append(&grid_edit);

        let entry_edit_name = Entry::builder().hexpand(true).build();
        entry_edit_name.add_css_class("inline-entry");
        let entry_edit_cmd = Entry::builder().hexpand(true).build();
        entry_edit_cmd.add_css_class("inline-entry");

        grid_edit.attach(&entry_edit_name, 0, 0, 1, 1);
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
            let btn_ph = Button::builder()
                .label(ph)
                .tooltip_text(&t(desc_key))
                .css_classes(vec!["flat".to_string(), "placeholder-btn-small".to_string()])
                .build();
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

        let btn_save = Button::builder()
            .label(&t("explore.settings_save"))
            .css_classes(vec!["suggested-action".to_string(), "small-btn".to_string()])
            .build();
        btn_save.set_cursor_from_name(Some("pointer"));
        let btn_cancel = Button::builder()
            .label(&t("explore.settings_cancel"))
            .css_classes(vec!["flat".to_string(), "small-btn".to_string()])
            .build();
        btn_cancel.set_cursor_from_name(Some("pointer"));

        vbox_buttons.append(&btn_save);
        vbox_buttons.append(&btn_cancel);

        // Sync view content helper
        let update_view_labels = {
            let lbl_n = lbl_name.clone();
            let lbl_c = lbl_cmd.clone();
            let saved = saved_item.clone();
            move || {
                let item = saved.borrow();
                lbl_n.set_label(&item.name);
                lbl_c.set_label(&item.command);
            }
        };
        update_view_labels();

        // Wire View Mode Edit click
        let row_c = row.clone();
        let hb_edit = hbox_edit.clone();
        let ent_name = entry_edit_name.clone();
        let ent_cmd = entry_edit_cmd.clone();
        let saved = saved_item.clone();
        btn_edit.connect_clicked(move |_| {
            let current = saved.borrow();
            ent_name.set_text(&current.name);
            ent_cmd.set_text(&current.command);
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
        let saved2 = saved_item.clone();
        let update_lbls = update_view_labels.clone();
        btn_save.connect_clicked(move |_| {
            let new_name = ent_name2.text().to_string();
            let new_cmd = ent_cmd2.text().to_string();
            if !new_name.is_empty() && !new_cmd.is_empty() {
                let old_name = saved2.borrow().name.clone();
                let old_cmd = saved2.borrow().command.clone();

                {
                    let mut s = saved2.borrow_mut();
                    s.name = new_name.clone();
                    s.command = new_cmd.clone();
                }

                // Update settings file
                let mut s = babydra_common::load_explore_settings();
                if let Some(idx) = s.custom_context_items.iter().position(|i| i.name == old_name && i.command == old_cmd) {
                    s.custom_context_items[idx].name = new_name;
                    s.custom_context_items[idx].command = new_cmd;
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
    };

    // Populate existing custom options
    for item in settings.custom_context_items {
        render_option_row(&context_listbox, item);
    }

    let sep = Separator::new(Orientation::Horizontal);
    tab_context.append(&sep);

    // Form to add new option
    let form_box = Box::new(Orientation::Vertical, 8);
    tab_context.append(&form_box);

    let lbl_add_title = Label::builder()
        .label(&t("explore.settings_add_option"))
        .halign(Align::Start)
        .build();
    lbl_add_title.add_css_class("settings-section-subtitle");
    form_box.append(&lbl_add_title);

    let grid = Grid::new();
    grid.set_row_spacing(8);
    grid.set_column_spacing(12);
    form_box.append(&grid);

    let lbl_name_field = Label::new(Some(&t("explore.settings_option_name")));
    lbl_name_field.set_halign(Align::Start);
    let entry_name = Entry::builder()
        .placeholder_text(&t("explore.settings_placeholder_name"))
        .hexpand(true)
        .build();
    grid.attach(&lbl_name_field, 0, 0, 1, 1);
    grid.attach(&entry_name, 1, 0, 1, 1);

    let lbl_cmd_field = Label::new(Some(&t("explore.settings_option_command")));
    lbl_cmd_field.set_halign(Align::Start);
    
    let entry_cmd_vbox = Box::new(Orientation::Vertical, 4);
    let entry_cmd = Entry::builder()
        .placeholder_text(&t("explore.settings_placeholder_command"))
        .hexpand(true)
        .build();
    entry_cmd_vbox.append(&entry_cmd);

    // Add clickable placeholders row
    let placeholders_box = Box::new(Orientation::Horizontal, 6);
    placeholders_box.set_margin_top(2);
    let placeholders = [
        ("{path}", "explore.placeholder_path_desc"),
        ("{dir}",  "explore.placeholder_dir_desc"),
        ("{name}", "explore.placeholder_name_desc"),
        ("{stem}", "explore.placeholder_stem_desc"),
        ("{ext}",  "explore.placeholder_ext_desc"),
    ];
    for (p, desc_key) in placeholders {
        let btn_p = Button::builder()
            .label(p)
            .tooltip_text(&t(desc_key))
            .css_classes(vec!["flat".to_string(), "placeholder-btn".to_string()])
            .build();
        btn_p.set_cursor_from_name(Some("pointer"));
        let entry_cmd_c = entry_cmd.clone();
        btn_p.connect_clicked(move |_| {
            let mut pos = entry_cmd_c.position();
            entry_cmd_c.insert_text(p, &mut pos);
            entry_cmd_c.grab_focus();
        });
        placeholders_box.append(&btn_p);
    }
    entry_cmd_vbox.append(&placeholders_box);

    grid.attach(&lbl_cmd_field, 0, 1, 1, 1);
    grid.attach(&entry_cmd_vbox, 1, 1, 1, 1);

    let btn_add = Button::builder()
        .label(&t("explore.settings_add"))
        .halign(Align::End)
        .css_classes(vec!["suggested-action".to_string()])
        .build();
    btn_add.set_cursor_from_name(Some("pointer"));
    form_box.append(&btn_add);

    // Add button logic
    let entry_name_c = entry_name.clone();
    let entry_cmd_c = entry_cmd.clone();
    btn_add.connect_clicked(move |_| {
        let name_str = entry_name_c.text().to_string();
        let cmd_str = entry_cmd_c.text().to_string();
        if !name_str.is_empty() && !cmd_str.is_empty() {
            let item = babydra_common::config::settings::CustomContextItem {
                name: name_str.clone(),
                command: cmd_str.clone(),
            };
            
            // Append and save setting
            let mut s = babydra_common::load_explore_settings();
            s.custom_context_items.push(item.clone());
            babydra_common::save_explore_settings(&s);

            // Add row to UI
            render_option_row(&listbox_c, item);

            // Clear entry inputs
            entry_name_c.set_text("");
            entry_cmd_c.set_text("");
        }
    });

    tab_context
}
