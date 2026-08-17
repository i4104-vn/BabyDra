use crate::components::badge::create_icon_badge;
use gtk4::prelude::*;

/// Standard placeholder states for settings ListBox containers.
pub enum PlaceholderState<'a> {
    Disabled {
        title_key: &'a str,
        desc_key: &'a str,
        icon_name: &'a str,
    },
    Loading,
    Empty {
        title_key: &'a str,
        desc_key: Option<&'a str>,
        icon_name: &'a str,
    },
}

/// Constructs a unified ListBoxRow placeholder for disabled, loading, or empty states.
pub fn create_placeholder_row(state: PlaceholderState) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.add_css_class("settings-card-row");
    row.set_selectable(false);
    row.set_activatable(false);
    row.set_vexpand(true);
    row.set_valign(gtk4::Align::Fill);

    let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    placeholder_box.set_valign(gtk4::Align::Center);
    placeholder_box.set_halign(gtk4::Align::Center);
    placeholder_box.set_vexpand(true);
    placeholder_box.set_hexpand(true);
    placeholder_box.set_margin_top(40);
    placeholder_box.set_margin_bottom(40);

    match state {
        PlaceholderState::Disabled {
            title_key,
            desc_key,
            icon_name,
        } => {
            let badge = create_icon_badge(icon_name, 24, false);
            placeholder_box.append(&badge);

            let lbl = gtk4::Label::new(Some(&babydra_core::i18n::t(title_key)));
            lbl.add_css_class("settings-row-title");
            lbl.set_halign(gtk4::Align::Center);
            placeholder_box.append(&lbl);

            let desc = gtk4::Label::new(Some(&babydra_core::i18n::t(desc_key)));
            desc.add_css_class("settings-row-desc");
            desc.set_halign(gtk4::Align::Center);
            placeholder_box.append(&desc);
        }
        PlaceholderState::Loading => {
            let spinner = gtk4::Spinner::new();
            spinner.set_size_request(32, 32);
            spinner.set_halign(gtk4::Align::Center);
            spinner.start();
            placeholder_box.append(&spinner);

            let lbl = gtk4::Label::new(Some(&babydra_core::i18n::t("settings.loading")));
            lbl.add_css_class("settings-row-title");
            lbl.set_halign(gtk4::Align::Center);
            placeholder_box.append(&lbl);
        }
        PlaceholderState::Empty {
            title_key,
            desc_key,
            icon_name,
        } => {
            let badge = create_icon_badge(icon_name, 24, false);
            placeholder_box.append(&badge);

            let lbl = gtk4::Label::new(Some(&babydra_core::i18n::t(title_key)));
            lbl.add_css_class("settings-row-title");
            lbl.set_halign(gtk4::Align::Center);
            placeholder_box.append(&lbl);

            if let Some(desc_k) = desc_key {
                let desc = gtk4::Label::new(Some(&babydra_core::i18n::t(desc_k)));
                desc.add_css_class("settings-row-desc");
                desc.set_halign(gtk4::Align::Center);
                placeholder_box.append(&desc);
            }
        }
    }

    row.set_child(Some(&placeholder_box));
    row
}
