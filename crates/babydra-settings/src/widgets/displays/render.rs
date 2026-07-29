use gtk4::prelude::*;
use gtk4::{Box, Button, DropDown, Label, Orientation, StringList, Switch};
use babydra_common::models::display::MonitorConfig;

pub struct DisplayCardRow {
    pub container: Box,
    pub enable_switch: Switch,
    pub resolution_dropdown: DropDown,
    pub rate_dropdown: DropDown,
    pub orientation_dropdown: DropDown,
}

pub struct DisplaysWidget {
    pub container: Box,
    pub save_btn: Button,
    pub refresh_btn: Button,
    pub card_rows: Vec<DisplayCardRow>,
}

pub fn build(monitors: &[MonitorConfig]) -> DisplaysWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Title and Actions Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some(&babydra_common::i18n::t("settings.displays_title")));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let refresh_btn = Button::with_label(&babydra_common::i18n::t("settings.refresh"));
    refresh_btn.add_css_class("connect-pill-btn");

    let save_btn = Button::with_label(&babydra_common::i18n::t("settings.save"));
    save_btn.add_css_class("suggested-action");

    header_box.append(&title_label);
    header_box.append(&refresh_btn);
    header_box.append(&save_btn);
    container.append(&header_box);

    let cards_box = Box::new(Orientation::Vertical, 12);
    cards_box.set_vexpand(true);
    cards_box.set_valign(gtk4::Align::Fill);

    let mut card_rows = Vec::new();

    for mon in monitors {
        // Single Horizontal Card Row for all monitor settings
        let card = Box::new(Orientation::Horizontal, 12);
        card.add_css_class("glass-panel");
        card.add_css_class("settings-card-row");
        card.set_valign(gtk4::Align::Center);

        // 1. Monitor Name / Model Title
        let display_title = if !mon.description.is_empty() && mon.description != mon.name {
            format!("{} ({})", mon.name, mon.description)
        } else {
            mon.name.clone()
        };
        let name_lbl = Label::new(Some(&display_title));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.set_hexpand(true);
        name_lbl.set_halign(gtk4::Align::Start);
        name_lbl.set_valign(gtk4::Align::Center);
        card.append(&name_lbl);

        // 2. Resolution Dropdown
        let res_strs: Vec<&str> = mon.available_resolutions.iter().map(|s| s.as_str()).collect();
        let res_model = StringList::new(&res_strs);
        let resolution_dropdown = DropDown::new(Some(res_model), Option::<gtk4::Expression>::None);
        resolution_dropdown.set_valign(gtk4::Align::Center);

        let cur_res_str = format!("{}x{}", mon.resolution_width, mon.resolution_height);
        if let Some(idx) = mon.available_resolutions.iter().position(|r| r == &cur_res_str) {
            resolution_dropdown.set_selected(idx as u32);
        }
        card.append(&resolution_dropdown);

        // 3. Refresh Rate Dropdown
        let rate_strs: Vec<String> = mon.available_rates.iter().map(|r| format!("{:.1} Hz", r)).collect();
        let rate_items: Vec<&str> = rate_strs.iter().map(|s| s.as_str()).collect();
        let rate_model = StringList::new(&rate_items);
        let rate_dropdown = DropDown::new(Some(rate_model), Option::<gtk4::Expression>::None);
        rate_dropdown.set_valign(gtk4::Align::Center);

        if let Some(idx) = mon.available_rates.iter().position(|r| (r - mon.refresh_rate).abs() < 0.5) {
            rate_dropdown.set_selected(idx as u32);
        }
        card.append(&rate_dropdown);

        // 4. Orientation Dropdown
        let orient_items_owned = vec![
            babydra_common::i18n::t("settings.orientation_normal"),
            babydra_common::i18n::t("settings.orientation_left"),
            babydra_common::i18n::t("settings.orientation_inverted"),
            babydra_common::i18n::t("settings.orientation_right"),
        ];
        let orient_items: Vec<&str> = orient_items_owned.iter().map(|s| s.as_str()).collect();
        let orient_model = StringList::new(&orient_items);
        let orientation_dropdown = DropDown::new(Some(orient_model), Option::<gtk4::Expression>::None);
        orientation_dropdown.set_valign(gtk4::Align::Center);

        let orient_idx = match mon.orientation.as_str() {
            "left" | "90" => 1,
            "inverted" | "180" => 2,
            "right" | "270" => 3,
            _ => 0,
        };
        orientation_dropdown.set_selected(orient_idx);
        card.append(&orientation_dropdown);

        // 5. Enable Switch
        let switch_box = Box::new(Orientation::Horizontal, 8);
        switch_box.set_valign(gtk4::Align::Center);

        let switch_lbl = Label::new(Some(&babydra_common::i18n::t("settings.on")));
        switch_lbl.add_css_class("settings-page-subtitle");
        switch_lbl.set_valign(gtk4::Align::Center);

        let enable_switch = Switch::new();
        enable_switch.set_active(mon.enabled);
        enable_switch.set_valign(gtk4::Align::Center);
        enable_switch.set_cursor_from_name(Some("pointer"));

        switch_box.append(&switch_lbl);
        switch_box.append(&enable_switch);

        card.append(&switch_box);

        cards_box.append(&card);

        card_rows.push(DisplayCardRow {
            container: card,
            enable_switch,
            resolution_dropdown,
            rate_dropdown,
            orientation_dropdown,
        });
    }

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&cards_box));

    container.append(&scroll);

    DisplaysWidget {
        container,
        save_btn,
        refresh_btn,
        card_rows,
    }
}
