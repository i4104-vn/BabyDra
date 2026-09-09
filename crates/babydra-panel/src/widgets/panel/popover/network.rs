use babydra_core::models::ActiveNetworkType;
use babydra_ui_kit::components::popovers::{TooltipPopover, TooltipRow};
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

/// Builds the network speed indicator UI.
pub fn build_network_update(net_popover: &gtk4::Popover) -> Rc<dyn Fn()> {
    let net_popover_c = net_popover.clone();

    Rc::new(move || {
        let active_net = babydra_core::services::system::network::get_active_network_info();
        let speed = babydra_core::services::system::network::get_network_speed();

        let rx_cls = get_speed_color_class(speed.rx_speed);
        let tx_cls = get_speed_color_class(speed.tx_speed);

        let rows = if !active_net.is_connected {
            vec![TooltipRow::new("Status", "Disconnected", None)]
        } else {
            let type_label = match active_net.network_type {
                ActiveNetworkType::Ethernet => "Ethernet",
                ActiveNetworkType::Wifi => "Wi-Fi",
                ActiveNetworkType::Disconnected => "Network",
            };
            let name_key = match active_net.network_type {
                ActiveNetworkType::Wifi => "SSID",
                _ => "Connection",
            };

            vec![
                TooltipRow::new("Type", type_label, None),
                TooltipRow::new(name_key, &active_net.name, None),
                TooltipRow::new("IP Address", &active_net.ip_address, None),
                TooltipRow::new(
                    "Download",
                    &format!(
                        "↓ {}",
                        babydra_core::services::system::network::format_speed(speed.rx_speed)
                    ),
                    Some(rx_cls),
                ),
                TooltipRow::new(
                    "Upload",
                    &format!(
                        "↑ {}",
                        babydra_core::services::system::network::format_speed(speed.tx_speed)
                    ),
                    Some(tx_cls),
                ),
            ]
        };

        let card = TooltipPopover::build_card("Network Connection", &rows);
        net_popover_c.set_child(Some(&card));
    })
}
