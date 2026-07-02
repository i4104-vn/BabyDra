use gtk4::prelude::*;
use std::rc::Rc;
use std::cell::Cell;
use super::{is_muted, get_current_volume, set_volume, get_audio_devices, update_topbar_volume_icon};

pub fn create_volume_row(
    on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>,
    vol_icon: gtk4::Image,
) -> (gtk4::Box, gtk4::Scale) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    main_box.add_css_class("control-slider-card");

    let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    let title_label = gtk4::Label::new(Some("Volume"));
    title_label.add_css_class("control-slider-title");
    title_label.set_xalign(0.0);
    title_label.set_hexpand(true);

    let initial_val = get_current_volume();
    let value_label = gtk4::Label::new(Some(&format!("{:.0}%", initial_val)));
    value_label.add_css_class("control-slider-value");
    value_label.set_xalign(1.0);

    header_box.append(&title_label);
    header_box.append(&value_label);

    let row_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);

    let icon_container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    icon_container.set_valign(gtk4::Align::Center);

    let muted_state = Rc::new(Cell::new(is_muted()));
    let update_mute_icon = {
        let icon_container = icon_container.clone();
        Rc::new(move |is_muted_val: bool| {
            if let Some(old) = icon_container.first_child() {
                icon_container.remove(&old);
            }
            let icon_widget = if is_muted_val {
                babydra_common::icon::get_icon("volume-mute", 16)
            } else {
                babydra_common::icon::get_icon("volume", 16)
            };
            icon_widget.add_css_class("slider-icon");
            icon_container.append(&icon_widget);
        })
    };

    let mute_btn = gtk4::Button::new();
    mute_btn.add_css_class("slider-mute-btn");
    mute_btn.set_child(Some(&icon_container));
    
    let update_mute_icon_clone = update_mute_icon.clone();
    let muted_state_clone = muted_state.clone();
    let vol_icon_c = vol_icon.clone();
    
    mute_btn.connect_clicked(move |_| {
        let new_mute = !muted_state_clone.get();
        muted_state_clone.set(new_mute);

        let mute_val = if new_mute { "1" } else { "0" };
        let _ = std::process::Command::new("wpctl")
            .args(&["set-mute", "@DEFAULT_AUDIO_SINK@", mute_val])
            .spawn();
        let _ = std::process::Command::new("pactl")
            .args(&["set-sink-mute", "@DEFAULT_SINK@", mute_val])
            .spawn();
        let _ = std::process::Command::new("amixer")
            .args(&["set", "Master", if new_mute { "mute" } else { "unmute" }])
            .spawn();

        update_mute_icon_clone(new_mute);
        update_topbar_volume_icon(&vol_icon_c);
    });
    
    row_box.append(&mute_btn);

    update_mute_icon(muted_state.get());

    let scale = gtk4::Scale::with_range(gtk4::Orientation::Horizontal, 0.0, 100.0, 1.0);
    scale.adjustment().set_page_increment(5.0);
    scale.set_value(initial_val);
    scale.set_hexpand(true);
    scale.set_draw_value(false);

    let last_source: Rc<Cell<Option<gtk4::glib::SourceId>>> = Rc::new(Cell::new(None));
    let last_source_clone = last_source.clone();
    let vol_icon_c2 = vol_icon.clone();
    let update_mute_icon_c = update_mute_icon.clone();
    let muted_state_c = muted_state.clone();

    scale.connect_value_changed(move |s| {
        let val = s.value();
        value_label.set_text(&format!("{:.0}%", val));

        if let Some(id) = last_source_clone.take() {
            id.remove();
        }

        let last_source_inner = last_source_clone.clone();
        let vol_icon_inner = vol_icon_c2.clone();
        let update_mute_icon_inner = update_mute_icon_c.clone();
        let muted_state_inner = muted_state_c.clone();
        
        let new_id = gtk4::glib::timeout_add_local_once(
            std::time::Duration::from_millis(80),
            move || {
                last_source_inner.set(None);
                set_volume(val);
                if val > 0.0 {
                    muted_state_inner.set(false);
                    update_mute_icon_inner(false);
                }
                update_topbar_volume_icon(&vol_icon_inner);
            }
        );
        last_source_clone.set(Some(new_id));
    });

    row_box.append(&scale);

    let menu_btn = gtk4::Button::new();
    menu_btn.add_css_class("slider-menu-btn");
    let menu_icon = babydra_common::icon::get_system_or_file_icon("go-next-symbolic", "image-missing");
    menu_icon.set_pixel_size(12);
    menu_btn.set_child(Some(&menu_icon));

    let popover = gtk4::Popover::new();
    popover.add_css_class("taskbar-popover");
    popover.set_parent(&menu_btn);
    popover.set_position(gtk4::PositionType::Bottom);
    popover.set_has_arrow(true);

    let popover_clone = popover.clone();
    let update_mute_clone = update_mute_icon.clone();
    let on_popover_toggled_c = on_popover_toggled.clone();
    
    menu_btn.connect_clicked(move |_| {
        let update_mute_wrapper = {
            let update_mute_clone = update_mute_clone.clone();
            Rc::new(move || {
                update_mute_clone(is_muted());
            }) as Rc<dyn Fn()>
        };
        populate_audio_menu(&popover_clone, update_mute_wrapper);
        popover_clone.popup();
        if let Some(ref cb) = on_popover_toggled_c {
            cb(true);
        }
    });

    if let Some(ref cb) = on_popover_toggled {
        let cb_clone = cb.clone();
        popover.connect_closed(move |_| {
            cb_clone(false);
        });
    }

    row_box.append(&menu_btn);

    main_box.append(&header_box);
    main_box.append(&row_box);
    (main_box, scale)
}

fn populate_audio_menu(popover: &gtk4::Popover, update_mute_btn: Rc<dyn Fn()>) {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    container.add_css_class("audio-menu-popover");
    container.set_size_request(260, -1);

    let out_label = gtk4::Label::new(Some("Output Devices"));
    out_label.add_css_class("audio-menu-section-title");
    out_label.set_xalign(0.0);
    container.append(&out_label);

    let sinks = get_audio_devices(false);
    if sinks.is_empty() {
        let empty = gtk4::Label::new(Some("No output devices found"));
        empty.add_css_class("tile-subtitle");
        container.append(&empty);
    } else {
        for sink in sinks {
            let btn = gtk4::Button::new();
            btn.add_css_class("audio-menu-item-btn");
            if sink.is_default {
                btn.add_css_class("active");
            }
            
            let btn_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
            let icon = babydra_common::icon::get_icon_colored(
                "volume", 14, 
                if sink.is_default { "#ffffff" } else { "rgba(255, 255, 255, 0.5)" }
            );
            let name_label = gtk4::Label::new(Some(&sink.description));
            name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
            name_label.set_hexpand(true);
            name_label.set_halign(gtk4::Align::Start);
            
            btn_box.append(&icon);
            btn_box.append(&name_label);

            if sink.is_default {
                let check_label = gtk4::Label::new(Some("✓"));
                check_label.add_css_class("audio-menu-item-check");
                btn_box.append(&check_label);
            }

            btn.set_child(Some(&btn_box));

            let name = sink.name.clone();
            let pop_clone = popover.clone();
            let update_mute_clone = update_mute_btn.clone();
            btn.connect_clicked(move |_| {
                let _ = std::process::Command::new("wpctl")
                    .args(&["set-default", &name])
                    .status();
                populate_audio_menu(&pop_clone, update_mute_clone.clone());
            });
            container.append(&btn);
        }
    }

    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.set_margin_top(4);
    sep.set_margin_bottom(4);
    container.append(&sep);

    let in_label = gtk4::Label::new(Some("Input Devices"));
    in_label.add_css_class("audio-menu-section-title");
    in_label.set_xalign(0.0);
    container.append(&in_label);

    let sources = get_audio_devices(true);
    let mut input_added = false;
    for source in sources {
        if source.name.contains(".monitor") {
            continue;
        }
        input_added = true;
        let btn = gtk4::Button::new();
        btn.add_css_class("audio-menu-item-btn");
        if source.is_default {
            btn.add_css_class("active");
        }
        
        let btn_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        let icon = babydra_common::icon::get_icon_colored(
            "microphone", 14, 
            if source.is_default { "#ffffff" } else { "rgba(255, 255, 255, 0.5)" }
        );
        let name_label = gtk4::Label::new(Some(&source.description));
        name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        name_label.set_hexpand(true);
        name_label.set_halign(gtk4::Align::Start);
        
        btn_box.append(&icon);
        btn_box.append(&name_label);

        if source.is_default {
            let check_label = gtk4::Label::new(Some("✓"));
            check_label.add_css_class("audio-menu-item-check");
            btn_box.append(&check_label);
        }

        btn.set_child(Some(&btn_box));

        let name = source.name.clone();
        let pop_clone = popover.clone();
        let update_mute_clone = update_mute_btn.clone();
        btn.connect_clicked(move |_| {
            let _ = std::process::Command::new("wpctl")
                .args(&["set-default", &name])
                .status();
            populate_audio_menu(&pop_clone, update_mute_clone.clone());
        });
        container.append(&btn);
    }

    if !input_added {
        let empty = gtk4::Label::new(Some("No input devices found"));
        empty.add_css_class("tile-subtitle");
        container.append(&empty);
    }

    popover.set_child(Some(&container));
}
