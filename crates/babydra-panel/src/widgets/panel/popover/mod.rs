pub mod battery;
pub mod network;
pub mod volume;
pub mod vpn;

use babydra_ui_kit::components::popovers::TooltipPopover;
use gtk4::prelude::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct StatusPopovers {
    pub vpn_popover: gtk4::Popover,
    pub net_popover: gtk4::Popover,
    pub vol_popover: gtk4::Popover,
    pub bat_popover_opt: Option<gtk4::Popover>,
    pub update_volume_popover: Rc<dyn Fn()>,
}

impl StatusPopovers {
    pub fn popdown_all(&self) {
        self.vpn_popover.popdown();
        self.net_popover.popdown();
        self.vol_popover.popdown();
        if let Some(ref bp) = self.bat_popover_opt {
            bp.popdown();
        }
    }
}

pub fn setup_status_popover(
    vol_icon: &gtk4::Image,
    net_widgets: &super::render::NetworkWidgets,
    vpn_icon: &gtk4::Image,
    bat_widget: &Option<gtk4::DrawingArea>,
    control_center_window: Rc<std::cell::RefCell<Option<gtk4::ApplicationWindow>>>,
    calendar_window: Rc<std::cell::RefCell<Option<gtk4::ApplicationWindow>>>,
    launcher_window: Rc<std::cell::RefCell<Option<gtk4::ApplicationWindow>>>,
    ws_popover: gtk4::Popover,
) -> StatusPopovers {
    let vpn_tooltip = TooltipPopover::new(vpn_icon, gtk4::PositionType::Bottom);
    let net_tooltip = TooltipPopover::new(&net_widgets.container, gtk4::PositionType::Bottom);
    let vol_tooltip = TooltipPopover::new(vol_icon, gtk4::PositionType::Bottom);

    let (bat_tooltip_opt, bat_popover_opt) = if let Some(ref bat_area) = bat_widget {
        let bat_pop = TooltipPopover::new(bat_area, gtk4::PositionType::Bottom);
        let pop = bat_pop.popover.clone();
        (Some(bat_pop), Some(pop))
    } else {
        (None, None)
    };

    let ccw_c = control_center_window.clone();
    let cw_c = calendar_window.clone();
    let lw_c = launcher_window.clone();
    let ws_pop_c = ws_popover.clone();
    let is_suppressed = Rc::new(move || {
        ccw_c.borrow().is_some()
            || cw_c.borrow().is_some()
            || lw_c.borrow().is_some()
            || ws_pop_c.is_visible()
    });

    let sup_vpn = is_suppressed.clone();
    vpn_tooltip.set_suppress_fn(move || sup_vpn());

    let sup_net = is_suppressed.clone();
    net_tooltip.set_suppress_fn(move || sup_net());

    let sup_vol = is_suppressed.clone();
    vol_tooltip.set_suppress_fn(move || sup_vol());

    if let Some(ref bat_tt) = bat_tooltip_opt {
        let sup_bat = is_suppressed.clone();
        bat_tt.set_suppress_fn(move || sup_bat());
    }

    let update_vpn_tooltip = vpn::build_vpn_update_fn(vpn_icon, &vpn_tooltip.popover);
    let update_network_tooltip = network::build_network_update(&net_tooltip.popover);
    let update_volume_popover = volume::build_volume_update(vol_icon, &vol_tooltip.popover);
    let update_battery_popover = battery::build_battery_update(&bat_popover_opt);

    if babydra_core::services::system::vpn::get_active_vpn_fast().is_some() {
        vpn_icon.set_visible(true);
    } else {
        vpn_icon.set_visible(false);
    }

    vpn_tooltip.attach_hover(vpn_icon, Some(update_vpn_tooltip.clone()));
    net_tooltip.attach_hover(&net_widgets.container, Some(update_network_tooltip.clone()));
    vol_tooltip.attach_hover(vol_icon, Some(update_volume_popover.clone()));

    if let Some(ref bat_area) = bat_widget {
        if let Some(ref bat_tt) = bat_tooltip_opt {
            bat_tt.attach_hover(bat_area, Some(update_battery_popover.clone()));
        }
    }

    // Timer loop for updates
    let vpn_pop_t = vpn_tooltip.popover.clone();
    let net_pop_t = net_tooltip.popover.clone();
    let vol_pop_t = vol_tooltip.popover.clone();
    let bat_pop_t = bat_popover_opt.clone();

    let update_vpn_t = update_vpn_tooltip.clone();
    let update_net_t = update_network_tooltip.clone();
    let update_vol_t = update_volume_popover.clone();
    let update_bat_t = update_battery_popover.clone();

    let bat_widget_timer = bat_widget.clone();
    let vpn_icon_timer = vpn_icon.clone();
    let wifi_icon_timer = net_widgets.wifi_icon.clone();
    let eth_area_timer = net_widgets.eth_area.clone();
    let last_net_icon = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    let sup_timer = is_suppressed.clone();

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(2000), move || {
        if sup_timer() {
            if vpn_pop_t.is_visible() {
                vpn_pop_t.popdown();
            }
            if net_pop_t.is_visible() {
                net_pop_t.popdown();
            }
            if vol_pop_t.is_visible() {
                vol_pop_t.popdown();
            }
            if let Some(ref bp) = bat_pop_t {
                if bp.is_visible() {
                    bp.popdown();
                }
            }
        } else {
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
        }

        let vpn_active = babydra_core::services::system::vpn::get_active_vpn_fast().is_some();
        if vpn_icon_timer.is_visible() != vpn_active {
            vpn_icon_timer.set_visible(vpn_active);
        }

        let active_net = babydra_core::services::system::network::get_active_network_info();
        if active_net.network_type == babydra_core::models::ActiveNetworkType::Ethernet {
            if !eth_area_timer.is_visible() {
                wifi_icon_timer.set_visible(false);
                eth_area_timer.set_visible(true);
                eth_area_timer.queue_draw();
            }
        } else {
            if !wifi_icon_timer.is_visible() {
                eth_area_timer.set_visible(false);
                wifi_icon_timer.set_visible(true);
            }
            if *last_net_icon.borrow() != active_net.icon_name {
                *last_net_icon.borrow_mut() = active_net.icon_name.clone();
                babydra_ui_kit::ui::icon::set_image_from_icon(
                    &wifi_icon_timer,
                    &active_net.icon_name,
                    14,
                );
            }
        }

        if let Some(ref bat_area) = bat_widget_timer {
            bat_area.queue_draw();
        }
        gtk4::glib::ControlFlow::Continue
    });

    StatusPopovers {
        vpn_popover: vpn_tooltip.popover,
        net_popover: net_tooltip.popover,
        vol_popover: vol_tooltip.popover,
        bat_popover_opt,
        update_volume_popover,
    }
}
