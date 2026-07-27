//! VPN and WireGuard connections manager.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::process::Command;

use babydra_common::get_vpn_connections;

mod render;

pub fn create_vpn_widget() -> gtk4::Box {
    let (main_box, _vpn_switch, import_btn, list_box) = render::build_vpn_ui();

    let state = Rc::new(RefCell::new(get_vpn_connections()));

    let render_vpns = {
        let list_box_clone = list_box.clone();
        let state_clone = state.clone();
        move || {
            while let Some(child) = list_box_clone.first_child() {
                list_box_clone.remove(&child);
            }

            let vpns = state_clone.borrow();
            if vpns.is_empty() {
                let row = gtk4::ListBoxRow::new();
                row.add_css_class("settings-card-row");
                row.set_selectable(false);
                row.set_activatable(false);
                row.set_vexpand(true);
                row.set_valign(gtk4::Align::Fill);

                let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 14);
                placeholder_box.set_valign(gtk4::Align::Center);
                placeholder_box.set_halign(gtk4::Align::Center);
                placeholder_box.set_vexpand(true);
                placeholder_box.set_hexpand(true);
                placeholder_box.set_margin_top(48);
                placeholder_box.set_margin_bottom(48);

                let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
                icon_badge.add_css_class("blue-icon-badge");
                icon_badge.set_valign(gtk4::Align::Center);
                icon_badge.set_halign(gtk4::Align::Center);

                let shield_icon = babydra_utils::ui::icon::get_icon("shield", 24);
                shield_icon.set_pixel_size(24);
                shield_icon.set_valign(gtk4::Align::Center);
                shield_icon.set_halign(gtk4::Align::Center);
                shield_icon.set_vexpand(true);
                icon_badge.append(&shield_icon);
                placeholder_box.append(&icon_badge);

                let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.vpn_no_profiles")));
                lbl.add_css_class("settings-row-title");
                placeholder_box.append(&lbl);

                let desc = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.vpn_no_profiles_sub")));
                desc.add_css_class("settings-row-desc");
                placeholder_box.append(&desc);

                row.set_child(Some(&placeholder_box));
                list_box_clone.append(&row);
                return;
            }

            for vpn in vpns.iter() {
                let row = gtk4::ListBoxRow::new();
                row.add_css_class("settings-card-row");

                let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 14);
                hbox.set_margin_top(12);
                hbox.set_margin_bottom(12);
                hbox.set_margin_start(16);
                hbox.set_margin_end(16);

                // Blue Rounded Square Badge with Shield Icon (Matching Wi-Fi)
                let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
                icon_badge.add_css_class("blue-icon-badge-sm");
                icon_badge.set_valign(gtk4::Align::Center);
                icon_badge.set_halign(gtk4::Align::Start);

                let shield_icon = babydra_utils::ui::icon::get_icon("shield", 18);
                shield_icon.set_pixel_size(18);
                shield_icon.set_valign(gtk4::Align::Center);
                shield_icon.set_halign(gtk4::Align::Center);
                shield_icon.set_vexpand(true);
                icon_badge.append(&shield_icon);
                hbox.append(&icon_badge);

                // VPN Title + Type (Aligned Left)
                let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
                text_box.set_valign(gtk4::Align::Center);
                text_box.set_halign(gtk4::Align::Start);
                text_box.set_hexpand(true);

                let name_lbl = gtk4::Label::new(Some(&vpn.name));
                name_lbl.add_css_class("settings-row-title");
                name_lbl.set_halign(gtk4::Align::Start);
                text_box.append(&name_lbl);

                let desc_lbl = gtk4::Label::new(Some(&format!("Type: {}", vpn.conn_type.to_uppercase())));
                desc_lbl.add_css_class("settings-row-desc");
                desc_lbl.set_halign(gtk4::Align::Start);
                text_box.append(&desc_lbl);

                hbox.append(&text_box);

                if vpn.active {
                    // Connected Checkmark Badge (Matching Wi-Fi)
                    let check_badge = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                    check_badge.add_css_class("active-check-badge");
                    check_badge.set_valign(gtk4::Align::Center);

                    let check_icon = babydra_utils::ui::icon::get_icon("check", 14);
                    check_icon.set_pixel_size(14);
                    check_badge.append(&check_icon);
                    hbox.append(&check_badge);

                    let disconnect_btn = gtk4::Button::with_label("Disconnect");
                    disconnect_btn.set_valign(gtk4::Align::Center);
                    disconnect_btn.add_css_class("connect-pill-btn");
                    let name_clone = vpn.name.clone();
                    disconnect_btn.connect_clicked(move |_| {
                        let _ = Command::new("nmcli").args(&["connection", "down", &name_clone]).output();
                    });
                    hbox.append(&disconnect_btn);
                } else {
                    let connect_btn = gtk4::Button::with_label("Connect");
                    connect_btn.set_valign(gtk4::Align::Center);
                    connect_btn.add_css_class("connect-pill-btn");
                    let name_clone = vpn.name.clone();
                    connect_btn.connect_clicked(move |_| {
                        let _ = Command::new("nmcli").args(&["connection", "up", &name_clone]).output();
                    });
                    hbox.append(&connect_btn);
                }

                row.set_child(Some(&hbox));
                list_box_clone.append(&row);
            }
        }
    };

    let refresh_vpns = {
        let state_clone = state.clone();
        let render_clone = render_vpns.clone();
        move || {
            *state_clone.borrow_mut() = get_vpn_connections();
            render_clone();
        }
    };

    // Load initial
    let refresh_init = refresh_vpns.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        refresh_init();
        glib::ControlFlow::Break
    });

    // Refresh periodic
    let refresh_periodic = refresh_vpns.clone();
    glib::timeout_add_local(std::time::Duration::from_secs(4), move || {
        refresh_periodic();
        glib::ControlFlow::Continue
    });

    // Handle config file import
    let list_box_parent = list_box.clone();
    let refresh_on_import = refresh_vpns.clone();
    import_btn.connect_clicked(move |_| {
        if let Some(win) = list_box_parent.root().and_then(|r| r.downcast::<gtk4::Window>().ok()) {
            let file_dialog = gtk4::FileDialog::new();
            file_dialog.set_title("Open VPN Profile");

            let filter = gtk4::FileFilter::new();
            filter.set_name(Some("VPN Configurations (*.ovpn, *.conf)"));
            filter.add_pattern("*.ovpn");
            filter.add_pattern("*.conf");
            file_dialog.set_default_filter(Some(&filter));

            let refresh_cb = refresh_on_import.clone();
            file_dialog.open(Some(&win), None::<&gio::Cancellable>, move |res| {
                if let Ok(file) = res {
                    if let Some(path) = file.path() {
                        let path_str = path.to_string_lossy().to_string();
                        let type_str = if path_str.ends_with(".ovpn") { "openvpn" } else { "wireguard" };
                        let _ = Command::new("nmcli")
                            .args(&["connection", "import", "type", type_str, "file", &path_str])
                            .output();
                        
                        refresh_cb();
                    }
                }
            });
        }
    });

    main_box
}
