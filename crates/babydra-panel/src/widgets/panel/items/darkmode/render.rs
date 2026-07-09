use gtk4::prelude::*;

pub fn create_small_theme_toggle_tile() -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("control-square-tile");
    btn.set_hexpand(true);
    btn.set_valign(gtk4::Align::Fill);
    btn.set_vexpand(true);

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    main_box.set_valign(gtk4::Align::Center);
    main_box.set_halign(gtk4::Align::Center);

    let is_dark_init = gtk4::Settings::default()
        .map(|s| s.is_gtk_application_prefer_dark_theme())
        .unwrap_or(true);

    if is_dark_init {
        btn.add_css_class("active");
    }

    let icon_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_container.set_halign(gtk4::Align::Center);

    let initial_icon_name = if is_dark_init { "dark-mode" } else { "brightness" };
    let initial_color = if is_dark_init { "#ffffff" } else { "rgba(255, 255, 255, 0.8)" };
    let icon_widget = babydra_common::icon::get_icon_colored(initial_icon_name, 16, initial_color);
    icon_container.append(&icon_widget);

    let label = gtk4::Label::new(Some(&babydra_common::i18n::t("control.dark_mode")));
    label.add_css_class("control-square-label");
    label.set_halign(gtk4::Align::Center);

    main_box.append(&icon_container);
    main_box.append(&label);
    btn.set_child(Some(&main_box));

    btn.connect_clicked(move |b| {
        let current_dark = babydra_common::is_dark_mode();
        let new_dark = !current_dark;

        babydra_common::set_dark_mode(new_dark);

        if let Some(old) = icon_container.first_child() {
            icon_container.remove(&old);
        }
        if new_dark {
            b.add_css_class("active");
            let new_img = babydra_common::icon::get_icon_colored("dark-mode", 16, "#ffffff");
            icon_container.append(&new_img);
        } else {
            b.remove_css_class("active");
            let new_img = babydra_common::icon::get_icon_colored("brightness", 16, "rgba(255, 255, 255, 0.8)");
            icon_container.append(&new_img);
        }
    });

    btn
}
