use super::detection::get_backlight_device;
use super::state::{DdcutilProxyBlocking, BRIGHTNESS_STATE};

extern "C" {
    fn getuid() -> u32;
}

#[zbus::proxy(
    gen_blocking = true,
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
pub trait LogindManager {
    fn get_session_by_pid(&self, pid: u32) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn list_sessions(
        &self,
    ) -> zbus::Result<Vec<(String, u32, String, String, zbus::zvariant::OwnedObjectPath)>>;
}

#[zbus::proxy(
    gen_blocking = true,
    interface = "org.freedesktop.login1.Session",
    default_service = "org.freedesktop.login1"
)]
pub trait LogindSession {
    fn set_brightness(&self, subsystem: &str, name: &str, value: u32) -> zbus::Result<()>;
}

static DDC_SET_SENDER: std::sync::OnceLock<std::sync::mpsc::Sender<i32>> =
    std::sync::OnceLock::new();

fn get_active_session_path(
    manager: &LogindManagerProxyBlocking<'_>,
) -> Option<zbus::zvariant::OwnedObjectPath> {
    let my_uid = unsafe { getuid() };
    if let Ok(sessions) = manager.list_sessions() {
        // Prefer seat0 sessions for this user
        for (_, uid, _, seat, path) in &sessions {
            if *uid == my_uid && seat == "seat0" {
                return Some(path.clone());
            }
        }
        // Fall back to any session for this user
        for (_, uid, _, _, path) in &sessions {
            if *uid == my_uid {
                return Some(path.clone());
            }
        }
    }
    None
}

fn set_brightness_logind(
    device: &str,
    val: f64,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = zbus::blocking::Connection::system()?;
    let manager = LogindManagerProxyBlocking::new(&conn)?;
    let session_path = manager
        .get_session_by_pid(std::process::id())
        .ok()
        .or_else(|| get_active_session_path(&manager))
        .ok_or_else(|| zbus::Error::Address("No active logind session found".to_string()))?;

    let session = LogindSessionProxyBlocking::builder(&conn)
        .path(session_path)?
        .build()?;

    let max_path = format!("/sys/class/backlight/{}/max_brightness", device);
    let max_str = std::fs::read_to_string(max_path)?;
    let max_val = max_str.trim().parse::<f64>()?;
    let abs_val = ((val / 100.0) * max_val).round() as u32;

    session.set_brightness("backlight", device, abs_val)?;
    Ok(())
}

fn init_ddc_set_worker() -> std::sync::mpsc::Sender<i32> {
    let (tx, rx) = std::sync::mpsc::channel::<i32>();
    std::thread::spawn(move || {
        while let Ok(val) = rx.recv() {
            let mut latest_val = val;
            while let Ok(next_val) = rx.try_recv() {
                latest_val = next_val;
            }
            if let Ok(conn) = zbus::blocking::Connection::session() {
                if let Ok(proxy) = DdcutilProxyBlocking::new(&conn) {
                    let _ = proxy.set_vcp(1, "", 0x10, latest_val as u16, 0);
                }
            }
        }
    });
    tx
}

/// Sets `brightness` to the given value.
pub fn set_brightness(val: f64) {
    if let Ok(mut guard) = BRIGHTNESS_STATE.lock() {
        *guard = val;
    }
    if let Some(device) = get_backlight_device() {
        // Laptop/internal: use logind SetBrightness via system D-Bus
        let _ = set_brightness_logind(&device, val);
    } else {
        // External monitor: use ddcutil-service via session D-Bus
        let tx = DDC_SET_SENDER.get_or_init(init_ddc_set_worker);
        let _ = tx.send(val as i32);
    }
}
