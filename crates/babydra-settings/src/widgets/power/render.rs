use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, Overlay, ProgressBar};
use babydra_utils::components::modal::PasswordDialog;
use babydra_common::{PerformanceProfile, BatteryInfo};

pub struct PowerWidget {
    pub root: Overlay,
    pub battery_card: Box,
    pub profile_balanced_btn: Button,
    pub profile_normal_btn: Button,
    pub profile_high_btn: Button,
    pub status_badge: Label,
}

fn create_profile_button(title: &str, subtitle: &str) -> Button {
    let btn = Button::new();
    btn.add_css_class("audio-menu-item-btn");
    btn.add_css_class("perf-profile-row-btn");
    btn.set_cursor_from_name(Some("pointer"));

    let row_box = Box::new(Orientation::Horizontal, 12);
    row_box.set_margin_top(8);
    row_box.set_margin_bottom(8);
    row_box.set_margin_start(8);
    row_box.set_margin_end(8);

    let icon = babydra_utils::ui::icon::get_icon_colored("performance", 20, "#ffffff");
    icon.set_valign(gtk4::Align::Center);

    let text_box = Box::new(Orientation::Vertical, 2);
    text_box.set_hexpand(true);

    let t_lbl = Label::new(Some(title));
    t_lbl.add_css_class("settings-row-title");
    t_lbl.set_halign(gtk4::Align::Start);

    let s_lbl = Label::new(Some(subtitle));
    s_lbl.add_css_class("settings-row-desc");
    s_lbl.set_halign(gtk4::Align::Start);

    text_box.append(&t_lbl);
    text_box.append(&s_lbl);

    let check_lbl = Label::new(Some("✓"));
    check_lbl.add_css_class("audio-menu-item-check");
    check_lbl.add_css_class("profile-check");
    check_lbl.set_valign(gtk4::Align::Center);
    check_lbl.set_visible(false);

    row_box.append(&icon);
    row_box.append(&text_box);
    row_box.append(&check_lbl);

    btn.set_child(Some(&row_box));
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

        if let Some(child) = btn.child() {
            if let Ok(row_box) = child.downcast::<Box>() {
                if let Some(check_child) = row_box.last_child() {
                    check_child.set_visible(is_active);
                }
            }
        }
    }
}

pub fn update_battery_card_ui(card: &Box, info_opt: Option<BatteryInfo>) {
    while let Some(child) = card.first_child() {
        card.remove(&child);
    }

    if let Some(info) = info_opt {
        let h_box = Box::new(Orientation::Horizontal, 16);
        h_box.set_valign(gtk4::Align::Center);

        let color = if info.is_charging || info.percentage > 60 {
            "#2ec27e"
        } else if info.percentage >= 20 {
            "#f5a623"
        } else {
            "#e05252"
        };

        let icon_name = if info.is_charging {
            "battery-charging"
        } else if info.percentage > 80 {
            "battery-full"
        } else if info.percentage < 20 {
            "battery-low"
        } else {
            "battery"
        };

        let icon = babydra_utils::ui::icon::get_icon_colored(icon_name, 48, color);

        let info_vbox = Box::new(Orientation::Vertical, 4);
        info_vbox.set_hexpand(true);

        let pct_label = Label::new(Some(&format!("{}%", info.percentage)));
        pct_label.add_css_class("battery-percentage-label");
        pct_label.set_halign(gtk4::Align::Start);
        let provider = gtk4::CssProvider::new();
        provider.load_from_data(&format!(".battery-percentage-label {{ font-size: 32px; font-weight: 800; color: {}; }}", color));
        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().unwrap(),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let status_lbl = Label::new(Some(&info.status_text));
        status_lbl.add_css_class("settings-row-desc");
        status_lbl.set_halign(gtk4::Align::Start);

        info_vbox.append(&pct_label);
        info_vbox.append(&status_lbl);

        if let Some(ref time_str) = info.time_remaining {
            let time_lbl = Label::new(Some(time_str));
            time_lbl.add_css_class("settings-row-desc");
            time_lbl.set_halign(gtk4::Align::Start);
            info_vbox.append(&time_lbl);
        }

        h_box.append(&icon);
        h_box.append(&info_vbox);

        let progress = ProgressBar::new();
        progress.set_fraction(info.percentage as f64 / 100.0);
        progress.add_css_class("battery-progress-bar");
        progress.set_margin_top(8);

        card.append(&h_box);
        card.append(&progress);
    } else {
        let h_box = Box::new(Orientation::Horizontal, 16);
        h_box.set_valign(gtk4::Align::Center);

        let icon = babydra_utils::ui::icon::get_icon_colored("power", 36, "#3584e4");

        let info_vbox = Box::new(Orientation::Vertical, 4);
        info_vbox.set_hexpand(true);

        let title_lbl = Label::new(Some("AC Power Connected"));
        title_lbl.add_css_class("settings-row-title");
        title_lbl.set_halign(gtk4::Align::Start);

        let sub_lbl = Label::new(Some("Desktop computer running on direct power source"));
        sub_lbl.add_css_class("settings-row-desc");
        sub_lbl.set_halign(gtk4::Align::Start);

        info_vbox.append(&title_lbl);
        info_vbox.append(&sub_lbl);

        h_box.append(&icon);
        h_box.append(&info_vbox);

        card.append(&h_box);
    }
}

pub fn build() -> (PowerWidget, PasswordDialog) {
    let overlay = Overlay::new();
    let container = Box::new(Orientation::Vertical, 16);
    container.add_css_class("settings-page-container");

    let header_box = Box::new(Orientation::Vertical, 4);
    let title_lbl = Label::new(Some(&babydra_common::i18n::t("settings.power_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);

    let desc_lbl = Label::new(Some(&babydra_common::i18n::t("settings.power_desc")));
    desc_lbl.add_css_class("settings-page-subtitle");
    desc_lbl.set_halign(gtk4::Align::Start);

    header_box.append(&title_lbl);
    header_box.append(&desc_lbl);
    container.append(&header_box);

    let status_badge = Label::new(None);
    status_badge.add_css_class("status-badge");
    status_badge.set_halign(gtk4::Align::Start);
    status_badge.set_visible(false);
    container.append(&status_badge);

    let battery_card = Box::new(Orientation::Vertical, 12);
    battery_card.add_css_class("settings-card");
    container.append(&battery_card);

    let perf_section = Box::new(Orientation::Vertical, 12);
    perf_section.add_css_class("settings-card");

    let perf_title = Label::new(Some("Performance Profile"));
    perf_title.add_css_class("settings-section-title");
    perf_title.set_halign(gtk4::Align::Start);

    let perf_desc = Label::new(Some("Select a profile to balance CPU power usage and system performance"));
    perf_desc.add_css_class("settings-section-desc");
    perf_desc.set_halign(gtk4::Align::Start);

    perf_section.append(&perf_title);
    perf_section.append(&perf_desc);

    let options_box = Box::new(Orientation::Vertical, 8);

    let profile_balanced_btn = create_profile_button("Balanced", "Standard balanced performance for everyday use");
    let profile_normal_btn = create_profile_button("Power Saver", "Lowers CPU energy consumption to extend battery life");
    let profile_high_btn = create_profile_button("High Performance", "Maximizes CPU frequency for maximum performance");

    options_box.append(&profile_balanced_btn);
    options_box.append(&profile_normal_btn);
    options_box.append(&profile_high_btn);

    perf_section.append(&options_box);
    container.append(&perf_section);

    overlay.set_child(Some(&container));

    let auth_dialog = PasswordDialog::new("Authentication Required", "Enter sudo password to change CPU performance profile:");
    overlay.add_overlay(&auth_dialog.container);

    let widget = PowerWidget {
        root: overlay,
        battery_card,
        profile_balanced_btn,
        profile_normal_btn,
        profile_high_btn,
        status_badge,
    };

    (widget, auth_dialog)
}
