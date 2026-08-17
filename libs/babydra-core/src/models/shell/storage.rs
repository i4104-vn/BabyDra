//! Storage disk information model.

#[derive(Clone, Debug)]
pub struct DiskInfo {
    pub filesystem: String,
    pub size: String,
    pub used: String,
    pub percent: f64,
    pub mount_point: String,
}
