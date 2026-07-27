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

    // Title and Actions
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some(&babydra_common::i18n::t("settings.displays_title")));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let refresh_btn = Button::with_label("Refresh");
    refresh_btn.add_css_class("connect-pill-btn");

    let save_btn = Button::with_label("Save");
    save_btn.add_css_class("connect-pill-btn");

    header_box.append(&title_label);
    header_box.append(&refresh_btn);
    header_box.append(&save_btn);
    container.append(&header_box);

    let cards_box = Box::new(Orientation::Vertical, 16);
    cards_box.set_vexpand(true);
    cards_box.set_valign(gtk4::Align::Fill);

    let mut card_rows = Vec::new();

    for mon in monitors {
        let card = Box::new(Orientation::Vertical, 0);
        card.add_css_class("glass-panel");

        // Monitor Name and Enable Switch
        let header = Box::new(Orientation::Horizontal, 12);
        header.add_css_class("settings-card-row");

        let name_lbl = Label::new(Some(&mon.name));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.set_hexpand(true);
        name_lbl.set_halign(gtk4::Align::Start);

        let enable_switch = Switch::new();
        enable_switch.set_active(mon.enabled);

        header.append(&name_lbl);
        header.append(&enable_switch);
        card.append(&header);

        // Resolution Row
        let res_row = Box::new(Orientation::Horizontal, 12);
        res_row.add_css_class("settings-card-row");
        let res_lbl = Label::new(Some(&babydra_common::i18n::t("settings.display_resolution")));
        res_lbl.add_css_class("settings-row-title");
        res_lbl.set_hexpand(true);
        res_lbl.set_halign(gtk4::Align::Start);

        let res_strs: Vec<&str> = mon.available_resolutions.iter().map(|s| s.as_str()).collect();
        let res_model = StringList::new(&res_strs);
        let resolution_dropdown = DropDown::new(Some(res_model), Option::<gtk4::Expression>::None);

        res_row.append(&res_lbl);
        res_row.append(&resolution_dropdown);
        card.append(&res_row);

        // Refresh Rate Row
        let rate_row = Box::new(Orientation::Horizontal, 12);
        rate_row.add_css_class("settings-card-row");
        let rate_lbl = Label::new(Some(&babydra_common::i18n::t("settings.display_refresh_rate")));
        rate_lbl.add_css_class("settings-row-title");
        rate_lbl.set_hexpand(true);
        rate_lbl.set_halign(gtk4::Align::Start);

        let rate_strs: Vec<String> = mon.available_rates.iter().map(|r| format!("{:.1} Hz", r)).collect();
        let rate_items: Vec<&str> = rate_strs.iter().map(|s| s.as_str()).collect();
        let rate_model = StringList::new(&rate_items);
        let rate_dropdown = DropDown::new(Some(rate_model), Option::<gtk4::Expression>::None);

        rate_row.append(&rate_lbl);
        rate_row.append(&rate_dropdown);
        card.append(&rate_row);

        // Orientation Row
        let orient_row = Box::new(Orientation::Horizontal, 12);
        orient_row.add_css_class("settings-card-row");
        let orient_lbl = Label::new(Some(&babydra_common::i18n::t("settings.display_orientation")));
        orient_lbl.add_css_class("settings-row-title");
        orient_lbl.set_hexpand(true);
        orient_lbl.set_halign(gtk4::Align::Start);

        let orient_items = vec!["Normal", "Left (90°)", "Inverted (180°)", "Right (270°)"];
        let orient_model = StringList::new(&orient_items);
        let orientation_dropdown = DropDown::new(Some(orient_model), Option::<gtk4::Expression>::None);

        orient_row.append(&orient_lbl);
        orient_row.append(&orientation_dropdown);
        card.append(&orient_row);

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
