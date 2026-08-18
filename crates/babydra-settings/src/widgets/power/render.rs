use babydra_core::PerformanceProfile;
use babydra_ui_kit::components::modals::PasswordDialog;
use babydra_ui_kit::components::CustomSlider;
use gtk4::prelude::*;
use gtk4::{Box, Button, Label, Orientation, Overlay, ScrolledWindow};

#[derive(Clone)]
pub struct PowerWidget {
    pub root: Overlay,
    pub battery_card: Box,
    pub title_lbl: Label,
    pub desc_lbl: Label,
    pub saver_title: Label,
    pub saver_desc: Label,
    pub perf_title: Label,
    pub perf_desc: Label,
    pub profile_balanced_btn: Button,
    pub profile_balanced_title: Label,
    pub profile_balanced_desc: Label,
    pub profile_normal_btn: Button,
    pub profile_normal_title: Label,
    pub profile_normal_desc: Label,
    pub profile_high_btn: Button,
    pub profile_high_title: Label,
    pub profile_high_desc: Label,
    pub threshold_slider: CustomSlider,
    pub charge_title: Label,
    pub charge_desc: Label,
    pub charge_slider: CustomSlider,
}

/// Creates a new `profile button`.
fn create_profile_button(
    title_key: &str,
    subtitle_key: &str,
    icon_name: &str,
) -> (Button, Label, Label) {
    let btn = Button::new();
    btn.add_css_class("perf-profile-row-btn");
    btn.set_cursor_from_name(Some("pointer"));

    let col_box = Box::new(Orientation::Vertical, 6);
    col_box.set_valign(gtk4::Align::Center);
    col_box.set_halign(gtk4::Align::Center);

    let icon_badge = crate::widgets::helpers::create_icon_badge(icon_name, 22, true);
    icon_badge.set_halign(gtk4::Align::Center);
    icon_badge.set_margin_bottom(2);

    let t_lbl = Label::new(Some(&babydra_core::i18n::t(title_key)));
    t_lbl.add_css_class("settings-row-title");
    t_lbl.set_halign(gtk4::Align::Center);
    t_lbl.set_justify(gtk4::Justification::Center);

    let s_lbl = Label::new(Some(&babydra_core::i18n::t(subtitle_key)));
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
    (btn, t_lbl, s_lbl)
}

/// Update profile selection.
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

/// Update power widget labels.
pub fn update_power_widget_labels(widget: &PowerWidget) {
    widget
        .title_lbl
        .set_text(&babydra_core::i18n::t("settings.power_title"));
    widget
        .desc_lbl
        .set_text(&babydra_core::i18n::t("settings.power_desc"));
    widget
        .saver_title
        .set_text(&babydra_core::i18n::t("settings.power_auto_saver_title"));
    widget
        .perf_title
        .set_text(&babydra_core::i18n::t("settings.power_perf_profile"));
    widget
        .perf_desc
        .set_text(&babydra_core::i18n::t("settings.power_perf_desc"));

    widget
        .profile_balanced_title
        .set_text(&babydra_core::i18n::t("settings.power_balanced"));
    widget
        .profile_balanced_desc
        .set_text(&babydra_core::i18n::t("settings.power_balanced_desc"));
    widget
        .profile_normal_title
        .set_text(&babydra_core::i18n::t("settings.power_saver"));
    widget
        .profile_normal_desc
        .set_text(&babydra_core::i18n::t("settings.power_saver_desc"));
    widget
        .profile_high_title
        .set_text(&babydra_core::i18n::t("settings.power_high"));
    widget
        .profile_high_desc
        .set_text(&babydra_core::i18n::t("settings.power_high_desc"));
}

pub use super::battery_card::update_battery_card_ui;

/// Builds the power settings page and its password dialog.
pub fn build() -> (PowerWidget, PasswordDialog) {
    // Render the UI instantly with placeholder values (0ms main-thread
    // blocking) so the window appears before background battery queries finish.
    let overlay = Overlay::new();
    overlay.set_vexpand(true);
    overlay.set_valign(gtk4::Align::Fill);

    let main_box = Box::new(Orientation::Vertical, 12);
    main_box.set_vexpand(true);
    main_box.set_valign(gtk4::Align::Fill);

    let header_box = Box::new(Orientation::Vertical, 2);
    header_box.set_margin_bottom(4);

    let title_lbl = Label::new(Some(&babydra_core::i18n::t("settings.power_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);

    let desc_lbl = Label::new(Some(&babydra_core::i18n::t("settings.power_desc")));
    desc_lbl.add_css_class("settings-page-subtitle");
    desc_lbl.set_halign(gtk4::Align::Start);

    header_box.append(&title_lbl);
    header_box.append(&desc_lbl);
    main_box.append(&header_box);

    let content_box = Box::new(Orientation::Vertical, 12);

    let battery_card = Box::new(Orientation::Vertical, 8);
    battery_card.add_css_class("glass-panel");
    content_box.append(&battery_card);

    // ── BATTERY SAVER MANAGEMENT SECTION ──
    let saver_section = Box::new(Orientation::Vertical, 10);
    saver_section.add_css_class("glass-panel");
    saver_section.set_vexpand(false);

    let saver_title = Label::new(Some(&babydra_core::i18n::t(
        "settings.power_auto_saver_title",
    )));
    saver_title.add_css_class("settings-section-title");
    saver_title.set_halign(gtk4::Align::Start);

    let saver_desc = Label::new(Some(&babydra_core::i18n::t(
        "settings.power_auto_saver_desc",
    )));
    saver_desc.add_css_class("settings-row-desc");
    saver_desc.set_halign(gtk4::Align::Start);

    saver_section.append(&saver_title);
    saver_section.append(&saver_desc);

    let threshold_slider = CustomSlider::new(20, |_| {});
    saver_section.append(&threshold_slider.container);
    content_box.append(&saver_section);

    // ── CHARGE LIMIT MANAGEMENT SECTION (80% - 100%) ──
    let charge_section = Box::new(Orientation::Vertical, 6);
    charge_section.add_css_class("glass-panel");
    charge_section.set_vexpand(false);
    charge_section.set_visible(babydra_core::services::system::battery::has_charge_limit_support());

    let charge_title = Label::new(Some(&babydra_core::i18n::t(
        "settings.power_charge_limit_title",
    )));
    charge_title.add_css_class("settings-section-title");
    charge_title.set_halign(gtk4::Align::Start);

    let charge_desc = Label::new(Some(&babydra_core::i18n::t(
        "settings.power_charge_limit_desc",
    )));
    charge_desc.add_css_class("settings-row-desc");
    charge_desc.set_halign(gtk4::Align::Start);

    let charge_slider = CustomSlider::new_range(80, 100, 5, 80, |_| {});

    charge_section.append(&charge_title);
    charge_section.append(&charge_desc);
    charge_section.append(&charge_slider.container);
    content_box.append(&charge_section);

    // ── PERFORMANCE PROFILE SECTION ──
    let perf_section = Box::new(Orientation::Vertical, 8);
    perf_section.add_css_class("glass-panel");
    perf_section.set_vexpand(false);

    let perf_title = Label::new(Some(&babydra_core::i18n::t("settings.power_perf_profile")));
    perf_title.add_css_class("settings-section-title");
    perf_title.set_halign(gtk4::Align::Start);
    perf_title.set_margin_top(0);
    perf_title.set_margin_bottom(2);

    let perf_header = Box::new(Orientation::Horizontal, 8);
    let perf_spacer = Box::new(Orientation::Horizontal, 0);
    perf_spacer.set_hexpand(true);

    perf_header.append(&perf_title);
    perf_header.append(&perf_spacer);

    if let Some((ghz, freq)) = super::battery_card::get_cpu_frequency() {
        let cpu_freq_lbl = Label::new(Some(&freq));
        cpu_freq_lbl.add_css_class("cpu-badge");
        cpu_freq_lbl.set_valign(gtk4::Align::Center);
        cpu_freq_lbl.set_margin_top(0);

        let provider = gtk4::CssProvider::new();
        cpu_freq_lbl
            .style_context()
            .add_provider(&provider, gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION);

        let update_color = |ghz: f64, prov: &gtk4::CssProvider| {
            let (r, g, b) = if ghz < 2.0 {
                (46, 194, 126) // Green
            } else if ghz < 3.5 {
                (255, 152, 0) // Orange
            } else {
                (244, 67, 54) // Red
            };
            let css = format!(
                ".cpu-badge {{ background-color: rgba({}, {}, {}, 0.18); border: 1px solid rgba({}, {}, {}, 0.35); color: rgb({}, {}, {}); border-radius: 9999px; padding: 3px 10px; font-size: 11px; font-weight: 600; }}",
                r, g, b, r, g, b, r, g, b
            );
            prov.load_from_data(&css);
        };
        update_color(ghz, &provider);

        let cpu_freq_lbl_clone = cpu_freq_lbl.clone();
        let provider_clone = provider.clone();
        gtk4::glib::timeout_add_local(std::time::Duration::from_secs(5), move || {
            if cpu_freq_lbl_clone.parent().is_none() {
                return gtk4::glib::ControlFlow::Break;
            }
            if let Some((new_ghz, new_freq)) = super::battery_card::get_cpu_frequency() {
                cpu_freq_lbl_clone.set_text(&new_freq);

                let (r, g, b) = if new_ghz < 2.0 {
                    (46, 194, 126)
                } else if new_ghz < 3.5 {
                    (255, 152, 0)
                } else {
                    (244, 67, 54)
                };
                let css = format!(
                    ".cpu-badge {{ background-color: rgba({}, {}, {}, 0.18); border: 1px solid rgba({}, {}, {}, 0.35); color: rgb({}, {}, {}); border-radius: 9999px; padding: 3px 10px; font-size: 11px; font-weight: 600; }}",
                    r, g, b, r, g, b, r, g, b
                );
                provider_clone.load_from_data(&css);
            }
            gtk4::glib::ControlFlow::Continue
        });

        perf_header.append(&cpu_freq_lbl);
    }

    let perf_desc = Label::new(Some(&babydra_core::i18n::t("settings.power_perf_desc")));
    perf_desc.add_css_class("settings-row-desc");
    perf_desc.set_halign(gtk4::Align::Start);

    perf_section.append(&perf_header);
    perf_section.append(&perf_desc);

    let options_box = Box::new(Orientation::Horizontal, 8);
    options_box.set_homogeneous(true);
    options_box.set_vexpand(false);
    options_box.set_margin_top(4);

    let (profile_balanced_btn, profile_balanced_title, profile_balanced_desc) =
        create_profile_button(
            "settings.power_balanced",
            "settings.power_balanced_desc",
            "sliders",
        );
    let (profile_normal_btn, profile_normal_title, profile_normal_desc) = create_profile_button(
        "settings.power_saver",
        "settings.power_saver_desc",
        "history",
    );
    let (profile_high_btn, profile_high_title, profile_high_desc) =
        create_profile_button("settings.power_high", "settings.power_high_desc", "cog");

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
        title_lbl,
        desc_lbl,
        saver_title,
        saver_desc,
        perf_title,
        perf_desc,
        profile_balanced_btn,
        profile_balanced_title,
        profile_balanced_desc,
        profile_normal_btn,
        profile_normal_title,
        profile_normal_desc,
        profile_high_btn,
        profile_high_title,
        profile_high_desc,
        threshold_slider,
        charge_title,
        charge_desc,
        charge_slider,
    };

    (widget, auth_dialog)
}
