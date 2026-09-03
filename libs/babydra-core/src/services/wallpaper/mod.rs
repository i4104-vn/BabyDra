//! Wallpaper and avatar management utilities.
//! Handles wallpaper persistence, background resolution, and avatar processing.

pub mod avatar;
pub mod wallpaper;

pub use avatar::{get_avatar_bytes, get_avatar_path, set_avatar};
pub use wallpaper::{
    apply_greeter_wp, apply_wallpaper, get_greeter_wp, get_greeter_wp_bytes, get_greeter_wp_css,
    get_live_wallpapers, get_local_wallpapers, get_or_create_thumbnail, get_static_wallpapers,
    get_video_duration, get_wallpaper, get_wallpaper_dir, get_wallpaper_mode, is_gif_file,
    is_gstreamer_plugin_available, is_live_wallpaper_file, is_static_wallpaper_file, is_video_file,
    read_image_bytes, set_greeter_wp, set_wallpaper, set_wallpaper_with_mode, sync_shared_assets,
};

