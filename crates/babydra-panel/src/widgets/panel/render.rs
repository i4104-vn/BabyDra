pub use babydra_core::get_battery_info;
use gtk4::prelude::*;

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

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct NetworkWidgets {
    pub container: gtk4::Box,
    pub wifi_icon: gtk4::Image,
    pub eth_area: gtk4::DrawingArea,
}

struct EthernetActivityState {
    pub is_connected: bool,
    pub tx_lit: bool,
    pub rx_lit: bool,
}

/// Creates a unified network status widget that dynamically switches between Wi-Fi and Desktop Ethernet with TX/RX blinking dots.
pub fn create_network_widget() -> NetworkWidgets {
    let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    container.add_css_class("status-icon");
    container.set_valign(gtk4::Align::Center);
    container.set_halign(gtk4::Align::Center);

    let initial_net = babydra_core::services::system::network::get_active_network_info();

    let wifi_icon = babydra_ui_kit::ui::icon::get_icon(&initial_net.icon_name, 14);
    wifi_icon.set_valign(gtk4::Align::Center);
    wifi_icon.set_halign(gtk4::Align::Center);

    let eth_area = gtk4::DrawingArea::new();
    eth_area.set_content_width(17);
    eth_area.set_content_height(14);
    eth_area.set_valign(gtk4::Align::Center);
    eth_area.set_halign(gtk4::Align::Center);

    let is_ethernet = initial_net.network_type == babydra_core::models::ActiveNetworkType::Ethernet;
    if is_ethernet {
        wifi_icon.set_visible(false);
        eth_area.set_visible(true);
    } else {
        wifi_icon.set_visible(true);
        eth_area.set_visible(false);
    }

    container.append(&wifi_icon);
    container.append(&eth_area);

    let state = Rc::new(RefCell::new(EthernetActivityState {
        is_connected: initial_net.is_connected,
        tx_lit: true,
        rx_lit: true,
    }));

    let state_draw = state.clone();
    eth_area.set_draw_func(move |_area, cr, width, height| {
        let is_dark = babydra_ui_kit::ui::theme::is_dark_mode();
        let s = state_draw.borrow();
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

    let eth_area_timer = eth_area.clone();
    let state_timer = state.clone();
    let mut last_stats = babydra_core::services::system::network::get_net_bytes();
    let mut blink_toggle = false;
    let mut tx_burst: u8 = 0;
    let mut rx_burst: u8 = 0;

    // Fast polling timer (~200ms) for responsive upload/download LED blinking
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(200), move || {
        if eth_area_timer.root().is_none() {
            return gtk4::glib::ControlFlow::Break;
        }

        if !eth_area_timer.is_visible() {
            return gtk4::glib::ControlFlow::Continue;
        }

        let cur_stats = babydra_core::services::system::network::get_net_bytes();
        let rx_diff = cur_stats.rx_bytes.saturating_sub(last_stats.rx_bytes);
        let tx_diff = cur_stats.tx_bytes.saturating_sub(last_stats.tx_bytes);
        last_stats = cur_stats;

        // Packet threshold (> 64 bytes)
        if tx_diff > 64 {
            tx_burst = 2; // Keep blinking for at least 2 ticks
        } else {
            tx_burst = tx_burst.saturating_sub(1);
        }

        if rx_diff > 64 {
            rx_burst = 2;
        } else {
            rx_burst = rx_burst.saturating_sub(1);
        }

        let tx_active = tx_burst > 0;
        let rx_active = rx_burst > 0;

        blink_toggle = !blink_toggle;

        let mut s = state_timer.borrow_mut();
        s.is_connected = true;
        // When active: alternate solid/dim. When idle: both solid.
        s.tx_lit = if tx_active { blink_toggle } else { true };
        s.rx_lit = if rx_active { blink_toggle } else { true };

        eth_area_timer.queue_draw();

        gtk4::glib::ControlFlow::Continue
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

