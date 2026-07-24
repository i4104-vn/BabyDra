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
    container.set_margin_top(16);
    container.set_margin_bottom(16);
    container.set_margin_start(16);
    container.set_margin_end(16);

    // Title and Actions
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some("Displays"));
    title_label.add_css_class("settings-title-label");
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

    let mut card_rows = Vec::new();

    for mon in monitors {
        let card = Box::new(Orientation::Vertical, 12);
        card.add_css_class("settings-card");

        // Card Header
        let card_header = Box::new(Orientation::Horizontal, 8);
        let mon_title = Label::new(Some(&format!("{} ({})", mon.name, mon.description)));
        mon_title.add_css_class("settings-group-header-title");
        mon_title.set_hexpand(true);
        mon_title.set_halign(gtk4::Align::Start);

        let enable_switch = Switch::new();
        enable_switch.set_active(mon.enabled);

        card_header.append(&mon_title);
        card_header.append(&enable_switch);
        card.append(&card_header);

        // Resolution Row
        let res_row = Box::new(Orientation::Horizontal, 12);
        res_row.add_css_class("settings-card-row");
        let res_lbl = Label::new(Some("Resolution"));
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
        let rate_lbl = Label::new(Some("Refresh Rate"));
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
        let orient_lbl = Label::new(Some("Orientation"));
        orient_lbl.add_css_class("settings-row-title");
        orient_lbl.set_hexpand(true);
        orient_lbl.set_halign(gtk4::Align::Start);

        let orient_items = vec!["Normal", "Left (90°)", "Inverted (180°)", "Right (270°)"];
        let orient_model = StringList::new(&orient_items);
        let orientation_dropdown = DropDown::new(Some(orient_model), Option::<gtk4::Expression>::None);

        orient_row.append(&orient_lbl);
        orient_row.append(&orientation_dropdown);
        card.append(&orient_row);

        container.append(&card);

        card_rows.push(DisplayCardRow {
            container: card,
            enable_switch,
            resolution_dropdown,
            rate_dropdown,
            orientation_dropdown,
        });
    }

    DisplaysWidget {
        container,
        save_btn,
        refresh_btn,
        card_rows,
    }
}
