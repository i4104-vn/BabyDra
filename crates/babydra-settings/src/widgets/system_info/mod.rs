//! System specifications overview and update tab.

use sysinfo::System;
use babydra_common::services::system::gpu::get_gpu_info;

mod render;

pub fn create_system_widget() -> gtk4::Box {
    let mut sys = System::new_all();
    sys.refresh_all();

    let hostname = System::host_name().unwrap_or_else(|| "localhost".to_string());
    let os_name = System::name().unwrap_or_else(|| "Arch Linux".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let cpu_model = sys.cpus().first().map(|cpu| cpu.brand()).unwrap_or("Intel/AMD CPU").trim().to_string();

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

    render::build_system_ui(
        &hostname,
        &os_name,
        &kernel_version,
        &cpu_model,
        &gpu_info,
        &memory_text,
        &uptime_text,
        &cpu_arch,
    )
}
