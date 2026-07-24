use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use babydra_common::{PerformanceProfile, get_current_profile, set_performance_profile};

fn create_performance_popover(perf_btn: &gtk4::MenuButton) -> gtk4::Popover {
    let popover = gtk4::Popover::new();
    popover.add_css_class("performance-popover");
    popover.set_position(gtk4::PositionType::Bottom);

    let pop_box = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    pop_box.set_margin_top(8);
    pop_box.set_margin_bottom(8);
    pop_box.set_margin_start(8);
    pop_box.set_margin_end(8);

    let title_lbl = gtk4::Label::new(Some("Performance Profile"));
    title_lbl.add_css_class("popover-header-title");
    title_lbl.set_halign(gtk4::Align::Start);
    pop_box.append(&title_lbl);

    let profiles = [
        (PerformanceProfile::Balanced, "Balanced"),
        (PerformanceProfile::Normal, "Normal"),
        (PerformanceProfile::HighPerformance, "High Performance"),
    ];

    let current_profile = std::cell::Cell::new(get_current_profile());
    let option_buttons: Rc<RefCell<Vec<(PerformanceProfile, gtk4::Button, gtk4::Image)>>> = Rc::new(RefCell::new(Vec::new()));

    for (prof, label) in profiles {
        let btn = gtk4::Button::new();
        btn.add_css_class("profile-option-btn");
        
        let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
        row_box.set_hexpand(true);

        let icon = babydra_utils::ui::icon::get_icon("performance", 14);
        let lbl = gtk4::Label::new(Some(label));
        lbl.set_hexpand(true);
        lbl.set_halign(gtk4::Align::Start);

        let check_icon = babydra_utils::ui::icon::get_icon("check", 14);
        check_icon.set_visible(prof == current_profile.get());

        row_box.append(&icon);
        row_box.append(&lbl);
        row_box.append(&check_icon);
        btn.set_child(Some(&row_box));

        let option_buttons_clone = option_buttons.clone();
        let popover_clone = popover.clone();
        let perf_btn_clone = perf_btn.clone();
        btn.connect_clicked(move |_| {
            set_performance_profile(prof);
            perf_btn_clone.set_tooltip_text(Some(&format!("Performance Profile: {}", prof.label())));
            
            for (p, _b, check) in option_buttons_clone.borrow().iter() {
                check.set_visible(*p == prof);
            }
            popover_clone.popdown();
        });

        pop_box.append(&btn);
        option_buttons.borrow_mut().push((prof, btn, check_icon));
    }

    popover.set_child(Some(&pop_box));
    popover
}

pub fn create_header_row() -> gtk4::Box {
    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    header_box.set_hexpand(true);

    let title = gtk4::Label::new(Some(&babydra_common::i18n::t("control.title")));
    title.add_css_class("control-center-title");
    title.set_xalign(0.0);
    title.set_hexpand(true);

    let btn_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
    btn_box.set_halign(gtk4::Align::End);

    // Performance profile button & popover
    let perf_btn = gtk4::MenuButton::new();
    perf_btn.add_css_class("circle-btn");
    let perf_icon = babydra_utils::ui::icon::get_icon("performance", 16);
    perf_btn.set_child(Some(&perf_icon));
    let cur_prof = get_current_profile();
    perf_btn.set_tooltip_text(Some(&format!("Performance Profile: {}", cur_prof.label())));

    let perf_popover = create_performance_popover(&perf_btn);
    perf_btn.set_popover(Some(&perf_popover));

    // Theme toggle: uses colored icon, reacts to theme changes
    let is_dark = babydra_utils::ui::theme::is_dark_mode();
    let theme_icon_name = if is_dark { "dark-mode" } else { "brightness" };
    let theme_icon_color = if is_dark { "#ffffff" } else { "rgba(255,255,255,0.8)" };
    let theme_tooltip = babydra_common::i18n::t("control.dark_mode");
    let theme_btn = babydra_utils::components::create_colored_icon_button(
        theme_icon_name,
        16,
        theme_icon_color,
        &["circle-btn"],
        Some(&theme_tooltip),
        || {
            babydra_utils::ui::theme::set_dark_mode(!babydra_utils::ui::theme::is_dark_mode());
        },
    );

    // Auto-update icon when theme changes
    if let Some(settings) = gtk4::Settings::default() {
        let btn_clone = theme_btn.clone();
        settings.connect_gtk_application_prefer_dark_theme_notify(move |_| {
            let dark = babydra_utils::ui::theme::is_dark_mode();
            let name = if dark { "dark-mode" } else { "brightness" };
            let color = if dark { "#ffffff" } else { "rgba(255,255,255,0.8)" };
            let new_icon = babydra_utils::ui::icon::get_icon_colored(name, 16, color);
            btn_clone.set_child(Some(&new_icon));
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
