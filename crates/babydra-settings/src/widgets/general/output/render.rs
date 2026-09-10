//! Audio Output UI builder.

use babydra_core::i18n::trans;
use babydra_core::models::volume::AudioDevice;
use babydra_core::services::system::volume::{get_audio_devices, get_current_volume, is_muted};
use babydra_ui_kit::components::cards::{create_collapsible_card, create_slider_header};
use babydra_ui_kit::components::{create_list_row, CustomSwitch, PillSlider};
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Button, DropDown, Label, Orientation, StringList};

/// Holds widget references for Audio Output card event wiring.
pub struct OutputWidgets {
    pub container: GtkBox,
    pub dropdown: DropDown,
    pub slider: PillSlider,
    #[allow(dead_code)]
    pub val_label: Label,
    pub mute_btn: Button,
    pub mute_icon_box: GtkBox,
    pub mute_switch: CustomSwitch,
}

/// Renders the Audio Output collapsible card and returns widgets + audio devices list.
pub fn render_output_card() -> (OutputWidgets, Vec<AudioDevice>) {
    let card = create_collapsible_card(
        &trans("settings.general_audio_output"),
        Some(&trans("settings.general_audio_output_desc")),
        Some("volume"),
        true,
    );

    // 1. Playback Device Selection
    let devices = get_audio_devices(false);
    let dev_names: Vec<&str> = if devices.is_empty() {
        vec!["Default Audio Output"]
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
    let dev_row = create_list_row("", &trans("settings.general_output_device"), "", Some(&dropdown));
    card.content.append(&dev_row);

    // 2. Volume Slider (Panel control-slider style with embedded overlay mute button)
    let cur_vol = get_current_volume();
    let vol_box = GtkBox::new(Orientation::Vertical, 6);
    vol_box.set_margin_start(8);
    vol_box.set_margin_end(8);
    vol_box.set_margin_top(4);
    vol_box.set_margin_bottom(4);

    let (vol_hdr, val_label) = create_slider_header(&trans("settings.general_output_volume"), cur_vol);
    vol_box.append(&vol_hdr);

    let row_box = GtkBox::new(Orientation::Horizontal, 8);

    let mute_icon_box = GtkBox::new(Orientation::Horizontal, 0);
    mute_icon_box.set_valign(gtk4::Align::Center);

    let icon_name = if is_muted() { "volume-mute" } else { "volume" };
    let icon_widget = babydra_ui_kit::ui::icon::get_icon_colored(icon_name, 16, "#ffffff");
    icon_widget.add_css_class("slider-icon");
    mute_icon_box.append(&icon_widget);

    let mute_btn = Button::new();
    mute_btn.add_css_class("slider-overlay-mute-btn");
    mute_btn.set_child(Some(&mute_icon_box));
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

    // 3. Output Mute Switch
    let mute_switch = CustomSwitch::new(is_muted());
    mute_switch.container.set_valign(gtk4::Align::Center);
    let mute_row = create_list_row(
        "",
        &trans("settings.general_output_mute"),
        "",
        Some(&mute_switch.container),
    );
    card.content.append(&mute_row);

    let widgets = OutputWidgets {
        container: card.container,
        dropdown,
        slider,
        val_label,
        mute_btn,
        mute_icon_box,
        mute_switch,
    };

    (widgets, devices)
}
