use gtk4::prelude::*;
use crate::components::card::standard::create_card;

/// Creates a switch card containing a title, subtitle, and an active switch.
pub fn create_switch_card(title: &str, subtitle: &str) -> (gtk4::Box, gtk4::Switch) {
    let card = create_card(gtk4::Orientation::Horizontal, 12);
    card.set_valign(gtk4::Align::Center);

    let label_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let status_title = gtk4::Label::new(Some(title));
    status_title.add_css_class("settings-label");
    status_title.set_halign(gtk4::Align::Start);
    
    let status_desc = gtk4::Label::new(Some(subtitle));
    status_desc.add_css_class("settings-desc");
    status_desc.set_halign(gtk4::Align::Start);
    
    label_box.append(&status_title);
    label_box.append(&status_desc);
    card.append(&label_box);

    let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    card.append(&spacer);

    let sw = crate::components::switch::create_switch(false, |_| {});
    card.append(&sw);

    (card, sw)
}
