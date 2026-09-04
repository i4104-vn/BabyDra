//! Standalone Wi-Fi signal strength icon component in its dedicated folder.

use gtk4::prelude::*;

/// Renders clean SVG string for Wi-Fi signal waves (0 to 4 bars).
pub fn render_wifi_svg(
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

    let bars = if !is_enabled || strength_pct == 0 {
        0
    } else if strength_pct <= 4 {
        // Support discrete bar values (1..4)
        strength_pct.min(3)
    } else if strength_pct <= 30 {
        1
    } else if strength_pct <= 65 {
        2
    } else {
        3
    };

    let o_dot = if bars >= 1 { "1.0" } else { "0.3" };
    let o1 = if bars >= 1 { "1.0" } else { "0.2" };
    let o2 = if bars >= 2 { "1.0" } else { "0.2" };
    let o3 = if bars >= 3 { "1.0" } else { "0.2" };

    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="{size}" height="{size}" viewBox="0 0 24 24" fill="none" stroke="{color}" stroke-width="2.2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="19.5" r="1" fill="{color}" opacity="{o_dot}" />
            <path d="M8.5 16.15a5 5 0 0 1 7 0" opacity="{o1}" />
            <path d="M5 12.5a10 10 0 0 1 14 0" opacity="{o2}" />
            <path d="M2 8.82a15 15 0 0 1 20 0" opacity="{o3}" />
        </svg>"##
    )
}

/// Creates a Wi-Fi signal strength icon widget from explicit signal strength and connection state.
pub fn create_rssi_icon(
    strength_pct: u32,
    is_enabled: bool,
    is_connected: bool,
    size: i32,
    custom_color: Option<&str>,
) -> gtk4::Widget {
    let svg = render_wifi_svg(strength_pct, is_enabled, is_connected, size, custom_color);
    crate::ui::icon::get_icon_from_svg(&svg, size).upcast()
}

/// Creates a Wi-Fi signal icon widget for a specific network (from signal percentage 0-100).
pub fn create_wifi_net_icon(
    signal_pct: u32,
    is_connected: bool,
    size: i32,
    custom_color: Option<&str>,
) -> gtk4::Widget {
    create_rssi_icon(signal_pct, true, is_connected, size, custom_color)
}

/// Creates a dynamic Wi-Fi signal icon widget querying current system Wi-Fi state.
pub fn create_sys_wifi_icon(size: i32, custom_color: Option<&str>) -> gtk4::Widget {
    let (is_enabled, is_connected, strength_pct) =
        babydra_core::services::system::wifi::get_wifi_signal();
    create_rssi_icon(
        strength_pct as u32,
        is_enabled,
        is_connected,
        size,
        custom_color,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unconnected_network_shows_signal_bars() {
        // Unconnected network with 90% signal must render all 3 arcs lit
        let svg = render_wifi_svg(90, true, false, 18, Some("#3B82F6"));
        assert!(svg.contains(r#"opacity="1.0""#));
        // All 3 arcs must be lit at 1.0 opacity
        assert_eq!(svg.matches(r#"opacity="1.0""#).count(), 4); // dot + 3 arcs
        assert_eq!(svg.matches(r#"opacity="0.2""#).count(), 0);
    }

    #[test]
    fn test_medium_and_low_signal_levels() {
        // Medium signal (50%): dot + 2 arcs lit, 1 arc dimmed
        let svg_med = render_wifi_svg(50, true, false, 18, None);
        assert_eq!(svg_med.matches(r#"opacity="1.0""#).count(), 3); // dot + 2 arcs
        assert_eq!(svg_med.matches(r#"opacity="0.2""#).count(), 1); // 1 dimmed arc

        // Low signal (20%): dot + 1 arc lit, 2 arcs dimmed
        let svg_low = render_wifi_svg(20, true, false, 18, None);
        assert_eq!(svg_low.matches(r#"opacity="1.0""#).count(), 2); // dot + 1 arc
        assert_eq!(svg_low.matches(r#"opacity="0.2""#).count(), 2); // 2 dimmed arcs
    }

    #[test]
    fn test_zero_or_disabled_signal() {
        // Disabled: all dimmed
        let svg_dis = render_wifi_svg(90, false, false, 18, None);
        assert_eq!(svg_dis.matches(r#"opacity="0.3""#).count(), 1); // dimmed dot
        assert_eq!(svg_dis.matches(r#"opacity="0.2""#).count(), 3); // 3 dimmed arcs
        assert_eq!(svg_dis.matches(r#"opacity="1.0""#).count(), 0);

        // Zero signal: all dimmed
        let svg_zero = render_wifi_svg(0, true, false, 18, None);
        assert_eq!(svg_zero.matches(r#"opacity="0.3""#).count(), 1);
        assert_eq!(svg_zero.matches(r#"opacity="0.2""#).count(), 3);
    }

    #[test]
    fn test_discrete_bars_input() {
        let svg_1 = render_wifi_svg(1, true, false, 18, None);
        assert_eq!(svg_1.matches(r#"opacity="1.0""#).count(), 2); // dot + 1 arc

        let svg_2 = render_wifi_svg(2, true, false, 18, None);
        assert_eq!(svg_2.matches(r#"opacity="1.0""#).count(), 3); // dot + 2 arcs

        let svg_3 = render_wifi_svg(3, true, false, 18, None);
        assert_eq!(svg_3.matches(r#"opacity="1.0""#).count(), 4); // dot + 3 arcs
    }
}
