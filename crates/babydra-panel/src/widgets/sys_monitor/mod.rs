//! System resource monitor widget for CPU load and RAM usage statistics.
//! Periodically polls procfs files (`/proc/stat` and `/proc/meminfo`) to feed historical data charts.

use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod render;

/// Raw CPU time values used to calculate delta load values.
#[derive(Clone, Debug)]
struct CpuTime {
    total: u64,
    idle: u64,
}

/// Reads raw CPU tick numbers from `/proc/stat`.
fn get_cpu_raw() -> Option<CpuTime> {
    let file = std::fs::File::open("/proc/stat").ok()?;
    let reader = std::io::BufReader::new(file);
    if let Some(Ok(line)) = std::io::BufRead::lines(reader).next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 && parts[0] == "cpu" {
            let user: u64 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            let nice: u64 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
            let system: u64 = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
            let idle: u64 = parts.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);
            let iowait: u64 = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);
            let irq: u64 = parts.get(6).and_then(|s| s.parse().ok()).unwrap_or(0);
            let softirq: u64 = parts.get(7).and_then(|s| s.parse().ok()).unwrap_or(0);
            let steal: u64 = parts.get(8).and_then(|s| s.parse().ok()).unwrap_or(0);

            let idle_time = idle + iowait;
            let total_time = user + nice + system + idle_time + irq + softirq + steal;
            return Some(CpuTime {
                total: total_time,
                idle: idle_time,
            });
        }
    }
    None
}

/// Reads raw RAM size information from `/proc/meminfo`.
/// Returns a tuple containing `(used_gb, total_gb, usage_percent)`.
fn get_ram_usage() -> Option<(f64, f64, f64)> {
    let file = std::fs::File::open("/proc/meminfo").ok()?;
    let reader = std::io::BufReader::new(file);

    let mut mem_total = 0.0;
    let mut mem_avail = 0.0;

    for line in std::io::BufRead::lines(reader).flatten() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            if parts[0] == "MemTotal:" {
                mem_total = parts[1].parse::<f64>().unwrap_or(0.0);
            } else if parts[0] == "MemAvailable:" {
                mem_avail = parts[1].parse::<f64>().unwrap_or(0.0);
            }
        }
    }

    if mem_total > 0.0 {
        let used = mem_total - mem_avail;
        let percent = (used / mem_total) * 100.0;
        let used_gb = used / 1024.0 / 1024.0;
        let total_gb = mem_total / 1024.0 / 1024.0;
        Some((used_gb, total_gb, percent))
    } else {
        None
    }
}

/// Creates a system resource monitoring capsule.
/// Displays basic stats on hover and draws CPU/RAM utilization graphs on a popup card.
pub fn create_sys_monitor_widget() -> gtk4::Box {
    let capsule = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);

    let (sys_label, popover, cpu_chart, ram_chart, cpu_label, ram_label, ram_detail) =
        render::build_sys_monitor_ui(&capsule);

    let cpu_history = Rc::new(RefCell::new(std::collections::VecDeque::from(vec![
        0.0;
        30
    ])));
    let ram_history = Rc::new(RefCell::new(std::collections::VecDeque::from(vec![
        0.0;
        30
    ])));

    render::setup_chart_draw(&cpu_chart, cpu_history.clone(), "#3b82f6");
    render::setup_chart_draw(&ram_chart, ram_history.clone(), "#a855f7");

    let last_cpu: Rc<RefCell<Option<CpuTime>>> = Rc::new(RefCell::new(None));
    let last_cpu_clone = last_cpu.clone();
    let sys_label_clone = sys_label.clone();
    let cpu_label_clone = cpu_label.clone();
    let ram_label_clone = ram_label.clone();
    let ram_detail_clone = ram_detail.clone();

    let cpu_history_loop = cpu_history.clone();
    let ram_history_loop = ram_history.clone();
    let cpu_chart_loop = cpu_chart.clone();
    let ram_chart_loop = ram_chart.clone();

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(2000), move || {
        if let Some(current_cpu) = get_cpu_raw() {
            let mut last_cpu_borrow = last_cpu_clone.borrow_mut();
            let cpu_percent = if let Some(ref last) = *last_cpu_borrow {
                let total_diff = current_cpu.total.saturating_sub(last.total);
                let idle_diff = current_cpu.idle.saturating_sub(last.idle);
                if total_diff > 0 {
                    let used_diff = total_diff.saturating_sub(idle_diff);
                    (used_diff as f64 / total_diff as f64) * 100.0
                } else {
                    0.0
                }
            } else {
                0.0
            };
            *last_cpu_borrow = Some(current_cpu);

            let ram_info = get_ram_usage().unwrap_or((0.0, 0.0, 0.0));

            let sys_template = babydra_core::i18n::t("sysmon.cpu_ram");
            sys_label_clone.set_text(
                &sys_template
                    .replace("{cpu}", &format!("{:.0}", cpu_percent))
                    .replace("{ram}", &format!("{:.0}", ram_info.2)),
            );

            cpu_label_clone.set_text(&format!(
                "{}: {:.1}%",
                babydra_core::i18n::t("panel.cpu_load"),
                cpu_percent
            ));
            ram_label_clone.set_text(&format!(
                "{}: {:.1}%",
                babydra_core::i18n::t("panel.ram_usage"),
                ram_info.2
            ));
            ram_detail_clone.set_text(&format!(
                "{:.} GB / {:.2} GB",
                format!("{:.2}", ram_info.0),
                ram_info.1
            ));

            {
                let mut hist = cpu_history_loop.borrow_mut();
                hist.pop_front();
                hist.push_back(cpu_percent);
            }
            cpu_chart_loop.queue_draw();

            {
                let mut hist = ram_history_loop.borrow_mut();
                hist.pop_front();
                hist.push_back(ram_info.2);
            }
            ram_chart_loop.queue_draw();
        }

        gtk4::glib::ControlFlow::Continue
    });

    let motion_controller = gtk4::EventControllerMotion::new();
    let popover_enter = popover.clone();

    let last_cpu_hover = last_cpu.clone();
    let cpu_label_hover = cpu_label.clone();
    let ram_label_hover = ram_label.clone();
    let ram_detail_hover = ram_detail.clone();
    let cpu_history_hover = cpu_history.clone();
    let ram_history_hover = ram_history.clone();
    let cpu_chart_hover = cpu_chart.clone();
    let ram_chart_hover = ram_chart.clone();

    motion_controller.connect_enter(move |_, _, _| {
        let mut cpu_percent = 0.0;
        if let Some(current_cpu) = get_cpu_raw() {
            let mut last_cpu_borrow = last_cpu_hover.borrow_mut();
            cpu_percent = if let Some(ref last) = *last_cpu_borrow {
                let total_diff = current_cpu.total.saturating_sub(last.total);
                let idle_diff = current_cpu.idle.saturating_sub(last.idle);
                if total_diff > 0 {
                    let used_diff = total_diff.saturating_sub(idle_diff);
                    (used_diff as f64 / total_diff as f64) * 100.0
                } else {
                    0.0
                }
            } else {
                0.0
            };
            *last_cpu_borrow = Some(current_cpu);
            cpu_label_hover.set_text(&format!(
                "{}: {:.1}%",
                babydra_core::i18n::t("panel.cpu_load"),
                cpu_percent
            ));
        }

        let mut ram_pct = 0.0;
        if let Some(ram_info) = get_ram_usage() {
            ram_pct = ram_info.2;
            ram_label_hover.set_text(&format!(
                "{}: {:.1}%",
                babydra_core::i18n::t("panel.ram_usage"),
                ram_pct
            ));
            ram_detail_hover.set_text(&format!("{:.2} GB / {:.2} GB", ram_info.0, ram_info.1));
        }

        {
            let mut hist = cpu_history_hover.borrow_mut();
            hist.pop_front();
            hist.push_back(cpu_percent);
        }
        cpu_chart_hover.queue_draw();

        {
            let mut hist = ram_history_hover.borrow_mut();
            hist.pop_front();
            hist.push_back(ram_pct);
        }
        ram_chart_hover.queue_draw();

        popover_enter.popup();
    });

    let popover_leave = popover.clone();
    motion_controller.connect_leave(move |_| {
        popover_leave.popdown();
    });

    capsule.add_controller(motion_controller);
    capsule
}
