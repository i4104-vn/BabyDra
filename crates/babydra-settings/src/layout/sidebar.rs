//! Sidebar helpers for the settings window (icons, labels, refresh).

use gtk4::prelude::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct SidebarStatus {
    pub wifi_enabled: bool,
    pub wifi_connected: bool,
    pub wifi_strength: u8,
    pub bluetooth_enabled: bool,
    pub vpn_connected: bool,
    pub battery: Option<(u32, bool)>,
}

pub(crate) fn create_sidebar_icon(id: &str, default_icon: &str) -> gtk4::Widget {
    create_sidebar_icon_with_status(id, default_icon, &SidebarStatus::default())
}

fn create_sidebar_icon_with_status(
    id: &str,
    default_icon: &str,
    status: &SidebarStatus,
) -> gtk4::Widget {
    match id {
        "wifi" => babydra_ui_kit::components::create_rssi_icon(
            status.wifi_strength as u32,
            status.wifi_enabled,
            status.wifi_connected,
            18,
            None,
        ),
        // Keep sidebar construction cheap; wallpaper thumbnails are loaded by
        // the Appearance page itself instead of blocking Settings startup.
        "appearance" => babydra_ui_kit::components::create_colored_icon("palette", 18, "#EC4899"),
        "power" => {
            let (percentage, charging) = status.battery.unwrap_or((100, false));
            babydra_ui_kit::ui::battery::create_battery_area(percentage, charging, 22, 12).upcast()
        }
        "vpn" => create_vpn_icon(status.vpn_connected),
        "bluetooth" => babydra_ui_kit::components::create_colored_icon(
            "bluetooth",
            18,
            if status.bluetooth_enabled {
                "#2563EB"
            } else {
                "#6B7280"
            },
        ),
        "hosts" => babydra_ui_kit::components::create_colored_icon("hosts", 18, "#10B981"),
        "displays" => babydra_ui_kit::components::create_colored_icon("displays", 18, "#0EA5E9"),
        "keybinds" => babydra_ui_kit::components::create_colored_icon("cog", 18, "#F97316"),
        "apps" => babydra_ui_kit::components::create_colored_icon("apps", 18, "#A855F7"),
        "startup" => babydra_ui_kit::components::create_colored_icon("cog", 18, "#6366F1"),
        "env" => babydra_ui_kit::components::create_colored_icon("env", 18, "#06B6D4"),
        "certificates" => {
            babydra_ui_kit::components::create_colored_icon("certificates", 18, "#EAB308")
        }
        "system_update" => {
            babydra_ui_kit::components::create_colored_icon("system_update", 18, "#10B981")
        }
        "recovery" => babydra_ui_kit::components::create_colored_icon("history", 18, "#EF4444"),
        "system" => babydra_ui_kit::components::create_colored_icon("system", 18, "#3B82F6"),
        _ => babydra_ui_kit::components::create_colored_icon(default_icon, 18, "#3B82F6"),
    }
}

fn create_vpn_icon(connected: bool) -> gtk4::Widget {
    let color = if connected { "#8B5CF6" } else { "#6B7280" };
    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"##
    );
    babydra_ui_kit::ui::icon::get_icon_from_svg(&svg, 18).upcast()
}

pub type NavButtonEntry = (&'static str, gtk4::Button, &'static str, &'static str);

/// Finds and updates the icon and label text inside a sidebar Button.
pub(crate) fn update_sidebar_icon(
    id: &str,
    btn: &gtk4::Button,
    new_text: &str,
    default_icon: &str,
    status: &SidebarStatus,
) {
    if let Some(child) = btn.child() {
        if let Ok(hbox) = child.downcast::<gtk4::Box>() {
            let mut widget = hbox.first_child();
            let mut is_first = true;
            while let Some(w) = widget {
                let next = w.next_sibling();
                if is_first {
                    hbox.remove(&w);
                    let new_icon = create_sidebar_icon_with_status(id, default_icon, status);
                    new_icon.set_valign(gtk4::Align::Center);
                    new_icon.set_halign(gtk4::Align::Center);
                    hbox.prepend(&new_icon);
                    is_first = false;
                } else if !new_text.is_empty() {
                    if let Ok(label) = w.clone().downcast::<gtk4::Label>() {
                        label.set_text(new_text);
                        return;
                    }
                }
                widget = next;
            }
        }
    }
}

/// Public function to refresh icons and labels for all items in the sidebar.
pub(crate) fn refresh_sidebar(nav_buttons: &Rc<RefCell<Vec<NavButtonEntry>>>) {
    for (_id, btn, key, _icon) in nav_buttons.borrow().iter() {
        update_sidebar_label(btn, &babydra_core::i18n::trans(key));
    }
}

fn update_sidebar_label(btn: &gtk4::Button, new_text: &str) {
    if let Some(child) = btn.child() {
        if let Ok(hbox) = child.downcast::<gtk4::Box>() {
            let mut widget = hbox.first_child();
            while let Some(current) = widget {
                if let Ok(label) = current.clone().downcast::<gtk4::Label>() {
                    label.set_text(new_text);
                    break;
                }
                widget = current.next_sibling();
            }
        }
    }
}

fn apply_status(nav_buttons: &Rc<RefCell<Vec<NavButtonEntry>>>, status: &SidebarStatus) {
    for (id, btn, _key, icon) in nav_buttons.borrow().iter() {
        if matches!(*id, "wifi" | "bluetooth" | "vpn" | "power") {
            update_sidebar_icon(id, btn, "", icon, status);
        }
    }
}

fn apply_wallpaper_thumbnail(nav_buttons: &Rc<RefCell<Vec<NavButtonEntry>>>, bytes: &[u8]) {
    for (id, btn, _key, _icon) in nav_buttons.borrow().iter() {
        if *id != "appearance" {
            continue;
        }
        let Some(new_thumb) = babydra_ui_kit::components::create_wp_thumb_from_bytes(bytes, 18)
        else {
            return;
        };
        if let Some(child) = btn.child() {
            if let Ok(hbox) = child.downcast::<gtk4::Box>() {
                if let Some(old_icon) = hbox.first_child() {
                    hbox.remove(&old_icon);
                    new_thumb.set_valign(gtk4::Align::Center);
                    new_thumb.set_halign(gtk4::Align::Center);
                    hbox.prepend(&new_thumb);
                }
            }
        }
        break;
    }
}

fn collect_status() -> (SidebarStatus, Option<PathBuf>) {
    let (wifi_enabled, wifi_connected, wifi_strength) =
        babydra_core::services::system::wifi::get_wifi_signal();
    let battery = babydra_core::services::system::battery::get_battery_info()
        .map(|info| (info.percentage, info.is_charging));
    let vpn_connected = babydra_core::services::system::vpn::get_vpn_connections()
        .iter()
        .any(|connection| connection.active);

    let status = SidebarStatus {
        wifi_enabled,
        wifi_connected,
        wifi_strength,
        bluetooth_enabled: babydra_core::is_bluetooth_enabled(),
        vpn_connected,
        battery,
    };
    let wallpaper_path = babydra_core::get_wallpaper().and_then(|path| {
        if path.exists() {
            Some(babydra_core::services::wallpaper::get_or_create_thumbnail(
                &path,
            ))
        } else {
            None
        }
    });

    (status, wallpaper_path)
}

/// Keeps dynamic sidebar icons current without querying system services on GTK.
pub(crate) fn start_status_updates(
    nav_buttons: Rc<RefCell<Vec<NavButtonEntry>>>,
    status: Rc<RefCell<SidebarStatus>>,
) {
    let (tx, rx) = std::sync::mpsc::channel::<(SidebarStatus, Option<PathBuf>)>();
    let (thumb_tx, thumb_rx) = std::sync::mpsc::channel::<(PathBuf, Vec<u8>)>();
    let in_flight = Rc::new(std::cell::Cell::new(false));
    let last_request = Rc::new(std::cell::Cell::new(None::<Instant>));
    let loaded_wallpaper = Rc::new(RefCell::new(None::<PathBuf>));
    let loading_wallpaper = Rc::new(RefCell::new(None::<PathBuf>));

    let nav_buttons_c = nav_buttons.clone();
    let status_c = status.clone();
    let in_flight_c = in_flight.clone();
    let last_request_c = last_request.clone();
    let loaded_wallpaper_c = loaded_wallpaper.clone();
    let loading_wallpaper_c = loading_wallpaper.clone();
    gtk4::glib::timeout_add_local(Duration::from_millis(100), move || {
        while let Ok((next_status, wallpaper_path)) = rx.try_recv() {
            in_flight_c.set(false);
            let changed = *status_c.borrow() != next_status;
            if changed {
                *status_c.borrow_mut() = next_status;
                apply_status(&nav_buttons_c, &next_status);
            }

            if wallpaper_path != *loaded_wallpaper_c.borrow()
                && wallpaper_path != *loading_wallpaper_c.borrow()
            {
                if let Some(path) = wallpaper_path {
                    *loading_wallpaper_c.borrow_mut() = Some(path.clone());
                    let thumb_tx_c = thumb_tx.clone();
                    std::thread::spawn(move || {
                        if let Ok(bytes) = std::fs::read(&path) {
                            let _ = thumb_tx_c.send((path, bytes));
                        }
                    });
                } else {
                    *loaded_wallpaper_c.borrow_mut() = None;
                }
            }
        }

        while let Ok((path, bytes)) = thumb_rx.try_recv() {
            *loading_wallpaper_c.borrow_mut() = None;
            if let Some(old_path) = loaded_wallpaper_c.borrow().as_ref() {
                if old_path == &path {
                    continue;
                }
            }
            apply_wallpaper_thumbnail(&nav_buttons_c, &bytes);
            *loaded_wallpaper_c.borrow_mut() = Some(path);
        }

        let should_request = last_request_c
            .get()
            .map(|instant| instant.elapsed() >= Duration::from_secs(2))
            .unwrap_or(true);
        if should_request && !in_flight_c.get() {
            in_flight_c.set(true);
            last_request_c.set(Some(Instant::now()));
            let tx_c = tx.clone();
            std::thread::spawn(move || {
                let _ = tx_c.send(collect_status());
            });
        }
        gtk4::glib::ControlFlow::Continue
    });
}

/// Creates a new `sidebar category header`.
pub(crate) fn create_sidebar_cat(key: &str) -> gtk4::Label {
    let lbl = gtk4::Label::new(Some(&babydra_core::i18n::trans(key)));
    lbl.add_css_class("sidebar-section-label");
    lbl.set_halign(gtk4::Align::Start);
    lbl.set_margin_top(8);
    lbl.set_margin_bottom(2);
    lbl
}
