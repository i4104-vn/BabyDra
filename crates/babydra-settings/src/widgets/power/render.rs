use gtk4::prelude::*;
use gtk4::{Box, Button, Grid, Label, Orientation, Overlay, ProgressBar, ScrolledWindow};
use babydra_utils::components::modal::PasswordDialog;
use babydra_common::{PerformanceProfile, BatteryInfo};

pub struct PowerWidget {
    pub root: Overlay,
    pub battery_card: Box,
    pub profile_balanced_btn: Button,
    pub profile_normal_btn: Button,
    pub profile_high_btn: Button,
}

fn create_profile_button(title_key: &str, subtitle_key: &str, icon_name: &str) -> Button {
    let btn = Button::new();
    btn.add_css_class("perf-profile-row-btn");
    btn.set_cursor_from_name(Some("pointer"));

    let col_box = Box::new(Orientation::Vertical, 6);
    col_box.set_valign(gtk4::Align::Center);
    col_box.set_halign(gtk4::Align::Center);

    let icon_badge = crate::widgets::helpers::create_icon_badge(icon_name, 22, true);
    icon_badge.set_halign(gtk4::Align::Center);
    icon_badge.set_margin_bottom(2);

    let t_lbl = Label::new(Some(&babydra_common::i18n::t(title_key)));
    t_lbl.add_css_class("settings-row-title");
    t_lbl.set_halign(gtk4::Align::Center);
    t_lbl.set_justify(gtk4::Justification::Center);

    let s_lbl = Label::new(Some(&babydra_common::i18n::t(subtitle_key)));
    s_lbl.add_css_class("settings-row-desc");
    s_lbl.set_halign(gtk4::Align::Center);
    s_lbl.set_justify(gtk4::Justification::Center);
    s_lbl.set_wrap(true);
    s_lbl.set_lines(2);
    s_lbl.set_ellipsize(gtk4::pango::EllipsizeMode::End);

    col_box.append(&icon_badge);
    col_box.append(&t_lbl);
    col_box.append(&s_lbl);

    btn.set_child(Some(&col_box));
    btn
}

pub fn update_profile_selection(
    balanced_btn: &Button,
    normal_btn: &Button,
    high_btn: &Button,
    current: PerformanceProfile,
) {
    let buttons = [
        (PerformanceProfile::Balanced, balanced_btn),
        (PerformanceProfile::Normal, normal_btn),
        (PerformanceProfile::HighPerformance, high_btn),
    ];

    for (prof, btn) in buttons {
        let is_active = prof == current;
        if is_active {
            btn.add_css_class("active");
        } else {
            btn.remove_css_class("active");
        }
    }
}

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

    let title_lbl = Label::new(Some(&babydra_common::i18n::t("settings.power_battery_status")));
    title_lbl.add_css_class("settings-section-title");
    title_lbl.set_halign(gtk4::Align::Start);
    title_lbl.set_margin_top(0);
    title_lbl.set_margin_bottom(6);
    left_box.append(&title_lbl);

    let left_h_box = Box::new(Orientation::Horizontal, 14);
    left_h_box.set_valign(gtk4::Align::Center);

    let huge_battery = babydra_utils::ui::battery::create_battery_drawing_area(info.percentage, info.is_charging, 96, 52);

    let left_text_box = Box::new(Orientation::Vertical, 2);
    left_text_box.set_valign(gtk4::Align::Center);

    if info.is_ac_only {
        let hero_label = Label::new(Some(&babydra_common::i18n::t("settings.power_direct_ac")));
        hero_label.add_css_class("settings-row-title");
        hero_label.set_halign(gtk4::Align::Start);

        let sub_label = Label::new(Some(&babydra_common::i18n::t("settings.power_ac_connected_sub")));
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
        (babydra_common::i18n::t("settings.power_manufacturer"), info.manufacturer.clone()),
        (babydra_common::i18n::t("settings.power_model"), info.model_name.clone()),
        (babydra_common::i18n::t("settings.power_serial"), info.serial_number.clone()),
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
        babydra_common::i18n::t("settings.power_ac_badge")
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

        let fill_color = babydra_utils::ui::battery::get_battery_color_hex(info.percentage, info.is_charging);

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

    let specs: Vec<(String, Option<String>)> = if info.is_ac_only {
        vec![
            (babydra_common::i18n::t("settings.power_active_profile"), info.active_profile),
            (babydra_common::i18n::t("settings.power_power_source"), info.power_source),
            (babydra_common::i18n::t("settings.power_system_type"), Some(babydra_common::i18n::t("settings.power_desktop_mains"))),
            (babydra_common::i18n::t("settings.power_battery_health"), info.health),
            (babydra_common::i18n::t("settings.power_battery_tech"), info.technology),
        ]
    } else {
        vec![
            (babydra_common::i18n::t("settings.power_active_profile"), info.active_profile),
            (babydra_common::i18n::t("settings.power_battery_health"), info.health),
            (babydra_common::i18n::t("settings.power_power_source"), info.power_source),
            (babydra_common::i18n::t("settings.power_energy_rate"), info.energy_rate),
            (babydra_common::i18n::t("settings.power_voltage"), info.voltage),
            (babydra_common::i18n::t("settings.power_capacity"), info.capacity_wh),
            (babydra_common::i18n::t("settings.power_design_capacity"), info.design_capacity),
            (babydra_common::i18n::t("settings.power_temperature"), info.temperature),
            (babydra_common::i18n::t("settings.power_cycle_count"), info.cycle_count.map(|c| format!("{} cycles", c))),
            (babydra_common::i18n::t("settings.power_battery_tech"), info.technology),
        ]
    };

    let mut spec_row = 0;
    for (label_text, val_opt) in specs {
        if let Some(val) = val_opt {
            let lbl = Label::new(Some(&label_text));
            lbl.add_css_class("spec-label");
            lbl.set_halign(gtk4::Align::Start);

            let val_lbl = Label::new(Some(&val));
            val_lbl.add_css_class("settings-row-title");
            val_lbl.set_halign(gtk4::Align::End);

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

pub fn build() -> (PowerWidget, PasswordDialog) {
    let overlay = Overlay::new();
    overlay.set_vexpand(true);
    overlay.set_valign(gtk4::Align::Fill);

    let main_box = Box::new(Orientation::Vertical, 12);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    let header_box = Box::new(Orientation::Vertical, 2);
    header_box.set_margin_bottom(4);

    let title_lbl = Label::new(Some(&babydra_common::i18n::t("settings.power_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);

    let desc_lbl = Label::new(Some(&babydra_common::i18n::t("settings.power_desc")));
    desc_lbl.add_css_class("settings-page-subtitle");
    desc_lbl.set_halign(gtk4::Align::Start);

    header_box.append(&title_lbl);
    header_box.append(&desc_lbl);
    main_box.append(&header_box);

    let content_box = Box::new(Orientation::Vertical, 12);

    let battery_card = Box::new(Orientation::Vertical, 8);
    battery_card.add_css_class("glass-panel");
    content_box.append(&battery_card);

    let perf_section = Box::new(Orientation::Vertical, 8);
    perf_section.add_css_class("glass-panel");
    perf_section.set_vexpand(false);

    let perf_title = Label::new(Some(&babydra_common::i18n::t("settings.power_perf_profile")));
    perf_title.add_css_class("settings-section-title");
    perf_title.set_halign(gtk4::Align::Start);
    perf_title.set_margin_top(0);
    perf_title.set_margin_bottom(2);

    let perf_desc = Label::new(Some(&babydra_common::i18n::t("settings.power_perf_desc")));
    perf_desc.add_css_class("settings-row-desc");
    perf_desc.set_halign(gtk4::Align::Start);

    perf_section.append(&perf_title);
    perf_section.append(&perf_desc);

    let options_box = Box::new(Orientation::Horizontal, 8);
    options_box.set_homogeneous(true);
    options_box.set_vexpand(false);
    options_box.set_margin_top(4);

    let profile_balanced_btn = create_profile_button(
        "settings.power_balanced",
        "settings.power_balanced_desc",
        "sliders",
    );
    let profile_normal_btn = create_profile_button(
        "settings.power_saver",
        "settings.power_saver_desc",
        "history",
    );
    let profile_high_btn = create_profile_button(
        "settings.power_high",
        "settings.power_high_desc",
        "cog",
    );

    options_box.append(&profile_normal_btn);
    options_box.append(&profile_balanced_btn);
    options_box.append(&profile_high_btn);

    perf_section.append(&options_box);
    content_box.append(&perf_section);

    let scroll = ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&content_box));

    main_box.append(&scroll);
    overlay.set_child(Some(&main_box));

    let auth_dialog = PasswordDialog::new(
        "Authentication Required",
        "Enter sudo password to change CPU performance profile:",
    );
    overlay.add_overlay(&auth_dialog.container);

    let widget = PowerWidget {
        root: overlay,
        battery_card,
        profile_balanced_btn,
        profile_normal_btn,
        profile_high_btn,
    };

    (widget, auth_dialog)
}
