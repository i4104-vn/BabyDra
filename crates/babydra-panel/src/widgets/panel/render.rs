use super::state::NetworkWidgets;
use babydra_core::models::{ActiveNetworkType, EthernetActivityState};
pub use babydra_core::services::system::battery::get_battery_info;
use babydra_core::services::system::network::{subscribe as subscribe_network, NetworkSnapshot};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Creates a new `battery widget`.
pub fn create_battery_w() -> Option<gtk4::DrawingArea> {
    get_battery_info()?;

    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_content_width(24);
    drawing_area.set_content_height(14);
    drawing_area.set_valign(gtk4::Align::Center);
    drawing_area.set_halign(gtk4::Align::Center);

    drawing_area.set_draw_func(move |_area, cr, width, height| {
        let bat_info = match get_battery_info() {
            Some(info) => info,
            None => return,
        };

        let is_dark = babydra_ui_kit::ui::theme::is_dark_mode();
        babydra_ui_kit::ui::battery::draw_cairo_battery(
            cr,
            width as f64,
            height as f64,
            bat_info.percentage,
            bat_info.is_charging,
            is_dark,
        );
    });

    Some(drawing_area)
}

/// Creates a unified network status widget that dynamically switches between Wi-Fi and Desktop Ethernet with TX/RX blinking dots.
pub fn create_network_widget() -> NetworkWidgets {
    let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    container.set_valign(gtk4::Align::Center);
    container.set_halign(gtk4::Align::Center);

    let wifi_icon = babydra_ui_kit::ui::icon::get_icon("wifi", 14);
    wifi_icon.set_valign(gtk4::Align::Center);
    wifi_icon.set_halign(gtk4::Align::Center);

    let eth_area = gtk4::DrawingArea::new();
    eth_area.set_content_width(16);
    eth_area.set_content_height(14);
    eth_area.set_valign(gtk4::Align::Center);
    eth_area.set_halign(gtk4::Align::Center);

    container.append(&wifi_icon);
    container.append(&eth_area);

    let state = Rc::new(RefCell::new(EthernetActivityState {
        is_connected: false,
        tx_lit: true,
        rx_lit: true,
    }));

    let state_draw = state.clone();
    eth_area.set_draw_func(move |_, cr, width, height| {
        let s = state_draw.borrow();
        let is_dark = babydra_ui_kit::ui::theme::is_dark_mode();
        babydra_ui_kit::ui::ethernet::draw_cairo_ethernet_desktop(
            cr,
            width as f64,
            height as f64,
            is_dark,
            s.is_connected,
            s.tx_lit,
            s.rx_lit,
        );
    });

    // Subscribe to the unified network service
    let net_rx = subscribe_network();
    let eth_area_timer = eth_area.clone();
    let state_timer = state.clone();
    let wifi_icon_clone = wifi_icon.clone();
    let mut last_icon_name = String::new();

    net_rx.attach(None, move |snapshot: NetworkSnapshot| {
        let active_info = snapshot.active_info;
        let is_ethernet = active_info.network_type == ActiveNetworkType::Ethernet;

        if is_ethernet {
            wifi_icon_clone.set_visible(false);
            eth_area_timer.set_visible(true);
        } else {
            wifi_icon_clone.set_visible(true);
            eth_area_timer.set_visible(false);
            if last_icon_name != active_info.icon_name {
                babydra_ui_kit::ui::icon::set_image_from_icon(
                    &wifi_icon_clone,
                    &active_info.icon_name,
                    14,
                );
                last_icon_name = active_info.icon_name.clone();
            }
            wifi_icon_clone.set_opacity(if active_info.is_connected { 1.0 } else { 0.5 });
        }

        let mut s = state_timer.borrow_mut();
        s.is_connected = active_info.is_connected;

        // Blink TX/RX based on network speed
        let tx_active = snapshot.tx_speed > 1024.0; // > 1 KB/s
        let rx_active = snapshot.rx_speed > 1024.0;

        // Simple blink toggle
        static mut BLINK_TOGGLE: bool = false;
        unsafe { BLINK_TOGGLE = !BLINK_TOGGLE };
        let blink = unsafe { BLINK_TOGGLE };

        s.tx_lit = if tx_active { blink } else { true };
        s.rx_lit = if rx_active { blink } else { true };

        eth_area_timer.queue_draw();

        glib::ControlFlow::Continue
    });

    NetworkWidgets {
        container,
        wifi_icon,
        eth_area,
    }
}

/// Builds the panel status indicators row.
pub fn build_status_row() -> (
    gtk4::Box,
    gtk4::Button,
    gtk4::Label,
    gtk4::Image,
    NetworkWidgets,
    gtk4::Image,
    Option<gtk4::DrawingArea>,
) {
    let status_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    status_box.add_css_class("status-indicators-box");

    let status_button = gtk4::Button::new();
    status_button.add_css_class("panel-status-btn");

    let inner_layout = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);

    let vpn_icon = babydra_ui_kit::ui::icon::get_icon("shield", 14);
    vpn_icon.add_css_class("status-icon");
    vpn_icon.set_visible(false);

    let net_widgets = create_network_widget();

    let vol_icon = if super::items::volume::is_muted() {
        babydra_ui_kit::ui::icon::get_icon("volume-mute", 14)
    } else {
        babydra_ui_kit::ui::icon::get_icon("volume", 14)
    };
    vol_icon.add_css_class("status-icon");

    inner_layout.append(&vpn_icon);
    inner_layout.append(&net_widgets.container);
    inner_layout.append(&vol_icon);

    let bat_widget = create_battery_w();
    if let Some(ref bat_area) = bat_widget {
        bat_area.add_css_class("status-icon");
        inner_layout.append(bat_area);
    }

    status_button.set_child(Some(&inner_layout));

    let separator = gtk4::Label::new(Some("│"));
    separator.add_css_class("capsule-separator");

    (
        status_box,
        status_button,
        separator,
        vol_icon,
        net_widgets,
        vpn_icon,
        bat_widget,
    )
}