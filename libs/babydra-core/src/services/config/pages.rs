pub use crate::models::cli::PageId;
use std::sync::OnceLock;

static VALID_PAGES: OnceLock<Vec<PageId>> = OnceLock::new();

fn valid_pages() -> &'static Vec<PageId> {
    VALID_PAGES.get_or_init(|| {
        vec![
            PageId::Wifi,
            PageId::Bluetooth,
            PageId::Vpn,
            PageId::Certificates,
            PageId::Hosts,
            PageId::Displays,
            PageId::Appearance,
            PageId::Power,
            PageId::Startup,
            PageId::Apps,
            PageId::Env,
            PageId::Keybinds,
            PageId::SystemUpdate,
            PageId::Recovery,
            PageId::System,
            PageId::General,
        ]
    })
}

pub fn normalize_page_name(name: &str) -> String {
    match name.trim().to_lowercase().as_str() {
        "wallpaper" | "wallpapers" | "theme" | "themes" | "appearance" => "appearance".to_string(),
        "display" | "displays" | "screen" | "screens" | "monitor" | "monitors" => "displays".to_string(),
        "wifi" | "wlan" | "network" | "networks" => "wifi".to_string(),
        "bluetooth" | "bt" => "bluetooth".to_string(),
        "vpn" | "shield" => "vpn".to_string(),
        "power" | "battery" | "energy" => "power".to_string(),
        "keybind" | "keybinds" | "shortcuts" | "keys" => "keybinds".to_string(),
        "startup" | "autostart" | "startup_apps" => "startup".to_string(),
        "app" | "apps" | "applications" | "installed" => "apps".to_string(),
        "env" | "environment" | "vars" => "env".to_string(),
        "cert" | "certs" | "certificates" | "ssl" => "certificates".to_string(),
        "host" | "hosts" => "hosts".to_string(),
        "system_update" | "update" | "updates" => "system_update".to_string(),
        "recovery" | "reset" | "factory-reset" | "factory_reset" | "restore" => "recovery".to_string(),
        "system" | "about" | "info" => "system".to_string(),
        "general" | "generic" => "general".to_string(),
        other => other.to_string(),
    }
}

pub fn is_valid_page(name: &str) -> bool {
    valid_pages().iter().any(|p| p.as_str() == name)
}