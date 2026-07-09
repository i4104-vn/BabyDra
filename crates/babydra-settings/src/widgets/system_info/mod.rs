//! System specifications overview and update tab.

use gtk4::prelude::*;
use std::process::Command;
use sysinfo::System;

pub fn create_system_widget() -> gtk4::Box {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 16);
    main_box.set_margin_start(10);
    main_box.set_margin_end(10);

    let title_lbl = gtk4::Label::new(Some("Thông tin hệ thống"));
    title_lbl.add_css_class("settings-title");
    title_lbl.set_halign(gtk4::Align::Start);
    main_box.append(&title_lbl);

    // Fetch system statistics
    let mut sys = System::new_all();
    sys.refresh_all();

    let hostname = System::host_name().unwrap_or_else(|| "localhost".to_string());
    let os_name = System::name().unwrap_or_else(|| "Arch Linux".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    let cpu_model = sys.cpus().first().map(|cpu| cpu.brand()).unwrap_or("Intel/AMD CPU").trim().to_string();

    let total_mem_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let used_mem_gb = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let memory_text = format!("{:.1} GB / {:.1} GB ({:.0}%)", used_mem_gb, total_mem_gb, (used_mem_gb / total_mem_gb) * 100.0);

    // Retrieve primary GPU information using lspci
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

    // Disk Usage
    let mut disk_text = "Unknown".to_string();
    if let Ok(output) = Command::new("df").arg("-h").arg("/").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = stdout.lines().collect();
        if lines.len() >= 2 {
            let parts: Vec<&str> = lines[1].split_whitespace().collect();
            if parts.len() >= 5 {
                disk_text = format!("{} / {} (Sử dụng {})", parts[2], parts[1], parts[4]);
            }
        }
    }

    let stats_card = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    stats_card.add_css_class("settings-card");

    let mut add_info_row = |label: &str, value: &str| {
        let row = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
        row.set_margin_top(4);
        row.set_margin_bottom(4);

        let lbl = gtk4::Label::new(Some(label));
        lbl.add_css_class("settings-label");
        lbl.set_halign(gtk4::Align::Start);
        row.append(&lbl);

        let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        spacer.set_hexpand(true);
        row.append(&spacer);

        let val = gtk4::Label::new(Some(value));
        val.add_css_class("settings-desc");
        val.set_halign(gtk4::Align::End);
        row.append(&val);

        stats_card.append(&row);
    };

    add_info_row("Tên máy (Hostname)", &hostname);
    add_info_row("Hệ điều hành (OS)", &os_name);
    add_info_row("Nhân Kernel", &kernel_version);
    add_info_row("Bộ vi xử lý (CPU)", &cpu_model);
    add_info_row("Card đồ họa (GPU)", &gpu_info);
    add_info_row("Bộ nhớ RAM", &memory_text);
    add_info_row("Ổ đĩa hệ thống (/)", &disk_text);

    main_box.append(&stats_card);

    // --- Update Card ---
    let update_card = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
    update_card.add_css_class("settings-card");
    update_card.set_valign(gtk4::Align::Center);

    let update_lbl_box = gtk4::Box::new(gtk4::Orientation::Vertical, 2);
    let update_title = gtk4::Label::new(Some("Cập nhật hệ thống"));
    update_title.add_css_class("settings-label");
    update_title.set_halign(gtk4::Align::Start);
    let update_desc = gtk4::Label::new(Some("Kiểm tra và cài đặt các bản nâng cấp Arch Linux mới nhất"));
    update_desc.add_css_class("settings-desc");
    update_desc.set_halign(gtk4::Align::Start);
    update_lbl_box.append(&update_title);
    update_lbl_box.append(&update_desc);
    update_card.append(&update_lbl_box);

    let update_spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    update_spacer.set_hexpand(true);
    update_card.append(&update_spacer);

    let update_btn = gtk4::Button::with_label("Cập nhật ngay");
    update_btn.set_valign(gtk4::Align::Center);
    update_btn.add_css_class("suggested-action");
    update_card.append(&update_btn);

    main_box.append(&update_card);

    update_btn.connect_clicked(|_| {
        // Launch a terminal window executing pacman/yay update
        let _ = Command::new("kitty")
            .args(&["-e", "yay", "-Syu"])
            .spawn();
    });

    main_box
}
