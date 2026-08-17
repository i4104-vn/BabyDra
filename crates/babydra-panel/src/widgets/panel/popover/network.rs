use babydra_ui_kit::components::popovers::hover::{
    build_hover_popover_card as build_popover_card, HoverPopoverRow as PopoverRow,
};
use gtk4::prelude::*;
use std::rc::Rc;

/// Returns the current `speed color class`.
fn get_speed_color_class(bytes_per_sec: f64) -> &'static str {
    if bytes_per_sec > 1_048_576.0 {
        "speed-high"
    } else if bytes_per_sec > 102_400.0 {
        "speed-medium"
    } else {
        "speed-low"
    }
}

/// Builds the `network update fn` UI.
pub fn build_network_update_fn(net_popover: &gtk4::Popover) -> Rc<dyn Fn()> {
    let net_popover_c = net_popover.clone();

    Rc::new(move || {
        let (enabled, ssid) = babydra_core::helper::wifi::get_wifi_state();
        let speed = babydra_core::helper::network::get_network_speed();
        let local_ip = babydra_core::helper::network::get_local_ip();

        let rx_cls = get_speed_color_class(speed.rx_speed);
        let tx_cls = get_speed_color_class(speed.tx_speed);

        let rows = if !enabled {
            vec![PopoverRow::new("Status", "Disabled", None)]
        } else if ssid == "Disconnected" || ssid == "Off" {
            vec![PopoverRow::new("Status", "Disconnected", None)]
        } else {
            vec![
                PopoverRow::new("SSID", &ssid, None),
                PopoverRow::new("IP Address", &local_ip, None),
                PopoverRow::new(
                    "Download",
                    &format!(
                        "↓ {}",
                        babydra_core::helper::network::format_speed(speed.rx_speed)
                    ),
                    Some(rx_cls),
                ),
                PopoverRow::new(
                    "Upload",
                    &format!(
                        "↑ {}",
                        babydra_core::helper::network::format_speed(speed.tx_speed)
                    ),
                    Some(tx_cls),
                ),
            ]
        };

        let card = build_popover_card("Network Connection", rows);
        net_popover_c.set_child(Some(&card));
    })
}
