use gtk4::prelude::*;
use gtk4::{Window, Label, Box, Orientation, Align, Button};
use std::rc::Rc;
use std::cell::RefCell;
use babydra_common::i18n::t;

pub fn show_capture_dialog(
    parent: &Window,
    action_desc: &str,
    on_capture: impl Fn(String) + 'static,
) {
    let window = Window::builder()
        .title(&t("explore.settings_capture_title"))
        .transient_for(parent)
        .modal(true)
        .resizable(false)
        .default_width(360)
        .default_height(200)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 16);
    vbox.set_margin_top(24);
    vbox.set_margin_bottom(24);
    vbox.set_margin_start(24);
    vbox.set_margin_end(24);
    vbox.set_valign(Align::Center);
    vbox.add_css_class("explore-dialog-box");
    window.set_child(Some(&vbox));

    let lbl_desc = Label::builder()
        .label(&format!("Press key combination for:\n{}", action_desc))
        .halign(Align::Center)
        .justify(gtk4::Justification::Center)
        .build();
    lbl_desc.add_css_class("settings-row-title");
    vbox.append(&lbl_desc);

    let lbl_shortcut = Label::builder()
        .label("Press any keys...")
        .halign(Align::Center)
        .build();
    lbl_shortcut.add_css_class("keybind-pill");
    lbl_shortcut.add_css_class("settings-item-command");
    vbox.append(&lbl_shortcut);

    let captured_shortcut = Rc::new(RefCell::new(None::<String>));

    let key_controller = gtk4::EventControllerKey::new();
    let lbl_shortcut_c = lbl_shortcut.clone();
    let captured_c = captured_shortcut.clone();
    key_controller.connect_key_pressed(move |_, keyval, _, state| {
        let clean_state = state & (gtk4::gdk::ModifierType::CONTROL_MASK 
                                 | gtk4::gdk::ModifierType::SHIFT_MASK 
                                 | gtk4::gdk::ModifierType::ALT_MASK);

        let is_modifier_only = keyval == gtk4::gdk::Key::Control_L 
            || keyval == gtk4::gdk::Key::Control_R
            || keyval == gtk4::gdk::Key::Shift_L 
            || keyval == gtk4::gdk::Key::Shift_R
            || keyval == gtk4::gdk::Key::Alt_L 
            || keyval == gtk4::gdk::Key::Alt_R;

        if !is_modifier_only {
            let shortcut_str = keyval_to_string(&keyval, clean_state);
            lbl_shortcut_c.set_text(&shortcut_str);
            captured_c.replace(Some(shortcut_str));
        }
        glib::Propagation::Stop
    });
    window.add_controller(key_controller);

    let bbox = Box::new(Orientation::Horizontal, 12);
    bbox.set_halign(Align::Center);
    vbox.append(&bbox);

    let btn_save = Button::builder().label(&t("explore.save")).build();
    btn_save.add_css_class("baby-button");
    btn_save.set_cursor_from_name(Some("pointer"));

    let btn_cancel = Button::builder().label(&t("explore.cancel")).build();
    btn_cancel.add_css_class("baby-button");
    btn_cancel.set_cursor_from_name(Some("pointer"));

    bbox.append(&btn_save);
    bbox.append(&btn_cancel);

    let win_c = window.clone();
    btn_cancel.connect_clicked(move |_| {
        win_c.close();
    });

    let win_save = window.clone();
    let on_cap = Rc::new(on_capture);
    btn_save.connect_clicked(move |_| {
        if let Some(ref val) = *captured_shortcut.borrow() {
            on_cap(val.clone());
        }
        win_save.close();
    });

    // Close Request & Animation Setup
    let win_cancel = window.clone();
    let vbox_cancel = vbox.clone();
    let is_animating = Rc::new(std::cell::Cell::new(false));
    let is_animating_cancel = is_animating.clone();
    window.connect_close_request(move |_| {
        if is_animating_cancel.get() {
            return glib::Propagation::Stop;
        }
        is_animating_cancel.set(true);
        let win_cb = win_cancel.clone();
        babydra_utils::ui::animation::genie_out(
            vbox_cancel.upcast_ref(),
            360,
            200,
            200,
            move || {
                win_cb.destroy();
            }
        );
        glib::Propagation::Stop
    });

    window.present();
    babydra_utils::ui::animation::genie_in(vbox.upcast_ref(), 360, 200, 200);
}

fn keyval_to_string(keyval: &gtk4::gdk::Key, state: gtk4::gdk::ModifierType) -> String {
    let mut parts = Vec::new();
    if state.contains(gtk4::gdk::ModifierType::CONTROL_MASK) {
        parts.push("Ctrl");
    }
    if state.contains(gtk4::gdk::ModifierType::ALT_MASK) {
        parts.push("Alt");
    }
    if state.contains(gtk4::gdk::ModifierType::SHIFT_MASK) {
        parts.push("Shift");
    }

    let key_name = if let Some(name) = keyval.name() {
        let name_str = name.to_string();
        match name_str.as_str() {
            "Return" => "Enter".to_string(),
            "space" => "Space".to_string(),
            "Escape" => "Esc".to_string(),
            _ => {
                if name_str.len() == 1 {
                    name_str.to_uppercase()
                } else {
                    name_str
                }
            }
        }
    } else {
        "Unknown".to_string()
    };
    
    parts.push(&key_name);
    parts.join(" + ")
}
