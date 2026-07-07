use gtk4::prelude::*;
use std::rc::Rc;
use tokio::sync::mpsc;
use super::{get_wifi_state, scan_networks, known_networks, connect_wifi_async};

pub fn create_wifi_tile(on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>) -> gtk4::Box {
    let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    container.add_css_class("control-tile-container");
    container.set_hexpand(false);
    
    let left_btn = gtk4::Button::new();
    left_btn.add_css_class("control-tile-left-btn");
    left_btn.set_hexpand(false);
    
    let main_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    main_box.set_valign(gtk4::Align::Center);
    
    let circle = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    circle.add_css_class("control-icon-circle");
    
    let (is_active, ssid) = get_wifi_state();
    if is_active {
        left_btn.add_css_class("active");
        circle.add_css_class("active");
    }
    
    let color = if is_active { "#ffffff" } else { "rgba(255, 255, 255, 0.7)" };
    let icon_widget = babydra_common::icon::get_icon_colored("wifi", 14, color);
    circle.append(&icon_widget);
    main_box.append(&circle);
    
    let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 1);
    let title_label = gtk4::Label::new(Some(&babydra_common::i18n::t("control.network")));
    title_label.set_xalign(0.0);
    title_label.add_css_class("tile-title");
    
    let sub_label = gtk4::Label::new(Some(&ssid));
    sub_label.set_xalign(0.0);
    sub_label.add_css_class("tile-subtitle");
    
    text_box.append(&title_label);
    text_box.append(&sub_label);
    main_box.append(&text_box);
    left_btn.set_child(Some(&main_box));
    
    let right_btn = gtk4::Button::new();
    right_btn.add_css_class("control-tile-right-btn");
    let arrow_icon = babydra_common::icon::get_icon_colored("go-next-symbolic", 12, "rgba(255, 255, 255, 0.7)");
    right_btn.set_child(Some(&arrow_icon));
    
    let popover = gtk4::Popover::new();
    popover.add_css_class("taskbar-popover");
    popover.set_parent(&container);
    popover.set_position(gtk4::PositionType::Right);
    popover.set_has_arrow(false);
    
    setup_wifi_popover(&popover, sub_label.clone(), left_btn.clone(), circle.clone(), icon_widget.clone());
    
    let on_popover_toggled_c = on_popover_toggled.clone();
    let popover_c1 = popover.clone();
    let right_btn_clone = right_btn.clone();
    right_btn.connect_clicked(move |_| {
        popover_c1.popup();
        if let Some(ref cb) = on_popover_toggled_c {
            cb(true);
        }
        let left_icon = babydra_common::icon::get_icon_colored("go-previous-symbolic", 12, "rgba(255, 255, 255, 0.7)");
        right_btn_clone.set_child(Some(&left_icon));
    });

    let right_btn_c2 = right_btn.clone();
    let on_popover_toggled_c2 = on_popover_toggled.clone();
    popover.connect_closed(move |_| {
        if let Some(ref cb) = on_popover_toggled_c2 {
            cb(false);
        }
        let right_icon = babydra_common::icon::get_icon_colored("go-next-symbolic", 12, "rgba(255, 255, 255, 0.7)");
        right_btn_c2.set_child(Some(&right_icon));
    });
    
    let circle_c = circle.clone();
    let icon_widget_c = icon_widget.clone();
    let sub_label_c = sub_label.clone();
    left_btn.connect_clicked(move |b| {
        let current_active = b.has_css_class("active");
        let new_active = !current_active;
        if new_active {
            b.add_css_class("active");
            circle_c.add_css_class("active");
            babydra_common::helper::wifi::set_wifi_enabled(true);
            sub_label_c.set_text("Scanning...");
        } else {
            b.remove_css_class("active");
            circle_c.remove_css_class("active");
            babydra_common::helper::wifi::set_wifi_enabled(false);
            sub_label_c.set_text("Off");
        }
        let color = if new_active { "#ffffff" } else { "rgba(255, 255, 255, 0.7)" };
        let new_img = babydra_common::icon::get_icon_colored("wifi", 14, color);
        if let Some(paintable) = new_img.paintable() {
            icon_widget_c.set_paintable(Some(&paintable));
        }
    });
    
    container.append(&left_btn);
    container.append(&right_btn);
    container
}

fn setup_wifi_popover(
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

fn refresh_wifi_popover_list(
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

    let title = gtk4::Label::new(Some("Wi-Fi Networks"));
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

    let scan_label = gtk4::Label::new(Some("Scanning for networks..."));
    scanning_box.append(&scan_label);
    main_box.append(&scanning_box);

    let main_box_clone = main_box.clone();
    let sub_label_clone = sub_label.clone();
    let left_btn_clone = left_btn.clone();
    let circle_clone = circle.clone();
    let icon_widget_clone = icon_widget.clone();
    let popover_clone = popover.clone();

    let (tx, mut rx) = mpsc::unbounded_channel::<Option<(Vec<(String, String, String, bool)>, Vec<String>)>>();

    std::thread::spawn(move || {
        let nets = scan_networks();
        let known = known_networks();
        let _ = tx.send(Some((nets, known)));
    });

    glib::spawn_future_local(async move {
        if let Some(Some((nets, known))) = rx.recv().await {
            build_wifi_list_ui(
                &main_box_clone,
                nets,
                known,
                sub_label_clone.clone(),
                left_btn_clone.clone(),
                circle_clone.clone(),
                icon_widget_clone.clone(),
                popover_clone.clone(),
            );
        }
    });
}

fn build_wifi_list_ui(
    main_box: &gtk4::Box,
    networks: Vec<(String, String, String, bool)>,
    known: Vec<String>,
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

    let title = gtk4::Label::new(Some("Wi-Fi Networks"));
    title.add_css_class("audio-menu-section-title");
    title.set_xalign(0.0);
    main_box.append(&title);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    for (ssid, security, _signal, is_connected) in networks {
        let row_btn = gtk4::Button::new();
        row_btn.add_css_class("audio-menu-item-btn");
        if is_connected {
            row_btn.add_css_class("active");
        }

        let item_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        item_box.set_valign(gtk4::Align::Center);

        let icon_color = if is_connected { "#ffffff" } else { "rgba(255, 255, 255, 0.5)" };
        let wifi_icon = babydra_common::icon::get_icon_colored("wifi", 14, icon_color);
        item_box.append(&wifi_icon);

        let name_label = gtk4::Label::new(Some(&ssid));
        name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        name_label.set_halign(gtk4::Align::Start);
        item_box.append(&name_label);

        let is_secured = security != "open";
        if is_secured {
            let lock_icon = babydra_common::icon::get_icon_colored("lock", 12, "rgba(255, 255, 255, 0.4)");
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
        let is_saved = known.contains(&ssid);
        
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

fn show_connecting_state(main_box: &gtk4::Box, ssid: &str) {
    while let Some(child) = main_box.first_child() {
        main_box.remove(&child);
    }
    
    main_box.set_size_request(260, -1);
    main_box.add_css_class("audio-menu-popover");

    let label = gtk4::Label::new(Some(&format!("Connecting to {}...", ssid)));
    label.add_css_class("audio-menu-section-title");
    label.set_margin_bottom(10);
    main_box.append(&label);

    let spinner = gtk4::Spinner::new();
    spinner.start();
    spinner.set_halign(gtk4::Align::Center);
    main_box.append(&spinner);
}

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

    let title = gtk4::Label::new(Some(&format!("Connect to {}", ssid)));
    title.add_css_class("audio-menu-section-title");
    title.set_xalign(0.0);
    main_box.append(&title);

    let form_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    form_box.set_margin_start(6);
    form_box.set_margin_end(6);

    let username_entry = if security == "8021x" {
        let entry = gtk4::Entry::new();
        entry.set_placeholder_text(Some("Username"));
        entry.add_css_class("wifi-input-field");
        form_box.append(&entry);
        Some(entry)
    } else {
        None
    };

    let password_entry = gtk4::Entry::new();
    password_entry.set_placeholder_text(Some("Password"));
    password_entry.set_visibility(false);
    password_entry.add_css_class("wifi-input-field");
    form_box.append(&password_entry);

    let button_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    button_box.add_css_class("wifi-button-row");
    button_box.set_homogeneous(true);

    let cancel_btn = gtk4::Button::new();
    cancel_btn.set_label("Cancel");
    cancel_btn.add_css_class("wifi-btn-secondary");

    let connect_btn = gtk4::Button::new();
    connect_btn.set_label("Connect");
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
