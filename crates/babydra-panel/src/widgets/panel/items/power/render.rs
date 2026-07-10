use gtk4::prelude::*;

pub fn create_header_row() -> gtk4::Box {
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    header_box.set_hexpand(true);

    let title = gtk4::Label::new(Some(&babydra_common::i18n::t("control.title")));
    title.add_css_class("control-center-title");
    title.set_xalign(0.0);
    title.set_hexpand(true);

    let btn_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    btn_box.set_halign(gtk4::Align::End);

    let theme_btn = baby_utils::components::create_theme_toggle_button();

    let settings_btn = baby_utils::components::create_icon_button("settings", 16, "circle-btn");
    settings_btn.connect_clicked(|_| {
    });

    let power_off = create_shutdown_button();

    btn_box.append(&theme_btn);
    btn_box.append(&settings_btn);
    btn_box.append(&power_off);

    header_box.append(&title);
    header_box.append(&btn_box);

    header_box
}

fn create_shutdown_button() -> gtk4::Button {
    let power_off = baby_utils::components::create_icon_button("power", 16, "circle-btn power-btn");
    power_off.connect_clicked(|_| {
        babydra_common::poweroff();
    });
    power_off
}
