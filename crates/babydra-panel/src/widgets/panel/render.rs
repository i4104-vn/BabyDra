use gtk4::prelude::*;
use gtk4_layer_shell::{KeyboardMode, Layer, Edge, LayerShell};

#[derive(Debug, Clone, Copy)]
pub struct BatteryInfo {
    pub percentage: u32,
    pub is_charging: bool,
}

pub fn get_battery_info() -> Option<BatteryInfo> {
    let power_dir = std::path::Path::new("/sys/class/power_supply");
    if !power_dir.exists() { return None; }
    if let Ok(entries) = std::fs::read_dir(power_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(kind) = std::fs::read_to_string(path.join("type")) {
                if kind.trim() == "Battery" {
                    if let Ok(scope) = std::fs::read_to_string(path.join("scope")) {
                        if scope.trim().eq_ignore_ascii_case("Device") {
                            continue;
                        }
                    }
                    let mut capacity_opt = std::fs::read_to_string(path.join("capacity"))
                        .ok()
                        .and_then(|s| s.trim().parse::<u32>().ok());
                    
                    if capacity_opt.is_none() {
                        let energy_now = std::fs::read_to_string(path.join("energy_now"))
                            .or_else(|_| std::fs::read_to_string(path.join("charge_now")))
                            .ok()
                            .and_then(|s| s.trim().parse::<f64>().ok());
                        let energy_full = std::fs::read_to_string(path.join("energy_full"))
                            .or_else(|_| std::fs::read_to_string(path.join("charge_full")))
                            .ok()
                            .and_then(|s| s.trim().parse::<f64>().ok());
                        if let (Some(now), Some(full)) = (energy_now, energy_full) {
                            if full > 0.0 {
                                capacity_opt = Some(((now / full) * 100.0) as u32);
                            }
                        }
                    }

                    let capacity = capacity_opt.unwrap_or(100);
                    let status = std::fs::read_to_string(path.join("status"))
                        .unwrap_or_default();
                    return Some(BatteryInfo {
                        percentage: capacity.min(100),
                        is_charging,
                    });
                }
            }
        }
    }

    // Mock battery data (75% charging) for testing/preview
    Some(BatteryInfo {
        percentage: 75,
        is_charging: true,
    })
}

fn draw_rounded_rectangle(cr: &gtk4::cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    let degrees = std::f64::consts::PI / 180.0;
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, -90.0 * degrees, 0.0 * degrees);
    cr.arc(x + w - r, y + h - r, r, 0.0 * degrees, 90.0 * degrees);
    cr.arc(x + r, y + h - r, r, 90.0 * degrees, 180.0 * degrees);
    cr.arc(x + r, y + r, r, 180.0 * degrees, 270.0 * degrees);
    cr.close_path();
}

pub fn create_battery_widget() -> Option<gtk4::DrawingArea> {
    if get_battery_info().is_none() {
        return None;
    }

    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_content_width(24);
    drawing_area.set_content_height(14);
    drawing_area.set_valign(gtk4::Align::Center);
    drawing_area.set_halign(gtk4::Align::Center);

    drawing_area.set_draw_func(move |_area, cr, _width, height| {
        let bat_info = match get_battery_info() {
            Some(info) => info,
            None => return,
        };

        let is_dark = babydra_utils::ui::theme::is_dark_mode();

        let (fg_r, fg_g, fg_b) = if is_dark {
            (1.0, 1.0, 1.0)
        } else {
            (0.15, 0.15, 0.15)
        };

        let (fill_r, fill_g, fill_b) = if bat_info.is_charging {
            (0.13, 0.77, 0.36) // Green (#22c55e)
        } else if bat_info.percentage <= 15 {
            (0.94, 0.27, 0.27) // Red (#ef4444)
        } else if bat_info.percentage <= 30 {
            (0.96, 0.62, 0.04) // Amber (#f59e0b)
        } else {
            (0.13, 0.77, 0.36) // Green (#22c55e)
        };

        let h = height as f64;
        
        let shell_x = 1.0;
        let shell_y = 1.0;
        let shell_w = 18.0;
        let shell_h = h - 2.0;

        // 1. Draw outer shell border
        draw_rounded_rectangle(cr, shell_x, shell_y, shell_w, shell_h, 2.5);
        cr.set_line_width(1.5);
        cr.set_source_rgba(fg_r, fg_g, fg_b, 0.75);
        let _ = cr.stroke();

        // 2. Draw terminal tip on right
        let tip_x = shell_x + shell_w + 1.0;
        let tip_y = shell_y + (shell_h - 4.0) / 2.0;
        draw_rounded_rectangle(cr, tip_x, tip_y, 2.0, 4.0, 1.0);
        cr.set_source_rgba(fg_r, fg_g, fg_b, 0.75);
        let _ = cr.fill();

        // 3. Draw inner fill bar matching percentage
        let pad = 2.0;
        let fill_max_w = shell_w - (pad * 2.0);
        let fill_h = shell_h - (pad * 2.0);
        let fill_w = (fill_max_w * (bat_info.percentage as f64 / 100.0)).clamp(1.5, fill_max_w);

        draw_rounded_rectangle(cr, shell_x + pad, shell_y + pad, fill_w, fill_h, 1.0);
        cr.set_source_rgba(fill_r, fill_g, fill_b, 0.95);
        let _ = cr.fill();

        // 4. Draw lightning bolt if charging
        if bat_info.is_charging {
            let cx = shell_x + shell_w / 2.0;
            let cy = shell_y + shell_h / 2.0;
            cr.new_path();
            cr.move_to(cx + 0.5, cy - 3.5);
            cr.line_to(cx - 2.0, cy + 0.5);
            cr.line_to(cx - 0.2, cy + 0.5);
            cr.line_to(cx - 0.5, cy + 3.5);
            cr.line_to(cx + 2.0, cy - 0.5);
            cr.line_to(cx + 0.2, cy - 0.5);
            cr.close_path();

            cr.set_source_rgba(1.0, 1.0, 1.0, 0.95);
            let _ = cr.fill();
        }
    });

    Some(drawing_area)
}

pub fn build_status_indicators_ui() -> (gtk4::Box, gtk4::Button, gtk4::Label, gtk4::Image, gtk4::Image, Option<gtk4::DrawingArea>) {
    let status_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 4);
    status_box.add_css_class("status-indicators-box");

    let status_button = gtk4::Button::new();
    status_button.add_css_class("panel-status-btn");

    let inner_layout = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    let net_icon = babydra_utils::ui::icon::get_icon("wifi", 14);
    net_icon.add_css_class("status-icon");
    
    let vol_icon = if super::items::volume::is_muted() {
        babydra_utils::ui::icon::get_icon("volume-mute", 14)
    } else {
        babydra_utils::ui::icon::get_icon("volume", 14)
    };
    vol_icon.add_css_class("status-icon");
    
    inner_layout.append(&net_icon);
    inner_layout.append(&vol_icon);

    let bat_widget = create_battery_widget();
    if let Some(ref bat_area) = bat_widget {
        bat_area.add_css_class("status-icon");
        if let Some(info) = get_battery_info() {
            let status_str = if info.is_charging { "Charging" } else { "Discharging" };
            bat_area.set_tooltip_text(Some(&format!("Battery: {}% ({})", info.percentage, status_str)));
        }
        inner_layout.append(bat_area);
    }

    status_button.set_child(Some(&inner_layout));

    let separator = gtk4::Label::new(Some("│"));
    separator.add_css_class("capsule-separator");

    (status_box, status_button, separator, vol_icon, net_icon, bat_widget)
}

pub fn build_control_center_window_ui(
    app: &gtk4::Application,
) -> (gtk4::ApplicationWindow, gtk4::Box) {
    let q_win = gtk4::ApplicationWindow::new(app);
    babydra_utils::ui::theme::apply_theme_class(&q_win);
    q_win.init_layer_shell();
    q_win.set_layer(Layer::Overlay);
    q_win.set_keyboard_mode(KeyboardMode::OnDemand);

    // Anchor to all 4 edges to cover the entire screen transparently
    q_win.set_anchor(Edge::Top, true);
    q_win.set_anchor(Edge::Bottom, true);
    q_win.set_anchor(Edge::Left, true);
    q_win.set_anchor(Edge::Right, true);
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
