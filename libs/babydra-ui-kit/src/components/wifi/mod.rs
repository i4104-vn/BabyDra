//! Standalone Wi-Fi signal strength icon component in its dedicated folder.

use gtk4::prelude::*;

/// Renders clean SVG string for Wi-Fi signal waves (0 to 4 bars).
pub fn render_wifi_signal_svg(
    strength_pct: u32,
    is_enabled: bool,
    is_connected: bool,
    size: i32,
    custom_color: Option<&str>,
) -> String {
    let color = match custom_color {
        Some(c) => c,
        None => {
            if !is_enabled {
                "#6B7280"
            } else if !is_connected {
                "#9CA3AF"
            } else {
                "#3B82F6"
            }
        }
    };

    let bars = if !is_enabled || !is_connected {
        0
    } else if strength_pct <= 25 {
        1
    } else if strength_pct <= 50 {
        2
    } else if strength_pct <= 75 {
        3
    } else {
        4
    };

    let o1 = if bars >= 1 { "1.0" } else { "0.2" };
    let o2 = if bars >= 2 { "1.0" } else { "0.2" };
    let o3 = if bars >= 3 { "1.0" } else { "0.2" };

    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="19.5" r="1" fill="{color}" />
            <path d="M8.5 16.15a5 5 0 0 1 7 0" opacity="{o1}" />
            <path d="M5 12.5a10 10 0 0 1 14 0" opacity="{o2}" />
            <path d="M2 8.82a15 15 0 0 1 20 0" opacity="{o3}" />
        </svg>"##
    )
}

/// Creates a Wi-Fi signal strength icon widget from explicit signal strength and connection state.
pub fn create_wifi_signal_icon_from_strength(
    strength_pct: u32,
    is_enabled: bool,
    is_connected: bool,
    size: i32,
    custom_color: Option<&str>,
) -> gtk4::Widget {
    let svg = render_wifi_signal_svg(strength_pct, is_enabled, is_connected, size, custom_color);
    crate::ui::icon::get_icon_from_svg(&svg, size).upcast()
}

/// Creates a Wi-Fi signal icon widget for a specific network (from signal percentage 0-100).
pub fn create_wifi_signal_icon_for_network(
    signal_pct: u32,
    is_connected: bool,
    size: i32,
    custom_color: Option<&str>,
) -> gtk4::Widget {
    create_wifi_signal_icon_from_strength(signal_pct, true, is_connected, size, custom_color)
}

/// Creates a dynamic Wi-Fi signal icon widget querying current system Wi-Fi state.
pub fn create_system_wifi_signal_icon(size: i32, custom_color: Option<&str>) -> gtk4::Widget {
    let (is_enabled, is_connected, strength_pct) =
        babydra_core::services::system::wifi::get_wifi_signal_strength();
    create_wifi_signal_icon_from_strength(
        strength_pct as u32,
        is_enabled,
        is_connected,
        size,
        custom_color,
    )
}
