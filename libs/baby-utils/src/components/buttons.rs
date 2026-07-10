use gtk4::prelude::*;
use std::rc::Rc;

/// Creates a standard interactive button.
pub fn create_button(label: &str) -> gtk4::Button {
    let btn = gtk4::Button::with_label(label);
    btn.add_css_class("baby-button");
    btn
}

/// Creates a highlighted/suggested action button (blue gradient/accent).
pub fn create_accent_button(label: &str) -> gtk4::Button {
    let btn = gtk4::Button::with_label(label);
    btn.add_css_class("suggested-action");
    btn
}

/// Creates a Floating Action Button (FAB) matching the circular blue "+" button.
pub fn create_fab(icon_name: &str) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("baby-fab");
    let icon = gtk4::Image::from_icon_name(icon_name);
    btn.set_child(Some(&icon));
    btn.set_halign(gtk4::Align::End);
    btn.set_valign(gtk4::Align::End);
    btn
}

/// Creates an icon-only button.
pub fn create_icon_button(icon_name: &str, size: i32, css_class: &str) -> gtk4::Button {
    let btn = gtk4::Button::new();
    if !css_class.is_empty() {
        btn.add_css_class(css_class);
    }
    let icon = babydra_common::icon::get_icon(icon_name, size);
    btn.set_child(Some(&icon));
    btn
}

/// Creates an icon + label button.
pub fn create_icon_label_button(icon_name: &str, label_text: &str, css_class: &str) -> gtk4::Button {
    let btn = gtk4::Button::new();
    if !css_class.is_empty() {
        btn.add_css_class(css_class);
    }
    let content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    let icon = babydra_common::icon::get_icon(icon_name, 16);
    let label = gtk4::Label::new(Some(label_text));
    content.append(&icon);
    content.append(&label);
    btn.set_child(Some(&content));
    btn
}

/// Creates a panel toggle tile with active/inactive state.
pub fn create_toggle_tile(
    icon_name: &str,
    title: &str,
    subtitle: &str,
    css_class: &str,
    initial_active: bool,
    on_click: impl Fn(bool) + 'static,
) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("control-tile-row");
    if !css_class.is_empty() {
        btn.add_css_class(css_class);
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

    let color = if initial_active { "#ffffff" } else { "rgba(255, 255, 255, 0.7)" };
    let icon_widget = babydra_common::icon::get_icon_colored(icon_name, 14, color);
    circle.append(&icon_widget);
    main_box.append(&circle);

    let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 1);
    let title_label = gtk4::Label::new(Some(title));
    title_label.set_xalign(0.0);
    title_label.add_css_class("tile-title");
    text_box.append(&title_label);

    if !subtitle.is_empty() {
        let sub_label = gtk4::Label::new(Some(subtitle));
        sub_label.set_xalign(0.0);
        sub_label.add_css_class("tile-subtitle");
        text_box.append(&sub_label);
    }
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
        let color = if is_now_active { "#ffffff" } else { "rgba(255, 255, 255, 0.7)" };
        let new_img = babydra_common::icon::get_icon_colored(&icon_name_str, 14, color);
        circle_clone.append(&new_img);

        on_click(is_now_active);
    });

    btn
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

    let color = if initial_active { "#ffffff" } else { "rgba(255, 255, 255, 0.8)" };
    let icon_widget = babydra_common::icon::get_icon_colored(icon_name, 18, color);
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
        let color = if is_now_active { "#ffffff" } else { "rgba(255, 255, 255, 0.8)" };
        let new_img = babydra_common::icon::get_icon_colored(&icon_name_str, 18, color);
        icon_container.append(&new_img);

        on_click(is_now_active);
    });

    btn
}
