//! Shared color tokens for Cairo drawing and CSS generation.
//!
//! Single source of truth for accent colors used both by custom Cairo
//! widgets (switch, slider) and generated stylesheets. Keep in sync with
//! `docs/09-design.md` (tokens) and the theme packages under `themes/`.

/// Accent blue (`#3b82f6`) as normalized RGBA for Cairo.
pub const ACCENT_RGB: (f64, f64, f64) = (0.23, 0.51, 0.96);
pub const ACCENT_ALPHA: f64 = 1.0;

/// Accent blue with reduced alpha (used for passed ticks, hover fills).
pub const ACCENT_DIM_ALPHA: f64 = 0.9;

/// Knob shadow under the thumb (shared by switch & slider).
pub const KNOB_SHADOW_RGBA: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.30);

/// Inactive track fill — dark theme.
pub const TRACK_DARK_RGBA: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 0.16);

/// Inactive track fill — light theme.
pub const TRACK_LIGHT_RGBA: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.14);

/// Border around the inactive track — dark theme.
pub const TRACK_BORDER_DARK_RGBA: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 0.20);

/// Border around the inactive track — light theme.
pub const TRACK_BORDER_LIGHT_RGBA: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.16);

/// Slider track (unfilled) — dark theme.
pub const SLIDER_TRACK_DARK_RGBA: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 0.15);

/// Slider track (unfilled) — light theme.
pub const SLIDER_TRACK_LIGHT_RGBA: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.12);

/// Slider tick mark (not passed) — dark theme.
pub const SLIDER_TICK_DARK_RGBA: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 0.25);

/// Slider tick mark (not passed) — light theme.
pub const SLIDER_TICK_LIGHT_RGBA: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.25);

/// Slider knob outer shadow.
pub const SLIDER_KNOB_SHADOW_RGBA: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.35);

/// Slider label text — dark theme (selected / passed / idle alphas).
pub const SLIDER_TEXT_DARK_RGBA: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 1.0);
pub const SLIDER_TEXT_DARK_PASSED_RGBA: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 0.75);
pub const SLIDER_TEXT_DARK_IDLE_RGBA: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 0.40);

/// Slider label text — light theme (selected / passed / idle alphas).
pub const SLIDER_TEXT_LIGHT_RGBA: (f64, f64, f64, f64) = (0.12, 0.16, 0.23, 1.0);
pub const SLIDER_TEXT_LIGHT_PASSED_RGBA: (f64, f64, f64, f64) = (0.12, 0.16, 0.23, 0.80);
pub const SLIDER_TEXT_LIGHT_IDLE_RGBA: (f64, f64, f64, f64) = (0.12, 0.16, 0.23, 0.45);

/// Convenience: pick the slider label color for the current tick state.
pub fn slider_text_rgba(is_dark: bool, selected: bool, passed: bool) -> (f64, f64, f64, f64) {
    if is_dark {
        if selected {
            SLIDER_TEXT_DARK_RGBA
        } else if passed {
            SLIDER_TEXT_DARK_PASSED_RGBA
        } else {
            SLIDER_TEXT_DARK_IDLE_RGBA
        }
    } else if selected {
        SLIDER_TEXT_LIGHT_RGBA
    } else if passed {
        SLIDER_TEXT_LIGHT_PASSED_RGBA
    } else {
        SLIDER_TEXT_LIGHT_IDLE_RGBA
    }
}

/// Knob fill (always white).
pub const KNOB_FILL_RGBA: (f64, f64, f64, f64) = (1.0, 1.0, 1.0, 1.0);

/// Convenience: pick the inactive track color for the active theme.
pub fn track_rgba(is_dark: bool) -> (f64, f64, f64, f64) {
    if is_dark {
        TRACK_DARK_RGBA
    } else {
        TRACK_LIGHT_RGBA
    }
}

/// Convenience: pick the inactive track border color for the active theme.
pub fn track_border_rgba(is_dark: bool) -> (f64, f64, f64, f64) {
    if is_dark {
        TRACK_BORDER_DARK_RGBA
    } else {
        TRACK_BORDER_LIGHT_RGBA
    }
}

/// Convenience: pick the unfilled slider track color for the active theme.
pub fn slider_track_rgba(is_dark: bool) -> (f64, f64, f64, f64) {
    if is_dark {
        SLIDER_TRACK_DARK_RGBA
    } else {
        SLIDER_TRACK_LIGHT_RGBA
    }
}

/// Convenience: pick the not-passed slider tick color for the active theme.
pub fn slider_tick_rgba(is_dark: bool) -> (f64, f64, f64, f64) {
    if is_dark {
        SLIDER_TICK_DARK_RGBA
    } else {
        SLIDER_TICK_LIGHT_RGBA
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accent_matches_css_hex_3b82f6() {
        // 0.23 * 255 = 58.65 -> #3b ; 0.51 * 255 = 130 -> #82 ; 0.96 * 255 = 244.8 -> #f5
        let (r, g, b) = ACCENT_RGB;
        assert!((r * 255.0).round() >= 59.0 - 1.0 && (r * 255.0).round() <= 59.0 + 1.0);
        assert!((g * 255.0).round() >= 130.0 - 1.0 && (g * 255.0).round() <= 130.0 + 1.0);
        assert!((b * 255.0).round() >= 245.0 - 1.0 && (b * 255.0).round() <= 245.0 + 1.0);
    }

    #[test]
    fn theme_helpers_switch_on_dark_flag() {
        assert_eq!(track_rgba(true), TRACK_DARK_RGBA);
        assert_eq!(track_rgba(false), TRACK_LIGHT_RGBA);
        assert_eq!(slider_track_rgba(true), SLIDER_TRACK_DARK_RGBA);
        assert_eq!(slider_track_rgba(false), SLIDER_TRACK_LIGHT_RGBA);
        assert_eq!(slider_tick_rgba(true), SLIDER_TICK_DARK_RGBA);
        assert_eq!(slider_tick_rgba(false), SLIDER_TICK_LIGHT_RGBA);
    }

    #[test]
    fn accent_alpha_values_are_sane() {
        assert!(ACCENT_ALPHA <= 1.0);
        assert!(ACCENT_DIM_ALPHA <= ACCENT_ALPHA);
    }
}
