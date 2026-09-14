//! Quality, framerate, format and audio settings card.

use super::state::ControlWindowState;
use babydra_ui_kit::prelude::*;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, DropDown, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

/// Builds the video and audio quality settings card.
pub fn build_quality_card(state: Rc<RefCell<ControlWindowState>>) -> GtkBox {
    let settings_card = create_card(Orientation::Vertical, 10);
    settings_card.set_margin_top(4);
    settings_card.set_margin_bottom(4);

    let settings_lbl =
        create_group_header(&babydra_core::i18n::trans("recorder.quality_header"));
    settings_card.append(&settings_lbl);

    // Resolution row
    let res_row = GtkBox::new(Orientation::Horizontal, 8);
    let res_lbl = Label::new(Some(&babydra_core::i18n::trans("recorder.resolution")));
    res_lbl.set_hexpand(true);
    res_lbl.set_halign(Align::Start);
    let native_str = babydra_core::i18n::trans("recorder.resolution_native");
    let res_dropdown = DropDown::from_strings(&[
        &native_str,
        "1080p (1920x1080)",
        "720p (1280x720)",
        "480p (854x480)",
    ]);
    {
        let state_c = state.clone();
        res_dropdown.connect_selected_notify(move |dd| {
            state_c.borrow_mut().resolution_idx = dd.selected() as usize;
        });
    }
    res_row.append(&res_lbl);
    res_row.append(&res_dropdown);
    settings_card.append(&res_row);

    // Framerate row
    let fps_row = GtkBox::new(Orientation::Horizontal, 8);
    let fps_lbl = Label::new(Some(&babydra_core::i18n::trans("recorder.framerate")));
    fps_lbl.set_hexpand(true);
    fps_lbl.set_halign(Align::Start);
    let fps_dropdown = DropDown::from_strings(&["60 FPS", "30 FPS", "24 FPS"]);
    {
        let state_c = state.clone();
        fps_dropdown.connect_selected_notify(move |dd| {
            let fps = match dd.selected() {
                1 => 30,
                2 => 24,
                _ => 60,
            };
            state_c.borrow_mut().framerate = fps;
        });
    }
    fps_row.append(&fps_lbl);
    fps_row.append(&fps_dropdown);
    settings_card.append(&fps_row);

    // Format row
    let fmt_row = GtkBox::new(Orientation::Horizontal, 8);
    let fmt_lbl = Label::new(Some(&babydra_core::i18n::trans("recorder.format")));
    fmt_lbl.set_hexpand(true);
    fmt_lbl.set_halign(Align::Start);
    let fmt_dropdown = DropDown::from_strings(&["MP4 (H.264)", "MKV (H.264)", "WebM (VP9)"]);
    {
        let state_c = state.clone();
        fmt_dropdown.connect_selected_notify(move |dd| {
            let fmt = match dd.selected() {
                1 => "mkv",
                2 => "webm",
                _ => "mp4",
            };
            state_c.borrow_mut().format = fmt.to_string();
        });
    }
    fmt_row.append(&fmt_lbl);
    fmt_row.append(&fmt_dropdown);
    settings_card.append(&fmt_row);

    // Audio switch row
    let audio_row = GtkBox::new(Orientation::Horizontal, 8);
    let audio_lbl = Label::new(Some(&babydra_core::i18n::trans("recorder.audio_toggle")));
    audio_lbl.set_hexpand(true);
    audio_lbl.set_halign(Align::Start);
    let state_audio = state.clone();
    let audio_switch = create_switch(false, move |active| {
        state_audio.borrow_mut().audio = active;
    });
    audio_row.append(&audio_lbl);
    audio_row.append(&audio_switch.container);
    settings_card.append(&audio_row);

    settings_card
}
