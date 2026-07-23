//! System specifications overview and update tab.

use std::process::Command;
use sysinfo::System;

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

    let mut gpu_info = "lspci lookup failed".to_string();
    if let Ok(output) = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i 'vga\\|3d' | cut -d: -f3 | head -n 1")
        .output()
    {
        let raw = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !raw.is_empty() {
            gpu_info = raw;
        }
    }

    let mut disk_text = "Unknown".to_string();
    let mut disk_percent = 0.0;
    if let Ok(output) = Command::new("df").arg("-h").arg("/").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.len() >= 2 {
            let parts: Vec<&str> = lines[1].split_whitespace().collect();
            if parts.len() >= 5 {
                disk_text = format!("{} / {}", parts[2], parts[1]);
                if let Ok(val) = parts[4].replace("%", "").parse::<f64>() {
                    disk_percent = val;
                }
            }
        }
    }

    render::build_system_ui(
        &hostname,
        &os_name,
        &kernel_version,
        &cpu_model,
        &gpu_info,
        &memory_text,
        &disk_text,
        disk_percent,
    )
}
