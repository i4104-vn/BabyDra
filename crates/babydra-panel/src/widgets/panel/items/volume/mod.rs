pub mod render;

pub use babydra_common::helper::volume::{
    is_muted, get_current_volume, set_volume, get_audio_devices,
};

pub fn update_topbar_volume_icon(vol_icon: &gtk4::Image) {
    let is_m = is_muted();
    let is_dark = babydra_utils::ui::icon::is_dark_mode();
    let svg_content = if is_m {
        if is_dark {
            babydra_utils::ui::icon::DARK_VOLUME_MUTE_SVG
        } else {
            babydra_utils::ui::icon::LIGHT_VOLUME_MUTE_SVG
        }
    } else {
        if is_dark {
            babydra_utils::ui::icon::DARK_VOLUME_SVG
        } else {
            babydra_utils::ui::icon::LIGHT_VOLUME_SVG
        }
    };

    let new_icon = babydra_utils::ui::icon::get_icon_from_svg(svg_content, 14);
    if let Some(paintable) = new_icon.paintable() {
        vol_icon.set_paintable(Some(&paintable));
    }
}
