use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Button, Window, Align, Notebook, Switch, ListBox, ListBoxRow, Entry, Grid, Separator};
use babydra_common::i18n::t;

pub fn show_settings_dialog(parent: &gtk4::Window, on_change_callback: impl Fn() + 'static) {
    let settings = babydra_common::load_explore_settings();

    let window = Window::builder()
        .title(&t("explore.settings"))
        .transient_for(parent)
        .modal(true)
        .resizable(true)
        .default_width(600)
        .default_height(580)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);
    window.set_child(Some(&vbox));

    let lbl_title = Label::builder()
        .label(&t("explore.settings"))
        .halign(Align::Start)
        .build();
    lbl_title.add_css_class("settings-title-label");
    vbox.append(&lbl_title);

    let notebook = Notebook::new();
    notebook.add_css_class("settings-notebook");
    vbox.append(&notebook);

    // ── Tab 1: General Settings ───────────────────────────────
    let tab_general = Box::new(Orientation::Vertical, 10);
    tab_general.set_margin_top(12);
    tab_general.set_margin_bottom(12);
    tab_general.set_margin_start(12);
    tab_general.set_margin_end(12);

    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::None);
    listbox.add_css_class("settings-listbox");
    tab_general.append(&listbox);

    // Helper to add switch row with a description
    let add_switch_row = |listbox: &ListBox, label_title: &str, label_desc: &str, active: bool, on_toggle: std::boxed::Box<dyn Fn(bool)>| {
        let row = ListBoxRow::new();
        let hbox = Box::new(Orientation::Horizontal, 12);
        hbox.set_margin_top(14);
        hbox.set_margin_bottom(14);
        hbox.set_margin_start(16);
        hbox.set_margin_end(16);

        let vbox_lbl = Box::new(Orientation::Vertical, 2);
        vbox_lbl.set_hexpand(true);

        let lbl_title = Label::builder()
            .label(label_title)
            .halign(Align::Start)
            .build();
        lbl_title.add_css_class("settings-row-title");

        let lbl_desc = Label::builder()
            .label(label_desc)
            .halign(Align::Start)
            .build();
        lbl_desc.add_css_class("settings-row-desc");

        vbox_lbl.append(&lbl_title);
        vbox_lbl.append(&lbl_desc);
        hbox.append(&vbox_lbl);

        let sw = Switch::builder()
            .active(active)
            .halign(Align::End)
            .valign(Align::Center)
            .build();
        
        sw.connect_active_notify(move |switch| {
            let state = switch.is_active();
            on_toggle(state);
        });

        hbox.append(&sw);
        row.set_child(Some(&hbox));
        listbox.append(&row);
    };

    // 1. Show hidden files
    add_switch_row(
        &listbox,
        &t("explore.toggle_hidden"),
        &t("explore.settings_toggle_hidden_desc"),
        settings.show_hidden,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.show_hidden = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 2. Preview Visible
    add_switch_row(
        &listbox,
        &t("explore.toggle_preview"),
        &t("explore.settings_toggle_preview_desc"),
        settings.preview_visible,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.preview_visible = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 3. Double click to open
    add_switch_row(
        &listbox,
        &t("explore.settings_double_click"),
        &t("explore.settings_double_click_desc"),
        settings.double_click_to_open,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.double_click_to_open = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 4. Permanent delete
    add_switch_row(
        &listbox,
        &t("explore.settings_permanent_delete"),
        &t("explore.settings_permanent_delete_desc"),
        settings.permanent_delete,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.permanent_delete = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 5. Calculate folder sizes
    add_switch_row(
        &listbox,
        &t("explore.settings_calculate_size"),
        &t("explore.settings_calculate_size_desc"),
        settings.calculate_dir_size,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.calculate_dir_size = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    let lbl_general_tab = Label::new(Some(&t("explore.settings_general")));
    notebook.append_page(&tab_general, Some(&lbl_general_tab));

    // ── Tab 2: Context Menu Configuration ──────────────────────
    let tab_context = Box::new(Orientation::Vertical, 10);
    tab_context.set_margin_top(12);
    tab_context.set_margin_bottom(12);
    tab_context.set_margin_start(12);
    tab_context.set_margin_end(12);

    let lbl_context_title = Label::builder()
        .label(&t("explore.settings_custom_options"))
        .halign(Align::Start)
        .build();
    lbl_context_title.add_css_class("settings-section-title");
    tab_context.append(&lbl_context_title);

    let scroll = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .min_content_height(180)
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
        hbox_view.set_margin_top(10);
        hbox_view.set_margin_bottom(10);
        hbox_view.set_margin_start(14);
        hbox_view.set_margin_end(14);

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
        hbox_view.append(&btn_edit);

        let btn_del = Button::from_icon_name("user-trash-symbolic");
        btn_del.set_tooltip_text(Some(&t("explore.settings_delete")));
        btn_del.add_css_class("flat");
        btn_del.add_css_class("destructive-action");
        hbox_view.append(&btn_del);

        // 2. EDIT MODE LAYOUT
        let hbox_edit = Box::new(Orientation::Horizontal, 10);
        hbox_edit.set_margin_top(10);
        hbox_edit.set_margin_bottom(10);
        hbox_edit.set_margin_start(14);
        hbox_edit.set_margin_end(14);

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
        let btn_cancel = Button::builder()
            .label(&t("explore.settings_cancel"))
            .css_classes(vec!["flat".to_string(), "small-btn".to_string()])
            .build();

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

    let lbl_context_tab = Label::new(Some(&t("explore.settings_context_menu")));
    notebook.append_page(&tab_context, Some(&lbl_context_tab));

    // ── Bottom Action Area ─────────────────────────────────────
    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let btn_close = Button::builder()
        .label(&t("explore.settings_close"))
        .css_classes(vec!["suggested-action".to_string()])
        .build();
    bbox.append(&btn_close);

    // Wire close and callback
    let on_change = std::rc::Rc::new(on_change_callback);
    let win_c = window.clone();
    let on_change_c = on_change.clone();
    btn_close.connect_clicked(move |_| {
        on_change_c();
        win_c.close();
    });

    // Also trigger on_change when window is destroyed/closed
    let on_change_destroy = on_change.clone();
    window.connect_destroy(move |_| {
        on_change_destroy();
    });

    window.present();
}
