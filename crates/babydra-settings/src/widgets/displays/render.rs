use crate::widgets::state::{DisplayCardRow, DisplaysWidget};
use babydra_core::models::display::MonitorConfig;
use gtk4::prelude::*;
use gtk4::{Box, Button, DropDown, Label, Orientation, StringList};

/// Builds the header row containing title and action buttons (Refresh, Save)
fn build_header() -> (Box, Button, Button) {
    let header_box = Box::new(Orientation::Horizontal, 12);
    header_box.set_margin_bottom(4);

    let title_label = Label::new(Some(&babydra_core::i18n::trans("settings.displays_title")));
    title_label.add_css_class("settings-page-title");
    title_label.set_hexpand(true);
    title_label.set_halign(gtk4::Align::Start);

    let refresh_btn = Button::new();
    refresh_btn.add_css_class("icon-btn");
    refresh_btn.add_css_class("circular");
    refresh_btn.set_cursor_from_name(Some("pointer"));
    refresh_btn.set_valign(gtk4::Align::Center);
    refresh_btn.set_size_request(34, 34);
    let refresh_icon = babydra_ui_kit::ui::icon::get_icon("refresh", 16);
    refresh_icon.set_pixel_size(16);
    refresh_btn.set_child(Some(&refresh_icon));
    refresh_btn.set_tooltip_text(Some(&babydra_core::i18n::trans("settings.refresh")));

    let save_btn = Button::with_label(&babydra_core::i18n::trans("settings.save"));
    save_btn.add_css_class("suggested-action");
    save_btn.set_valign(gtk4::Align::Center);

    header_box.append(&title_label);
    header_box.append(&refresh_btn);
    header_box.append(&save_btn);

    (header_box, refresh_btn, save_btn)
}

/// Builds a configuration card for a single monitor in smart-card style
fn build_monitor_card(mon: &MonitorConfig) -> DisplayCardRow {
    let card = Box::new(Orientation::Vertical, 10);
    card.add_css_class("settings-card");
    card.add_css_class("device-card");
    card.add_css_class("display-grid-card");
    card.set_size_request(360, 240);
    card.set_halign(gtk4::Align::Start);
    card.set_valign(gtk4::Align::Start);

    // ── Header Row: Monitor Icon + Name/Desc on left, Active Badge on right ──
    let header_row = Box::new(Orientation::Horizontal, 10);
    header_row.set_valign(gtk4::Align::Center);

    let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    icon_badge.add_css_class("blue-icon-badge-sm");
    icon_badge.set_valign(gtk4::Align::Center);
    icon_badge.set_halign(gtk4::Align::Start);

    let mon_icon = babydra_ui_kit::ui::icon::get_icon("video-display", 18);
    mon_icon.set_pixel_size(18);
    mon_icon.set_valign(gtk4::Align::Center);
    mon_icon.set_halign(gtk4::Align::Center);
    mon_icon.set_vexpand(true);
    icon_badge.append(&mon_icon);
    header_row.append(&icon_badge);

    let title_box = Box::new(Orientation::Vertical, 2);
    title_box.set_valign(gtk4::Align::Center);
    title_box.set_halign(gtk4::Align::Start);
    title_box.set_hexpand(true);

    let name_lbl = Label::new(Some(&mon.name));
    name_lbl.add_css_class("settings-row-title");
    name_lbl.add_css_class("device-card-title");
    name_lbl.set_halign(gtk4::Align::Start);
    title_box.append(&name_lbl);

    let desc_text = if !mon.description.is_empty() && mon.description != mon.name {
        mon.description.clone()
    } else {
        format!("{}x{} @ {:.0}Hz", mon.resolution_width, mon.resolution_height, mon.refresh_rate)
    };
    let desc_lbl = Label::new(Some(&desc_text));
    desc_lbl.add_css_class("settings-row-desc");
    desc_lbl.add_css_class("device-card-subtitle");
    desc_lbl.set_halign(gtk4::Align::Start);
    title_box.append(&desc_lbl);

    header_row.append(&title_box);

    let active_badge = Label::new(Some("Connected"));
    active_badge.add_css_class("device-card-status-on");
    active_badge.set_valign(gtk4::Align::Center);
    header_row.append(&active_badge);

    card.append(&header_row);

    // Separator line
    let sep = gtk4::Separator::new(Orientation::Horizontal);
    sep.add_css_class("settings-card-separator");
    card.append(&sep);

    // ── Controls Body: Resolution, Rate, Orientation ──
    let body_box = Box::new(Orientation::Vertical, 8);
    body_box.set_valign(gtk4::Align::Fill);
    body_box.set_vexpand(true);

    // Resolution Row
    let res_row = Box::new(Orientation::Horizontal, 8);
    res_row.set_valign(gtk4::Align::Center);
    let res_lbl = Label::new(Some(&babydra_core::i18n::trans("settings.display_resolution")));
    res_lbl.add_css_class("display-prop-label");
    res_lbl.set_halign(gtk4::Align::Start);
    res_lbl.set_hexpand(true);
    res_row.append(&res_lbl);

    let res_strs: Vec<&str> = mon
        .available_resolutions
        .iter()
        .map(|s| s.as_str())
        .collect();
    let resolution_dropdown = DropDown::new(
        Some(StringList::new(&res_strs)),
        Option::<gtk4::Expression>::None,
    );
    resolution_dropdown.set_valign(gtk4::Align::Center);
    resolution_dropdown.set_halign(gtk4::Align::End);

    let cur_res_str = format!("{}x{}", mon.resolution_width, mon.resolution_height);
    if let Some(idx) = mon
        .available_resolutions
        .iter()
        .position(|r| r == &cur_res_str)
    {
        resolution_dropdown.set_selected(idx as u32);
    }
    res_row.append(&resolution_dropdown);
    body_box.append(&res_row);

    // Refresh Rate Row
    let rate_row = Box::new(Orientation::Horizontal, 8);
    rate_row.set_valign(gtk4::Align::Center);
    let rate_lbl = Label::new(Some(&babydra_core::i18n::trans("settings.display_refresh_rate")));
    rate_lbl.add_css_class("display-prop-label");
    rate_lbl.set_halign(gtk4::Align::Start);
    rate_lbl.set_hexpand(true);
    rate_row.append(&rate_lbl);

    let rate_strs: Vec<String> = mon
        .available_rates
        .iter()
        .map(|r| format!("{:.1} Hz", r))
        .collect();
    let rate_items: Vec<&str> = rate_strs.iter().map(|s| s.as_str()).collect();
    let rate_dropdown = DropDown::new(
        Some(StringList::new(&rate_items)),
        Option::<gtk4::Expression>::None,
    );
    rate_dropdown.set_valign(gtk4::Align::Center);
    rate_dropdown.set_halign(gtk4::Align::End);

    if let Some(idx) = mon
        .available_rates
        .iter()
        .position(|r| (r - mon.refresh_rate).abs() < 0.5)
    {
        rate_dropdown.set_selected(idx as u32);
    }
    rate_row.append(&rate_dropdown);
    body_box.append(&rate_row);

    // Orientation Row
    let orient_row = Box::new(Orientation::Horizontal, 8);
    orient_row.set_valign(gtk4::Align::Center);
    let orient_lbl = Label::new(Some(&babydra_core::i18n::trans("settings.display_orientation")));
    orient_lbl.add_css_class("display-prop-label");
    orient_lbl.set_halign(gtk4::Align::Start);
    orient_lbl.set_hexpand(true);
    orient_row.append(&orient_lbl);

    let orient_items_owned = [
        babydra_core::i18n::trans("settings.orientation_normal"),
        babydra_core::i18n::trans("settings.orientation_left"),
        babydra_core::i18n::trans("settings.orientation_inverted"),
        babydra_core::i18n::trans("settings.orientation_right"),
    ];
    let orient_items: Vec<&str> = orient_items_owned.iter().map(|s| s.as_str()).collect();
    let orientation_dropdown = DropDown::new(
        Some(StringList::new(&orient_items)),
        Option::<gtk4::Expression>::None,
    );
    orientation_dropdown.set_valign(gtk4::Align::Center);
    orientation_dropdown.set_halign(gtk4::Align::End);

    let orient_idx = match mon.orientation.as_str() {
        "left" | "90" => 1,
        "inverted" | "180" => 2,
        "right" | "270" => 3,
        _ => 0,
    };
    orientation_dropdown.set_selected(orient_idx);
    orient_row.append(&orientation_dropdown);
    body_box.append(&orient_row);

    card.append(&body_box);

    DisplayCardRow {
        container: card,
        resolution_dropdown,
        rate_dropdown,
        orientation_dropdown,
    }
}

/// Builds the main displays UI layout
pub fn build(monitors: &[MonitorConfig]) -> DisplaysWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    let (header_box, refresh_btn, save_btn) = build_header();
    container.append(&header_box);

    let flow_box = gtk4::FlowBox::new();
    flow_box.add_css_class("settings-card-flow");
    flow_box.set_selection_mode(gtk4::SelectionMode::None);
    flow_box.set_valign(gtk4::Align::Start);
    flow_box.set_halign(gtk4::Align::Start);
    flow_box.set_column_spacing(16);
    flow_box.set_row_spacing(16);
    flow_box.set_min_children_per_line(1);
    flow_box.set_max_children_per_line(10);

    let mut card_rows = Vec::with_capacity(monitors.len());
    for mon in monitors {
        let row = build_monitor_card(mon);
        flow_box.append(&row.container);
        card_rows.push(row);
    }

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&flow_box));

    container.append(&scroll);

    DisplaysWidget {
        container,
        save_btn,
        refresh_btn,
        card_rows,
    }
}
