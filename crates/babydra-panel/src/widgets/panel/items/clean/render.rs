use super::popover::setup_clean_popover;
use gtk4::prelude::*;
use std::rc::Rc;

/// Creates a new `clean tile`.
pub fn create_clean_tile(on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>) -> gtk4::Button {
    let btn = babydra_ui_kit::components::create_colored_icon_button(
        "broom",
        18,
        "rgba(255, 255, 255, 0.8)",
        &["control-square-tile"],
        None,
        || {},
    );
    btn.set_size_request(56, 56);
    btn.set_halign(gtk4::Align::Center);
    btn.set_valign(gtk4::Align::Center);
    btn.set_hexpand(false);
    btn.set_vexpand(false);

    let popover = babydra_ui_kit::components::create_popover(
        &btn,
        gtk4::PositionType::Bottom,
        "media-popover",
    );
    popover.set_has_arrow(false);

    let popover_box = setup_clean_popover(&popover);

    let on_popover_toggled_c = on_popover_toggled.clone();
    let popover_c = popover.clone();
    btn.connect_clicked(move |_| {
        popover_c.popup();
    });

    let btn_c = btn.clone();
    let popover_box_clone = popover_box.clone();
    let on_popover_toggled_c_map = on_popover_toggled.clone();
    popover.connect_map(move |_| {
        btn_c.add_css_class("active");
        let active_icon = babydra_ui_kit::ui::icon::get_icon_colored("broom", 18, "#ffffff");
        btn_c.set_child(Some(&active_icon));

        if let Some(ref cb) = on_popover_toggled_c_map {
            cb(true);
        }

        babydra_ui_kit::ui::animation::slide_in(
            popover_box_clone.upcast_ref(),
            babydra_ui_kit::ui::animation::SlideDirection::Down,
            15,
            450,
        );
    });

    let btn_c2 = btn.clone();
    popover.connect_closed(move |_| {
        btn_c2.remove_css_class("active");
        let inactive_icon =
            babydra_ui_kit::ui::icon::get_icon_colored("broom", 18, "rgba(255, 255, 255, 0.8)");
        btn_c2.set_child(Some(&inactive_icon));

        if let Some(ref cb) = on_popover_toggled_c {
            cb(false);
        }
    });

    btn
}
