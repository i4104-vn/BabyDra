pub mod render;
use gtk4::prelude::*;

pub use babydra_core::helper::volume::{
    get_audio_devices, get_current_volume, is_muted, set_volume,
};

/// Returns the current `active output device name`.
pub fn get_active_output_device_name() -> Option<String> {
    if let Ok(out) = std::process::Command::new("wpctl")
        .args(["inspect", "@DEFAULT_AUDIO_SINK@"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&out.stdout);
        for line in stdout.lines() {
            let line = line.trim();
            if line.contains("node.description =") {
                if let Some(val) = line.split('=').nth(1) {
                    return Some(val.trim().trim_matches('"').to_string());
                }
            }
        }
    }
    None
}

/// Update topbar volume icon.
pub fn update_topbar_volume_icon(vol_icon: &gtk4::Image) {
    let is_m = is_muted();
    let vol_pct = get_current_volume();
    let is_dark = babydra_ui_kit::ui::icon::is_dark_mode();
    let svg_content = if is_m || vol_pct == 0.0 {
        if is_dark {
            babydra_ui_kit::ui::icon::DARK_VOLUME_MUTE_SVG
        } else {
            babydra_ui_kit::ui::icon::LIGHT_VOLUME_MUTE_SVG
        }
    } else if vol_pct <= 45.0 {
        if is_dark {
            babydra_ui_kit::ui::icon::DARK_VOLUME_LOW_SVG
        } else {
            babydra_ui_kit::ui::icon::LIGHT_VOLUME_LOW_SVG
        }
    } else {
        if is_dark {
            babydra_ui_kit::ui::icon::DARK_VOLUME_SVG
        } else {
            babydra_ui_kit::ui::icon::LIGHT_VOLUME_SVG
        }
    };

    let new_icon = babydra_ui_kit::ui::icon::get_icon_from_svg(svg_content, 14);
    if let Some(paintable) = new_icon.paintable() {
        vol_icon.set_paintable(Some(&paintable));
    }

    let dev_name = get_active_output_device_name();
    let dev_suffix = dev_name
        .as_deref()
        .map(|d| format!(" • {}", d))
        .unwrap_or_default();

    let tooltip = if is_m {
        format!("Volume: Muted ({:.0}%){}", vol_pct, dev_suffix)
    } else {
        format!("Volume: {:.0}%{}", vol_pct, dev_suffix)
    };
    vol_icon.set_tooltip_text(Some(&tooltip));
}
