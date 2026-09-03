//! UI rendering structure for the system monitor popover and draw graphs.

use babydra_core::i18n::trans;
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Builds widgets for system monitoring.
///
/// Returns labels, popover, drawings areas, and details components.
pub fn build_sys_monitor(
    capsule: &gtk4::Box,
) -> (
    gtk4::Label,
    gtk4::Popover,
    gtk4::DrawingArea,
    gtk4::DrawingArea,
    gtk4::DrawingArea,
    gtk4::Label,
    gtk4::Label,
    gtk4::Label,
    gtk4::Label,
) {
    capsule.add_css_class("sys-monitor-box");
    capsule.set_valign(gtk4::Align::Center);

    let sys_tpl = trans("sysmon.cpu_ram");
    let sys_label = gtk4::Label::new(Some(&sys_tpl.replace("{cpu}", "0").replace("{ram}", "0")));
    sys_label.add_css_class("status-text");
    capsule.append(&sys_label);

    let popover = babydra_ui_kit::components::create_popover(
        capsule,
        gtk4::PositionType::Bottom,
        "sys-monitor-popover",
    );
    popover.set_autohide(false);

    let popover_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    popover_box.set_size_request(230, -1);

    let popover_title =
        gtk4::Label::new(Some(&babydra_core::i18n::trans("panel.system_resources")));
    popover_title.add_css_class("tile-title");
    popover_title.set_xalign(0.0);
    popover_title.set_margin_bottom(4);

    let cpu_label = gtk4::Label::new(Some(&trans("sysmon.cpu_usage")));
    cpu_label.set_xalign(0.0);
    cpu_label.add_css_class("tile-subtitle");

    let cpu_chart = gtk4::DrawingArea::new();
    cpu_chart.set_size_request(230, 52);
    cpu_chart.add_css_class("sys-chart");

    let ram_header = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    let ram_label = gtk4::Label::new(Some(&trans("sysmon.ram_usage")));
    ram_label.set_xalign(0.0);
    ram_label.set_hexpand(true);
    ram_label.set_halign(gtk4::Align::Start);
    ram_label.add_css_class("tile-subtitle");

    let ram_detail = gtk4::Label::new(Some("0.0 GB / 0.0 GB"));
    ram_detail.set_xalign(1.0);
    ram_detail.set_halign(gtk4::Align::End);
    ram_detail.add_css_class("control-square-label");
    ram_detail.set_opacity(0.7);

    ram_header.append(&ram_label);
    ram_header.append(&ram_detail);

    let ram_chart = gtk4::DrawingArea::new();
    ram_chart.set_size_request(230, 52);
    ram_chart.add_css_class("sys-chart");

    let gpu_label = gtk4::Label::new(Some(&trans("sysmon.gpu_usage")));
    gpu_label.set_xalign(0.0);
    gpu_label.add_css_class("tile-subtitle");

    let gpu_chart = gtk4::DrawingArea::new();
    gpu_chart.set_size_request(230, 52);
    gpu_chart.add_css_class("sys-chart");

    popover_box.append(&popover_title);
    popover_box.append(&cpu_label);
    popover_box.append(&cpu_chart);
    popover_box.append(&ram_header);
    popover_box.append(&ram_chart);
    popover_box.append(&gpu_label);
    popover_box.append(&gpu_chart);

    popover.set_child(Some(&popover_box));

    (
        sys_label, popover, cpu_chart, ram_chart, gpu_chart, cpu_label, ram_label, ram_detail,
        gpu_label,
    )
}

/// Converts a hex color string to normalized RGB f64 values.
fn hex_to_rgb(hex: &str) -> (f64, f64, f64) {
    let clean = hex.trim_start_matches('#');
    if let Ok(val) = u32::from_str_radix(clean, 16) {
        let r = ((val >> 16) & 0xFF) as f64 / 255.0;
        let g = ((val >> 8) & 0xFF) as f64 / 255.0;
        let b = (val & 0xFF) as f64 / 255.0;
        (r, g, b)
    } else {
        (0.5, 0.5, 0.5)
    }
}

/// Configures custom draw functions on a GTK DrawingArea to plot system metrics history graphs.
pub fn setup_chart_draw(
    chart: &gtk4::DrawingArea,
    history: Rc<RefCell<std::collections::VecDeque<f64>>>,
    color_hex: &'static str,
) {
    chart.set_draw_func(move |_, cr, width, height| {
        let history = history.borrow();
        if history.is_empty() {
            return;
        }

        let w = width as f64;
        let h = height as f64;

        cr.set_source_rgba(1.0, 1.0, 1.0, 0.08);
        cr.set_line_width(1.0);
        for i in 1..4 {
            let y = h * (i as f64) / 4.0;
            cr.move_to(0.0, y);
            cr.line_to(w, y);
            let _ = cr.stroke();
        }

        let len = history.len();
        let step = w / ((len - 1) as f64);

        cr.move_to(0.0, h);
        for (i, &val) in history.iter().enumerate() {
            let x = i as f64 * step;
            let y = h - (val / 100.0) * h;
            cr.line_to(x, y.max(0.0).min(h));
        }
        cr.line_to(w, h);
        cr.close_path();

        let (r, g, b) = hex_to_rgb(color_hex);
        cr.set_source_rgba(r, g, b, 0.15);
        let _ = cr.fill();

        let mut first = true;
        for (i, &val) in history.iter().enumerate() {
            let x = i as f64 * step;
            let y = h - (val / 100.0) * h;
            if first {
                cr.move_to(x, y.max(0.0).min(h));
                first = false;
            } else {
                cr.line_to(x, y.max(0.0).min(h));
            }
        }

        cr.set_source_rgba(r, g, b, 0.85);
        cr.set_line_width(2.0);
        let _ = cr.stroke();
    });
}
