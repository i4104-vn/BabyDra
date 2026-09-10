//! Microphone / Audio Input UI builder.

use babydra_core::i18n::trans;
use babydra_core::models::volume::AudioDevice;
use babydra_core::services::system::volume::{
    get_audio_devices, get_current_microphone_volume, is_microphone_muted,
};
use babydra_ui_kit::components::cards::{create_collapsible_card, create_slider_header};
use babydra_ui_kit::components::{create_list_row, CustomSwitch, PillSlider};
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, DropDown, Label, Orientation, StringList};

/// Holds widget references for Microphone / Audio Input card.
pub struct InputWidgets {
    pub container: GtkBox,
    pub dropdown: DropDown,
    pub slider: PillSlider,
    #[allow(dead_code)]
    pub val_label: Label,
    pub mute_btn: Button,
    pub mute_switch: CustomSwitch,
}

/// Renders the Microphone collapsible card and returns widgets + audio sources list.
pub fn render_input_card() -> (InputWidgets, Vec<AudioDevice>) {
    let card = create_collapsible_card(
        &trans("settings.general_audio_input"),
        Some(&trans("settings.general_audio_input_desc")),
        Some("microphone"),
        false,
    );

    // 1. Input Device Selection
    let devices = get_audio_devices(true);
    let dev_names: Vec<&str> = if devices.is_empty() {
        vec!["Default Microphone"]
    } else {
        devices.iter().map(|d| d.description.as_str()).collect()
    };
    let dropdown = DropDown::new(
        Some(StringList::new(&dev_names)),
        Option::<gtk4::Expression>::None,
    );
    dropdown.set_valign(gtk4::Align::Center);
    if let Some(pos) = devices.iter().position(|d| d.is_default) {
        dropdown.set_selected(pos as u32);
    }
    let dev_row = create_list_row("", &trans("settings.general_input_device"), "", Some(&dropdown));
    card.content.append(&dev_row);

    // 2. Microphone Level Slider (Panel control-slider style)
    let cur_vol = get_current_microphone_volume();
    let vol_box = GtkBox::new(Orientation::Vertical, 6);
    vol_box.set_margin_start(8);
    vol_box.set_margin_end(8);
    vol_box.set_margin_top(4);
    vol_box.set_margin_bottom(4);

    let (vol_hdr, val_label) = create_slider_header(&trans("settings.general_input_volume"), cur_vol);
    vol_box.append(&vol_hdr);

    let row_box = GtkBox::new(Orientation::Horizontal, 8);

    let icon_container = GtkBox::new(Orientation::Horizontal, 0);
    icon_container.set_valign(gtk4::Align::Center);

    let icon_widget = babydra_ui_kit::ui::icon::get_icon_colored("microphone", 16, "#ffffff");
    icon_widget.add_css_class("slider-icon");
    icon_container.append(&icon_widget);

    let mute_btn = Button::new();
    mute_btn.add_css_class("slider-overlay-mute-btn");
    mute_btn.set_child(Some(&icon_container));
    mute_btn.set_halign(gtk4::Align::Start);
    mute_btn.set_valign(gtk4::Align::Center);
    mute_btn.set_margin_start(10);
    mute_btn.set_can_focus(false);
    mute_btn.set_focus_on_click(false);

    let slider = PillSlider::new(cur_vol, |_| {});
    slider.bind_label(&val_label);
    slider.add_scroll_to(&row_box);
    slider.add_scroll_to(&vol_box);

    let overlay = gtk4::Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_valign(gtk4::Align::Center);
    overlay.set_child(Some(&slider.container));
    overlay.add_overlay(&mute_btn);

    row_box.append(&overlay);
    vol_box.append(&row_box);
    card.content.append(&vol_box);

    // 3. Microphone Mute Switch
    let mute_switch = CustomSwitch::new(is_microphone_muted());
    mute_switch.container.set_valign(gtk4::Align::Center);
    let mute_row = create_list_row(
        "",
        &trans("settings.general_input_mute"),
        "",
        Some(&mute_switch.container),
    );
    card.content.append(&mute_row);

    let widgets = InputWidgets {
        container: card.container,
        dropdown,
        slider,
        val_label,
        mute_btn,
        mute_switch,
    };

    (widgets, devices)
}
