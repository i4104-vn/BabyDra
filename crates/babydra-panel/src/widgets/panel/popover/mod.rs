pub mod battery;
pub mod network;
pub mod volume;
pub mod vpn;

use super::state::StatusPopovers;
use babydra_ui_kit::components::popovers::TooltipPopover;
use gtk4::prelude::*;
use std::rc::Rc;

pub(super) struct StatusPopoverContext<'a> {
    pub volume_icon: &'a gtk4::Image,
    pub network: &'a super::state::NetworkWidgets,
    pub vpn_icon: &'a gtk4::Image,
    pub battery: &'a Option<gtk4::DrawingArea>,
    pub control_center_window: Rc<std::cell::RefCell<Option<gtk4::ApplicationWindow>>>,
    pub calendar_window: Rc<std::cell::RefCell<Option<gtk4::ApplicationWindow>>>,
    pub launcher_window: Rc<std::cell::RefCell<Option<gtk4::ApplicationWindow>>>,
    pub workspace_popover: gtk4::Popover,
}

pub(super) fn setup_status_popover(context: StatusPopoverContext<'_>) -> StatusPopovers {
    let vpn_tooltip = TooltipPopover::new(context.vpn_icon, gtk4::PositionType::Bottom);
    let net_tooltip = TooltipPopover::new(&context.network.container, gtk4::PositionType::Bottom);
    let vol_tooltip = TooltipPopover::new(context.volume_icon, gtk4::PositionType::Bottom);

    let (bat_tooltip_opt, bat_popover_opt) = if let Some(bat_area) = context.battery {
        let bat_pop = TooltipPopover::new(bat_area, gtk4::PositionType::Bottom);
        let pop = bat_pop.popover.clone();
        (Some(bat_pop), Some(pop))
    } else {
        (None, None)
    };

    let ccw_c = context.control_center_window.clone();
    let cw_c = context.calendar_window.clone();
    let lw_c = context.launcher_window.clone();
    let ws_pop_c = context.workspace_popover.clone();
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

    let update_vpn_tooltip = vpn::build_vpn_update_fn(context.vpn_icon, &vpn_tooltip.popover);
    let update_network_tooltip = network::build_network_update(&net_tooltip.popover);
    let update_volume_popover =
        volume::build_volume_update(context.volume_icon, &vol_tooltip.popover);
    let update_battery_popover = battery::build_battery_update(&bat_popover_opt);

    if babydra_core::services::system::vpn::get_active_vpn_fast().is_some() {
        context.vpn_icon.set_visible(true);
    } else {
        context.vpn_icon.set_visible(false);
    }

    vpn_tooltip.attach_hover(context.vpn_icon, Some(update_vpn_tooltip.clone()));
    net_tooltip.attach_hover(
        &context.network.container,
        Some(update_network_tooltip.clone()),
    );
    vol_tooltip.attach_hover(context.volume_icon, Some(update_volume_popover.clone()));

    if let Some(bat_area) = context.battery {
        if let Some(bat_tt) = &bat_tooltip_opt {
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

    let bat_widget_timer = context.battery.clone();
    let vpn_icon_timer = context.vpn_icon.clone();
    let sup_timer = is_suppressed.clone();

    // Track last known volume/mute
    let (init_vol, init_muted) = babydra_core::services::system::volume::get_volume_state();
    let last_vol = std::rc::Rc::new(std::cell::Cell::new(init_vol));
    let last_muted = std::rc::Rc::new(std::cell::Cell::new(init_muted));

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

        if let Some(bat_area) = &bat_widget_timer {
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
        current_volume: last_vol,
        current_muted: last_muted,
    }
}
