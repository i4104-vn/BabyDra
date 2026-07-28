use gtk4::prelude::*;
use gtk4::{Box, Button, Label, ListBox, Orientation, ScrolledWindow};
use babydra_common::models::system_update::PackageUpdate;

pub struct SystemUpdateWidget {
    pub container: Box,
    pub count_label: Label,
    pub sub_label: Label,
    pub update_all_btn: Button,
    pub refresh_btn: Button,
    pub list_box: ListBox,
}

pub fn create_update_row(pkg: &PackageUpdate) -> Box {
    let row_box = Box::new(Orientation::Horizontal, 14);
    row_box.add_css_class("settings-card-row");
    row_box.set_margin_top(10);
    row_box.set_margin_bottom(10);
    row_box.set_margin_start(16);
    row_box.set_margin_end(16);

    let icon_box = Box::new(Orientation::Vertical, 0);
    icon_box.add_css_class("blue-icon-badge-sm");
    icon_box.set_valign(gtk4::Align::Center);
    icon_box.set_halign(gtk4::Align::Start);

    let icon_img = babydra_utils::ui::icon::get_icon("download", 18);
    icon_img.set_pixel_size(18);
    icon_img.set_valign(gtk4::Align::Center);
    icon_img.set_halign(gtk4::Align::Center);
    icon_img.set_vexpand(true);
    icon_box.append(&icon_img);
    row_box.append(&icon_box);

    let text_box = Box::new(Orientation::Vertical, 2);
    text_box.set_hexpand(true);
    text_box.set_valign(gtk4::Align::Center);

    let name_lbl = Label::new(Some(&pkg.name));
    name_lbl.add_css_class("settings-row-title");
    name_lbl.set_halign(gtk4::Align::Start);
    text_box.append(&name_lbl);

    let ver_lbl = Label::new(Some(&format!("{} → {}", pkg.old_version, pkg.new_version)));
    ver_lbl.add_css_class("settings-row-desc");
    ver_lbl.set_halign(gtk4::Align::Start);
    text_box.append(&ver_lbl);

    row_box.append(&text_box);
    row_box
}

pub fn create_empty_up_to_date_row() -> Box {
    let row_box = Box::new(Orientation::Horizontal, 14);
    row_box.add_css_class("settings-card-row");
    row_box.set_margin_top(16);
    row_box.set_margin_bottom(16);
    row_box.set_margin_start(16);
    row_box.set_margin_end(16);

    let icon_box = Box::new(Orientation::Vertical, 0);
    icon_box.add_css_class("blue-icon-badge-sm");
    icon_box.set_valign(gtk4::Align::Center);

    let icon_img = babydra_utils::ui::icon::get_icon("check", 18);
    icon_img.set_pixel_size(18);
    icon_img.set_valign(gtk4::Align::Center);
    icon_img.set_halign(gtk4::Align::Center);
    icon_img.set_vexpand(true);
    icon_box.append(&icon_img);
    row_box.append(&icon_box);

    let text_lbl = Label::new(Some(&babydra_common::i18n::t("settings.up_to_date")));
    text_lbl.add_css_class("settings-row-title");
    text_lbl.set_halign(gtk4::Align::Start);
    text_lbl.set_valign(gtk4::Align::Center);
    text_lbl.set_hexpand(true);

    row_box.append(&text_lbl);
    row_box
}

pub fn build(updates: &[PackageUpdate]) -> SystemUpdateWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Header
    let header_box = Box::new(Orientation::Horizontal, 12);
    let title_label = Label::new(Some(&babydra_common::i18n::t("settings.update_title")));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let refresh_btn = Button::with_label(&babydra_common::i18n::t("settings.update_check"));
    refresh_btn.add_css_class("connect-pill-btn");
    refresh_btn.set_cursor_from_name(Some("pointer"));

    header_box.append(&title_label);
    header_box.append(&refresh_btn);
    container.append(&header_box);

    // Hero Summary Card
    let hero_card = Box::new(Orientation::Horizontal, 16);
    hero_card.add_css_class("glass-panel");
    hero_card.set_margin_bottom(4);
    hero_card.set_margin_top(4);

    let count_box = Box::new(Orientation::Vertical, 4);
    count_box.set_hexpand(true);
    let count_label = Label::new(Some(&format!("{}", updates.len())));
    count_label.add_css_class("hero-hostname");
    let sub_label = Label::new(Some(&babydra_common::i18n::t("settings.updates_available")));
    sub_label.add_css_class("settings-row-desc");
    sub_label.set_halign(gtk4::Align::Start);

    count_box.append(&count_label);
    count_box.append(&sub_label);

    let update_all_btn = Button::with_label(&babydra_common::i18n::t("settings.update_all"));
    update_all_btn.add_css_class("connect-pill-btn");
    update_all_btn.set_valign(gtk4::Align::Center);
    update_all_btn.set_cursor_from_name(Some("pointer"));

    hero_card.append(&count_box);
    hero_card.append(&update_all_btn);
    container.append(&hero_card);

    // Package List Glass Card
    let glass_card = Box::new(Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    if updates.is_empty() {
        list_box.append(&create_empty_up_to_date_row());
    } else {
        for pkg in updates {
            list_box.append(&create_update_row(pkg));
        }
    }

    let scroll = ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&list_box));

    glass_card.append(&scroll);
    container.append(&glass_card);

    SystemUpdateWidget {
        container,
        count_label,
        sub_label,
        update_all_btn,
        refresh_btn,
        list_box,
    }
}
