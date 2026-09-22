//! Settings dialog shell: navigation, page stack, and dialog lifecycle.

use super::pages::{build_editor_page, build_font_page, build_saving_page};
use babydra_core::i18n::trans;
use babydra_core::models::notepad::{load_notepad_cfg, NotepadSettings};
use babydra_ui_kit::components::create_sidebar_btn;
use gtk4::prelude::*;
use gtk4::{Box, Button, Orientation, ScrolledWindow, Stack, Window};
use std::cell::RefCell;
use std::rc::Rc;

pub fn show_settings_dialog(
    parent: &impl IsA<Window>,
    on_change_callback: impl Fn(NotepadSettings) + 'static,
) {
    let state = Rc::new(RefCell::new(load_notepad_cfg()));
    let changed: Rc<dyn Fn(NotepadSettings)> = Rc::new(on_change_callback);
    let window = Window::builder()
        .title(trans("notepad.settings"))
        .transient_for(parent.as_ref())
        .modal(true)
        .resizable(false)
        .default_width(650)
        .default_height(480)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let root = Box::new(Orientation::Vertical, 0);
    root.add_css_class("explore-dialog-box");
    let content = Box::new(Orientation::Horizontal, 0);
    content.set_hexpand(true);
    content.set_vexpand(true);

    let stack = Stack::new();
    stack.set_transition_type(gtk4::StackTransitionType::Crossfade);
    stack.set_transition_duration(250);
    stack.set_hexpand(true);
    stack.set_vexpand(true);
    stack.add_named(&build_font_page(&state, &changed), Some("font"));
    stack.add_named(&build_editor_page(&state, &changed), Some("editor"));
    stack.add_named(&build_saving_page(&state, &changed), Some("saving"));

    content.append(&build_navigation(&stack));
    let stack_container = Box::new(Orientation::Vertical, 0);
    stack_container.set_margin_top(12);
    stack_container.set_margin_start(16);
    stack_container.set_margin_end(16);
    stack_container.set_margin_bottom(12);
    stack_container.set_hexpand(true);
    stack_container.set_vexpand(true);
    stack_container.append(&stack);
    content.append(&stack_container);
    root.append(&content);

    let footer = Box::new(Orientation::Horizontal, 8);
    footer.set_halign(gtk4::Align::End);
    footer.set_margin_end(16);
    footer.set_margin_bottom(12);
    let close = Button::builder()
        .label(trans("notepad.settings_save_btn"))
        .css_classes(vec!["action-btn".to_string(), "active".to_string()])
        .build();
    let window_c = window.clone();
    close.connect_clicked(move |_| window_c.close());
    footer.append(&close);
    root.append(&footer);
    window.set_child(Some(&root));
    window.present();
}

fn build_navigation(stack: &Stack) -> ScrolledWindow {
    let sidebar = ScrolledWindow::new();
    sidebar.set_hscrollbar_policy(gtk4::PolicyType::Never);
    sidebar.add_css_class("sidebar");
    sidebar.set_width_request(185);
    sidebar.set_hexpand(false);
    sidebar.set_vexpand(true);
    sidebar.set_margin_top(8);
    sidebar.set_margin_bottom(8);
    sidebar.set_margin_start(8);

    let container = Box::new(Orientation::Vertical, 2);
    container.set_margin_top(4);
    container.set_margin_bottom(4);
    sidebar.set_child(Some(&container));
    let buttons = [
        create_sidebar_btn(
            &trans("notepad.settings_font"),
            "edit",
            "sidebar-item",
            || {},
        ),
        create_sidebar_btn(
            &trans("notepad.settings_editor"),
            "sliders",
            "sidebar-item",
            || {},
        ),
        create_sidebar_btn(
            &trans("notepad.settings_formatting"),
            "save",
            "sidebar-item",
            || {},
        ),
    ];
    buttons[0].add_css_class("active-nav");
    for button in &buttons {
        button.set_cursor_from_name(Some("pointer"));
        container.append(button);
    }

    for (index, name) in ["font", "editor", "saving"].iter().enumerate() {
        let stack_c = stack.clone();
        let buttons_c = buttons.clone();
        let name = (*name).to_string();
        buttons[index].connect_clicked(move |_| {
            for (button_index, button) in buttons_c.iter().enumerate() {
                if button_index == index {
                    button.add_css_class("active-nav");
                } else {
                    button.remove_css_class("active-nav");
                }
            }
            stack_c.set_visible_child_name(&name);
        });
    }
    sidebar
}
