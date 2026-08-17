use babydra_core::i18n::t;
use babydra_core::services::system::vpn::{
    connect_vpn, disconnect_vpn, get_vpn_connections, is_vpn_active_fast, VpnConn,
};
use gtk4::prelude::*;
use std::rc::Rc;
use tokio::sync::mpsc;

/// Update VPN tile icon state async.
pub fn update_vpn_tile_icon_state_async(btn: &gtk4::Button) {
    let (tx, mut rx) = mpsc::unbounded_channel::<bool>();
    std::thread::spawn(move || {
        let is_connected = is_vpn_active_fast();
        let _ = tx.send(is_connected);
    });

    let btn_clone = btn.clone();
    glib::spawn_future_local(async move {
        if let Some(is_connected) = rx.recv().await {
            if is_connected {
                if !btn_clone.has_css_class("active") {
                    btn_clone.add_css_class("active");
                }
                let active_icon =
                    babydra_ui_kit::ui::icon::get_icon_colored("shield", 18, "#ffffff");
                btn_clone.set_child(Some(&active_icon));
            } else if !btn_clone.has_css_class("popover-open") {
                btn_clone.remove_css_class("active");
                let inactive_icon = babydra_ui_kit::ui::icon::get_icon_colored(
                    "shield",
                    18,
                    "rgba(255, 255, 255, 0.8)",
                );
                btn_clone.set_child(Some(&inactive_icon));
            }
        }
    });
}

/// Creates a new `VPN tile`.
pub fn create_vpn_tile(on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>) -> gtk4::Button {
    let btn = babydra_ui_kit::components::create_colored_icon_button(
        "shield",
        18,
        "rgba(255, 255, 255, 0.8)",
        &["control-square-tile"],
        None,
        || {},
    );
    btn.set_size_request(56, 56);
    btn.set_halign(gtk4::Align::Center);
    btn.set_valign(gtk4::Align::Center);
    btn.set_hexpand(false);
    btn.set_vexpand(false);

    update_vpn_tile_icon_state_async(&btn);

    let popover = babydra_ui_kit::components::create_popover(
        &btn,
        gtk4::PositionType::Bottom,
        "media-popover",
    );
    popover.set_has_arrow(false);

    let main_box = setup_vpn_popover(&popover);

    let on_popover_toggled_c = on_popover_toggled.clone();
    let popover_c = popover.clone();
    btn.connect_clicked(move |_| {
        popover_c.popup();
    });

    let btn_c = btn.clone();
    let main_box_clone = main_box.clone();
    let on_popover_toggled_c_map = on_popover_toggled.clone();
    popover.connect_map(move |_| {
        btn_c.add_css_class("popover-open");
        btn_c.add_css_class("active");
        let active_icon = babydra_ui_kit::ui::icon::get_icon_colored("shield", 18, "#ffffff");
        btn_c.set_child(Some(&active_icon));

        if let Some(ref cb) = on_popover_toggled_c_map {
            cb(true);
        }

        refresh_vpn_popover_list(&main_box_clone, Some(btn_c.clone()));

        babydra_ui_kit::ui::animation::slide_in(
            main_box_clone.upcast_ref(),
            babydra_ui_kit::ui::animation::SlideDirection::Down,
            15,
            450,
        );
    });

    let btn_c2 = btn.clone();
    popover.connect_closed(move |_| {
        btn_c2.remove_css_class("popover-open");
        update_vpn_tile_icon_state_async(&btn_c2);

        if let Some(ref cb) = on_popover_toggled_c {
            cb(false);
        }
    });

    // Periodic sync (every 5 seconds) when mapped
    let btn_periodic = btn.clone();
    gtk4::glib::timeout_add_local(std::time::Duration::from_secs(5), move || {
        if btn_periodic.is_mapped() {
            update_vpn_tile_icon_state_async(&btn_periodic);
        }
        gtk4::glib::ControlFlow::Continue
    });

    btn
}

/// Sets up `VPN popover`.
fn setup_vpn_popover(popover: &gtk4::Popover) -> gtk4::Box {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    main_box.add_css_class("media-popover-box");
    main_box.set_margin_start(4);
    main_box.set_margin_end(4);
    popover.set_child(Some(&main_box));
    main_box
}

/// Refresh VPN popover list.
fn refresh_vpn_popover_list(main_box: &gtk4::Box, tile_btn: Option<gtk4::Button>) {
    while let Some(child) = main_box.first_child() {
        main_box.remove(&child);
    }

    main_box.set_size_request(280, -1);

    // Header
    let popover_header = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    popover_header.add_css_class("media-popover-header");
    popover_header.set_valign(gtk4::Align::Center);
    let popover_app_icon = babydra_ui_kit::ui::icon::get_icon_colored("shield", 14, "#3b82f6");
    let popover_app_name =
        gtk4::Label::new(Some(&babydra_core::i18n::t("control.vpn_connections")));
    popover_app_name.add_css_class("media-popover-app-name");
    popover_header.append(&popover_app_icon);
    popover_header.append(&popover_app_name);
    main_box.append(&popover_header);

    // Loading indicator
    let loading_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    loading_box.set_halign(gtk4::Align::Center);
    loading_box.set_margin_top(20);
    loading_box.set_margin_bottom(20);

    let spinner = gtk4::Spinner::new();
    spinner.start();
    loading_box.append(&spinner);

    let scan_label = gtk4::Label::new(Some(&t("vpn.loading")));
    scan_label.add_css_class("media-time-label");
    loading_box.append(&scan_label);
    main_box.append(&loading_box);

    let main_box_clone = main_box.clone();
    let (tx, mut rx) = mpsc::unbounded_channel::<Vec<VpnConn>>();

    std::thread::spawn(move || {
        let vpns = get_vpn_connections();
        let _ = tx.send(vpns);
    });

    glib::spawn_future_local(async move {
        if let Some(vpns) = rx.recv().await {
            build_vpn_list_ui(&main_box_clone, vpns, tile_btn.clone());
        }
    });
}

/// Builds the `VPN list ui` UI.
fn build_vpn_list_ui(main_box: &gtk4::Box, vpns: Vec<VpnConn>, tile_btn: Option<gtk4::Button>) {
    if let Some(ref btn) = tile_btn {
        update_vpn_tile_icon_state_async(btn);
    }
    while let Some(child) = main_box.first_child() {
        main_box.remove(&child);
    }

    main_box.set_size_request(280, -1);

    // Re-create Header
    let popover_header = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    popover_header.add_css_class("media-popover-header");
    popover_header.set_valign(gtk4::Align::Center);
    let popover_app_icon = babydra_ui_kit::ui::icon::get_icon_colored("shield", 14, "#3b82f6");
    let popover_app_name =
        gtk4::Label::new(Some(&babydra_core::i18n::t("control.vpn_connections")));
    popover_app_name.add_css_class("media-popover-app-name");
    popover_header.append(&popover_app_icon);
    popover_header.append(&popover_app_name);
    main_box.append(&popover_header);

    let connected_vpns: Vec<VpnConn> = vpns.iter().filter(|v| v.active).cloned().collect();
    let available_vpns: Vec<VpnConn> = vpns.iter().filter(|v| !v.active).cloned().collect();

    // Content container
    let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    content_box.set_margin_top(8);
    content_box.set_margin_bottom(8);
    content_box.set_margin_start(8);
    content_box.set_margin_end(8);

    // Section 1: Connected VPN (rendered at top without extra "CONNECTED" header text)
    if !connected_vpns.is_empty() {
        let conn_list = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
        for vpn in &connected_vpns {
            let row = create_vpn_item_row(vpn, true, main_box);
            conn_list.append(&row);
        }
        content_box.append(&conn_list);

        if !available_vpns.is_empty() {
            let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
            sep.set_margin_top(4);
            sep.set_margin_bottom(4);
            content_box.append(&sep);
        }
    }

    // Section 2: SAVED VPNS
    if available_vpns.is_empty() && connected_vpns.is_empty() {
        let empty_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
        empty_box.set_halign(gtk4::Align::Center);
        empty_box.set_margin_top(12);
        empty_box.set_margin_bottom(12);

        let empty_icon =
            babydra_ui_kit::ui::icon::get_icon_colored("shield", 24, "rgba(255, 255, 255, 0.4)");
        empty_icon.set_halign(gtk4::Align::Center);
        empty_box.append(&empty_icon);

        let empty_lbl =
            gtk4::Label::new(Some(&babydra_core::i18n::t("control.vpn_no_connections")));
        empty_lbl.add_css_class("media-time-label");
        empty_lbl.set_halign(gtk4::Align::Center);
        empty_box.append(&empty_lbl);

        let empty_sub = gtk4::Label::new(Some(&babydra_core::i18n::t(
            "control.vpn_no_connections_sub",
        )));
        empty_sub.add_css_class("settings-row-desc");
        empty_sub.set_halign(gtk4::Align::Center);
        empty_box.append(&empty_sub);

        content_box.append(&empty_box);
    } else if !available_vpns.is_empty() {
        let avail_title = gtk4::Label::new(Some(&babydra_core::i18n::t(
            "control.vpn_available_section",
        )));
        avail_title.add_css_class("audio-menu-section-title");
        avail_title.set_xalign(0.0);
        avail_title.set_margin_start(4);
        content_box.append(&avail_title);

        let list_box = gtk4::ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::None);
        list_box.add_css_class("audio-menu-popover");

        for vpn in &available_vpns {
            let row = create_vpn_item_row(vpn, false, main_box);
            list_box.append(&row);
        }

        let scroll = gtk4::ScrolledWindow::new();
        scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        scroll.set_max_content_height(180);
        scroll.set_propagate_natural_height(true);
        scroll.set_child(Some(&list_box));

        content_box.append(&scroll);
    }

    main_box.append(&content_box);
}

/// Creates a new `VPN item row`.
fn create_vpn_item_row(vpn: &VpnConn, is_connected: bool, main_box: &gtk4::Box) -> gtk4::Box {
    let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    row_box.add_css_class("clean-target-row");
    row_box.set_hexpand(true);
    row_box.set_valign(gtk4::Align::Center);
    row_box.set_margin_top(2);
    row_box.set_margin_bottom(2);

    let icon_color = if is_connected {
        "#3b82f6"
    } else {
        "rgba(255, 255, 255, 0.5)"
    };
    let shield_icon = babydra_ui_kit::ui::icon::get_icon_colored("shield", 14, icon_color);
    shield_icon.set_valign(gtk4::Align::Center);
    row_box.append(&shield_icon);

    let text_box = gtk4::Box::new(gtk4::Orientation::Vertical, 1);
    text_box.set_hexpand(true);
    text_box.set_valign(gtk4::Align::Center);

    let name_str = if vpn.name.chars().count() > 22 {
        let truncated: String = vpn.name.chars().take(22).collect();
        format!("{}...", truncated)
    } else {
        vpn.name.clone()
    };

    let name_lbl = gtk4::Label::new(Some(&name_str));
    name_lbl.add_css_class("clean-target-name");
    name_lbl.set_halign(gtk4::Align::Start);
    name_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);
    text_box.append(&name_lbl);

    let sub_str = if is_connected && !vpn.ip_address.is_empty() {
        format!("VPN • {}", vpn.ip_address)
    } else {
        vpn.conn_type.to_uppercase()
    };

    let sub_lbl = gtk4::Label::new(Some(&sub_str));
    sub_lbl.add_css_class("settings-row-desc");
    sub_lbl.set_halign(gtk4::Align::Start);
    text_box.append(&sub_lbl);

    row_box.append(&text_box);

    let action_btn = gtk4::Button::new();
    action_btn.set_valign(gtk4::Align::Center);
    action_btn.add_css_class("vpn-action-toggle");
    action_btn.set_cursor_from_name(Some("pointer"));

    if is_connected {
        action_btn.add_css_class("active");
        let icon = babydra_ui_kit::ui::icon::get_icon_colored("power", 14, "#ffffff");
        action_btn.set_child(Some(&icon));
        action_btn.set_tooltip_text(Some(&babydra_core::i18n::t("settings.disconnect")));
    } else {
        let icon =
            babydra_ui_kit::ui::icon::get_icon_colored("power", 14, "rgba(255, 255, 255, 0.6)");
        action_btn.set_child(Some(&icon));
        action_btn.set_tooltip_text(Some(&babydra_core::i18n::t("settings.connect")));
    }

    let vpn_name = vpn.name.clone();
    let main_box_c = main_box.clone();
    let action_btn_c = action_btn.clone();

    action_btn.connect_clicked(move |_| {
        action_btn_c.set_sensitive(false);
        let spinner = gtk4::Spinner::new();
        spinner.start();
        action_btn_c.set_child(Some(&spinner));

        let name = vpn_name.clone();
        let main_box_inner = main_box_c.clone();

        let (tx, mut rx) = mpsc::unbounded_channel::<()>();

        std::thread::spawn(move || {
            if is_connected {
                let _ = disconnect_vpn(&name);
            } else {
                let _ = connect_vpn(&name);
            }
            let _ = tx.send(());
        });

        glib::spawn_future_local(async move {
            let _ = rx.recv().await;
            refresh_vpn_popover_list(&main_box_inner, None);
        });
    });

    row_box.append(&action_btn);
    row_box
}
