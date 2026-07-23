//! VPN and WireGuard connections manager.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::process::Command;

use babydra_common::get_vpn_connections;

mod render;

pub fn create_vpn_widget() -> gtk4::Box {
    let (main_box, import_btn, list_box) = render::build_vpn_ui();

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

                let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
                placeholder_box.set_valign(gtk4::Align::Center);
                placeholder_box.set_halign(gtk4::Align::Center);
                placeholder_box.set_margin_top(30);
                placeholder_box.set_margin_bottom(30);

                let shield_icon = babydra_utils::ui::icon::get_icon("shield", 24);
                shield_icon.set_pixel_size(24);
                shield_icon.add_css_class("settings-row-icon");
                placeholder_box.append(&shield_icon);

                let lbl = gtk4::Label::new(Some("Chưa có cấu hình VPN nào"));
                lbl.add_css_class("settings-row-title");
                placeholder_box.append(&lbl);

                let desc = gtk4::Label::new(Some("Bấm nút 'Nhập file cấu hình' phía trên để thêm kết nối VPN mới"));
                desc.add_css_class("settings-row-desc");
                placeholder_box.append(&desc);

                row.set_child(Some(&placeholder_box));
                list_box_clone.append(&row);
                return;
            }

            for vpn in vpns.iter() {
                let row = gtk4::ListBoxRow::new();
                row.add_css_class("settings-card-row");

                let hbox = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
                hbox.set_margin_top(10);
                hbox.set_margin_bottom(10);
                hbox.set_margin_start(16);
                hbox.set_margin_end(16);

                let icon = babydra_utils::ui::icon::get_icon("shield", 16);
                icon.set_pixel_size(16);
                icon.set_valign(gtk4::Align::Center);
                icon.add_css_class("settings-row-icon");
                hbox.append(&icon);

                let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
                text_box.set_valign(gtk4::Align::Center);
                text_box.set_hexpand(true);

                let name_lbl = gtk4::Label::new(Some(&vpn.name));
                name_lbl.add_css_class("settings-row-title");
                name_lbl.set_halign(gtk4::Align::Start);
                text_box.append(&name_lbl);

                let desc_lbl = gtk4::Label::new(Some(&format!("Kiểu kết nối: {}", vpn.conn_type.to_uppercase())));
                desc_lbl.add_css_class("settings-row-desc");
                desc_lbl.set_halign(gtk4::Align::Start);
                text_box.append(&desc_lbl);

                hbox.append(&text_box);

                if vpn.active {
                    let connected_badge = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
                    connected_badge.add_css_class("connected-pill");
                    connected_badge.set_valign(gtk4::Align::Center);

                    let check_icon = babydra_utils::ui::icon::get_icon("check", 14);
                    check_icon.set_pixel_size(14);
                    connected_badge.append(&check_icon);

                    let connected_lbl = gtk4::Label::new(Some("Đang kết nối"));
                    connected_lbl.add_css_class("connected-text");
                    connected_badge.append(&connected_lbl);

                    hbox.append(&connected_badge);

                    let disconnect_btn = gtk4::Button::with_label("Ngắt");
                    disconnect_btn.set_valign(gtk4::Align::Center);
                    disconnect_btn.add_css_class("connect-pill-btn");
                    let name_clone = vpn.name.clone();
                    disconnect_btn.connect_clicked(move |_| {
                        let _ = Command::new("nmcli").args(&["connection", "down", &name_clone]).output();
                    });
                    hbox.append(&disconnect_btn);
                } else {
                    let connect_btn = gtk4::Button::with_label("Kết nối");
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
