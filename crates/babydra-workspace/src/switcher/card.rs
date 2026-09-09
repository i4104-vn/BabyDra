use crate::models::Workspace;
use gtk4::prelude::*;

/// Builds an individual workspace card button widget.
pub fn build_workspace_card(
    ws: &Workspace,
    on_click: impl Fn(u32) + 'static,
) -> gtk4::Button {
    let btn = gtk4::Button::new();
    btn.add_css_class("switcher-card");
    btn.add_css_class("workspace-card");
    if ws.is_active {
        btn.add_css_class("active");
    }

    let card_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    card_box.set_valign(gtk4::Align::Center);
    card_box.set_halign(gtk4::Align::Center);
    card_box.set_size_request(160, 120);

    // Number badge
    let num_lbl = gtk4::Label::new(Some(&ws.id.to_string()));
    num_lbl.add_css_class("workspace-card-number");
    card_box.append(&num_lbl);

    // Title
    let title_lbl = gtk4::Label::new(Some(&ws.name));
    title_lbl.add_css_class("workspace-card-title");
    card_box.append(&title_lbl);

    // Active dot indicator
    let dot = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    dot.add_css_class("workspace-card-dot");
    if ws.is_active {
        dot.add_css_class("active");
    }
    card_box.append(&dot);

    btn.set_child(Some(&card_box));

    let target_id = ws.id;
    btn.connect_clicked(move |_| {
        on_click(target_id);
    });

    btn
}
