use gtk4::prelude::*;
use gtk4::{Box, Orientation, ListBox, ListBoxRow, Label, Switch, Align};
use babydra_common::i18n::t;

pub fn build_general_page() -> Box {
    let settings = babydra_common::load_explore_settings();
    let tab_general = Box::new(Orientation::Vertical, 10);

    let lbl_general_title = Label::builder()
        .label(&t("explore.settings_general"))
        .halign(Align::Start)
        .build();
    lbl_general_title.add_css_class("settings-title-label");
    tab_general.append(&lbl_general_title);

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

    tab_general
}
