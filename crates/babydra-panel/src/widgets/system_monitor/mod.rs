//! System resource monitor widget for CPU load and RAM usage statistics.
//! Uses the unified MonitorSnapshot service from babydra-core.

use babydra_core::services::system::monitor::{subscribe as subscribe_monitor, MonitorSnapshot};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

mod render;

/// Creates a system resource monitoring capsule.
/// Displays basic stats on hover and draws CPU/RAM/GPU utilization graphs on a popup card.
pub fn create_sys_monitor_w() -> gtk4::Box {
    let capsule = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);

    let (
        sys_label,
        popover,
        cpu_chart,
        ram_chart,
        gpu_chart,
        cpu_label,
        ram_label,
        ram_detail,
        gpu_label,
    ) = render::build_sys_monitor(&capsule);

    let cpu_history = Rc::new(RefCell::new(VecDeque::from(vec![0.0; 30])));
    let ram_history = Rc::new(RefCell::new(VecDeque::from(vec![0.0; 30])));
    let gpu_history = Rc::new(RefCell::new(VecDeque::from(vec![0.0; 30])));

    render::setup_chart_draw(&cpu_chart, cpu_history.clone(), "#3b82f6");
    render::setup_chart_draw(&ram_chart, ram_history.clone(), "#a855f7");
    render::setup_chart_draw(&gpu_chart, gpu_history.clone(), "#10b981");

    let cpu_history_loop = cpu_history.clone();
    let ram_history_loop = ram_history.clone();
    let gpu_history_loop = gpu_history.clone();
    let cpu_chart_loop = cpu_chart.clone();
    let ram_chart_loop = ram_chart.clone();
    let gpu_chart_loop = gpu_chart.clone();
    let sys_label_clone = sys_label.clone();
    let cpu_label_clone = cpu_label.clone();
    let ram_label_clone = ram_label.clone();
    let ram_detail_clone = ram_detail.clone();
    let gpu_label_clone = gpu_label.clone();

    // Subscribe to the unified monitor service
    let monitor_rx = subscribe_monitor();
    monitor_rx.attach(None, move |snapshot: MonitorSnapshot| {
        let sys_template = babydra_core::i18n::trans("sysmon.cpu_ram");
        sys_label_clone.set_text(
            &sys_template
                .replace("{cpu}", &format!("{:.0}", snapshot.cpu_percent))
                .replace("{ram}", &format!("{:.0}", snapshot.ram_percent)),
        );

        cpu_label_clone.set_text(&format!(
            "{}: {:.1}%",
            babydra_core::i18n::trans("panel.cpu_load"),
            snapshot.cpu_percent
        ));
        ram_label_clone.set_text(&format!(
            "{}: {:.1}%",
            babydra_core::i18n::trans("panel.ram_usage"),
            snapshot.ram_percent
        ));
        ram_detail_clone.set_text(&format!("{:.2} GB / {:.2} GB", snapshot.ram_used_gb, snapshot.ram_total_gb));
        gpu_label_clone.set_text(&format!(
            "{}: {:.1}%",
            babydra_core::i18n::trans("panel.gpu_usage"),
            snapshot.gpu_percent
        ));

        {
            let mut hist = cpu_history_loop.borrow_mut();
            hist.pop_front();
            hist.push_back(snapshot.cpu_percent);
        }
        cpu_chart_loop.queue_draw();

        {
            let mut hist = ram_history_loop.borrow_mut();
            hist.pop_front();
            hist.push_back(snapshot.ram_percent);
        }
        ram_chart_loop.queue_draw();

        {
            let mut hist = gpu_history_loop.borrow_mut();
            hist.pop_front();
            hist.push_back(snapshot.gpu_percent);
        }
        gpu_chart_loop.queue_draw();

        glib::ControlFlow::Continue
    });

    let cpu_chart_hover = cpu_chart.clone();
    let ram_chart_hover = ram_chart.clone();
    let gpu_chart_hover = gpu_chart.clone();
    let popover_enter = popover.clone();

    let motion_controller = gtk4::EventControllerMotion::new();
    motion_controller.connect_enter(move |_, _, _| {
        // Just show the popover on hover - data is already updated via the monitor service
        cpu_chart_hover.queue_draw();
        ram_chart_hover.queue_draw();
        gpu_chart_hover.queue_draw();
        popover_enter.popup();
    });

    let popover_leave = popover.clone();
    motion_controller.connect_leave(move |_| {
        popover_leave.popdown();
    });

    capsule.add_controller(motion_controller);
    capsule
}