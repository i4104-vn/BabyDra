pub mod items;
pub mod toggle_grid;
mod render;

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

use toggle_grid::create_control_center_grid;
use items::power::render::create_header_row;

pub use items::backlight::detect_ddc_bus;

/// Creates a unified status indicators capsule containing (1) status details button and (2) clock button.
/// Clicking the status button toggles Control Center; clicking the clock button toggles Calendar.
/// The two panels are mutually exclusive.
pub fn create_status_indicators(
    app: &gtk4::Application,
    control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    calendar_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
) -> gtk4::Box {
    let (status_box, status_button, separator, vol_icon, net_icon) = render::build_status_indicators_ui();

    // Initial update of volume & network tooltips on load
    items::volume::update_topbar_volume_icon(&vol_icon);

    let update_network_tooltip = {
        let net_icon_c = net_icon.clone();
        Rc::new(move || {
            let (enabled, ssid) = babydra_common::helper::wifi::get_wifi_state();
            let speed = babydra_common::helper::network::get_network_speed();

            let net_tooltip = if !enabled {
                "Network: Disabled".to_string()
            } else if ssid == "Disconnected" || ssid == "Off" {
                "Network: Disconnected".to_string()
            } else {
                format!(
                    "Network: {}\n↓ {}   ↑ {}",
                    ssid,
                    babydra_common::helper::network::format_speed(speed.rx_speed),
                    babydra_common::helper::network::format_speed(speed.tx_speed)
                )
            };
            net_icon_c.set_tooltip_text(Some(&net_tooltip));
        })
    };

    update_network_tooltip();

    let scroll_controller = gtk4::EventControllerScroll::new(
        gtk4::EventControllerScrollFlags::VERTICAL
    );
    let vol_icon_scroll = vol_icon.clone();
    scroll_controller.connect_scroll(move |_, _dx, dy| {
        let current_vol = items::volume::get_current_volume();
        let step = 5.0;
        let new_vol = if dy < 0.0 {
            (current_vol + step).min(100.0)
        } else if dy > 0.0 {
            (current_vol - step).max(0.0)
        } else {
            current_vol
        };

        if (new_vol - current_vol).abs() > 0.1 {
            items::volume::set_volume(new_vol);
            items::volume::update_topbar_volume_icon(&vol_icon_scroll);
        }
        gtk4::glib::Propagation::Stop
    });
    status_button.add_controller(scroll_controller);


    let motion_controller = gtk4::EventControllerMotion::new();
    let update_net_enter = update_network_tooltip.clone();
    let vol_icon_enter = vol_icon.clone();
    motion_controller.connect_enter(move |_, _, _| {
        items::volume::update_topbar_volume_icon(&vol_icon_enter);
        update_net_enter();
    });
    status_button.add_controller(motion_controller);

    let vol_icon_timer = vol_icon.clone();
    let update_net_timer = update_network_tooltip.clone();
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(1500), move || {
        items::volume::update_topbar_volume_icon(&vol_icon_timer);
        update_net_timer();
        gtk4::glib::ControlFlow::Continue
    });

    let app_clone = app.clone();
    let ccw_clone = control_center_window.clone();
    let cw_clone = calendar_window.clone();
    let lw_clone = launcher_window.clone();
    let vol_icon_clone = vol_icon.clone();
    status_button.connect_clicked(move |_| {
        let launcher_active = { lw_clone.borrow().clone() };
        if let Some(win) = launcher_active {
            win.close();
        }

        let cal_active = { cw_clone.borrow().clone() };
        if let Some(win) = cal_active {
            win.close();
        }

        let existing = {
            let borrow = ccw_clone.borrow();
            borrow.clone()
        };
        if let Some(existing_window) = existing {
            existing_window.close();
        } else {
            let q_win = create_control_center_window(&app_clone, ccw_clone.clone(), vol_icon_clone.clone());
            if let Ok(mut borrow) = ccw_clone.try_borrow_mut() {
                *borrow = Some(q_win);
            }
        }
    });

    let clock_button = crate::widgets::clock::create_clock_widget(
        app,
        control_center_window.clone(),
        calendar_window.clone(),
        launcher_window.clone(),
    );

    status_box.append(&status_button);
    status_box.append(&separator);
    status_box.append(&clock_button);

    status_box
}

fn rebuild_control_center_contents(
    main_box: &gtk4::Box,
    on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>,
    vol_icon: gtk4::Image,
) {
    // Sync the topbar volume icon to current hardware state on load
    items::volume::update_topbar_volume_icon(&vol_icon);

    // 1. Remove all existing children
    while let Some(child) = main_box.first_child() {
        main_box.remove(&child);
    }

    // 2. Append header
    main_box.append(&create_header_row());

    // 3. Append grid
    main_box.append(&create_control_center_grid(on_popover_toggled.clone()));

    // 4. Append volume slider
    let (volume_row, _volume_scale) = items::volume::render::create_volume_row(
        on_popover_toggled.clone(),
        vol_icon.clone(),
    );
    main_box.append(&volume_row);

    // 5. Append brightness slider
    let (brightness_row, _brightness_scale) = items::backlight::render::create_brightness_row();
    main_box.append(&brightness_row);

    // 6. Append disk monitor box
    main_box.append(&items::storage::render::create_disk_list_box());
}

/// Builds and maps a glassmorphic Control Center popup ApplicationWindow anchored
/// to the top-right corner. It binds volume and brightness sliders, grid toggles,
/// and registers Genie animations on close and map events.
fn create_control_center_window(
    app: &gtk4::Application,
    control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    vol_icon: gtk4::Image,
) -> gtk4::ApplicationWindow {
    let (q_win, main_box) = render::build_control_center_window_ui(app);

    let popover_active = Rc::new(std::cell::Cell::new(false));
    let popover_active_clone = popover_active.clone();
    let q_win_weak = q_win.downgrade();
    
    let motion_controller = gtk4::EventControllerMotion::new();
    main_box.add_controller(motion_controller.clone());
    let motion_c = motion_controller.clone();

    let on_popover_toggled = Rc::new(move |is_open: bool| {
        popover_active_clone.set(is_open);
        if !is_open {
            if !motion_c.contains_pointer() {
                if let Some(win) = q_win_weak.upgrade() {
                    win.close();
                }
            }
        }
    }) as Rc<dyn Fn(bool)>;

    let on_popover_toggled_opt = Some(on_popover_toggled.clone());
    rebuild_control_center_contents(&main_box, on_popover_toggled_opt.clone(), vol_icon.clone());

    if let Some(settings) = gtk4::Settings::default() {
        let main_box_c = main_box.clone();
        let on_popover_toggled_c = on_popover_toggled_opt.clone();
        let vol_icon_c = vol_icon.clone();
        settings.connect_gtk_application_prefer_dark_theme_notify(move |_| {
            rebuild_control_center_contents(&main_box_c, on_popover_toggled_c.clone(), vol_icon_c.clone());
        });
    }

    // Dismiss when clicking outside the control center box area
    babydra_utils::ui::window::setup_click_outside_dismiss(&q_win, &main_box);

    let popover_active_for_notify = popover_active.clone();
    q_win.connect_is_active_notify(move |win| {
        if !win.is_active() && !popover_active_for_notify.get() {
            win.close();
        }
    });

    let is_animating = Rc::new(std::cell::Cell::new(false));
    let is_animating_clone = is_animating.clone();
    let ccw_inner = control_center_window.clone();
    let q_win_clone = q_win.clone();
    let main_box_clone = main_box.clone();
    q_win.connect_close_request(move |_| {
        if is_animating_clone.get() {
            return glib::Propagation::Stop;
        }
        is_animating_clone.set(true);
        if let Ok(mut borrow) = ccw_inner.try_borrow_mut() {
            *borrow = None;
        }
        let q_win_cb = q_win_clone.clone();
        babydra_utils::ui::animation::genie_out(
            main_box_clone.upcast_ref(),
            360,
            480,
            450,
            move || {
                q_win_cb.destroy();
            }
        );
        glib::Propagation::Stop
    });

    q_win.present();
    babydra_utils::ui::animation::genie_in(main_box.upcast_ref(), 360, 480, 450);

    q_win
}
