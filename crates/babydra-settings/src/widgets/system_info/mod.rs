//! System specifications overview and update tab.

<<<<<<< HEAD
=======
use std::process::Command;
>>>>>>> hard-develop
use sysinfo::System;
use babydra_common::services::system::gpu::get_gpu_info;
use babydra_common::models::SystemInfoData;

mod render;

pub fn create_system_widget() -> gtk4::Widget {
    // Render UI instantly with initial placeholder values (0ms blocking!)
    let (main_box, labels) = render::build_system_ui(
        "BabyDra Linux",
        "Linux",
        "...",
        "Loading...",
        "Loading...",
        "...",
        "...",
        "...",
    );

    // Fetch heavy system info asynchronously off the main GTK GUI thread
    let (tx, rx) = std::sync::mpsc::channel::<SystemInfoData>();
    std::thread::spawn(move || {
        let mut sys = System::new_all();
        sys.refresh_all();

<<<<<<< HEAD
        let hostname = System::host_name().unwrap_or_else(|| "localhost".to_string());
        let os_name = System::name().unwrap_or_else(|| "Arch Linux".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let cpu_model = sys.cpus().first().map(|cpu| cpu.brand()).unwrap_or("Intel/AMD CPU").trim().to_string();
=======
    let total_mem_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let memory_text = format!("{:.1} GB", total_mem_gb);
>>>>>>> hard-develop

        let total_mem_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        let memory_text = format!("{:.1} GB", total_mem_gb);

        let gpu_info = get_gpu_info();

        let uptime_secs = System::uptime();
        let days = uptime_secs / 86400;
        let hours = (uptime_secs % 86400) / 3600;
        let mins = (uptime_secs % 3600) / 60;

        let uptime_text = if days > 0 {
            format!("{}d {}h {}m", days, hours, mins)
        } else if hours > 0 {
            format!("{}h {}m", hours, mins)
        } else {
            format!("{}m", mins)
        };

        let cpu_arch = System::cpu_arch().unwrap_or_else(|| "x86_64".to_string());

        let _ = tx.send(SystemInfoData {
            hostname,
            os_name,
            kernel_version,
            cpu_model,
            gpu_info,
            memory_text,
            uptime_text,
            cpu_arch,
        });
    });

    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(50), move || {
        if let Ok(data) = rx.try_recv() {
            let display_host = if !data.hostname.is_empty() && data.hostname != "localhost" {
                &data.hostname
            } else {
                "BabyDra Linux"
            };
            labels.os_label.set_text(display_host);

            let sub_title = format!("{} ({}) • Kernel {}", data.os_name, data.cpu_arch, data.kernel_version);
            labels.sub_label.set_text(&sub_title);

            let formatted_uptime = babydra_common::i18n::t("settings.up_time").replace("{}", &data.uptime_text);
            labels.uptime_lbl.set_text(&formatted_uptime);

            labels.kernel_lbl.set_text(&data.kernel_version);
            labels.cpu_lbl.set_text(&data.cpu_model);
            labels.mem_lbl.set_text(&data.memory_text);
            labels.gpu_lbl.set_text(&data.gpu_info);

            gtk4::glib::ControlFlow::Break
        } else {
            gtk4::glib::ControlFlow::Continue
        }
    });

    main_box.into()
}
