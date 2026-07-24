use gtk4::prelude::*;
use std::rc::Rc;
use babydra_common::{PerformanceProfile, get_current_profile, set_performance_profile};

fn populate_performance_popover(
    popover: &gtk4::Popover,
    perf_btn: &gtk4::Button,
) {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    container.add_css_class("audio-menu-popover");
    container.set_size_request(220, -1);

    let title_label = gtk4::Label::new(Some("Performance Profile"));
    title_label.add_css_class("audio-menu-section-title");
    title_label.set_xalign(0.0);
    container.append(&title_label);

    let profiles = [
        (PerformanceProfile::Balanced, "Balanced"),
        (PerformanceProfile::Normal, "Normal"),
        (PerformanceProfile::HighPerformance, "High Performance"),
    ];

    let current = get_current_profile();

    for (prof, label) in profiles {
        let is_active = prof == current;
        let btn = gtk4::Button::new();
        btn.add_css_class("audio-menu-item-btn");
        if is_active {
            btn.add_css_class("active");
        }

        let btn_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        let icon = babydra_utils::ui::icon::get_icon_colored(
            "performance",
            14,
            if is_active { "#ffffff" } else { "rgba(255, 255, 255, 0.5)" },
        );
        let name_label = gtk4::Label::new(Some(label));
        name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        name_label.set_hexpand(true);
        name_label.set_halign(gtk4::Align::Start);

        btn_box.append(&icon);
        btn_box.append(&name_label);

        if is_active {
            let check_label = gtk4::Label::new(Some("✓"));
            check_label.add_css_class("audio-menu-item-check");
            btn_box.append(&check_label);
        }

        btn.set_child(Some(&btn_box));

        let pop_clone = popover.clone();
        let perf_btn_clone = perf_btn.clone();
        btn.connect_clicked(move |_| {
            set_performance_profile(prof);
            perf_btn_clone.set_tooltip_text(Some(&format!("Performance Profile: {}", prof.label())));
            pop_clone.popdown();
        });

        container.append(&btn);
    }

    popover.set_child(Some(&container));
}

pub fn create_header_row(
    on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>,
) -> gtk4::Box {
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    header_box.set_hexpand(true);

    let title = gtk4::Label::new(Some(&babydra_common::i18n::t("control.title")));
    title.add_css_class("control-center-title");
    title.set_xalign(0.0);
    title.set_hexpand(true);

    let btn_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    btn_box.set_halign(gtk4::Align::End);

    let is_dark = babydra_utils::ui::theme::is_dark_mode();
    let theme_icon_name = if is_dark { "dark-mode" } else { "brightness" };
    let icon_color = if is_dark { "#ffffff" } else { "rgba(255,255,255,0.8)" };
    let theme_tooltip = babydra_common::i18n::t("control.dark_mode");

    // Performance profile button & popover
    let cur_prof = get_current_profile();
    let perf_tooltip = format!("Performance Profile: {}", cur_prof.label());
    let perf_btn = babydra_utils::components::create_colored_icon_button(
        "performance",
        16,
        icon_color,
        &["circle-btn"],
        Some(&perf_tooltip),
        || {},
    );

    let perf_popover = babydra_utils::components::create_popover(&perf_btn, gtk4::PositionType::Bottom, "taskbar-popover");
    perf_popover.set_has_arrow(true);

    let perf_popover_clone = perf_popover.clone();
    let on_popover_toggled_c1 = on_popover_toggled.clone();
    let perf_btn_c1 = perf_btn.clone();

    perf_btn.connect_clicked(move |_| {
        populate_performance_popover(&perf_popover_clone, &perf_btn_c1);
        if let Some(ref cb) = on_popover_toggled_c1 {
            cb(true);
        }
        perf_popover_clone.popup();
    });

    let on_popover_toggled_c2 = on_popover_toggled.clone();
    perf_popover.connect_closed(move |_| {
        if let Some(ref cb) = on_popover_toggled_c2 {
            cb(false);
        }
    });

    // Theme toggle button
    let theme_btn = babydra_utils::components::create_colored_icon_button(
        theme_icon_name,
        16,
        icon_color,
        &["circle-btn"],
        Some(&theme_tooltip),
        || {
            babydra_utils::ui::theme::set_dark_mode(!babydra_utils::ui::theme::is_dark_mode());
        },
    );

    // Auto-update icons when theme changes
    if let Some(settings) = gtk4::Settings::default() {
        let btn_clone = theme_btn.clone();
        let perf_btn_clone = perf_btn.clone();
        settings.connect_gtk_application_prefer_dark_theme_notify(move |_| {
            let dark = babydra_utils::ui::theme::is_dark_mode();
            let name = if dark { "dark-mode" } else { "brightness" };
            let color = if dark { "#ffffff" } else { "rgba(255,255,255,0.8)" };
            let new_theme_icon = babydra_utils::ui::icon::get_icon_colored(name, 16, color);
            btn_clone.set_child(Some(&new_theme_icon));

            let new_perf_icon = babydra_utils::ui::icon::get_icon_colored("performance", 16, color);
            perf_btn_clone.set_child(Some(&new_perf_icon));
        });
    }

    let settings_btn = babydra_utils::components::create_icon_button(
        "settings",
        16,
        &["circle-btn"],
        Some(&babydra_common::i18n::t("control.settings")),
        || {},
    );

    btn_box.append(&perf_btn);
    btn_box.append(&theme_btn);
    btn_box.append(&settings_btn);

    header_box.append(&title);
    header_box.append(&btn_box);

    header_box
}
