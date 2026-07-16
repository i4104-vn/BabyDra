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

    // Theme toggle: uses colored icon, reacts to theme changes
    let is_dark = babydra_common::is_dark_mode();
    let theme_icon_name = if is_dark { "dark-mode" } else { "brightness" };
    let theme_icon_color = if is_dark { "#ffffff" } else { "rgba(255,255,255,0.8)" };
    let theme_tooltip = babydra_common::i18n::t("control.dark_mode");
    let theme_btn = babydra_utils::components::create_colored_icon_button(
        theme_icon_name,
        16,
        theme_icon_color,
        &["circle-btn"],
        Some(&theme_tooltip),
        || {
            babydra_common::set_dark_mode(!babydra_common::is_dark_mode());
        },
    );

    // Auto-update icon when theme changes
    if let Some(settings) = gtk4::Settings::default() {
        let btn_clone = theme_btn.clone();
        settings.connect_gtk_application_prefer_dark_theme_notify(move |_| {
            let dark = babydra_common::is_dark_mode();
            let name = if dark { "dark-mode" } else { "brightness" };
            let color = if dark { "#ffffff" } else { "rgba(255,255,255,0.8)" };
            let new_icon = babydra_common::icon::get_icon_colored(name, 16, color);
            btn_clone.set_child(Some(&new_icon));
        });
    }

    let settings_btn = babydra_utils::components::create_icon_button(
        "settings",
        16,
        &["circle-btn"],
        Some(&babydra_common::i18n::t("control.settings")),
        || {},
    );

    btn_box.append(&theme_btn);
    btn_box.append(&settings_btn);

    header_box.append(&title);
    header_box.append(&btn_box);

    header_box
}
