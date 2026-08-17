use gtk4::prelude::*;

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
    let icon = crate::ui::icon::get_icon(icon_name, 24);
    btn.set_child(Some(&icon));
    btn.set_halign(gtk4::Align::End);
    btn.set_valign(gtk4::Align::End);
    btn
}
