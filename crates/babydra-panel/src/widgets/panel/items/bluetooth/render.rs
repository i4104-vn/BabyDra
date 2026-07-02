use gtk4::prelude::*;

pub fn create_bluetooth_tile() -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("control-tile-row");
    btn.set_hexpand(true);
    btn.set_vexpand(true);
    btn.set_valign(gtk4::Align::Fill);

    let main_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    main_box.set_valign(gtk4::Align::Center);

    let circle = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    circle.add_css_class("control-icon-circle");
    
    let is_active = false;
    let initial_color = if is_active { "#ffffff" } else { "rgba(255, 255, 255, 0.7)" };
    let icon_widget = babydra_common::icon::get_icon_colored("bluetooth", 14, initial_color);
    circle.append(&icon_widget);
    main_box.append(&circle);

    let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 1);
    let title_label = gtk4::Label::new(Some(&babydra_common::i18n::t("control.bluetooth")));
    title_label.set_xalign(0.0);
    title_label.add_css_class("tile-title");

    let sub_label = gtk4::Label::new(Some(&babydra_common::i18n::t("control.not_connected")));
    sub_label.set_xalign(0.0);
    sub_label.add_css_class("tile-subtitle");

    text_box.append(&title_label);
    text_box.append(&sub_label);
    main_box.append(&text_box);

    btn.set_child(Some(&main_box));

    let circle_clone = circle.clone();
    let icon_widget_clone = icon_widget.clone();

    btn.connect_clicked(move |b| {
        let is_now_active = if b.has_css_class("active") {
            b.remove_css_class("active");
            circle_clone.remove_css_class("active");
            false
        } else {
            b.add_css_class("active");
            circle_clone.add_css_class("active");
            true
        };

        let color = if is_now_active { "#ffffff" } else { "rgba(255, 255, 255, 0.7)" };
        let new_img = babydra_common::icon::get_icon_colored("bluetooth", 14, color);
        if let Some(paintable) = new_img.paintable() {
            icon_widget_clone.set_paintable(Some(&paintable));
        }
    });

    btn
}
