//! About BabyDra UI layout generator.

use babydra_ui_kit::components::cards::create_card;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Label, Orientation, ScrolledWindow};

#[derive(Clone)]
pub struct AboutLabels {
    pub os_label: Label,
    pub sub_label: Label,
    pub uptime_lbl: Label,
    pub cpu_lbl: Label,
    pub mem_lbl: Label,
    pub gpu_lbl: Label,
    pub arch_lbl: Label,
}

pub struct AboutWidgets {
    pub root: GtkBox,
    pub labels: AboutLabels,
}

/// Builds the About BabyDra page UI.
pub fn build_about_ui() -> AboutWidgets {
    let main_box = GtkBox::new(Orientation::Vertical, 16);
    main_box.set_vexpand(true);
    main_box.set_valign(Align::Fill);

    // Header Row
    let header_box = GtkBox::new(Orientation::Horizontal, 12);
    header_box.set_margin_bottom(4);

    let page_title = Label::new(Some(&babydra_core::i18n::trans("settings.about_title")));
    page_title.add_css_class("settings-page-title");
    page_title.set_halign(Align::Start);
    page_title.set_hexpand(true);
    header_box.append(&page_title);
    main_box.append(&header_box);

    let content_box = GtkBox::new(Orientation::Vertical, 20);
    content_box.set_valign(Align::Start);

    // ── Hero Card: BabyDra Logo + Version + OS Info ──
    let hero_card = create_card(Orientation::Horizontal, 24);
    hero_card.add_css_class("glass-panel");
    hero_card.set_valign(Align::Start);
    hero_card.set_margin_start(4);
    hero_card.set_margin_end(4);

    // Logo
    let logo_box = GtkBox::new(Orientation::Vertical, 0);
    logo_box.set_valign(Align::Center);
    logo_box.set_size_request(100, 100);

    let logo_img = babydra_ui_kit::ui::icon::get_icon("logo", 100);
    logo_img.set_pixel_size(100);
    logo_img.set_valign(Align::Center);
    logo_img.set_halign(Align::Center);
    logo_box.append(&logo_img);
    hero_card.append(&logo_box);

    // Version & Build Info
    let version_box = GtkBox::new(Orientation::Vertical, 8);
    version_box.set_valign(Align::Center);
    version_box.set_hexpand(true);

    let version_title = Label::new(Some("BabyDra Linux"));
    version_title.add_css_class("hero-hostname");
    version_title.set_halign(Align::Start);
    version_box.append(&version_title);

    let version_info = Label::new(Some(&format!(
        "BabyDra v{} • Arch Linux",
        env!("CARGO_PKG_VERSION"),
    )));
    version_info.add_css_class("hero-subtitle");
    version_info.set_halign(Align::Start);
    version_box.append(&version_info);

    // OS info placeholders (will be updated async)
    let os_label = Label::new(Some("Loading..."));
    os_label.add_css_class("hero-subtitle");
    os_label.set_halign(Align::Start);
    version_box.append(&os_label);

    let sub_label = Label::new(Some("Loading..."));
    sub_label.add_css_class("settings-row-desc");
    sub_label.set_halign(Align::Start);
    version_box.append(&sub_label);

    // Uptime Badge
    let uptime_box = GtkBox::new(Orientation::Horizontal, 8);
    uptime_box.add_css_class("hero-uptime-badge");
    uptime_box.set_valign(Align::Start);
    uptime_box.set_halign(Align::End);
    uptime_box.set_margin_top(4);

    let clock_icon = babydra_ui_kit::ui::icon::get_icon("history", 16);
    clock_icon.set_pixel_size(16);
    clock_icon.set_valign(Align::Center);
    uptime_box.append(&clock_icon);

    let uptime_lbl = Label::new(Some(
        &babydra_core::i18n::trans("settings.up_time").replace("{}", "..."),
    ));
    uptime_lbl.add_css_class("hero-uptime-label");
    uptime_lbl.set_valign(Align::Center);
    uptime_box.append(&uptime_lbl);

    hero_card.append(&version_box);
    hero_card.append(&uptime_box);

    content_box.append(&hero_card);

    // ── Section: Donation ──
    let donate_label = Label::new(Some(&babydra_core::i18n::trans(
        "settings.about_donate_title",
    )));
    donate_label.add_css_class("settings-row-desc");
    donate_label.set_halign(Align::Start);
    donate_label.set_margin_start(8);
    donate_label.set_margin_top(8);
    donate_label.set_margin_bottom(4);
    content_box.append(&donate_label);

    let donate_card = create_card(Orientation::Horizontal, 24);
    donate_card.add_css_class("glass-panel");
    donate_card.add_css_class("about-donate-card");
    donate_card.set_valign(Align::Start);
    donate_card.set_margin_start(4);
    donate_card.set_margin_end(4);

    let qr_box = GtkBox::new(Orientation::Vertical, 0);
    qr_box.set_valign(Align::Center);
    qr_box.set_halign(Align::Center);
    qr_box.add_css_class("qr-code-box");

    let qr_size = 140;
    let qr_img = babydra_ui_kit::ui::icon::get_icon_from_svg(include_str!("qr.svg"), qr_size);
    qr_img.set_pixel_size(qr_size);
    qr_img.set_valign(Align::Center);
    qr_img.set_halign(Align::Center);
    qr_img.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.about_qr_alt")));
    qr_box.append(&qr_img);
    donate_card.append(&qr_box);

    let donate_info = GtkBox::new(Orientation::Vertical, 8);
    donate_info.set_valign(Align::Center);
    donate_info.set_hexpand(true);

    let donate_title = Label::new(Some(&babydra_core::i18n::trans(
        "settings.about_donate_title",
    )));
    donate_title.add_css_class("hero-hostname");
    donate_title.set_halign(Align::Start);
    donate_info.append(&donate_title);

    let donate_desc = Label::new(Some(&babydra_core::i18n::trans(
        "settings.about_donate_desc",
    )));
    donate_desc.add_css_class("hero-subtitle");
    donate_desc.set_halign(Align::Start);
    donate_info.append(&donate_desc);

    donate_card.append(&donate_info);
    content_box.append(&donate_card);

    // ── Section: Hardware Specifications ──
    let hw_label = Label::new(Some(&babydra_core::i18n::trans("settings.about_hardware")));
    hw_label.add_css_class("settings-row-desc");
    hw_label.set_halign(Align::Start);
    hw_label.set_margin_start(8);
    hw_label.set_margin_top(8);
    hw_label.set_margin_bottom(4);
    content_box.append(&hw_label);

    let grid = gtk4::Grid::new();
    grid.set_column_spacing(20);
    grid.set_row_spacing(20);
    grid.set_column_homogeneous(true);
    grid.set_valign(Align::Start);
    grid.set_margin_start(4);
    grid.set_margin_end(4);

    let cpu_lbl = Label::new(Some("Loading..."));
    let mem_lbl = Label::new(Some("Loading..."));
    let gpu_lbl = Label::new(Some("Loading..."));
    let arch_lbl = Label::new(Some("Loading..."));

    let specs: Vec<(&str, String, &Label)> = vec![
        (
            "sliders",
            babydra_core::i18n::trans("settings.about_cpu"),
            &cpu_lbl,
        ),
        (
            "history",
            babydra_core::i18n::trans("settings.about_memory"),
            &mem_lbl,
        ),
        (
            "palette",
            babydra_core::i18n::trans("settings.about_gpu"),
            &gpu_lbl,
        ),
        (
            "cog",
            babydra_core::i18n::trans("settings.about_arch"),
            &arch_lbl,
        ),
    ];

    for (idx, (icon_name, label, value_widget)) in specs.into_iter().enumerate() {
        let card = GtkBox::new(Orientation::Vertical, 12);
        card.add_css_class("spec-card");
        card.set_halign(Align::Fill);
        card.set_valign(Align::Fill);

        // Centered Blue Icon Badge
        let icon_badge = GtkBox::new(Orientation::Vertical, 0);
        icon_badge.add_css_class("blue-icon-badge");
        icon_badge.set_halign(Align::Center);
        icon_badge.set_valign(Align::Center);
        icon_badge.set_size_request(44, 44);

        let icon_img = babydra_ui_kit::ui::icon::get_icon(icon_name, 22);
        icon_img.set_pixel_size(22);
        icon_img.set_vexpand(true);
        icon_img.set_hexpand(true);
        icon_img.set_valign(Align::Center);
        icon_img.set_halign(Align::Center);
        icon_badge.append(&icon_img);
        card.append(&icon_badge);

        // Label
        let label_widget = Label::new(Some(label.as_str()));
        label_widget.add_css_class("spec-label");
        label_widget.set_halign(Align::Center);
        card.append(&label_widget);

        // Value
        value_widget.add_css_class("spec-value");
        value_widget.set_halign(Align::Center);
        value_widget.set_justify(gtk4::Justification::Center);
        value_widget.set_wrap(true);
        card.append(value_widget);

        let col = (idx % 2) as i32;
        let row = (idx / 2) as i32;
        grid.attach(&card, col, row, 1, 1);
    }

    content_box.append(&grid);

    let scroll = ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(Align::Fill);
    scroll.set_child(Some(&content_box));

    main_box.append(&scroll);

    let labels = AboutLabels {
        os_label,
        sub_label,
        uptime_lbl,
        cpu_lbl,
        mem_lbl,
        gpu_lbl,
        arch_lbl,
    };

    AboutWidgets {
        root: main_box,
        labels,
    }
}
