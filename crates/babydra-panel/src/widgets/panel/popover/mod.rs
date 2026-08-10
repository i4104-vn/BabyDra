pub mod vpn;
pub mod network;
pub mod volume;
pub mod battery;

use gtk4::prelude::*;
use std::rc::Rc;
use babydra_utils::components::popovers::hover::attach_hover_popover;

#[allow(dead_code)]
pub struct StatusPopovers {
    pub vpn_popover: gtk4::Popover,
    pub net_popover: gtk4::Popover,
    pub vol_popover: gtk4::Popover,
    pub bat_popover_opt: Option<gtk4::Popover>,
    pub update_volume_popover: Rc<dyn Fn()>,
}

pub fn setup_status_popovers(
    vol_icon: &gtk4::Image,
    net_icon: &gtk4::Image,
    vpn_icon: &gtk4::Image,
    bat_widget: &Option<gtk4::DrawingArea>,
) -> StatusPopovers {
    let vpn_popover = babydra_utils::components::create_popover(vpn_icon, gtk4::PositionType::Bottom, "status-popover");
    let net_popover = babydra_utils::components::create_popover(net_icon, gtk4::PositionType::Bottom, "status-popover");
    let vol_popover = babydra_utils::components::create_popover(vol_icon, gtk4::PositionType::Bottom, "status-popover");

    let bat_popover_opt = if let Some(ref bat_area) = bat_widget {
        let bat_pop = babydra_utils::components::create_popover(bat_area, gtk4::PositionType::Bottom, "status-popover");
        Some(bat_pop)
    } else {
        None
    };

    let update_vpn_tooltip = vpn::build_vpn_update_fn(vpn_icon, &vpn_popover);
    let update_network_tooltip = network::build_network_update_fn(&net_popover);
    let update_volume_popover = volume::build_volume_update_fn(vol_icon, &vol_popover);
    let update_battery_popover = battery::build_battery_update_fn(&bat_popover_opt);

    if babydra_common::services::system::vpn::get_active_vpn_fast().is_some() {
        vpn_icon.set_visible(true);
    } else {
        vpn_icon.set_visible(false);
    }

    attach_hover_popover(vpn_icon, &vpn_popover, update_vpn_tooltip.clone());
    attach_hover_popover(net_icon, &net_popover, update_network_tooltip.clone());
    attach_hover_popover(vol_icon, &vol_popover, update_volume_popover.clone());

    if let Some(ref bat_area) = bat_widget {
        if let Some(ref bat_pop) = bat_popover_opt {
            attach_hover_popover(bat_area, bat_pop, update_battery_popover.clone());
        }
    }

    // Timer loop for updates
    let vpn_pop_t = vpn_popover.clone();
    let net_pop_t = net_popover.clone();
    let vol_pop_t = vol_popover.clone();
    let bat_pop_t = bat_popover_opt.clone();

    let update_vpn_t = update_vpn_tooltip.clone();
    let update_net_t = update_network_tooltip.clone();
    let update_vol_t = update_volume_popover.clone();
    let update_bat_t = update_battery_popover.clone();

    let bat_widget_timer = bat_widget.clone();
    let vpn_icon_timer = vpn_icon.clone();

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(2000), move || {
        if vpn_pop_t.is_visible() {
            update_vpn_t();
        }
        if net_pop_t.is_visible() {
            update_net_t();
        }
        if vol_pop_t.is_visible() {
            update_vol_t();
        }
        if let Some(ref bp) = bat_pop_t {
            if bp.is_visible() {
                update_bat_t();
            }
        }

        let vpn_active = babydra_common::services::system::vpn::get_active_vpn_fast().is_some();
        if vpn_icon_timer.is_visible() != vpn_active {
            vpn_icon_timer.set_visible(vpn_active);
        }

        if let Some(ref bat_area) = bat_widget_timer {
            bat_area.queue_draw();
        }
        gtk4::glib::ControlFlow::Continue
    });

    StatusPopovers {
        vpn_popover,
        net_popover,
        vol_popover,
        bat_popover_opt,
        update_volume_popover,
    }
}
