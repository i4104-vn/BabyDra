use babydra_core::BatteryInfo;
use gtk4::prelude::*;
use gtk4::{Box, Grid, Label, Orientation, ProgressBar};

/// Returns the current `cpu frequency`.
pub fn get_cpu_frequency() -> Option<(f64, String)> {
    let mut max_freq = 0.0;
    if let Ok(entries) = std::fs::read_dir("/sys/devices/system/cpu") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with("cpu") && name[3..].chars().all(char::is_numeric) {
                    let freq_path = path.join("cpufreq/scaling_cur_freq");
                    if freq_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&freq_path) {
                            if let Ok(khz) = content.trim().parse::<f64>() {
                                let ghz = khz / 1_000_000.0;
                                if ghz > max_freq {
                                    max_freq = ghz;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if max_freq > 0.0 {
        Some((max_freq, format!("{:.2} GHz", max_freq)))
    } else {
        None
    }
}

/// Update battery card ui.
pub fn update_battery_card_ui(card: &Box, info_opt: Option<BatteryInfo>) {
    while let Some(child) = card.first_child() {
        card.remove(&child);
    }

    let info = match info_opt {
        Some(info) => {
            card.set_visible(true);
            info
        }
        None => {
            card.set_visible(false);
            return;
        }
    };

    let split_grid = Grid::new();
    split_grid.set_column_spacing(24);
    split_grid.set_column_homogeneous(true);

    // ── LEFT COLUMN (50%): Title, Cairo Graphic, Hero Text, Time & HW Info ──
    let left_box = Box::new(Orientation::Vertical, 6);
    left_box.set_valign(gtk4::Align::Start);

    let title_lbl = Label::new(Some(&babydra_core::i18n::t(
        "settings.power_battery_status",
    )));
    title_lbl.add_css_class("settings-section-title");
    title_lbl.set_halign(gtk4::Align::Start);
    title_lbl.set_margin_top(0);
    title_lbl.set_margin_bottom(6);
    left_box.append(&title_lbl);

    let left_h_box = Box::new(Orientation::Horizontal, 14);
    left_h_box.set_valign(gtk4::Align::Center);

    let huge_battery = babydra_ui_kit::ui::battery::create_battery_drawing_area(
        info.percentage,
        info.is_charging,
        96,
        52,
    );

    let left_text_box = Box::new(Orientation::Vertical, 2);
    left_text_box.set_valign(gtk4::Align::Center);

    if info.is_ac_only {
        let hero_label = Label::new(Some(&babydra_core::i18n::t("settings.power_direct_ac")));
        hero_label.add_css_class("settings-row-title");
        hero_label.set_halign(gtk4::Align::Start);

        let sub_label = Label::new(Some(&babydra_core::i18n::t(
            "settings.power_ac_connected_sub",
        )));
        sub_label.add_css_class("settings-row-desc");
        sub_label.set_halign(gtk4::Align::Start);

        left_text_box.append(&hero_label);
        left_text_box.append(&sub_label);
    } else {
        let pct_label = Label::new(Some(&format!("{}%", info.percentage)));
        pct_label.add_css_class("hero-hostname");
        pct_label.set_halign(gtk4::Align::Start);

        let status_lbl = Label::new(Some(&info.status_text));
        status_lbl.add_css_class("settings-row-title");
        status_lbl.set_halign(gtk4::Align::Start);

        left_text_box.append(&pct_label);
        left_text_box.append(&status_lbl);

        if let Some(ref time_str) = info.time_remaining {
            let time_lbl = Label::new(Some(time_str));
            time_lbl.add_css_class("settings-row-desc");
            time_lbl.set_halign(gtk4::Align::Start);
            left_text_box.append(&time_lbl);
        }
    }

    left_h_box.append(&huge_battery);
    left_h_box.append(&left_text_box);
    left_box.append(&left_h_box);

    // Optional Hardware Info in Left Box
    let hw_specs: Vec<(String, Option<String>)> = vec![
        (
            babydra_core::i18n::t("settings.power_manufacturer"),
            info.manufacturer.clone(),
        ),
        (
            babydra_core::i18n::t("settings.power_model"),
            info.model_name.clone(),
        ),
        (
            babydra_core::i18n::t("settings.power_serial"),
            info.serial_number.clone(),
        ),
    ];

    let mut has_hw_info = false;
    let hw_grid = Grid::new();
    hw_grid.set_column_spacing(20);
    hw_grid.set_row_spacing(4);
    hw_grid.set_halign(gtk4::Align::Start);
    hw_grid.set_margin_top(6);

    let mut hw_row = 0;
    for (label_text, val_opt) in hw_specs {
        if let Some(val) = val_opt {
            has_hw_info = true;
            let lbl = Label::new(Some(&label_text));
            lbl.add_css_class("spec-label");
            lbl.set_halign(gtk4::Align::Start);

            let val_lbl = Label::new(Some(&val));
            val_lbl.add_css_class("settings-row-title");
            val_lbl.set_halign(gtk4::Align::Start);

            hw_grid.attach(&lbl, 0, hw_row, 1, 1);
            hw_grid.attach(&val_lbl, 1, hw_row, 1, 1);
            hw_row += 1;
        }
    }

    if has_hw_info {
        let sep = gtk4::Separator::new(Orientation::Horizontal);
        sep.add_css_class("profile-separator");
        sep.set_margin_top(4);
        sep.set_margin_bottom(4);
        left_box.append(&sep);
        left_box.append(&hw_grid);
    }

    // ── RIGHT COLUMN (50%): Badge Pill, Progress Bar (only if battery) & Specs Grid ──
    let right_box = Box::new(Orientation::Vertical, 6);
    right_box.set_valign(gtk4::Align::Start);

    // Top Right Status Badge Pill (aligned with left title)
    let right_header = Box::new(Orientation::Horizontal, 8);
    right_header.set_margin_bottom(6);
    let dummy_space = Box::new(Orientation::Horizontal, 0);
    dummy_space.set_hexpand(true);
    right_header.append(&dummy_space);

    let badge_text = if info.is_ac_only {
        babydra_core::i18n::t("settings.power_ac_badge")
    } else {
        info.status_text.clone()
    };

    let status_badge = Label::new(Some(&badge_text));
    if info.is_charging || info.is_ac_only {
        status_badge.add_css_class("battery-badge-charging");
    } else {
        status_badge.add_css_class("battery-badge-discharging");
    }
    status_badge.set_valign(gtk4::Align::Center);
    right_header.append(&status_badge);

    right_box.append(&right_header);

    // Battery Level Progress Bar (only show for physical batteries, hide for Direct AC)
    if !info.is_ac_only {
        let progress = ProgressBar::new();
        progress.set_fraction(info.percentage as f64 / 100.0);
        progress.add_css_class("battery-level-bar");

        let fill_color =
            babydra_ui_kit::ui::battery::get_battery_color_hex(info.percentage, info.is_charging);

        let provider = gtk4::CssProvider::new();
        provider.load_from_data(&format!(
            "progressbar.battery-level-bar progress {{ background-color: {}; }}",
            fill_color
        ));
        if let Some(display) = gtk4::gdk::Display::default() {
            gtk4::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
        right_box.append(&progress);
    }

    // Specs Grid (Tightly grouped on the right side)
    let specs_grid = Grid::new();
    specs_grid.set_column_spacing(20);
    specs_grid.set_row_spacing(4);
    specs_grid.set_halign(gtk4::Align::End);

    let mut specs: Vec<(String, Option<String>)> = if info.is_ac_only {
        vec![
            (
                babydra_core::i18n::t("settings.power_active_profile"),
                info.active_profile,
            ),
            (
                babydra_core::i18n::t("settings.power_power_source"),
                info.power_source,
            ),
            (
                babydra_core::i18n::t("settings.power_system_type"),
                Some(babydra_core::i18n::t("settings.power_desktop_mains")),
            ),
            (
                babydra_core::i18n::t("settings.power_battery_health"),
                info.health,
            ),
            (
                babydra_core::i18n::t("settings.power_battery_tech"),
                info.technology,
            ),
        ]
    } else {
        vec![
            (
                babydra_core::i18n::t("settings.power_active_profile"),
                info.active_profile,
            ),
            (
                babydra_core::i18n::t("settings.power_battery_health"),
                info.health,
            ),
            (
                babydra_core::i18n::t("settings.power_power_source"),
                info.power_source,
            ),
            (
                babydra_core::i18n::t("settings.power_energy_rate"),
                info.energy_rate,
            ),
            (
                babydra_core::i18n::t("settings.power_voltage"),
                info.voltage,
            ),
            (
                babydra_core::i18n::t("settings.power_capacity"),
                info.capacity_wh,
            ),
            (
                babydra_core::i18n::t("settings.power_design_capacity"),
                info.design_capacity,
            ),
            (
                babydra_core::i18n::t("settings.power_temperature"),
                info.temperature,
            ),
            (
                babydra_core::i18n::t("settings.power_cycle_count"),
                info.cycle_count.map(|c| format!("{} cycles", c)),
            ),
            (
                babydra_core::i18n::t("settings.power_battery_tech"),
                info.technology,
            ),
        ]
    };
    let mut spec_row = 0;
    for (label_text, val_opt) in specs {
        if let Some(val) = val_opt {
            let lbl = Label::new(Some(&label_text));
            lbl.add_css_class("spec-label");
            lbl.set_halign(gtk4::Align::Start);

            let val_lbl = Label::new(Some(&val));
            val_lbl.set_halign(gtk4::Align::End);
            val_lbl.set_valign(gtk4::Align::Center);

            val_lbl.add_css_class("settings-row-title");
            val_lbl.set_text(&val);

            specs_grid.attach(&lbl, 0, spec_row, 1, 1);
            specs_grid.attach(&val_lbl, 1, spec_row, 1, 1);
            spec_row += 1;
        }
    }

    right_box.append(&specs_grid);

    split_grid.attach(&left_box, 0, 0, 1, 1);
    split_grid.attach(&right_box, 1, 0, 1, 1);

    card.append(&split_grid);
}
