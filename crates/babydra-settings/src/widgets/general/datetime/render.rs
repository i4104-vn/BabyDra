//! Date & Time Settings card UI builder.

use babydra_core::i18n::trans;
use babydra_ui_kit::components::cards::create_collapsible_card;
use babydra_ui_kit::components::{create_list_row, CustomSwitch};
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, DropDown, Label};

use super::controls::{self, ManualDateTimeControls};

/// Holds widget references for the Date & Time card.
pub struct DateTimeWidgets {
    pub container: GtkBox,
    pub auto_time_switch: CustomSwitch,
    pub auto_timezone_switch: CustomSwitch,
    pub time_format_combo: DropDown,
    pub manual: ManualDateTimeControls,
    pub time_label: Label,
    pub date_label: Label,
    pub timezone_label: Label,
}

/// Renders the Date & Time Settings collapsible card.
pub fn render_datetime_card() -> DateTimeWidgets {
    let card = create_collapsible_card(
        &trans("settings.general_datetime"),
        Some(&trans("settings.general_datetime_desc")),
        Some("history"),
        false,
    );

    let content = card.content.clone();

    // Auto Time Toggle
    let auto_time_switch = CustomSwitch::new(true);
    auto_time_switch.container.set_valign(Align::Center);
    let auto_time_row = create_list_row(
        "",
        &trans("settings.datetime_auto_time"),
        &trans("settings.datetime_auto_time_desc"),
        Some(&auto_time_switch.container),
    );
    content.append(&auto_time_row);

    // Auto Timezone Toggle
    let auto_timezone_switch = CustomSwitch::new(true);
    auto_timezone_switch.container.set_valign(Align::Center);
    let auto_timezone_row = create_list_row(
        "",
        &trans("settings.datetime_auto_timezone"),
        &trans("settings.datetime_auto_timezone_desc"),
        Some(&auto_timezone_switch.container),
    );
    content.append(&auto_timezone_row);

    // Time Format Selector (12h / 24h)
    let time_format_combo = DropDown::from_strings(&[
        &trans("settings.datetime_24h"),
        &trans("settings.datetime_12h"),
    ]);
    time_format_combo.set_selected(0);
    time_format_combo.set_valign(Align::Center);
    let time_format_row = create_list_row(
        "",
        &trans("settings.datetime_time_format"),
        &trans("settings.datetime_time_format_desc"),
        Some(&time_format_combo),
    );
    content.append(&time_format_row);

    let manual = controls::build();
    content.append(&manual.container);

    // Current Time Display
    let time_box = GtkBox::new(gtk4::Orientation::Horizontal, 12);
    time_box.set_margin_top(8);
    time_box.set_margin_bottom(8);
    time_box.set_margin_start(8);
    time_box.set_margin_end(8);

    let time_icon = babydra_ui_kit::ui::icon::get_icon("history", 20);
    time_icon.set_valign(Align::Center);
    time_box.append(&time_icon);

    let time_text_box = GtkBox::new(gtk4::Orientation::Vertical, 2);
    time_text_box.set_valign(Align::Center);
    let time_title = Label::new(Some(&trans("settings.datetime_current_time")));
    time_title.add_css_class("settings-label");
    time_title.set_halign(Align::Start);
    time_text_box.append(&time_title);

    let time_label = Label::new(Some("--:--"));
    time_label.add_css_class("settings-value");
    time_label.set_halign(Align::Start);
    time_text_box.append(&time_label);
    time_box.append(&time_text_box);

    let spacer = GtkBox::new(gtk4::Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    time_box.append(&spacer);
    content.append(&time_box);

    // Current Date Display
    let date_box = GtkBox::new(gtk4::Orientation::Horizontal, 12);
    date_box.set_margin_top(8);
    date_box.set_margin_bottom(8);
    date_box.set_margin_start(8);
    date_box.set_margin_end(8);

    let date_icon = babydra_ui_kit::ui::icon::get_icon("calendar", 20);
    date_icon.set_valign(Align::Center);
    date_box.append(&date_icon);

    let date_text_box = GtkBox::new(gtk4::Orientation::Vertical, 2);
    date_text_box.set_valign(Align::Center);
    let date_title = Label::new(Some(&trans("settings.datetime_current_date")));
    date_title.add_css_class("settings-label");
    date_title.set_halign(Align::Start);
    date_text_box.append(&date_title);

    let date_label = Label::new(Some("Loading..."));
    date_label.add_css_class("settings-value");
    date_label.set_halign(Align::Start);
    date_text_box.append(&date_label);
    date_box.append(&date_text_box);

    let spacer2 = GtkBox::new(gtk4::Orientation::Horizontal, 0);
    spacer2.set_hexpand(true);
    date_box.append(&spacer2);
    content.append(&date_box);

    // Timezone Display
    let tz_box = GtkBox::new(gtk4::Orientation::Horizontal, 12);
    tz_box.set_margin_top(8);
    tz_box.set_margin_bottom(8);
    tz_box.set_margin_start(8);
    tz_box.set_margin_end(8);

    let tz_icon = babydra_ui_kit::ui::icon::get_icon("map-pin", 20);
    tz_icon.set_valign(Align::Center);
    tz_box.append(&tz_icon);

    let tz_text_box = GtkBox::new(gtk4::Orientation::Vertical, 2);
    tz_text_box.set_valign(Align::Center);
    let tz_title = Label::new(Some(&trans("settings.datetime_timezone")));
    tz_title.add_css_class("settings-label");
    tz_title.set_halign(Align::Start);
    tz_text_box.append(&tz_title);

    let timezone_label = Label::new(Some("Loading..."));
    timezone_label.add_css_class("settings-value");
    timezone_label.set_halign(Align::Start);
    tz_text_box.append(&timezone_label);
    tz_box.append(&tz_text_box);

    let spacer3 = GtkBox::new(gtk4::Orientation::Horizontal, 0);
    spacer3.set_hexpand(true);
    tz_box.append(&spacer3);
    content.append(&tz_box);

    DateTimeWidgets {
        container: card.container,
        auto_time_switch,
        auto_timezone_switch,
        time_format_combo,
        manual,
        time_label,
        date_label,
        timezone_label,
    }
}
