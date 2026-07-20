use gtk4::prelude::*;
use gtk4::{Box, Orientation, Label, Button, Align, ListBox, Entry, Grid, Separator};
use babydra_common::i18n::t;

pub mod row;

const AVAILABLE_ICONS: &[&str] = &[
    "settings", "terminal", "folder", "text", "camera", "music", "user",
    "activity", "lock", "wifi", "refresh", "power", "search", "logo"
];

/// Builds the context menu options page inside the Settings Dialog, displaying list of custom options and a new item form.
pub fn build_context_menu_page() -> Box {
    let settings = babydra_common::load_explore_settings();
    let tab_context = Box::new(Orientation::Vertical, 10);

    let lbl_context_title = Label::builder()
        .label(&t("explore.settings_context_menu"))
        .halign(Align::Start)
        .build();
    lbl_context_title.add_css_class("settings-title-label");
    tab_context.append(&lbl_context_title);

    let lbl_section_title = Label::builder()
        .label(&t("explore.settings_custom_options"))
        .halign(Align::Start)
        .build();
    lbl_section_title.add_css_class("settings-section-title");
    tab_context.append(&lbl_section_title);

    let scroll = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .vexpand(true)
        .min_content_height(140)
        .build();
    tab_context.append(&scroll);

    let context_listbox = ListBox::new();
    context_listbox.set_selection_mode(gtk4::SelectionMode::None);
    context_listbox.add_css_class("settings-listbox");
    scroll.set_child(Some(&context_listbox));

    let listbox_c = context_listbox.clone();

    // Populate existing custom options
    for item in settings.custom_context_items {
        row::render_option_row(&context_listbox, item);
    }

    let sep = Separator::new(Orientation::Horizontal);
    tab_context.append(&sep);

    // Form to add new option
    let form_box = Box::new(Orientation::Vertical, 8);
    tab_context.append(&form_box);

    let lbl_add_title = Label::builder()
        .label(&t("explore.settings_add_option"))
        .halign(Align::Start)
        .build();
    lbl_add_title.add_css_class("settings-section-subtitle");
    form_box.append(&lbl_add_title);

    let grid = Grid::new();
    grid.set_row_spacing(8);
    grid.set_column_spacing(12);
    form_box.append(&grid);

    // Row 0: Option Name & Circular Icon Button
    let lbl_name_field = Label::new(Some(&t("explore.settings_option_name")));
    lbl_name_field.set_halign(Align::Start);

    let name_hbox = Box::new(Orientation::Horizontal, 8);
    let entry_name = Entry::builder()
        .placeholder_text(&t("explore.settings_placeholder_name"))
        .hexpand(true)
        .css_classes(vec!["small-entry".to_string()])
        .build();
    
    // State to store currently selected icon
    let selected_icon = std::rc::Rc::new(std::cell::RefCell::new("settings".to_string()));

    let btn_select_icon = Button::builder()
        .css_classes(vec!["circular".to_string(), "icon-select-btn".to_string()])
        .valign(Align::Center)
        .build();
    btn_select_icon.set_cursor_from_name(Some("pointer"));
    
    // Set initial icon image
    let current_icon_img = babydra_utils::ui::icon::get_icon("settings", 16);
    current_icon_img.set_pixel_size(16);
    btn_select_icon.set_child(Some(&current_icon_img));

    // Popover setup
    let popover_icon = gtk4::Popover::builder()
        .has_arrow(true)
        .autohide(true)
        .build();
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
        
        let img = babydra_utils::ui::icon::get_icon(icon_name, 20);
        img.set_pixel_size(20);
        
        let btn_item = Button::builder()
            .child(&img)
            .css_classes(vec!["flat".to_string(), "icon-grid-item".to_string()])
            .tooltip_text(*icon_name)
            .build();
        btn_item.set_cursor_from_name(Some("pointer"));

        let icon_name_str = icon_name.to_string();
        let selected_icon_c = selected_icon.clone();
        let btn_select_icon_c = btn_select_icon.clone();
        let popover_icon_c = popover_icon.clone();
        
        btn_item.connect_clicked(move |_| {
            selected_icon_c.replace(icon_name_str.clone());
            
            // Update button child with selected icon
            let new_img = babydra_utils::ui::icon::get_icon(&icon_name_str, 16);
            new_img.set_pixel_size(16);
            btn_select_icon_c.set_child(Some(&new_img));
            
            popover_icon_c.popdown();
        });
        
        icon_grid.attach(&btn_item, c, r, 1, 1);
    }
    popover_icon.set_child(Some(&icon_grid));

    let popover_icon_c = popover_icon.clone();
    btn_select_icon.connect_clicked(move |_| {
        popover_icon_c.popup();
    });

    name_hbox.append(&entry_name);
    name_hbox.append(&btn_select_icon);

    grid.attach(&lbl_name_field, 0, 0, 1, 1);
    grid.attach(&name_hbox, 1, 0, 1, 1);

    // Row 1: Command
    let lbl_cmd_field = Label::new(Some(&t("explore.settings_option_command")));
    lbl_cmd_field.set_halign(Align::Start);
    
    let entry_cmd_vbox = Box::new(Orientation::Vertical, 4);
    let entry_cmd = Entry::builder()
        .placeholder_text(&t("explore.settings_placeholder_command"))
        .hexpand(true)
        .css_classes(vec!["small-entry".to_string()])
        .build();
    entry_cmd_vbox.append(&entry_cmd);

    // Add clickable placeholders row
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
        let btn_p = Button::builder()
            .label(p)
            .tooltip_text(&t(desc_key))
            .css_classes(vec!["flat".to_string(), "placeholder-btn".to_string()])
            .build();
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

    let btn_add = Button::builder()
        .label(&t("explore.settings_add"))
        .halign(Align::End)
        .css_classes(vec!["suggested-action".to_string()])
        .build();
    btn_add.set_cursor_from_name(Some("pointer"));
    form_box.append(&btn_add);

    // Add button logic
    let entry_name_c = entry_name.clone();
    let entry_cmd_c = entry_cmd.clone();
    let selected_icon_c = selected_icon.clone();
    let btn_select_icon_c = btn_select_icon.clone();
    btn_add.connect_clicked(move |_| {
        let name_str = entry_name_c.text().to_string();
        let cmd_str = entry_cmd_c.text().to_string();
        let icon_str = selected_icon_c.borrow().clone();
        if !name_str.is_empty() && !cmd_str.is_empty() {
            let item = babydra_common::config::settings::CustomContextItem {
                name: name_str.clone(),
                command: cmd_str.clone(),
                icon: Some(icon_str),
            };
            
            // Append and save setting
            let mut s = babydra_common::load_explore_settings();
            s.custom_context_items.push(item.clone());
            babydra_common::save_explore_settings(&s);

            // Add row to UI
            row::render_option_row(&listbox_c, item);

            // Clear entry inputs
            entry_name_c.set_text("");
            entry_cmd_c.set_text("");
            
            // Reset selected icon to "settings"
            selected_icon_c.replace("settings".to_string());
            let reset_img = babydra_utils::ui::icon::get_icon("settings", 16);
            reset_img.set_pixel_size(16);
            btn_select_icon_c.set_child(Some(&reset_img));
        }
    });

    tab_context
}
