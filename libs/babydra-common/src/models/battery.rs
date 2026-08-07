//! Battery state data model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryInfo {
    pub percentage: u32,
    pub is_charging: bool,
}
