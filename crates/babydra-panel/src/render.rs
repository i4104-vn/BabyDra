use crate::widgets::panel::create_status_icons;
use crate::widgets::system_monitor::create_sys_monitor_w;
use crate::widgets::tray::create_tray_widget;
use crate::widgets::workspace::create_workspace_sw;
use babydra_island::create_system_island;
use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;

/// Rebuild panel window.
pub fn rebuild_panel_window(
    window: &gtk4::ApplicationWindow,
    app: &gtk4::Application,
    control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    calendar_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
) {
    // 1. Remove child
    window.set_child(None::<&gtk4::Widget>);

    // 2. Layout container
    let box_layout = gtk4::CenterBox::new();
    box_layout.add_css_class("panel-box");

    // 3. Logo Button
    let logo_btn = gtk4::Button::new();
    logo_btn.add_css_class("panel-logo-btn");
    logo_btn.set_cursor_from_name(Some("pointer"));
    logo_btn.set_valign(gtk4::Align::Center);
    logo_btn.set_halign(gtk4::Align::Center);
    logo_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("panel.logo_tooltip")));
    let logo_icon = babydra_ui_kit::ui::icon::get_icon("logo", 16);
    logo_btn.set_child(Some(&logo_icon));

    let lw_clone = launcher_window.clone();
    let ccw_clone = control_center_window.clone();
    let cw_clone = calendar_window.clone();
    let app_clone = app.clone();

    let click_gesture = gtk4::GestureClick::new();
    click_gesture.set_button(0); // Listen to both left (1) and right (3) click
    click_gesture.connect_pressed(move |gesture, _, _, _| {
        let button = gesture.current_button();

        let cc_win = { ccw_clone.borrow().clone() };
        if let Some(win) = cc_win {
            win.close();
        }
        let cal_win = { cw_clone.borrow().clone() };
        if let Some(win) = cal_win {
            win.close();
        }

        if button == 1 {
            // Left-click: Toggle Launcher
            let existing = { lw_clone.borrow().clone() };
            if let Some(win) = existing {
                win.close();
            } else {
                let l_win = babydra_launcher::build_launcher_ui(&app_clone, lw_clone.clone());
                l_win.present();
                if let Ok(mut borrow) = lw_clone.try_borrow_mut() {
                    *borrow = Some(l_win);
                }
            }
        } else if button == 3 {
            // Right-click: Minimize all application windows to show desktop
            let existing = { lw_clone.borrow().clone() };
            if let Some(win) = existing {
                win.close();
            }
            babydra_core::services::window::minimize_all_windows();
        }
    });
    logo_btn.add_controller(click_gesture);

    // 4. Workspace Switcher
    let workspace_box = create_workspace_sw();
    let separator = gtk4::Label::new(Some("│"));
    separator.add_css_class("capsule-separator");
    separator.set_valign(gtk4::Align::Center);
    workspace_box.prepend(&separator);
    workspace_box.prepend(&logo_btn);

    // 5. Unified Status and Clock Capsule
    let status_indicators = create_status_icons(
        app,
        control_center_window.clone(),
        calendar_window.clone(),
        launcher_window.clone(),
    );

    let system_monitor = create_sys_monitor_w();
    let tray_widget = create_tray_widget(window);

    // Left Wrapper
    let left_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    left_box.set_hexpand(true);
    left_box.set_halign(gtk4::Align::Start);
    left_box.set_valign(gtk4::Align::Center);
    left_box.append(&workspace_box);

    let left_wrapper = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    left_wrapper.set_valign(gtk4::Align::Start);
    left_wrapper.set_size_request(-1, 36);
    left_wrapper.append(&left_box);

    // Center Wrapper
    let center_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    center_box.set_hexpand(true);
    center_box.set_halign(gtk4::Align::Center);
    center_box.set_valign(gtk4::Align::Start);

    let notch_capsule = create_system_island();
    center_box.append(&notch_capsule);

    // Right Wrapper
    let right_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    right_box.set_hexpand(true);
    right_box.set_halign(gtk4::Align::End);
    right_box.set_valign(gtk4::Align::Center);
    right_box.append(&tray_widget);
    right_box.append(&system_monitor);
    right_box.append(&status_indicators);

    let right_wrapper = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    right_wrapper.set_valign(gtk4::Align::Start);
    right_wrapper.set_size_request(-1, 36);
    right_wrapper.append(&right_box);

    // Assemble CenterBox
    box_layout.set_start_widget(Some(&left_wrapper));
    box_layout.set_center_widget(Some(&center_box));
    box_layout.set_end_widget(Some(&right_wrapper));

    window.set_child(Some(&box_layout));

    // Input region handler: Ensure transparent areas outside top bar and notch capsule pass mouse clicks to underlying windows
    let notch_clone = notch_capsule.clone();
    window.add_tick_callback(move |win, _| {
        if let Some(surface) = win.surface() {
            let win_w = win.width();
            let region = gtk4::cairo::Region::create();

            // Top bar panel rect (height 36px)
            let top_rect = gtk4::cairo::RectangleInt::new(0, 0, win_w, 36);
            let _ = region.union_rectangle(&top_rect);

            // Notch capsule rect when expanded
            if notch_clone.is_visible() {
                if let Some((nx, ny)) = notch_clone.translate_coordinates(win, 0.0, 0.0) {
                    let nw = notch_clone.width();
                    let nh = notch_clone.height();
                    if nh > 36 && nw > 0 {
                        let notch_rect =
                            gtk4::cairo::RectangleInt::new(nx as i32, ny as i32, nw, nh);
                        let _ = region.union_rectangle(&notch_rect);
                    }
                }
            }

            surface.set_input_region(&region);
        }
        glib::ControlFlow::Continue
    });
}

/// Builds the top panel window UI.
pub fn build_panel_ui(
    app: &gtk4::Application,
    control_center_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    calendar_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
    launcher_window: Rc<RefCell<Option<gtk4::ApplicationWindow>>>,
) -> gtk4::ApplicationWindow {
    let window = gtk4::ApplicationWindow::new(app);
    babydra_ui_kit::ui::theme::apply_theme_class(&window);

    // Initialize layer shell properties on the window
    babydra_ui_kit::ui::window::init_layer_window(
        &window,
        Layer::Top,
        KeyboardMode::OnDemand,
        38,
        &[(Edge::Top, true), (Edge::Left, true), (Edge::Right, true)],
        0,
        None,
    );

    // Float topbar flush against top edge
    window.set_margin(Edge::Top, 8);
    window.set_margin(Edge::Left, 1);
    window.set_margin(Edge::Right, 1);

    window.set_default_size(0, 36);

    window.add_css_class("panel-window");

    let window_c = window.clone();
    let app_c = app.clone();
    let ccw_c = control_center_window.clone();
    let cw_c = calendar_window.clone();
    let lw_c = launcher_window.clone();

    rebuild_panel_window(
        &window,
        app,
        control_center_window.clone(),
        calendar_window.clone(),
        launcher_window.clone(),
    );

    if let Some(settings) = gtk4::Settings::default() {
        settings.connect_gtk_application_prefer_dark_theme_notify(move |_| {
            rebuild_panel_window(&window_c, &app_c, ccw_c.clone(), cw_c.clone(), lw_c.clone());
        });
    }

    let window_c2 = window.clone();
    let app_c2 = app.clone();
    let ccw_c2 = control_center_window.clone();
    let cw_c2 = calendar_window.clone();
    let lw_c2 = launcher_window.clone();
    babydra_core::i18n::watch_locale_change(move |_| {
        rebuild_panel_window(
            &window_c2,
            &app_c2,
            ccw_c2.clone(),
            cw_c2.clone(),
            lw_c2.clone(),
        );
    });

    window
}
