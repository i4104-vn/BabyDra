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
    pub orientation: String,       // "normal", "left", "right", "inverted"
    pub mode: String,              // "extend", "mirror"
    pub mirror_of: Option<String>,
    pub enabled: bool,
    pub available_resolutions: Vec<String>,
    pub available_rates: Vec<f64>,
}
