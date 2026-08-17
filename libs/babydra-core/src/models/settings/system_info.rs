#[derive(Debug, Clone, Default)]
pub struct SystemInfoData {
    pub hostname: String,
    pub os_name: String,
    pub kernel_version: String,
    pub cpu_model: String,
    pub gpu_info: String,
    pub memory_text: String,
    pub uptime_text: String,
    pub cpu_arch: String,
}
