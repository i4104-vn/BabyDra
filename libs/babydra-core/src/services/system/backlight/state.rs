use super::detection::get_backlight_device;

pub static BRIGHTNESS_STATE: std::sync::Mutex<f64> = std::sync::Mutex::new(60.0);

#[zbus::proxy(
    gen_blocking = true,
    interface = "com.ddcutil.DdcutilInterface",
    default_service = "com.ddcutil.DdcutilService",
    default_path = "/com/ddcutil/DdcutilObject"
)]
pub trait Ddcutil {
    #[zbus(name = "GetVcp")]
    fn get_vcp(
        &self,
        display_number: i32,
        edid_txt: &str,
        vcp_code: u8,
        flags: u32,
    ) -> zbus::Result<(u16, u16, String, i32, String)>;

    #[zbus(name = "SetVcp")]
    fn set_vcp(
        &self,
        display_number: i32,
        edid_txt: &str,
        vcp_code: u8,
        vcp_new_value: u16,
        flags: u32,
    ) -> zbus::Result<(i32, String)>;
}

/// Queries `ddcutil brightness`.
pub fn query_ddc_brightness() -> Option<f64> {
    if let Ok(conn) = zbus::blocking::Connection::session() {
        if let Ok(proxy) = DdcutilProxyBlocking::new(&conn) {
            if let Ok((current, _max, _formatted, status, _msg)) = proxy.get_vcp(1, "", 0x10, 0) {
                if status == 0 {
                    return Some(current as f64);
                }
            }
        }
    }
    None
}

/// Returns the current brightness level (0–100).
pub fn get_brightness() -> f64 {
    if let Some(device) = get_backlight_device() {
        let path = format!("/sys/class/backlight/{}/brightness", device);
        let max_path = format!("/sys/class/backlight/{}/max_brightness", device);
        if let (Ok(b_str), Ok(m_str)) = (
            std::fs::read_to_string(path),
            std::fs::read_to_string(max_path),
        ) {
            if let (Ok(b_val), Ok(m_val)) =
                (b_str.trim().parse::<f64>(), m_str.trim().parse::<f64>())
            {
                if m_val > 0.0 {
                    return (b_val / m_val) * 100.0;
                }
            }
        }
    } else {
        if let Ok(guard) = BRIGHTNESS_STATE.lock() {
            return *guard;
        }
    }
    60.0
}
