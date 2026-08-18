pub use babydra_core::get_battery_info;
use gtk4::prelude::*;

/// Creates a new `battery widget`.
pub fn create_battery_widget() -> Option<gtk4::DrawingArea> {
    if get_battery_info().is_none() {
        return None;
    }

    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_content_width(24);
    drawing_area.set_content_height(14);
    drawing_area.set_valign(gtk4::Align::Center);
    drawing_area.set_halign(gtk4::Align::Center);

    drawing_area.set_draw_func(move |_area, cr, width, height| {
        let bat_info = match get_battery_info() {
            Some(info) => info,
            None => return,
        };

        let is_dark = babydra_ui_kit::ui::theme::is_dark_mode();
        babydra_ui_kit::ui::battery::draw_cairo_battery(
            cr,
            width as f64,
            height as f64,
            bat_info.percentage,
            bat_info.is_charging,
            is_dark,
        );
    });

    Some(drawing_area)
}

/// Builds the panel status indicators row.
pub fn build_status_indicators_ui() -> (
    gtk4::Box,
    gtk4::Button,
    gtk4::Label,
    gtk4::Image,
    gtk4::Image,
    gtk4::Image,
    Option<gtk4::DrawingArea>,
) {
    let status_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    status_box.add_css_class("status-indicators-box");

    let status_button = gtk4::Button::new();
    status_button.add_css_class("panel-status-btn");

    let inner_layout = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);

    let vpn_icon = babydra_ui_kit::ui::icon::get_icon("shield", 14);
    vpn_icon.add_css_class("status-icon");
    vpn_icon.set_visible(false);

    let net_icon = babydra_ui_kit::ui::icon::get_icon("wifi", 14);
    net_icon.add_css_class("status-icon");

    let vol_icon = if super::items::volume::is_muted() {
        babydra_ui_kit::ui::icon::get_icon("volume-mute", 14)
    } else {
        babydra_ui_kit::ui::icon::get_icon("volume", 14)
    };
    vol_icon.add_css_class("status-icon");

    inner_layout.append(&vpn_icon);
    inner_layout.append(&net_icon);
    inner_layout.append(&vol_icon);

    let bat_widget = create_battery_widget();
    if let Some(ref bat_area) = bat_widget {
        bat_area.add_css_class("status-icon");
        inner_layout.append(bat_area);
    }

    status_button.set_child(Some(&inner_layout));

    let separator = gtk4::Label::new(Some("│"));
    separator.add_css_class("capsule-separator");

    (
        status_box,
        status_button,
        separator,
        vol_icon,
        net_icon,
        vpn_icon,
        bat_widget,
    )
}
