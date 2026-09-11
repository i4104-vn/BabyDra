//! Shared CLI options, actions, and settings PageId models.

use crate::models::system::PerformanceProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PageId {
    Wifi,
    Bluetooth,
    Vpn,
    Certificates,
    Hosts,
    Displays,
    Appearance,
    Power,
    Startup,
    Apps,
    Env,
    Keybinds,
    SystemUpdate,
    Recovery,
    System,
    General,
}

impl PageId {
    pub fn as_str(&self) -> &'static str {
        match self {
            PageId::Wifi => "wifi",
            PageId::Bluetooth => "bluetooth",
            PageId::Vpn => "vpn",
            PageId::Certificates => "certificates",
            PageId::Hosts => "hosts",
            PageId::Displays => "displays",
            PageId::Appearance => "appearance",
            PageId::Power => "power",
            PageId::Startup => "startup",
            PageId::Apps => "apps",
            PageId::Env => "env",
            PageId::Keybinds => "keybinds",
            PageId::SystemUpdate => "system_update",
            PageId::Recovery => "recovery",
            PageId::System => "system",
            PageId::General => "general",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "wifi" => PageId::Wifi,
            "bluetooth" => PageId::Bluetooth,
            "vpn" => PageId::Vpn,
            "certificates" => PageId::Certificates,
            "hosts" => PageId::Hosts,
            "displays" => PageId::Displays,
            "appearance" => PageId::Appearance,
            "power" => PageId::Power,
            "startup" => PageId::Startup,
            "apps" => PageId::Apps,
            "env" => PageId::Env,
            "keybinds" => PageId::Keybinds,
            "system_update" => PageId::SystemUpdate,
            "recovery" => PageId::Recovery,
            "system" => PageId::System,
            "general" => PageId::General,
            _ => PageId::General,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CliOptions {
    pub page: Option<PageId>,
    pub action: Option<CliAction>,
}

#[derive(Debug, Clone)]
pub enum CliAction {
    ApplyBatterySaver,
    CheckBatterySaver,
    SetPowerProfile(PerformanceProfile),
    ApplyAllSettings,
    ApplyDisplays,
    SyncGreeterWallpaper,
    RunBackgroundUpdate(Option<String>),
    Help,
}
