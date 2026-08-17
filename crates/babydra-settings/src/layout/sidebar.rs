//! Sidebar helpers for the settings window (icons, labels, refresh).

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn create_sidebar_icon_for_item(id: &str, default_icon: &str) -> gtk4::Widget {
    match id {
        "wifi" => babydra_ui_kit::components::create_wifi_signal_icon(18),
        "appearance" => babydra_ui_kit::components::create_wallpaper_thumbnail_icon(18),
        "power" => babydra_ui_kit::components::create_battery_percentage_icon(18),
        "vpn" => babydra_ui_kit::components::create_vpn_shield_icon(18),
        "bluetooth" => {
            babydra_ui_kit::components::create_colored_icon_widget("bluetooth", 18, "#2563EB")
        }
        "hosts" => babydra_ui_kit::components::create_colored_icon_widget("hosts", 18, "#10B981"),
        "displays" => {
            babydra_ui_kit::components::create_colored_icon_widget("displays", 18, "#0EA5E9")
        }
        "keybinds" => babydra_ui_kit::components::create_colored_icon_widget("cog", 18, "#F97316"),
        "apps" => babydra_ui_kit::components::create_colored_icon_widget("apps", 18, "#A855F7"),
        "startup" => babydra_ui_kit::components::create_colored_icon_widget("cog", 18, "#6366F1"),
        "env" => babydra_ui_kit::components::create_colored_icon_widget("env", 18, "#06B6D4"),
        "certificates" => {
            babydra_ui_kit::components::create_colored_icon_widget("certificates", 18, "#EAB308")
        }
        "system_update" => {
            babydra_ui_kit::components::create_colored_icon_widget("system_update", 18, "#10B981")
        }
        "system" => babydra_ui_kit::components::create_colored_icon_widget("system", 18, "#3B82F6"),
        _ => babydra_ui_kit::components::create_colored_icon_widget(default_icon, 18, "#3B82F6"),
    }
}

pub type NavButtonEntry = (&'static str, gtk4::Button, &'static str, &'static str);

/// Finds and updates the icon and label text inside a sidebar Button.
pub(crate) fn update_sidebar_icon_and_label(
    id: &str,
    btn: &gtk4::Button,
    new_text: &str,
    default_icon: &str,
) {
    if let Some(child) = btn.child() {
        if let Ok(hbox) = child.downcast::<gtk4::Box>() {
            let mut widget = hbox.first_child();
            let mut is_first = true;
            while let Some(w) = widget {
                let next = w.next_sibling();
                if is_first {
                    hbox.remove(&w);
                    let new_icon = create_sidebar_icon_for_item(id, default_icon);
                    new_icon.set_valign(gtk4::Align::Center);
                    new_icon.set_halign(gtk4::Align::Center);
                    hbox.prepend(&new_icon);
                    is_first = false;
                } else if let Ok(label) = w.clone().downcast::<gtk4::Label>() {
                    label.set_text(new_text);
                    return;
                }
                widget = next;
            }
        }
    }
}

/// Public function to refresh icons and labels for all items in the sidebar.
pub(crate) fn refresh_sidebar(nav_buttons: &Rc<RefCell<Vec<NavButtonEntry>>>) {
    for (id, btn, key, icon) in nav_buttons.borrow().iter() {
        update_sidebar_icon_and_label(id, btn, &babydra_core::i18n::t(key), icon);
    }
}

/// Creates a new `sidebar category header`.
pub(crate) fn create_sidebar_category_header(key: &str) -> gtk4::Label {
    let lbl = gtk4::Label::new(Some(&babydra_core::i18n::t(key)));
    lbl.add_css_class("sidebar-section-label");
    lbl.set_halign(gtk4::Align::Start);
    lbl.set_margin_top(8);
    lbl.set_margin_bottom(2);
    lbl
}
