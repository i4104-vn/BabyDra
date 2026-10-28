//! Generic Settings (Cài đặt chung) widget assembling modular cards.

pub mod clipboard;
pub mod datetime;
pub mod default_apps;
pub mod device_account;
pub mod input;
pub mod output;
pub mod recovery;
pub mod sound_effects;
pub mod system_update;

use std::rc::Rc;
use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Button, Label, Orientation, Overlay, ScrolledWindow, Widget};

fn section_container(cards: Vec<GtkBox>) -> GtkBox {
    let section = GtkBox::new(Orientation::Vertical, 10);
    section.set_hexpand(true);

    for card in cards {
        card.set_hexpand(true);
        section.append(&card);
    }

    section
}

/// Creates the Generic Settings (Cài đặt chung) widget page.
pub fn create_general_widget() -> Widget {
    let overlay = Overlay::new();
    overlay.set_vexpand(true);
    overlay.set_hexpand(true);

    let scrolled = ScrolledWindow::new();
    scrolled.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scrolled.set_vexpand(true);
    scrolled.set_hexpand(true);

    let container = GtkBox::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_hexpand(true);

    // Page header matching standard settings layout
    let header_box = GtkBox::new(Orientation::Vertical, 4);
    header_box.set_margin_bottom(6);

    let title_lbl = Label::new(Some(&trans("settings.general_title")));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(Align::Start);
    title_lbl.set_hexpand(true);
    header_box.append(&title_lbl);

    let subtitle_lbl = Label::new(Some(&trans("settings.general_subtitle")));
    subtitle_lbl.add_css_class("settings-page-subtitle");
    subtitle_lbl.set_halign(Align::Start);
    subtitle_lbl.set_xalign(0.0);
    subtitle_lbl.set_wrap(true);
    header_box.append(&subtitle_lbl);

    container.append(&header_box);

    // Category Tabs Bar (without 'All', cleanly divided into 4 focused categories)
    let tabs_box = GtkBox::new(Orientation::Horizontal, 4);
    tabs_box.add_css_class("settings-tab-bar");
    tabs_box.set_halign(Align::Start);
    tabs_box.set_margin_bottom(8);

    let tab_devices = Button::with_label(&trans("settings.general_category_devices"));
    let tab_audio = Button::with_label(&trans("settings.general_category_audio"));
    let tab_datetime = Button::with_label(&trans("settings.general_datetime"));
    let tab_maintenance = Button::with_label(&trans("settings.general_category_maintenance"));

    let tabs = [
        tab_devices.clone(),
        tab_audio.clone(),
        tab_datetime.clone(),
        tab_maintenance.clone(),
    ];
    for (i, btn) in tabs.iter().enumerate() {
        btn.add_css_class("app-tab-btn");
        btn.set_cursor_from_name(Some("pointer"));
        if i == 0 {
            btn.add_css_class("active");
        }
        tabs_box.append(btn);
    }
    container.append(&tabs_box);

    // 1. Devices & Applications (Device Hostname, Password, Default Apps)
    let devices_section = section_container(vec![
        device_account::build_device_account_card(&overlay),
        default_apps::build_default_apps_card(),
    ]);

    // 2. Audio (Output playback, Microphone, System sound effects)
    let audio_section = section_container(vec![
        output::build_output_card(),
        input::build_input_card(),
        sound_effects::build_sound_effects_card(),
    ]);

    // 3. Date, Time & Clipboard (Date & Time, Timezone, Clipboard history)
    let datetime_section = section_container(vec![
        datetime::build_datetime_card(),
        clipboard::build_clipboard_card(),
    ]);

    // 4. System Maintenance & Recovery (System updates, Factory reset)
    let maintenance_section = section_container(vec![
        system_update::build_system_update_card(&overlay),
        recovery::build_recovery_card(&overlay),
    ]);

    // Set initial visibility (Tab 0 is active by default)
    devices_section.set_visible(true);
    audio_section.set_visible(false);
    datetime_section.set_visible(false);
    maintenance_section.set_visible(false);

    container.append(&devices_section);
    container.append(&audio_section);
    container.append(&datetime_section);
    container.append(&maintenance_section);

    // Wire Category Tabs filtering
    let tabs_rc = Rc::new(tabs);
    let dev_rc = Rc::new(devices_section);
    let aud_rc = Rc::new(audio_section);
    let date_rc = Rc::new(datetime_section);
    let maint_rc = Rc::new(maintenance_section);

    let switch_tab = {
        let tabs = tabs_rc.clone();
        let dev = dev_rc.clone();
        let aud = aud_rc.clone();
        let date = date_rc.clone();
        let maint = maint_rc.clone();
        Rc::new(move |selected_idx: usize| {
            for (i, btn) in tabs.iter().enumerate() {
                if i == selected_idx {
                    btn.add_css_class("active");
                } else {
                    btn.remove_css_class("active");
                }
            }
            dev.set_visible(selected_idx == 0);
            aud.set_visible(selected_idx == 1);
            date.set_visible(selected_idx == 2);
            maint.set_visible(selected_idx == 3);
        })
    };

    for (i, btn) in tabs_rc.iter().enumerate() {
        let sw = switch_tab.clone();
        btn.connect_clicked(move |_| {
            sw(i);
        });
    }

    scrolled.set_child(Some(&container));
    overlay.set_child(Some(&scrolled));
    overlay.into()
}
