//! Reusable UI helpers and utilities for babydra-settings widgets.

use gtk4::prelude::*;

/// Removes all children from a ListBox.
pub fn clear_list_box(list_box: &gtk4::ListBox) {
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }
}

/// Removes all children from a Box container.
pub fn clear_box(box_container: &gtk4::Box) {
    while let Some(child) = box_container.first_child() {
        box_container.remove(&child);
    }
}

/// Creates a standard icon badge containing a GTK icon widget.
pub fn create_icon_badge(icon_name: &str, icon_size: i32, is_small: bool) -> gtk4::Box {
    let icon_badge = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    if is_small {
        icon_badge.add_css_class("blue-icon-badge-sm");
        icon_badge.set_halign(gtk4::Align::Start);
    } else {
        icon_badge.add_css_class("blue-icon-badge");
        icon_badge.set_size_request(44, 44);
        icon_badge.set_halign(gtk4::Align::Center);
    }
    icon_badge.set_valign(gtk4::Align::Center);

    let icon = babydra_utils::ui::icon::get_icon(icon_name, icon_size);
    icon.set_pixel_size(icon_size);
    icon.set_valign(gtk4::Align::Center);
    icon.set_halign(gtk4::Align::Center);
    icon.set_vexpand(true);
    icon_badge.append(&icon);
    icon_badge
}

/// Standard placeholder states for settings ListBox containers.
pub enum PlaceholderState<'a> {
    Disabled { title_key: &'a str, desc_key: &'a str, icon_name: &'a str },
    Loading,
    Empty { title_key: &'a str, desc_key: Option<&'a str>, icon_name: &'a str },
}

/// Constructs a unified ListBoxRow placeholder for disabled, loading, or empty states.
pub fn create_placeholder_row(state: PlaceholderState) -> gtk4::ListBoxRow {
    let row = gtk4::ListBoxRow::new();
    row.add_css_class("settings-card-row");
    row.set_selectable(false);
    row.set_activatable(false);
    row.set_vexpand(true);
    row.set_valign(gtk4::Align::Fill);

    let placeholder_box = gtk4::Box::new(gtk4::Orientation::Vertical, 12);
    placeholder_box.set_valign(gtk4::Align::Center);
    placeholder_box.set_halign(gtk4::Align::Center);
    placeholder_box.set_vexpand(true);
    placeholder_box.set_hexpand(true);
    placeholder_box.set_margin_top(40);
    placeholder_box.set_margin_bottom(40);

    match state {
        PlaceholderState::Disabled { title_key, desc_key, icon_name } => {
            let badge = create_icon_badge(icon_name, 24, false);
            placeholder_box.append(&badge);

            let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t(title_key)));
            lbl.add_css_class("settings-row-title");
            lbl.set_halign(gtk4::Align::Center);
            placeholder_box.append(&lbl);

            let desc = gtk4::Label::new(Some(&babydra_common::i18n::t(desc_key)));
            desc.add_css_class("settings-row-desc");
            desc.set_halign(gtk4::Align::Center);
            placeholder_box.append(&desc);
        }
        PlaceholderState::Loading => {
            let spinner = gtk4::Spinner::new();
            spinner.set_size_request(32, 32);
            spinner.set_halign(gtk4::Align::Center);
            spinner.start();
            placeholder_box.append(&spinner);

            let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t("settings.loading")));
            lbl.add_css_class("settings-row-title");
            lbl.set_halign(gtk4::Align::Center);
            placeholder_box.append(&lbl);
        }
        PlaceholderState::Empty { title_key, desc_key, icon_name } => {
            let badge = create_icon_badge(icon_name, 24, false);
            placeholder_box.append(&badge);

            let lbl = gtk4::Label::new(Some(&babydra_common::i18n::t(title_key)));
            lbl.add_css_class("settings-row-title");
            lbl.set_halign(gtk4::Align::Center);
            placeholder_box.append(&lbl);

            if let Some(desc_k) = desc_key {
                let desc = gtk4::Label::new(Some(&babydra_common::i18n::t(desc_k)));
                desc.add_css_class("settings-row-desc");
                desc.set_halign(gtk4::Align::Center);
                placeholder_box.append(&desc);
            }
        }
    }

    row.set_child(Some(&placeholder_box));
    row
}

/// Creates a standard glass panel container enclosing a ScrolledWindow and ListBox.
pub fn create_scrollable_glass_list() -> (gtk4::Box, gtk4::ListBox, gtk4::ScrolledWindow) {
    let glass_card = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    glass_card.add_css_class("glass-panel");
    glass_card.set_vexpand(true);
    glass_card.set_valign(gtk4::Align::Fill);

    let list_box = gtk4::ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&list_box));

    glass_card.append(&scroll);
    (glass_card, list_box, scroll)
}

/// Spawns a background task thread and invokes the `on_done` callback on the main GTK thread upon completion.
pub fn spawn_async_task<T, F, G>(task: F, on_done: G, poll_ms: u64)
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
    G: FnOnce(T) + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel::<T>();
    std::thread::spawn(move || {
        let res = task();
        let _ = tx.send(res);
    });

    let mut on_done_opt = Some(on_done);
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(poll_ms), move || {
        if let Ok(res) = rx.try_recv() {
            if let Some(cb) = on_done_opt.take() {
                cb(res);
            }
            gtk4::glib::ControlFlow::Break
        } else {
            gtk4::glib::ControlFlow::Continue
        }
    });
}
