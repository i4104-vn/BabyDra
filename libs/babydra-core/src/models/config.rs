//! Configuration data models for BabyDra.
//! Pure data structs + default impls; load/save logic stays in `config::`.

use crate::models::ThemeConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PowerConfig {
    #[serde(default = "default_power_profile")]
    pub profile: String,
    #[serde(default = "is_auto_saver_on")]
    pub auto_saver_enabled: bool,
    #[serde(default = "default_saver_threshold")]
    pub saver_threshold: u32,
    #[serde(default = "default_charge_limit")]
    pub charge_limit: u32,
}

fn default_power_profile() -> String {
    "balanced".to_string()
}
fn is_auto_saver_on() -> bool {
    true
}
fn default_saver_threshold() -> u32 {
    20
}
fn default_charge_limit() -> u32 {
    80
}

impl Default for PowerConfig {
    fn default() -> Self {
        Self {
            profile: default_power_profile(),
            auto_saver_enabled: is_auto_saver_on(),
            saver_threshold: default_saver_threshold(),
            charge_limit: default_charge_limit(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct WallpaperConfig {
    #[serde(default)]
    pub current: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NotificationConfig {
    #[serde(default)]
    pub dnd: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CustomContextItem {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SidebarItem {
    pub id: String,
    pub name: String,
    pub icon: String,
    pub path: std::path::PathBuf,
    pub is_bookmark: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ExploreSettings {
    #[serde(default = "default_view_mode")]
    pub view_mode: String,          // "icons" | "list"
    #[serde(default = "default_true")]
    pub preview_visible: bool,      // true | false
    #[serde(default)]
    pub show_hidden: bool,          // true | false
    #[serde(default = "default_true")]
    pub double_click_to_open: bool, // true | false
    #[serde(default)]
    pub permanent_delete: bool,     // true | false
    #[serde(default = "default_true")]
    pub calculate_dir_size: bool,   // true | false
    #[serde(default)]
    pub custom_context_items: Vec<CustomContextItem>,
    #[serde(default)]
    pub keybinds: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub sidebar_items: Vec<SidebarItem>,
}

fn default_view_mode() -> String {
    "icons".to_string()
}
fn default_true() -> bool {
    true
}

impl ExploreSettings {
    pub fn get_keybind(&self, action: &str) -> String {
        self.keybinds
            .get(action)
            .cloned()
            .unwrap_or_else(|| match action {
                "toggle_split" => "F3".to_string(),
                "toggle_preview" => "F4".to_string(),
                "toggle_hidden" => "Ctrl + H".to_string(),
                "cut" => "Ctrl + X".to_string(),
                "copy" => "Ctrl + C".to_string(),
                "paste" => "Ctrl + V".to_string(),
                "undo" => "Ctrl + Z".to_string(),
                "delete" => "Delete".to_string(),
                "permanent_delete" => "Shift + Delete".to_string(),
                "select_all" => "Ctrl + A".to_string(),
                "new_tab" => "Ctrl + N".to_string(),
                "close_tab" => "Ctrl + W".to_string(),
                _ => "".to_string(),
            })
    }
}

impl Default for ExploreSettings {
    fn default() -> Self {
        Self {
            view_mode: default_view_mode(),
            preview_visible: default_true(),
            show_hidden: false,
            double_click_to_open: default_true(),
            permanent_delete: false,
            calculate_dir_size: default_true(),
            custom_context_items: Vec::new(),
            keybinds: std::collections::HashMap::new(),
            sidebar_items: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DisplayMonitorSetting {
    #[serde(default)]
    pub name: String,
    #[serde(default = "default_width")]
    pub resolution_width: u32,
    #[serde(default = "default_height")]
    pub resolution_height: u32,
    #[serde(default = "default_refresh_rate")]
    pub refresh_rate: f64,
    #[serde(default)]
    pub position_x: i32,
    #[serde(default)]
    pub position_y: i32,
    #[serde(default = "default_orientation")]
    pub orientation: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_scale")]
    pub scale: f64,
}

fn default_width() -> u32 {
    1920
}
fn default_height() -> u32 {
    1080
}
fn default_refresh_rate() -> f64 {
    60.0
}
fn default_orientation() -> String {
    "normal".to_string()
}
fn default_enabled() -> bool {
    true
}
fn default_scale() -> f64 {
    1.0
}

impl Default for DisplayMonitorSetting {
    fn default() -> Self {
        Self {
            name: String::new(),
            resolution_width: default_width(),
            resolution_height: default_height(),
            refresh_rate: default_refresh_rate(),
            position_x: 0,
            position_y: 0,
            orientation: default_orientation(),
            enabled: default_enabled(),
            scale: default_scale(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DisplayConfig {
    #[serde(default)]
    pub monitors: Vec<DisplayMonitorSetting>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LockscreenConfig {
    #[serde(default)]
    pub background: String,
    #[serde(default)]
    pub avatar: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DesktopConfig {
    #[serde(default = "default_show_icons")]
    pub show_icons: bool,
    #[serde(default = "default_icon_size")]
    pub icon_size: u32,
    #[serde(default = "default_grid_spacing")]
    pub grid_spacing: u32,
    #[serde(default = "default_desktop_sort_by")]
    pub sort_by: String, // "name" | "type" | "modified" | "size"
    #[serde(default = "default_auto_arrange")]
    pub auto_arrange: bool,
    #[serde(skip)]
    pub icon_positions: std::collections::HashMap<String, (i32, i32)>,
    #[serde(default)]
    pub custom_context_items: Vec<CustomContextItem>,
}

fn default_show_icons() -> bool {
    true
}
fn default_icon_size() -> u32 {
    48
}
fn default_grid_spacing() -> u32 {
    100
}
fn default_desktop_sort_by() -> String {
    "name".to_string()
}
fn default_auto_arrange() -> bool {
    true
}

impl Default for DesktopConfig {
    fn default() -> Self {
        Self {
            show_icons: default_show_icons(),
            icon_size: default_icon_size(),
            grid_spacing: default_grid_spacing(),
            sort_by: default_desktop_sort_by(),
            auto_arrange: default_auto_arrange(),
            icon_positions: std::collections::HashMap::new(),
            custom_context_items: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct BabyDraConfig {
    #[serde(default)]
    pub power: PowerConfig,
    #[serde(default, skip_serializing)]
    pub explore: ExploreSettings,
    #[serde(default)]
    pub wallpaper: WallpaperConfig,
    #[serde(default)]
    pub notification: NotificationConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub lockscreen: LockscreenConfig,
    #[serde(default, skip_serializing)]
    pub desktop: DesktopConfig,
    /// Theme package selection (id + dark preference).
    /// Empty `id` = engine default (`babydra-default`).
    #[serde(default)]
    pub theme: ThemeConfig,
}

/// Parsed `variant.toml`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Variant {
    #[serde(default)]
    pub name: String,
    /// Theme package id to use (see `themes/`).
    pub theme: String,
    /// List of apps this variant installs / runs.
    #[serde(default)]
    pub apps: Vec<String>,
    /// Keybind map (action → target).
    #[serde(default)]
    pub keybinds: std::collections::HashMap<String, String>,
    /// Config overrides (dotted path → value).
    #[serde(default)]
    pub config_overrides: std::collections::HashMap<String, toml::Value>,
}
