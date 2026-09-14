use super::{
    get_audio_devices, get_current_volume, is_muted, set_volume, update_topbar_volume_state,
};
use babydra_core::i18n::trans;
use babydra_core::services::system::volume::AudioDevice;
use babydra_ui_kit::components::PillSlider;
use gtk4::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

/// Creates a new `volume row`.
pub(crate) fn create_volume_row(
    on_popover_toggled: Option<Rc<dyn Fn(bool) + 'static>>,
    vol_icon: gtk4::Image,
) -> (gtk4::Box, Rc<dyn Fn(f64, bool)>) {
    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 4);
    main_box.add_css_class("control-slider-card");

    let initial_val = get_current_volume();
    let current_val = Rc::new(Cell::new(initial_val));
    let (header_box, value_label) =
        babydra_ui_kit::components::create_slider_header(&trans("volume.title"), initial_val);

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
                babydra_ui_kit::ui::icon::get_icon_colored("volume-mute", 16, "#ffffff")
            } else {
                babydra_ui_kit::ui::icon::get_icon_colored("volume", 16, "#ffffff")
            };
            icon_widget.add_css_class("slider-icon");
            icon_container.append(&icon_widget);
        })
    };

    let mute_btn = gtk4::Button::new();
    mute_btn.add_css_class("slider-overlay-mute-btn");
    mute_btn.set_child(Some(&icon_container));
    mute_btn.set_halign(gtk4::Align::Start);
    mute_btn.set_valign(gtk4::Align::Center);
    mute_btn.set_margin_start(10);
    mute_btn.set_can_focus(false);
    mute_btn.set_focus_on_click(false);

    {
        let update_mute_icon_clone = update_mute_icon.clone();
        let muted_state_clone = muted_state.clone();
        let vol_icon_c = vol_icon.clone();
        let current_val_c = current_val.clone();
        mute_btn.connect_clicked(move |_| {
            let new_mute = !muted_state_clone.get();
            muted_state_clone.set(new_mute);
            babydra_core::volume::set_muted(new_mute);
            update_mute_icon_clone(new_mute);
            update_topbar_volume_state(&vol_icon_c, current_val_c.get(), new_mute);
        });
    }
    update_mute_icon(muted_state.get());

    let slider = PillSlider::new(initial_val, |_| {});
    slider.bind_label(&value_label);
    slider.add_scroll_to(&row_box);
    slider.add_scroll_to(&main_box);

    let vol_icon_c2 = vol_icon.clone();
    let update_mute_icon_c = update_mute_icon.clone();
    let muted_state_c = muted_state.clone();
    let current_val_c2 = current_val.clone();
    slider.connect_debounced(80, move |val| {
        current_val_c2.set(val);
        set_volume(val);
        if val > 0.0 {
            muted_state_c.set(false);
            update_mute_icon_c(false);
        }
        update_topbar_volume_state(&vol_icon_c2, val, muted_state_c.get());
    });

    let menu_btn = gtk4::Button::new();
    menu_btn.add_css_class("slider-popover-btn");
    menu_btn.set_valign(gtk4::Align::Center);
    let menu_icon = babydra_ui_kit::ui::icon::get_icon("go-up-symbolic", 12);
    menu_btn.set_child(Some(&menu_icon));

    let overlay = gtk4::Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_valign(gtk4::Align::Center);
    overlay.set_child(Some(&slider.container));
    overlay.add_overlay(&mute_btn);

    row_box.append(&overlay);
    row_box.append(&menu_btn);

    let popover = babydra_ui_kit::components::create_popover(
        &menu_btn,
        gtk4::PositionType::Bottom,
        "volume-popover audio-popover control-popover",
    );
    popover.set_has_arrow(true);

    let popover_clone = popover.clone();
    let update_mute_clone = update_mute_icon.clone();
    let on_popover_toggled_c = on_popover_toggled.clone();
    let menu_btn_clone = menu_btn.clone();

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

        let down_icon = babydra_ui_kit::ui::icon::get_icon("go-down-symbolic", 12);
        menu_btn_clone.set_child(Some(&down_icon));
    });

    let menu_btn_c2 = menu_btn.clone();
    let on_popover_toggled_c2 = on_popover_toggled.clone();
    popover.connect_closed(move |_| {
        if let Some(ref cb) = on_popover_toggled_c2 {
            cb(false);
        }
        let up_icon = babydra_ui_kit::ui::icon::get_icon("go-up-symbolic", 12);
        menu_btn_c2.set_child(Some(&up_icon));
    });

    main_box.append(&header_box);
    main_box.append(&row_box);

    let slider_sync = slider.clone();
    let update_mute_icon_sync = update_mute_icon.clone();
    let muted_state_sync = muted_state.clone();
    let current_val_sync = current_val.clone();
    let vol_icon_sync = vol_icon.clone();
    let sync_callback: Rc<dyn Fn(f64, bool)> = Rc::new(move |vol: f64, is_m: bool| {
        current_val_sync.set(vol);
        muted_state_sync.set(is_m);
        slider_sync.set_value_silent(vol);
        update_mute_icon_sync(is_m);
        update_topbar_volume_state(&vol_icon_sync, vol, is_m);
    });

    (main_box, sync_callback)
}

/// Populate audio menu.
fn populate_audio_menu(popover: &gtk4::Popover, update_mute_btn: Rc<dyn Fn()>) {
    let container = gtk4::Box::new(gtk4::Orientation::Vertical, 6);
    container.add_css_class("audio-menu-popover");
    container.set_size_request(260, -1);
    let menu = AudioMenu {
        container: &container,
        popover,
        update_mute_btn: &update_mute_btn,
    };

    append_device_section(
        &menu,
        &trans("volume.output_devices"),
        &trans("volume.no_output"),
        "volume",
        get_audio_devices(false),
        babydra_core::volume::select_audio_device,
    );

    let sep = gtk4::Separator::new(gtk4::Orientation::Horizontal);
    sep.set_margin_top(4);
    sep.set_margin_bottom(4);
    container.append(&sep);

    append_device_section(
        &menu,
        &trans("volume.input_devices"),
        &trans("volume.no_input"),
        "microphone",
        get_audio_devices(true)
            .into_iter()
            .filter(|source| !source.name.contains(".monitor")),
        babydra_core::volume::select_audio_source,
    );

    popover.set_child(Some(&container));
}

struct AudioMenu<'a> {
    container: &'a gtk4::Box,
    popover: &'a gtk4::Popover,
    update_mute_btn: &'a Rc<dyn Fn()>,
}

fn append_device_section(
    menu: &AudioMenu<'_>,
    title: &str,
    empty_message: &str,
    icon_name: &str,
    devices: impl IntoIterator<Item = AudioDevice>,
    select_device: fn(&str),
) {
    let title_label = gtk4::Label::new(Some(title));
    title_label.add_css_class("audio-menu-section-title");
    title_label.set_xalign(0.0);
    menu.container.append(&title_label);

    let mut has_devices = false;
    for device in devices {
        has_devices = true;
        let button = gtk4::Button::new();
        button.add_css_class("audio-menu-item-btn");
        if device.is_default {
            button.add_css_class("active");
        }

        let content = gtk4::Box::new(gtk4::Orientation::Horizontal, 8);
        let icon = babydra_ui_kit::ui::icon::get_icon_colored(
            icon_name,
            14,
            if device.is_default {
                "#ffffff"
            } else {
                "rgba(255, 255, 255, 0.5)"
            },
        );
        let name_label = gtk4::Label::new(Some(&device.description));
        name_label.set_ellipsize(gtk4::pango::EllipsizeMode::End);
        name_label.set_hexpand(true);
        name_label.set_halign(gtk4::Align::Start);
        content.append(&icon);
        content.append(&name_label);

        if device.is_default {
            let check_label = gtk4::Label::new(Some("✓"));
            check_label.add_css_class("audio-menu-item-check");
            content.append(&check_label);
        }
        button.set_child(Some(&content));

        let name = device.name;
        let popover = menu.popover.clone();
        let update_mute_btn = menu.update_mute_btn.clone();
        button.connect_clicked(move |_| {
            select_device(&name);
            let update_mute_btn = update_mute_btn.clone();
            gtk4::glib::timeout_add_local_once(std::time::Duration::from_millis(150), {
                let popover = popover.clone();
                move || populate_audio_menu(&popover, update_mute_btn)
            });
        });
        menu.container.append(&button);
    }

    if !has_devices {
        let empty = gtk4::Label::new(Some(empty_message));
        empty.add_css_class("tile-subtitle");
        menu.container.append(&empty);
    }
}
