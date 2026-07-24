use gtk4::prelude::*;
use gtk4::{Box, Button, Label, ListBox, Orientation};
use babydra_common::models::system_update::PackageUpdate;

pub struct SystemUpdateWidget {
    pub container: Box,
    pub count_label: Label,
    pub update_all_btn: Button,
    pub refresh_btn: Button,
    pub list_box: ListBox,
}

pub fn build(updates: &[PackageUpdate]) -> SystemUpdateWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_margin_top(16);
    container.set_margin_bottom(16);
    container.set_margin_start(16);
    container.set_margin_end(16);

    // Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some("System Update"));
    title_label.add_css_class("settings-title-label");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let refresh_btn = Button::with_label("Check for Updates");
    refresh_btn.add_css_class("connect-pill-btn");

    header_box.append(&title_label);
    header_box.append(&refresh_btn);
    container.append(&header_box);

    // Hero Summary Card
    let hero_card = Box::new(Orientation::Horizontal, 16);
    hero_card.add_css_class("settings-card");
    hero_card.set_margin_bottom(12);

    let count_box = Box::new(Orientation::Vertical, 4);
    count_box.set_hexpand(true);
    let count_label = Label::new(Some(&format!("{}", updates.len())));
    count_label.add_css_class("hero-hostname");
    let sub_lbl = Label::new(Some("Updates Available"));
    sub_lbl.add_css_class("settings-row-desc");
    sub_lbl.set_halign(gtk4::Align::Start);

    count_box.append(&count_label);
    count_box.append(&sub_lbl);

    let update_all_btn = Button::with_label("Update All");
    update_all_btn.add_css_class("connect-pill-btn");
    update_all_btn.set_valign(gtk4::Align::Center);

    hero_card.append(&count_box);
    hero_card.append(&update_all_btn);
    container.append(&hero_card);

    // Package List
    let list_box = ListBox::new();
    list_box.add_css_class("settings-card");

    for pkg in updates {
        let row_box = Box::new(Orientation::Horizontal, 12);
        row_box.add_css_class("settings-card-row");
        row_box.set_margin_top(8);
        row_box.set_margin_bottom(8);
        row_box.set_margin_start(12);
        row_box.set_margin_end(12);

        let name_lbl = Label::new(Some(&pkg.name));
        name_lbl.add_css_class("settings-row-title");
        name_lbl.set_hexpand(true);
        name_lbl.set_halign(gtk4::Align::Start);

        let ver_lbl = Label::new(Some(&format!("{} -> {}", pkg.old_version, pkg.new_version)));
        ver_lbl.add_css_class("settings-row-desc");

        row_box.append(&name_lbl);
        row_box.append(&ver_lbl);
        list_box.append(&row_box);
    }

    container.append(&list_box);

    SystemUpdateWidget {
        container,
        count_label,
        update_all_btn,
        refresh_btn,
        list_box,
    }
}
