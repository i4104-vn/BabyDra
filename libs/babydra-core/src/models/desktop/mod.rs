//! Desktop application, grid layout, workspace and EXIF data models.

pub mod app;
pub mod desktop_state;
pub mod exif;
pub mod image_meta;
pub mod workspace;

pub use app::{AppChoice, DesktopApp, DesktopCache};
pub use desktop_state::{
    calc_auto_arrange, snap_to_grid, sort_entries, DesktopState, DEFAULT_CELL_HEIGHT,
    DEFAULT_CELL_WIDTH, DEFAULT_MARGIN_X, DEFAULT_MARGIN_Y,
};
pub use exif::ExifData;
pub use image_meta::ImageMetadata;
pub use workspace::{Workspace, WorkspaceReceiver, WorkspaceSnapshot};
