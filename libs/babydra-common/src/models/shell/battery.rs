//! Battery state data model.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryInfo {
    pub percentage: u32,
    pub is_charging: bool,
    pub is_ac_only: bool,
    pub status_text: String,
    pub time_remaining: Option<String>,
    pub health: Option<String>,
    pub technology: Option<String>,
    pub power_source: Option<String>,
    pub cycle_count: Option<u32>,
    pub voltage: Option<String>,
    pub energy_rate: Option<String>,
    pub capacity_wh: Option<String>,
    pub design_capacity: Option<String>,
    pub manufacturer: Option<String>,
    pub model_name: Option<String>,
    pub serial_number: Option<String>,
    pub temperature: Option<String>,
    pub active_profile: Option<String>,
}
