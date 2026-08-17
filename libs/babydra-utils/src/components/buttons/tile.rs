use gtk4::prelude::*;
use std::rc::Rc;

/// Creates a panel toggle tile with active/inactive state.
pub fn create_toggle_tile(
    icon_name: &str,
    title: &str,
    subtitle: &str,
    css_class: &str,
    initial_active: bool,
    on_click: impl Fn(bool) + 'static,
) -> (gtk4::Button, gtk4::Label) {
    let btn = gtk4::Button::new();
    if !css_class.is_empty() {
        btn.add_css_class(css_class);
    } else {
        btn.add_css_class("control-tile-row");
    }
    btn.set_hexpand(true);
    btn.set_vexpand(true);
    btn.set_valign(gtk4::Align::Fill);

    let main_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    main_box.set_valign(gtk4::Align::Center);

    let circle = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    circle.add_css_class("control-icon-circle");

    if initial_active {
        btn.add_css_class("active");
        circle.add_css_class("active");
    }

    let is_dark = crate::ui::theme::is_dark_mode();
    let inactive_color = if is_dark {
        "rgba(255, 255, 255, 0.7)"
    } else {
        "rgba(28, 28, 30, 0.85)"
    };
    let color = if initial_active {
        "#ffffff"
    } else {
        inactive_color
    };
    let icon_widget = crate::ui::icon::get_icon_colored(icon_name, 14, color);
    circle.append(&icon_widget);
    main_box.append(&circle);

    let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 1);
    let title_label = gtk4::Label::new(Some(title));
    title_label.set_xalign(0.0);
    title_label.add_css_class("tile-title");
    text_box.append(&title_label);

    let sub_label = gtk4::Label::new(Some(subtitle));
    sub_label.set_xalign(0.0);
    sub_label.add_css_class("tile-subtitle");
    text_box.append(&sub_label);
    main_box.append(&text_box);

    btn.set_child(Some(&main_box));

    let circle_clone = circle.clone();
    let icon_name_str = icon_name.to_string();
    let on_click = Rc::new(on_click);

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

        if let Some(old) = circle_clone.first_child() {
            circle_clone.remove(&old);
        }
        let is_dark = crate::ui::theme::is_dark_mode();
        let inactive_color = if is_dark {
            "rgba(255, 255, 255, 0.7)"
        } else {
            "rgba(28, 28, 30, 0.85)"
        };
        let color = if is_now_active {
            "#ffffff"
        } else {
            inactive_color
        };
        let new_img = crate::ui::icon::get_icon_colored(&icon_name_str, 14, color);
        circle_clone.append(&new_img);

        on_click(is_now_active);
    });

    (btn, sub_label)
}

/// Updates a toggle tile's active state, icon circle styling, and icon color.
pub fn update_toggle_tile_state(btn: &gtk4::Button, is_active: bool, icon_name: &str) {
    if is_active {
        btn.add_css_class("active");
    } else {
        btn.remove_css_class("active");
    }

    let circle = btn
        .child()
        .and_then(|w| w.downcast::<gtk4::Box>().ok())
        .and_then(|main_box| main_box.first_child())
        .and_then(|c| c.downcast::<gtk4::Box>().ok());

    if let Some(circle) = circle {
        if is_active {
            circle.add_css_class("active");
        } else {
            circle.remove_css_class("active");
        }

        while let Some(old) = circle.first_child() {
            circle.remove(&old);
        }
        let is_dark = crate::ui::theme::is_dark_mode();
        let inactive_color = if is_dark {
            "rgba(255, 255, 255, 0.7)"
        } else {
            "rgba(28, 28, 30, 0.85)"
        };
        let color = if is_active { "#ffffff" } else { inactive_color };
        let new_img = crate::ui::icon::get_icon_colored(icon_name, 14, color);
        circle.append(&new_img);
    }
}

/// Creates a square panel toggle tile with active/inactive state.
pub fn create_square_toggle_tile(
    icon_name: &str,
    label_text: &str,
    initial_active: bool,
    on_click: impl Fn(bool) + 'static,
) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("control-square-tile");
    btn.set_size_request(56, 56);
    btn.set_halign(gtk4::Align::Center);
    btn.set_valign(gtk4::Align::Center);
    btn.set_hexpand(false);
    btn.set_vexpand(false);

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    main_box.set_valign(gtk4::Align::Center);
    main_box.set_halign(gtk4::Align::Center);

    if initial_active {
        btn.add_css_class("active");
    }

    let icon_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_container.set_halign(gtk4::Align::Center);

    let is_dark = crate::ui::theme::is_dark_mode();
    let inactive_color = if is_dark {
        "rgba(255, 255, 255, 0.8)"
    } else {
        "rgba(28, 28, 30, 0.85)"
    };
    let color = if initial_active {
        "#ffffff"
    } else {
        inactive_color
    };
    let icon_widget = crate::ui::icon::get_icon_colored(icon_name, 18, color);
    icon_container.append(&icon_widget);

    main_box.append(&icon_container);

    if !label_text.is_empty() {
        let label = gtk4::Label::new(Some(label_text));
        label.add_css_class("control-square-label");
        label.set_halign(gtk4::Align::Center);
        main_box.append(&label);
    }

    btn.set_child(Some(&main_box));

    let icon_name_str = icon_name.to_string();
    let on_click = Rc::new(on_click);

    btn.connect_clicked(move |b| {
        let is_now_active = if b.has_css_class("active") {
            b.remove_css_class("active");
            false
        } else {
            b.add_css_class("active");
            true
        };

        if let Some(old) = icon_container.first_child() {
            icon_container.remove(&old);
        }
        let is_dark = crate::ui::theme::is_dark_mode();
        let inactive_color = if is_dark {
            "rgba(255, 255, 255, 0.8)"
        } else {
            "rgba(28, 28, 30, 0.85)"
        };
        let color = if is_now_active {
            "#ffffff"
        } else {
            inactive_color
        };
        let new_img = crate::ui::icon::get_icon_colored(&icon_name_str, 18, color);
        icon_container.append(&new_img);

        on_click(is_now_active);
    });

    btn
}
