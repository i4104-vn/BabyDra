//! Wi-Fi popover UI (network list, connect forms, connecting state).
//! Split out of `render.rs` to keep the tile builder focused.

use super::{connect_wifi_async, scan_networks};
use babydra_core::i18n::t;
use gtk4::prelude::*;
use tokio::sync::mpsc;

pub(crate) fn setup_wifi_popover(
    popover: &gtk4::Popover,
    sub_label: gtk4::Label,
    left_btn: gtk4::Button,
    circle: gtk4::Box,
    icon_widget: gtk4::Image,
) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    main_box.add_css_class("wifi-popover-box");
    popover.set_child(Some(&main_box));

    let popover_clone = popover.clone();
    let sub_label_clone = sub_label.clone();
    let left_btn_clone = left_btn.clone();
    let circle_clone = circle.clone();
    let icon_widget_clone = icon_widget.clone();

    popover.connect_map(move |_| {
        refresh_wifi_popover_list(
            &main_box,
            sub_label_clone.clone(),
            left_btn_clone.clone(),
            circle_clone.clone(),
            icon_widget_clone.clone(),
            popover_clone.clone(),
        );
    });
}

/// Refresh wifi popover list.
pub(crate) fn refresh_wifi_popover_list(
    main_box: &gtk4::Box,
    sub_label: gtk4::Label,
    left_btn: gtk4::Button,
    circle: gtk4::Box,
    icon_widget: gtk4::Image,
    popover: gtk4::Popover,
) {
    while let Some(child) = main_box.first_child() {
        main_box.remove(&child);
    }

    main_box.set_size_request(260, -1);
    main_box.add_css_class("audio-menu-popover");

    let title = gtk4::Label::new(Some(&t("wifi.networks")));
    title.add_css_class("audio-menu-section-title");
    title.set_xalign(0.0);
    main_box.append(&title);

    let scanning_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    scanning_box.set_halign(gtk4::Align::Center);
    scanning_box.set_margin_top(15);
    scanning_box.set_margin_bottom(15);

    let spinner = gtk4::Spinner::new();
    spinner.start();
    scanning_box.append(&spinner);

    let scan_label = gtk4::Label::new(Some(&t("wifi.scanning_networks")));
    scanning_box.append(&scan_label);
    main_box.append(&scanning_box);

    let main_box_clone = main_box.clone();
    let sub_label_clone = sub_label.clone();
    let left_btn_clone = left_btn.clone();
    let circle_clone = circle.clone();
    let icon_widget_clone = icon_widget.clone();
    let popover_clone = popover.clone();

    let (tx, mut rx) =
        mpsc::unbounded_channel::<Option<Vec<babydra_core::models::wifi::WifiNetwork>>>();

    std::thread::spawn(move || {
        let nets = scan_networks();

        let _ = tx.send(Some(nets));
    });

    glib::spawn_future_local(async move {
        if let Some(Some(nets)) = rx.recv().await {
            build_wifi_list_ui(
                &main_box_clone,
                nets,
                sub_label_clone.clone(),
                left_btn_clone.clone(),
                circle_clone.clone(),
                icon_widget_clone.clone(),
                popover_clone.clone(),
            );
        }
    });
}

/// Builds the `wifi list ui` UI.
fn build_wifi_list_ui(
    main_box: &gtk4::Box,
    networks: Vec<babydra_core::models::wifi::WifiNetwork>,

    sub_label: gtk4::Label,
    left_btn: gtk4::Button,
    circle: gtk4::Box,
    icon_widget: gtk4::Image,
    popover: gtk4::Popover,
) {
    while let Some(child) = main_box.first_child() {
        main_box.remove(&child);
    }

    main_box.set_size_request(260, -1);
    main_box.add_css_class("audio-menu-popover");

    let title = gtk4::Label::new(Some(&t("wifi.networks")));
    title.add_css_class("audio-menu-section-title");
    title.set_xalign(0.0);
    main_box.append(&title);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    for net in networks {
        let ssid = net.ssid;
        let security = net.security;
        let is_connected = net.is_connected;
        let row_btn = gtk4::Button::new();
        row_btn.add_css_class("audio-menu-item-btn");
        if is_connected {
            row_btn.add_css_class("active");
        }

        let item_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        item_box.set_valign(gtk4::Align::Center);

        let icon_color = if is_connected {
            "#ffffff"
        } else {
            "rgba(255, 255, 255, 0.5)"
        };
        let wifi_icon = babydra_ui_kit::components::create_wifi_signal_icon_for_network(
            net.signal,
            is_connected,
            14,
            Some(icon_color),
        );
        item_box.append(&wifi_icon);

        let name_label = gtk4::Label::new(Some(&ssid));
        name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        name_label.set_halign(gtk4::Align::Start);
        item_box.append(&name_label);

        let is_secured = security != "open";
        if is_secured {
            let lock_icon =
                babydra_ui_kit::ui::icon::get_icon_colored("lock", 12, "rgba(255, 255, 255, 0.4)");
            item_box.append(&lock_icon);
        }

        if is_connected {
            let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
            spacer.set_hexpand(true);
            item_box.append(&spacer);

            let check_label = gtk4::Label::new(Some("✓"));
            check_label.add_css_class("audio-menu-item-check");
            item_box.append(&check_label);
        }

        row_btn.set_child(Some(&item_box));

        let ssid_clone = ssid.clone();
        let security_clone = security.clone();
        let is_saved = net.is_saved;

        let main_box_c = main_box.clone();
        let sub_label_c = sub_label.clone();
        let left_btn_c = left_btn.clone();
        let circle_c = circle.clone();
        let icon_widget_c = icon_widget.clone();
        let popover_c = popover.clone();

        row_btn.connect_clicked(move |_| {
            if is_connected {
                return;
            }

            if is_saved || security_clone == "open" {
                show_connecting_state(&main_box_c, &ssid_clone);
                connect_wifi_async(
                    &ssid_clone,
                    None,
                    None,
                    sub_label_c.clone(),
                    left_btn_c.clone(),
                    circle_c.clone(),
                    icon_widget_c.clone(),
                    popover_c.clone(),
                );
            } else {
                show_credentials_form(
                    &main_box_c,
                    &ssid_clone,
                    &security_clone,
                    sub_label_c.clone(),
                    left_btn_c.clone(),
                    circle_c.clone(),
                    icon_widget_c.clone(),
                    popover_c.clone(),
                );
            }
        });

        list_box.append(&row_btn);
    }

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Never);
    scroll.set_max_content_height(200);
    scroll.set_propagate_natural_height(true);
    scroll.set_child(Some(&list_box));

    main_box.append(&scroll);
}

/// Shows the `connecting state`.
fn show_connecting_state(main_box: &gtk4::Box, ssid: &str) {
    while let Some(child) = main_box.first_child() {
        main_box.remove(&child);
    }

    main_box.set_size_request(260, -1);
    main_box.add_css_class("audio-menu-popover");

    let label = gtk4::Label::new(Some(&t("wifi.connecting_to").replace("{}", ssid)));
    label.add_css_class("audio-menu-section-title");
    label.set_margin_bottom(10);
    main_box.append(&label);

    let spinner = gtk4::Spinner::new();
    spinner.start();
    spinner.set_halign(gtk4::Align::Center);
    main_box.append(&spinner);
}

/// Shows the `credentials form`.
fn show_credentials_form(
    main_box: &gtk4::Box,
    ssid: &str,
    security: &str,
    sub_label: gtk4::Label,
    left_btn: gtk4::Button,
    circle: gtk4::Box,
    icon_widget: gtk4::Image,
    popover: gtk4::Popover,
) {
    while let Some(child) = main_box.first_child() {
        main_box.remove(&child);
    }

    main_box.set_size_request(260, -1);
    main_box.add_css_class("audio-menu-popover");

    let title = gtk4::Label::new(Some(&t("wifi.connect_to").replace("{}", ssid)));
    title.add_css_class("audio-menu-section-title");
    title.set_xalign(0.0);
    main_box.append(&title);

    let form_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    form_box.set_margin_start(6);
    form_box.set_margin_end(6);

    let username_entry = if security == "8021x" {
        let entry = gtk4::Entry::new();
        entry.set_placeholder_text(Some(&t("common.username")));
        entry.add_css_class("wifi-input-field");
        form_box.append(&entry);
        Some(entry)
    } else {
        None
    };

    let password_entry = gtk4::Entry::new();
    password_entry.set_placeholder_text(Some(&t("common.password")));
    password_entry.set_visibility(false);
    password_entry.add_css_class("wifi-input-field");
    form_box.append(&password_entry);

    let button_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    button_box.add_css_class("wifi-button-row");
    button_box.set_homogeneous(true);

    let cancel_btn = gtk4::Button::new();
    cancel_btn.set_label(&t("common.cancel"));
    cancel_btn.add_css_class("wifi-btn-secondary");

    let connect_btn = gtk4::Button::new();
    connect_btn.set_label(&t("common.connect"));
    connect_btn.add_css_class("wifi-btn-primary");

    button_box.append(&cancel_btn);
    button_box.append(&connect_btn);
    form_box.append(&button_box);
    main_box.append(&form_box);

    let sub_label_c = sub_label.clone();
    let left_btn_c = left_btn.clone();
    let circle_c = circle.clone();
    let icon_widget_c = icon_widget.clone();
    let popover_c = popover.clone();
    let main_box_c = main_box.clone();

    cancel_btn.connect_clicked(move |_| {
        refresh_wifi_popover_list(
            &main_box_c,
            sub_label_c.clone(),
            left_btn_c.clone(),
            circle_c.clone(),
            icon_widget_c.clone(),
            popover_c.clone(),
        );
    });

    let main_box_c2 = main_box.clone();
    let ssid_str2 = ssid.to_string();
    connect_btn.connect_clicked(move |_| {
        let user = username_entry.as_ref().map(|e| e.text().to_string());
        let pass = Some(password_entry.text().to_string());

        show_connecting_state(&main_box_c2, &ssid_str2);
        connect_wifi_async(
            &ssid_str2,
            user,
            pass,
            sub_label.clone(),
            left_btn.clone(),
            circle.clone(),
            icon_widget.clone(),
            popover.clone(),
        );
    });
}
