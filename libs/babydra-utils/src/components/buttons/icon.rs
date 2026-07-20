use gtk4::prelude::*;

/// Creates a fully generic icon button reusable across all crates.
pub fn create_icon_button(
    icon_name: &str,
    size: i32,
    css_classes: &[&str],
    tooltip: Option<&str>,
    on_click: impl Fn() + 'static,
) -> gtk4::Button {
    let btn = gtk4::Button::new();
    for cls in css_classes {
        if !cls.is_empty() {
            btn.add_css_class(cls);
        }
    }
    if let Some(tip) = tooltip {
        btn.set_tooltip_text(Some(tip));
    }
    let icon = crate::ui::icon::get_icon(icon_name, size);
    btn.set_child(Some(&icon));
    btn.connect_clicked(move |_| on_click());
    btn
}

/// Creates a generic icon button using a **colored** icon.
pub fn create_colored_icon_button(
    icon_name: &str,
    size: i32,
    color: &str,
    css_classes: &[&str],
    tooltip: Option<&str>,
    on_click: impl Fn() + 'static,
) -> gtk4::Button {
    let btn = gtk4::Button::new();
    for cls in css_classes {
        if !cls.is_empty() {
            btn.add_css_class(cls);
        }
    }
    if let Some(tip) = tooltip {
        btn.set_tooltip_text(Some(tip));
    }
    let icon = crate::ui::icon::get_icon_colored(icon_name, size, color);
    btn.set_child(Some(&icon));
    btn.connect_clicked(move |_| on_click());
    btn
}

/// Creates an icon + label button.
pub fn create_icon_label_button(icon_name: &str, label_text: &str, css_class: &str) -> gtk4::Button {
    let btn = gtk4::Button::new();
    if !css_class.is_empty() {
        btn.add_css_class(css_class);
    }
    let content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    let icon = crate::ui::icon::get_icon(icon_name, 16);
    let label = gtk4::Label::new(Some(label_text));
    content.append(&icon);
    content.append(&label);
    btn.set_child(Some(&content));
    btn
}

/// Creates a generic sidebar-style item button with an icon and label.
pub fn create_sidebar_item_button(
    name: &str,
    icon_name: &str,
    css_class: &str,
    on_click: impl Fn() + 'static,
) -> gtk4::Button {
    let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    hbox.set_margin_start(8);
    hbox.set_margin_end(8);
    hbox.set_margin_top(1);
    hbox.set_margin_bottom(1);

    let img = crate::ui::icon::get_icon(icon_name, 18);

    let lbl = gtk4::Label::builder()
        .label(name)
        .halign(gtk4::Align::Start)
        .hexpand(true)
        .build();

    hbox.append(&img);
    hbox.append(&lbl);

    let btn = gtk4::Button::builder()
        .child(&hbox)
        .css_classes(vec![css_class.to_string(), "flat".to_string()])
        .build();

    btn.connect_clicked(move |_| on_click());
    btn
}
