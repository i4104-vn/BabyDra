//! Power action trigger triggers.

use zbus::blocking::Connection;

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait LogindManager {
    fn power_off(&self, interactive: bool) -> zbus::Result<()>;
    fn reboot(&self, interactive: bool) -> zbus::Result<()>;
    fn suspend(&self, interactive: bool) -> zbus::Result<()>;
}

/// Triggers a system poweroff.
pub fn poweroff() {
    if let Ok(conn) = Connection::system() {
        if let Ok(manager) = LogindManagerProxyBlocking::new(&conn) {
            let _ = manager.power_off(true);
            return;
        }
    }
    let _ = std::process::Command::new("systemctl").arg("poweroff").spawn();
}

/// Triggers a system reboot.
pub fn reboot() {
    if let Ok(conn) = Connection::system() {
        if let Ok(manager) = LogindManagerProxyBlocking::new(&conn) {
            let _ = manager.reboot(true);
            return;
        }
    }
    let _ = std::process::Command::new("systemctl").arg("reboot").spawn();
}

/// Triggers a system suspend.
pub fn suspend() {
    if let Ok(conn) = Connection::system() {
        if let Ok(manager) = LogindManagerProxyBlocking::new(&conn) {
            let _ = manager.suspend(true);
            return;
        }
    }
    let _ = std::process::Command::new("systemctl").arg("suspend").spawn();
}
