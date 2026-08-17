use gtk4::{Box, Button, DropDown};

#[derive(Debug, Clone)]
pub struct MonitorConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub resolution_width: u32,
    pub resolution_height: u32,
    pub refresh_rate: f64,
    pub position_x: i32,
    pub position_y: i32,
    pub orientation: String, // "normal", "left", "right", "inverted"
    pub mode: String,        // "extend", "mirror"
    pub mirror_of: Option<String>,
    pub enabled: bool,
    pub available_resolutions: Vec<String>,
    pub available_rates: Vec<f64>,
}

pub struct DisplayCardRow {
    pub container: Box,
    pub resolution_dropdown: DropDown,
    pub rate_dropdown: DropDown,
    pub orientation_dropdown: DropDown,
}

pub struct DisplaysWidget {
    pub container: Box,
    pub save_btn: Button,
    pub refresh_btn: Button,
    pub card_rows: Vec<DisplayCardRow>,
}
