//! System Sound Effects UI builder.

use babydra_core::i18n::trans;
use babydra_core::services::system::default_apps::{
    get_event_sounds_enabled, get_input_feedback_sounds_enabled,
};
use babydra_ui_kit::components::cards::create_collapsible_card;
use babydra_ui_kit::components::{create_list_row, CustomSwitch};
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Label, Orientation};

/// Holds widget references for Sound Effects card.
pub struct SoundEffectsWidgets {
    pub container: GtkBox,
    pub event_sounds_switch: CustomSwitch,
    pub feedback_sounds_switch: CustomSwitch,
    pub test_sound_btn: Button,
}

/// Renders the System Sound Effects collapsible card.
pub fn render_sound_effects_card() -> SoundEffectsWidgets {
    let card = create_collapsible_card(
        &trans("settings.general_sound_effects"),
        Some(&trans("settings.general_sound_effects_desc")),
        Some("bell"),
        false,
    );

    // 1. Event Sounds Switch
    let event_sounds_switch = CustomSwitch::new(get_event_sounds_enabled());
    event_sounds_switch.container.set_valign(Align::Center);
    let ev_row = create_list_row(
        "",
        &trans("settings.general_event_sounds"),
        &trans("settings.general_event_sounds_desc"),
        Some(&event_sounds_switch.container),
    );
    card.content.append(&ev_row);

    // 2. Input Feedback Sounds Switch
    let feedback_sounds_switch = CustomSwitch::new(get_input_feedback_sounds_enabled());
    feedback_sounds_switch.container.set_valign(Align::Center);
    let fb_row = create_list_row(
        "",
        &trans("settings.general_feedback_sounds"),
        &trans("settings.general_feedback_sounds_desc"),
        Some(&feedback_sounds_switch.container),
    );
    card.content.append(&fb_row);

    // 3. Test Alert Sound Button
    let test_sound_btn = Button::new();
    test_sound_btn.add_css_class("connect-pill-btn");
    test_sound_btn.set_cursor_from_name(Some("pointer"));
    test_sound_btn.set_valign(Align::Center);
    let btn_box = GtkBox::new(Orientation::Horizontal, 6);
    btn_box.set_valign(Align::Center);
    let icon = babydra_ui_kit::ui::icon::get_icon("volume", 16);
    icon.set_pixel_size(16);
    let lbl = Label::new(Some(&trans("settings.general_test_sound")));
    btn_box.append(&icon);
    btn_box.append(&lbl);
    test_sound_btn.set_child(Some(&btn_box));

    let test_row = create_list_row(
        "",
        &trans("settings.general_test_sound"),
        &trans("settings.general_test_sound_desc"),
        Some(&test_sound_btn),
    );
    card.content.append(&test_row);

    SoundEffectsWidgets {
        container: card.container,
        event_sounds_switch,
        feedback_sounds_switch,
        test_sound_btn,
    }
}
