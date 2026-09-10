use gtk4::prelude::*;

/// Builds the compact idle logo pill content.
pub fn idle_logo_view() -> gtk4::Box {
    let default_view = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    default_view.set_valign(gtk4::Align::Center);
    default_view.set_halign(gtk4::Align::Center);
    let icon = babydra_ui_kit::ui::icon::get_icon("logo", 12);
    default_view.append(&icon);
    default_view
}
