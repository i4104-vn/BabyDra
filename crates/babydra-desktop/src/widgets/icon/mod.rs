//! Desktop icon widget for files and shortcuts on ~/Desktop.

pub mod launcher;
mod render;
pub mod thumbnail;

pub use launcher::launch_entry;
pub use render::create_desktop_icon;
pub use thumbnail::{build_icon_frame, is_image_path};
