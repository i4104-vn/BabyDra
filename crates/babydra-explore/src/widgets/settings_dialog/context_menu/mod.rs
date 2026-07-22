use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Align, ListBox, Entry, Grid, Overlay, Window};
use babydra_common::i18n::t;
use std::rc::Rc;
use std::cell::RefCell;

pub mod row;

const AVAILABLE_ICONS: &[&str] = &[
    "settings", "terminal", "folder", "text", "camera", "music", "user",
    "activity", "lock", "wifi", "refresh", "power", "search", "logo"
];

/// Builds the context menu options page inside the Settings Dialog.
/// Features a full-height scrolled list of options and a floating action button (FAB)
/// at the bottom-right corner to open the "Add Option" dialog.
pub fn build_context_menu_page(parent_window: &Window) -> Overlay {
    let settings = babydra_common::load_explore_settings();
    let overlay = Overlay::new();
    overlay.set_hexpand(true);
    overlay.set_vexpand(true);

    let scroll = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .hexpand(true)
        .vexpand(true)
        .build();
    overlay.set_child(Some(&scroll));

    let context_listbox = ListBox::new();
    context_listbox.set_selection_mode(gtk4::SelectionMode::None);
    context_listbox.add_css_class("settings-card");
    scroll.set_child(Some(&context_listbox));

    // Populate existing custom options
    for item in settings.custom_context_items {
        row::render_option_row(&context_listbox, item);
    }

    let btn_fab = babydra_utils::components::create_fab("plus");
    btn_fab.add_css_class("circular");
    btn_fab.set_margin_bottom(10);
    btn_fab.set_margin_end(10);
    btn_fab.set_tooltip_text(Some(&t("explore.settings_add_option")));
    btn_fab.set_cursor_from_name(Some("pointer"));

    overlay.add_overlay(&btn_fab);

    let listbox_c = context_listbox.clone();
    let parent_win_c = parent_window.clone();
    btn_fab.connect_clicked(move |_| {
        show_add_option_dialog(&parent_win_c, &listbox_c);
    });

    overlay
}

/// Displays a dedicated modal dialog for adding a new custom context menu option.
fn show_add_option_dialog(parent: &Window, listbox: &ListBox) {
    let dialog = Window::builder()
        .title(&t("explore.settings_add_option"))
        .transient_for(parent)
        .modal(true)
        .resizable(false)
        .default_width(420)
        .default_height(280)
        .css_classes(vec!["explore-dialog".to_string()])
        .build();

    let vbox = Box::new(Orientation::Vertical, 12);
    vbox.add_css_class("explore-dialog-box");
    vbox.set_margin_top(16);
    vbox.set_margin_bottom(16);
    vbox.set_margin_start(16);
    vbox.set_margin_end(16);
    dialog.set_child(Some(&vbox));

    let lbl_add_title = Label::builder()
        .label(&t("explore.settings_add_option"))
        .halign(Align::Start)
        .build();
    lbl_add_title.add_css_class("settings-row-title");
    vbox.append(&lbl_add_title);

    let grid = Grid::new();
    grid.set_row_spacing(10);
    grid.set_column_spacing(12);
    vbox.append(&grid);

    // Row 0: Option Name & Circular Icon Button
    let lbl_name_field = Label::new(Some(&t("explore.settings_option_name")));
    lbl_name_field.set_halign(Align::Start);
    lbl_name_field.add_css_class("settings-row-desc");

    let name_hbox = Box::new(Orientation::Horizontal, 8);
    let entry_name = Entry::builder()
        .placeholder_text(&t("explore.settings_placeholder_name"))
        .hexpand(true)
        .css_classes(vec!["small-entry".to_string(), "inline-entry".to_string()])
        .build();

    let selected_icon = Rc::new(RefCell::new("settings".to_string()));

    let popover_icon = gtk4::Popover::builder()
        .has_arrow(true)
        .autohide(true)
        .build();

    let popover_icon_c = popover_icon.clone();
    let btn_select_icon = babydra_utils::components::create_icon_button("settings", 16, &["circular", "icon-select-btn"], None, move || popover_icon_c.popup());
    btn_select_icon.set_valign(Align::Center);
    btn_select_icon.set_cursor_from_name(Some("pointer"));
    popover_icon.set_parent(&btn_select_icon);

    let icon_grid = Grid::new();
    icon_grid.set_row_spacing(6);
    icon_grid.set_column_spacing(6);
    icon_grid.set_margin_top(8);
    icon_grid.set_margin_bottom(8);
    icon_grid.set_margin_start(8);
    icon_grid.set_margin_end(8);

    let cols = 4;
    for (idx, icon_name) in AVAILABLE_ICONS.iter().enumerate() {
        let r = (idx / cols) as i32;
        let c = (idx % cols) as i32;

        let icon_name_str = icon_name.to_string();
        let selected_icon_c = selected_icon.clone();
        let btn_select_icon_c = btn_select_icon.clone();
        let popover_icon_c = popover_icon.clone();

        let btn_item = babydra_utils::components::create_icon_button(icon_name, 20, &["flat", "icon-grid-item"], Some(*icon_name), move || { selected_icon_c.replace(icon_name_str.clone()); let new_img = babydra_utils::ui::icon::get_icon(&icon_name_str, 16); new_img.set_pixel_size(16); btn_select_icon_c.set_child(Some(&new_img)); popover_icon_c.popdown(); });
        btn_item.set_cursor_from_name(Some("pointer"));

        icon_grid.attach(&btn_item, c, r, 1, 1);
    }
    popover_icon.set_child(Some(&icon_grid));

    name_hbox.append(&entry_name);
    name_hbox.append(&btn_select_icon);

    grid.attach(&lbl_name_field, 0, 0, 1, 1);
    grid.attach(&name_hbox, 1, 0, 1, 1);

    // Row 1: Command & Placeholders
    let lbl_cmd_field = Label::new(Some(&t("explore.settings_option_command")));
    lbl_cmd_field.set_halign(Align::Start);
    lbl_cmd_field.add_css_class("settings-row-desc");

    let entry_cmd_vbox = Box::new(Orientation::Vertical, 4);
    let entry_cmd = Entry::builder()
        .placeholder_text(&t("explore.settings_placeholder_command"))
        .hexpand(true)
        .css_classes(vec!["small-entry".to_string(), "inline-entry".to_string()])
        .build();
    entry_cmd_vbox.append(&entry_cmd);

    let placeholders_box = Box::new(Orientation::Horizontal, 6);
    placeholders_box.set_margin_top(2);
    let placeholders = [
        ("{path}", "explore.placeholder_path_desc"),
        ("{dir}",  "explore.placeholder_dir_desc"),
        ("{name}", "explore.placeholder_name_desc"),
        ("{stem}", "explore.placeholder_stem_desc"),
        ("{ext}",  "explore.placeholder_ext_desc"),
    ];
    for (p, desc_key) in placeholders {
        let btn_p = babydra_utils::components::create_button(p);
        btn_p.remove_css_class("baby-button");
        btn_p.add_css_class("flat");
        btn_p.add_css_class("placeholder-btn");
        btn_p.set_tooltip_text(Some(&t(desc_key)));
        btn_p.set_cursor_from_name(Some("pointer"));
        let entry_cmd_c = entry_cmd.clone();
        btn_p.connect_clicked(move |_| {
            let mut pos = entry_cmd_c.position();
            entry_cmd_c.insert_text(p, &mut pos);
            entry_cmd_c.grab_focus();
        });
        placeholders_box.append(&btn_p);
    }
    entry_cmd_vbox.append(&placeholders_box);

    grid.attach(&lbl_cmd_field, 0, 1, 1, 1);
    grid.attach(&entry_cmd_vbox, 1, 1, 1, 1);

    // Action buttons (Cancel & Add)
    let bbox = Box::new(Orientation::Horizontal, 8);
    bbox.set_halign(Align::End);
    bbox.set_margin_top(8);
    vbox.append(&bbox);

    let btn_cancel = babydra_utils::components::create_button(&t("explore.settings_cancel"));
    let btn_add = babydra_utils::components::create_accent_button(&t("explore.settings_add"));
    btn_add.set_cursor_from_name(Some("pointer"));

    bbox.append(&btn_cancel);
    bbox.append(&btn_add);

    let win_cancel_btn = dialog.clone();
    btn_cancel.connect_clicked(move |_| {
        win_cancel_btn.close();
    });

    let win_cancel = dialog.clone();
    let vbox_cancel = vbox.clone();
    let is_animating = Rc::new(std::cell::Cell::new(false));
    let is_animating_cancel = is_animating.clone();
    dialog.connect_close_request(move |_| {
        if is_animating_cancel.get() {
            return glib::Propagation::Stop;
        }
        is_animating_cancel.set(true);
        let win_cb = win_cancel.clone();
        babydra_utils::ui::animation::genie_out(
            vbox_cancel.upcast_ref(),
            420,
            280,
            200,
            move || {
                win_cb.destroy();
            }
        );
        glib::Propagation::Stop
    });

    let entry_name_c = entry_name.clone();
    let entry_cmd_c = entry_cmd.clone();
    let selected_icon_c = selected_icon.clone();
    let listbox_c = listbox.clone();
    let win_add = dialog.clone();

    btn_add.connect_clicked(move |_| {
        let name_str = entry_name_c.text().to_string();
        let cmd_str = entry_cmd_c.text().to_string();
        let icon_str = selected_icon_c.borrow().clone();
        if !name_str.is_empty() && !cmd_str.is_empty() {
            let item = babydra_common::config::settings::CustomContextItem {
                name: name_str,
                command: cmd_str,
                icon: Some(icon_str),
            };

            let mut s = babydra_common::load_explore_settings();
            s.custom_context_items.push(item.clone());
            babydra_common::save_explore_settings(&s);

            row::render_option_row(&listbox_c, item);
            win_add.close();
        }
    });

    dialog.present();
    babydra_utils::ui::animation::genie_in(vbox.upcast_ref(), 420, 280, 200);
}