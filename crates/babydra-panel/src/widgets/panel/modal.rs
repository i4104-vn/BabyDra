//! Control Center modal window UI construction and animation callbacks.

use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};
use std::cell::RefCell;
use std::rc::Rc;

use super::items;
use super::items::header::render::create_header_row;
use super::toggle_grid::create_control_center_grid;

/// Builds the control center window UI.
pub fn build_control_center_window_ui(
    app: &gtk4::Application,
) -> (gtk4::ApplicationWindow, gtk4::Box) {
    let q_win = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&q_win);
    babydra_ui_kit::ui::window::init_layer_window(
        &q_win,
        Layer::Overlay,
        KeyboardMode::OnDemand,
        0,
        &[
            (Edge::Top, true),
            (Edge::Bottom, true),
            (Edge::Left, true),
            (Edge::Right, true),
        ],
        0,
        None,
    );
    q_win.add_css_class("control-center-window");

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 14);
    main_box.add_css_class("control-center-box");
    main_box.set_halign(gtk4::Align::End);
    main_box.set_valign(gtk4::Align::Start);
    main_box.set_size_request(360, 480);
    main_box.set_margin_top(6);
    main_box.set_margin_end(12);

    q_win.set_child(Some(&main_box));

    (q_win, main_box)
}

/// Rebuild control center contents.
pub fn rebuild_control_center_contents(
    main_box: &gtk4::Box,
    on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>,
    vol_icon: gtk4::Image,
) {
    items::volume::update_topbar_volume_icon(&vol_icon);

    // 1. Remove all existing children
    while let Some(child) = main_box.first_child() {
        main_box.remove(&child);
    }

    // 2. Append header
    main_box.append(&create_header_row(on_popover_toggled.clone()));

    // 3. Append grid
    main_box.append(&create_control_center_grid(on_popover_toggled.clone()));

    // 4. Append volume slider
    let (volume_row, _volume_scale) =
        items::volume::render::create_volume_row(on_popover_toggled.clone(), vol_icon.clone());
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
pub fn create_control_center_window(
    app: &gtk4::Application,
    control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    vol_icon: gtk4::Image,
) -> gtk4::ApplicationWindow {
    let (q_win, main_box) = build_control_center_window_ui(app);

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
            rebuild_control_center_contents(
                &main_box_c,
                on_popover_toggled_c.clone(),
                vol_icon_c.clone(),
            );
        });
    }

    // Dismiss when clicking outside main box
    babydra_ui_kit::ui::window::setup_click_outside_dismiss(&q_win, &main_box);

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
        babydra_ui_kit::ui::animation::genie_out(
            main_box_clone.upcast_ref(),
            360,
            480,
            450,
            move || {
                q_win_cb.destroy();
            },
        );
        glib::Propagation::Stop
    });

    q_win.present();
    babydra_ui_kit::ui::animation::genie_in(main_box.upcast_ref(), 360, 480, 450);

    q_win
}
