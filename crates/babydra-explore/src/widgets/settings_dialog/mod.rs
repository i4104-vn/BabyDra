use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Button, Window, Align, Notebook, Switch, ListBox, ListBoxRow};
use babydra_common::i18n::t;

pub fn show_settings_dialog(parent: &gtk4::Window, on_change_callback: impl Fn() + 'static) {
    let settings = babydra_common::load_explore_settings();

    let window = Window::builder()
        .title(&t("explore.settings"))
        .transient_for(parent)
        .modal(true)
        .resizable(true)
        .default_width(550)
        .default_height(550)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let lbl_title = Label::builder()
        .label(&t("explore.settings"))
        .halign(Align::Start)
        .build();
    lbl_title.add_css_class("settings-title-label");
    vbox.append(&lbl_title);

    let notebook = Notebook::new();
    vbox.append(&notebook);

    // ── Tab 1: General Settings ───────────────────────────────
    let tab_general = Box::new(Orientation::Vertical, 10);
    tab_general.set_margin_top(10);
    tab_general.set_margin_bottom(10);
    tab_general.set_margin_start(10);
    tab_general.set_margin_end(10);

    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::None);
    listbox.add_css_class("settings-listbox");
    tab_general.append(&listbox);

    // Helper to add switch row
    let add_switch_row = |listbox: &ListBox, label_text: &str, active: bool, on_toggle: std::boxed::Box<dyn Fn(bool)>| {
        let row = ListBoxRow::new();
        let hbox = Box::new(Orientation::Horizontal, 12);
        hbox.set_margin_top(12);
        hbox.set_margin_bottom(12);
        hbox.set_margin_start(12);
        hbox.set_margin_end(12);

        let lbl = Label::builder()
            .label(label_text)
            .halign(Align::Start)
            .hexpand(true)
            .build();
        hbox.append(&lbl);

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
        &t("explore.toggle_hidden"), // "Ẩn/hiện tệp ẩn"
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
        &t("explore.toggle_preview"), // "Ẩn/hiện xem trước"
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
    tab_context.set_margin_top(10);
    tab_context.set_margin_bottom(10);
    tab_context.set_margin_start(10);
    tab_context.set_margin_end(10);

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
        .min_content_height(150)
        .build();
    tab_context.append(&scroll);

    let context_listbox = ListBox::new();
    context_listbox.set_selection_mode(gtk4::SelectionMode::None);
    context_listbox.add_css_class("settings-listbox");
    scroll.set_child(Some(&context_listbox));

    // Helper to render custom option row
    let listbox_c = context_listbox.clone();
    let render_option_row = move |listbox: &ListBox, item: babydra_common::config::settings::CustomContextItem| {
        let row = ListBoxRow::new();
        let hbox = Box::new(Orientation::Horizontal, 10);
        hbox.set_margin_top(8);
        hbox.set_margin_bottom(8);
        hbox.set_margin_start(12);
        hbox.set_margin_end(12);

        let vbox_text = Box::new(Orientation::Vertical, 2);
        let lbl_name = Label::builder()
            .label(&item.name)
            .halign(Align::Start)
            .build();
        lbl_name.add_css_class("settings-item-name");
        
        let lbl_cmd = Label::builder()
            .label(&item.command)
            .halign(Align::Start)
            .build();
        lbl_cmd.add_css_class("settings-item-command");
        
        vbox_text.append(&lbl_name);
        vbox_text.append(&lbl_cmd);
        hbox.append(&vbox_text);

        // Spacer to push delete button to right
        let spacer = Box::new(Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        hbox.append(&spacer);

        let btn_del = Button::from_icon_name("user-trash-symbolic");
        btn_del.set_tooltip_text(Some(&t("explore.settings_delete")));
        btn_del.add_css_class("flat");
        btn_del.add_css_class("destructive-action");
        hbox.append(&btn_del);

        row.set_child(Some(&hbox));

        // Delete logic
        let name_val = item.name.clone();
        let cmd_val = item.command.clone();
        let listbox_c2 = listbox.clone();
        let row_c = row.clone();
        btn_del.connect_clicked(move |_| {
            listbox_c2.remove(&row_c);
            let mut s = babydra_common::load_explore_settings();
            s.custom_context_items.retain(|i| i.name != name_val || i.command != cmd_val);
            babydra_common::save_explore_settings(&s);
        });

        listbox.append(&row);
    };

    // Populate existing custom options
    for item in settings.custom_context_items {
        render_option_row(&context_listbox, item);
    }

    let sep = gtk4::Separator::new(Orientation::Horizontal);
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

    let grid = gtk4::Grid::new();
    grid.set_row_spacing(8);
    grid.set_column_spacing(12);
    form_box.append(&grid);

    let lbl_name_field = Label::new(Some(&t("explore.settings_option_name")));
    lbl_name_field.set_halign(Align::Start);
    let entry_name = gtk4::Entry::builder()
        .placeholder_text(&t("explore.settings_placeholder_name"))
        .hexpand(true)
        .build();
    grid.attach(&lbl_name_field, 0, 0, 1, 1);
    grid.attach(&entry_name, 1, 0, 1, 1);

    let lbl_cmd_field = Label::new(Some(&t("explore.settings_option_command")));
    lbl_cmd_field.set_halign(Align::Start);
    let entry_cmd = gtk4::Entry::builder()
        .placeholder_text(&t("explore.settings_placeholder_command"))
        .hexpand(true)
        .build();
    grid.attach(&lbl_cmd_field, 0, 1, 1, 1);
    grid.attach(&entry_cmd, 1, 1, 1, 1);

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

    let close_text = if babydra_common::i18n::get_locale() == "vi" { "Đóng" } else { "Close" };
    let btn_close = Button::builder()
        .label(close_text)
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
