//! System Theme UI layout generator matching reference design Image 4.

use gtk4::prelude::*;
use gtk4::{Box, Button, DropDown, Grid, Label, Orientation, StringList};

pub struct ThemesWidget {
    pub container: Box,
    pub gtk_theme_dropdown: DropDown,
    pub icon_theme_dropdown: DropDown,
    pub cursor_theme_dropdown: DropDown,
    pub cursor_size_dropdown: DropDown,
    pub apply_btn: Button,
}

pub fn build(
    gtk_themes: &[String],
    icon_themes: &[String],
    cursor_themes: &[String],
    cursor_sizes: &[u32],
) -> ThemesWidget {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_vexpand(true);
    container.set_valign(gtk4::Align::Fill);

    // Page Header Row (System Theme Title synchronized with VPN & Bluetooth layout)
    let header_row = Box::new(Orientation::Horizontal, 12);
    header_row.set_margin_bottom(4);

    let title_lbl = Label::new(Some("System Theme"));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);
    header_row.append(&title_lbl);

    let spacer = Box::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    header_row.append(&spacer);

    container.append(&header_row);

    // Overlay to place Floating Action Button (FAB) at bottom-right
    let overlay = gtk4::Overlay::new();
    overlay.set_vexpand(true);
    overlay.set_hexpand(true);

    // Glass Panel Card Container
    let card = Box::new(Orientation::Vertical, 20);
    card.add_css_class("glass-panel");
    card.set_valign(gtk4::Align::Start);

    // 2-Column Form Grid
    let grid = Grid::new();
    grid.set_column_spacing(24);
    grid.set_row_spacing(16);
    grid.set_column_homogeneous(true);

    // Field 1: GTK Theme
    let gtk_box = Box::new(Orientation::Vertical, 6);
    let gtk_lbl = Label::new(Some("GTK Theme"));
    gtk_lbl.add_css_class("spec-label");
    gtk_lbl.set_halign(gtk4::Align::Start);
    gtk_box.append(&gtk_lbl);

    let gtk_items: Vec<&str> = gtk_themes.iter().map(|s| s.as_str()).collect();
    let gtk_model = StringList::new(&gtk_items);
    let gtk_theme_dropdown = DropDown::new(Some(gtk_model), Option::<gtk4::Expression>::None);
    gtk_theme_dropdown.set_cursor_from_name(Some("pointer"));
    gtk_box.append(&gtk_theme_dropdown);
    grid.attach(&gtk_box, 0, 0, 1, 1);

    // Field 2: Icon Theme
    let icon_box = Box::new(Orientation::Vertical, 6);
    let icon_lbl = Label::new(Some("Icon Theme"));
    icon_lbl.add_css_class("spec-label");
    icon_lbl.set_halign(gtk4::Align::Start);
    icon_box.append(&icon_lbl);

    let icon_items: Vec<&str> = icon_themes.iter().map(|s| s.as_str()).collect();
    let icon_model = StringList::new(&icon_items);
    let icon_theme_dropdown = DropDown::new(Some(icon_model), Option::<gtk4::Expression>::None);
    icon_theme_dropdown.set_cursor_from_name(Some("pointer"));
    icon_box.append(&icon_theme_dropdown);
    grid.attach(&icon_box, 1, 0, 1, 1);

    // Field 3: Cursor Theme
    let cursor_box = Box::new(Orientation::Vertical, 6);
    let cursor_lbl = Label::new(Some("Cursor Theme"));
    cursor_lbl.add_css_class("spec-label");
    cursor_lbl.set_halign(gtk4::Align::Start);
    cursor_box.append(&cursor_lbl);

    let cursor_items: Vec<&str> = cursor_themes.iter().map(|s| s.as_str()).collect();
    let cursor_model = StringList::new(&cursor_items);
    let cursor_theme_dropdown = DropDown::new(Some(cursor_model), Option::<gtk4::Expression>::None);
    cursor_theme_dropdown.set_cursor_from_name(Some("pointer"));
    cursor_box.append(&cursor_theme_dropdown);
    grid.attach(&cursor_box, 0, 1, 1, 1);

    // Field 4: Cursor Size
    let size_box = Box::new(Orientation::Vertical, 6);
    let size_lbl = Label::new(Some("Cursor Size"));
    size_lbl.add_css_class("spec-label");
    size_lbl.set_halign(gtk4::Align::Start);
    size_box.append(&size_lbl);

    let size_strs: Vec<String> = cursor_sizes.iter().map(|s| format!("{} px", s)).collect();
    let size_items: Vec<&str> = size_strs.iter().map(|s| s.as_str()).collect();
    let size_model = StringList::new(&size_items);
    let cursor_size_dropdown = DropDown::new(Some(size_model), Option::<gtk4::Expression>::None);
    cursor_size_dropdown.set_cursor_from_name(Some("pointer"));
    size_box.append(&cursor_size_dropdown);
    grid.attach(&size_box, 1, 1, 1, 1);

    card.append(&grid);

    let scroll = gtk4::ScrolledWindow::new();
    scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    scroll.set_vexpand(true);
    scroll.set_valign(gtk4::Align::Fill);
    scroll.set_child(Some(&card));

    overlay.set_child(Some(&scroll));

    // Floating Action Button (FAB) for Apply / Save Themes (matching VPN FAB component)
    let apply_btn = babydra_utils::components::create_fab("check");
    apply_btn.set_tooltip_text(Some("Apply Themes"));
    apply_btn.set_margin_end(24);
    apply_btn.set_margin_bottom(24);

    overlay.add_overlay(&apply_btn);

    container.append(&overlay);

    ThemesWidget {
        container,
        gtk_theme_dropdown,
        icon_theme_dropdown,
        cursor_theme_dropdown,
        cursor_size_dropdown,
        apply_btn,
    }
}

