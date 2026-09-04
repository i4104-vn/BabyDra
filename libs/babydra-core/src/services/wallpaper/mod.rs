//! Wallpaper service: config persistence, thumbnail, greeter sync, and shared types.

pub mod avatar;
pub mod config;
pub mod greeter;
pub mod thumbnail;
pub mod types;
pub mod utils;

// Re-exports for backward compatibility
pub use avatar::{get_avatar_bytes, get_avatar_path, set_avatar};
pub use config::{
    apply_wallpaper, get_live_wallpapers, get_local_wallpapers, get_static_wallpapers,
    get_wallpaper, get_wallpaper_dir, get_wallpaper_mode, set_wallpaper, set_wallpaper_with_mode,
};
pub use greeter::{
    apply_greeter_wp, get_greeter_wp, get_greeter_wp_bytes, get_greeter_wp_css, read_image_bytes,
    set_greeter_wp, sync_shared_assets,
};
pub use thumbnail::{get_or_create_first_frame, get_or_create_thumbnail};
pub use types::{WallpaperKind, WallpaperMode};
pub use utils::{
    get_video_duration, is_gif_file, is_gstreamer_plugin_available,
    is_live_wallpaper_file, is_static_wallpaper_file, is_video_file,
};