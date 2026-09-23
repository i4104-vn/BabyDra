use super::row::render_option_row;
use super::AVAILABLE_ICONS;
use babydra_core::i18n::trans;
use gtk4::prelude::*;
use gtk4::{Align, Box, Entry, Grid, Label, ListBox, Orientation, Window};
use std::cell::RefCell;
use std::rc::Rc;

/// Displays the modal form used to add a custom context-menu option.
pub(super) fn show_add_option_dialog(parent: &Window, listbox: &ListBox) {
    let dialog = Window::builder()
        .title(trans("explore.settings_add_option"))
        .transient_for(parent)
        .modal(true)
        .resizable(false)
        .default_width(420)
        .default_height(280)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let form = Box::new(Orientation::Vertical, 12);
    form.add_css_class("explore-dialog-box");
    form.set_margin_top(16);
    form.set_margin_bottom(16);
    form.set_margin_start(16);
    form.set_margin_end(16);
    dialog.set_child(Some(&form));

    let title = Label::builder()
        .label(trans("explore.settings_add_option"))
        .halign(Align::Start)
        .build();
    title.add_css_class("settings-row-title");
    form.append(&title);

    let grid = Grid::new();
    grid.set_row_spacing(10);
    grid.set_column_spacing(12);
    form.append(&grid);

    let name_label = field_label("explore.settings_option_name");
    let name_entry = Entry::builder()
        .placeholder_text(trans("explore.settings_placeholder_name"))
        .hexpand(true)
        .css_classes(vec![
            "small-entry".to_string(),
            "inline-entry".to_string(),
            "modern-dialog-entry".to_string(),
        ])
        .build();
    let selected_icon = Rc::new(RefCell::new("settings".to_string()));
    let name_control = icon_entry(name_entry.clone(), selected_icon.clone(), "settings");
    grid.attach(&name_label, 0, 0, 1, 1);
    grid.attach(&name_control, 1, 0, 1, 1);

    let command_label = field_label("explore.settings_option_command");
    let command_entry = Entry::builder()
        .placeholder_text(trans("explore.settings_placeholder_command"))
        .hexpand(true)
        .css_classes(vec![
            "small-entry".to_string(),
            "inline-entry".to_string(),
            "modern-dialog-entry".to_string(),
        ])
        .build();
    let command_control = command_entry_with_placeholders(command_entry.clone());
    grid.attach(&command_label, 0, 1, 1, 1);
    grid.attach(&command_control, 1, 1, 1, 1);

    let actions = Box::new(Orientation::Horizontal, 10);
    actions.set_halign(Align::End);
    actions.set_margin_top(10);
    form.append(&actions);

    let cancel = babydra_ui_kit::components::create_button(&trans("explore.settings_cancel"));
    cancel.add_css_class("modern-dialog-cancel-btn");
    cancel.set_cursor_from_name(Some("pointer"));
    let add = babydra_ui_kit::components::create_accent_button(&trans("explore.settings_add"));
    add.add_css_class("modern-dialog-primary-btn");
    add.set_cursor_from_name(Some("pointer"));
    actions.append(&cancel);
    actions.append(&add);

    let dialog_to_close = dialog.clone();
    cancel.connect_clicked(move |_| dialog_to_close.close());

    let dialog_to_close = dialog.clone();
    let form_to_close = form.clone();
    let is_animating = Rc::new(std::cell::Cell::new(false));
    let is_animating_close = is_animating.clone();
    dialog.connect_close_request(move |_| {
        if is_animating_close.replace(true) {
            return glib::Propagation::Stop;
        }

        let dialog = dialog_to_close.clone();
        babydra_ui_kit::ui::animation::genie_out(
            form_to_close.upcast_ref(),
            420,
            280,
            200,
            move || {
                dialog.destroy();
            },
        );
        glib::Propagation::Stop
    });

    let listbox = listbox.clone();
    let dialog_to_close = dialog.clone();
    add.connect_clicked(move |_| {
        let name = name_entry.text().to_string();
        let command = command_entry.text().to_string();
        if name.is_empty() || command.is_empty() {
            return;
        }

        let item = babydra_core::config::settings::CustomContextItem {
            name,
            command,
            icon: Some(selected_icon.borrow().clone()),
        };
        let mut settings = babydra_core::load_explore_cfg();
        settings.custom_context_items.push(item.clone());
        babydra_core::save_explore_cfg(&settings);
        render_option_row(&listbox, item);
        dialog_to_close.close();
    });

    dialog.present();
    babydra_ui_kit::ui::animation::genie_in(form.upcast_ref(), 420, 280, 200);
}

fn field_label(key: &str) -> Label {
    let label = Label::new(Some(&trans(key)));
    label.set_halign(Align::Start);
    label.add_css_class("settings-row-desc");
    label
}

fn icon_entry(entry: Entry, selected_icon: Rc<RefCell<String>>, initial_icon: &str) -> Box {
    let container = Box::new(Orientation::Horizontal, 8);
    container.append(&entry);

    let popover = gtk4::Popover::builder()
        .has_arrow(true)
        .autohide(true)
        .build();
    let popover_to_open = popover.clone();
    let select_button = babydra_ui_kit::components::create_icon_button(
        initial_icon,
        16,
        &["circular", "icon-select-btn"],
        None,
        move || popover_to_open.popup(),
    );
    select_button.set_valign(Align::Center);
    select_button.set_cursor_from_name(Some("pointer"));
    popover.set_parent(&select_button);

    let icon_grid = Grid::new();
    icon_grid.set_row_spacing(6);
    icon_grid.set_column_spacing(6);
    icon_grid.set_margin_top(8);
    icon_grid.set_margin_bottom(8);
    icon_grid.set_margin_start(8);
    icon_grid.set_margin_end(8);

    for (index, icon_name) in AVAILABLE_ICONS.iter().enumerate() {
        let icon_name = icon_name.to_string();
        let icon_name_for_callback = icon_name.clone();
        let selected_icon = selected_icon.clone();
        let select_button = select_button.clone();
        let popover = popover.clone();
        let button = babydra_ui_kit::components::create_icon_button(
            &icon_name,
            20,
            &["flat", "icon-grid-item"],
            Some(icon_name.as_str()),
            move || {
                selected_icon.replace(icon_name_for_callback.clone());
                let image = babydra_ui_kit::ui::icon::get_icon(&icon_name_for_callback, 16);
                image.set_pixel_size(16);
                select_button.set_child(Some(&image));
                popover.popdown();
            },
        );
        button.set_cursor_from_name(Some("pointer"));
        icon_grid.attach(&button, (index % 4) as i32, (index / 4) as i32, 1, 1);
    }
    popover.set_child(Some(&icon_grid));
    container.append(&select_button);
    container
}

fn command_entry_with_placeholders(entry: Entry) -> Box {
    let container = Box::new(Orientation::Vertical, 4);
    container.append(&entry);

    let placeholders = Box::new(Orientation::Horizontal, 6);
    placeholders.set_margin_top(2);
    for (placeholder, tooltip) in [
        ("%f", "explore.settings_tooltip_f"),
        ("%d", "explore.settings_tooltip_d"),
        ("%n", "explore.settings_tooltip_n"),
    ] {
        let button = babydra_ui_kit::components::create_button(placeholder);
        button.add_css_class("flat");
        button.add_css_class("placeholder-btn");
        button.set_tooltip_text(Some(&trans(tooltip)));
        button.set_cursor_from_name(Some("pointer"));
        let entry = entry.clone();
        button.connect_clicked(move |_| {
            let mut position = entry.position();
            entry.insert_text(placeholder, &mut position);
            entry.grab_focus();
        });
        placeholders.append(&button);
    }
    container.append(&placeholders);
    container
}
