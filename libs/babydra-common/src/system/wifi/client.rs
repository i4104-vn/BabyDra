//! D-Bus proxy clients for NetworkManager interfaces.

use std::collections::HashMap;
use zbus::blocking::Connection;
use zbus::zvariant::{ObjectPath, Value};

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
pub trait NetworkManager {
    #[zbus(property)]
    fn wireless_enabled(&self) -> zbus::Result<bool>;
    #[zbus(property)]
    fn set_wireless_enabled(&self, value: bool) -> zbus::Result<()>;

    #[zbus(property)]
    fn all_devices(&self) -> zbus::Result<Vec<ObjectPath<'static>>>;

    fn get_device_by_ip_iface(&self, iface: &str) -> zbus::Result<ObjectPath<'static>>;

    fn activate_connection(
        &self,
        connection: &ObjectPath<'_>,
        device: &ObjectPath<'_>,
        specific_object: &ObjectPath<'_>,
    ) -> zbus::Result<(ObjectPath<'static>, ObjectPath<'static>)>;
}

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager.Device",
    default_service = "org.freedesktop.NetworkManager"
)]
pub trait Device {
    #[zbus(property)]
    fn device_type(&self) -> zbus::Result<u32>;
    #[zbus(property)]
    fn interface(&self) -> zbus::Result<String>;
}

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager.Device.Wifi",
    default_service = "org.freedesktop.NetworkManager"
)]
pub trait DeviceWifi {
    #[zbus(property)]
    fn active_access_point(&self) -> zbus::Result<ObjectPath<'static>>;

    fn get_access_points(&self) -> zbus::Result<Vec<ObjectPath<'static>>>;

    fn request_scan(&self, options: HashMap<&str, Value<'_>>) -> zbus::Result<()>;
}

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager.AccessPoint",
    default_service = "org.freedesktop.NetworkManager"
)]
pub trait AccessPoint {
    #[zbus(property)]
    fn ssid(&self) -> zbus::Result<Vec<u8>>;

    #[zbus(property)]
    fn strength(&self) -> zbus::Result<u8>;

    #[zbus(property)]
    fn wpa_flags(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn rsn_flags(&self) -> zbus::Result<u32>;
}

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager.Settings",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager/Settings"
)]
pub trait Settings {
    fn list_connections(&self) -> zbus::Result<Vec<ObjectPath<'static>>>;

    fn add_connection(
        &self,
        connection: HashMap<&str, HashMap<&str, Value<'_>>>,
    ) -> zbus::Result<ObjectPath<'static>>;
}

#[zbus::proxy(
    blocking,
    interface = "org.freedesktop.NetworkManager.Settings.Connection",
    default_service = "org.freedesktop.NetworkManager"
)]
pub trait ConnectionSettings {
    fn get_settings(&self) -> zbus::Result<HashMap<String, HashMap<String, Value<'static>>>>;
    fn delete(&self) -> zbus::Result<()>;
}

pub fn get_wifi_device(conn: &Connection) -> Option<ObjectPath<'static>> {
    let nm = NetworkManagerProxyBlocking::new(conn).ok()?;
    let devices = nm.all_devices().ok()?;
    for dev_path in devices {
        if let Ok(dev) = DeviceProxyBlocking::builder(conn).path(dev_path.clone()).ok()?.build() {
            if let Ok(dtype) = dev.device_type() {
                if dtype == 2 {
                    return Some(dev_path);
                }
            }
        }
    }
    None
}

pub fn val_to_str(val: &Value<'_>) -> Option<String> {
    if let Ok(s) = <&str>::try_from(val) {
        return Some(s.to_string());
    }
    if let Ok(s) = String::try_from(val.clone()) {
        return Some(s);
    }
    None
}
