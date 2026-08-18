pub mod dialog;
pub mod log_dialog;

pub use dialog::show_password_dialog;
pub use log_dialog::show_decompress_log;

use babydra_core::services::explore::is_zip_encrypted;
use std::path::PathBuf;
use std::rc::Rc;

/// Perform decompress async.
pub fn decompress_async(
    archive_path: PathBuf,
    current_path: PathBuf,
    nav_callback: Rc<dyn Fn(PathBuf)>,
    parent: Option<&gtk4::Window>,
) {
    let nav_c = nav_callback.clone();
    let cp_c = current_path.clone();
    let archive_path_c = archive_path.clone();
    let parent_c = parent.cloned();

    glib::spawn_future_local(async move {
        let name = archive_path_c.to_string_lossy().to_lowercase();
        let is_zip = name.ends_with(".zip");

        if is_zip && is_zip_encrypted(&archive_path_c).await {
            show_password_dialog(archive_path_c, cp_c, nav_c, parent_c.as_ref());
        } else {
            show_decompress_log(archive_path_c, cp_c, nav_c, None, parent_c.as_ref());
        }
    });
}
