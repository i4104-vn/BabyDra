use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Button, Window, Align, Notebook, Switch, ListBox, ListBoxRow};
use babydra_common::i18n::t;

pub fn show_settings_dialog(parent: &gtk4::Window, on_change_callback: impl Fn() + 'static) {
    let settings = babydra_common::load_explore_settings();

    let window = Window::builder()
        .title(&t("explore.settings"))
        .transient_for(parent)
        .modal(true)
        .resizable(true)
        .default_width(550)
        .default_height(450)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    window.set_child(Some(&vbox));

    let lbl_title = Label::builder()
        .label(&t("explore.settings"))
        .halign(Align::Start)
        .build();
    lbl_title.add_css_class("settings-title-label");
    vbox.append(&lbl_title);

    let notebook = Notebook::new();
    vbox.append(&notebook);

    // ── Tab 1: General Settings ───────────────────────────────
    let tab_general = Box::new(Orientation::Vertical, 10);
    tab_general.set_margin_top(10);
    tab_general.set_margin_bottom(10);
    tab_general.set_margin_start(10);
    tab_general.set_margin_end(10);

    let listbox = ListBox::new();
    listbox.set_selection_mode(gtk4::SelectionMode::None);
    listbox.add_css_class("settings-listbox");
    tab_general.append(&listbox);

    // Helper to add switch row
    let add_switch_row = |listbox: &ListBox, label_text: &str, active: bool, on_toggle: std::boxed::Box<dyn Fn(bool)>| {
        let row = ListBoxRow::new();
        let hbox = Box::new(Orientation::Horizontal, 12);
        hbox.set_margin_top(12);
        hbox.set_margin_bottom(12);
        hbox.set_margin_start(12);
        hbox.set_margin_end(12);

        let lbl = Label::builder()
            .label(label_text)
            .halign(Align::Start)
            .hexpand(true)
            .build();
        hbox.append(&lbl);

        let sw = Switch::builder()
            .active(active)
            .halign(Align::End)
            .valign(Align::Center)
            .build();
        
        sw.connect_active_notify(move |switch| {
            let state = switch.is_active();
            on_toggle(state);
        });

        hbox.append(&sw);
        row.set_child(Some(&hbox));
        listbox.append(&row);
    };

    // 1. Show hidden files
    add_switch_row(
        &listbox,
        &t("explore.toggle_hidden"), // "Ẩn/hiện tệp ẩn"
        settings.show_hidden,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.show_hidden = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 2. Preview Visible
    add_switch_row(
        &listbox,
        &t("explore.toggle_preview"), // "Ẩn/hiện xem trước"
        settings.preview_visible,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.preview_visible = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 3. Double click to open
    add_switch_row(
        &listbox,
        &t("explore.settings_double_click"),
        settings.double_click_to_open,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.double_click_to_open = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 4. Permanent delete
    add_switch_row(
        &listbox,
        &t("explore.settings_permanent_delete"),
        settings.permanent_delete,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.permanent_delete = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    // 5. Calculate folder sizes
    add_switch_row(
        &listbox,
        &t("explore.settings_calculate_size"),
        settings.calculate_dir_size,
        std::boxed::Box::new(|state| {
            let mut s = babydra_common::load_explore_settings();
            s.calculate_dir_size = state;
            babydra_common::save_explore_settings(&s);
        }),
    );

    let lbl_general_tab = Label::new(Some(&t("explore.settings_general")));
    notebook.append_page(&tab_general, Some(&lbl_general_tab));

    // ── Tab 2: Context Menu Configuration (Placeholder for Phase 4) ──
    let tab_context = Box::new(Orientation::Vertical, 10);
    tab_context.set_margin_top(10);
    tab_context.set_margin_bottom(10);
    tab_context.set_margin_start(10);
    tab_context.set_margin_end(10);

    let lbl_placeholder = Label::new(Some("Context Menu Customization (Phase 4)"));
    tab_context.append(&lbl_placeholder);

    let lbl_context_tab = Label::new(Some(&t("explore.settings_context_menu")));
    notebook.append_page(&tab_context, Some(&lbl_context_tab));

    // ── Bottom Action Area ─────────────────────────────────────
    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    vbox.append(&bbox);

    let close_text = if babydra_common::i18n::get_locale() == "vi" { "Đóng" } else { "Close" };
    let btn_close = Button::builder()
        .label(close_text)
        .css_classes(vec!["suggested-action".to_string()])
        .build();
    bbox.append(&btn_close);

    // Wire close and callback
    let on_change = std::rc::Rc::new(on_change_callback);
    let win_c = window.clone();
    let on_change_c = on_change.clone();
    btn_close.connect_clicked(move |_| {
        on_change_c();
        win_c.close();
    });

    // Also trigger on_change when window is destroyed/closed
    let on_change_destroy = on_change.clone();
    window.connect_destroy(move |_| {
        on_change_destroy();
    });

    window.present();
}
