//! Top bar UI construction: clock, date, and power action buttons.

use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Label, Orientation};

/// Builds the top bar with clock/date display and suspend/reboot/power buttons.
pub fn build() -> super::TopBarWidget {
    tracing::info!(target: "babydra-greeter", "Building TopBar Widget (clock, date, power buttons)");
    let top_bar = GtkBox::new(Orientation::Horizontal, 0);
    top_bar.add_css_class("top-bar");
    top_bar.set_valign(Align::Start);

    let time_box = GtkBox::new(Orientation::Vertical, 0);
    time_box.set_halign(Align::Start);

    let clock_label = Label::new(None);
    clock_label.add_css_class("clock-time");

    let date_label = Label::new(None);
    date_label.add_css_class("clock-date");

    time_box.append(&clock_label);
    time_box.append(&date_label);

    let actions_box = GtkBox::new(Orientation::Horizontal, 10);
    actions_box.set_halign(Align::End);
    actions_box.set_hexpand(true);
    actions_box.set_valign(Align::Center);

    tracing::info!(target: "babydra-greeter", "Asset loaded: rendering top bar icons (sleep, restart, power)");
    let suspend_btn = Button::new();
    suspend_btn.add_css_class("power-btn");
    suspend_btn.add_css_class("action-btn-suspend");
    suspend_btn.set_tooltip_text(Some(&babydra_core::i18n::t("greeter.suspend")));
    suspend_btn.set_cursor_from_name(Some("pointer"));
    let suspend_icon = babydra_ui_kit::ui::icon::get_icon_colored("sleep", 18, "#ffffff");
    suspend_btn.set_child(Some(&suspend_icon));

    let reboot_btn = Button::new();
    reboot_btn.add_css_class("power-btn");
    reboot_btn.add_css_class("action-btn-reboot");
    reboot_btn.set_tooltip_text(Some(&babydra_core::i18n::t("greeter.reboot")));
    reboot_btn.set_cursor_from_name(Some("pointer"));
    let reboot_icon = babydra_ui_kit::ui::icon::get_icon_colored("restart", 18, "#ffffff");
    reboot_btn.set_child(Some(&reboot_icon));

    let power_btn = Button::new();
    power_btn.add_css_class("power-btn");
    power_btn.set_tooltip_text(Some(&babydra_core::i18n::t("greeter.power_off")));
    power_btn.set_cursor_from_name(Some("pointer"));
    let power_icon = babydra_ui_kit::ui::icon::get_icon_colored("power", 18, "#ffffff");
    power_btn.set_child(Some(&power_icon));

    actions_box.append(&suspend_btn);
    actions_box.append(&reboot_btn);
    actions_box.append(&power_btn);

    top_bar.append(&time_box);
    top_bar.append(&actions_box);

    super::TopBarWidget {
        container: top_bar,
        power_btn,
        reboot_btn,
        suspend_btn,
        clock_label,
        date_label,
    }
}
