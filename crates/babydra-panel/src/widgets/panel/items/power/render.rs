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

    let lang_btn = gtk4::Button::new();
    lang_btn.add_css_class("circle-btn");
    lang_btn.set_tooltip_text(Some(&babydra_common::i18n::t("control.switch_language")));
    
    let current_lang = babydra_common::i18n::get_locale().to_uppercase();
    let lang_lbl = gtk4::Label::new(Some(&current_lang));
    lang_lbl.add_css_class("control-lang-label");
    lang_btn.set_child(Some(&lang_lbl));

    let lang_lbl_clone = lang_lbl.clone();
    lang_btn.connect_clicked(move |_| {
        let new_locale = if babydra_common::i18n::get_locale() == "vi" { "en" } else { "vi" };
        babydra_common::i18n::set_locale(new_locale);
        lang_lbl_clone.set_text(&new_locale.to_uppercase());

        let notif_title = babydra_common::i18n::t("control.lang_changed_title");
        let notif_msg = babydra_common::i18n::t("control.lang_changed_msg");

        let _ = std::process::Command::new("notify-send")
            .args(&["-i", "preferences-desktop-locale", &notif_title, &notif_msg])
            .spawn();
    });

    let settings_btn = gtk4::Button::new();
    settings_btn.add_css_class("circle-btn");
    let settings_icon = babydra_common::icon::get_icon("settings", 16);
    settings_btn.set_child(Some(&settings_icon));
    settings_btn.connect_clicked(|_| {
        println!("Settings window triggered...");
    });

    let power_off = create_shutdown_button();

    btn_box.append(&lang_btn);
    btn_box.append(&settings_btn);
    btn_box.append(&power_off);

    header_box.append(&title);
    header_box.append(&btn_box);

    header_box
}

fn create_shutdown_button() -> gtk4::Button {
    let power_off = gtk4::Button::new();
    power_off.add_css_class("circle-btn");
    power_off.add_css_class("power-btn");
    let power_icon = babydra_common::icon::get_icon("power", 16);
    power_off.set_child(Some(&power_icon));
    power_off.connect_clicked(|_| {
        babydra_common::poweroff();
    });
    power_off
}
