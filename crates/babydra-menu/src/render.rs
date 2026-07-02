//! UI layout and renderer for the desktop context menu.

use gtk4::prelude::*;
use gtk4_layer_shell::{Edge, KeyboardMode, Layer};
use std::cell::Cell;
use std::rc::Rc;

use crate::widgets::item::add_menu_item;
use crate::widgets::position::position_and_animate_menu;
use crate::widgets::events::setup_menu_dismiss_events;
use crate::widgets::actions;

/// Closes context menu with a genie out animation.
pub fn close_menu_animated(
    window: &gtk4::ApplicationWindow,
    menu_box: &gtk4::Box,
    action: Option<Rc<dyn Fn()>>,
) {
    let win = window.clone();
    let w = menu_box.width().max(220);
    let h = menu_box.height().max(240);
    babydra_common::animation::genie_out(
        menu_box.upcast_ref(),
        w,
        h,
        450,
        move || {
            if let Some(act) = action {
                act();
            }
            win.close();
        }
    );
}

/// Builds Context Menu UI ApplicationWindow.
pub fn build_menu_ui(app: &gtk4::Application) -> gtk4::ApplicationWindow {
    let window = gtk4::ApplicationWindow::new(app);
    babydra_common::apply_theme_class(&window);
    
    babydra_common::window::init_layer_window(
        &window,
        Layer::Overlay,
        KeyboardMode::Exclusive,
        -1,
        &[
            (Edge::Top, true),
            (Edge::Bottom, true),
            (Edge::Left, true),
            (Edge::Right, true),
        ],
        -1,
    );

    window.add_css_class("menu-fullscreen");

    let fixed_layout = gtk4::Fixed::new();
    window.set_child(Some(&fixed_layout));

    let menu_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    menu_box.add_css_class("menu-box");
    menu_box.set_width_request(220);

    let is_positioned = Rc::new(Cell::new(false));
    let motion_controller = gtk4::EventControllerMotion::new();
    let menu_box_clone = menu_box.clone();
    let fixed_layout_clone = fixed_layout.clone();
    let window_clone = window.clone();
    let is_positioned_clone = is_positioned.clone();

    let position_menu = move |x: f64, y: f64| {
        if is_positioned_clone.get() {
            return;
        }
        is_positioned_clone.set(true);
        position_and_animate_menu(x, y, &window_clone, &fixed_layout_clone, &menu_box_clone);
    };

    let pos_menu_enter = position_menu.clone();
    motion_controller.connect_enter(move |_, x, y| {
        pos_menu_enter(x, y);
    });

    let pos_menu_motion = position_menu.clone();
    motion_controller.connect_motion(move |_, x, y| {
        pos_menu_motion(x, y);
    });
    window.add_controller(motion_controller);

    setup_menu_dismiss_events(&window, &menu_box);

    add_menu_item(
        &window,
        &menu_box,
        &babydra_common::i18n::t("menu.terminal"),
        "terminal",
        Rc::new(actions::execute_terminal),
    );

    add_menu_item(
        &window,
        &menu_box,
        &babydra_common::i18n::t("menu.file_manager"),
        "folder",
        Rc::new(actions::execute_file_manager),
    );

    let window_file_dialog = window.clone();
    add_menu_item(
        &window,
        &menu_box,
        &babydra_common::i18n::t("menu.change_wallpaper"),
        "display",
        Rc::new(move || {
            actions::execute_change_wallpaper(&window_file_dialog);
        }),
    );

    add_menu_item(
        &window,
        &menu_box,
        &babydra_common::i18n::t("menu.reconfigure_shell"),
        "restart",
        Rc::new(actions::execute_reconfigure_shell),
    );

    let sep = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    sep.add_css_class("menu-item-separator");
    menu_box.append(&sep);

    add_menu_item(
        &window,
        &menu_box,
        &babydra_common::i18n::t("menu.exit_shell"),
        "logout",
        Rc::new(actions::execute_exit_shell),
    );

    menu_box.set_opacity(0.0);

    window
}

