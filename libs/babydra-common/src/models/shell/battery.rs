//! Battery state data model.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryInfo {
    pub percentage: u32,
    pub is_charging: bool,
    pub status_text: String,
    pub time_remaining: Option<String>,
}
