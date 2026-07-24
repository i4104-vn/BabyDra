//! System Theme UI layout generator matching reference design Image 4.

use gtk4::prelude::*;
use gtk4::{Box, Button, DropDown, Grid, Label, Orientation, StringList};

pub struct ThemesWidget {
    pub container: Box,
    pub gtk_theme_dropdown: DropDown,
    pub cursor_theme_dropdown: DropDown,
    pub cursor_size_dropdown: DropDown,
    pub apply_btn: Button,
}

pub fn build(
    gtk_themes: &[String],
    cursor_themes: &[String],
    cursor_sizes: &[u32],
) -> ThemesWidget {
    let container = Box::new(Orientation::Vertical, 20);

    // Glass Panel Card Container
    let card = Box::new(Orientation::Vertical, 20);
    card.add_css_class("glass-panel");

    // Card Header Row (Gradient Icon + Title + Description)
    let header_row = Box::new(Orientation::Horizontal, 16);
    header_row.set_margin_bottom(8);

    // Gradient Icon Container
    let icon_badge = Box::new(Orientation::Vertical, 0);
    icon_badge.add_css_class("blue-icon-badge");
    icon_badge.set_valign(gtk4::Align::Center);
    icon_badge.set_halign(gtk4::Align::Start);

    let palette_img = babydra_utils::ui::icon::get_icon("palette", 24);
    palette_img.set_pixel_size(24);
    palette_img.set_valign(gtk4::Align::Center);
    palette_img.set_halign(gtk4::Align::Center);
    palette_img.set_vexpand(true);
    icon_badge.append(&palette_img);
    header_row.append(&icon_badge);

    let header_text_box = Box::new(Orientation::Vertical, 4);
    header_text_box.set_valign(gtk4::Align::Center);

    let title_lbl = Label::new(Some("System Theme"));
    title_lbl.add_css_class("settings-page-title");
    title_lbl.set_halign(gtk4::Align::Start);
    header_text_box.append(&title_lbl);

    let desc_lbl = Label::new(Some("Customize color scheme, GTK themes, mouse cursors, and cursor sizes"));
    desc_lbl.add_css_class("settings-page-subtitle");
    desc_lbl.set_halign(gtk4::Align::Start);
    header_text_box.append(&desc_lbl);

    header_row.append(&header_text_box);
    card.append(&header_row);

    // Separator
    let sep = gtk4::Separator::new(Orientation::Horizontal);
    sep.add_css_class("profile-separator");
    card.append(&sep);

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

    // Field 2: Cursor Theme
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
    grid.attach(&cursor_box, 1, 0, 1, 1);

    // Field 3: Cursor Size
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
    grid.attach(&size_box, 0, 1, 1, 1);

    card.append(&grid);

    // Bottom Right Action Footer: Apply Themes Button
    let footer_box = Box::new(Orientation::Horizontal, 0);
    footer_box.set_margin_top(12);

    let apply_btn = Button::new();
    apply_btn.add_css_class("btn-golden");
    apply_btn.set_cursor_from_name(Some("pointer"));

    let btn_content = Box::new(Orientation::Horizontal, 8);
    let check_icon = babydra_utils::ui::icon::get_icon("check", 16);
    check_icon.set_pixel_size(16);
    btn_content.append(&check_icon);

    let btn_lbl = Label::new(Some("Apply Themes"));
    btn_lbl.add_css_class("spec-value");
    btn_content.append(&btn_lbl);

    apply_btn.set_child(Some(&btn_content));
    footer_box.append(&apply_btn);
    footer_box.set_halign(gtk4::Align::End);

    card.append(&footer_box);
    container.append(&card);

    ThemesWidget {
        container,
        gtk_theme_dropdown,
        cursor_theme_dropdown,
        cursor_size_dropdown,
        apply_btn,
    }
}

